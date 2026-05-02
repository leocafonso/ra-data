# SYSC YAML versions

The SYSC peripheral is split across multiple YAML files by **IP-block layout**, not by chip family. Renesas reuses SYSC IP across families: RA4M2 shares its layout with RA6E2 (not with RA4M1); RA6T1 shares with RA6M1/M2/M3 (not with RA6T2/T3).

The canonical chip → version mapping lives in [`ra-data-gen/src/perimap.rs`](../../../ra-data-gen/src/perimap.rs) (search for `SYSC mappings`). This README is the human-readable companion.

## Versions

| File | Class | Distinguishing signature | Chips |
|------|-------|--------------------------|-------|
| `sysc_v1.yaml` | RA2 | No PLL hardware. MEMWAIT[0] (1-bit) in SYSC. MOMCR.MODRV1[3] (1-bit drive). | All RA2 (R7FA2*) |
| `sysc_v2.yaml` | RA4M1 legacy | `PLLCCR2` 8-bit (PLODIV[6:7], PLLMUL[0:4]). No 16-bit `PLLCCR`. MEMWAIT[0] in SYSC. MOMCR.MODRV1[3]. | R7FA4M1, R7FA4W1 |
| `sysc_v3.yaml` | RA6M1/M2/M3 | 16-bit `PLLCCR` (PLIDIV[0:1] / PLSRCSEL[4] / PLLMUL[8:13]). No PLLCCR2. No MEMWAIT in SYSC. MOMCR has **AUTODRVEN[7]** and uses **MODRV0[4:5]**. | R7FA6M1, R7FA6M2, R7FA6M3, R7FA6T1 |
| `sysc_v4.yaml` | "modern" | Same `PLLCCR` as v3 but MOMCR uses **MODRV[4:5]** (no AUTODRVEN). `CKSEL.V_101 = PLL`. | R7FA4M2, R7FA4M3, R7FA4E1, R7FA4E2, R7FA4T1, R7FA6E1, R7FA6E2, R7FA6M4, R7FA6M5, R7FA6T2, R7FA6T3 |
| `sysc_v5.yaml` | RA4L1/C1 | 16-bit `PLLCCR` but PLLMUL is **5 bits** (`[8:12]`, not `[8:13]`). MEMWAIT is **2 bits** (`[0:1]`) in SYSC. MOMCR.MODRV1[3]. | R7FA4L1, R7FA4C1 |
| `sysc_v6.yaml` | RA8 | `PLLCCR` widened plus `PLLCCR2` with `PLODIVP/Q/R`. Distinct MOMCR with AGC. | All RA8 (R7FA8*), RKA8 |
| `sysc_v7.yaml` | RA0 | Completely different register layout (registers at 0x800+). | All RA0 (R7FA0*) |

## How the mapping was determined

For every chip with an SVD in `sources/svd/`, the four most-disambiguating SYSC registers were diffed: `PLLCCR`, `PLLCCR2`, `MEMWAIT`, `MOMCR`. Chips with identical layouts across all four were grouped into the same version. The script below reproduces this analysis:

```bash
python3 << 'EOF'
import re, os, glob
SVD_DIR = 'sources/svd'
def find_reg(c, name):
    s = c.find(f'<name>{name}</name>')
    if s < 0: return 'absent'
    e = c.find('</register>', s)
    body = c[s:e]
    sz = re.search(r'<size>(\d+)</size>', body)
    fields = re.findall(r'<name>([A-Z0-9_]+)</name>\s*(?:<description>[^<]*</description>\s*)?<lsb>(\d+)</lsb>\s*<msb>(\d+)</msb>', body)
    return f"{sz.group(1) if sz else '?'}b: " + ','.join(sorted(f"{n}[{l}:{m}]" for n,l,m in fields[:8]))
for path in sorted(glob.glob(f'{SVD_DIR}/*.svd')):
    chip = os.path.basename(path).replace('.svd', '')
    with open(path) as f: c = f.read()
    print(f'{chip:12} PLLCCR={find_reg(c,"PLLCCR"):60} PLLCCR2={find_reg(c,"PLLCCR2"):35} MEMWAIT={find_reg(c,"MEMWAIT"):20} MOMCR={find_reg(c,"MOMCR")}')
EOF
```

## Adding a new chip

1. Run the diff script above on the new chip's SVD.
2. Find the existing version whose signature matches all four registers exactly. Add the chip to that version's row in this table.
3. Add a `("R7FA<chip>.*:SYSC", PeriInfo { … version: "vN" … })` line to `ra-data-gen/src/perimap.rs`, ordered before any catch-all pattern that would otherwise capture it.
4. If no existing version matches, generate a new YAML from the chip's SVD via `python3 scripts/svd_to_sysc_yaml.py <svd> data/registers/sysc/sysc_v<N+1>.yaml --description "…"`, add a row here, and wire it in `perimap.rs`.

## Generating a YAML from an SVD

The YAML files in this directory are produced from SVDs by `scripts/svd_to_sysc_yaml.py`. To regenerate one:

```bash
python3 scripts/svd_to_sysc_yaml.py \
  sources/svd/R7FA6E2BB.svd \
  data/registers/sysc/sysc_v4.yaml \
  --description "System Control - 'modern' RA4/RA6 class. ..."
```

Hand-edits to a YAML may be lost if the file is regenerated. If a hand-edit is necessary (e.g., to fix an SVD bug), document it in a comment near the affected entry.
