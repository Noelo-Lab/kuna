#!/usr/bin/env python3
"""Generate ``adrnext_sh4``: a synthetic static SuperH-4 little-endian ELF
whose ``mova`` forms the address right after a delay slot (GH #1360).

No SuperH toolchain is packaged here, so the bytes are written by hand.
Regenerate with ``python3 adrnext_sh4.py``; the output is deterministic. Read
it with ``--target SuperH4:LE:32:default`` (kuna maps no ELF ``EM_SH``).

    0x10000 rts_mova   rts ; mova @(0,pc),r0        r0 = 0x10004 (the pool word after it)
    0x10008 bts_mova   cmp/eq #0,r0 ; nop ; bt/s 0x10014 ; mova @(0,pc),r0
                                                    r0 = 0x10010 (after the slot)
    0x10018 mova_next  nop ; mova @(0,pc),r0        r0 = 0x1001c (no delay slot)
    0x10020 bsr_next   bsr leaf ; nop               pr = 0x10024 (a link)
    0x10028 leaf       rts ; nop
"""
import pathlib
import struct

BASE = 0x10000
TEXT_OFFSET = 0x54
WORDS = [
    0x000B, 0xC700, 0x3344, 0x1122,  # rts_mova (+ pool word 0x11223344)
    0x8800, 0x0009, 0x8D02, 0xC700, 0x000B, 0x0009, 0x000B, 0x0009,  # bts_mova
    0x0009, 0xC700, 0x000B, 0x0009,  # mova_next
    0xB002, 0x0009, 0x000B, 0x0009,  # bsr_next
    0x000B, 0x0009,  # leaf
]
SYMBOLS = [
    ("rts_mova", 0x0, 8),
    ("bts_mova", 0x8, 16),
    ("mova_next", 0x18, 8),
    ("bsr_next", 0x20, 8),
    ("leaf", 0x28, 4),
]
EM_SH = 42


def build() -> bytes:
    text = b"".join(struct.pack("<H", w) for w in WORDS)
    strtab = b"\0"
    symtab = b"\0" * 16
    for name, offset, size in SYMBOLS:
        symtab += struct.pack("<IIIBBH", len(strtab), BASE + offset, size, 0x12, 0, 1)
        strtab += name.encode() + b"\0"
    shstrtab = b"\0.text\0.symtab\0.strtab\0.shstrtab\0"
    symtab_offset = (TEXT_OFFSET + len(text) + 3) & ~3
    strtab_offset = symtab_offset + len(symtab)
    shstrtab_offset = strtab_offset + len(strtab)
    sections_offset = (shstrtab_offset + len(shstrtab) + 3) & ~3
    image = bytearray(b"\x7fELF" + bytes([1, 1, 1, 0]) + bytes(8))
    image += struct.pack("<HHIIIIIHHHHHH", 2, EM_SH, 1, BASE, 52, sections_offset, 0, 52, 32, 1, 40, 5, 4)
    image += struct.pack("<IIIIIIII", 1, TEXT_OFFSET, BASE, BASE, len(text), len(text), 5, 4)
    image += bytes(TEXT_OFFSET - len(image)) + text
    image += bytes(symtab_offset - len(image)) + symtab + strtab + shstrtab
    image += bytes(sections_offset - len(image))
    for header in [
        (0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
        (1, 1, 6, BASE, TEXT_OFFSET, len(text), 0, 0, 4, 0),
        (7, 2, 0, 0, symtab_offset, len(symtab), 3, 1, 4, 16),
        (15, 3, 0, 0, strtab_offset, len(strtab), 0, 0, 1, 0),
        (23, 3, 0, 0, shstrtab_offset, len(shstrtab), 0, 0, 1, 0),
    ]:
        image += struct.pack("<IIIIIIIIII", *header)
    return bytes(image)


if __name__ == "__main__":
    pathlib.Path(__file__).with_name("adrnext_sh4").write_bytes(build())
