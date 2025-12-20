import yaml
import sys

def normalize_obj(obj):
    if isinstance(obj, dict):
        new_obj = {}
        for k, v in obj.items():
            if k == 'description': continue
            if k == 'fields' and isinstance(v, list):
                v = [f for f in v if f.get('name', '').lower() != 'reserved']
            normalized_v = normalize_obj(v)
            new_obj[k] = normalized_v
        return new_obj
    elif isinstance(obj, list):
        if all(isinstance(i, dict) and 'name' in i for i in obj):
            obj = sorted(obj, key=lambda x: x['name'])
        return [normalize_obj(i) for i in obj]
    else:
        return obj

with open(sys.argv[1], 'r') as f:
    data = yaml.safe_load(f)
print(yaml.dump(normalize_obj(data), sort_keys=True))
