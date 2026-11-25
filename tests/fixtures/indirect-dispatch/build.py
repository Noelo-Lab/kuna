#!/usr/bin/env python3
"""Rebuild original MIPS dispatch fixtures and retain linkable execution images."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

p = argparse.ArgumentParser()
for name in ('assembler', 'linker', 'objcopy', 'nm', 'artifacts'):
    p.add_argument('--' + name, required=True)
a = p.parse_args()
root = Path(__file__).resolve().parent
artifacts = Path(a.artifacts).resolve()
artifacts.mkdir(parents=True, exist_ok=True)
records = []
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
for source in sorted(root.glob('*.S')):
    name = source.stem
    fields = name.split('-')
    abi, register, steps = fields[1], int(fields[2][1:]), int(fields[3])
    base = 0x407ff0 if name.endswith('-addiu') else 0x400000
    shutil.copy2(source, artifacts / source.name)
    commands = [
        [a.assembler, '-EB', '-march=octeon', '-mabi=' + abi, '-o', name + '.o', source.name],
        [a.linker, '-m', 'elf32btsmipn32' if abi == 'n32' else 'elf64btsmip', '-Ttext', hex(base),
         '-Tdata', '0x500000', '-e', '_start', '--build-id=none', '-o', name + '.elf', name + '.o'],
        [a.objcopy, '--only-section=.text', '-O', 'binary', name + '.elf', name + '.text.bin'],
    ]
    for command in commands:
        subprocess.run(command, cwd=artifacts, check=True)
    symbols = subprocess.check_output([a.nm, '-S', name + '.elf'], cwd=artifacts, text=True)
    symbol = next(line.split() for line in symbols.splitlines() if line.split()[-1:] == ['bounce'])
    assert int(symbol[0], 16) == base
    size = int(symbol[1], 16)
    binary = root / (name + '.bin')
    binary.write_bytes((artifacts / (name + '.text.bin')).read_bytes()[:size])
    records.append({'source': source.name, 'source_sha256': sha(source), 'binary': binary.name,
                    'binary_sha256': sha(binary), 'elf_sha256': sha(artifacts / (name + '.elf')),
                    'base': base, 'size': size, 'abi': abi, 'register': register, 'steps': steps,
                    'wide_return': '-wide-' in name, 'join': name.endswith('-join'),
                    'default_return': name.endswith('-condreturn'),
                    'return_shift': 31 if name.endswith('-wide-shift31') else 63 if name.endswith('-wide-shift63') else 32,
                    'commands': [[Path(c[0]).name, *c[1:]] for c in commands]})
tools = {name: {'name': Path(getattr(a, name)).name, 'sha256': sha(Path(getattr(a, name))),
               'version': subprocess.check_output([getattr(a, name), '--version'], text=True).splitlines()[0]}
         for name in ('assembler', 'linker', 'objcopy', 'nm')}
(root / 'provenance.json').write_text(json.dumps({'tools': tools, 'fixtures': records}, indent=2) + '\n')
