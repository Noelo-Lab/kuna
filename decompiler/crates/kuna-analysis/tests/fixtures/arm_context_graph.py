#!/usr/bin/env python3
"""Authored mixed-ISA graphs; vary layout without changing their call semantics.

Usage: arm_context_graph.py OUTPUT little|big LAYOUT arm|thumb LEAVES [seed|frame]
The ELF entry calls a callee with a distant block. An independent symbol-seeded
ARM caller crosses another gap. A third ARM caller is deliberately unreachable.
"""
import pathlib
import struct
import sys


LAYOUTS = [(0x1000, 0x1600, 0x1580, 0x1750),
           (0x1700, 0x1600, 0x1380, 0x1500),
           (0x1400, 0x1500, 0x1680, 0x1780),
           (0x1800, 0x1400, 0x1580, 0x1380)]


def image(big=False, layout=0, thumb=True, leaves=0, discovery="seed"):
    order = '>' if big else '<'
    pack = lambda fmt, *v: struct.pack(order + fmt, *v)
    entry, seed, callee, tail = LAYOUTS[layout]
    code = bytearray(pack('I', 0xE12FFF1E) * (0x1400 // 4))

    def put(at, fmt, value):
        data = pack(fmt, value)
        code[at - 0x1000:at - 0x1000 + len(data)] = data

    def arm(at, to, opcode=0xEB000000):
        put(at, 'I', opcode | (((to - at - 8) // 4) & 0xFFFFFF))

    put(entry, 'I', 0xE92D4000)
    arm(entry + 4, callee, 0xFA000000 if thumb else 0xEB000000)
    for i in range(leaves):
        at = 0x2000 + i * 16
        arm(entry + 8 + i * 4, at)
        put(at, 'I', 0xE3A00007)
        put(at + 4, 'I', 0xE3A01000)
    put(entry + 8 + leaves * 4, 'I', 0xE8BD8000)
    put(seed, 'I', 0xE1A0C00E)
    arm(seed + 4, 0x1A00, 0xEA000000)
    arm(0x1A00, 0x1100)
    put(0x1A04, 'I', 0xE1A0E00C)
    put(0x1B00, 'I', 0xE1A0C00E)
    arm(0x1B04, 0x1100)
    put(0x1B08, 'I', 0xE1A0E00C)
    if thumb:
        put(callee, 'H', 0xB500)
        put(callee + 2, 'H', 0xE000 | (((tail - callee - 6) // 2) & 0x7FF))
        offset = 0x1200 - tail - 4
        put(tail, 'H', 0xF000 | ((offset >> 12) & 0x7FF))
        put(tail + 2, 'H', 0xF800 | ((offset >> 1) & 0x7FF))
        put(tail + 4, 'H', 0xBD00)
        put(0x1200, 'H', 0x4770)
    else:
        put(callee, 'I', 0xE92D4000)
        arm(callee + 4, tail, 0xEA000000)
        arm(tail, 0x1200)
        put(tail + 4, 'I', 0xE8BD8000)

    if discovery == 'frame':
        entry = 0x1900
        put(entry, 'I', 0xE92D4000)
        for i in range(leaves):
            arm(entry + 4 + i * 4, 0x2000 + i * 16)
        put(entry + 4 + leaves * 4, 'I', 0xE8BD8000)

    content = bytearray(0x100) + code
    strings = b'\0independent\0.text\0.symtab\0.strtab\0'
    symoff = len(content)
    content += bytes(16) + pack('IIIBBH', 1, seed, 0, 0x12, 0, 1)
    stroff = len(content)
    content += strings
    content += bytes((-len(content)) % 4)
    shoff = len(content)
    sections = [(0,) * 10, (strings.index(b'.text'), 1, 6, 0x1000, 0x100, len(code), 0, 0, 4, 0),
                (strings.index(b'.symtab'), 2, 0, 0, symoff, 32, 3, 1, 4, 16),
                (strings.index(b'.strtab'), 3, 0, 0, stroff, len(strings), 0, 0, 1, 0)]
    content += b''.join(pack('10I', *s) for s in sections)
    content[:52] = b'\x7fELF' + bytes([1, 2 if big else 1, 1]) + bytes(9) + pack(
        'HH5I6H', 2, 40, 1, entry, 52, shoff, 0x05000000, 52, 32, 1, 40, 4, 3)
    content[52:84] = pack('8I', 1, 0x100, 0x1000, 0x1000, len(code), len(code), 5, 4)
    return content


if __name__ == '__main__':
    pathlib.Path(sys.argv[1]).write_bytes(image(sys.argv[2] == 'big',
        int(sys.argv[3]), sys.argv[4] == 'thumb', int(sys.argv[5]),
        sys.argv[6] if len(sys.argv) > 6 else 'seed'))
