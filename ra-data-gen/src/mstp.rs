//! MSTP (Module Stop) mapping based on FSP's bsp_module_stop.h
//!
//! This module provides authoritative MSTP register and bit mappings for RA peripherals,
//! derived from the FSP (Flexible Software Package) bsp_module_stop.h header file.
//!
//! The mappings define which MSTP register (MSTPCRA/B/C/D/E) and which bit controls
//! the module stop state for each peripheral type and channel.

use crate::regex;

#[derive(Debug, Clone)]
pub struct MstpInfo {
    pub register: String,
    pub bit: u32,
}

/// Bit calculation direction for channel-based peripherals
#[derive(Debug, Clone, Copy)]
enum BitDir {
    /// bit = base - channel (e.g., SCI: 31 - channel)
    Minus,
    /// bit = base + channel (e.g., SAU: 6 + channel)  
    Plus,
    /// bit = base (fixed, channel ignored)
    Fixed,
}

/// MSTP rule for a peripheral type
#[derive(Debug, Clone, Copy)]
struct MstpRule {
    register: &'static str,
    base_bit: u32,
    direction: BitDir,
}

impl MstpRule {
    const fn new(register: &'static str, base_bit: u32, direction: BitDir) -> Self {
        Self { register, base_bit, direction }
    }
    
    fn compute_bit(&self, channel: u32) -> u32 {
        match self.direction {
            BitDir::Minus => self.base_bit.saturating_sub(channel),
            BitDir::Plus => self.base_bit + channel,
            BitDir::Fixed => self.base_bit,
        }
    }
}

