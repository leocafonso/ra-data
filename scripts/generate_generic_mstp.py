import yaml

def generate_mstp(name, registers, include_lsmrwdis=False):
    data = {
        "block/MSTP": {
            "description": "Module Stop Control.",
            "items": []
        }
    }
    
    for i, reg in enumerate(registers):
        data["block/MSTP"]["items"].append({
            "name": reg,
            "description": f"Module Stop Control Register {reg[-1]}.",
            "byte_offset": i * 4,
            "fieldset": reg
        })
        
        fields = []
        for bit in range(32):
            fields.append({
                "name": f"MSTP{reg[-1]}{bit}",
                "description": f"Module Stop bit {bit}.",
                "bit_offset": bit,
                "bit_size": 1
            })
            
        data[f"fieldset/{reg}"] = {
            "description": f"Module Stop Control Register {reg[-1]}.",
            "fields": fields
        }
        
    if include_lsmrwdis:
        data["block/MSTP"]["items"].append({
            "name": "LSMRWDIS",
            "description": "Low Speed Module R/W Disable Control Register.",
            "byte_offset": len(registers) * 4,
            "bit_size": 16,
            "fieldset": "LSMRWDIS"
        })
        
        data["fieldset/LSMRWDIS"] = {
            "description": "Low Speed Module R/W Disable Control Register.",
            "bit_size": 16,
            "fields": [
                {"name": "RTCRWDIS", "description": "RTC Register R/W Enable Control.", "bit_offset": 0, "bit_size": 1},
                {"name": "WDTDIS", "description": "WDT Operate Clock Control.", "bit_offset": 1, "bit_size": 1},
                {"name": "IWDTIDS", "description": "IWDT Register Clock Control.", "bit_offset": 2, "bit_size": 1},
                {"name": "WREN", "description": "Write Enable for bits [2:0].", "bit_offset": 7, "bit_size": 1},
                {"name": "PRKEY", "description": "LSMRWDIS Key Code.", "bit_offset": 8, "bit_size": 8}
            ]
        }
        
    with open(f"data/registers/{name}.yaml", "w") as f:
        yaml.dump(data, f, sort_keys=False)

# Variation 2: MSTPCRB, MSTPCRC, MSTPCRD (RA0, etc)
generate_mstp("mstp_v1", ["MSTPCRB", "MSTPCRC", "MSTPCRD"])

# Variation 1: MSTPCRA, MSTPCRB, MSTPCRC, MSTPCRD, MSTPCRE (RA4/6/8)
generate_mstp("mstp_v2", ["MSTPCRA", "MSTPCRB", "MSTPCRC", "MSTPCRD", "MSTPCRE"])

# Variation 3: MSTPCRB, MSTPCRC, MSTPCRD + LSMRWDIS (RA2)
generate_mstp("mstp_v3", ["MSTPCRB", "MSTPCRC", "MSTPCRD"], include_lsmrwdis=True)
