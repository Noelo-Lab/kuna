#!/usr/bin/env python3
"""Build original weak relocations without linked/vendor implementations."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

p = argparse.ArgumentParser()
for name in ('cc', 'mips_cc', 'linker', 'artifacts'):
    p.add_argument('--' + name.replace('_', '-'), required=True)
a = p.parse_args()
r = Path(__file__).resolve().parent
artifacts = Path(a.artifacts).resolve()
artifacts.mkdir(parents=True, exist_ok=True)
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
rows = []
for abi in ['n32', '64', 'x86-64', 'x86-64-large']:
    mips = abi in ['n32', '64']
    cc = a.mips_cc if mips else a.cc
    flags = ['-mabi=' + abi, '-march=octeon', '-mno-abicalls', '-fno-pic', '-G0'] if mips else ['-fno-pic', '-fno-pie']
    if abi.endswith('large'):
        flags += ['-mcmodel=large']
    for source in ['weak.c', 'address.c']:
        name = source[:-2] + '-' + abi + '.elf'
        extra = ['-DTYPE_HINT'] if abi.endswith('large') and source == 'weak.c' else []
        command = [cc, '-O2', '-fno-stack-protector', '-c', source, '-o', name, *flags, *extra]
        subprocess.run(command, cwd=r, check=True)
        rows.append({'file': name, 'abi': abi, 'kind': source[:-2], 'source': source,
                     'sha256': sha(r / name), 'commands': [[Path(cc).name, *command[1:]]]})
    if mips:
        name = 'readonly-' + abi + '.elf'
        command = [cc, '-c', 'readonly-mips.S', '-o', name, '-mabi=' + abi,
                   '-march=octeon', '-mabicalls', '-fPIC', '-DABI64=' + ('1' if abi == '64' else '0')]
        subprocess.run(command, cwd=r, check=True)
        rows.append({'file': name, 'abi': abi, 'kind': 'address', 'source': 'readonly-mips.S',
                     'sha256': sha(r / name), 'commands': [command]})
        pic = artifacts / ('pic-' + abi + '.o')
        command = [cc, '-O2', '-fno-stack-protector', '-c', str(r / 'weak.c'), '-o', str(pic),
                   '-mabi=' + abi, '-march=octeon', '-mabicalls', '-fPIC', '-G0',
                   '-Dguard=pic_guard', '-Doffset_guard=pic_offset_guard',
                   '-Dpresent=pic_present', '-Ddefined_guard=pic_defined_guard']
        subprocess.run(command, check=True)
        name = 'mixed-' + abi + '.elf'
        link = [a.linker, '-r', '-m', 'elf32btsmipn32' if abi == 'n32' else 'elf64btsmip',
                '-o', str(r / name), str(r / ('weak-' + abi + '.elf')), str(pic)]
        result = subprocess.run(link, check=True, capture_output=True, text=True)
        rows.append({'file': name, 'abi': abi, 'kind': 'mixed', 'source': 'weak.c',
                     'sha256': sha(r / name), 'commands': [command, link], 'link_diagnostics': result.stderr})
for abi in ['n32', '64', 'x86-64']:
    mips = abi != 'x86-64'
    cc = a.mips_cc if mips else a.cc
    flags = ['-mabi=' + abi, '-march=octeon', '-mno-abicalls', '-fno-pic', '-G0'] if mips else ['-fno-pic', '-fno-pie']
    name = 'common-' + abi + '.elf'
    command = [cc, '-O2', '-fno-stack-protector', '-fcommon', '-c', 'common.c', '-o', name, *flags]
    subprocess.run(command, cwd=r, check=True)
    rows.append({'file': name, 'abi': abi, 'kind': 'common', 'source': 'common.c',
                 'sha256': sha(r / name), 'commands': [[Path(cc).name, *command[1:]]]})
command = [a.cc, '-O2', '-fno-stack-protector', '-fPIC', '-c', 'weak.c', '-o', 'weak-x86-64-pic.elf']
subprocess.run(command, cwd=r, check=True)
rows.append({'file': 'weak-x86-64-pic.elf', 'abi': 'x86-64-pic', 'kind': 'diagnosed', 'source': 'weak.c',
             'sha256': sha(r / 'weak-x86-64-pic.elf'), 'commands': [[Path(a.cc).name, *command[1:]]]})
for source, name, abi in [('rip.S', 'rip-x86-64.elf', 'x86-64-rip'),
                          ('readonly.S', 'readonly-x86-64.elf', 'x86-64-rodata')]:
    command = [a.cc, '-c', source, '-o', name]
    subprocess.run(command, cwd=r, check=True)
    rows.append({'file': name, 'abi': abi, 'kind': 'address', 'source': source,
                 'sha256': sha(r / name), 'commands': [[Path(a.cc).name, *command[1:]]]})
def portable(argument):
    argument = str(argument)
    for prefix in [str(r) + '/', str(artifacts) + '/']:
        if argument.startswith(prefix): return argument[len(prefix):]
    return argument
for row in rows:
    row['commands'] = [[Path(command[0]).name, *map(portable, command[1:])] for command in row['commands']]
    if 'link_diagnostics' in row:
        row['link_diagnostics'] = re.sub(r'(?<![\w.])/(?:[^\s:/]+/)*([^\s:/]+)', r'\1',
                                         row['link_diagnostics'])
for row in rows:
    row['source_sha256'] = sha(r / row['source'])
(r / 'provenance.json').write_text(json.dumps({'objects': rows, 'tools': {
    name: {'name': Path(getattr(a, name)).name, 'sha256': sha(Path(getattr(a, name))),
           'version': subprocess.check_output([getattr(a, name), '--version'], text=True).splitlines()[0]}
    for name in ['cc', 'mips_cc', 'linker']}}, indent=2) + '\n')