/// Standard MSTP rules based on bsp_module_stop.h
/// These apply to most RA families unless overridden by family-specific rules
static MSTP_RULES: &[(&str, MstpRule)] = &[
    // MSTPCRB peripherals
    ("SCI",     MstpRule::new("MSTPCRB", 31, BitDir::Minus)),  // bit 31-channel
    ("SPI",     MstpRule::new("MSTPCRB", 19, BitDir::Minus)),  // bit 19-channel
    ("IIC",     MstpRule::new("MSTPCRB",  9, BitDir::Minus)),  // bit 9-channel
    ("RIIC",    MstpRule::new("MSTPCRB",  9, BitDir::Minus)),  // alias for IIC
    ("CAN",     MstpRule::new("MSTPCRB",  2, BitDir::Minus)),  // bit 2-channel
    ("CEC",     MstpRule::new("MSTPCRB",  3, BitDir::Fixed)),  // bit 3
    ("IRDA",    MstpRule::new("MSTPCRB",  5, BitDir::Minus)),  // bit 5-channel
    ("QSPI",    MstpRule::new("MSTPCRB",  6, BitDir::Minus)),  // bit 6-channel
    ("SAU",     MstpRule::new("MSTPCRB",  6, BitDir::Plus)),   // bit 6+channel
    ("IICA",    MstpRule::new("MSTPCRB", 10, BitDir::Plus)),   // bit 10+channel
    ("USBFS",   MstpRule::new("MSTPCRB", 11, BitDir::Minus)),  // bit 11-channel
    ("USB",     MstpRule::new("MSTPCRB", 11, BitDir::Minus)),  // alias for USBFS
    ("USBHS",   MstpRule::new("MSTPCRB", 12, BitDir::Minus)),  // bit 12-channel
    ("EPTPC",   MstpRule::new("MSTPCRB", 13, BitDir::Minus)),  // bit 13-channel
    ("USBCC",   MstpRule::new("MSTPCRB", 14, BitDir::Fixed)),  // bit 14
    ("ETHER",   MstpRule::new("MSTPCRB", 15, BitDir::Minus)),  // bit 15-channel
    ("ETHERC",  MstpRule::new("MSTPCRB", 15, BitDir::Minus)),  // alias for ETHER
    ("EDMAC",   MstpRule::new("MSTPCRB", 15, BitDir::Minus)),  // alias for ETHER
    ("OSPI",    MstpRule::new("MSTPCRB", 16, BitDir::Plus)),   // bit 16+channel
    
    // MSTPCRC peripherals
    ("CAC",     MstpRule::new("MSTPCRC",  0, BitDir::Fixed)),  // bit 0
    ("CRC",     MstpRule::new("MSTPCRC",  1, BitDir::Minus)),  // bit 1-channel
    ("PDC",     MstpRule::new("MSTPCRC",  2, BitDir::Minus)),  // bit 2-channel
    ("CTSU",    MstpRule::new("MSTPCRC",  3, BitDir::Minus)),  // bit 3-channel
    ("SLCDC",   MstpRule::new("MSTPCRC",  4, BitDir::Minus)),  // bit 4-channel
    ("GLCDC",   MstpRule::new("MSTPCRC",  4, BitDir::Minus)),  // bit 4-channel
    ("LCDC",    MstpRule::new("MSTPCRC",  4, BitDir::Minus)),  // alias
    ("JPEG",    MstpRule::new("MSTPCRC",  5, BitDir::Minus)),  // bit 5-channel
    ("DRW",     MstpRule::new("MSTPCRC",  6, BitDir::Minus)),  // bit 6-channel
    ("SSI",     MstpRule::new("MSTPCRC",  8, BitDir::Minus)),  // bit 8-channel
    ("SSIE",    MstpRule::new("MSTPCRC",  8, BitDir::Minus)),  // alias for SSI
    ("SRC",     MstpRule::new("MSTPCRC",  9, BitDir::Minus)),  // bit 9-channel
    ("MIPI_DSI",MstpRule::new("MSTPCRC", 10, BitDir::Minus)),  // bit 10-channel
    ("SDHIMMC", MstpRule::new("MSTPCRC", 12, BitDir::Minus)),  // bit 12-channel
    ("SDHI",    MstpRule::new("MSTPCRC", 12, BitDir::Minus)),  // alias for SDHIMMC
    ("DOC",     MstpRule::new("MSTPCRC", 13, BitDir::Minus)),  // bit 13-channel
    ("ELC",     MstpRule::new("MSTPCRC", 14, BitDir::Minus)),  // bit 14-channel
    ("MACL",    MstpRule::new("MSTPCRC", 15, BitDir::Minus)),  // bit 15-channel
    ("CEU",     MstpRule::new("MSTPCRC", 16, BitDir::Minus)),  // bit 16-channel
    ("MIPI_CSI",MstpRule::new("MSTPCRC", 17, BitDir::Minus)),  // bit 17-channel
    ("TFU",     MstpRule::new("MSTPCRC", 20, BitDir::Minus)),  // bit 20-channel
    ("IIRFA",   MstpRule::new("MSTPCRC", 21, BitDir::Minus)),  // bit 21-channel
    ("PDM",     MstpRule::new("MSTPCRC", 24, BitDir::Fixed)),  // bit 24
    ("CANFD",   MstpRule::new("MSTPCRC", 27, BitDir::Minus)),  // bit 27-channel
    ("TRNG",    MstpRule::new("MSTPCRC", 28, BitDir::Minus)),  // bit 28-channel
    ("SCE",     MstpRule::new("MSTPCRC", 31, BitDir::Minus)),  // bit 31-channel
    ("AES",     MstpRule::new("MSTPCRC", 31, BitDir::Minus)),  // bit 31-channel
    
    // MSTPCRD peripherals
    ("TAU",     MstpRule::new("MSTPCRD",  0, BitDir::Fixed)),  // bit 0
    ("AGT",     MstpRule::new("MSTPCRD",  3, BitDir::Minus)),  // bit 3-channel (default)
    ("AGTW",    MstpRule::new("MSTPCRD",  3, BitDir::Minus)),  // alias for AGT
    ("TML",     MstpRule::new("MSTPCRD",  4, BitDir::Fixed)),  // bit 4
    ("GPT",     MstpRule::new("MSTPCRD",  6, BitDir::Fixed)),  // bit 6 (default, no MSTPCRE)
    ("DSMIF",   MstpRule::new("MSTPCRD",  9, BitDir::Minus)),  // bit 9-channel
    ("POEG",    MstpRule::new("MSTPCRD", 14, BitDir::Minus)),  // bit 14-channel
    ("ADC",     MstpRule::new("MSTPCRD", 16, BitDir::Minus)),  // bit 16-channel
    ("ADC12",   MstpRule::new("MSTPCRD", 16, BitDir::Minus)),  // alias
    ("ADC14",   MstpRule::new("MSTPCRD", 16, BitDir::Minus)),  // alias (14-bit ADC)
    ("ADC140",  MstpRule::new("MSTPCRD", 16, BitDir::Fixed)),  // specific ADC140 -> channel 0
    ("SDADC",   MstpRule::new("MSTPCRD", 17, BitDir::Minus)),  // bit 17-channel
    ("DAC",     MstpRule::new("MSTPCRD", 20, BitDir::Minus)),  // bit 20-channel
    ("DAC12",   MstpRule::new("MSTPCRD", 20, BitDir::Fixed)),  // 12-bit DAC channel 0
    ("DAC8",    MstpRule::new("MSTPCRD", 19, BitDir::Fixed)),  // 8-bit DAC
    ("TSN",     MstpRule::new("MSTPCRD", 22, BitDir::Minus)),  // bit 22-channel
    ("RTC",     MstpRule::new("MSTPCRD", 23, BitDir::Fixed)),  // bit 23
    ("ACMPHS",  MstpRule::new("MSTPCRD", 28, BitDir::Minus)),  // bit 28-channel
    ("ACMPLP",  MstpRule::new("MSTPCRD", 29, BitDir::Fixed)),  // bit 29
    ("OPAMP",   MstpRule::new("MSTPCRD", 31, BitDir::Minus)),  // bit 31-channel
    
    // MSTPCRA peripherals
    ("NPU",     MstpRule::new("MSTPCRA", 16, BitDir::Fixed)),  // bit 16
    ("DMAC",    MstpRule::new("MSTPCRA", 22, BitDir::Fixed)),  // bit 22
    ("DMA",     MstpRule::new("MSTPCRA", 22, BitDir::Fixed)),  // alias for DMAC
    ("DTC",     MstpRule::new("MSTPCRA", 22, BitDir::Fixed)),  // bit 22 (same as DMAC)
];

