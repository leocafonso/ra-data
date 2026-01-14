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

/// Event that can be mapped to an ICU IELSR slot.
/// For devices with grouped interrupts (RA2 family), `irq_slots` contains
/// the allowed IELSR indices. For other devices, it's empty (any slot allowed).
#[derive(Copy, Clone)]
pub struct Event {
    pub name: &'static str,
    pub id: u16,
    /// Allowed IELSR slot indices. Empty means any slot is allowed.
    pub irq_slots: &'static [u8],
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
