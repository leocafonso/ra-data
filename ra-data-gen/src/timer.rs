use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::Context;

pub fn parse_all() -> anyhow::Result<BTreeMap<String, BTreeMap<String, u32>>> {
    let mut chip_timers = BTreeMap::new();
    let svd_dir = Path::new("sources/svd");

    for entry in fs::read_dir(svd_dir).context("failed to read svd directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "svd") {
            let chip_name = path.file_stem().unwrap().to_string_lossy().to_string();
            let timer_map = parse_svd(&path)?;
            chip_timers.insert(chip_name, timer_map);
        }
    }

    Ok(chip_timers)
}

fn parse_svd(path: &Path) -> anyhow::Result<BTreeMap<String, u32>> {
    let content = fs::read_to_string(path)?;
    let doc = roxmltree::Document::parse(&content)?;
    
    let mut timer_map = BTreeMap::new();

    for peri in doc.descendants().filter(|n| n.has_tag_name("peripheral")) {
        let name = peri.children().find(|n| n.has_tag_name("name")).and_then(|n| n.text()).unwrap_or("");
        
        if name.contains("GPT") {
            // Determine bit width from name if possible
            let mut bit_width = if name.contains("32") {
                Some(32)
            } else if name.contains("16") {
                Some(16)
            } else {
                None
            };

            // If not in name, find GTCNT register and its size
            if bit_width.is_none() {
                let gtcnt = peri.descendants().find(|n| n.has_tag_name("register") && n.children().any(|c| c.has_tag_name("name") && c.text() == Some("GTCNT")));
                if let Some(reg) = gtcnt {
                    bit_width = reg.children().find(|n| n.has_tag_name("size")).and_then(|n| n.text())
                        .and_then(|t| {
                            if t.starts_with("0x") {
                                u32::from_str_radix(&t[2..], 16).ok()
                            } else {
                                t.parse::<u32>().ok()
                            }
                        });
                }
            }

            if let Some(bw) = bit_width {
                // Normalize name: GPT16E0 -> GPT0, GPT32E0 -> GPT0, GPT320 -> GPT0, GPT164 -> GPT4
                // Enhanced High Resolution: GPT32EH0 -> GPT0
                let normalized_name = if name.starts_with("GPT32EH") {
                    format!("GPT{}", &name[7..])
                } else if name.starts_with("GPT16EH") {
                    format!("GPT{}", &name[7..])
                } else if name.starts_with("GPT16E") || name.starts_with("GPT32E") {
                    format!("GPT{}", &name[6..])
                } else if name.starts_with("GPT32") {
                    format!("GPT{}", &name[5..])
                } else if name.starts_with("GPT16") {
                    format!("GPT{}", &name[5..])
                } else {
                    name.to_string()
                };
                
                timer_map.insert(normalized_name, bw);
            }
        }
    }

    Ok(timer_map)
}
