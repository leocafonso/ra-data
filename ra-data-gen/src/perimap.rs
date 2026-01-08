use crate::util::RegexMap;

pub struct PeriInfo {
    pub peri_type: &'static str,
    pub version: &'static str,
}

pub static PERIMAP: RegexMap<PeriInfo> = RegexMap::new(&[
    // GPIO mappings
    ("R7FA0.*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v3" }),
    ("R7FA8.*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v2" }),
    ("R7FA6T2.*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v2" }),
    (".*:PORT\\d+", PeriInfo { peri_type: "gpio", version: "v1" }),
    (".*:PFS", PeriInfo { peri_type: "pfs", version: "v1" }),

    // Timer mappings (General PWM Timer)
    ("R7FA4E2.*:GPT\\d+", PeriInfo { peri_type: "timer", version: "v2" }),
    ("R7FA8.*:GPT\\d+", PeriInfo { peri_type: "timer", version: "v2" }),
    ("R7FA6[MT]1.*:GPT32E[H]?[0-7]", PeriInfo { peri_type: "timer", version: "v4" }),
    ("R7FA6[MT]1.*:GPT32\\d+", PeriInfo { peri_type: "timer", version: "v3" }),
    ("R7FA2.*:GPT(?:32|16)\\d+", PeriInfo { peri_type: "timer", version: "v5" }),
    (".*:GPT\\d+", PeriInfo { peri_type: "timer", version: "v1" }),

    // ICU mappings (Interrupt Controller Unit)
    ("R7FA0.*:ICU", PeriInfo { peri_type: "icu", version: "v1" }),
    ("R7FA2.*:ICU", PeriInfo { peri_type: "icu", version: "v2" }),
    ("R7FA[46].*:ICU", PeriInfo { peri_type: "icu", version: "v3" }),
    ("R7FA8.*:ICU(?:_NS)?", PeriInfo { peri_type: "icu", version: "v5" }), // RA8 ICU in rzone is actually ICU_COMMON
    ("R7FA8.*:ICU_COMMON(?:_NS)?", PeriInfo { peri_type: "icu", version: "v5" }),

    // System mappings
    ("R7FA[46][EGLMT].*:MSTP", PeriInfo { peri_type: "mstp", version: "v2" }),
    ("R7FA8.*:MSTP", PeriInfo { peri_type: "mstp", version: "v2" }),
    ("R7KA8.*:MSTP", PeriInfo { peri_type: "mstp", version: "v2" }),
    ("R7FA2.*:MSTP", PeriInfo { peri_type: "mstp", version: "v3" }),
    (".*:MSTP", PeriInfo { peri_type: "mstp", version: "v1" }),
    ("R7FA0E1.*:SYSTEM", PeriInfo { peri_type: "system", version: "v1" }),
    ("R7FA6M5.*:SYSTEM", PeriInfo { peri_type: "system", version: "v3" }),
    ("R7FA6M1.*:SYSTEM", PeriInfo { peri_type: "system", version: "v4" }),
    ("R7FA8.*:SYSTEM", PeriInfo { peri_type: "system", version: "v5" }),
    (".*:SYSTEM", PeriInfo { peri_type: "system", version: "v2" }), // Default to v2
]);

