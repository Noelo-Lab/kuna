#!/usr/bin/env python3
"""Authored, Apache-2.0 ARM ELF fixture; no compiler or binary inputs.

Generate with: python3 arm_frame_retain.py OUTPUT

The entry calls twenty mov/mov leaves, stocking AIF's fingerprint corpus. Two
routines sit in an unreferenced gap: a frame at 0x1500 that tail-branches to a
mov/mov routine at 0x1600. AIF finds 0x1600 by itself; frame recovery walks the
frame, whose tail branch reaches 0x1600 as same-function flow.
"""
import pathlib
import struct
import sys

BASE = 0x1000


def image():
    code = bytearray(0x800)

    def word(at, value):
        code[at - BASE:at - BASE + 4] = struct.pack('<I', value)

    def branch(at, target, opcode):
        word(at, opcode | (((target - at - 8) // 4) & 0xFFFFFF))

    for at in range(BASE, BASE + len(code), 4):
        word(at, 0xE12FFF1E)
    word(BASE, 0xE92D4000)
    for i in range(20):
        leaf = 0x1100 + 16 * i
        branch(BASE + 4 + 4 * i, leaf, 0xEB000000)
        word(leaf, 0xE3A00000 | i)
        word(leaf + 4, 0xE3A01000 | i)
    word(BASE + 84, 0xE8BD8000)
    word(0x1500, 0xE92D4010)
    word(0x1504, 0xE1A04000)
    branch(0x1508, 0x1100, 0xEB000000)
    word(0x150C, 0xE1A00004)
    word(0x1510, 0xE8BD4010)
    branch(0x1514, 0x1600, 0xEA000000)
    word(0x1600, 0xE3A00001)
    word(0x1604, 0xE3A01002)
    textoff = 0x100
    shstr = b'\0.text\0.shstrtab\0'
    content = bytearray(textoff) + code
    shstroff = len(content)
    content += shstr
    while len(content) % 4:
        content += b'\0'
    sections = [(0,) * 10,
                (1, 1, 6, BASE, textoff, len(code), 0, 0, 4, 0),
                (7, 3, 0, 0, shstroff, len(shstr), 0, 0, 1, 0)]
    shoff = len(content)
    content += b''.join(struct.pack('<10I', *section) for section in sections)
    content[:52] = (b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)
                    + struct.pack('<HHIIIIIHHHHHH', 2, 40, 1, BASE, 52, shoff, 0x05000000,
                                  52, 32, 1, 40, len(sections), 2))
    content[52:84] = struct.pack('<8I', 1, textoff, BASE, BASE, len(code), len(code), 5, 4)
    return content


if __name__ == '__main__':
    pathlib.Path(sys.argv[1]).write_bytes(image())
