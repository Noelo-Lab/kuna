#!/usr/bin/env python3
"""Generate `pe_dataimports_i386.exe`: a minimal PE32 that flags every section
as data AND carries an import table, so its whole function inventory is import
pointer slots (RE-need `function-inventory-silently-lists`, round-12 challenge
5ab77f5433c5d40ad448c1c1).

The reported image is the same NEOLite-packed crackme as
`pe_datasection_entry_i386.exe`, but seen through `kuna functions` rather than
`kuna decompile-all`: its six sections all carry characteristics 0xc0000040
(INITIALIZED_DATA|READ|WRITE) so nothing is discovered in them, while its
import directory still names six Win32 entry points.  The inventory was
therefore SIX entries and no bodies -- non-empty, so the zero-discovery
diagnostic `decompile-all` prints for the same file never fired.

    00401000  helper   push ebp ; mov ebp,esp ; mov eax,7 ; pop ebp ; ret
    00402001  entry    push ebp ; mov ebp,esp ; call helper ; pop ebp ; ret
    00403000  IAT      GetProcAddress, GetModuleHandleA, LoadLibraryA

`kuna functions` must keep all three import names -- they are how a call to an
import renders -- and still report the run-level discovery failure, because not
one of them is a body.

    python3 pe_dataimports_i386.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
DIRS = 16
OPT_SIZE = 0xE0

TEXT_RVA = 0x1000
STUB_RVA = 0x2000
ENTRY_RVA = 0x2001
ENTRY_END = 0x200C
IDATA_RVA = 0x3000

IAT_RVA = 0x3000
DESC_RVA = 0x3040
INT_RVA = 0x3068
NAMES_RVA = 0x3080
DLL_RVA = 0x30C0

# Neither IMAGE_SCN_CNT_CODE (0x20) nor IMAGE_SCN_MEM_EXECUTE (0x20000000):
# INITIALIZED_DATA | MEM_READ | MEM_WRITE, on every section including `.text`.
DATA_CHARS = 0xC0000040

IMPORTS = ['GetProcAddress', 'GetModuleHandleA', 'LoadLibraryA']

HELPER = bytes([
    0x55,                          # push ebp
    0x89, 0xE5,                    # mov  ebp,esp
    0xB8, 0x07, 0x00, 0x00, 0x00,  # mov  eax,7
    0x5D,                          # pop  ebp
    0xC3,                          # ret
])


def build_stub():
    # The entry sits one byte into the section, so pad the leading byte.
    call_site = ENTRY_RVA + 3          # the 0xE8 opcode
    rel = TEXT_RVA - (call_site + 5)   # relative to the NEXT instruction
    body = bytes([0x55, 0x89, 0xE5, 0xE8]) + struct.pack('<i', rel)
    body += bytes([0x5D, 0xC3, 0x90])  # pop ebp ; ret ; pad to 11 bytes
    assert len(body) == ENTRY_END - ENTRY_RVA, len(body)
    return b'\0' + body


def build_idata():
    """The import directory: descriptors, INT, IAT and the hint/name blobs."""
    body = bytearray(b'\0' * SECT_ALIGN)

    def put(rva, data):
        off = rva - IDATA_RVA
        body[off:off + len(data)] = data

    name_rvas = []
    at = NAMES_RVA
    for name in IMPORTS:
        name_rvas.append(at)
        blob = struct.pack('<H', 0) + name.encode() + b'\0'
        put(at, blob)
        at += len(blob) + (len(blob) & 1)

    thunks = b''.join(struct.pack('<I', rva) for rva in name_rvas) + b'\0\0\0\0'
    put(INT_RVA, thunks)
    # Pre-binding the IAT holds the same name thunks: this is what the Windows
    # loader overwrites with the resolved addresses.
    put(IAT_RVA, thunks)

    put(DLL_RVA, b'KERNEL32.dll\0')
    put(DESC_RVA, struct.pack('<IIIII', INT_RVA, 0, 0, DLL_RVA, IAT_RVA))
    put(DESC_RVA + 20, b'\0' * 20)
    return bytes(body)


def section(name, vsize, rva, raw_size, raw_off):
    return name.ljust(8, '\0').encode() + struct.pack(
        '<IIIIIIHHI', vsize, rva, raw_size, raw_off, 0, 0, 0, 0, DATA_CHARS)


def build():
    stub = build_stub()
    idata = build_idata()

    b = bytearray(b'MZ')
    b += b'\0' * (0x3C - len(b))
    b += struct.pack('<I', 0x40)
    b += b'\0' * (0x40 - len(b))

    b += b'PE\0\0'
    b += struct.pack('<HHIIIHH', 0x14C, 3, 0, 0, 0, OPT_SIZE, 0x010F)

    opt = struct.pack('<HBB', 0x10B, 14, 0)                  # magic, linker version
    opt += struct.pack('<III', 0, len(HELPER) + len(stub), 0)  # code/init/uninit sizes
    opt += struct.pack('<II', ENTRY_RVA, TEXT_RVA)           # entry, BaseOfCode
    opt += struct.pack('<I', TEXT_RVA)                       # BaseOfData
    opt += struct.pack('<III', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)      # OS/image/subsystem versions
    opt += struct.pack('<III', IDATA_RVA + SECT_ALIGN, FILE_ALIGN, 0)  # image/headers/checksum
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

    b += section('.text', len(HELPER), TEXT_RVA, FILE_ALIGN, FILE_ALIGN)
    b += section('.stub', len(stub), STUB_RVA, FILE_ALIGN, 2 * FILE_ALIGN)
    b += section('.idata', SECT_ALIGN, IDATA_RVA, SECT_ALIGN, 3 * FILE_ALIGN)
    assert len(b) <= FILE_ALIGN, hex(len(b))

    b += b'\0' * (FILE_ALIGN - len(b))
    b += HELPER.ljust(FILE_ALIGN, b'\0')
    b += stub.ljust(FILE_ALIGN, b'\0')
    b += idata
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_dataimports_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
