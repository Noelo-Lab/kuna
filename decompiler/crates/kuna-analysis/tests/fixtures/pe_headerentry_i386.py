#!/usr/bin/env python3
"""Generate `pe_headerentry_i386.exe`: a minimal PE32 whose declared entry point
lives in the header page, the byte after its own section table (RE-need
`pe-header-entry-mapped`, round-4 challenge 5ab77f6333c5d40ad448ca40).

Windows maps `SizeOfHeaders` file bytes at `ImageBase` before it maps a single
section, so an image is free to put code there -- and the reported one did. This
reproduces its layout exactly: `e_lfanew` 0x0c, so the PE signature overlaps the
DOS header and the optional header's `BaseOfData` field *is* the `e_lfanew` word
at 0x3c; a 0xe0-byte optional header; two sections, whose table therefore runs
0x104..0x154; and `AddressOfEntryPoint` 0x154, one byte past its end.

    00400154  55              push ebp
    00400155  89 e5           mov  ebp,esp
    00400157  8b 45 08        mov  eax,[ebp+8]
    0040015a  83 c0 07        add  eax,7
    0040015d  5d              pop  ebp
    0040015e  c3              ret

The first section is virtual-only (`SizeOfRawData` 0), as in the reported image,
so nothing but the header page backs the entry.

    python3 pe_headerentry_i386.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
NT_OFF = 0x0c
DIRS = 16
OPT_SIZE = 0xe0
ENTRY_RVA = 0x154
TEXT_RVA = 0x2000

ENTRY_CODE = bytes([
    0x55,                          # push ebp
    0x89, 0xe5,                    # mov  ebp,esp
    0x8b, 0x45, 0x08,              # mov  eax,[ebp+8]
    0x83, 0xc0, 0x07,              # add  eax,7
    0x5d,                          # pop  ebp
    0xc3,                          # ret
])

TEXT_CODE = bytes([
    0x55,                          # push ebp
    0x89, 0xe5,                    # mov  ebp,esp
    0xb8, 0x01, 0x00, 0x00, 0x00,  # mov  eax,1
    0x5d,                          # pop  ebp
    0xc3,                          # ret
])


def build():
    b = bytearray(b'MZ')
    b += b'\0' * (NT_OFF - len(b))

    b += b'PE\0\0'
    b += struct.pack('<HHIIIHH', 0x14c, 2, 0, 0, 0, OPT_SIZE, 0x010f)

    opt = struct.pack('<HBB', 0x10b, 14, 0)                  # magic, linker version
    opt += struct.pack('<III', len(TEXT_CODE), 0, 0)         # code/init/uninit sizes
    opt += struct.pack('<II', ENTRY_RVA, TEXT_RVA)           # entry, BaseOfCode
    # BaseOfData sits at file offset 0x3c, i.e. it *is* the DOS header's
    # `e_lfanew`: writing NT_OFF here is what puts the PE headers at 0x0c.
    opt += struct.pack('<I', NT_OFF)
    opt += struct.pack('<III', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)      # OS/image/subsystem versions
    opt += struct.pack('<III', TEXT_RVA + SECT_ALIGN, FILE_ALIGN, 0)  # SizeOfImage/Headers/CheckSum
    opt += struct.pack('<HH', 3, 0)                          # Subsystem = CONSOLE
    opt += struct.pack('<IIII', 0x100000, 0x1000, 0x100000, 0x1000)   # stack/heap
    opt += struct.pack('<I', 0)                              # LoaderFlags
    opt += struct.pack('<I', DIRS)
    assert len(opt) == 96, len(opt)
    opt += b'\0' * (DIRS * 8)
    assert len(opt) == OPT_SIZE, len(opt)
    b += opt
    assert len(b) == 0x104, hex(len(b))

    # A virtual-only section, then the real one: the table ends at 0x154.
    b += b'\0' * 8 + struct.pack(
        '<IIIIIIHHI', SECT_ALIGN, SECT_ALIGN, 0, 0, 0, 0, 0, 0, 0xc0000040)
    b += b'.text\0\0\0' + struct.pack(
        '<IIIIIIHHI', len(TEXT_CODE), TEXT_RVA, FILE_ALIGN, FILE_ALIGN, 0, 0, 0, 0, 0x60000020)
    assert len(b) == ENTRY_RVA, hex(len(b))

    b += ENTRY_CODE
    b += b'\0' * (FILE_ALIGN - len(b))
    b += TEXT_CODE.ljust(FILE_ALIGN, b'\0')
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_headerentry_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
