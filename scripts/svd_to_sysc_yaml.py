#!/usr/bin/env python3
"""
Convert an SVD's SYSC peripheral block into the YAML format used by
data/registers/sysc/*.yaml.

Usage:
    python3 svd_to_sysc_yaml.py <SVD_PATH> <OUTPUT_YAML> [--description "..."]

Strategy:
- Walk every <register> under the SYSC peripheral.
- Emit one block/SYSC item per register, one fieldset/<NAME> per register,
  and one enum/<NAME> per enumerated field. Enum names are taken from the
  field name; if two fields in different registers have the same name but
  different variants, the second is prefixed with the register name to
  disambiguate.
- Bit-size on registers is omitted when 32 (the default) and emitted
  otherwise, matching the convention in the existing YAMLs.
"""

from __future__ import annotations
import sys
import argparse
import xml.etree.ElementTree as ET
from typing import Optional

# ----------------------------------------------------------------------------
# SVD parsing
# ----------------------------------------------------------------------------

def text(el, tag, default=None):
    child = el.find(tag)
    if child is None or child.text is None:
        return default
    return child.text.strip()


def parse_int(s):
    if s is None:
        return None
    s = s.strip()
    if s.startswith('0x') or s.startswith('0X'):
        return int(s, 16)
    if s.startswith('#'):
        return int(s[1:], 2)
    return int(s)


class Field:
    def __init__(self, name, description, lsb, msb, access, variants):
        self.name = name
        self.description = description
        self.bit_offset = lsb
        self.bit_size = msb - lsb + 1
        self.access = access
        self.variants = variants  # list of (name, description, value) or None

    def __repr__(self):
        return f'Field({self.name}, [{self.bit_offset+self.bit_size-1}:{self.bit_offset}])'


class Register:
    def __init__(self, name, description, offset, size, access, fields):
        self.name = name
        self.description = description
        self.offset = offset
        self.size = size  # in bits
        self.access = access
        self.fields = fields


def find_sysc_peripheral(root) -> ET.Element:
    for p in root.iter('peripheral'):
        name = text(p, 'name')
        if name == 'SYSC' or name == 'SYSTEM':
            return p
    raise RuntimeError("SYSC/SYSTEM peripheral not found in SVD")


def parse_field(f_el) -> Field:
    name = text(f_el, 'name')
    description = text(f_el, 'description', '')
    lsb_el = f_el.find('lsb')
    msb_el = f_el.find('msb')
    bo_el = f_el.find('bitOffset')
    bw_el = f_el.find('bitWidth')
    if lsb_el is not None and msb_el is not None:
        lsb = parse_int(lsb_el.text)
        msb = parse_int(msb_el.text)
    elif bo_el is not None and bw_el is not None:
        lsb = parse_int(bo_el.text)
        msb = lsb + parse_int(bw_el.text) - 1
    else:
        raise RuntimeError(f"Field {name} has no bit range")
    access = text(f_el, 'access', None)
    variants = None
    enums = f_el.find('enumeratedValues')
    if enums is not None:
        collected = []
        for v in enums.findall('enumeratedValue'):
            vn = text(v, 'name')
            vd = text(v, 'description', '')
            vv_text = text(v, 'value')
            if vv_text is None:
                continue
            vv = parse_int(vv_text)
            collected.append((vn, vd, vv))
        # Empty <enumeratedValues></enumeratedValues> ⇒ treat as no enum.
        variants = collected if collected else None
    return Field(name, description, lsb, msb, access, variants)


def expand_dim_index(dim_index_text: str) -> list[str]:
    """Parse SVD <dimIndex> like '1,2' or '0-3' or 'A,B,C' into a list of strings."""
    s = dim_index_text.strip()
    if '-' in s and ',' not in s:
        a, b = s.split('-')
        try:
            return [str(i) for i in range(int(a), int(b) + 1)]
        except ValueError:
            # Letter range like A-D
            return [chr(c) for c in range(ord(a), ord(b) + 1)]
    return [t.strip() for t in s.split(',')]


