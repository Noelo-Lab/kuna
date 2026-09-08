#!/usr/bin/env python3
"""Generate `pe_iatincode_i386.exe`: a minimal PE32 whose Import Address Table
lives INSIDE its one executable code section (RE-need
`bulk-decompilation-decodes-pe`, round-9 challenge 5ab77f5c33c5d40ad448c681).

The reported image is a crypter whose two sections are both unnamed and whose
first one -- characteristics 0xe0000060, i.e. CODE|EXECUTE|READ|WRITE --
contains the whole import directory, IAT included.  Every other PE kuna sees
keeps its IAT in `.idata`/`.rdata`, so "is this address in a CODE section?" was
enough to keep the pointer slots out of the whole-binary decompile set; here it
is not, and 50 of the image's 56 inventory entries were import pointer words
that decompiled to a body dereferencing an uninitialized pointer.

    00401000  IAT              VirtualAlloc, GetModuleHandleA, ExitProcess
    00401010  entry            calls GetModuleHandleA and VirtualAlloc through
                               the slots at 0x401004 / 0x401000
    00401040  import directory descriptors, INT, hint/name blobs, DLL name

`kuna decompile-all` must report the entry and NOT the three slots, while
`kuna functions` keeps all four and the two calls still render their import
names.

    python3 pe_iatincode_i386.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
DIRS = 16
OPT_SIZE = 0xE0

SECT_RVA = 0x1000
IAT_RVA = 0x1000
ENTRY_RVA = 0x1010
DESC_RVA = 0x1040
INT_RVA = 0x1068
NAMES_RVA = 0x1080
DLL_RVA = 0x10C0

# CNT_CODE | CNT_INITIALIZED_DATA | MEM_EXECUTE | MEM_READ | MEM_WRITE -- the
# reported image's characteristics verbatim.
CODE_CHARS = 0xE0000060

IMPORTS = ['VirtualAlloc', 'GetModuleHandleA', 'ExitProcess']


def build_entry():
    """push 0; call [GetModuleHandleA]; push eax; push 0x40; call [VirtualAlloc]."""
    return bytes([
        0x55,                                      # push ebp
        0x89, 0xE5,                                # mov  ebp,esp
        0x6A, 0x00,                                # push 0
        0xFF, 0x15,                                # call dword ptr [...]
    ]) + struct.pack('<I', IMAGE_BASE + IAT_RVA + 4) + bytes([
        0x50,                                      # push eax
        0x6A, 0x40,                                # push 0x40
        0xFF, 0x15,                                # call dword ptr [...]
    ]) + struct.pack('<I', IMAGE_BASE + IAT_RVA) + bytes([
        0x5D,                                      # pop  ebp
        0xC3,                                      # ret
    ])


def build_section():
    """The one section: IAT, code, import descriptors, INT and the name blobs."""
    body = bytearray(b'\0' * SECT_ALIGN)

    def put(rva, data):
        off = rva - SECT_RVA
        body[off:off + len(data)] = data

    # Hint/name blobs, and the INT/IAT thunk words that point at them.
    name_rvas = []
    at = NAMES_RVA
    for name in IMPORTS:
        name_rvas.append(at)
        blob = struct.pack('<H', 0) + name.encode() + b'\0'
        put(at, blob)
        at += len(blob) + (len(blob) & 1)

    thunks = b''.join(struct.pack('<I', rva) for rva in name_rvas) + b'\0\0\0\0'
    put(INT_RVA, thunks)
    # The IAT holds the same name thunks pre-binding: this is what the Windows
    # loader overwrites with the resolved addresses.
    put(IAT_RVA, thunks)

    put(DLL_RVA, b'KERNEL32.dll\0')
    # One descriptor plus the null terminator.
    put(DESC_RVA, struct.pack('<IIIII', INT_RVA, 0, 0, DLL_RVA, IAT_RVA))
    put(DESC_RVA + 20, b'\0' * 20)

    put(ENTRY_RVA, build_entry())
    return bytes(body)


def section(name, vsize, rva, raw_size, raw_off):
    return name.ljust(8, '\0').encode() + struct.pack(
        '<IIIIIIHHI', vsize, rva, raw_size, raw_off, 0, 0, 0, 0, CODE_CHARS)


def build():
    sect = build_section()

    b = bytearray(b'MZ')
    b += b'\0' * (0x3C - len(b))
    b += struct.pack('<I', 0x40)
    b += b'\0' * (0x40 - len(b))

    b += b'PE\0\0'
    b += struct.pack('<HHIIIHH', 0x14C, 1, 0, 0, 0, OPT_SIZE, 0x010F)

    opt = struct.pack('<HBB', 0x10B, 14, 0)                  # magic, linker version
    opt += struct.pack('<III', SECT_ALIGN, 0, 0)             # code/init/uninit sizes
    opt += struct.pack('<II', ENTRY_RVA, SECT_RVA)           # entry, BaseOfCode
    opt += struct.pack('<I', SECT_RVA)                       # BaseOfData
    opt += struct.pack('<III', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)      # OS/image/subsystem versions
    opt += struct.pack('<III', SECT_RVA + SECT_ALIGN, FILE_ALIGN, 0)  # image/headers/checksum
    opt += struct.pack('<HH', 3, 0)                          # Subsystem = CONSOLE
    opt += struct.pack('<IIII', 0x100000, 0x1000, 0x100000, 0x1000)   # stack/heap
    opt += struct.pack('<I', 0)                              # LoaderFlags
    opt += struct.pack('<I', DIRS)
    assert len(opt) == 96, len(opt)
    dirs = [(0, 0)] * DIRS
    dirs[1] = (DESC_RVA, 40)                                 # Import Directory
    dirs[12] = (IAT_RVA, 16)                                 # IAT
    for rva, size in dirs:
        opt += struct.pack('<II', rva, size)
    assert len(opt) == OPT_SIZE, len(opt)
    b += opt

    # Unnamed, exactly as the reported image's two sections are.
    b += section('', SECT_ALIGN, SECT_RVA, SECT_ALIGN, FILE_ALIGN)
    assert len(b) <= FILE_ALIGN, hex(len(b))

    b += b'\0' * (FILE_ALIGN - len(b))
    b += sect
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_iatincode_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
