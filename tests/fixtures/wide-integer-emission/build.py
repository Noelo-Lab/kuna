#!/usr/bin/env python3
"""Build original integer-emission objects with a pinned LLVM compiler."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

p = argparse.ArgumentParser()
p.add_argument('--compiler', required=True)
a = p.parse_args()
r = Path(__file__).resolve().parent
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
rows = []
for name, triple, target in [
    ('x86-64', 'x86_64-linux-gnu', 'x86:LE:64:default'),
    ('aarch64', 'aarch64-linux-gnu', 'AARCH64:LE:64:v8A'),
    ('ppc64', 'powerpc64-linux-gnu', 'PowerPC:BE:64:default'),
    ('mips64', 'mips64-linux-gnuabi64', 'MIPS:BE:64:octeon:default'),
]:
    output = r / (name + '.elf')
    flags = ['-DTEST_CALL'] if name == 'x86-64' else []
    target_flags = ['-march=octeon', '-mabi=64', '-mno-abicalls', '-msoft-float'] if name == 'mips64' else []
    command = [a.compiler, '--target=' + triple, '-O2', '-ffreestanding',
               '-fno-pic', '-fno-pie', '-fno-stack-protector', '-c',
               'shift.c', '-o', output.name, *target_flags, *flags]
    subprocess.run(command, cwd=r, check=True)
    rows.append({'file': output.name, 'sha256': sha(output), 'target': target,
                 'call': name == 'x86-64', 'command': [Path(a.compiler).name,
                 *command[1:]]})
(r / 'provenance.json').write_text(json.dumps({
    'source': 'shift.c', 'source_sha256': sha(r / 'shift.c'),
    'compiler': {'name': Path(a.compiler).name, 'sha256': sha(Path(a.compiler)),
                 'version': subprocess.check_output([a.compiler, '--version'], text=True).splitlines()[0]},
    'objects': rows}, indent=2) + '\n')
