use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::Context;
use roxmltree::Node;

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

    let mstp_peri = doc.descendants()
        .find(|n| n.has_tag_name("peripheral") && n.children().any(|c| c.has_tag_name("name") && c.text() == Some("MSTP")));

    if let Some(mstp) = mstp_peri {
        for register in mstp.descendants().filter(|n| n.has_tag_name("register")) {
            let reg_name = register.children().find(|n| n.has_tag_name("name")).and_then(|n| n.text()).unwrap_or("");
            
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
                    
                    if let Some(peri_name) = extract_peri_name(field_desc) {
                        mstp_map.insert(peri_name, MstpInfo {
                            register: reg_name.to_string(),
                            bit,
                        });
                    }
                }
            }
        }
    }

    Ok(mstp_map)
}

fn extract_peri_name(desc: &str) -> Option<String> {
    let desc = desc.to_uppercase();
    let desc = desc.strip_suffix(" MODULE STOP").unwrap_or(&desc);
    let desc = desc.strip_suffix(" MODULE STOP.").unwrap_or(desc);

    // Common mappings
    if desc.starts_with("GPT") {
        return Some(desc.split_whitespace().next()?.to_string());
    }
    if desc.starts_with("SERIAL COMMUNICATION INTERFACE") {
        let num = desc.split_whitespace().last()?;
        return Some(format!("SCI{}", num));
    }
    if desc.starts_with("12-BIT A/D CONVERTER") {
        let num = desc.split_whitespace().last()?;
        return Some(format!("ADC{}", num));
    }
    if desc.starts_with("12-BIT D/A CONVERTER") {
        let parts: Vec<&str> = desc.split_whitespace().collect();
        if let Some(num) = parts.last().and_then(|s| s.parse::<u32>().ok()) {
             return Some(format!("DAC{}", num));
        }
        return Some("DAC0".to_string());
    }
    if desc.starts_with("SERIAL PERIPHERAL INTERFACE") {
        let num = desc.split_whitespace().last()?;
        return Some(format!("SPI{}", num));
    }
    if desc.starts_with("I3C BUS INTERFACE") {
        let num = desc.split_whitespace().last()?;
        return Some(format!("I3C{}", num));
    }
    if desc.starts_with("CANFD") {
        return Some("CANFD0".to_string());
    }
    if desc.starts_with("UNIVERSAL SERIAL BUS 2.0 FS INTERFACE") {
        let num = desc.split_whitespace().last()?;
        return Some(format!("USB_FS{}", num));
    }
    if desc.starts_with("PORT OUTPUT ENABLE FOR GPT GROUP") {
        let group = desc.split_whitespace().last()?;
        let num = match group {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            "D" => 3,
            _ => return None,
        };
        return Some(format!("GPT_POEG{}", num));
    }
    if desc.starts_with("LOW POWER ASYNCHRONOUS GENERAL PURPOSE TIMER") {
        let num = desc.split_whitespace().last()?;
        return Some(format!("AGTW{}", num));
    }
    if desc.starts_with("QUAD SERIAL PERIPHERAL INTERFACE") {
        return Some("QSPI0".to_string());
    }
    if desc.starts_with("EVENT LINK CONTROL") {
        return Some("ELC0".to_string());
    }
    if desc.starts_with("DATA OPERATION CIRCUIT") {
        return Some("DOC0".to_string());
    }
    if desc.starts_with("CYCLIC REDUNDANCY CHECK CALCULATOR") {
        return Some("CRC0".to_string());
    }
    if desc.starts_with("CLOCK FREQUENCY ACCURACY MEASUREMENT CIRCUIT") {
        return Some("CAC0".to_string());
    }
    if desc.starts_with("SERIAL SOUND INTERFACE ENHANCED") {
        return Some("SSIE0".to_string());
    }
    if desc.starts_with("TEMPERATURE SENSOR") {
        return Some("TSN0".to_string());
    }
    if desc.starts_with("RANDOM NUMBER GENERATOR") {
        return Some("TRNG0".to_string());
    }
    if desc.starts_with("DMA CONTROLLER/DATA TRANSFER CONTROLLER") {
        return Some("DMAC0".to_string());
    }
    if desc.starts_with("SRAM0") {
        return Some("SRAM0".to_string());
    }
    if desc.starts_with("STANDBY SRAM") {
        return Some("SRAM_STB0".to_string());
    }
    
    // Fallback: if it's just one word, it might be the peripheral name
    let parts: Vec<&str> = desc.split_whitespace().collect();
    if parts.len() == 1 {
        return Some(parts[0].to_string());
    }

    None
}
