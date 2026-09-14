#!/usr/bin/env python3
"""Generate a minimal PE32+ with a REX-prefixed import tail jump and two real thunks.

`__scrt_initialize_type_info` in an MSVC `/O2` image is `lea rcx,[list]` followed by
`48 FF 25 <disp32>` (`jmp qword ptr [__imp_InitializeSListHead]`), and the linker's
import thunk table carries a bare `FF 25 <disp32>` through the same slot. The second
real thunk sits directly after a function ending in `jmp qword ptr [rax+0x48]`
(`48 FF 60 48`), so the byte before its `FF` is `48` too:

  0x140001000  entry    sub rsp,0x28; call wrapper; lea rcx,[list]; call thunk;
                        lea rcx,[list]; call vcall; xor ecx,ecx; call exit_thunk;
                        add rsp,0x28; ret
  0x140001030  wrapper  lea rcx,[list]; jmp qword ptr [slot1]   (48 ff 25)
  0x140001040  thunk    jmp qword ptr [slot1]                   (ff 25)
  0x140001050  vcall    mov rax,[rcx]; jmp qword ptr [rax+0x48] (48 ff 60 48)
  0x140001057  exit_thunk  jmp qword ptr [slot2]                (ff 25)
  0x140002050  slot1    KERNEL32.dll!InitializeSListHead
  0x140002058  slot2    KERNEL32.dll!ExitProcess

The import-thunk scan used to match the `FF 25` at 0x140001038, one byte into the
wrapper's tail jump, and name it as a second `InitializeSListHead` thunk. The thunk
at 0x140001057 is genuine and called, and must keep the `ExitProcess` name.
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY_RVA = 0x1000
WRAPPER_RVA = 0x1030
THUNK_RVA = 0x1040
VCALL_RVA = 0x1050
EXIT_THUNK_RVA = 0x1057
LIST_RVA = 0x2100
IMPORTS = [b"InitializeSListHead", b"ExitProcess"]


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_rdata():
    desc_rva = RDATA_RVA
    int_rva = RDATA_RVA + 0x30
    iat_rva = RDATA_RVA + 0x50
    names_rva = RDATA_RVA + 0x70
    data = bytearray(0x120)
    name_rvas = []
    cursor = names_rva
    for name in IMPORTS:
        entry = b"\0\0" + name + b"\0"
        data[cursor - RDATA_RVA:cursor - RDATA_RVA + len(entry)] = entry
        name_rvas.append(cursor)
        cursor += (len(entry) + 1) & ~1
    dll_rva = cursor
    dll = b"KERNEL32.dll\0"
    data[dll_rva - RDATA_RVA:dll_rva - RDATA_RVA + len(dll)] = dll
    struct.pack_into("<IIIII", data, 0, int_rva, 0, 0, dll_rva, iat_rva)
    for k, rva in enumerate(name_rvas):
        struct.pack_into("<Q", data, int_rva - RDATA_RVA + 8 * k, rva)
        struct.pack_into("<Q", data, iat_rva - RDATA_RVA + 8 * k, rva)
    assert cursor + len(dll) <= LIST_RVA
    return bytes(data), desc_rva, iat_rva


def place(text, rva, code):
    text[rva - TEXT_RVA:rva - TEXT_RVA + len(code)] = code


def build_text(iat_rva):
    text = bytearray(b"\xcc" * 0x100)
    slot1, slot2 = iat_rva, iat_rva + 8

    body = bytearray(b"\x48\x83\xec\x28")
    body += b"\xe8" + rel32(ENTRY_RVA + len(body), 5, WRAPPER_RVA)
    body += b"\x48\x8d\x0d" + rel32(ENTRY_RVA + len(body), 7, LIST_RVA)
    body += b"\xe8" + rel32(ENTRY_RVA + len(body), 5, THUNK_RVA)
    body += b"\x48\x8d\x0d" + rel32(ENTRY_RVA + len(body), 7, LIST_RVA)
    body += b"\xe8" + rel32(ENTRY_RVA + len(body), 5, VCALL_RVA)
    body += b"\x33\xc9"
    body += b"\xe8" + rel32(ENTRY_RVA + len(body), 5, EXIT_THUNK_RVA)
    body += b"\x48\x83\xc4\x28\xc3"
    assert ENTRY_RVA + len(body) <= WRAPPER_RVA
    place(text, ENTRY_RVA, body)

    wrapper = bytearray(b"\x48\x8d\x0d" + rel32(WRAPPER_RVA, 7, LIST_RVA))
    wrapper += b"\x48\xff\x25" + rel32(WRAPPER_RVA + len(wrapper), 7, slot1)
    place(text, WRAPPER_RVA, wrapper)

    place(text, THUNK_RVA, b"\xff\x25" + rel32(THUNK_RVA, 6, slot1))

    vcall = b"\x48\x8b\x01\x48\xff\x60\x48"
    assert VCALL_RVA + len(vcall) == EXIT_THUNK_RVA
    place(text, VCALL_RVA, vcall)
    place(text, EXIT_THUNK_RVA, b"\xff\x25" + rel32(EXIT_THUNK_RVA, 6, slot2))
    return bytes(text)


def build():
    rdata, import_rva, iat_rva = build_rdata()
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
    directories[1] = (import_rva, 40)
    directories[12] = (iat_rva, 8 * (len(IMPORTS) + 1))
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
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pe_rexthunk_x86_64.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
