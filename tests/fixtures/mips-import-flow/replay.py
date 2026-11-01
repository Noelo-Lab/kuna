#!/usr/bin/env python3
"""Execute untouched recovered helper bodies against an independent byte/effect oracle."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent
SUPPORT = r'''
typedef __SIZE_TYPE__ Size;
static char allocation[256];
static unsigned copies;
Size strlen(const char *p) { Size n=0; while(p[n]) n++; return n; }
void *malloc(Size n) { if(n>sizeof allocation)return (void*)0; return allocation; }
void *memcpy(void *d,const void *s,Size n) {
  char *out=d; const char *in=s; copies++;
  for(Size i=0;i<n;i++)out[i]=in[i]; return d;
}
char *strcpy(char *d,const char *s) { char *r=d; while((*d++=*s++)); return r; }
int external_transform(char *p) { return p[0]+3; }
extern char *original_duplicate(const char *);
extern char *RECOVERED(char *);
int main(void) {
  char input[41];
  for(unsigned n=0;n<=40;n++) {
    for(unsigned i=0;i<n;i++)input[i]='A'+i%25;
    input[n]=0;
    for(unsigned which=0;which<2;which++) {
      for(unsigned i=0;i<256;i++)allocation[i]=(char)0xa5;
      copies=0;
      char *r=which ? RECOVERED(input) : original_duplicate(input);
      if(r!=allocation || copies!=2 || r[2*n]!=0 || (unsigned char)r[2*n+1]!=0xa5)return 1;
      for(unsigned i=0;i<n;i++)if(r[i]!=input[i] || r[n+i]!=input[i])return 2;
    }
  }
  return 0;
}
'''
START = '''.set noreorder
.option pic0
.text
.globl _start
_start:
la $25,main
jalr $25
nop
move $4,$2
li $2,4001
syscall
'''


def main():
    p = argparse.ArgumentParser(description=__doc__)
    for option in ['kuna', 'specs', 'cc', 'assembler', 'ld', 'qemu_be', 'qemu_le', 'out']:
        p.add_argument('--' + option.replace('_', '-'), required=True)
    args = p.parse_args()
    out = Path(args.out); out.mkdir(parents=True, exist_ok=True)
    def identity(path):
        return {'path': str(Path(path).resolve()),
                'sha256': hashlib.sha256(Path(path).read_bytes()).hexdigest()}

    report = {'commands': [], 'runs': [], 'tools': {
        name: identity(getattr(args, name))
        for name in ['kuna', 'cc', 'assembler', 'ld', 'qemu_be', 'qemu_le']},
        'sources': {name: identity(ROOT / name) for name in ['flow.c', 'replay.py']}}

    def run(command):
        r = subprocess.run(list(map(str, command)), text=True, capture_output=True, timeout=120)
        report['commands'].append({'argv': list(map(str, command)), 'exit_code': r.returncode,
                                   'stdout': r.stdout, 'stderr': r.stderr})
        (out / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        if r.returncode: raise RuntimeError(r.stderr or r.stdout or str(command))
        return r.stdout

    manifest = json.loads((ROOT / 'provenance.json').read_text())
    for image in manifest['images']:
        name = image['file']
        if not (name.startswith('flow-mips32r2-') and name.endswith('ssp-stripped.so')): continue
        directory = out / name; directory.mkdir(exist_ok=True)
        doc = json.loads(run([args.kuna, 'decompile-all', ROOT / name, '--json', '--sleighpath', args.specs]))
        function = next(f for f in doc['functions'] if f['address'] == image['symbols']['duplicate']['address'])
        assert not function['error'], function
        body = function['code']; recovered = function['name']
        assert re.fullmatch(r'[a-zA-Z_][a-zA-Z0-9_]*', recovered)
        (directory / 'recovered.c').write_text('typedef unsigned int uint;\ntypedef unsigned long ulong;\n'
            'extern unsigned int strlen(const char *);\nextern void *malloc(unsigned int);\n'
            'extern void *memcpy(void *,const void *,unsigned int);\n' + body + '\n')
        (directory / 'support.c').write_text(SUPPORT.replace('RECOVERED', recovered))
        (directory / 'start.S').write_text(START)
        endian = '-EL' if '-le-' in name else '-EB'
        for optimization in ['0', '2']:
            flags = ['-march=mips32r2', '-mabi=32', '-msoft-float', endian, '-O' + optimization,
                     '-fPIC', '-mabicalls', '-fno-builtin', '-ffreestanding', '-fno-stack-protector']
            objects = []
            for label, source, extra in [('original', ROOT / 'flow.c', ['-DEXPORTED', '-Dduplicate=original_duplicate']),
                                         ('recovered', directory / 'recovered.c', []),
                                         ('support', directory / 'support.c', [])]:
                assembly = directory / (label + '.s'); obj = directory / (label + '.o')
                run([args.cc, *flags, *extra, '-S', source, '-o', assembly])
                run([args.assembler, '-32', endian, '-march=mips32r2', assembly, '-o', obj]); objects.append(obj)
            start = directory / 'start.o'
            run([args.assembler, '-32', endian, '-march=mips32r2', directory / 'start.S', '-o', start])
            executable = directory / ('replay-O' + optimization + '.elf')
            run([args.ld, '-m', 'elf32ltsmip' if endian == '-EL' else 'elf32btsmip', '-static',
                 '-e', '_start', '-z', 'max-page-size=0x1000', start, *objects, '-o', executable])
            run([args.qemu_le if endian == '-EL' else args.qemu_be, executable])
            report['runs'].append({'fixture': name, 'optimization': optimization,
                                   'original_cases': 41, 'recovered_cases': 41,
                                   'input': identity(ROOT / name),
                                   'recovered': identity(directory / 'recovered.c'),
                                   'executable': identity(executable)})
    assert len(report['runs']) == 8
    report['status'] = 'passed'
    (out / 'report.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
