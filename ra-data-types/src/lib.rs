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
    pub mstp: Option<Mstp>,
    pub bit_width: Option<u32>,
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

