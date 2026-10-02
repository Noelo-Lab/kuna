#!/usr/bin/env python3
"""Generate `floatret_restart_pe_i386.exe`: a minimal PE32 whose `f` returns the
handle `GetStdHandle` gave it on the one path that returns, and whose entry
hands what `f` returns to `ExitProcess` (minipig's `sub_401937` shape, MinGW
-O0: every import is called as `mov eax,[iat]; call eax`).

    00401000  .text  entry     push -11 / call f / add esp,4 / push eax /
                               mov eax,[ExitProcess] / call eax / ret
    00401020  .text  f         push ebp / mov ebp,esp / sub esp,0x18 /
                               mov eax,[ebp+8] / mov [esp],eax /
                               mov eax,[GetStdHandle] / call eax / sub esp,4 /
                               mov [ebp-4],eax / cmp dword [ebp-4],-1 / je done /
                               mov dword [esp],0 / mov eax,[ExitProcess] /
                               call eax / done: leave / ret
    00402000  .rdata IAT       GetStdHandle, ExitProcess
    00402020  .rdata           import descriptors, INT, hint/name blob, DLL name

The entry reads `f`'s `eax`, so `decompile-all` decompiles `f` again to return
it. That decompile restarts once the second indirect call resolves to
`ExitProcess`, which never returns: before the restart the RETURN's `eax`
joins the handle with what the unresolved call left, and is returned; after
it, `eax` is the handle the comparison also reads, and is not. The output the
first pass recovered must not survive the restart, which printed `unsigned int
f(..)` around a bare `return;`.

    python3 floatret_restart_pe_i386.py
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
ENTRY_RVA = 0x1000
DATA_RVA = 0x2000
IAT_RVA = 0x2000
DESC_RVA = 0x2020
INT_RVA = 0x2048
NAMES_RVA = 0x2060
DLL_RVA = 0x20A0

CODE_CHARS = 0x60000020  # CNT_CODE | MEM_EXECUTE | MEM_READ
DATA_CHARS = 0xC0000040  # CNT_INITIALIZED_DATA | MEM_READ | MEM_WRITE

IMPORTS = ['GetStdHandle', 'ExitProcess']
F_RVA = 0x1020


def build_entry():
    """The reader: `ExitProcess(f(-11))`."""
    exit_process = IMAGE_BASE + IAT_RVA + 4
    call = F_RVA - (ENTRY_RVA + 2 + 5)
    return bytes([0x6A, 0xF5]) + b'\xE8' + struct.pack('<i', call) + bytes([
        0x83, 0xC4, 0x04,                           # add  esp,4
        0x50,                                       # push eax
    ]) + b'\xA1' + struct.pack('<I', exit_process) + bytes([
        0xFF, 0xD0,                                 # call eax
        0xC3,                                       # ret
    ])


def build_f():
    """Returns the handle on the path that returns; exits on the other."""
    get_std_handle = IMAGE_BASE + IAT_RVA
    exit_process = IMAGE_BASE + IAT_RVA + 4
    return bytes([
        0x55,                                       # push ebp
        0x89, 0xE5,                                 # mov  ebp,esp
        0x83, 0xEC, 0x18,                           # sub  esp,0x18
        0x8B, 0x45, 0x08,                           # mov  eax,[ebp+8]
        0x89, 0x04, 0x24,                           # mov  [esp],eax
    ]) + b'\xA1' + struct.pack('<I', get_std_handle) + bytes([
        0xFF, 0xD0,                                 # call eax
        0x83, 0xEC, 0x04,                           # sub  esp,4
        0x89, 0x45, 0xFC,                           # mov  [ebp-4],eax
        0x83, 0x7D, 0xFC, 0xFF,                     # cmp  dword [ebp-4],-1
        0x74, 0x0E,                                 # je   done
        0xC7, 0x04, 0x24, 0x00, 0x00, 0x00, 0x00,   # mov  dword [esp],0
    ]) + b'\xA1' + struct.pack('<I', exit_process) + bytes([
        0xFF, 0xD0,                                 # call eax
        0xC9,                                       # done: leave
        0xC3,                                       # ret
    ])


def build_text():
    body = bytearray(b'\0' * SECT_ALIGN)
    entry = build_entry()
    body[ENTRY_RVA - TEXT_RVA:ENTRY_RVA - TEXT_RVA + len(entry)] = entry
    f = build_f()
    body[F_RVA - TEXT_RVA:F_RVA - TEXT_RVA + len(f)] = f
    return bytes(body)


def build_data():
    body = bytearray(b'\0' * SECT_ALIGN)

    def put(rva, data):
        off = rva - DATA_RVA
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

    put(DLL_RVA, b'KERNEL32.dll\0')
    put(DESC_RVA, struct.pack('<IIIII', INT_RVA, 0, 0, DLL_RVA, IAT_RVA))
    put(DESC_RVA + 20, b'\0' * 20)
    return bytes(body)


def section(name, vsize, rva, raw_size, raw_off, chars):
    return name.ljust(8, '\0').encode() + struct.pack(
        '<IIIIIIHHI', vsize, rva, raw_size, raw_off, 0, 0, 0, 0, chars)


def build():
    text = build_text()
    data = build_data()

    b = bytearray(b'MZ')
    b += b'\0' * (0x3C - len(b))
    b += struct.pack('<I', 0x40)
    b += b'\0' * (0x40 - len(b))

    b += b'PE\0\0'
    b += struct.pack('<HHIIIHH', 0x14C, 2, 0, 0, 0, OPT_SIZE, 0x010F)

    opt = struct.pack('<HBB', 0x10B, 14, 0)
    opt += struct.pack('<III', SECT_ALIGN, 0, 0)
    opt += struct.pack('<II', ENTRY_RVA, TEXT_RVA)
    opt += struct.pack('<I', DATA_RVA)
    opt += struct.pack('<III', IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack('<HHHHHHI', 4, 0, 0, 0, 4, 0, 0)
    opt += struct.pack('<III', DATA_RVA + SECT_ALIGN, FILE_ALIGN, 0)
    opt += struct.pack('<HH', 3, 0)
    opt += struct.pack('<IIII', 0x100000, 0x1000, 0x100000, 0x1000)
    opt += struct.pack('<I', 0)
    opt += struct.pack('<I', DIRS)
    assert len(opt) == 96, len(opt)
    dirs = [(0, 0)] * DIRS
    dirs[1] = (DESC_RVA, 40)
    dirs[12] = (IAT_RVA, 16)
    for rva, size in dirs:
        opt += struct.pack('<II', rva, size)
    assert len(opt) == OPT_SIZE, len(opt)
    b += opt

    b += section('.text', SECT_ALIGN, TEXT_RVA, SECT_ALIGN, FILE_ALIGN, CODE_CHARS)
    b += section('.rdata', SECT_ALIGN, DATA_RVA, SECT_ALIGN,
                 FILE_ALIGN + SECT_ALIGN, DATA_CHARS)
    assert len(b) <= FILE_ALIGN, hex(len(b))

    b += b'\0' * (FILE_ALIGN - len(b))
    b += text
    b += data
    return bytes(b)


if __name__ == '__main__':
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'floatret_restart_pe_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
