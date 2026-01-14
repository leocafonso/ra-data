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
    // RA0 family
    ("R7FA0E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra0e1" }),
    ("R7FA0E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra0e2" }),
    ("R7FA0L1.*:ICU", PeriInfo { peri_type: "icu", version: "ra0l1" }),
    // RA2 family
    ("R7FA2A1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2a1" }),
    ("R7FA2A2.*:ICU", PeriInfo { peri_type: "icu", version: "ra2a2" }),
    ("R7FA2E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2e1" }),
    ("R7FA2E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra2e2" }),
    ("R7FA2E3.*:ICU", PeriInfo { peri_type: "icu", version: "ra2e3" }),
    ("R7FA2L1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2l1" }),
    ("R7FA2L2.*:ICU", PeriInfo { peri_type: "icu", version: "ra2l2" }),
    ("R7FA2T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2t1" }),
    // RA4 family
    ("R7FA4C1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4c1" }),
    ("R7FA4E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4e1" }),
    ("R7FA4E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra4e2" }),
    ("R7FA4L1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4l1" }),
    ("R7FA4M1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m1" }),
    ("R7FA4M2.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m2" }),
    ("R7FA4M3.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m3" }),
    ("R7FA4T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4t1" }),
    ("R7FA4W1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4w1" }),
    // RA6 family (some shared with RA4)
    ("R7FA6E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra6e1" }),
    ("R7FA6E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra4e2" }), // same as RA4E2
    ("R7FA6M1.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m1" }),
    ("R7FA6M2.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m2" }),
    ("R7FA6M3.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m2" }), // same as RA6M2
    ("R7FA6M4.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m3" }), // same as RA4M3
    ("R7FA6M5.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m5" }),
    ("R7FA6T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m1" }), // same as RA6M1
    ("R7FA6T2.*:ICU", PeriInfo { peri_type: "icu", version: "ra6t2" }),
    ("R7FA6T3.*:ICU", PeriInfo { peri_type: "icu", version: "ra6t3" }),
    // RA8 family
    ("R7FA8D1.*:ICU", PeriInfo { peri_type: "icu", version: "ra8d1" }),
    ("R7FA8E.*:ICU", PeriInfo { peri_type: "icu", version: "ra8e1" }),
    ("R7FA8M1.*:ICU", PeriInfo { peri_type: "icu", version: "ra8m1" }),
    ("R7FA8T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra8t1" }),
    // RKA8 family
    ("R7KA8[DMP].*:ICU", PeriInfo { peri_type: "icu", version: "rka8d2" }),
    ("R7KA8T.*:ICU", PeriInfo { peri_type: "icu", version: "rka8t2" }),

    // System mappings
    ("R7FA[46][ELMT].*:MSTP", PeriInfo { peri_type: "mstp", version: "v2" }),
    ("R7FA8.*:MSTP", PeriInfo { peri_type: "mstp", version: "v2" }),
    ("R7KA8.*:MSTP", PeriInfo { peri_type: "mstp", version: "v2" }),
    ("R7FA2.*:MSTP", PeriInfo { peri_type: "mstp", version: "v3" }),
    (".*:MSTP", PeriInfo { peri_type: "mstp", version: "v1" }),

    // SYSC mappings (System Control)
    // RA0 family - all share same structure
    ("R7FA0.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra0" }),
    // RA2 family
    ("R7FA2A1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2a1" }),
    ("R7FA2A2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2a2" }),
    ("R7FA2E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e1" }),
    ("R7FA2E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e2" }),
    ("R7FA2E3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e1" }), // same as RA2E1
    ("R7FA2L1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2l1" }),
    ("R7FA2L2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e1" }), // same as RA2E1
    ("R7FA2T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2t1" }),
    // RA4 family
    ("R7FA4C1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4c1" }),
    ("R7FA4E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4e1" }),
    ("R7FA4E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4e2" }),
    ("R7FA4L1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4l1" }),
    ("R7FA4M1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4m1" }),
    ("R7FA4M2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4m2" }),
    ("R7FA4M3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4m3" }),
    ("R7FA4T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4t1" }),
    ("R7FA4W1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4w1" }),
    // RA6 family
    ("R7FA6E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6e1" }),
    ("R7FA6E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4e2" }), // same as RA4E2
    ("R7FA6M1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m1" }),
    ("R7FA6M2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m2" }),
    ("R7FA6M3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m2" }), // same as RA6M2
    ("R7FA6M4.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m4" }),
    ("R7FA6M5.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m5" }),
    ("R7FA6T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6t1" }),
    ("R7FA6T2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6t2" }),
    ("R7FA6T3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6t3" }),
    // RA8 family
    ("R7FA8D1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8d1" }),
    ("R7FA8E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8e1" }),
    ("R7FA8E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8e2" }),
    ("R7FA8M1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8m1" }),
    ("R7FA8T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8t1" }),
    // RKA8 family
    ("R7KA8D2.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8d2" }),
    ("R7KA8M2.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8m2" }),
    ("R7KA8P1.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8p1" }),
    ("R7KA8T2.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8t2" }),
]);

