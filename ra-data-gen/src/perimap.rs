use crate::util::RegexMap;

pub struct PeriInfo {
    pub peri_type: &'static str,
    pub version: &'static str,
    pub block: &'static str,
}

pub static PERIMAP: RegexMap<PeriInfo> = RegexMap::new(&[
    // PORT (GPIO) mappings
    // RA0 family - each subfamily has different pin configs
    ("R7FA0E1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT" }),
    ("R7FA0E2.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT" }),
    ("R7FA0L1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT" }),
    // RA2 family
    ("R7FA2A1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA2A2.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA2[ELT].*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    // RA4 family
    ("R7FA4M1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA4W1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA4[CELT].*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    // RA6 family
    ("R7FA6[MT]1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }), // same as RA4W1
    ("R7FA6T2.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA6[EM].*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA6T3.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    // RA8 family
    ("R7FA8D1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA8E.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    ("R7FA8[MT]1.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),
    // RKA8 family
    ("R7KA8.*:PORT\\d+", PeriInfo { peri_type: "port", version: "v1", block: "PORT_extended" }),

    // PFS (Pin Function Select)
    // RA0 family uses 16-bit PFS_A (normalized to PFS in generate.rs)
    ("R7FA0.*:PFS", PeriInfo { peri_type: "pfs", version: "v2", block: "PFS" }),
    // All other chips use 32-bit PFS
    (".*:PFS", PeriInfo { peri_type: "pfs", version: "v1", block: "PFS" }),

    // Timer mappings (General PWM Timer) - consolidated to gpt_v1.yaml
    // TODO: GPT16 and GPT32 share the same register layout. The only difference is that
    // GPT16 counter and compare registers use only 16 bits of the 32-bit registers.
    // Consider tracking this bit-width information separately (e.g., in chip data or a lookup table).
    // RA2 family
    ("R7FA2A1.*:GPT(?:1|2|3|4|5|6)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA2A1.*:GPT0", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA2A2.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA2E1.*:GPT(?:4|5|6|7|8|9)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA2E1.*:GPT0", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA2E2.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA2E3.*:GPT(?:4|5|6|7|8|9)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA2E3.*:GPT0", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA2L1.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA2L1.*:GPT(?:4|5|6|7|8|9)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA2L2.*:GPT(?:4|5|6|7|8|9)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA2L2.*:GPT0", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA2T1.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    // RA4 family
    ("R7FA4C1.*:GPT(?:0|1)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA4C1.*:GPT(?:2|3|4|5)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA4E1.*:GPT(?:1|2)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA4E1.*:GPT(?:4|5)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA4E2.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT16E" }),
    ("R7FA4L1.*:GPT(?:0|1)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA4L1.*:GPT(?:2|3|4|5)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA4M1.*:GPT(?:0|1)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA4M1.*:GPT(?:2|3|4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA4M2.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA4M2.*:GPT(?:4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA4M3.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA4M3.*:GPT(?:4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA4T1.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT16E" }),
    ("R7FA4W1.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA4W1.*:GPT(?:4|5|8)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    // RA6 family
    ("R7FA6E1.*:GPT(?:1|2)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6E1.*:GPT(?:4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA6E2.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT16E" }),
    ("R7FA6M1.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }), // gpt32eh
    ("R7FA6M1.*:GPT(?:4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }),
    ("R7FA6M1.*:GPT(?:8|9|10|11|12)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6M2.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }), // gpt32eh
    ("R7FA6M2.*:GPT(?:4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }),
    ("R7FA6M2.*:GPT(?:8|9|10|11|12|13)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6M3.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }), // gpt32eh
    ("R7FA6M3.*:GPT(?:4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }),
    ("R7FA6M3.*:GPT(?:8|9|10|11|12|13)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6M4.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6M4.*:GPT(?:4|5|6|7|8|9)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA6M5.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6M5.*:GPT(?:4|5|6|7|8|9)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA6T1.*:GPT(?:0|1|2|3)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }), // gpt32eh
    ("R7FA6T1.*:GPT(?:4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT32E" }),
    ("R7FA6T1.*:GPT(?:8|9|10|11|12)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6T2.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA6T3.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT16E" }),
    // RA8 family
    ("R7FA8D1.*:GPT(?:0|1|2|3|4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA8D1.*:GPT(?:8|9|10|11|12|13)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA8E1.*:GPT(?:0|1|2|3|4|5)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA8E1.*:GPT(?:10|11|12|13)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA8E2.*:GPT(?:0|1|2|3|4|5)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA8E2.*:GPT(?:10|11|12|13)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA8M1.*:GPT(?:0|1|2|3|4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA8M1.*:GPT(?:8|9|10|11|12|13)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    ("R7FA8T1.*:GPT(?:0|1|2|3|4|5|6|7)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32
    ("R7FA8T1.*:GPT(?:8|9|10|11|12|13)", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt16
    // RKA8 family
    ("R7KA8P1.*:GPT\\d+", PeriInfo { peri_type: "gpt", version: "v1", block: "GPT" }), // gpt32

    // ICU mappings (Interrupt Controller Unit)
    // RA0 family
    ("R7FA0E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra0e1", block: "ICU" }),
    ("R7FA0E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra0e2", block: "ICU" }),
    ("R7FA0L1.*:ICU", PeriInfo { peri_type: "icu", version: "ra0l1", block: "ICU" }),
    // RA2 family
    ("R7FA2A1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2a1", block: "ICU" }),
    ("R7FA2A2.*:ICU", PeriInfo { peri_type: "icu", version: "ra2a2", block: "ICU" }),
    ("R7FA2E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2e1", block: "ICU" }),
    ("R7FA2E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra2e2", block: "ICU" }),
    ("R7FA2E3.*:ICU", PeriInfo { peri_type: "icu", version: "ra2e3", block: "ICU" }),
    ("R7FA2L1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2l1", block: "ICU" }),
    ("R7FA2L2.*:ICU", PeriInfo { peri_type: "icu", version: "ra2l2", block: "ICU" }),
    ("R7FA2T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra2t1", block: "ICU" }),
    // RA4 family
    ("R7FA4C1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4c1", block: "ICU" }),
    ("R7FA4E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4e1", block: "ICU" }),
    ("R7FA4E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra4e2", block: "ICU" }),
    ("R7FA4L1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4l1", block: "ICU" }),
    ("R7FA4M1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m1", block: "ICU" }),
    ("R7FA4M2.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m2", block: "ICU" }),
    ("R7FA4M3.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m3", block: "ICU" }),
    ("R7FA4T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4t1", block: "ICU" }),
    ("R7FA4W1.*:ICU", PeriInfo { peri_type: "icu", version: "ra4w1", block: "ICU" }),
    // RA6 family (some shared with RA4)
    ("R7FA6E1.*:ICU", PeriInfo { peri_type: "icu", version: "ra6e1", block: "ICU" }),
    ("R7FA6E2.*:ICU", PeriInfo { peri_type: "icu", version: "ra4e2", block: "ICU" }), // same as RA4E2
    ("R7FA6M1.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m1", block: "ICU" }),
    ("R7FA6M2.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m2", block: "ICU" }),
    ("R7FA6M3.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m2", block: "ICU" }), // same as RA6M2
    ("R7FA6M4.*:ICU", PeriInfo { peri_type: "icu", version: "ra4m3", block: "ICU" }), // same as RA4M3
    ("R7FA6M5.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m5", block: "ICU" }),
    ("R7FA6T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra6m1", block: "ICU" }), // same as RA6M1
    ("R7FA6T2.*:ICU", PeriInfo { peri_type: "icu", version: "ra6t2", block: "ICU" }),
    ("R7FA6T3.*:ICU", PeriInfo { peri_type: "icu", version: "ra6t3", block: "ICU" }),
    // RA8 family
    ("R7FA8D1.*:ICU", PeriInfo { peri_type: "icu", version: "ra8d1", block: "ICU" }),
    ("R7FA8E.*:ICU", PeriInfo { peri_type: "icu", version: "ra8e1", block: "ICU" }),
    ("R7FA8M1.*:ICU", PeriInfo { peri_type: "icu", version: "ra8m1", block: "ICU" }),
    ("R7FA8T1.*:ICU", PeriInfo { peri_type: "icu", version: "ra8t1", block: "ICU" }),
    // RKA8 family
    ("R7KA8[DMP].*:ICU", PeriInfo { peri_type: "icu", version: "rka8d2", block: "ICU" }),
    ("R7KA8T.*:ICU", PeriInfo { peri_type: "icu", version: "rka8t2", block: "ICU" }),

    // System mappings
    ("R7FA[46][ELMT].*:MSTP", PeriInfo { peri_type: "mstp", version: "v2", block: "MSTP" }),
    ("R7FA8.*:MSTP", PeriInfo { peri_type: "mstp", version: "v2", block: "MSTP" }),
    ("R7KA8.*:MSTP", PeriInfo { peri_type: "mstp", version: "v2", block: "MSTP" }),
    ("R7FA2.*:MSTP", PeriInfo { peri_type: "mstp", version: "v3", block: "MSTP" }),
    (".*:MSTP", PeriInfo { peri_type: "mstp", version: "v1", block: "MSTP" }),

    // SYSC mappings (System Control)
    // RA0 family - all share same structure
    ("R7FA0.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra0", block: "SYSC" }),
    // RA2 family
    ("R7FA2A1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2a1", block: "SYSC" }),
    ("R7FA2A2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2a2", block: "SYSC" }),
    ("R7FA2E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e1", block: "SYSC" }),
    ("R7FA2E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e2", block: "SYSC" }),
    ("R7FA2E3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e1", block: "SYSC" }), // same as RA2E1
    ("R7FA2L1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2l1", block: "SYSC" }),
    ("R7FA2L2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2e1", block: "SYSC" }), // same as RA2E1
    ("R7FA2T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra2t1", block: "SYSC" }),
    // RA4 family
    ("R7FA4C1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4c1", block: "SYSC" }),
    ("R7FA4E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4e1", block: "SYSC" }),
    ("R7FA4E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4e2", block: "SYSC" }),
    ("R7FA4L1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4l1", block: "SYSC" }),
    ("R7FA4M1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4m1", block: "SYSC" }),
    ("R7FA4M2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4m2", block: "SYSC" }),
    ("R7FA4M3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4m3", block: "SYSC" }),
    ("R7FA4T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4t1", block: "SYSC" }),
    ("R7FA4W1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4w1", block: "SYSC" }),
    // RA6 family
    ("R7FA6E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6e1", block: "SYSC" }),
    ("R7FA6E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra4e2", block: "SYSC" }), // same as RA4E2
    ("R7FA6M1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m1", block: "SYSC" }),
    ("R7FA6M2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m2", block: "SYSC" }),
    ("R7FA6M3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m2", block: "SYSC" }), // same as RA6M2
    ("R7FA6M4.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m4", block: "SYSC" }),
    ("R7FA6M5.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6m5", block: "SYSC" }),
    ("R7FA6T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6t1", block: "SYSC" }),
    ("R7FA6T2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6t2", block: "SYSC" }),
    ("R7FA6T3.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra6t3", block: "SYSC" }),
    // RA8 family
    ("R7FA8D1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8d1", block: "SYSC" }),
    ("R7FA8E1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8e1", block: "SYSC" }),
    ("R7FA8E2.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8e2", block: "SYSC" }),
    ("R7FA8M1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8m1", block: "SYSC" }),
    ("R7FA8T1.*:SYSC", PeriInfo { peri_type: "sysc", version: "ra8t1", block: "SYSC" }),
    // RKA8 family
    ("R7KA8D2.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8d2", block: "SYSC" }),
    ("R7KA8M2.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8m2", block: "SYSC" }),
    ("R7KA8P1.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8p1", block: "SYSC" }),
    ("R7KA8T2.*:SYSC", PeriInfo { peri_type: "sysc", version: "rka8t2", block: "SYSC" }),
]);
