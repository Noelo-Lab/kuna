#!/usr/bin/env python3
"""Rebuild the original x86-64 partial-unit DWARF regression; CI uses the sealed ELF."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--cc', default='gcc')
parser.add_argument('--dwz', default='dwz')
args = parser.parse_args()
root = Path(__file__).resolve().parent
sources = ['decl.h', 'initialize.c', 'caller.c', 'other.c']
flags = ['-O0', '-g', '-gdwarf-4', '-fno-pie', '-no-pie',
         '-fdebug-prefix-map=' + str(root) + '=.']
command = [args.cc, *flags, '-o', 'partial-unit.elf', *sources[1:]]
subprocess.run(command, cwd=root, check=True)
before = hashlib.sha256((root/'partial-unit.elf').read_bytes()).hexdigest()
subprocess.run([args.dwz, 'partial-unit.elf'], cwd=root, check=True)
info = subprocess.check_output(['readelf', '--debug-dump=info', 'partial-unit.elf'], cwd=root)
assert b'DW_TAG_partial_unit' in info
assert subprocess.run([str(root/'partial-unit.elf')]).returncode == 1
record = {
    'source': 'Original minimal callback/caller regression.',
    'compiler': subprocess.check_output([args.cc, '--version'], text=True).splitlines()[0],
    'dwz': subprocess.check_output([args.dwz, '--version'], text=True).splitlines()[0],
    'command': ['gcc', *['-fdebug-prefix-map=.=.' if f.startswith('-fdebug-prefix-map=') else f for f in flags],
                '-o', 'partial-unit.elf', *sources[1:]],
    'before_dwz_sha256': before,
    'sha256': {name: hashlib.sha256((root/name).read_bytes()).hexdigest()
               for name in sources + ['partial-unit.elf']},
    'original_exit_code': 1,
}
(root/'provenance.json').write_text(json.dumps(record, indent=2)+'\n')