/// RA family classification for family-specific MSTP rules
#[derive(Debug, Clone, Copy, PartialEq)]
enum RaFamily {
    Ra2,        // RA2xx series (some have MSTPCRE)
    Ra4,        // RA4xx series
    Ra6,        // RA6xx series
    Ra6t2,      // RA6T2 specifically (special GPT handling)
    Ra8,        // RA8xx series
    Ra8Gen2,    // RA8 Gen2 (special GPT/AGT/ADC handling)
    Other,
}

impl RaFamily {
    fn from_chip_name(name: &str) -> Self {
        let name = name.to_uppercase();
        
        // Check for specific MCU groups first
        if name.contains("RA6T2") || name.starts_with("R7FA6T2") {
            return RaFamily::Ra6t2;
        }
        
        // RA8 Gen2 detection (R7FA8D1, R7FA8T1, etc. - this is a heuristic)
        if name.starts_with("R7FA8D") || name.starts_with("R7FA8T") || name.starts_with("R7KA8") {
            return RaFamily::Ra8Gen2;
        }
        
        // General family detection from chip name pattern R7FAxxxx
        if name.starts_with("R7FA2") {
            RaFamily::Ra2
        } else if name.starts_with("R7FA4") {
            RaFamily::Ra4
        } else if name.starts_with("R7FA6") {
            RaFamily::Ra6
        } else if name.starts_with("R7FA8") || name.starts_with("R7KA8") {
            RaFamily::Ra8
        } else {
            RaFamily::Other
        }
    }
}

