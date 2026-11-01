#!/usr/bin/env python3
"""Regenerate original sealed fixtures; compilation is not required by CI."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cc', required=True)
    parser.add_argument('--assembler', required=True)
    parser.add_argument('--ld', required=True)
    parser.add_argument('--strip', required=True)
    parser.add_argument('--nm', required=True)
    args = parser.parse_args()
    report = {'sources': {n: digest(ROOT / n) for n in ['flow.c', 'select.c']},
              'tools': {}, 'images': []}
    for name, path in vars(args).items():
        report['tools'][name] = {'sha256': digest(path), 'version': subprocess.check_output(
            [path, '--version'], text=True).splitlines()[0]}
    with tempfile.TemporaryDirectory() as temporary:
        obj = Path(temporary) / 'fixture.o'

        def build(name, source, cpu, abi, endian, extra, shared):
            flags = ['-march=' + cpu, '-mabi=' + abi, endian, '-msoft-float', '-O1',
                     '-fno-builtin', '-ffreestanding', '-fno-ident', '-fPIC', '-mabicalls', *extra]
            assembly = obj.with_suffix('.s')
            subprocess.run([args.cc, *flags, '-S', source, '-o', str(assembly)], cwd=ROOT, check=True)
            assemble = [('-32' if abi == '32' else '-n32' if abi == 'n32' else '-64'),
                        endian, '-march=' + cpu]
            subprocess.run([args.assembler, *assemble, str(assembly), '-o', str(obj)], check=True)
            emulation = ('elf32ltsmip' if endian == '-EL' else 'elf32btsmip') if abi == '32' else (
                'elf32btsmipn32' if abi == 'n32' else 'elf64btsmip')
            link = ['-m', emulation, '-z', 'max-page-size=0x1000', '--build-id=none']
            link += ['-shared'] if shared else ['-static', '-e', '__start']
            path = ROOT / name
            subprocess.run([args.ld, *link, str(obj), '-o', str(path)], check=True)
            symbols = {}
            for line in subprocess.check_output([args.nm, '-S', str(path)], text=True).splitlines():
                parts = line.split()
                if len(parts) == 4 and parts[2] in ['t', 'T']:
                    symbols[parts[3]] = {'address': int(parts[0], 16), 'size': int(parts[1], 16)}
            row = {'file': name, 'source': source, 'flags': flags, 'assembly_flags': assemble, 'link_flags': link,
                   'sha256': digest(path), 'symbols': symbols}
            report['images'].append(row)
            if shared:
                stripped = path.with_name(path.stem + '-stripped.so')
                subprocess.run([args.strip, '--strip-all', '-o', str(stripped), str(path)], check=True)
                report['images'].append(dict(row, file=stripped.name, sha256=digest(stripped), stripped=True))

        for endian, label in [('-EB', 'be'), ('-EL', 'le')]:
            for cpu in ['mips32r2', 'octeon', 'octeon2', 'octeon3']:
                build(f'select-{cpu}-{label}.elf', 'select.c', cpu, '32', endian,
                      ['-fno-stack-protector'], False)
            for cpu in ['mips32r2', 'octeon']:
                for exported in [False, True]:
                    for protected in [False, True]:
                        name = f'flow-{cpu}-{label}-' + ('exported' if exported else 'local')
                        name += '-ssp.so' if protected else '-plain.so'
                        flags = ['-fstack-protector-strong' if protected else '-fno-stack-protector']
                        if exported:
                            flags += ['-DEXPORTED']
                        build(name, 'flow.c', cpu, '32', endian, flags, True)
        for abi in ['n32', '64']:
            build(f'flow-octeon3-{abi}.so', 'flow.c', 'octeon3', abi, '-EB',
                  ['-fstack-protector-strong'], True)
    (ROOT / 'provenance.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
