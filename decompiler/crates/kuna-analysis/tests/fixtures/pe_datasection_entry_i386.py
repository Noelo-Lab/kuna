#!/usr/bin/env python3
"""Generate `pe_datasection_entry_i386.exe`: a minimal PE32 that flags EVERY
section as data (RE-need `batch-silently-omits-explicitly`, round-8 challenge
5ab77f5433c5d40ad448c1c1).

The reported image is a NEOLite-packed crackme whose six sections -- `.text`
included -- all carry characteristics 0xc0000040
(INITIALIZED_DATA|READ|WRITE), with neither IMAGE_SCN_MEM_EXECUTE nor
IMAGE_SCN_CNT_CODE anywhere, and whose `AddressOfEntryPoint` lands in the
packer's own `.NEOpack` section.  A packer defeats the section-flag test simply
by not setting it, so a caller who has worked out where a function starts and
passes `--define-function` is the only thing left that knows.  This reproduces
that layout in two sections.

    00401000  helper   push ebp ; mov ebp,esp ; mov eax,7 ; pop ebp ; ret
    00402001  entry    push ebp ; mov ebp,esp ; call helper ; pop ebp ; ret
                       (11 bytes, 0x402001..0x40200c, as in the reported image)

`AddressOfEntryPoint` is 0x2001, so the entry is at an odd offset inside the
stub section exactly as the reported 0x4f7001 is.

    python3 pe_datasection_entry_i386.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
DIRS = 16
OPT_SIZE = 0xe0

TEXT_RVA = 0x1000
STUB_RVA = 0x2000
ENTRY_RVA = 0x2001
ENTRY_END = 0x200C

# Not one of IMAGE_SCN_CNT_CODE (0x20) or IMAGE_SCN_MEM_EXECUTE (0x20000000):
# INITIALIZED_DATA | MEM_READ | MEM_WRITE, on the code section too.
DATA_CHARS = 0xC0000040

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


def section(name, vsize, rva, raw_size, raw_off):
    return name.ljust(8, '\0').encode() + struct.pack(
        '<IIIIIIHHI', vsize, rva, raw_size, raw_off, 0, 0, 0, 0, DATA_CHARS)


def build():
    stub = build_stub()

    b = bytearray(b'MZ')
    b += b'\0' * (0x3C - len(b))
    b += struct.pack('<I', 0x40)
    b += b'\0' * (0x40 - len(b))

    b += b'PE\0\0'
    b += struct.pack('<HHIIIHH', 0x14c, 2, 0, 0, 0, OPT_SIZE, 0x010f)

    opt = struct.pack('<HBB', 0x10b, 14, 0)                  # magic, linker version
    opt += struct.pack('<III', 0, len(HELPER) + len(stub), 0)  # code/init/uninit sizes
    opt += struct.pack('<II', ENTRY_RVA, TEXT_RVA)           # entry, BaseOfCode
    opt += struct.pack('<I', TEXT_RVA)                       # BaseOfData
    opt += struct.pack('<III', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)      # OS/image/subsystem versions
    opt += struct.pack('<III', STUB_RVA + SECT_ALIGN, FILE_ALIGN, 0)  # SizeOfImage/Headers/CheckSum
    opt += struct.pack('<HH', 3, 0)                          # Subsystem = CONSOLE
    opt += struct.pack('<IIII', 0x100000, 0x1000, 0x100000, 0x1000)   # stack/heap
    opt += struct.pack('<I', 0)                              # LoaderFlags
    opt += struct.pack('<I', DIRS)
    assert len(opt) == 96, len(opt)
    opt += b'\0' * (DIRS * 8)
    assert len(opt) == OPT_SIZE, len(opt)
    b += opt

    b += section('.text', len(HELPER), TEXT_RVA, FILE_ALIGN, FILE_ALIGN)
    b += section('.stub', len(stub), STUB_RVA, FILE_ALIGN, 2 * FILE_ALIGN)
    assert len(b) <= FILE_ALIGN, hex(len(b))

    b += b'\0' * (FILE_ALIGN - len(b))
    b += HELPER.ljust(FILE_ALIGN, b'\0')
    b += stub.ljust(FILE_ALIGN, b'\0')
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'pe_datasection_entry_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
