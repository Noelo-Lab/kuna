#!/usr/bin/env python3
"""Generate two minimal PE32+ images whose synthesized structures meet `pebnames`.

`pebnames` creates the `PEB` and `TEB` types the first time a function reads
`gs:[0x30]` or `gs:[0x60]`, so under `--jobs N` only the worker that decompiled
such a function holds them.

* `structsynth_peb_pe_x86_64.exe`: the function at 0x140001040 stores the PEB
  pointer into its structure argument, so a synthesized structure has a
  `PEB *` field that another worker process cannot rebuild. Five more functions
  each measure a structure of their own.
* `structsynth_teb_pe_x86_64.exe`: the same five structure functions, and one
  at 0x140001180 that reads `BeingDebugged` through the TEB and synthesizes
  nothing, so the `TEB`/`PEB` definitions come from a function that never asks
  the structure ledger.

Every function is a leaf: `mov dword [rcx], k; mov dword [rcx+4], k;
mov [rcx+8+8k], rdx; mov rax, [rcx+8]; ret`, called in turn from the entry.
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY_RVA = 0x1000
VENEER_RVA = 0x10A0


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
    put(export_dll_rva, b"structs.dll\0")
    struct.pack_into("<II", data, export_funcs_rva - RDATA_RVA, 0x1080, 0x1040)
    struct.pack_into("<II", data, export_names_rva - RDATA_RVA, name_td_rva, name_zu_rva)
    struct.pack_into("<HH", data, export_ordinals_rva - RDATA_RVA, 0, 1)
    put(name_td_rva, b"store_b\0")
    put(name_zu_rva, b"store_a\0")
    return bytes(data), import_rva, iat_rva, export_rva


F_RVAS = [0x1040, 0x1080, 0x10c0, 0x1100, 0x1140, 0x1180]
def fn_peb():
    b = bytearray()
    b += b"\x65\x48\x8b\x04\x25\x60\x00\x00\x00"   # mov rax, gs:[0x60]
    b += b"\x48\x89\x41\x08"                       # mov [rcx+8], rax
    b += b"\xc7\x01\x01\x00\x00\x00"               # mov dword [rcx], 1
    b += b"\xc7\x41\x10\x02\x00\x00\x00"           # mov dword [rcx+0x10], 2
    b += b"\x48\x89\x51\x18"                       # mov [rcx+0x18], rdx
    b += b"\xc3"
    return b
def fn_plain(k):
    b = bytearray()
    b += b"\xc7\x01" + bytes([k,0,0,0])            # mov dword [rcx], k
    b += b"\xc7\x41\x04" + bytes([k,0,0,0])        # mov dword [rcx+4], k
    b += b"\x48\x89\x51" + bytes([0x08 + 8*k])     # mov [rcx+8+8k], rdx
    b += b"\x48\x8b\x41\x08"                       # mov rax,[rcx+8]
    b += b"\xc3"
    return b
def build_text(iat_rva, variant):
    text = bytearray(0x200)
    def put(rva, raw):
        text[rva - TEXT_RVA:rva - TEXT_RVA + len(raw)] = raw
    entry = bytearray(b"\x48\x83\xec\x28")
    for r in F_RVAS:
        entry += b"\xe8" + rel32(ENTRY_RVA + len(entry), 5, r)
    entry += b"\x31\xc0\x48\x83\xc4\x28\xc3"
    put(ENTRY_RVA, entry)
    if variant == "peb":
        put(F_RVAS[0], fn_peb())
        for i, r in enumerate(F_RVAS[1:]):
            put(r, fn_plain(i + 1))
    else:
        put(F_RVAS[0], fn_plain(7))
        for i, r in enumerate(F_RVAS[1:-1]):
            put(r, fn_plain(i + 1))
        # mov rax, gs:[0x60]; movzx eax, byte [rax+2]; ret
        put(F_RVAS[-1], b"\x65\x48\x8b\x04\x25\x60\x00\x00\x00\x0f\xb6\x40\x02\xc3")
    put(0x11c0, b"\xff\x25" + rel32(0x11c0, 6, iat_rva))
    return bytes(text)


def build(variant):
    rdata, import_rva, iat_rva, export_rva = build_rdata()
    text = build_text(iat_rva, variant)
    headers_size = FILE_ALIGN
    text_size = 0x200
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
    here = os.path.dirname(os.path.abspath(__file__))
    for variant in ("peb", "teb"):
        out = os.path.join(here, f"structsynth_{variant}_pe_x86_64.exe")
        with open(out, "wb") as f:
            f.write(build(variant))
        print(f"wrote {out} ({os.path.getsize(out)} bytes)")