/// Compute MSTP info for a peripheral based on its type, channel, and chip family
pub fn compute_mstp(peri_type: &str, channel: u32, chip_name: &str) -> Option<MstpInfo> {
    let family = RaFamily::from_chip_name(chip_name);
    
    // Handle family-specific GPT rules
    if peri_type == "GPT" {
        return compute_gpt_mstp(channel, family);
    }
    
    // Handle family-specific AGT rules
    if peri_type == "AGT" || peri_type == "AGTW" {
        return compute_agt_mstp(channel, family);
    }
    
    // Handle family-specific ADC rules for RA8 Gen2
    if peri_type == "ADC" && family == RaFamily::Ra8Gen2 {
        return Some(MstpInfo {
            register: "MSTPCRD".to_string(),
            bit: 21 - channel, // RA8 Gen2: bit 21-channel
        });
    }
    
    // Handle family-specific POEG rules
    if peri_type == "POEG" {
        return compute_poeg_mstp(channel, family);
    }
    
    // Look up in standard rules
    for (name, rule) in MSTP_RULES {
        if *name == peri_type {
            return Some(MstpInfo {
                register: rule.register.to_string(),
                bit: rule.compute_bit(channel),
            });
        }
    }
    
    None
}

/// Compute GPT MSTP info based on family
/// GPT has complex family-dependent rules from bsp_module_stop.h
fn compute_gpt_mstp(channel: u32, family: RaFamily) -> Option<MstpInfo> {
    match family {
        RaFamily::Ra6t2 => {
            // RA6T2: MSTPCRE bit 31 for all channels
            Some(MstpInfo {
                register: "MSTPCRE".to_string(),
                bit: 31,
            })
        }
        RaFamily::Ra8Gen2 => {
            // RA8 Gen2: MSTPCRE, channels 4-9 share bit with channel 4
            let effective_channel = if channel >= 4 && channel <= 9 { 4 } else { channel };
            Some(MstpInfo {
                register: "MSTPCRE".to_string(),
                bit: 31 - effective_channel,
            })
        }
        RaFamily::Ra6 | RaFamily::Ra8 => {
            // RA6/RA8 with MSTPCRE: bit 31-channel
            Some(MstpInfo {
                register: "MSTPCRE".to_string(),
                bit: 31 - channel,
            })
        }
        _ => {
            // Devices without MSTPCRE: MSTPCRD bit 6 (or 5 for some)
            // Default to bit 6 for GPT
            Some(MstpInfo {
                register: "MSTPCRD".to_string(),
                bit: 6,
            })
        }
    }
}

/// Compute AGT MSTP info based on family
fn compute_agt_mstp(channel: u32, family: RaFamily) -> Option<MstpInfo> {
    match family {
        RaFamily::Ra8Gen2 => {
            // RA8 Gen2: MSTPCRD bit 5-channel
            Some(MstpInfo {
                register: "MSTPCRD".to_string(),
                bit: 5 - channel,
            })
        }
        _ => {
            // Default: MSTPCRD bit 3-channel
            Some(MstpInfo {
                register: "MSTPCRD".to_string(),
                bit: 3 - channel,
            })
        }
    }
}

/// Compute POEG MSTP info based on family  
fn compute_poeg_mstp(channel: u32, family: RaFamily) -> Option<MstpInfo> {
    // Most families: MSTPCRD bit 14-channel
    // Some older families might have fixed bit 14
    match family {
        RaFamily::Ra2 => {
            // Some RA2 devices have fixed bit 14
            Some(MstpInfo {
                register: "MSTPCRD".to_string(),
                bit: 14,
            })
        }
        _ => {
            Some(MstpInfo {
                register: "MSTPCRD".to_string(),
                bit: 14 - channel,
            })
        }
    }
}