def parse_register(r_el) -> list[Register]:
    """Return a list because SVD <dim> arrays expand into multiple entries."""
    name = text(r_el, 'name')
    description = text(r_el, 'description', '')
    offset = parse_int(text(r_el, 'addressOffset'))
    size = parse_int(text(r_el, 'size', '32'))
    access = text(r_el, 'access', None)
    fields_el = r_el.find('fields')
    fields = []
    if fields_el is not None:
        for f in fields_el.findall('field'):
            fields.append(parse_field(f))
    fields.sort(key=lambda f: f.bit_offset)

    # Handle SVD <dim> arrays: expand `LVD%sCR1` with dimIndex `1,2` into LVD1CR1, LVD2CR1.
    dim = text(r_el, 'dim')
    dim_increment = text(r_el, 'dimIncrement')
    dim_index = text(r_el, 'dimIndex')
    if dim is None or '%s' not in (name or ''):
        return [Register(name, description, offset, size, access, fields)]
    n = parse_int(dim)
    stride = parse_int(dim_increment) if dim_increment else 0
    if dim_index:
        indexes = expand_dim_index(dim_index)
    else:
        indexes = [str(i) for i in range(n)]
    if len(indexes) != n:
        # Fall back to numeric range
        indexes = [str(i) for i in range(n)]
    out = []
    for i, idx in enumerate(indexes):
        rn = name.replace('%s', idx)
        rd = (description or '').replace('%s', idx)
        out.append(Register(rn, rd, offset + i * stride, size, access, fields))
    return out


def parse_sysc(svd_path) -> list[Register]:
    tree = ET.parse(svd_path)
    root = tree.getroot()
    sysc = find_sysc_peripheral(root)
    regs_el = sysc.find('registers')
    if regs_el is None:
        return []
    regs = []
    for r in regs_el.findall('register'):
        regs.extend(parse_register(r))
    regs.sort(key=lambda r: r.offset)
    return regs


# ----------------------------------------------------------------------------
# YAML emission (hand-rolled to control formatting)
# ----------------------------------------------------------------------------

_YAML_UNSAFE_CHARS = (':', '#', '"', "'", '&', '*', '!', '|', '>', '%', '@', '`', '{', '}', '[', ']', ',')


def yaml_safe_inline(s: str) -> str:
    """Return a YAML-safe inline scalar. Quote when necessary; otherwise pass through."""
    if s is None:
        return "''"
    s = ' '.join(s.split())  # collapse whitespace
    if not s:
        return "''"
    needs_quote = (
        any(c in s for c in _YAML_UNSAFE_CHARS)
        or s.lower() in ('true', 'false', 'yes', 'no', 'null', '~')
        or s[0] in ('-', '?', '!', ' ')
        or s[-1] in (' ',)
    )
    if not needs_quote:
        return s
    return "'" + s.replace("'", "''") + "'"


def emit_field_description(d: str, indent: int) -> list[str]:
    """Emit a description for a field, possibly multi-line."""
    pad = ' ' * indent
    if not d:
        return []
    d = d.strip()
    if '\n' not in d and len(d) < 80 and not d.startswith('-') and ':' not in d.split(' ', 1)[0]:
        return [f"{pad}description: {d}"]
    # Multi-line: use block scalar style with single quotes
    quoted = "'" + d.replace("'", "''").replace('\n', '\n\n      ') + "\n\n      '"
    out = [f"{pad}description: {quoted.split(chr(10))[0]}"]
    for line in quoted.split('\n')[1:]:
        out.append(line)
    return out


class YamlEmitter:
    def __init__(self):
        self.lines: list[str] = []
        # name -> list of (variants_tuple, used_count)
        # to detect collisions
        self.enum_registry: dict[str, tuple] = {}

    def write(self, line: str):
        self.lines.append(line)

    def get_yaml(self) -> str:
        return '\n'.join(self.lines) + '\n'


