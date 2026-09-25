#!/usr/bin/env python3
"""Generate `pe_ordinal_i386.exe`: a minimal PE32 whose imports are all by
ordinal, with no name in the Import Name Table.

    00401000  IAT   OLEAUT32.dll #2 (SysAllocString), #6 (SysFreeString)
    00401010  IAT   WS2_32.dll   #23 (socket), #115 (WSAStartup)
    00401020  IAT   MYLIB.dll    #7 (not a covered DLL)
    00401024  IAT   OLEAUT32.dll #9999 (not in the table)
    00401100  entry calls each slot through `call dword ptr [slot]`

With `peordinal` on the covered slots are named from the built-in export table;
`MYLIB_Ordinal_7` and `OLEAUT32_Ordinal_9999` keep the synthesized names.

    python3 pe_ordinal_i386.py
"""
import os
import struct

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
DIRS = 16
OPT_SIZE = 0xE0
SECT_RVA = 0x1000
CODE_CHARS = 0xE0000020  # CNT_CODE | MEM_EXECUTE | MEM_READ | MEM_WRITE

# (dll, [ordinals], IAT rva, INT rva, name rva)
DLLS = [
    ('OLEAUT32.dll', [2, 6], 0x1000, 0x1040, 0x1100),
    ('WS2_32.dll', [23, 115], 0x1010, 0x1050, 0x1110),
    ('MYLIB.dll', [7], 0x1020, 0x1060, 0x1120),
    ('OLEAUT32.dll', [9999], 0x1028, 0x1068, 0x1130),
]
DESC_RVA = 0x1080
ENTRY_RVA = 0x1200
WSTR_RVA = 0x1300


def slot(dll_index, i):
    return IMAGE_BASE + DLLS[dll_index][2] + 4 * i


def call_slot(va):
    return b'\xFF\x15' + struct.pack('<I', va)


def build_entry():
    code = b''
    code += b'\x68' + struct.pack('<I', IMAGE_BASE + WSTR_RVA)   # push L"kuna"
    code += call_slot(slot(0, 0))                                 # SysAllocString
    code += b'\x50'                                               # push eax
    code += call_slot(slot(0, 1))                                 # SysFreeString
    code += b'\x6A\x06\x6A\x01\x6A\x02'                           # push 6; push 1; push 2
    code += call_slot(slot(1, 0))                                 # socket
    code += b'\x6A\x00\x68' + struct.pack('<I', 0x202)            # push 0; push 0x202
    code += call_slot(slot(1, 1))                                 # WSAStartup
    code += call_slot(slot(2, 0))                                 # MYLIB #7
    code += call_slot(slot(3, 0))                                 # OLEAUT32 #9999
    code += b'\x31\xC0\xC3'                                       # xor eax,eax; ret
    return code


def build_section():
    body = bytearray(b'\0' * SECT_ALIGN)

    def put(rva, data):
        off = rva - SECT_RVA
        body[off:off + len(data)] = data

    descs = b''
    for name, ords, iat, int_rva, name_rva in DLLS:
        thunks = b''.join(struct.pack('<I', 0x80000000 | o) for o in ords) + b'\0\0\0\0'
        put(iat, thunks)
        put(int_rva, thunks)
        put(name_rva, name.encode() + b'\0')
        descs += struct.pack('<IIIII', int_rva, 0, 0, name_rva, iat)
    put(DESC_RVA, descs + b'\0' * 20)
    put(WSTR_RVA, 'kuna\0'.encode('utf-16-le'))
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
    dirs[1] = (DESC_RVA, 20 * (len(DLLS) + 1))
    dirs[12] = (0x1000, 0x30)
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
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'pe_ordinal_i386.exe')
    with open(out, 'wb') as f:
        f.write(build())
    print(out)
