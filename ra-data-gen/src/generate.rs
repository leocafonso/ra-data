use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::Context;
use crate::rzone::Rzones;
use crate::pinmapping::PinMappings;
use crate::perimap::{PERIMAP};
use crate::mstp;
use ra_data_types::*;

/// Extract peripheral name and channel from a peripheral instance name
/// e.g., "GPT0" -> ("GPT", Some(0)), "SCI1" -> ("SCI", Some(1)), "DTC" -> ("DTC", None)
fn parse_peripheral_name(name: &str) -> (&str, Option<u32>) {
    // Use the mstp module's parsing which handles complex naming like GPT320, ADC140, etc.
    mstp::parse_peripheral_name(name)
}

/// Find interrupt signals that belong to a peripheral instance
fn find_interrupts_for_peripheral(
    interrupts: &[Interrupt],
    peri_type: &str,
    channel: Option<u32>,
) -> Vec<String> {
    interrupts
        .iter()
        .filter(|irq| {
            irq.peripheral.as_deref() == Some(peri_type) && irq.channel == channel
        })
        .filter_map(|irq| irq.signal.clone())
        .collect()
}

pub fn generate(
    rzones: &Rzones,
    pin_mappings: &PinMappings,
    family_interrupts: &BTreeMap<String, Vec<Interrupt>>,
) -> anyhow::Result<()> {
    let chips_dir = Path::new("./build/data/chips/");
    let regs_out_dir = Path::new("./build/data/registers/");
    fs::create_dir_all(chips_dir).context("failed to create chips directory")?;
    fs::create_dir_all(regs_out_dir).context("failed to create registers output directory")?;

    let registers_dir = Path::new("data/registers/");
    let mut available_registers = std::collections::HashSet::new();
    
    // Helper function to process YAML files recursively
    fn process_register_dir(
        dir: &Path, 
        regs_out_dir: &Path, 
        available_registers: &mut std::collections::HashSet<String>
    ) -> anyhow::Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Recursively process subdirectories
                process_register_dir(&path, regs_out_dir, available_registers)?;
            } else if path.extension().map_or(false, |ext| ext == "yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    available_registers.insert(stem.to_string());
                    
                    // Convert YAML to JSON for the build/data/registers directory
                    let content = fs::read_to_string(&path)?;
                    let yaml_val: serde_yaml::Value = serde_yaml::from_str(&content)?;
                    let json_content = serde_json::to_string_pretty(&yaml_val)?;
                    fs::write(regs_out_dir.join(format!("{}.json", stem)), json_content)?;
                }
            }
        }
        Ok(())
    }
    
    process_register_dir(registers_dir, regs_out_dir, &mut available_registers)?;

    let generate_chip = |(name, parsed): (&String, &crate::rzone::ParsedRzone)| -> anyhow::Result<()> {
        let mut packages = Vec::new();
        
        if let Some(mapping) = pin_mappings.get_for_chip(name) {
            packages.push(Package {
                chip: name.clone(),
                package: mapping.name.clone(),
                pins: mapping.pins.iter().map(|p| Pin {
                    position: p.position.clone(),
                    signals: p.signals.clone(),
                }).collect(),
            });
        }

           let mut peripherals = Vec::new();

        for p in &parsed.peripherals {
            // Normalize peripheral name (e.g., SYSTEM -> SYSC, PFS_A/PFS_B/PFS_NS -> PFS)
            let peri_name = match p.name.as_str() {
                "SYSTEM" => "SYSC",
                "PFS_A" | "PFS_B" | "PFS_NS" => "PFS",
                _ => &p.name,
            };
            let key = format!("{}:{}", name, peri_name);
            if let Some(info) = PERIMAP.get(&key) {
                let reg_key = format!("{}_{}", info.peri_type, info.version);
                if available_registers.contains(&reg_key) {
                    let mstp = mstp::get_mstp_for_peripheral(&p.name, name).map(|m| Mstp {
                        register: m.register,
                        bit: m.bit,
                    });

                    peripherals.push(Peripheral {
                        name: peri_name.to_string(),
                        address: p.address,
                        peri_type: info.peri_type.to_string(),
                        version: info.version.to_string(),
                        block: info.block.to_string(),
                        mstp,
                        interrupts: Vec::new(),
                    });
                }
            }
        }

        let family_dir = {
            let dname = parsed.family.to_lowercase();
            if dname.starts_with("r7f") || dname.starts_with("r7k") {
                format!("ra{}", &dname[4..])
            } else {
                dname
            }
        };

        let interrupts = family_interrupts.get(&family_dir).cloned().unwrap_or_default();

        // Link interrupts to peripherals
        let peripherals = peripherals.into_iter().map(|mut peri| {
            let (peri_type, channel) = parse_peripheral_name(&peri.name);
            peri.interrupts = find_interrupts_for_peripheral(&interrupts, peri_type, channel);
            peri
        }).collect();

        // Extract unique GPIO pins from packages
        // Filter to only include GPIO pins (starting with 'p' like p100, p104)
        let mut pin_set = std::collections::BTreeSet::new();
        for pkg in &packages {
            for pin in &pkg.pins {
                for signal in &pin.signals {
                    // Include only GPIO pins (start with 'p' followed by digits)
                    if signal.starts_with('p') && signal.len() > 1 && signal[1..].chars().next().map_or(false, |c| c.is_ascii_digit()) {
                        // Convert to uppercase P100 format
                        pin_set.insert(signal.to_uppercase());
                    }
                }
            }
        }
        let pins: Vec<ChipPin> = pin_set.into_iter().map(|name| ChipPin { name }).collect();

        let chip = Chip {
            name: name.clone(),
            family: parsed.family.clone(),
            core: parsed.core.clone(),
            interrupt_count: parsed.interrupt_count,
            memory: parsed.memories.iter().map(|m| Memory {
                name: m.name.clone(),
                kind: m.kind.clone(),
                address: m.address,
                size: m.size,
            }).collect(),
            peripherals,
            interrupts,
            packages,
            pins,
        };


        let file_path = chips_dir.join(format!("{}.json", name));
        let file = fs::File::create(file_path)?;
        serde_json::to_writer_pretty(file, &chip)?;
        Ok(())
    };

    #[cfg(feature = "rayon")]
    {
        use rayon::prelude::*;
        rzones.rzones.par_iter().try_for_each(generate_chip)?;
    }
    #[cfg(not(feature = "rayon"))]
    {
        rzones.rzones.iter().try_for_each(generate_chip)?;
    }

    Ok(())
}
