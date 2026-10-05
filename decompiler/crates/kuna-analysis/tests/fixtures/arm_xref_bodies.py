#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Authored ARM ELF: direct calls establish functions around a discontiguous body."""
import argparse
import pathlib
import struct


def build(endian='little'):
    order = '<' if endian == 'little' else '>'
    pack = lambda fmt, *values: struct.pack(order + fmt, *values)
    branch = lambda at, to, link=False: (0xEB000000 if link else 0xEA000000) | (((to - at - 8) >> 2) & 0xFFFFFF)
    code = bytearray(pack('I', 0xE7F000F0) * (0x800 // 4))
    words = {
        0x1000: 0xE92D4000,
        0x1004: branch(0x1004, 0x1500, True),
        0x1008: branch(0x1008, 0x1600, True),
        0x100C: 0xE8BD8000,
        0x1100: 0xE12FFF1E,
        0x1500: 0xE92D4000,
        0x1504: branch(0x1504, 0x1750),
        0x1600: 0xE3A00001,
        0x1604: 0xE12FFF1E,
        0x1750: branch(0x1750, 0x1100, True),
        0x1754: 0xE12FFF33,
        0x1758: 0xE8BD8000,
    }
    for at, word in words.items():
        code[at - 0x1000:at - 0x1000 + 4] = pack('I', word)
    elf = bytearray(0x100) + code
    strings = b'\0.text\0.shstrtab\0'
    strings_at = len(elf)
    elf += strings
    elf[:52] = b'\x7fELF\x01' + bytes([1 if endian == 'little' else 2, 1]) + bytes(9) + pack(
        'HH5I6H', 2, 40, 1, 0x1000, 52, len(elf), 0x5000000, 52, 32, 1, 40, 3, 2)
    elf[52:84] = pack('8I', 1, 0x100, 0x1000, 0x1000, len(code), len(code), 5, 4)
    elf += bytes(40) + pack('10I', 1, 1, 6, 0x1000, 0x100, len(code), 0, 0, 4, 0)
    elf += pack('10I', 7, 3, 0, 0, strings_at, len(strings), 0, 0, 1, 0)
    return bytes(elf)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=pathlib.Path)
    parser.add_argument('endian', choices=('little', 'big'), nargs='?', default='little')
    args = parser.parse_args()
    args.output.write_bytes(build(args.endian))
