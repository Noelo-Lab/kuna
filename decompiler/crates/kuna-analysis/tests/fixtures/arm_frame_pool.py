#!/usr/bin/env python3
"""Authored stripped ARM ELF: an LR-saving frame follows a loaded literal.

The true entry copies its input before saving LR. AIF's mov/mov histogram also
accepts the literal word as a prefix, changing that input to zero.
Generate with: python3 arm_frame_pool.py OUTPUT
"""
from pathlib import Path
import struct
import sys

BASE = 0x08000000
ENTRY = BASE + 0x100
POOL = BASE + 0x3FC
TARGET = BASE + 0x400


def image():
    code = bytearray(0x420)

    def word(at, value):
        struct.pack_into('<I', code, at - BASE, value)

    def call(at, target):
        word(at, 0xEB000000 | ((target - at - 8) // 4) & 0xFFFFFF)

    for at in range(BASE, BASE + len(code), 4):
        word(at, 0xE12FFF1E)
    word(ENTRY, 0xE92D4000)
    word(ENTRY + 4, 0xE59F22F0)
    for i in range(20):
        leaf = BASE + 0x200 + 0x10 * i
        call(ENTRY + 8 + 4 * i, leaf)
        word(leaf, 0xE3A01000 | i)
        word(leaf + 4, 0xE3A02000 | i)
    word(ENTRY + 88, 0xE8BD8000)
    word(POOL, 0xE3A00000)
    word(TARGET, 0xE1A03000)
    word(TARGET + 4, 0xE92D4010)
    word(TARGET + 8, 0xE1A04003)
    call(TARGET + 12, BASE + 0x200)
    word(TARGET + 16, 0xE1A00004)
    word(TARGET + 20, 0xE8BD8010)

    textoff = 0x100
    content = bytearray(textoff) + code
    shstr = b'\0.text\0.shstrtab\0'
    shstroff = len(content)
    content += shstr
    while len(content) % 4:
        content += b'\0'
    sections = [(0,) * 10,
                (1, 1, 6, BASE, textoff, len(code), 0, 0, 4, 0),
                (7, 3, 0, 0, shstroff, len(shstr), 0, 0, 1, 0)]
    shoff = len(content)
    content += b''.join(struct.pack('<10I', *s) for s in sections)
    content[:52] = (b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)
                   + struct.pack('<HHIIIIIHHHHHH', 2, 40, 1, ENTRY, 52, shoff,
                                 0x05000000, 52, 32, 1, 40, len(sections), 2))
    content[52:84] = struct.pack('<8I', 1, textoff, BASE, BASE,
                                  len(code), len(code), 5, 4)
    return content


if __name__ == '__main__':
    Path(sys.argv[1]).write_bytes(image())
