"""Build an ELF with two constant-return functions and a scalable NOP run.

python3 listing_fallthrough.py x86_64 200000 /tmp/listing.elf
python3 listing_fallthrough.py aarch64 200000 /tmp/listing-arm64.elf
"""
import argparse
from pathlib import Path
import struct


def build(arch: str, count: int) -> bytes:
    base, text_offset = 0x400000, 0x1000
    if arch == "x86_64":
        machine = 62
        tiny = bytes.fromhex("b807000000c3")
        nop = b"\x90"
        tail = bytes.fromhex("b803000000c3")
    else:
        machine = 183
        tiny = bytes.fromhex("e0008052c0035fd6")
        nop = bytes.fromhex("1f2003d5")
        tail = bytes.fromhex("60008052c0035fd6")
    text = tiny.ljust(16, b"\0") + nop * count + tail
    names = b"\0_start\0bulk\0"
    shnames = b"\0.text\0.symtab\0.strtab\0.shstrtab\0"
    image = bytearray(text_offset) + text
    image += bytes(-len(image) % 8)
    symoff = len(image)
    image += bytes(24)
    image += struct.pack("<IBBHQQ", 1, 0x12, 0, 1, base + text_offset, len(tiny))
    image += struct.pack("<IBBHQQ", 8, 0x12, 0, 1, base + text_offset + 16,
                         count * len(nop) + len(tail))
    stroff = len(image)
    image += names
    shstroff = len(image)
    image += shnames
    image += bytes(-len(image) % 8)
    shoff = len(image)

    def section(name, kind, flags, addr, offset, size, link=0, info=0, align=1, entry=0):
        return struct.pack("<IIQQQQIIQQ", name, kind, flags, addr, offset, size,
                           link, info, align, entry)

    image += bytes(64)
    image += section(1, 1, 6, base + text_offset, text_offset, len(text), align=16)
    image += section(7, 2, 0, 0, symoff, 72, 3, 1, 8, 24)
    image += section(15, 3, 0, 0, stroff, len(names))
    image += section(23, 3, 0, 0, shstroff, len(shnames))
    image[:64] = struct.pack(
        "<16sHHIQQQIHHHHHH", b"\x7fELF\x02\x01\x01" + bytes(9),
        2, machine, 1, base + text_offset, 64, shoff, 0, 64, 56, 1, 64, 5, 4,
    )
    image[64:120] = struct.pack("<IIQQQQQQ", 1, 5, 0, base, base,
                                text_offset + len(text), text_offset + len(text), 0x1000)
    return bytes(image)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("arch", choices=["x86_64", "aarch64"])
    parser.add_argument("count", type=int)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    if args.count < 0:
        parser.error("count must be nonnegative")
    args.output.write_bytes(build(args.arch, args.count))
