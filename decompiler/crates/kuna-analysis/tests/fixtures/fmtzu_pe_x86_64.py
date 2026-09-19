#!/usr/bin/env python3
"""Generate a minimal PE32+ whose two exported functions print a size_t and a ptrdiff_t.

Win64 is LLP64: `long` is 4 bytes and `size_t`/`ptrdiff_t` are 8. Each function is

    void show_zu(size_t n)    { printf("%zu\\n", (n << 32) | 5); }
    void show_td(ptrdiff_t n) { printf("%td\\n", (n << 32) | 5); }

compiled the way MSVC and MinGW emit it (`mov rdx,rcx; shl rdx,32; or rdx,5`), with
the format in `.rdata` and `printf` reached through an `FF 25` import veneer over the
`msvcrt.dll` IAT slot. A 4-byte reading of `%zu`/`%td` keeps only the low half, which
is the constant 5, so the shift, the OR and the parameter all disappear.
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY_RVA = 0x1000
SHOW_ZU_RVA = 0x1020
SHOW_TD_RVA = 0x1060
VENEER_RVA = 0x10A0
FMT_ZU_RVA = 0x2100
FMT_TD_RVA = 0x2110


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_rdata():
    import_rva = RDATA_RVA
    int_rva = RDATA_RVA + 0x40
    iat_rva = RDATA_RVA + 0x50
    name_rva = RDATA_RVA + 0x60
    dll_rva = RDATA_RVA + 0x70
    export_rva = RDATA_RVA + 0x80
    export_dll_rva = RDATA_RVA + 0xA8
    export_funcs_rva = RDATA_RVA + 0xB8
    export_names_rva = RDATA_RVA + 0xC0
    export_ordinals_rva = RDATA_RVA + 0xC8
    name_td_rva = RDATA_RVA + 0xD0
    name_zu_rva = RDATA_RVA + 0xD8
    data = bytearray(0x120)

    def put(rva, raw):
        data[rva - RDATA_RVA:rva - RDATA_RVA + len(raw)] = raw

    struct.pack_into("<IIIII", data, 0, int_rva, 0, 0, dll_rva, iat_rva)
    struct.pack_into("<QQ", data, int_rva - RDATA_RVA, name_rva, 0)
    struct.pack_into("<QQ", data, iat_rva - RDATA_RVA, name_rva, 0)
    put(name_rva, b"\0\0printf\0")
    put(dll_rva, b"msvcrt.dll\0")
    struct.pack_into(
        "<IIHHIIIIIII",
        data,
        export_rva - RDATA_RVA,
        0, 0, 0, 0,
        export_dll_rva,
        1, 2, 2,
        export_funcs_rva, export_names_rva, export_ordinals_rva,
    )
    put(export_dll_rva, b"fmtzu.dll\0")
    struct.pack_into("<II", data, export_funcs_rva - RDATA_RVA, SHOW_TD_RVA, SHOW_ZU_RVA)
    struct.pack_into("<II", data, export_names_rva - RDATA_RVA, name_td_rva, name_zu_rva)
    struct.pack_into("<HH", data, export_ordinals_rva - RDATA_RVA, 0, 1)
    put(name_td_rva, b"show_td\0")
    put(name_zu_rva, b"show_zu\0")
    put(FMT_ZU_RVA, b"%zu\n\0")
    put(FMT_TD_RVA, b"%td\n\0")
    return bytes(data), import_rva, iat_rva, export_rva


def shifted_print(rva, fmt_rva):
    body = bytearray(b"\x48\x83\xec\x28")                 # sub rsp,0x28
    body += b"\x48\x89\xca"                               # mov rdx,rcx
    body += b"\x48\xc1\xe2\x20"                           # shl rdx,32
    body += b"\x48\x83\xca\x05"                           # or rdx,5
    body += b"\x48\x8d\x0d" + rel32(rva + len(body), 7, fmt_rva)
    body += b"\xe8" + rel32(rva + len(body), 5, VENEER_RVA)
    body += b"\x48\x83\xc4\x28\xc3"                      # add rsp,0x28; ret
    return body


def build_text(iat_rva):
    text = bytearray(0x100)

    def put(rva, raw):
        text[rva - TEXT_RVA:rva - TEXT_RVA + len(raw)] = raw

    entry = bytearray(b"\x48\x83\xec\x28")                # sub rsp,0x28
    entry += b"\xe8" + rel32(ENTRY_RVA + len(entry), 5, SHOW_ZU_RVA)
    entry += b"\xe8" + rel32(ENTRY_RVA + len(entry), 5, SHOW_TD_RVA)
    entry += b"\x31\xc0"                                  # xor eax,eax
    entry += b"\x48\x83\xc4\x28\xc3"                     # add rsp,0x28; ret
    put(ENTRY_RVA, entry)
    put(SHOW_ZU_RVA, shifted_print(SHOW_ZU_RVA, FMT_ZU_RVA))
    put(SHOW_TD_RVA, shifted_print(SHOW_TD_RVA, FMT_TD_RVA))
    put(VENEER_RVA, b"\xff\x25" + rel32(VENEER_RVA, 6, iat_rva))
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
    directories[0] = (export_rva, 0x60)
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
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "fmtzu_pe_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
