#!/usr/bin/env python3
"""Generate `arm_funcmode_pool_le32` (armfuncmode): an A32 `a_func` whose literal
pool holds 0x0A000000 (a word shaped like an A32 `b`), followed by an
unsymbolized Thumb `t_s` at 0x02000094. Regenerate with `python3 arm_funcmode_pool_le32.py`.
"""
from pathlib import Path
import struct

def a_bl(src, dst):
    off = (dst - (src + 8)) >> 2
    return 0xEB000000 | (off & 0xFFFFFF)

def a_blx(src, dst):  # A32 -> Thumb
    off = dst - (src + 8)
    h = (off >> 1) & 1
    return 0xFA000000 | (h << 24) | ((off >> 2) & 0xFFFFFF)

def t_bl_common(src, dst, blx):
    pc = src + 4
    if blx:
        pc &= ~3
    off = dst - pc
    s = (off >> 24) & 1
    i1 = (off >> 23) & 1
    i2 = (off >> 22) & 1
    imm10 = (off >> 12) & 0x3FF
    imm11 = (off >> 1) & 0x7FF
    j1 = (~(i1 ^ s)) & 1
    j2 = (~(i2 ^ s)) & 1
    hw1 = 0xF000 | (s << 10) | imm10
    hw2 = (0xC000 if blx else 0xD000) | (j1 << 13) | (j2 << 11) | imm11
    if blx:
        hw2 &= ~1
    return struct.pack('<HH', hw1, hw2)

def t_bl(src, dst):
    return t_bl_common(src, dst, False)

def t_blx(src, dst):
    return t_bl_common(src, dst, True)

def build(text, base, funcs, maps=(), etype=2, entry=None, be=False, extra_sections=()):
    E = '>' if be else '<'
    strtab = bytearray(b"\0")
    symbols = [bytes(16)]
    def symbol(name, value, size, info, shndx=1):
        strtab.extend(name + b"\0")
        symbols.append(struct.pack(E + "IIIBBH", len(strtab) - len(name) - 1, value, size, info, 0, shndx))
    for name, value in maps:
        symbol(name, value, 0, 0x00)
    first_global = len(symbols)
    for name, value, size in funcs:
        symbol(name, value, size, 0x12)
    names = b"\0.text\0.symtab\0.strtab\0.shstrtab\0"
    contents = bytearray(84)
    sections = [bytes(40)]
    def section(name, data, kind, flags=0, address=0, link=0, info=0, entsize=0):
        contents.extend(bytes((-len(contents)) % 4))
        offset = len(contents)
        contents.extend(data)
        sections.append(struct.pack(E + "10I", names.index(name + b"\0"), kind, flags, address,
                                    offset, len(data), link, info, 4, entsize))
        return len(sections) - 1, offset
    _, text_offset = section(b".text", bytes(text), 1, 6, base)
    section(b".symtab", b"".join(symbols), 2, link=3, info=first_global, entsize=16)
    section(b".strtab", bytes(strtab), 3)
    names_index, _ = section(b".shstrtab", names, 3)
    contents.extend(bytes((-len(contents)) % 4))
    section_offset = len(contents)
    contents.extend(b"".join(sections))
    flags = 0x05000000 | (0x00800000 if be else 0)
    ident = b"\x7fELF\x01" + (b"\x02" if be else b"\x01") + b"\x01" + bytes(9)
    contents[:52] = struct.pack(E + "16sHHIIIIIHHHHHH", ident, etype, 40, 1,
                                base if entry is None else entry, 52, section_offset, flags,
                                52, 32, 1, 40, len(sections), names_index)
    contents[52:84] = struct.pack(E + "8I", 1, text_offset, base, base, len(text), len(text), 5, 4)
    return bytes(contents)

B = 0x02000000
t = bytearray(0xB0)
w = lambda off, *ws: struct.pack_into('<%dI' % len(ws), t, off, *ws)
w(0, 0xE52DE004, a_blx(B + 4, B + 0x40), a_bl(B + 8, B + 0x80), 0xE49DF004)  # entry: blx t_func; bl a_func
struct.pack_into("<2H", t, 0x40, 0x2007, 0x4770)  # t_func: movs r0,#7; bx lr
w(0x80, 0xE59F1004, 0xE0800001, 0xE12FFF1E, 0x0A000000)          # a_func: ldr r1,=0x0A000000; add r0,r0,r1; bx lr; pool
struct.pack_into('<2H', t, 0x90, 0xBF00, 0xBF00)                # thumb nop padding
struct.pack_into('<3H', t, 0x94, 0x2103, 0x4348, 0x4770)        # t_s (Thumb, no symbol): movs r1,#3; muls r0,r1,r0; bx lr
struct.pack_into('<H', t, 0xA0, 0xB500); t[0xA2:0xA6] = t_bl(B + 0xA2, B + 0x94); struct.pack_into('<H', t, 0xA6, 0xBD00)  # t_last: bl t_s
funcs = [(b"entry", B, 0x10), (b"t_func", B + 0x41, 8), (b"a_func", B + 0x80, 0x10), (b"t_last", B + 0xA1, 8)]
open(Path(__file__).parent / 'arm_funcmode_pool_le32', 'wb').write(build(t, B, funcs))
