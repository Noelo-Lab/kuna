#!/usr/bin/env python3
"""Generate a minimal PE32 with code and its IAT in one executable/read section.

The fixture is derived from ``pe_iatincode_i386.py``. Unlike that writable
crypter-shaped image, this section is non-writable, which makes its IAT slots
both ``readonly`` and ``externref`` inside the decompiler. The on-disk thunk
RVAs are loader inputs, not the values held by the slots at run time.

    00401000  IAT       VirtualAlloc, GetModuleHandleA, GetDlgItemTextA
    00401010  entry     VirtualAlloc(0x40, GetModuleHandleA(0))
    00401028  dialog    calls GetDlgItemTextA twice with distinct arguments
    00401060  imports   descriptors, INT, names, DLL name

    python3 pe_iatincode_readonly_i386.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
OPT_SIZE = 0xE0

SECT_RVA = 0x1000
IAT_RVA = 0x1000
ENTRY_RVA = 0x1010
DIALOG_RVA = 0x1028
DESC_RVA = 0x1060
INT_RVA = 0x1088
NAMES_RVA = 0x10A0
DLL_RVA = 0x10E0

# CNT_CODE | CNT_INITIALIZED_DATA | MEM_EXECUTE | MEM_READ. The missing
# MEM_WRITE bit is the regression trigger.
CODE_CHARS = 0x60000060
IMPORTS = ["VirtualAlloc", "GetModuleHandleA", "GetDlgItemTextA"]


def build_entry():
    """Call GetModuleHandleA, then pass its result to VirtualAlloc."""
    return bytes([
        0x55,                                      # push ebp
        0x89, 0xE5,                                # mov ebp, esp
        0x6A, 0x00,                                # push 0
        0xFF, 0x15,                                # call dword ptr [...]
    ]) + struct.pack("<I", IMAGE_BASE + IAT_RVA + 4) + bytes([
        0x50,                                      # push eax
        0x6A, 0x40,                                # push 0x40
        0xFF, 0x15,                                # call dword ptr [...]
    ]) + struct.pack("<I", IMAGE_BASE + IAT_RVA) + bytes([
        0x5D,                                      # pop ebp
        0xC3,                                      # ret
    ])


def build_dialog():
    """Call GetDlgItemTextA twice, retaining every distinguishing argument."""
    out = bytearray([0x55, 0x89, 0xE5])                   # push ebp; mov ebp,esp
    for item, buffer in [(0x68, 0x404000), (0x69, 0x404020)]:
        out += bytes([0x6A, 0x20])                        # push 0x20
        out += bytes([0x68]) + struct.pack("<I", buffer)  # push buffer
        out += bytes([0x6A, item])                        # push item id
        out += bytes([0xFF, 0x75, 0x08])                  # push [ebp+8]
        out += bytes([0xFF, 0x15])                        # call dword ptr [...]
        out += struct.pack("<I", IMAGE_BASE + IAT_RVA + 8)
    out += bytes([0x5D, 0xC3])                            # pop ebp; ret
    return bytes(out)


def build_section():
    body = bytearray(b"\0" * FILE_ALIGN)

    def put(rva, data):
        off = rva - SECT_RVA
        body[off:off + len(data)] = data

    name_rvas = []
    at = NAMES_RVA
    for name in IMPORTS:
        name_rvas.append(at)
        blob = struct.pack("<H", 0) + name.encode() + b"\0"
        put(at, blob)
        at += len(blob) + (len(blob) & 1)

    thunks = b"".join(struct.pack("<I", rva) for rva in name_rvas) + b"\0" * 4
    put(INT_RVA, thunks)
    put(IAT_RVA, thunks)
    put(DLL_RVA, b"KERNEL32.dll\0")
    put(DESC_RVA, struct.pack("<IIIII", INT_RVA, 0, 0, DLL_RVA, IAT_RVA))
    put(DESC_RVA + 20, b"\0" * 20)
    put(ENTRY_RVA, build_entry())
    put(DIALOG_RVA, build_dialog())
    return bytes(body)


def section(name, vsize, rva, raw_size, raw_off):
    return name.ljust(8, "\0").encode() + struct.pack(
        "<IIIIIIHHI", vsize, rva, raw_size, raw_off, 0, 0, 0, 0, CODE_CHARS)


def build():
    sect = build_section()
    out = bytearray(b"MZ")
    out += b"\0" * (0x3C - len(out))
    out += struct.pack("<I", 0x40)
    out += b"\0" * (0x40 - len(out))
    out += b"PE\0\0"
    out += struct.pack("<HHIIIHH", 0x14C, 1, 0, 0, 0, OPT_SIZE, 0x010F)

    opt = struct.pack("<HBB", 0x10B, 14, 0)
    opt += struct.pack("<III", FILE_ALIGN, 0, 0)
    opt += struct.pack("<II", ENTRY_RVA, SECT_RVA)
    opt += struct.pack("<I", SECT_RVA)
    opt += struct.pack("<III", IMAGE_BASE, SECT_ALIGN, FILE_ALIGN)
    opt += struct.pack("<HHHHHHI", 4, 0, 0, 0, 4, 0, 0)
    opt += struct.pack("<III", SECT_RVA + SECT_ALIGN, FILE_ALIGN, 0)
    opt += struct.pack("<HH", 3, 0)
    opt += struct.pack("<IIII", 0x100000, 0x1000, 0x100000, 0x1000)
    opt += struct.pack("<II", 0, 16)
    directories = [(0, 0)] * 16
    directories[1] = (DESC_RVA, 40)
    directories[12] = (IAT_RVA, 16)
    for rva, size in directories:
        opt += struct.pack("<II", rva, size)
    assert len(opt) == OPT_SIZE
    out += opt
    out += section("", FILE_ALIGN, SECT_RVA, FILE_ALIGN, FILE_ALIGN)
    assert len(out) <= FILE_ALIGN
    out += b"\0" * (FILE_ALIGN - len(out))
    out += sect
    return bytes(out)


if __name__ == "__main__":
    path = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "pe_iatincode_readonly_i386.exe")
    with open(path, "wb") as output:
        output.write(build())
    print("wrote %s (%d bytes)" % (path, os.path.getsize(path)))
