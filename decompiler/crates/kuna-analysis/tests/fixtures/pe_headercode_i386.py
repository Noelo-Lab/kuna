#!/usr/bin/env python3
"""Generate `pe_headercode_i386.exe`: a minimal PE32 that declares its entry
point inside its own header page AND flags every section non-executable
(RE-need `whole-binary-decompilation-treats`, round-4 challenge
5ab77f6333c5d40ad448ca40).

The reported image is a MEW-packed keygenme with two nameless sections, both
characteristics 0xc00000e0 -- CNT_CODE|INITIALIZED|UNINITIALIZED|READ|WRITE with
MEM_EXECUTE clear -- and `AddressOfEntryPoint` 0x154, the byte immediately after
its two-entry section table.  So the image has no executable section for a
discovered entry to land in, and its entry is not in a section at all: the whole
inventory was the two Import Address Table slots and `kuna decompile-all`
answered `count: 0`.  This reproduces that layout with a stub small enough to
read.

    00400154  55                    push ebp
    00400155  89 e5                 mov  ebp,esp
    00400157  68 80 20 40 00        push 0x402080          ; "KERNEL32.dll"
    0040015c  ff 15 00 20 40 00     call [0x402000]        ; LoadLibraryA
    00400162  68 52 20 40 00        push 0x402052          ; "LoadLibraryA"
    00400167  50                    push eax
    00400168  ff 15 04 20 40 00     call [0x402004]        ; GetProcAddress
    0040016e  5d                    pop  ebp
    0040016f  c3                    ret

The first section is virtual-only (`SizeOfRawData` 0) as in the reported image;
the second holds the whole import directory, so the two IAT slots are function
symbols that must stay data while the calls through them keep their names.

    python3 pe_headercode_i386.py
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

STAGE_RVA = 0x1000          # virtual-only, the packer's unpack destination
DATA_RVA = 0x2000           # the import directory lives here

IAT_RVA = 0x2000            # LoadLibraryA, GetProcAddress, terminator
DESC_RVA = 0x2010           # one IMAGE_IMPORT_DESCRIPTOR + the null one
INT_RVA = 0x2040
NAME_LOADLIB_RVA = 0x2050   # hint word + "LoadLibraryA\0"
NAME_GETPROC_RVA = 0x2060
DLL_RVA = 0x2080

# CNT_CODE | CNT_INITIALIZED_DATA | CNT_UNINITIALIZED_DATA | MEM_READ |
# MEM_WRITE -- the reported image's characteristics verbatim, MEM_EXECUTE clear.
SECT_CHARS = 0xc00000e0

IMPORTS = [('LoadLibraryA', NAME_LOADLIB_RVA), ('GetProcAddress', NAME_GETPROC_RVA)]


def build_entry():
    """LoadLibraryA("KERNEL32.dll") then GetProcAddress(it, "LoadLibraryA")."""
    return bytes([0x55, 0x89, 0xe5, 0x68]) \
        + struct.pack('<I', IMAGE_BASE + DLL_RVA) \
        + bytes([0xff, 0x15]) + struct.pack('<I', IMAGE_BASE + IAT_RVA) \
        + bytes([0x68]) + struct.pack('<I', IMAGE_BASE + NAME_LOADLIB_RVA + 2) \
        + bytes([0x50, 0xff, 0x15]) + struct.pack('<I', IMAGE_BASE + IAT_RVA + 4) \
        + bytes([0x5d, 0xc3])


def build_data():
    """The import directory of the second section, as a FILE_ALIGN-sized block."""
    blk = bytearray(FILE_ALIGN)

    def put(rva, raw):
        off = rva - DATA_RVA
        blk[off:off + len(raw)] = raw

    # IAT and INT: the same two hint/name pointers, terminated by a null word.
    thunks = b''.join(struct.pack('<I', rva) for _name, rva in IMPORTS) + b'\0\0\0\0'
    put(IAT_RVA, thunks)
    put(INT_RVA, thunks)
    # One descriptor for KERNEL32.dll, then the null descriptor.
    put(DESC_RVA, struct.pack('<IIIII', INT_RVA, 0, 0, DLL_RVA, IAT_RVA) + b'\0' * 20)
    for name, rva in IMPORTS:
        put(rva, b'\0\0' + name.encode() + b'\0')
    put(DLL_RVA, b'KERNEL32.dll\0')
    return bytes(blk)


def build():
    entry = build_entry()
    assert ENTRY_RVA + len(entry) <= FILE_ALIGN, 'the stub must fit in the header page'

    b = bytearray(b'MZ')
    b += b'\0' * (NT_OFF - len(b))

    b += b'PE\0\0'
    b += struct.pack('<HHIIIHH', 0x14c, 2, 0, 0, 0, OPT_SIZE, 0x010f)

    opt = struct.pack('<HBB', 0x10b, 14, 0)                  # magic, linker version
    opt += struct.pack('<III', 0, FILE_ALIGN, SECT_ALIGN)    # code/init/uninit sizes
    opt += struct.pack('<II', ENTRY_RVA, STAGE_RVA)          # entry, BaseOfCode
    # BaseOfData sits at file offset 0x3c, i.e. it *is* the DOS header's
    # `e_lfanew`: writing NT_OFF here is what puts the PE headers at 0x0c.
    opt += struct.pack('<I', NT_OFF)
    opt += struct.pack('<III', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)      # OS/image/subsystem versions
    opt += struct.pack('<III', DATA_RVA + SECT_ALIGN, FILE_ALIGN, 0)  # SizeOfImage/Headers/CheckSum
    opt += struct.pack('<HH', 3, 0)                          # Subsystem = CONSOLE
    opt += struct.pack('<IIII', 0x100000, 0x1000, 0x100000, 0x1000)   # stack/heap
    opt += struct.pack('<I', 0)                              # LoaderFlags
    opt += struct.pack('<I', DIRS)
    assert len(opt) == 96, len(opt)
    dirs = [(0, 0)] * DIRS
    dirs[1] = (DESC_RVA, 40)                                 # Import
    dirs[12] = (IAT_RVA, 12)                                 # IAT
    opt += b''.join(struct.pack('<II', rva, size) for rva, size in dirs)
    assert len(opt) == OPT_SIZE, len(opt)
    b += opt
    assert len(b) == 0x104, hex(len(b))

    # Two nameless sections, as in the reported image: the table ends at 0x154.
    b += b'\0' * 8 + struct.pack(
        '<IIIIIIHHI', SECT_ALIGN, STAGE_RVA, 0, 0, 0, 0, 0, 0, SECT_CHARS)
    b += b'\0' * 8 + struct.pack(
        '<IIIIIIHHI', SECT_ALIGN, DATA_RVA, FILE_ALIGN, FILE_ALIGN, 0, 0, 0, 0, SECT_CHARS)
    assert len(b) == ENTRY_RVA, hex(len(b))

    b += entry
    b += b'\0' * (FILE_ALIGN - len(b))
    b += build_data()
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_headercode_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
