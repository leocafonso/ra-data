use crate::util::RegexMap;

pub struct PeriInfo {
    pub peri_type: &'static str,
    pub version: &'static str,
}

pub static PERIMAP: RegexMap<PeriInfo> = RegexMap::new(&[
    // SRAM mappings
    // RA0, RA2, RA4, RA6, RA8 have different SRAM controllers
    // Some are "Classic" (no TrustZone), some are "TrustZone" enabled.
    
    // Example: RA0 series (M23)
    ("R7FA0E1.*:SRAM", PeriInfo { peri_type: "sram", version: "v1_tz" }),
    
    // Example: RA2 series (M23)
    ("R7FA2.*:SRAM", PeriInfo { peri_type: "sram", version: "v1_tz" }),
    
    // Example: RA4 series (M4 - Classic)
    ("R7FA4M1.*:SRAM", PeriInfo { peri_type: "sram", version: "v1_classic" }),
    
    // Example: RA4 series (M33 - TrustZone)
    ("R7FA4E1.*:SRAM", PeriInfo { peri_type: "sram", version: "v2_tz" }),

    // GPIO mappings
    ("R7FA0.*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v3" }),
    ("R7FA8.*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v2" }),
    ("R7FA6T2.*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v2" }),
    (".*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v1" }),
    
    // SAU mappings (Serial Array Unit - Base)
    (".*:SAU\\d+", PeriInfo { peri_type: "sau", version: "v1" }),
    
    // UARTA mappings
    (".*:UARTA\\d+", PeriInfo { peri_type: "uart", version: "v1" }),

    // IICA mappings
    (".*:IICA\\d+", PeriInfo { peri_type: "i2c", version: "v1" }),

    // IIC mappings
    (".*:IIC\\d+", PeriInfo { peri_type: "i2c", version: "v1" }),
    (".*:IIC_B\\d+", PeriInfo { peri_type: "i2c", version: "v1" }),

    // SPI mappings
    (".*:SPI\\d+", PeriInfo { peri_type: "spi", version: "v1" }),
    (".*:SPI_B\\d+", PeriInfo { peri_type: "spi", version: "v1" }),

    // Timer mappings (Timer Array Unit)
    (".*:TAU", PeriInfo { peri_type: "timer", version: "v1" }),

    // Timer mappings (General PWM Timer)
    (".*:GPT\\d+", PeriInfo { peri_type: "timer", version: "v1" }),

    // ADC mappings
    (".*:ADC_D\\d+", PeriInfo { peri_type: "adc", version: "v1" }),

    // Other common peripherals
    (".*:CRC", PeriInfo { peri_type: "crc", version: "v1" }),
    (".*:DTC", PeriInfo { peri_type: "dtc", version: "v1" }),
    (".*:ELC", PeriInfo { peri_type: "elc", version: "v1" }),
    (".*:ICU", PeriInfo { peri_type: "icu", version: "v1" }),
    (".*:IWDT", PeriInfo { peri_type: "iwdt", version: "v1" }),
    (".*:RTC_C", PeriInfo { peri_type: "rtc", version: "v1" }),
    ("R7FA0E1.*:SYSTEM", PeriInfo { peri_type: "system", version: "v1" }),
    ("R7FA6M5.*:SYSTEM", PeriInfo { peri_type: "system", version: "v3" }),
    ("R7FA6M1.*:SYSTEM", PeriInfo { peri_type: "system", version: "v4" }),
    ("R7FA8.*:SYSTEM", PeriInfo { peri_type: "system", version: "v5" }),
    (".*:SYSTEM", PeriInfo { peri_type: "system", version: "v2" }), // Default to v2
    (".*:TRNG", PeriInfo { peri_type: "trng", version: "v1" }),
    (".*:WDT", PeriInfo { peri_type: "wdt", version: "v1" }),

    // USB
    (".*:USB\\d+", PeriInfo { peri_type: "usb", version: "v1" }),
    (".*:USB_FS", PeriInfo { peri_type: "usb", version: "v1" }),
    (".*:USB_HS", PeriInfo { peri_type: "usb", version: "v1" }),

    // Ethernet
    (".*:ETHERC\\d+", PeriInfo { peri_type: "eth", version: "v1" }),
    (".*:ETHERC_EDMAC\\d+", PeriInfo { peri_type: "eth_edmac", version: "v1" }),

    // CAN
    (".*:CAN\\d+", PeriInfo { peri_type: "can", version: "v1" }),
    (".*:CANFD\\d+", PeriInfo { peri_type: "can", version: "v1" }),

    // SPI Flash
    (".*:QSPI", PeriInfo { peri_type: "qspi", version: "v1" }),
    (".*:OSPI", PeriInfo { peri_type: "ospi", version: "v1" }),
    (".*:OSPI_B\\d+", PeriInfo { peri_type: "ospi", version: "v1" }),
]);
