#!/usr/bin/env python3
"""Generate an arm64 Mach-O inventory benchmark without an external toolchain.

Each function is one RET instruction, with its entry in LC_FUNCTION_STARTS.
Usage: python3 scripts/bench_functions_macho.py /tmp/functions.macho
"""

import argparse
import struct
from pathlib import Path


def align(value):
    return (value + 4095) & ~4095


def uleb(value):
    result = bytearray()
    while value >= 128:
        result.append((value & 127) | 128)
        value >>= 7
    result.append(value)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    parser.add_argument("--functions", type=int, default=1_000_000)
    args = parser.parse_args()
    if args.functions < 1 or args.functions > (0xffffffff - 8192) // 4:
        parser.error("--functions must fit the Mach-O file offsets and be positive")

    base, code_offset = 0x100000000, 4096
    code_size = 4 * args.functions
    linkedit_offset = align(code_offset + code_size)
    starts = uleb(code_offset) + b"\x04" * (args.functions - 1) + b"\0"
    segment = "<II16sQQQQiiII"
    text = struct.pack(segment, 0x19, 152, b"__TEXT", base, linkedit_offset,
                       0, linkedit_offset, 7, 5, 1, 0)
    text += struct.pack("<16s16sQQIIIIIIII", b"__text", b"__TEXT",
                        base + code_offset, code_size, code_offset, 2,
                        0, 0, 0x80000400, 0, 0, 0)
    linkedit = struct.pack(segment, 0x19, 72, b"__LINKEDIT",
                           base + linkedit_offset, align(len(starts)),
                           linkedit_offset, len(starts), 7, 1, 0, 0)
    commands = text + linkedit
    commands += struct.pack("<IIII", 0x26, 16, linkedit_offset, len(starts))
    commands += struct.pack("<IIQQ", 0x80000028, 24, code_offset, 0)
    header = struct.pack("<IIIIIIII", 0xfeedfacf, 0x0100000c, 0, 2,
                         4, len(commands), 1, 0)
    with args.output.open("wb") as output:
        output.write(header + commands)
        output.write(bytes(code_offset - output.tell()))
        output.write(struct.pack("<I", 0xd65f03c0) * args.functions)
        output.write(bytes(linkedit_offset - output.tell()))
        output.write(starts)


if __name__ == "__main__":
    main()
