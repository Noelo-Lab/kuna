#!/usr/bin/env python3
"""Generate `win32sigs_pe_i386.exe`: a minimal PE32 that calls `LoadLibraryExW`
through an `FF 25` thunk, in the shape that loses its arguments (RE-need
`resource-loader-c-retains`, round-7 challenge 6442366033c5d43938912a85).

The reported image (a 193 KB Free Pascal resource loader) fills its outgoing
argument slots several instructions BEFORE the call, with an unrelated call in
between, and then calls with nothing pushed at the call site itself.  Call-site
argument recovery has nothing local to score, so the call renders
`LoadLibraryExW()` and the writes -- plus the CALL's own return-address push --
survive as mapped stack locals.  The entry below reproduces that shape.

    00401000  IAT              LoadLibraryExW
    00401010  thunk            jmp dword ptr [0x401000]   (`FF 25`)
    00401020  entry            fills [esp]/[esp+4]/[esp+8], calls 0x401080,
                               `add [esp],edx`, then calls the thunk
    00401060  import directory descriptors, INT, hint/name blob, DLL name
    00401080  helper           xor edx,edx; ret

Without a Win32 signature the call keeps ONE argument
(`LoadLibraryExW((int)v2 + v1)`) and the two it dropped are attributed to the
intervening helper; with one it is `LoadLibraryExW(L"kernel32.dll",NULL,8)`.

    python3 win32sigs_pe_i386.py
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
THUNK_RVA = 0x1010
ENTRY_RVA = 0x1020
DESC_RVA = 0x1060
INT_RVA = 0x1088
NAMES_RVA = 0x10A0
DLL_RVA = 0x10C0
WNAME_RVA = 0x10E0
HELPER_RVA = 0x1080

CODE_CHARS = 0x60000020  # CNT_CODE | MEM_EXECUTE | MEM_READ

IMPORTS = ['LoadLibraryExW']


def build_entry():
    """The witness shape: reserve the outgoing slots with pushes, fill them well
    before the call, then `call thunk` with nothing pushed at the call itself."""
    head = bytes([
        0x53,                                       # push ebx
        0x31, 0xC0,                                 # xor  eax,eax
        0x50, 0x50, 0x50,                           # push eax x3   (12 bytes of
                                                    #                outgoing area)
        0xC7, 0x44, 0x24, 0x08, 0x08, 0x00, 0x00, 0x00,   # mov [esp+8],8
        0xC7, 0x44, 0x24, 0x04, 0x00, 0x00, 0x00, 0x00,   # mov [esp+4],0
        0xC7, 0x04, 0x24,                           # mov  dword ptr [esp],
    ]) + struct.pack('<I', IMAGE_BASE + WNAME_RVA)
    # `call rel32` is 5 bytes and rel32 is relative to the NEXT instruction.
    # The intervening call is what separates the argument stores from the API
    # call, which is the whole point of the fixture.
    head += b'\xE8' + struct.pack('<i', HELPER_RVA - (ENTRY_RVA + len(head) + 5))
    head += bytes([0x01, 0x14, 0x24])               # add dword ptr [esp],edx
    return head + b'\xE8' + struct.pack(
        '<i', THUNK_RVA - (ENTRY_RVA + len(head) + 5)) + bytes([
            0x5B,                                   # pop  ebx
            0xC3,                                   # ret
        ])


def build_section():
    body = bytearray(b'\0' * SECT_ALIGN)

    def put(rva, data):
        off = rva - SECT_RVA
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
    put(IAT_RVA, thunks)

    # The `FF 25` veneer a direct `call` targets: jmp dword ptr [IAT slot].
    put(THUNK_RVA, b'\xFF\x25' + struct.pack('<I', IMAGE_BASE + IAT_RVA))

    put(DLL_RVA, b'KERNEL32.dll\0')
    put(DESC_RVA, struct.pack('<IIIII', INT_RVA, 0, 0, DLL_RVA, IAT_RVA))
    put(DESC_RVA + 20, b'\0' * 20)

    # The UTF-16LE library name the call passes.
    put(WNAME_RVA, 'kernel32.dll\0'.encode('utf-16-le'))

    put(ENTRY_RVA, build_entry())
    put(HELPER_RVA, bytes([0x31, 0xD2, 0xC3]))      # xor edx,edx; ret
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

    opt = struct.pack('<HBB', 0x10B, 14, 0)
    opt += struct.pack('<III', SECT_ALIGN, 0, 0)
    opt += struct.pack('<II', ENTRY_RVA, SECT_RVA)
    opt += struct.pack('<I', SECT_RVA)
    opt += struct.pack('<III', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)
    opt += struct.pack('<III', SECT_RVA + SECT_ALIGN, FILE_ALIGN, 0)
    opt += struct.pack('<HH', 3, 0)
    opt += struct.pack('<IIII', 0x100000, 0x1000, 0x100000, 0x1000)
    opt += struct.pack('<I', 0)
    opt += struct.pack('<I', DIRS)
    assert len(opt) == 96, len(opt)
    dirs = [(0, 0)] * DIRS
    dirs[1] = (DESC_RVA, 40)
    dirs[12] = (IAT_RVA, 8)
    for rva, size in dirs:
        opt += struct.pack('<II', rva, size)
    assert len(opt) == OPT_SIZE, len(opt)
    b += opt

    b += section('.text', SECT_ALIGN, SECT_RVA, SECT_ALIGN, FILE_ALIGN)
    assert len(b) <= FILE_ALIGN, hex(len(b))

    b += b'\0' * (FILE_ALIGN - len(b))
    b += sect
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'win32sigs_pe_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
