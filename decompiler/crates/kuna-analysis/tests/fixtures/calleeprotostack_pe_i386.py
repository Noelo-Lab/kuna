#!/usr/bin/env python3
"""Generate `calleeprotostack_pe_i386.exe`: a minimal PE32 whose entry stages a
seven-argument `FormatMessageA` around an argument-less `GetLastError` call, in
the shape that loses the four arguments staged first (RE-need
`getlasterror-consumes-arguments-prepared`, round-10 challenge
5ab77f5d33c5d40ad448c6f6).

The reported image is an MSVC error-reporting helper.  It reserves the output
slot with `push ecx`, pushes `FormatMessageA`'s arguments 7..4, calls
`GetLastError` through its IAT slot, pushes arguments 3..1, and calls
`FormatMessageA` through its own slot.  `GetLastError` takes none, so the four
pushes in front of it are the outer call's -- but a call whose model leaves its
extrapop unknown has its cleanup guessed off the caller's push run, and the
guess credits them to the callee they precede.  The entry below is that
sequence, byte for byte.

    00401000  .text  entry     push ecx / push 0 / lea eax,[esp+4] / push 0 /
                               push eax / push 0 / call [GetLastError] /
                               push eax / push 0 / push 0x1300 /
                               call [FormatMessageA] / pop ecx /
                               push 0 / call [ExitProcess] / ret
    00402000  .rdata IAT       GetLastError, FormatMessageA, ExitProcess
    00402020  .rdata           import descriptors, INT, hint/name blob, DLL name

The IAT lives in its own data section, as it does in a real image: an import
slot inside the executable section is discovered as a four-byte FUNCTION and
never becomes the size-0 external the resolver names.

With `option calleeprotostack off` the call renders
`FormatMessageA(0x1300,0,GetLastError())` -- three of its seven arguments, and
the address of the output slot is gone; with it on, all seven survive and the
fifth is `&`-taken.

    python3 calleeprotostack_pe_i386.py
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

IMPORTS = ['GetLastError', 'FormatMessageA', 'ExitProcess']


def build_entry():
    """The witness shape: `FormatMessageA`'s arguments 7..4 are pushed, then an
    argument-less `GetLastError` is called, then arguments 3..1 are pushed."""
    get_last_error = IMAGE_BASE + IAT_RVA
    format_message = IMAGE_BASE + IAT_RVA + 4
    exit_process = IMAGE_BASE + IAT_RVA + 8
    return bytes([
        0x51,                                       # push ecx      (output slot)
        0x6A, 0x00,                                 # push 0        (arg 7)
        0x8D, 0x44, 0x24, 0x04,                     # lea  eax,[esp+4]
        0x6A, 0x00,                                 # push 0        (arg 6)
        0x50,                                       # push eax      (arg 5)
        0x6A, 0x00,                                 # push 0        (arg 4)
    ]) + b'\xFF\x15' + struct.pack('<I', get_last_error) + bytes([
        0x50,                                       # push eax      (arg 3)
        0x6A, 0x00,                                 # push 0        (arg 2)
        0x68, 0x00, 0x13, 0x00, 0x00,               # push 0x1300   (arg 1)
    ]) + b'\xFF\x15' + struct.pack('<I', format_message) + bytes([
        0x59,                                       # pop  ecx
        0x6A, 0x00,                                 # push 0
    ]) + b'\xFF\x15' + struct.pack('<I', exit_process) + bytes([
        0xC3,                                       # ret
    ])


def build_text():
    body = bytearray(b'\0' * SECT_ALIGN)
    entry = build_entry()
    body[ENTRY_RVA - TEXT_RVA:ENTRY_RVA - TEXT_RVA + len(entry)] = entry
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
        os.path.dirname(os.path.abspath(__file__)), 'calleeprotostack_pe_i386.exe')
    with open(out, 'wb') as fh:
        fh.write(build())
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
