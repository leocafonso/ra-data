import os
import yaml
import hashlib
from collections import defaultdict

def normalize_obj(obj):
    if isinstance(obj, dict):
        new_obj = {}
        for k, v in obj.items():
            if k == 'description': continue
            if k == 'fields' and isinstance(v, list):
                v = [f for f in v if f.get('name', '').lower() != 'reserved']
                for f in v:
                    if 'bit_offset' in f: f['name'] = f'field_{f["bit_offset"]}'
            if k == 'variants' and isinstance(v, list):
                for var in v:
                    if 'value' in var: var['name'] = f'val_{var["value"]}'
            normalized_v = normalize_obj(v)
            new_obj[k] = normalized_v
        return new_obj
    elif isinstance(obj, list):
        if all(isinstance(i, dict) and 'name' in i for i in obj):
            obj = sorted(obj, key=lambda x: x['name'])
        return [normalize_obj(i) for i in obj]
    else:
        return obj

def get_functional_hash(path):
    with open(path, 'r') as f:
        data = yaml.safe_load(f)
    norm_data = normalize_obj(data)
    enum_map = {}
    for k in list(norm_data.keys()):
        if k.startswith('enum/'):
            if len(norm_data[k].get('variants', [])) <= 1:
                norm_data.pop(k)
                continue
            variants_str = yaml.dump(norm_data[k], sort_keys=True)
            h = hashlib.md5(variants_str.encode()).hexdigest()[:8]
            enum_map[k] = f'enum/normalized_{h}'
            norm_data[enum_map[k]] = norm_data.pop(k)
    for k in norm_data:
        if k.startswith('fieldset/'):
            for field in norm_data[k].get('fields', []):
                if 'enum' in field:
                    old_enum = f'enum/{field["enum"]}'
                    if old_enum in enum_map:
                        field['enum'] = enum_map[old_enum].replace('enum/', '')
                    else:
                        field.pop('enum')
    for k in list(norm_data.keys()):
        if k.startswith('block/'):
            norm_data['block/normalized'] = norm_data.pop(k)
    stable_str = yaml.dump(norm_data, sort_keys=True)
    return hashlib.sha256(stable_str.encode()).hexdigest()

def main():
    sram_dir = 'tmp/SRAM'
    groups = defaultdict(list)
    for filename in os.listdir(sram_dir):
        if filename.endswith('.yaml'):
            path = os.path.join(sram_dir, filename)
            h = get_functional_hash(path)
            groups[h].append(filename.replace('.yaml', ''))
            
    sorted_hashes = sorted(groups.keys(), key=lambda x: sorted(groups[x])[0])
    
    mapping = {}
    for i, h in enumerate(sorted_hashes, 1):
        for chip in groups[h]:
            mapping[chip] = f"SRAM_grp{i}.yaml"
            
    with open('data/sram_mapping.yaml', 'w') as f:
        yaml.dump(mapping, f)
    print("Created data/sram_mapping.yaml")

if __name__ == "__main__":
    main()
