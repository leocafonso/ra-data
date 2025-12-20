import os
import sys
import xml.etree.ElementTree as ET

def sanitize_svd(file_path):
    try:
        # Register namespaces to preserve them if possible
        # ET.register_namespace('', "http://www.w3.org/2001/XMLSchema-instance")
        
        tree = ET.parse(file_path)
        root = tree.getroot()
        modified = False

        def is_valid_int(s):
            if not s: return False
            s = s.strip()
            if s.startswith('0x') or s.startswith('0X'):
                try:
                    int(s, 16)
                    return True
                except ValueError:
                    return False
            if s.startswith('#'):
                try:
                    int(s[1:], 2)
                    return True
                except ValueError:
                    return False
            try:
                int(s)
                return True
            except ValueError:
                return False

        for reg in root.iter('register'):
            dim_node = reg.find('dim')
            if dim_node is not None:
                name_node = reg.find('name')
                if name_node is not None and name_node.text and '%s' not in name_node.text:
                    name_node.text = name_node.text + '%s'
                    modified = True

        for evs in root.iter('enumeratedValues'):
            to_remove = []
            for i, ev in enumerate(evs.findall('enumeratedValue')):
                name_node = ev.find('name')
                if name_node is None or not name_node.text:
                    # Fix missing name
                    if name_node is None:
                        name_node = ET.SubElement(ev, 'name')
                    name_node.text = f'UNKNOWN_{i}'
                    modified = True
                
                name = name_node.text
                # 1. Fix names starting with digits (e.g., 0x78 -> V_0x78)
                if name and name[0].isdigit():
                    name_node.text = 'V_' + name
                    modified = True
                    
                # 2. Fix "Others" or any name with isDefault=true but no value
                is_default_node = ev.find('isDefault')
                value_node = ev.find('value')
                if is_default_node is not None and is_default_node.text == 'true' and value_node is None:
                    # chiptool panics on isDefault without value in svd2ir.rs
                    # We remove it because it doesn't map to a specific Rust enum variant.
                    to_remove.append(ev)
                    modified = True
                elif value_node is None or not is_valid_int(value_node.text):
                    # Fix missing or invalid value
                    if value_node is None:
                        value_node = ET.SubElement(ev, 'value')
                    value_node.text = str(i)
                    modified = True
                elif value_node.text.startswith('#') or (name_node.text.startswith('V_') and all(c in '01' for c in name_node.text[2:]) and value_node.text == str(i)):
                    # Convert binary to decimal for better compatibility
                    # Also handle cases where we already messed up by replacing binary with index i
                    try:
                        b_str = value_node.text[1:] if value_node.text.startswith('#') else name_node.text[2:]
                        val = int(b_str, 2)
                        value_node.text = str(val)
                        modified = True
                    except ValueError:
                        if not value_node.text.startswith('#'):
                             pass # Not actually a binary name
                        else:
                            value_node.text = str(i)
                            modified = True
            for ev in to_remove:
                evs.remove(ev)

        if modified:
            # Use a simple write. Note: this might change formatting/namespaces slightly
            # but it's better than a crashing chiptool.
            tree.write(file_path, encoding='utf-8', xml_declaration=True)
            return True
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
    return False

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 sanitize_svd.py <file_or_dir>")
        sys.exit(1)
    
    target = sys.argv[1]
    if os.path.isdir(target):
        for f in os.listdir(target):
            if f.endswith('.svd'):
                p = os.path.join(target, f)
                if sanitize_svd(p):
                    print(f"Sanitized {p}")
    else:
        if sanitize_svd(target):
            print(f"Sanitized {target}")