def emit_block(em: YamlEmitter, regs: list[Register], description: str):
    em.write('block/SYSC:')
    em.write(f'  description: {yaml_safe_inline(description)}')
    em.write('  items:')
    for r in regs:
        em.write(f'  - name: {r.name}')
        if r.description:
            desc = r.description.rstrip('.') + '.'
            em.write(f'    description: {yaml_safe_inline(desc)}')
        em.write(f'    byte_offset: {r.offset}')
        if r.size != 32:
            em.write(f'    bit_size: {r.size}')
        if r.access == 'read-only':
            em.write('    access: Read')
        elif r.access == 'write-only':
            em.write('    access: Write')
        if r.fields:
            em.write(f'    fieldset: {r.name}')


def variants_tuple(field: Field):
    if field.variants is None:
        return None
    return tuple((n, v) for (n, _d, v) in field.variants)


def resolve_enum_name(em: YamlEmitter, register_name: str, field: Field) -> Optional[str]:
    """Return the enum name to use, registering it. Returns None if no enum."""
    if field.variants is None:
        return None
    # 1-bit fields are rendered as `bool` by chiptool when no enum is attached.
    # The existing hand-curated YAMLs (sysc_v6, sysc_v7) follow this convention,
    # and consumers (e.g. embassy-ra) expect `set_prc0(bool)`, not `set_prc0(Prc0)`.
    if field.bit_size == 1:
        return None
    # Single-variant "enums" in SVDs are usually magic-value key codes
    # (e.g. PRKEY = 0xA5). Treat as plain integer field.
    if len(field.variants) <= 1:
        return None
    candidates = [field.name, f'{register_name}_{field.name}']
    sig = variants_tuple(field)
    for cand in candidates:
        existing = em.enum_registry.get(cand)
        if existing is None:
            em.enum_registry[cand] = (sig, field.bit_size, [(n, d, v) for (n, d, v) in field.variants])
            return cand
        if existing[0] == sig and existing[1] == field.bit_size:
            return cand
    # Fallback: attach numeric suffix
    i = 2
    while True:
        cand = f'{register_name}_{field.name}_{i}'
        if cand not in em.enum_registry:
            em.enum_registry[cand] = (sig, field.bit_size, [(n, d, v) for (n, d, v) in field.variants])
            return cand
        i += 1


def emit_fieldsets(em: YamlEmitter, regs: list[Register]):
    for r in regs:
        if not r.fields:
            continue
        em.write(f'fieldset/{r.name}:')
        if r.description:
            desc = r.description.rstrip('.') + '.'
            em.write(f'  description: {yaml_safe_inline(desc)}')
        em.write(f'  bit_size: {r.size}')
        em.write('  fields:')
        for f in r.fields:
            em.write(f'  - name: {f.name}')
            if f.description:
                em.write(f'    description: {yaml_safe_inline(f.description)}')
            em.write(f'    bit_offset: {f.bit_offset}')
            em.write(f'    bit_size: {f.bit_size}')
            enum_name = resolve_enum_name(em, r.name, f)
            if enum_name:
                em.write(f'    enum: {enum_name}')


def emit_enums(em: YamlEmitter):
    for name in sorted(em.enum_registry.keys()):
        _sig, bit_size, variants = em.enum_registry[name]
        em.write(f'enum/{name}:')
        em.write(f'  bit_size: {bit_size}')
        em.write('  variants:')
        for (vn, vd, vv) in variants:
            em.write(f'  - name: {vn}')
            if vd:
                em.write(f'    description: {yaml_safe_inline(vd)}')
            em.write(f'    value: {vv}')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('svd', help='Path to .svd file')
    ap.add_argument('output', help='Output YAML path')
    ap.add_argument('--description', default='System Control.', help='block/SYSC description string')
    args = ap.parse_args()

    regs = parse_sysc(args.svd)
    em = YamlEmitter()
    emit_block(em, regs, args.description)
    emit_fieldsets(em, regs)
    emit_enums(em)
    with open(args.output, 'w') as f:
        f.write(em.get_yaml())
    print(f'Wrote {len(regs)} registers, {len(em.enum_registry)} enums to {args.output}', file=sys.stderr)


if __name__ == '__main__':
    main()
