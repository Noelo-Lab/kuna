#!/usr/bin/env python3
"""Generate a minimal PE32+ that passes a value through its caller's home slot.

`entry` (0x140001000) stores the address of the exported `worker_proc`
(0x140001080) into the home slot its caller allocated at entry `rsp+0x18`,
then passes that slot's address to the exported `consume` (0x140001060).
MSVC emits this for every Qt `connect(sender, &Class::signal, ...)`: the
member function pointer is spilled to a home slot and passed by reference.

    python3 homeslot_pe_x86_64.py
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY_RVA = 0x1000
CONSUME_RVA = 0x1060
WORKER_RVA = 0x1080

EXPORT_RVA = 0x2000
EXPORT_DLL_RVA = 0x2030
EXPORT_FUNCS_RVA = 0x2040
EXPORT_NAMES_RVA = 0x2048
EXPORT_ORDINALS_RVA = 0x2050
CONSUME_NAME_RVA = 0x2058
WORKER_NAME_RVA = 0x2060


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_rdata():
    data = bytearray(0x100)
    struct.pack_into(
        "<IIHHIIIIIII",
        data,
        EXPORT_RVA - RDATA_RVA,
        0, 0, 0, 0,
        EXPORT_DLL_RVA, 1, 2, 2,
        EXPORT_FUNCS_RVA, EXPORT_NAMES_RVA, EXPORT_ORDINALS_RVA,
    )
    data[EXPORT_DLL_RVA - RDATA_RVA:EXPORT_DLL_RVA - RDATA_RVA + 12] = b"fixture.exe\0"
    struct.pack_into("<II", data, EXPORT_FUNCS_RVA - RDATA_RVA, CONSUME_RVA, WORKER_RVA)
    struct.pack_into("<II", data, EXPORT_NAMES_RVA - RDATA_RVA, CONSUME_NAME_RVA, WORKER_NAME_RVA)
    struct.pack_into("<HH", data, EXPORT_ORDINALS_RVA - RDATA_RVA, 0, 1)
    data[CONSUME_NAME_RVA - RDATA_RVA:CONSUME_NAME_RVA - RDATA_RVA + 8] = b"consume\0"
    data[WORKER_NAME_RVA - RDATA_RVA:WORKER_NAME_RVA - RDATA_RVA + 12] = b"worker_proc\0"
    return bytes(data)


def build_text():
    text = bytearray(0x100)
    body = bytearray(b"\x48\x83\xec\x28")                 # sub rsp,0x28
    body += b"\x48\x8d\x05" + rel32(ENTRY_RVA + len(body), 7, WORKER_RVA)
    body += b"\x48\x89\x44\x24\x40"                       # mov [rsp+0x40],rax
    body += b"\x48\x8d\x4c\x24\x40"                       # lea rcx,[rsp+0x40]
    body += b"\xe8" + rel32(ENTRY_RVA + len(body), 5, CONSUME_RVA)
    body += b"\x48\x83\xc4\x28\xc3"                       # add rsp,0x28; ret
    text[:len(body)] = body
    text[CONSUME_RVA - TEXT_RVA:CONSUME_RVA - TEXT_RVA + 4] = b"\x48\x8b\x01\xc3"  # mov rax,[rcx]; ret
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
    directories[0] = (EXPORT_RVA, WORKER_NAME_RVA + 12 - EXPORT_RVA)
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
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "homeslot_pe_x86_64.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
