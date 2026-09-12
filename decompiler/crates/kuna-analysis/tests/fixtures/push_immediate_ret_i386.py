#!/usr/bin/env python3
"""Generate a minimal PE32 twin of CryptoME's unpacker tail transfer.

The entry executes ``call unpacker; push 0x401000; ret``.  The target section
contains CryptoME's encrypted on-disk bytes and has no function/export symbol.
It is below the entry at ``0x402000``. Additional fixed-address routines are
conservative negative controls.
"""

import os
import struct

IMAGE_BASE = 0x400000
FILE_ALIGN = 0x200
SECTION_ALIGN = 0x1000
TEXT_RVA = 0x2000
PACKED_RVA = 0x1000


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_text():
    text = bytearray(FILE_ALIGN)

    def put(rva, data):
        offset = rva - TEXT_RVA
        text[offset:offset + len(data)] = data

    entry = bytearray(b"\xe8")
    entry += rel32(0x2000, 5, 0x2010)
    entry += b"\x68" + struct.pack("<I", IMAGE_BASE + PACKED_RVA) + b"\xc3"
    put(0x2000, entry)
    put(0x2010, b"\xb8\x11\x11\x00\x00\xc3")
    put(0x2020, b"\xc3")

    argument = bytearray(b"\x68") + struct.pack("<I", IMAGE_BASE + PACKED_RVA)
    argument += b"\xe8" + rel32(0x2035, 5, 0x2070)
    argument += b"\x83\xc4\x04\xc3"
    put(0x2030, argument)
    put(0x2040, b"\x68" + struct.pack("<I", IMAGE_BASE + PACKED_RVA) + b"\x83\xc4\x04\xc3")
    put(0x2050, b"\x68" + struct.pack("<I", IMAGE_BASE + PACKED_RVA) + b"\x89\x04\x24\xc3")
    put(0x2060, b"\x50\xc3")
    put(0x2070, b"\x8b\x44\x24\x04\xc3")
    put(0x2080, b"\x85\xc0\x74\x05\x68" + struct.pack("<I", IMAGE_BASE + PACKED_RVA) + b"\xc3")
    put(
        0x2090,
        b"\x68" + struct.pack("<I", IMAGE_BASE + 0x209b)
        + b"\x68" + struct.pack("<I", IMAGE_BASE + 0x20a0)
        + b"\xc3\xc3",
    )
    put(0x20A0, b"\xba\x22\x22\x00\x00\xc3")
    return bytes(text)


def section(name, virtual_size, rva, raw_offset, flags):
    return name.encode().ljust(8, b"\0") + struct.pack(
        "<IIIIIIHHI", virtual_size, rva, FILE_ALIGN, raw_offset, 0, 0, 0, 0, flags
    )


def build():
    text = build_text()
    packed = bytes.fromhex("29608c1d23332b30b84d4cc30254c02b").ljust(FILE_ALIGN, b"\0")
    image = bytearray(b"MZ")
    image += bytes(0x3C - len(image))
    image += struct.pack("<I", 0x40)
    image += bytes(0x40 - len(image))
    image += b"PE\0\0"
    image += struct.pack("<HHIIIHH", 0x14C, 2, 0, 0, 0, 0xE0, 0x010F)

    optional = struct.pack("<HBB", 0x10B, 14, 0)
    optional += struct.pack("<III", FILE_ALIGN, FILE_ALIGN, 0)
    optional += struct.pack("<II", TEXT_RVA, TEXT_RVA)
    optional += struct.pack("<I", PACKED_RVA)
    optional += struct.pack("<III", IMAGE_BASE, SECTION_ALIGN, FILE_ALIGN)
    optional += struct.pack("<HHHHHHI", 4, 0, 0, 0, 4, 0, 0)
    optional += struct.pack("<III", 0x3000, FILE_ALIGN, 0)
    optional += struct.pack("<HH", 3, 0)
    optional += struct.pack("<IIII", 0x100000, 0x1000, 0x100000, 0x1000)
    optional += struct.pack("<II", 0, 16)
    optional += bytes(16 * 8)
    assert len(optional) == 0xE0
    image += optional
    image += section(".text", len(text), TEXT_RVA, FILE_ALIGN, 0x60000020)
    image += section(".packed", 16, PACKED_RVA, FILE_ALIGN * 2, 0xC0000040)
    image += bytes(FILE_ALIGN - len(image))
    image += text
    image += packed
    return bytes(image)


if __name__ == "__main__":
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "push_immediate_ret_i386.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
