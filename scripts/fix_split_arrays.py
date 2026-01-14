#!/usr/bin/env python3
"""
Fix split register arrays in SVD files.

Some Renesas RA SVD files have register arrays split into multiple definitions
with the same name but different dimIndex ranges. This causes chiptool to generate
duplicate function names.

Examples:
- R7FA2A2AD: IELSR split into 0-31 and 32-67
- R7FA4M1AB: IRQCR split into 0-12 and 14,15 (with gap at 13)

This script merges these split arrays into a single contiguous array.
"""

import os
import sys
import xml.etree.ElementTree as ET
from collections import defaultdict


def parse_dim_index(dim_index_text):
    """Parse dimIndex like '0-31' or '14,15' into a list of integers.
    Returns None if the dimIndex contains non-numeric values (like 'MBIU')."""
    if not dim_index_text:
        return None
    
    indices = []
    for part in dim_index_text.split(','):
        part = part.strip()
        if '-' in part:
            # Check if it's a range like '0-31' vs something like 'A-B'
            parts = part.split('-')
            if len(parts) == 2:
                try:
                    start, end = int(parts[0]), int(parts[1])
                    indices.extend(range(start, end + 1))
                except ValueError:
                    return None  # Non-numeric range
            else:
                return None
        else:
            try:
                indices.append(int(part))
            except ValueError:
                return None  # Non-numeric index
    return indices


def merge_split_arrays(file_path):
    """
    Merge split register arrays with the same name into single arrays.
    Returns True if file was modified.
    """
    try:
        tree = ET.parse(file_path)
        root = tree.getroot()
        modified = False
        
        # Find all peripherals
        for peripheral in root.iter('peripheral'):
            registers_node = peripheral.find('registers')
            if registers_node is None:
                continue
            
            # Group registers by name
            reg_groups = defaultdict(list)
            for reg in registers_node.findall('register'):
                name_node = reg.find('name')
                if name_node is not None and name_node.text:
                    # Normalize name (remove %s suffix for grouping)
                    base_name = name_node.text.replace('%s', '')
                    reg_groups[base_name].append(reg)
            
            # Process groups with multiple registers (potential splits)
            for base_name, regs in reg_groups.items():
                if len(regs) <= 1:
                    continue
                
                # Check if these are array registers (have dim element)
                array_regs = [r for r in regs if r.find('dim') is not None]
                if len(array_regs) <= 1:
                    continue
                
                # Collect all indices and their info
                all_indices = []
                first_reg = None
                base_offset = None
                increment = None
                
                for reg in array_regs:
                    dim_index = reg.find('dimIndex')
                    dim_inc = reg.find('dimIncrement')
                    offset = reg.find('addressOffset')
                    
                    if dim_index is None or offset is None:
                        continue
                    
                    indices = parse_dim_index(dim_index.text)
                    if indices is None:
                        # Non-numeric dimIndex, skip this register group
                        all_indices = []
                        break
                    if not indices:
                        continue
                    
                    curr_offset = int(offset.text, 16) if offset.text.startswith('0x') else int(offset.text)
                    curr_inc = int(dim_inc.text, 16) if dim_inc is not None and dim_inc.text.startswith('0x') else (int(dim_inc.text) if dim_inc is not None else 1)
                    
                    # Track the first (lowest index) register
                    if first_reg is None or min(indices) < min(parse_dim_index(first_reg.find('dimIndex').text)):
                        first_reg = reg
                        base_offset = curr_offset
                        increment = curr_inc
                    
                    all_indices.extend(indices)
                
                if not all_indices or first_reg is None:
                    continue
                
                all_indices = sorted(set(all_indices))
                
                # Check if we actually have a split (more than one reg with same name)
                if len(array_regs) <= 1:
                    continue
                
                peri_name = peripheral.find('name')
                peri_name_str = peri_name.text if peri_name is not None else "unknown"
                
                print(f"  {peri_name_str}: Merging {base_name}%s arrays: indices {min(all_indices)}-{max(all_indices)} ({len(all_indices)} elements)")
                
                # Update first_reg to cover all indices
                first_reg.find('dim').text = str(len(all_indices))
                
                # Create proper dimIndex
                if all_indices == list(range(min(all_indices), max(all_indices) + 1)):
                    # Contiguous range
                    first_reg.find('dimIndex').text = f"{min(all_indices)}-{max(all_indices)}"
                else:
                    # Non-contiguous, list all indices
                    first_reg.find('dimIndex').text = ','.join(str(i) for i in all_indices)
                
                # Remove the other duplicate registers
                for reg in array_regs:
                    if reg is not first_reg:
                        registers_node.remove(reg)
                
                modified = True
        
        if modified:
            tree.write(file_path, encoding='utf-8', xml_declaration=True)
        
        return modified
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        import traceback
        traceback.print_exc()
        return False


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 fix_split_arrays.py <file_or_dir>")
        print()
        print("Fix split register arrays in Renesas RA SVD files.")
        print("This merges registers like IELSR%s that are split into")
        print("multiple definitions (e.g., 0-31 and 32-67) into a single array.")
        sys.exit(1)
    
    target = sys.argv[1]
    
    if os.path.isdir(target):
        for f in sorted(os.listdir(target)):
            if f.endswith('.svd'):
                p = os.path.join(target, f)
                print(f"Processing {f}...")
                if merge_split_arrays(p):
                    print(f"  Modified {f}")
    else:
        print(f"Processing {target}...")
        if merge_split_arrays(target):
            print(f"  Modified {target}")


if __name__ == "__main__":
    main()
