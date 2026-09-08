#!/usr/bin/env python3
"""Generate `segmentgap_i386` — an i386 image whose executable segment ends in
the middle of a page, with a genuine unmapped hole above it.

The shape the `keygenme` witness had: `LOAD` #1 is `R E` and stops at
0x08048014, `LOAD` #2 starts a page and a half later at 0x0804a000, and nothing
maps the bytes between them. The load image answers a read that STARTS inside
LOAD #1 for its whole length anyway, zero-filling the hole (the upstream BFD
`loadFill` contract), so a listing that walks off the end of `.text` decodes
that zero fill as `ADD byte ptr [EAX],AL` rows the file does not contain.

The last mapped byte is a single 0x00 on purpose: the two-byte `add [eax],al`
the translator would decode there STRADDLES the boundary, so the listing has to
refuse the decode and fall back to `.byte 0x00` rather than merely stop one row
earlier.

No toolchain is needed; the ELF is assembled here byte by byte. Regenerate with:

    python3 segmentgap_i386.py

Layout:

  PT_LOAD [0x08048000, 0x08048014)  PF_R|PF_X
  PT_LOAD [0x0804a000, 0x0804a010)  PF_R|PF_W        <- the far side of the hole

  .text @0x08048000  SHF_ALLOC|SHF_EXECINSTR, e_entry = 0x08048000
      0x08048000  push ebp
      0x08048001  mov ebp,esp
      0x08048003  mov eax,dword ptr [ebp + 0x8]
      0x08048006  add eax,0x1
      0x08048009  pop ebp
      0x0804800a  ret
      0x0804800b  mov esp,dword ptr [0x847c2b0]   \\ the witness's own tail, kept
      0x08048011  mov cs,ax                        | verbatim: the last two real
      0x08048013  .byte 0x00                      /  instructions and one stray 0
"""
import struct, os

TEXT_VMA = 0x08048000
DATA_VMA = 0x0804A000
E_ENTRY = TEXT_VMA

SHF_ALLOC, SHF_EXECINSTR, SHF_WRITE = 0x2, 0x4, 0x1
SHT_PROGBITS, SHT_STRTAB = 1, 3
PF_X, PF_W, PF_R = 0x1, 0x2, 0x4

TEXT = bytes([
    0x55,                                # push ebp
    0x89, 0xE5,                          # mov ebp,esp
    0x8B, 0x45, 0x08,                    # mov eax,[ebp+8]
    0x83, 0xC0, 0x01,                    # add eax,1
    0x5D,                                # pop ebp
    0xC3,                                # ret
    0x8B, 0x25, 0xB0, 0xC2, 0x47, 0x08,  # mov esp,[0x847c2b0]
    0x8E, 0xC8,                          # mov cs,ax
    0x00,                                # the last mapped byte
])
DATA = bytes(range(0x10))

TEXT_END = TEXT_VMA + len(TEXT)

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 2, 4  # two PT_LOAD; null/.text/.data/.shstrtab
PAGE = 0x1000


def build():
    ph_off = EHDR
    text_off = ph_off + PHDR * NPH
    # p_offset must stay congruent to p_vaddr modulo the page size, so the data
    # segment starts on the file page its vma sits on.
    data_off = PAGE * ((text_off + len(TEXT)) // PAGE + 1) + (DATA_VMA % PAGE)

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.text', '.data'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = data_off + len(DATA)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 3, 1)                       # ET_EXEC, EM_386, v1
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0)                               # e_flags
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 3)

    b += struct.pack('<IIIIIIII', 1, text_off, TEXT_VMA, TEXT_VMA,
                     len(TEXT), len(TEXT), PF_R | PF_X, PAGE)
    b += struct.pack('<IIIIIIII', 1, data_off, DATA_VMA, DATA_VMA,
                     len(DATA), len(DATA), PF_R | PF_W, PAGE)
    assert len(b) == text_off
    b += TEXT
    b += bytes(data_off - len(b))
    assert len(b) == data_off
    b += DATA
    assert len(b) == shstr_off
    b += shstr
    assert len(b) == sh_off

    def shdr(name, typ, flags, addr, off, size, align=1):
        return struct.pack('<IIIIIIIIII', name, typ, flags, addr, off, size, 0, 0, align, 0)

    b += shdr(0, 0, 0, 0, 0, 0, 0)
    b += shdr(names['.text'], SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR,
              TEXT_VMA, text_off, len(TEXT), 4)
    b += shdr(names['.data'], SHT_PROGBITS, SHF_ALLOC | SHF_WRITE,
              DATA_VMA, data_off, len(DATA), 4)
    b += shdr(names['.shstrtab'], SHT_STRTAB, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'segmentgap_i386')
    with open(out, 'wb') as fh:
        fh.write(build())
    os.chmod(out, 0o755)
    print('wrote %s (%d bytes, text ends 0x%x)'
          % (out, os.path.getsize(out), TEXT_END))
