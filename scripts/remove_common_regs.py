import yaml
import os

def cleanup_group(path, common_regs):
    with open(path, 'r') as f:
        data = yaml.safe_load(f)
    
    block_key = next(k for k in data if k.startswith('block/'))
    items = data[block_key].get('items', [])
    
    # Keep track of fieldsets and enums used by the registers we are KEEPING
    used_fieldsets = set()
    new_items = []
    
    for item in items:
        if item['name'] not in common_regs:
            new_items.append(item)
            if 'fieldset' in item:
                used_fieldsets.add(item['fieldset'])
    
    data[block_key]['items'] = new_items
    
    # Second pass: find enums used by kept fieldsets
    used_enums = set()
    for fs_name in used_fieldsets:
        fs_key = f'fieldset/{fs_name}'
        if fs_key in data:
            for field in data[fs_key].get('fields', []):
                if 'enum' in field:
                    used_enums.add(field['enum'])
                    
    # Filter the top-level keys
    new_data = {block_key: data[block_key]}
    for k, v in data.items():
        if k.startswith('fieldset/'):
            name = k.split('/')[1]
            if name in used_fieldsets:
                new_data[k] = v
        elif k.startswith('enum/'):
            name = k.split('/')[1]
            if name in used_enums:
                new_data[k] = v
                
    with open(path, 'w') as f:
        yaml.dump(new_data, f, sort_keys=False)

def main():
    reg_dir = 'data/registers'
    
    classic_common = ['PARIOAD', 'SRAMPRCR']
    tz_common = ['SRAMPRCR_S', 'SRAMWTSC', 'SRAMESR', 'SRAMESCLR']
    
    for i in range(1, 6):
        path = os.path.join(reg_dir, f'SRAM_grp{i}.yaml')
        if os.path.exists(path):
            print(f"Cleaning {path} (Classic)")
            cleanup_group(path, classic_common)
            
    for i in range(6, 10):
        path = os.path.join(reg_dir, f'SRAM_grp{i}.yaml')
        if os.path.exists(path):
            print(f"Cleaning {path} (TrustZone)")
            cleanup_group(path, tz_common)

if __name__ == "__main__":
    main()
