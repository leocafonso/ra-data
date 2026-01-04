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

fn parse_elc_h(path: &Path) -> anyhow::Result<Vec<Interrupt>> {
    let content = fs::read_to_string(path)?;
    let mut interrupts = Vec::new();

    let re = regex!(r"ELC_EV(?:E)?NT_([A-Z0-9_]+)\s*=\s*\((0x[0-9A-F]+|[0-9]+)\),?\s*//\s*(.*)");

    for cap in re.captures_iter(&content) {
        let name = cap[1].to_string();
        let value_str = &cap[2];
        let description = cap[3].trim().to_string();

        let value = if value_str.starts_with("0x") {
            u32::from_str_radix(&value_str[2..], 16)?
        } else {
            value_str.parse()?
        };

        if name == "NONE" {
            continue;
        }

        interrupts.push(Interrupt {
            name,
            value,
            description: Some(description),
        });
    }

    Ok(interrupts)
}
