#[derive(Copy, Clone)]
pub struct MemoryRegion {
    pub name: &'static str,
    pub kind: &'static str,
    pub address: u64,
    pub size: u64,
}

#[derive(Copy, Clone)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u64,
    pub kind: &'static str,
    pub version: &'static str,
    pub mstp: Option<Mstp>,
    pub bit_width: Option<u32>,
}

#[derive(Copy, Clone)]
pub struct Mstp {
    pub register: &'static str,
    pub bit: u32,
}

#[derive(Copy, Clone)]
pub struct Pin {
    pub position: &'static str,
    pub signals: &'static [&'static str],
}

#[derive(Copy, Clone)]
pub struct Package {
    pub name: &'static str,
    pub pins: &'static [Pin],
}
