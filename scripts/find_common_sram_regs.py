import yaml
import os

def get_registers(path):
    with open(path, 'r') as f:
        data = yaml.safe_load(f)
    
    # Find the block definition
    block_key = next(k for k in data if k.startswith('block/'))
    registers = set()
    for item in data[block_key].get('items', []):
        if 'name' in item:
            registers.add(item['name'])
    return registers

def main():
    reg_dir = 'data/registers'
    files = [f for f in os.listdir(reg_dir) if f.startswith('SRAM_grp') and f.endswith('.yaml')]
    
    if not files:
        print("No SRAM group files found.")
        return

    common_regs = None
    
    for f in files:
        regs = get_registers(os.path.join(reg_dir, f))
        if common_regs is None:
            common_regs = regs
        else:
            common_regs &= regs
            
    common_1_5 = None
    for f in sorted(files):
        if f in ['SRAM_grp1.yaml', 'SRAM_grp2.yaml', 'SRAM_grp3.yaml', 'SRAM_grp4.yaml', 'SRAM_grp5.yaml']:
            regs = get_registers(os.path.join(reg_dir, f))
            if common_1_5 is None:
                common_1_5 = regs
            else:
                common_1_5 &= regs
    
    if common_1_5:
        print(f"\nCommon registers across Groups 1-5:")
        for reg in sorted(common_1_5):
            print(f" - {reg}")

    common_6_9 = None
    for f in sorted(files):
        if f in ['SRAM_grp6.yaml', 'SRAM_grp7.yaml', 'SRAM_grp8.yaml', 'SRAM_grp9.yaml']:
            regs = get_registers(os.path.join(reg_dir, f))
            if common_6_9 is None:
                common_6_9 = regs
            else:
                common_6_9 &= regs
    
    if common_6_9:
        print(f"\nCommon registers across Groups 6-9:")
        for reg in sorted(common_6_9):
            print(f" - {reg}")

if __name__ == "__main__":
    main()
