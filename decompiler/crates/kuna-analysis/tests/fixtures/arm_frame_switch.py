#!/usr/bin/env python3
"""Authored stripped ARM ELF: a disconnected A32 frame dispatches a switch.

Only frame recovery finds the dispatcher at 0x08000400. Its `ldrls pc` table
reaches four case blocks that open like the twenty mov/mov leaves, and its
default case calls a shrink-wrapped routine that saves LR one instruction
after its entry. Neither the cases nor that `push` is a function start.
Generate with: python3 arm_frame_switch.py OUTPUT
"""
from pathlib import Path
import struct
import sys

BASE = 0x08000000
ENTRY = BASE + 0x100
LEAVES = BASE + 0x200
SWITCH = BASE + 0x400
CASES = SWITCH + 0x24
DEFAULT = CASES + 0x40
WRAPPED = BASE + 0x480


def image():
    code = bytearray(0x4a0)

    def word(at, value):
        struct.pack_into('<I', code, at - BASE, value)

    def branch(at, target, link):
        word(at, (0xEB000000 if link else 0xEA000000) | ((target - at - 8) // 4) & 0xFFFFFF)

    for at in range(BASE, BASE + len(code), 4):
        word(at, 0xE12FFF1E)
    word(ENTRY, 0xE92D4000)
    for i in range(20):
        leaf = LEAVES + 0x10 * i
        branch(ENTRY + 4 + 4 * i, leaf, True)
        word(leaf, 0xE3A01000 | i)
        word(leaf + 4, 0xE3A02000 | i)
    word(ENTRY + 84, 0xE8BD8000)

    word(SWITCH, 0xE92D4010)
    word(SWITCH + 4, 0xE1A04000)
    word(SWITCH + 8, 0xE3500003)
    word(SWITCH + 12, 0x979FF100)
    branch(SWITCH + 16, DEFAULT, False)
    for i in range(4):
        case = CASES + 0x10 * i
        word(SWITCH + 20 + 4 * i, case)
        word(case, 0xE1A00004)
        word(case + 4, 0xE3A01001 + i)
        branch(case + 8, LEAVES + 0x10 * i, True)
        word(case + 12, 0xE8BD8010)
    word(DEFAULT, 0xE3A00000)
    branch(DEFAULT + 4, WRAPPED, True)
    word(DEFAULT + 8, 0xE8BD8010)

    word(WRAPPED, 0xE1A0C000)
    word(WRAPPED + 4, 0xE92D4010)
    word(WRAPPED + 8, 0xE1A0400C)
    branch(WRAPPED + 12, LEAVES, True)
    word(WRAPPED + 16, 0xE1A00004)
    word(WRAPPED + 20, 0xE8BD8010)

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
