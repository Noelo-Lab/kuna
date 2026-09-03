#!/usr/bin/env python3
"""Generate `unmapped_call_x86_64` — a stripped ELF whose recursive-descent walk
is offered a CALL target that is not in the image.

This is the shape `kuna functions` reported on
crackmes.one/66fd63059b533b4c22bd0b91 (32-bit ELF, "Overlapping / misaligned
instructions"), reduced to the smallest image that reproduces it and moved to
x86-64 so the always-built `x86` `.sla` is enough to run it. No compiler emits
this on purpose, so the ELF is assembled here byte by byte. Regenerate with:

    python3 unmapped_call_x86_64.py

Layout — `.text` @ 0x401000, stripped (no `.symtab`), `e_entry` = 0x401000:

  0x401000  31 c0              xor  eax,eax          ; ZF := 1
  0x401002  74 01              je   0x401005         ; ALWAYS taken
  0x401004  e8 b8 78 56 34     call 0x349688c1       ; never executed
  0x401005  b8 78 56 34 12     mov  eax,0x12345678   ; the REAL instruction
  0x40100a  e8 01 00 00 00     call 0x401010         ; a real callee
  0x40100f  c3                 ret
  0x401010  b8 2a 00 00 00     mov  eax,0x2a         ; the real callee
  0x401015  c3                 ret

The `e8` at 0x401004 is the byte before the real `mov`, so the "call" the walk
decodes on the (never-executed) fall-through of the always-taken `je` reads the
`mov`'s own immediate as its displacement: 0x401009 + 0x345678b8 = 0x349688c1,
half a gigabyte above a 190-byte image and inside no section at all.

The expected discovery sets are therefore:

  * `unmappedentry` OFF: 0x401000 (e_entry), 0x401010 (the real callee) and the
    phantom 0x349688c1 — a size-0 `sub_349688c1` with no bytes behind it,
  * `unmappedentry` ON (default): 0x401000 and 0x401010 only.
"""
import os
import struct

TEXT_VMA = 0x401000
E_ENTRY = TEXT_VMA
CALLEE = 0x401010
PHANTOM = 0x349688C1

SHT_PROGBITS, SHT_STRTAB = 1, 3
SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PT_LOAD, PF_X, PF_R = 1, 0x1, 0x4

TEXT = bytes([
    0x31, 0xC0,                                # 0x401000 xor eax,eax
    0x74, 0x01,                                # 0x401002 je  0x401005
    0xE8,                                      # 0x401004 the overlapping call opcode
    0xB8, 0x78, 0x56, 0x34, 0x12,              # 0x401005 mov eax,0x12345678
    0xE8, 0x01, 0x00, 0x00, 0x00,              # 0x40100a call 0x401010
    0xC3,                                      # 0x40100f ret
    0xB8, 0x2A, 0x00, 0x00, 0x00,              # 0x401010 mov eax,0x2a
    0xC3,                                      # 0x401015 ret
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


def phantom_target():
    """Re-derive the phantom the walk decodes, so the docstring cannot drift."""
    disp = struct.unpack('<i', TEXT[5:9])[0]
    return (TEXT_VMA + 4 + 5 + disp) & 0xFFFFFFFFFFFFFFFF


if __name__ == '__main__':
    assert phantom_target() == PHANTOM, hex(phantom_target())
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'unmapped_call_x86_64')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes), phantom target {hex(PHANTOM)}')
