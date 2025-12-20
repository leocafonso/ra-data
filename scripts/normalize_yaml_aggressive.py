import yaml
import sys
import hashlib

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

def full_normalize(path):
    with open(path, 'r') as f:
        data = yaml.safe_load(f)
    norm_data = normalize_obj(data)
    enum_map = {}
    for k in list(norm_data.keys()):
        if k.startswith('enum/'):
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
    for k in list(norm_data.keys()):
        if k.startswith('block/'):
            norm_data['block/normalized'] = norm_data.pop(k)
    return norm_data

print(yaml.dump(full_normalize(sys.argv[1]), sort_keys=True))
