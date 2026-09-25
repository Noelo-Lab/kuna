#!/usr/bin/env python3
"""Generate `pe_dosmagic_i386.exe`: the two-function PE32 of
`pe_datadircount_i386.py`, with a well-formed data-directory count and a
corrupted DOS `e_magic` (RE-need `pe-dos-magic`).

The first byte is 0x15 instead of 'M' (`e_magic` 0x5a15), as in the reported
image, while `e_lfanew` still points at an intact `PE\\0\\0` signature. `object`
sniffs the format from the magic alone, so every kuna surface used to reject
the file as "Unknown file magic" before a byte of code was mapped.

    python3 pe_dosmagic_i386.py
"""
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pe_datadircount_i386 as base  # noqa: E402

NRVA_OFFSET = 0x40 + 4 + 20 + 92


def build():
    b = bytearray(base.build())
    struct.pack_into('<I', b, NRVA_OFFSET, base.DIRS)
    b[0] = 0x15
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_dosmagic_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
