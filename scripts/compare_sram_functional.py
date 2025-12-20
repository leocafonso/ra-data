import os
import yaml
import hashlib
from collections import defaultdict

def normalize_obj(obj):
    if isinstance(obj, dict):
        new_obj = {}
        for k, v in obj.items():
            if k == 'description':
                continue
            
            # Normalize field names to their bit offset to ignore naming variations
            if k == 'fields' and isinstance(v, list):
                v = [f for f in v if f.get('name', '').lower() != 'reserved']
                for f in v:
                    if 'bit_offset' in f:
                        f['name'] = f'field_{f["bit_offset"]}'
            
            # Normalize variant names to their value
            if k == 'variants' and isinstance(v, list):
                for var in v:
                    if 'value' in var:
                        var['name'] = f'val_{var["value"]}'
            
            normalized_v = normalize_obj(v)
            new_obj[k] = normalized_v
        return new_obj
    elif isinstance(obj, list):
        if all(isinstance(i, dict) and 'name' in i for i in obj):
            obj = sorted(obj, key=lambda x: x['name'])
        return [normalize_obj(i) for i in obj]
    else:
        return obj

def main():
    sram_dir = 'tmp/SRAM'
    groups = defaultdict(list)
    
    for filename in os.listdir(sram_dir):
        if filename.endswith('.yaml'):
            path = os.path.join(sram_dir, filename)
            with open(path, 'r') as f:
                data = yaml.safe_load(f)
            
            # 1. Normalize structure
            norm_data = normalize_obj(data)
            
            # 2. Normalize Enum names (replace with hash of variants)
            # This handles cases where one chip calls it 'KW' and another 'ECCPRCR_KW'
            enum_map = {}
            for k in list(norm_data.keys()):
                if k.startswith('enum/'):
                    # If the enum has only one variant, it's often a Key Code or similar
                    # that might be missing in other SVDs. We ignore it for functional grouping
                    # if it's just a single value.
                    if len(norm_data[k].get('variants', [])) <= 1:
                        norm_data.pop(k)
                        continue

                    variants_str = yaml.dump(norm_data[k], sort_keys=True)
                    h = hashlib.md5(variants_str.encode()).hexdigest()[:8]
                    enum_map[k] = f'enum/normalized_{h}'
                    norm_data[enum_map[k]] = norm_data.pop(k)
            
            # 3. Update references to enums in fields
            for k in norm_data:
                if k.startswith('fieldset/'):
                    for field in norm_data[k].get('fields', []):
                        if 'enum' in field:
                            old_enum = f'enum/{field["enum"]}'
                            if old_enum in enum_map:
                                field['enum'] = enum_map[old_enum].replace('enum/', '')
                            else:
                                # Enum was removed (single variant)
                                field.pop('enum')

            # 4. Normalize Block item names
            for k in list(norm_data.keys()):
                if k.startswith('block/'):
                    norm_data['block/normalized'] = norm_data.pop(k)

            stable_str = yaml.dump(norm_data, sort_keys=True)
            h = hashlib.sha256(stable_str.encode()).hexdigest()
            groups[h].append(filename.replace('.yaml', ''))
            
    print(f"Found {len(groups)} functionally unique SRAM variations (ignoring Reserved and Descriptions):\n")
    for i, (h, chips) in enumerate(groups.items(), 1):
        print(f"Functional Variation {i}:")
        print(f"  Chips: {', '.join(sorted(chips))}")
        print()

if __name__ == "__main__":
    main()
