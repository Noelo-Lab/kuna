#!/usr/bin/env python3
"""Generate `unmapped_branch_x86_64` — a stripped ELF whose only function tail-
jumps to an address the image does not map.

The shape `kuna decompile` lost on crackmes.one/5e51836e33c5d4439bb2dc24: a
function with 160 mapped bytes that disassembles cleanly, whose `jmp` displacement
resolves to 0x0. Reduced here to the smallest image that reproduces it.
Regenerate with:

    python3 unmapped_branch_x86_64.py

Layout — `.text` @ 0x401000, stripped (no `.symtab`), `e_entry` = 0x401000:

  0x401000  31 c0              xor  eax,eax
  0x401002  85 ff              test edi,edi
  0x401004  74 06              je   0x40100c
  0x401006  b8 39 05 00 00     mov  eax,0x539    ; the value the body carries
  0x40100b  c3                 ret
  0x40100c  e9 ef ef bf ff     jmp  0x0          ; flows into unmapped memory

Address 0 is in no segment of this image, so following the `jmp` asks the loader
for bytes it does not have. Before the fix that raised out of the whole flow
follow and `kuna decompile` reported the entry as an external symbol; now the
edge ends in a halt and the body renders with a `flows into unmapped memory`
warning.
"""
import os
import struct

TEXT_VMA = 0x401000
E_ENTRY = TEXT_VMA
UNMAPPED = 0x0

SHT_PROGBITS, SHT_STRTAB = 1, 3
SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PT_LOAD, PF_X, PF_R = 1, 0x1, 0x4

TEXT = bytes([
    0x31, 0xC0,                     # 0x401000 xor  eax,eax
    0x85, 0xFF,                     # 0x401002 test edi,edi
    0x74, 0x06,                     # 0x401004 je   0x40100c
    0xB8, 0x39, 0x05, 0x00, 0x00,   # 0x401006 mov  eax,0x539
    0xC3,                           # 0x40100b ret
    0xE9, 0xEF, 0xEF, 0xBF, 0xFF,   # 0x40100c jmp  0x0
])

EHDR, PHDR, SHDR = 64, 56, 64
NPH, NSH = 1, 3  # one PT_LOAD; null/.text/.shstrtab


def build():
    ph_off = EHDR
    text_off = ph_off + PHDR * NPH
    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.text'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = text_off + len(TEXT)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([2, 1, 1]) + bytes(9)            # e_ident (ELF64/LSB)
    b += struct.pack('<HHI', 2, 62, 1)                        # ET_EXEC, EM_X86_64, v1
    b += struct.pack('<QQQ', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0)                                 # e_flags
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 2)
    assert len(b) == ph_off

    b += struct.pack('<IIQQQQQQ', PT_LOAD, PF_R | PF_X, text_off, TEXT_VMA, TEXT_VMA,
                     len(TEXT), len(TEXT), 0x1000)
    assert len(b) == text_off
    b += TEXT
    b += shstr
    assert len(b) == sh_off

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIQQQQIIQQ', name, stype, flags, addr, off, size, 0, 0, 1, 0)

    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.text'], SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off,
              len(TEXT))
    b += shdr(names['.shstrtab'], SHT_STRTAB, 0, 0, shstr_off, len(shstr))
    return bytes(b)


def branch_target():
    """Re-derive the address the `jmp` resolves to, so the docstring cannot drift."""
    disp = struct.unpack('<i', TEXT[13:17])[0]
    return (TEXT_VMA + 0xC + 5 + disp) & 0xFFFFFFFFFFFFFFFF


if __name__ == '__main__':
    assert branch_target() == UNMAPPED, hex(branch_target())
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       'unmapped_branch_x86_64')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes), branch target {hex(UNMAPPED)}')
