#!/usr/bin/env python3
"""Generate tiny ELF32/ELF64 mapped-flow witnesses (Apache-2.0, no external input)."""
from pathlib import Path
import struct

CODE = bytes.fromhex("85c07506b807000000c3bb05000000")


def image(bits, code=CODE):
    ident = b"\x7fELF" + bytes([1 if bits == 32 else 2, 1, 1]) + bytes(9)
    if bits == 32:
        header = ident + struct.pack("<HHIIIIIHHHHHH", 2, 3, 1, 0x10000, 52,
                                    0, 0, 52, 32, 1, 40, 0, 0)
        ph = struct.pack("<IIIIIIII", 1, 84, 0x10000, 0x10000,
                         len(code), len(code), 5, 1)
    else:
        header = ident + struct.pack("<HHIQQQIHHHHHH", 2, 62, 1, 0x10000, 64,
                                    0, 0, 64, 56, 1, 64, 0, 0)
        ph = struct.pack("<IIQQQQQQ", 1, 5, 120, 0x10000, 0x10000,
                         len(code), len(code), 1)
    return header + ph + code


if __name__ == "__main__":
    for bits in (32, 64):
        Path(__file__).with_name(f"mapped_flow_boundary_{bits}.elf").write_bytes(image(bits))
