#[derive(Copy, Clone)]
pub struct Metadata {
    pub name: &'static str,
    pub family: &'static str,
    pub core: &'static str,
    pub memory: &'static [MemoryRegion],
    pub peripherals: &'static [Peripheral],
    pub events: &'static [Event],
    pub packages: &'static [Package],
    pub interrupt_count: usize,
}

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
    pub interrupts: &'static [PeripheralInterrupt],
}

/// Signal name for a peripheral interrupt (e.g., "COUNTER_OVERFLOW")
/// The full event name can be constructed as "{peripheral_name}_{signal}"
pub type PeripheralInterrupt = &'static str;

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
    /// The peripheral this event belongs to (e.g., "GPT", "SCI", "IIC").
    pub peripheral: Option<&'static str>,
    /// The channel/instance number of the peripheral (e.g., 0 for GPT0).
    pub channel: Option<u8>,
    /// The specific event signal (e.g., "COUNTER_OVERFLOW", "RXI", "TXI").
    pub signal: Option<&'static str>,
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
