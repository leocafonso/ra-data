use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::Context;
use crate::regex;

#[derive(Debug, Clone)]
pub struct MstpInfo {
    pub register: String,
    pub bit: u32,
}

pub fn parse_all() -> anyhow::Result<BTreeMap<String, BTreeMap<String, MstpInfo>>> {
    let mut chip_mstp = BTreeMap::new();
    let svd_dir = Path::new("sources/svd");

    for entry in fs::read_dir(svd_dir).context("failed to read svd directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "svd") {
            let chip_name = path.file_stem().unwrap().to_string_lossy().to_string();
            let mstp_map = parse_svd(&path, &chip_name)?;
            chip_mstp.insert(chip_name, mstp_map);
        }
    }

    Ok(chip_mstp)
}

fn parse_svd(path: &Path, _chip_name: &str) -> anyhow::Result<BTreeMap<String, MstpInfo>> {
    let content = fs::read_to_string(path)?;
    let doc = roxmltree::Document::parse(&content)?;
    
    let mut mstp_map = BTreeMap::new();

    let peri_names = ["MSTP", "SYSTEM", "SYSC", "CPG"];
    for mstp in doc.descendants().filter(|n| n.has_tag_name("peripheral") && n.children().any(|c| c.has_tag_name("name") && peri_names.contains(&c.text().unwrap_or("")))) {
        let p_name = mstp.children().find(|n| n.has_tag_name("name")).and_then(|n| n.text()).unwrap_or("");
        for register in mstp.descendants().filter(|n| n.has_tag_name("register")) {
            let reg_name = register.children().find(|n| n.has_tag_name("name")).and_then(|n| n.text()).unwrap_or("");
            if !reg_name.starts_with("MSTPCR") {
                continue;
            }
            for field in register.descendants().filter(|n| n.has_tag_name("field")) {
                let field_desc = field.children().find(|n| n.has_tag_name("description")).and_then(|n| n.text()).unwrap_or("");
                
                let bit_offset = field.children().find(|n| n.has_tag_name("bitOffset")).and_then(|n| n.text())
                    .or_else(|| field.children().find(|n| n.has_tag_name("lsb")).and_then(|n| n.text()))
                    .and_then(|t| t.parse::<u32>().ok());

                if let Some(bit) = bit_offset {
                    // Try to extract peripheral name from description
                    // Example: "GPT0 Module Stop" -> "GPT0"
                    // Example: "Serial Communication Interface 0 Module Stop" -> "SCI0"
                    // Example: "12-bit A/D Converter 0 Module Stop" -> "ADC0"
                    
                    if let Some(peri_names) = extract_peri_names(field_desc) {
                        for peri_name in peri_names {
                            mstp_map.insert(peri_name, MstpInfo {
                                register: reg_name.to_string(),
                                bit,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(mstp_map)
}

fn extract_peri_names(desc: &str) -> Option<Vec<String>> {
    let desc = desc.to_uppercase();
    let desc = desc.strip_suffix(" MODULE STOP").unwrap_or(&desc);
    let desc = desc.strip_suffix(" MODULE STOP.").unwrap_or(desc);

    if desc.contains("PORT OUTPUT ENABLE") {
        return None;
    }

    let mut names = Vec::new();

    if desc.contains("GPT") || desc.contains("GENERAL PWM TIMER") {
        // Pattern: ch13-ch8 or ch7-ch0 or ch6 - ch1
        if let Some(caps) = regex!(r"CH([0-9]+)\s*-\s*CH([0-9]+)").captures(desc) {
            let n1: u32 = caps[1].parse().ok()?;
            let n2: u32 = caps[2].parse().ok()?;
            let (start, end) = if n1 < n2 { (n1, n2) } else { (n2, n1) };
            for i in start..=end {
                names.push(format!("GPT{}", i));
            }
        }
        // Pattern: 323 to 320 or 164 to 169 or 32EH0 to 32EH3 or 4-9
        else if let Some(caps) = regex!(r"TIMER\s*(?:32|16|8)?(?:EH|E|H|CH)?([0-9]+)\s*(?:TO|-)\s*(?:32|16|8)?(?:EH|E|H|CH)?([0-9]+)").captures(desc) {
            let n1: u32 = caps[1].parse().ok()?;
            let n2: u32 = caps[2].parse().ok()?;
            let (start, end) = if n1 < n2 { (n1, n2) } else { (n2, n1) };
            for i in start..=end {
                names.push(format!("GPT{}", i));
            }
        }
        // Pattern: 32n
        else if desc.contains("32N") {
            for i in 0..16 {
                names.push(format!("GPT{}", i));
            }
        }
        // Single channel GPT0, GPT1, etc.
        else if let Some(caps) = regex!(r"(?:GPT|TIMER)\s*(?:32|16|8)?(?:EH|E|H|CH)?([0-9]+)").captures(desc) {
            names.push(format!("GPT{}", &caps[1]));
        }
        
        if !names.is_empty() {
            return Some(names);
        }
    }

    None
}
