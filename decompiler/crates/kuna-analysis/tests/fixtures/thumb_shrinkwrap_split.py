#!/usr/bin/env python3
"""Authored, Apache-2.0 Thumb ELF fixture; no compiler or binary inputs.

Generate with: python3 thumb_shrinkwrap_split.py OUTPUT

A routine at 0x1200 that nothing references saves LR and calls 0x1100. The
callee is shrink-wrapped: it tests its argument and returns before it saves
LR, so its `push {r4, lr}` at 0x1104 is reached by the fall-through of a
conditional branch. The Thumb prologue scan finds both pushes; the one at
0x1104 is the interior of 0x1100, not a function.
"""
import pathlib
import struct
import sys

BASE = 0x1000


def image():
    code = bytearray(0x400)

    def half(at, value):
        code[at - BASE:at - BASE + 2] = struct.pack('<H', value)

    def bl(at, target):
        off = target - (at + 4)
        s = (off >> 24) & 1
        j1 = 1 ^ ((off >> 23) & 1) ^ s
        j2 = 1 ^ ((off >> 22) & 1) ^ s
        half(at, 0xF000 | (s << 10) | ((off >> 12) & 0x3FF))
        half(at + 2, 0xD000 | (j1 << 13) | (j2 << 11) | ((off >> 1) & 0x7FF))

    for at in range(BASE, BASE + len(code), 2):
        half(at, 0x4770)
    half(0x1000, 0x2000)
    half(0x1100, 0x2800)
    half(0x1102, 0xD005)
    half(0x1104, 0xB510)
    half(0x1106, 0x4604)
    bl(0x1108, 0x1300)
    half(0x110C, 0x1C60)
    half(0x110E, 0xBD10)
    half(0x1200, 0xB510)
    bl(0x1202, 0x1100)
    half(0x1206, 0x2000)
    half(0x1208, 0xBD10)
    half(0x1300, 0x2001)
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
                    + struct.pack('<HHIIIIIHHHHHH', 2, 40, 1, BASE | 1, 52, shoff, 0x05000000,
                                  52, 32, 1, 40, len(sections), 2))
    content[52:84] = struct.pack('<8I', 1, textoff, BASE, BASE, len(code), len(code), 5, 4)
    return content


if __name__ == '__main__':
    pathlib.Path(sys.argv[1]).write_bytes(image())
