pub struct MemoryRegion {
    pub name: &'static str,
    pub kind: &'static str,
    pub address: u64,
    pub size: u64,
}

pub struct Peripheral {
    pub name: &'static str,
    pub address: u64,
    pub kind: &'static str,
    pub version: &'static str,
}

pub struct Pin {
    pub position: &'static str,
    pub signals: &'static [&'static str],
}

pub struct Package {
    pub name: &'static str,
    pub pins: &'static [Pin],
}
