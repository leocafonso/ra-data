use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::Context;
use ra_data_types::Interrupt;
use crate::regex;

pub fn parse_all() -> anyhow::Result<BTreeMap<String, Vec<Interrupt>>> {
    let mut family_interrupts = BTreeMap::new();
    let mcu_dir = Path::new("sources/bsp/mcu");

    for entry in fs::read_dir(mcu_dir).context("failed to read mcu directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let family_name = path.file_name().unwrap().to_string_lossy().to_string();
            let elc_h = path.join("bsp_elc.h");
            if elc_h.exists() {
                let interrupts = parse_elc_h(&elc_h)?;
                family_interrupts.insert(family_name, interrupts);
            }
        }
    }

    Ok(family_interrupts)
}

/// Parses an interrupt name like "GPT0_COUNTER_OVERFLOW" or "SCI1_RXI" into
/// (peripheral, channel, signal) components.
/// Returns (Some("GPT"), Some(0), Some("COUNTER_OVERFLOW")) for "GPT0_COUNTER_OVERFLOW"
/// Returns (Some("SCI"), Some(1), Some("RXI")) for "SCI1_RXI"
/// Returns (Some("DTC"), None, Some("COMPLETE")) for "DTC_COMPLETE"
/// Returns (Some("ICU"), Some(0), Some("IRQ")) for "ICU_IRQ0"
fn parse_interrupt_name(name: &str) -> (Option<String>, Option<u32>, Option<String>) {
    // First try: pattern with channel number in peripheral name
    // Patterns like: GPT0_xxx, SCI1_xxx, IIC0_xxx, SPI0_xxx, AGT1_xxx, ADC0_xxx
    let re_with_channel = regex!(r"^([A-Z]+)(\d+)_(.+)$");
    
    if let Some(caps) = re_with_channel.captures(name) {
        let peripheral = caps.get(1).map(|m| m.as_str().to_string());
        let channel = caps.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
        let signal = caps.get(3).map(|m| m.as_str().to_string());
        return (peripheral, channel, signal);
    }
    
    // Second try: pattern with channel in signal name (e.g., ICU_IRQ0, LVD_LVD1)
    // Handles: ICU_IRQ0 -> (ICU, 0, IRQ), LVD_LVD1 -> (LVD, 1, LVD)
    let re_signal_channel = regex!(r"^([A-Z]+)_([A-Z]+)(\d+)$");
    
    if let Some(caps) = re_signal_channel.captures(name) {
        let peripheral = caps.get(1).map(|m| m.as_str().to_string());
        let signal = caps.get(2).map(|m| m.as_str().to_string());
        let channel = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
        return (peripheral, channel, signal);
    }
    
    // Third try: simple pattern without channel (e.g., DTC_COMPLETE, GPT_UVWEDGE)
    let re_no_channel = regex!(r"^([A-Z]+)_(.+)$");
    
    if let Some(caps) = re_no_channel.captures(name) {
        let peripheral = caps.get(1).map(|m| m.as_str().to_string());
        let signal = caps.get(2).map(|m| m.as_str().to_string());
        return (peripheral, None, signal);
    }
    
    (None, None, None)
}

fn parse_elc_h(path: &Path) -> anyhow::Result<Vec<Interrupt>> {
    let content = fs::read_to_string(path)?;
    let mut interrupts: BTreeMap<String, Interrupt> = BTreeMap::new();

    let re_icu = regex!(r"ICU_EVENT_([A-Z0-9_]+)\s*=\s*\((0x[0-9A-F]+|[0-9]+)\),?\s*//\s*(.*)");

    for cap in re_icu.captures_iter(&content) {
        let raw_name = cap[1].to_string();
        let value_str = &cap[2];
        let description = cap[3].trim().to_string();

        let value = if value_str.starts_with("0x") {
            u32::from_str_radix(&value_str[2..], 16)?
        } else {
            value_str.parse()?
        };

        if raw_name == "NONE" || description.contains("DEPRECATED") {
            continue;
        }

        let (name, irq_number) = if let Some(idx) = raw_name.rfind("_GROUP") {
            let suffix = &raw_name[idx..];
            if suffix.len() == 7 && suffix.as_bytes()[6].is_ascii_digit() {
                let group_digit = (suffix.as_bytes()[6] as char).to_digit(10).unwrap();
                (raw_name[..idx].to_string(), Some(vec![group_digit, group_digit + 8, group_digit + 16, group_digit + 24]))
            } else {
                (raw_name, None)
            }
        } else {
            (raw_name, None)
        };

        // Parse peripheral, channel, and signal from the name
        let (peripheral, channel, signal) = parse_interrupt_name(&name);

        if let Some(existing) = interrupts.get_mut(&name) {
            match (&mut existing.irq_number, irq_number) {
                (Some(existing_irqs), Some(new_irqs)) => {
                    existing_irqs.extend(new_irqs);
                    existing_irqs.sort();
                    existing_irqs.dedup();
                }
                (None, Some(new_irqs)) => {
                    existing.irq_number = Some(new_irqs);
                }
                _ => {}
            }
            if let Some(existing_desc) = &mut existing.description {
                existing_desc.push_str(" / ");
                existing_desc.push_str(&description);
            } else {
                existing.description = Some(description);
            }
        } else {
            interrupts.insert(name.clone(), Interrupt {
                name,
                value,
                description: Some(description),
                irq_number,
                peripheral,
                channel,
                signal,
            });
        }
    }

    let re_elc = regex!(r"ELC_EV(?:E)?NT_([A-Z0-9_]+)\s*=\s*\((0x[0-9A-F]+|[0-9]+)\),?\s*//\s*(.*)");

    for cap in re_elc.captures_iter(&content) {
        let name = cap[1].to_string();
        
        if name == "NONE" || interrupts.contains_key(&name) {
            continue;
        }

        let value_str = &cap[2];
        let description = cap[3].trim().to_string();

        let value = if value_str.starts_with("0x") {
            u32::from_str_radix(&value_str[2..], 16)?
        } else {
            value_str.parse()?
        };

        // Parse peripheral, channel, and signal from the name
        let (peripheral, channel, signal) = parse_interrupt_name(&name);

        interrupts.insert(name.clone(), Interrupt {
            name,
            value,
            description: Some(description),
            irq_number: None,
            peripheral,
            channel,
            signal,
        });
    }

    Ok(interrupts.into_values().collect())
}
