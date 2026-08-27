#!/usr/bin/env python3
"""Patch the three byte-level hostile symbol names into hostile_symname_x86_64.

GNU as will not emit a raw 0x0a, 0x80 or 0x81 inside a symbol name, so the
assembly declares same-length ASCII placeholders and this rewrites the middle
byte of each in `.strtab`.  Usage:

    python3 hostile_symname_x86_64_patch.py <linked-elf> <output>
"""
import sys

PATCHES = ((b"aQb\x00", 0x0A), (b"aWb\x00", 0x80), (b"aXb\x00", 0x81))


def main(src, dst):
    data = bytearray(open(src, "rb").read())
    for placeholder, byte in PATCHES:
        at = data.find(placeholder)
        if at < 0:
            raise SystemExit("placeholder %r not found in %s" % (placeholder, src))
        if data.find(placeholder, at + 1) >= 0:
            raise SystemExit("placeholder %r is not unique in %s" % (placeholder, src))
        data[at + 1] = byte
        print("patched offset %d: %r -> %r" % (at, bytes(placeholder[:3]), bytes(data[at:at + 3])))
    open(dst, "wb").write(bytes(data))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    main(sys.argv[1], sys.argv[2])
