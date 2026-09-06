#!/usr/bin/env python3
"""Generate `pe_datadircount_i386.exe`: a minimal PE32 whose
`NumberOfRvaAndSizes` is far larger than its own optional header can hold, while
the 16 real data directories are physically present (RE-need
`pe-data-directory-count`, round-4 challenge 5ab77f5c33c5d40ad448c67e).

The reported image was Invius-packed and carried 1531532893 in that field, in a
224-byte optional header -- room for exactly the 16 directories that were there.
Windows reads `min(declared, what fits)`; `object::File::parse` insists on the
declared count and rejects the whole file with "Invalid PE number of RVA and
sizes", so every kuna surface exited 1 before a byte of code was mapped. That
value is reproduced verbatim here over two functions, the entry stub and the leaf
it calls:

    00401000  55                push ebp
    00401001  89 e5             mov  ebp,esp
    00401003  e8 07 00 00 00    call 0x40100f
    00401008  5d                pop  ebp
    00401009  c3                ret

    0040100f  8b 44 24 04       mov  eax,[esp+4]
    00401013  83 c0 07          add  eax,7
    00401016  c3                ret

    python3 pe_datadircount_i386.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
DIRS = 16

# The trashed count, copied from the reported image.
NUMBER_OF_RVA_AND_SIZES = 1531532893

CODE = bytes([
    0x55,                                 # push ebp
    0x89, 0xE5,                           # mov  ebp,esp
    0xE8, 0x07, 0x00, 0x00, 0x00,         # call 0x40100f
    0x5D,                                 # pop  ebp
    0xC3,                                 # ret
    0xCC, 0xCC, 0xCC, 0xCC, 0xCC,         # int3 padding
    0x8B, 0x44, 0x24, 0x04,               # mov  eax,[esp+4]
    0x83, 0xC0, 0x07,                     # add  eax,7
    0xC3,                                 # ret
])


def build():
    opt_size = 96 + DIRS * 8              # PE32 fixed part + the directories
    headers = 0x40 + 4 + 20 + opt_size + 40
    assert headers <= FILE_ALIGN

    dos = bytearray(0x40)
    dos[0:2] = b'MZ'
    struct.pack_into('<I', dos, 0x3C, 0x40)

    b = bytearray(dos)
    b += b'PE\0\0'
    # IMAGE_FILE_HEADER: i386, one section, EXECUTABLE_IMAGE | 32BIT_MACHINE.
    b += struct.pack('<HHIIIHH', 0x014C, 1, 0, 0, 0, opt_size, 0x0102)

    opt = bytearray()
    opt += struct.pack('<HBBIIIII', 0x10B, 14, 0, len(CODE), 0, 0, TEXT_RVA, TEXT_RVA)
    opt += struct.pack('<III', 0, IMAGE_BASE, SECT_ALIGN)   # BaseOfData, ImageBase, SectionAlignment
    opt += struct.pack('<I', FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)     # OS/image/subsystem versions
    opt += struct.pack('<III', TEXT_RVA + SECT_ALIGN, FILE_ALIGN, 0)  # SizeOfImage/Headers/CheckSum
    opt += struct.pack('<HH', 3, 0)                          # Subsystem = CONSOLE
    opt += struct.pack('<IIII', 0x100000, 0x1000, 0x100000, 0x1000)   # stack/heap
    opt += struct.pack('<I', 0)                              # LoaderFlags
    opt += struct.pack('<I', NUMBER_OF_RVA_AND_SIZES)
    assert len(opt) == 96, len(opt)
    opt += b'\0' * (DIRS * 8)                                # the 16 real, empty directories
    assert len(opt) == opt_size
    b += opt

    b += b'.text\0\0\0' + struct.pack(
        '<IIIIIIHHI', len(CODE), TEXT_RVA, FILE_ALIGN, FILE_ALIGN, 0, 0, 0, 0, 0x60000020)
    b += b'\0' * (FILE_ALIGN - len(b))
    b += CODE.ljust(FILE_ALIGN, b'\0')
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_datadircount_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
