#!/usr/bin/env python3
"""Generate a minimal PE32+ whose caller reaches `memcmp` through an import veneer.

The PE resolver names the IAT slot at 0x140002050 and the `FF 25` veneer at
0x140001080 as imported `memcmp` targets. It also reports an exported, defined
`memcmp` at 0x140001060. The caller at 0x140001000 keeps RDX and R8 live through
a length comparison before calling the veneer, matching the duplicate-name
prototype-routing shape in `comparison-calls-lose-live` while proving that
address-keyed library knowledge never leaks onto the same-named export.
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
CALLER_RVA = 0x1000
VENEER_RVA = 0x1080
DEFINED_MEMCMP_RVA = 0x1060
BUF1_RVA = 0x2100
BUF2_RVA = 0x2110


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_rdata():
    desc_rva = RDATA_RVA
    int_rva = RDATA_RVA + 0x40
    iat_rva = RDATA_RVA + 0x50
    name_rva = RDATA_RVA + 0x60
    dll_rva = RDATA_RVA + 0x70
    export_rva = RDATA_RVA + 0x80
    export_dll_rva = RDATA_RVA + 0xA8
    export_funcs_rva = RDATA_RVA + 0xB8
    export_names_rva = RDATA_RVA + 0xBC
    export_ordinals_rva = RDATA_RVA + 0xC0
    export_name_rva = RDATA_RVA + 0xC8
    data = bytearray(0x120)
    struct.pack_into("<IIIII", data, 0, int_rva, 0, 0, dll_rva, iat_rva)
    struct.pack_into("<QQ", data, int_rva - RDATA_RVA, name_rva, 0)
    struct.pack_into("<QQ", data, iat_rva - RDATA_RVA, name_rva, 0)
    data[name_rva - RDATA_RVA:name_rva - RDATA_RVA + 9] = b"\0\0memcmp\0"
    data[dll_rva - RDATA_RVA:dll_rva - RDATA_RVA + 11] = b"msvcrt.dll\0"
    struct.pack_into(
        "<IIHHIIIIIII",
        data,
        export_rva - RDATA_RVA,
        0,
        0,
        0,
        0,
        export_dll_rva,
        1,
        1,
        1,
        export_funcs_rva,
        export_names_rva,
        export_ordinals_rva,
    )
    data[export_dll_rva - RDATA_RVA:export_dll_rva - RDATA_RVA + 12] = b"fixture.dll\0"
    struct.pack_into("<I", data, export_funcs_rva - RDATA_RVA, DEFINED_MEMCMP_RVA)
    struct.pack_into("<I", data, export_names_rva - RDATA_RVA, export_name_rva)
    struct.pack_into("<H", data, export_ordinals_rva - RDATA_RVA, 0)
    data[export_name_rva - RDATA_RVA:export_name_rva - RDATA_RVA + 7] = b"memcmp\0"
    data[BUF1_RVA - RDATA_RVA:BUF1_RVA - RDATA_RVA + 4] = b"abc\0"
    data[BUF2_RVA - RDATA_RVA:BUF2_RVA - RDATA_RVA + 4] = b"abc\0"
    return bytes(data), desc_rva, iat_rva, export_rva


def build_text(iat_rva):
    text = bytearray(0x100)
    body = bytearray(b"\x48\x83\xec\x28")                 # sub rsp,0x28
    body += b"\x48\x8d\x15" + rel32(CALLER_RVA + len(body), 7, BUF2_RVA)
    body += b"\x4c\x8d\x42\x03"                         # lea r8,[rdx+3]
    body += b"\x4c\x2b\xc2"                             # sub r8,rdx
    body += b"\x48\x8d\x42\x03"                         # lea rax,[rdx+3]
    body += b"\x48\x2b\xc2"                             # sub rax,rdx
    body += b"\x49\x39\xc0"                             # cmp r8,rax
    body += b"\x75\x0c"                                  # jne done
    body += b"\x48\x8d\x0d" + rel32(CALLER_RVA + len(body), 7, BUF1_RVA)
    body += b"\xe8" + rel32(CALLER_RVA + len(body), 5, VENEER_RVA)
    body += b"\x48\x83\xc4\x28\xc3"                    # add rsp,0x28; ret
    text[:len(body)] = body
    text[DEFINED_MEMCMP_RVA - TEXT_RVA:DEFINED_MEMCMP_RVA - TEXT_RVA + 3] = b"\x31\xc0\xc3"
    text[VENEER_RVA - TEXT_RVA:VENEER_RVA - TEXT_RVA + 6] = (
        b"\xff\x25" + rel32(VENEER_RVA, 6, iat_rva)
    )
    return bytes(text)


def build():
    rdata, import_rva, iat_rva, export_rva = build_rdata()
    text = build_text(iat_rva)
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
        "<HBBIIIII", 0x20B, 14, 0, len(text), len(rdata), 0, CALLER_RVA, TEXT_RVA
    )
    optional += struct.pack("<Q", IMAGE_BASE)
    optional += struct.pack(
        "<IIHHHHHHIIIIHHQQQQII",
        SECTION_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
        RDATA_RVA + SECTION_ALIGN, headers_size, 0, 3, 0x8160,
        0x100000, 0x1000, 0x100000, 0x1000, 0, 16,
    )
    directories = [(0, 0)] * 16
    directories[0] = (export_rva, 0x50)
    directories[1] = (import_rva, 40)
    directories[12] = (iat_rva, 16)
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
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "libcsigs_pe_x86_64.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
