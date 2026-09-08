#!/usr/bin/env python3
"""Generate `pe_bsstail_x86_64.exe`: a minimal PE32+ whose `.data` section
declares more RAM than the file backs (RE-need `pe-zero-filled-data`, round-8
challenge 640a526833c5d447bc761899).

The reported image's `.data` sits at RVA 0x8000 with `VirtualSize` 0x7a8 and
`SizeOfRawData` 0x200: Windows maps 0x7a8 bytes there and zero-fills everything
past the 0x200 the file supplies, which is where its C runtime keeps its
zero-initialized globals. This twin reproduces exactly that shape one section
lower, at RVA 0x2000 (`ImageBase` 0x140000000, so vma 0x140002000):

    0x140002000 .. 0x1400021ff   file-backed, 0xaa at the last eight bytes
    0x140002200 .. 0x1400027a7   zero-filled RAM tail
    0x1400027a8 ..               past VirtualSize, unmapped

    python3 pe_bsstail_x86_64.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
NT_OFF = 0x40
DIRS = 16
OPT_SIZE = 0xf0
TEXT_RVA = 0x1000
DATA_RVA = 0x2000
DATA_VSIZE = 0x7a8

TEXT_CODE = bytes([
    0x8b, 0x04, 0x25, 0x00, 0x20, 0x00, 0x40,  # mov eax,[0x40002000]
    0xc3,                                      # ret
])


def build():
    b = bytearray(b'MZ')
    b += b'\0' * (0x3c - len(b))
    b += struct.pack('<I', NT_OFF)
    b += b'\0' * (NT_OFF - len(b))

    b += b'PE\0\0'
    b += struct.pack('<HHIIIHH', 0x8664, 2, 0, 0, 0, OPT_SIZE, 0x0022)

    opt = struct.pack('<HBB', 0x20b, 14, 0)                   # magic, linker version
    opt += struct.pack('<III', FILE_ALIGN, FILE_ALIGN, 0)     # code/init/uninit sizes
    opt += struct.pack('<II', TEXT_RVA, TEXT_RVA)             # entry, BaseOfCode
    opt += struct.pack('<QII', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 6, 0, 0, 0, 6, 0, 0)       # OS/image/subsystem versions
    opt += struct.pack('<III', DATA_RVA + SECT_ALIGN, FILE_ALIGN, 0)  # image/headers/checksum
    opt += struct.pack('<HH', 3, 0)                           # Subsystem = CONSOLE
    opt += struct.pack('<QQQQ', 0x100000, 0x1000, 0x100000, 0x1000)   # stack/heap
    opt += struct.pack('<I', 0)                               # LoaderFlags
    opt += struct.pack('<I', DIRS)
    assert len(opt) == 112, len(opt)
    opt += b'\0' * (DIRS * 8)
    assert len(opt) == OPT_SIZE, len(opt)
    b += opt

    b += b'.text\0\0\0' + struct.pack(
        '<IIIIIIHHI', len(TEXT_CODE), TEXT_RVA, FILE_ALIGN, FILE_ALIGN, 0, 0, 0, 0, 0x60000020)
    # VirtualSize 0x7a8 over SizeOfRawData 0x200: the zero-filled tail.
    b += b'.data\0\0\0' + struct.pack(
        '<IIIIIIHHI', DATA_VSIZE, DATA_RVA, FILE_ALIGN, 2 * FILE_ALIGN, 0, 0, 0, 0, 0xc0000040)
    assert len(b) <= FILE_ALIGN, hex(len(b))

    b += b'\0' * (FILE_ALIGN - len(b))
    b += TEXT_CODE.ljust(FILE_ALIGN, b'\0')
    data = bytearray(FILE_ALIGN)
    data[-8:] = b'\xaa' * 8          # the last byte the file actually backs
    b += data
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_bsstail_x86_64.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
