#!/usr/bin/env python3
"""Build Go DWARF 5 entry-range regressions without a Go toolchain."""
from pathlib import Path
import subprocess
import tempfile
import shutil

root = Path(__file__).resolve().parent
cases = [
    ("identity", "ret", 0, 0, 0, 1),
    ("wrongRegister", "ret", 3, 0, 0, 1),
    ("laterRange", "ret", 0, 1, 1, 1),
    ("notReturn", "nop", 0, 0, 0, 1),
    ("longBody", "nop; ret", 0, 0, 0, 1),
    ("duplicateRange", "ret", 0, 0, 0, 2),
    ("badBase", "ret", 0, 0, 0, 1),
    ("resultRange", "ret", 0, 0, 0, 1),
]
lines = [".text", ".global _start", "_start: ret"]
def emit(*items): lines.extend(items)
def string(value): return '.asciz "' + value + '"'
def abbrev(number, tag, children, attrs):
    emit(f".uleb128 {number}", f".uleb128 {tag}", f".byte {children}")
    for attribute, form in attrs: emit(f".uleb128 {attribute}", f".uleb128 {form}")
    emit(".byte 0,0")
for name, body, *_ in cases:
    emit(".p2align 4", f".global main.{name}", f".type main.{name},@function",
         f"main.{name}:", body, f"end_{name}:", f".size main.{name},.-main.{name}")
emit('.section .go.buildinfo,"a"', '.balign 16', '.ascii "\\377 Go buildinf:"',
     '.byte 8,2', '.zero 16', '.byte 8', '.ascii "go1.27.3"', '.byte 0',
     '.section .debug_abbrev,"",@progbits')
abbrev(1, 0x11, 1, [(3,8),(0x13,5),(0x25,8),(0x73,0x17)])
abbrev(2, 0x2e, 1, [(3,8),(0x11,1),(0x12,0xf)])
abbrev(3, 5, 0, [(3,8),(0x4b,0xc),(0x49,0x10),(2,0x17)])
abbrev(4, 5, 0, [(3,8),(0x4b,0xc),(0x49,0x10),(2,0x18)])
abbrev(5, 0x24, 0, [(3,8),(0x3e,0xb),(0xb,0xb),(0x2900,0xb)])
abbrev(6, 0xf, 0, [(3,8),(0x49,0x10),(0x2900,0xb)])
emit('.byte 0', '.section .debug_addr,"",@progbits', 'debug_addr:',
     '.long addr_end-debug_addr-4', '.short 5', '.byte 8,0', 'addresses:')
for name, *_ in cases: emit(f".quad main.{name}" + ("+1" if name == "badBase" else ""))
emit('addr_end:', '.section .debug_loclists,"",@progbits', 'debug_loclists:',
     '.long loclists_end-debug_loclists-4', '.short 5', '.byte 8,0', '.long 0')
for index, (name, _, register, begin, end, count) in enumerate(cases):
    emit(f"loc_{name}:", f".byte 1,{index}")
    for _ in range(count): emit(f".byte 4,{begin},{end},1,{0x50+register}")
    emit('.byte 0')
emit('loclists_end:', '.section .debug_info,"",@progbits', 'debug_info:',
     '.long info_end-debug_info-4', '.short 5', '.byte 1,8', '.long 0', '.byte 1',
     string('main'), '.short 22', string('Go cmd/compile go1.27.3; regabi'),
     '.long addresses-debug_addr')
for name, *_ in cases:
    emit('.byte 2', string('main.'+name), f'.quad main.{name}', f'.uleb128 end_{name}-main.{name}',
         '.byte 3', string('p'), '.byte 0', '.long type_u64-debug_info', f'.long loc_{name}-debug_loclists',
         '.byte '+('3' if name=='resultRange' else '4'), string('~r0'), '.byte 1', '.long type_ptr-debug_info',
         f'.long loc_{name}-debug_loclists' if name=='resultRange' else '.byte 0', '.byte 0')
emit('type_u64:', '.byte 5', string('u64'), '.byte 7,8,11',
     'type_i32:', '.byte 5', string('i32'), '.byte 5,4,5',
     'type_ptr:', '.byte 6', string('*i32'), '.long type_i32-debug_info', '.byte 22',
     '.byte 0', 'info_end:', '.section .note.GNU-stack,"",@progbits')
with tempfile.TemporaryDirectory(prefix='kuna-go-entry-') as directory:
    directory = Path(directory)
    source, obj = directory/'fixture.S', directory/'fixture.o'
    source.write_text('\n'.join(lines)+'\n')
    subprocess.run(['as','--64',str(source),'-o',str(obj)],check=True)
    subprocess.run(['ld','-m','elf_x86_64','-e','_start','-Ttext=0x401000','--build-id=none',
                    '-o',str(directory/'plain'),str(obj)],check=True)
    shutil.copyfile(directory/'plain', root/'entry-ranges-plain')
    subprocess.run(['objcopy','--compress-debug-sections=zlib',str(directory/'plain'),str(root/'entry-ranges')],check=True)
