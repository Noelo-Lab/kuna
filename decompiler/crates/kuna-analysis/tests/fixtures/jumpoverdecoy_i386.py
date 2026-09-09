#!/usr/bin/env python3
"""Generate `jumpoverdecoy_i386` — an i386 image whose branches jump over decoy
bytes, so a straight-line listing of it decodes out of phase.

The 60 bytes of `.text` are lifted verbatim from the round-12 witness
(`LoaderCrackMev2.0` `corrupt.exe`, 0x43d08a..0x43d0c6): a
LODSB/arithmetic/STOSB decryption loop whose every basic block is entered by an
`EB 01` / `EB 04` jump over one junk byte. Only the base address moved -- every
branch in the run is PC-relative, so the shape is bit-identical to the witness.

Read straight through from 0x10000, four of the six direct branches land inside
an instruction rather than on one:

    0x10002  eb 04           JMP 0x10008   \\ both jump over the 0xc2 at 0x10007
    0x10005  eb 01           JMP 0x10008   /
    0x10007  c2 ac eb        RET 0xebac    <- covers 0x10008, and is not there

and the same at 0x10024 and 0x10037. From 0x10007 on, the listing spells bytes
the program never executes that way: it prints a CALL and a JMP to an address
outside the image, neither of which is in the bytes. Decoding from the branch
targets instead (`kuna disassemble --follow`) recovers LODSB / STOSB and the
`LOOP 0x10008` back edge that closes the loop.

No toolchain is needed; the ELF is assembled here byte by byte. Regenerate with:

    python3 jumpoverdecoy_i386.py

Layout:

  PT_LOAD [0x10000, 0x1003d)  PF_R|PF_X
  .text @0x10000  SHF_ALLOC|SHF_EXECINSTR, e_entry = 0x10000
"""
import os
import struct

TEXT_VMA = 0x10000
E_ENTRY = TEXT_VMA

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
SHT_PROGBITS, SHT_STRTAB = 1, 3
PF_X, PF_R = 0x1, 0x4

# corrupt.exe 0x43d08a..0x43d0c6, verbatim.
WITNESS = bytes([
    0x33, 0xc0, 0xeb, 0x04, 0x90, 0xeb, 0x01, 0xc2, 0xac, 0xeb, 0x01, 0xe9,
    0x90, 0xfe, 0xc8, 0x2c, 0x2b, 0x04, 0xbf, 0xf9, 0xf9, 0x90, 0x04, 0x6c,
    0x2c, 0xf8, 0x2a, 0xc1, 0xc0, 0xc8, 0x71, 0x2c, 0x8b, 0xeb, 0x01, 0xe9,
    0xc0, 0xc8, 0x95, 0x04, 0x59, 0x2a, 0xc1, 0x34, 0x92, 0xc0, 0xc0, 0xdb,
    0xf8, 0x02, 0xc1, 0xf8, 0xeb, 0x01, 0xe8, 0x34, 0x92, 0xaa, 0xe2, 0xcc,
])
# The witness runs on into more of the packer stub; the fixture ends the run.
TEXT = WITNESS + b'\xc3'  # ret
TEXT_END = TEXT_VMA + len(TEXT)

EHDR, PHDR, SHDR = 52, 32, 40
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
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 3, 1)                       # ET_EXEC, EM_386, v1
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0)                               # e_flags
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 2)

    b += struct.pack('<IIIIIIII', 1, text_off, TEXT_VMA, TEXT_VMA,
                     len(TEXT), len(TEXT), PF_R | PF_X, 4)
    assert len(b) == text_off
    b += TEXT
    assert len(b) == shstr_off
    b += shstr
    assert len(b) == sh_off

    def shdr(name, typ, flags, addr, off, size, align=1):
        return struct.pack('<IIIIIIIIII', name, typ, flags, addr, off, size, 0, 0, align, 0)

    b += shdr(0, 0, 0, 0, 0, 0, 0)
    b += shdr(names['.text'], SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR,
              TEXT_VMA, text_off, len(TEXT), 1)
    b += shdr(names['.shstrtab'], SHT_STRTAB, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'jumpoverdecoy_i386')
    with open(out, 'wb') as fh:
        fh.write(build())
    os.chmod(out, 0o755)
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
