#!/usr/bin/env python3
"""Generate `protoorder_codeptr_thumb_le32` -- a stripped Thumb ELF whose `caller`
hands a Thumb function address to a callee that reads bytes through its argument.

`peek`'s own recovery types its parameter a byte pointer.  `caller` passes the
literal `target|1` (the Thumb odd-address convention), a constant inside
`target`'s code.  A pointer vote on that constant renders it `&sub_8130[1]`,
a subscript of a function, which is not C; `protoorder` refuses a pointer vote
on a constant inside a function's code.

No cross toolchain is needed; the ELF is assembled here byte by byte. Regenerate
with:

    python3 protoorder_codeptr_thumb_le32.py

Layout (one R|X PT_LOAD at 0x8000, file offset == vma - 0x8000):

  .text @0x8100
      0x8100  _start: push {r4,lr}; bl caller; bl target; pop {r4,pc}
      0x8110  caller: push {r4,lr}; ldr r0,[pc,#8]; bl peek; pop {r4,pc}
      0x811c          .word 0x8131           <- target|1
      0x8120  peek:   ldrb r1,[r0]; ldrb r0,[r0,#2]; adds r0,r0,r1; bx lr
      0x8130  target: adds r0,#1; bx lr
"""
import os
import struct

BASE = 0x8000
TEXT_VMA = 0x8100
START, CALLER, PEEK, TARGET = 0x8100, 0x8110, 0x8120, 0x8130
TEXT_END = 0x8134
SHT_PROGBITS, SHT_STRTAB = 1, 3
SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
NOP = struct.pack("<H", 0xBF00)


def bl(at, target):
    """`bl <target>` (Thumb-2 T1); the Thumb PC is four bytes ahead."""
    imm = (target - (at + 4)) & 0x1FFFFFF
    s, i1, i2 = (imm >> 24) & 1, (imm >> 23) & 1, (imm >> 22) & 1
    j1, j2 = (1 - i1) ^ s, (1 - i2) ^ s
    return struct.pack("<HH", 0xF000 | (s << 10) | ((imm >> 12) & 0x3FF),
                       0xD000 | (j1 << 13) | (j2 << 11) | ((imm >> 1) & 0x7FF))


def h(*halfwords):
    return b"".join(struct.pack("<H", x) for x in halfwords)


def build_text():
    code = {
        START: h(0xB510) + bl(START + 2, CALLER) + bl(START + 6, TARGET) + h(0xBD10) + NOP + NOP,
        CALLER: h(0xB510, 0x4802) + bl(CALLER + 4, PEEK) + h(0xBD10) + NOP + struct.pack("<I", TARGET | 1),
        PEEK: h(0x7801, 0x7880, 0x1840, 0x4770) + NOP * 4,
        TARGET: h(0x3001, 0x4770),
    }
    text = b"".join(code[a] for a in sorted(code))
    assert len(text) == TEXT_END - TEXT_VMA, hex(len(text))
    return text


def main():
    image = bytearray(TEXT_VMA - BASE) + build_text()
    names = [b"", b".text", b".shstrtab"]
    shstr = bytearray()
    name_off = {}
    for n in names:
        name_off[n] = len(shstr)
        shstr.extend(n + b"\0")
    shstr_off = len(image)
    blob = bytearray(image) + shstr
    while len(blob) % 4:
        blob.append(0)
    shoff = len(blob)
    sections = [
        (b"", 0, 0, 0, 0, 0),
        (b".text", SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, TEXT_VMA - BASE, TEXT_END - TEXT_VMA),
        (b".shstrtab", SHT_STRTAB, 0, 0, shstr_off, len(shstr)),
    ]
    for name, typ, flags, vma, off, size in sections:
        blob.extend(struct.pack("<10I", name_off[name], typ, flags, vma, off, size, 0, 0, 4 if vma else 1, 0))
    phdr = struct.pack("<8I", 1, 0, BASE, BASE, len(image), len(image), 0x5, 0x1000)
    ehdr = struct.pack(
        "<4s5B7x2H5I6H",
        b"\x7fELF", 1, 1, 1, 0, 0,
        2, 40,
        1, START | 1, 52, shoff, 0x5000200,
        52, 32, 1, 40, len(sections), len(sections) - 1)
    blob[0:52] = ehdr
    blob[52:52 + len(phdr)] = phdr
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "protoorder_codeptr_thumb_le32")
    with open(out, "wb") as f:
        f.write(bytes(blob))
    print("wrote %s (%d bytes)" % (out, len(blob)))


if __name__ == "__main__":
    main()