/// Parse peripheral name from SVD peripheral name
/// Returns (base_type, channel)
/// 
/// Examples:
/// - "SCI0" -> ("SCI", Some(0))
/// - "GPT320" -> ("GPT", Some(0))   // 32-bit GPT channel 0
/// - "GPT162" -> ("GPT", Some(2))   // 16-bit GPT channel 2
/// - "ADC140" -> ("ADC", Some(0))   // 14-bit ADC channel 0
/// - "SSIE0" -> ("SSI", Some(0))    // SSI Extended channel 0
/// - "IIC0" -> ("IIC", Some(0))
/// - "DAC12" -> ("DAC", Some(0))    // 12-bit DAC
/// - "DAC8" -> ("DAC8", None)       // 8-bit DAC (separate peripheral)
/// - "RTC" -> ("RTC", None)
pub fn parse_peripheral_name(name: &str) -> (&str, Option<u32>) {
    let name_upper = name.to_uppercase();
    
    // Handle GPT naming: GPT320, GPT321, GPT162, GPT163, etc.
    // Format: GPT<bit_width><channel> where bit_width is 32, 16, or 8
    if let Some(caps) = regex!(r"^GPT(32|16|8)?([0-9]+)$").captures(&name_upper) {
        if let Some(ch_match) = caps.get(2) {
            if let Ok(channel) = ch_match.as_str().parse::<u32>() {
                return ("GPT", Some(channel));
            }
        }
    }
    
    // Handle ADC naming: ADC140, ADC141, ADC120, etc.
    // Format: ADC<bit_width><channel> where bit_width is 14, 12, 16, etc.
    if let Some(caps) = regex!(r"^ADC(14|12|16|24)?([0-9])$").captures(&name_upper) {
        if let Some(ch_match) = caps.get(2) {
            if let Ok(channel) = ch_match.as_str().parse::<u32>() {
                return ("ADC", Some(channel));
            }
        }
    }
    
    // Handle SSI/SSIE naming
    if let Some(caps) = regex!(r"^SSIE?([0-9]+)$").captures(&name_upper) {
        if let Ok(channel) = caps[1].parse::<u32>() {
            return ("SSI", Some(channel));
        }
    }
    
    // Handle DAC naming: DAC12, DAC8 (bit width only, no channel)
    if name_upper == "DAC12" {
        return ("DAC", Some(0));
    }
    if name_upper == "DAC8" {
        return ("DAC8", None);
    }
    
    // Handle RIIC -> IIC alias
    if let Some(caps) = regex!(r"^RIIC([0-9]+)$").captures(&name_upper) {
        if let Ok(channel) = caps[1].parse::<u32>() {
            return ("IIC", Some(channel));
        }
    }
    
    // Handle ETHERC/EDMAC -> ETHER
    if name_upper.starts_with("ETHERC") || name_upper.starts_with("EDMAC") {
        if let Some(caps) = regex!(r"[0-9]+$").captures(&name_upper) {
            if let Ok(channel) = caps[0].parse::<u32>() {
                return ("ETHER", Some(channel));
            }
        }
        return ("ETHER", Some(0));
    }
    
    // Generic pattern: NAME followed by digits
    if let Some(caps) = regex!(r"^([A-Z_]+)([0-9]+)$").captures(&name_upper) {
        let base = caps.get(1).map(|m| m.as_str()).unwrap_or(name);
        if let Ok(channel) = caps[2].parse::<u32>() {
            // Return a reference to static str for common peripherals
            let base_static = match base {
                "SCI" => "SCI",
                "SPI" => "SPI",
                "IIC" => "IIC",
                "CAN" => "CAN",
                "AGT" => "AGT",
                "AGTW" => "AGT",
                "GPT" => "GPT",
                "ADC" => "ADC",
                "DAC" => "DAC",
                "SSI" => "SSI",
                "DMAC" => "DMAC",
                "POEG" => "POEG",
                "CANFD" => "CANFD",
                "SDHI" => "SDHI",
                "ACMPHS" => "ACMPHS",
                "DSMIF" => "DSMIF",
                "ULPT" => "ULPT",
                "PORT" => return (name, None), // PORTs don't have MSTP
                _ => return (name, None),
            };
            return (base_static, Some(channel));
        }
    }
    
    // No channel number - return as-is
    (name, None)
}

/// Get MSTP info for a peripheral instance name
pub fn get_mstp_for_peripheral(peri_name: &str, chip_name: &str) -> Option<MstpInfo> {
    let (peri_type, channel) = parse_peripheral_name(peri_name);
    compute_mstp(peri_type, channel.unwrap_or(0), chip_name)
}
