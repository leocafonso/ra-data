import os
import hashlib
from collections import defaultdict

def get_hash(filepath):
    hasher = hashlib.sha256()
    with open(filepath, 'rb') as f:
        hasher.update(f.read())
    return hasher.hexdigest()

def main():
    sram_dir = 'tmp/SRAM'
    groups = defaultdict(list)
    
    for filename in os.listdir(sram_dir):
        if filename.endswith('.yaml'):
            path = os.path.join(sram_dir, filename)
            h = get_hash(path)
            groups[h].append(filename.replace('.yaml', ''))
            
    print(f"Found {len(groups)} unique SRAM variations:\n")
    for i, (h, chips) in enumerate(groups.items(), 1):
        print(f"Variation {i} (Hash: {h[:8]}...):")
        print(f"  Chips: {', '.join(sorted(chips))}")
        print()

if __name__ == "__main__":
    main()
