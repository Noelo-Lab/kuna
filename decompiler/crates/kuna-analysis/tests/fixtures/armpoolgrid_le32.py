#!/usr/bin/env python3
"""Generate `armpoolgrid_le32` — an A32 ARM image whose literal pool opens with a
word that does not decode.

The shape a stripped ARM crackme's `main` has: four constants parked after the
epilogue, each named by a PC-relative `ldr`, and the first of them
(`0xfffffeb8`) is a bit pattern the ARM translator refuses. A listing that
recovers from that refusal one byte at a time restarts on 0x10021 and every row
after it is off the 4-byte grid, so no pool word lands on a row boundary and
none of the four can be folded back to data.

No cross toolchain is needed; the ELF is assembled here byte by byte and the A32
bodies are hand-encoded. Regenerate with:

    python3 armpoolgrid_le32.py

Layout:

  PT_LOAD [0x10000, 0x10030)  PF_R|PF_X

  .text @0x10000  SHF_ALLOC|SHF_EXECINSTR, e_entry = 0x10000
      0x10000  stmdb sp!,{r11,lr}
      0x10004  mov r11,sp
      0x10008  ldr r0,[0x10020]      \\ four PC-relative loads, one per pool word:
      0x1000c  ldr r1,[0x10024]       | the listing's own evidence that each of
      0x10010  ldr r2,[0x10028]       | those addresses holds a constant
      0x10014  ldr r3,[0x1002c]      /
      0x10018  add r0,r0,r1
      0x1001c  ldmia sp!,{r11,pc}
      0x10020  .word 0xfffffeb8      <- REFUSED by the translator
      0x10024  .word 0xfffffe89
      0x10028  .word 0xfffffd84
      0x1002c  .word 0xfffffe3f

The four words are the ones the witness carried, kept verbatim: they are what a
`-fPIC` ARM build parks for its GOT-relative string addresses, and `0xfffffeb8`
is the one that does not decode.
"""
import struct, os

TEXT_VMA = 0x10000
E_ENTRY = TEXT_VMA

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
SHT_PROGBITS, SHT_STRTAB = 1, 3
PF_X, PF_R = 0x1, 0x4

POOL = TEXT_VMA + 0x20
# 0xfffffeb8 is the word that does not decode; the other three are its
# neighbours in the witness, kept so the fold has more than one word to prove.
WORDS = [0xFFFFFEB8, 0xFFFFFE89, 0xFFFFFD84, 0xFFFFFE3F]
TEXT_END = POOL + 4 * len(WORDS)


def ldr_pc(at, rd, target):
    """`ldr rd,[pc,#imm]` (A32): the PC base is `at + 8`."""
    off = target - (at + 8)
    assert 0 <= off < 0x1000, (hex(at), hex(target), off)
    return 0xE59F0000 | (rd << 12) | off


def build_text():
    t = bytearray()

    def at():
        return TEXT_VMA + len(t)

    def w(word):
        t.extend(struct.pack('<I', word))

    w(0xE92D4800)  # stmdb sp!,{r11,lr}
    w(0xE1A0B00D)  # mov r11,sp
    for rd in range(4):
        w(ldr_pc(at(), rd, POOL + 4 * rd))
    w(0xE0800001)  # add r0,r0,r1
    w(0xE8BD8800)  # ldmia sp!,{r11,pc}
    assert at() == POOL, hex(at())
    for word in WORDS:
        w(word)
    assert at() == TEXT_END, hex(at())
    return bytes(t)


TEXT = build_text()

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
    b += struct.pack('<HHI', 2, 40, 1)                      # ET_EXEC, EM_ARM, v1
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0x05000200)                      # e_flags: EABI5
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
              TEXT_VMA, text_off, len(TEXT), 4)
    b += shdr(names['.shstrtab'], SHT_STRTAB, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'armpoolgrid_le32')
    with open(out, 'wb') as fh:
        fh.write(build())
    os.chmod(out, 0o755)
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
