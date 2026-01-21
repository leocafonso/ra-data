use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chip {
    pub name: String,
    pub family: String,
    pub core: String,
    pub interrupt_count: u32,
    pub memory: Vec<Memory>,
    pub peripherals: Vec<Peripheral>,
    pub interrupts: Vec<Interrupt>,
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interrupt {
    pub name: String,
    pub value: u32,
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irq_number: Option<Vec<u32>>,
    /// The peripheral this interrupt belongs to (e.g., "GPT", "SCI", "IIC")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peripheral: Option<String>,
    /// The channel/instance number of the peripheral (e.g., 0 for GPT0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<u32>,
    /// The specific event signal (e.g., "COUNTER_OVERFLOW", "RXI", "TXI")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory {
    pub name: String,
    pub kind: String,
    pub address: u64,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub address: u64,
    #[serde(rename = "type")]
    pub peri_type: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mstp: Option<Mstp>,
    /// Interrupts/events associated with this peripheral (signal names only)
    /// The full event name can be constructed as "{peripheral_name}_{signal}"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interrupts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mstp {
    pub register: String,
    pub bit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub chip: String,
    pub package: String,
    pub pins: Vec<Pin>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pin {
    pub position: String,
    pub signals: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    pub flash_size: u32,
    pub ram_size: u32,
    pub packages: Vec<String>,
}

