#!/usr/bin/env python3
"""
Script to prepend BNF element names to grammar definitions in zim wiki files.
Transforms definitions to conform to BNF syntax: <element> ::= <definition>
"""

import os
import re
import sys

def process_file(filepath):
    """Process a single zim wiki file to add BNF element name."""

    with open(filepath, 'r') as f:
        content = f.read()

    lines = content.split('\n')

    # Find the title line (====== element name ======)
    title_line_idx = None
    element_name = None

    for i, line in enumerate(lines):
        match = re.match(r'^======\s+(.+?)\s+======\s*$', line)
        if match:
            title_line_idx = i
            element_name = match.group(1)
            break

    if title_line_idx is None or element_name is None:
        print(f"  Skipping {filepath}: No title found")
        return False

    # Convert element name to snake_case (spaces to underscores)
    element_name_snake = element_name.replace(' ', '_')

    # Find the "Created ..." line
    created_line_idx = None
    for i in range(title_line_idx + 1, min(title_line_idx + 3, len(lines))):
        if lines[i].startswith('Created '):
            created_line_idx = i
            break

    if created_line_idx is None:
        print(f"  Skipping {filepath}: No 'Created' line found")
        return False

    # Find the first definition line (skip empty lines after Created)
    def_start_idx = None
    for i in range(created_line_idx + 1, len(lines)):
        line = lines[i].strip()
        if line == '':
            continue
        # Check if this is an index page (Child pages section)
        if line.startswith('===== Child pages ====='):
            print(f"  Skipping {filepath}: Index page (Child pages)")
            return False
        # Check for IEEE reference lines
        if line.startswith('IEEE '):
            continue
        # Check if line already has BNF format
        if '::=' in lines[i]:
            print(f"  Skipping {filepath}: Already has BNF format")
            return False
        # Check for Backlinks section
        if line.startswith('===== Backlinks ====='):
            print(f"  Skipping {filepath}: No definition found before Backlinks")
            return False
        def_start_idx = i
        break

    if def_start_idx is None:
        print(f"  Skipping {filepath}: No definition found")
        return False

    # Prepend the element name with ::= to the first definition line
    original_first_def = lines[def_start_idx]
    lines[def_start_idx] = f"{element_name_snake} ::=\n\t{original_first_def}"

    # Write back the file
    with open(filepath, 'w') as f:
        f.write('\n'.join(lines))

    print(f"  Processed: {filepath}")
    return True

def main():
    base_dir = '/home/dhk/projects/verilog-1800-2003-tests/doc/Structure'

    processed = 0
    skipped = 0

    for root, dirs, files in os.walk(base_dir):
        for filename in files:
            if not filename.endswith('.txt'):
                continue

            filepath = os.path.join(root, filename)

            # Skip script files and non-grammar files
            if filename.endswith('.py') or filename.endswith('.sh'):
                continue

            if process_file(filepath):
                processed += 1
            else:
                skipped += 1

    print(f"\nTotal processed: {processed}")
    print(f"Total skipped: {skipped}")

if __name__ == '__main__':
    main()
