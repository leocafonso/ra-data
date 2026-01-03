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
                # For MSTP, we want to keep the bit names if possible, but let's see if they match
                # Actually, MSTP bits are usually named MSTPXY where X is register and Y is bit.
                # Let's just normalize them to bit_offset for comparison.
                v = [f for f in v if f.get('name', '').lower() != 'reserved']
                for f in v:
                    if 'bit_offset' in f:
                        # We keep the bit_offset and bit_size, but normalize the name
                        f['name'] = f'bit_{f["bit_offset"]}'
            
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

def get_structural_hash(data):
    if 'block/MSTP' not in data:
        return "no_block"
    block = data['block/MSTP']
    items = block.get('items', [])
    # Only keep name and byte_offset for structural comparison
    struct = [(item.get('name'), item.get('byte_offset')) for item in items]
    s = str(sorted(struct))
    return hashlib.sha256(s.encode()).hexdigest()

def main():
    mstp_dir = 'tmp/MSTP'
    groups = defaultdict(list)
    
    for filename in os.listdir(mstp_dir):
        if filename.endswith('.yaml'):
            path = os.path.join(mstp_dir, filename)
            with open(path, 'r') as f:
                try:
                    data = yaml.safe_load(f)
                    h = get_structural_hash(data)
                    groups[h].append(filename.replace('.yaml', ''))
                except Exception as e:
                    print(f"Error processing {filename}: {e}")
            
    print(f"Found {len(groups)} structurally unique MSTP variations:\n")
    for i, (h, chips) in enumerate(sorted(groups.items(), key=lambda x: len(x[1]), reverse=True), 1):
        print(f"Variation {i} (Hash: {h[:8]}..., Count: {len(chips)}):")
        # Print the structure for the first chip in the group
        first_chip = chips[0]
        with open(os.path.join(mstp_dir, first_chip + '.yaml'), 'r') as f:
            data = yaml.safe_load(f)
            items = data['block/MSTP'].get('items', [])
            for item in items:
                print(f"    - {item.get('name')}: {item.get('byte_offset')}")
        print(f"  Chips: {', '.join(sorted(chips))}")
        print()

if __name__ == "__main__":
    main()
