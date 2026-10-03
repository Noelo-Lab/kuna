#!/usr/bin/env python3
"""Generate a minimal PE32+ whose entry calls the two MSVC helpers x86-64-win.cspec
replaces with call-fixups.

`entry` (0x140001000) probes its frame with `__chkstk` (0x140001080, fixup
`alloca_probe`), then makes a Control Flow Guard virtual call: it loads the
target from the object's vtable into RAX and calls through the read-only
`__guard_dispatch_icall_fptr` slot (0x140002000), which holds
`_guard_dispatch_icall_nop` (0x1400010a0, fixup `guard_dispatch_icall`, a bare
`jmp rax`).  Stock Ghidra prints neither helper: the probe disappears and the
guarded call is the virtual call it dispatches.

    python3 callfixup_pe_x86_64.py
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY_RVA = 0x1000
CHKSTK_RVA = 0x1080
GUARD_NOP_RVA = 0x10A0
FPTR_RVA = 0x2000

EXPORT_RVA = 0x2010
EXPORT_DLL_RVA = 0x2040
EXPORT_FUNCS_RVA = 0x2050
EXPORT_NAMES_RVA = 0x2058
EXPORT_ORDINALS_RVA = 0x2060
CHKSTK_NAME_RVA = 0x2068
GUARD_NOP_NAME_RVA = 0x2078


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_rdata():
    data = bytearray(0x100)
    struct.pack_into("<Q", data, FPTR_RVA - RDATA_RVA, IMAGE_BASE + GUARD_NOP_RVA)
    struct.pack_into(
        "<IIHHIIIIIII",
        data,
        EXPORT_RVA - RDATA_RVA,
        0, 0, 0, 0,
        EXPORT_DLL_RVA, 1, 2, 2,
        EXPORT_FUNCS_RVA, EXPORT_NAMES_RVA, EXPORT_ORDINALS_RVA,
    )
    data[EXPORT_DLL_RVA - RDATA_RVA:EXPORT_DLL_RVA - RDATA_RVA + 12] = b"fixture.exe\0"
    struct.pack_into("<II", data, EXPORT_FUNCS_RVA - RDATA_RVA, CHKSTK_RVA, GUARD_NOP_RVA)
    struct.pack_into("<II", data, EXPORT_NAMES_RVA - RDATA_RVA, CHKSTK_NAME_RVA, GUARD_NOP_NAME_RVA)
    struct.pack_into("<HH", data, EXPORT_ORDINALS_RVA - RDATA_RVA, 0, 1)
    data[CHKSTK_NAME_RVA - RDATA_RVA:CHKSTK_NAME_RVA - RDATA_RVA + 9] = b"__chkstk\0"
    name = b"_guard_dispatch_icall_nop\0"
    data[GUARD_NOP_NAME_RVA - RDATA_RVA:GUARD_NOP_NAME_RVA - RDATA_RVA + len(name)] = name
    return bytes(data)


def build_text():
    text = bytearray(0x100)
    body = bytearray(b"\x53")                             # push rbx
    body += b"\xb8\x20\x10\x00\x00"                       # mov eax,0x1020
    body += b"\xe8" + rel32(ENTRY_RVA + len(body), 5, CHKSTK_RVA)
    body += b"\x48\x29\xc4"                               # sub rsp,rax
    body += b"\x48\x89\xcb"                               # mov rbx,rcx
    body += b"\x48\x8b\x01"                               # mov rax,[rcx]
    body += b"\x48\x8b\x40\x68"                           # mov rax,[rax+0x68]
    body += b"\x48\x89\xd9"                               # mov rcx,rbx
    body += b"\xff\x15" + rel32(ENTRY_RVA + len(body), 6, FPTR_RVA)
    body += b"\x48\x81\xc4\x20\x10\x00\x00"               # add rsp,0x1020
    body += b"\x5b\xc3"                                   # pop rbx; ret
    text[:len(body)] = body
    text[CHKSTK_RVA - TEXT_RVA:CHKSTK_RVA - TEXT_RVA + 1] = b"\xc3"
    text[GUARD_NOP_RVA - TEXT_RVA:GUARD_NOP_RVA - TEXT_RVA + 2] = b"\xff\xe0"  # jmp rax
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
    directories[0] = (EXPORT_RVA, GUARD_NOP_NAME_RVA + 26 - EXPORT_RVA)
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
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "callfixup_pe_x86_64.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
