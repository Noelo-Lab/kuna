#!/usr/bin/env python3
"""Generate a minimal PE32+ that starts a thread through its import table.

`entry` (0x140001000) passes the exported `worker_proc` (0x140001080) to
`CreateThread` and hands the result to `CloseHandle`, both called through
their KERNEL32.dll IAT slots (`FF 15`). It is the shape of every Win32 call in
an MSVC C++ program: the callee is reached only through an import slot, and a
local function is passed by address.

    python3 importcall_pe_x86_64.py
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY_RVA = 0x1000
WORKER_RVA = 0x1080

DESC_RVA = 0x2000
INT_RVA = 0x2040
IAT_RVA = 0x2060
CREATETHREAD_NAME_RVA = 0x2080
CLOSEHANDLE_NAME_RVA = 0x2090
DLL_RVA = 0x20A0
EXPORT_RVA = 0x20C0
EXPORT_DLL_RVA = 0x20F0
EXPORT_FUNCS_RVA = 0x2100
EXPORT_NAMES_RVA = 0x2104
EXPORT_ORDINALS_RVA = 0x2108
EXPORT_NAME_RVA = 0x2110


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_rdata():
    data = bytearray(0x200)
    struct.pack_into("<IIIII", data, DESC_RVA - RDATA_RVA, INT_RVA, 0, 0, DLL_RVA, IAT_RVA)
    for table in (INT_RVA, IAT_RVA):
        struct.pack_into(
            "<QQQ", data, table - RDATA_RVA, CREATETHREAD_NAME_RVA, CLOSEHANDLE_NAME_RVA, 0
        )
    data[CREATETHREAD_NAME_RVA - RDATA_RVA:CREATETHREAD_NAME_RVA - RDATA_RVA + 15] = (
        b"\0\0CreateThread\0"
    )
    data[CLOSEHANDLE_NAME_RVA - RDATA_RVA:CLOSEHANDLE_NAME_RVA - RDATA_RVA + 14] = (
        b"\0\0CloseHandle\0"
    )
    data[DLL_RVA - RDATA_RVA:DLL_RVA - RDATA_RVA + 13] = b"KERNEL32.dll\0"
    struct.pack_into(
        "<IIHHIIIIIII",
        data,
        EXPORT_RVA - RDATA_RVA,
        0, 0, 0, 0,
        EXPORT_DLL_RVA, 1, 1, 1,
        EXPORT_FUNCS_RVA, EXPORT_NAMES_RVA, EXPORT_ORDINALS_RVA,
    )
    data[EXPORT_DLL_RVA - RDATA_RVA:EXPORT_DLL_RVA - RDATA_RVA + 12] = b"fixture.exe\0"
    struct.pack_into("<I", data, EXPORT_FUNCS_RVA - RDATA_RVA, WORKER_RVA)
    struct.pack_into("<I", data, EXPORT_NAMES_RVA - RDATA_RVA, EXPORT_NAME_RVA)
    struct.pack_into("<H", data, EXPORT_ORDINALS_RVA - RDATA_RVA, 0)
    data[EXPORT_NAME_RVA - RDATA_RVA:EXPORT_NAME_RVA - RDATA_RVA + 12] = b"worker_proc\0"
    return bytes(data)


def build_text():
    text = bytearray(0x100)
    body = bytearray(b"\x48\x83\xec\x38")                 # sub rsp,0x38
    body += b"\x31\xc9"                                   # xor ecx,ecx
    body += b"\x31\xd2"                                   # xor edx,edx
    body += b"\x4c\x8d\x05" + rel32(ENTRY_RVA + len(body), 7, WORKER_RVA)
    body += b"\x45\x31\xc9"                               # xor r9d,r9d
    body += b"\x48\xc7\x44\x24\x20\x00\x00\x00\x00"       # mov qword [rsp+0x20],0
    body += b"\x48\xc7\x44\x24\x28\x00\x00\x00\x00"       # mov qword [rsp+0x28],0
    body += b"\xff\x15" + rel32(ENTRY_RVA + len(body), 6, IAT_RVA)
    body += b"\x48\x85\xc0"                               # test rax,rax
    body += b"\x74\x09"                                   # je done
    body += b"\x48\x89\xc1"                               # mov rcx,rax
    body += b"\xff\x15" + rel32(ENTRY_RVA + len(body), 6, IAT_RVA + 8)
    body += b"\x48\x83\xc4\x38\xc3"                       # done: add rsp,0x38; ret
    text[:len(body)] = body
    text[WORKER_RVA - TEXT_RVA:WORKER_RVA - TEXT_RVA + 3] = b"\x31\xc0\xc3"
    return bytes(text)


def build():
    rdata = build_rdata()
    text = build_text()
    headers_size = FILE_ALIGN
    text_size = FILE_ALIGN
    rdata_size = FILE_ALIGN

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)
    image = bytearray(dos)
    image += b"PE\0\0"
    image += struct.pack("<HHIIIHH", 0x8664, 2, 0, 0, 0, 240, 0x0022)
    optional = bytearray()
    optional += struct.pack(
        "<HBBIIIII", 0x20B, 14, 0, len(text), len(rdata), 0, ENTRY_RVA, TEXT_RVA
    )
    optional += struct.pack("<Q", IMAGE_BASE)
    optional += struct.pack(
        "<IIHHHHHHIIIIHHQQQQII",
        SECTION_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
        RDATA_RVA + SECTION_ALIGN, headers_size, 0, 3, 0x8160,
        0x100000, 0x1000, 0x100000, 0x1000, 0, 16,
    )
    directories = [(0, 0)] * 16
    directories[0] = (EXPORT_RVA, EXPORT_NAME_RVA + 12 - EXPORT_RVA)
    directories[1] = (DESC_RVA, 40)
    directories[12] = (IAT_RVA, 24)
    for rva, size in directories:
        optional += struct.pack("<II", rva, size)
    assert len(optional) == 240
    image += optional

    def section(name, virtual_size, rva, raw_size, raw_offset, flags):
        return name.encode().ljust(8, b"\0") + struct.pack(
            "<IIIIIIHHI", virtual_size, rva, raw_size, raw_offset, 0, 0, 0, 0, flags
        )

    image += section(".text", len(text), TEXT_RVA, text_size, headers_size, 0x60000020)
    image += section(
        ".rdata", len(rdata), RDATA_RVA, rdata_size,
        headers_size + text_size, 0x40000040,
    )
    image += bytes(headers_size - len(image))
    image += text.ljust(text_size, b"\0")
    image += rdata.ljust(rdata_size, b"\0")
    return bytes(image)


if __name__ == "__main__":
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "importcall_pe_x86_64.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
