#!/usr/bin/env python3
"""Generate a minimal PE32+ whose contiguous import thunk table sits more than 1 GiB below its IAT.

link.exe lays import thunks out back to back, so the byte before each thunk after the
first is the high byte of the previous thunk's displacement. With `.idata` at RVA
0x40002000 every displacement is 0x40000ffa..0x40001006, so that byte is 0x40, a REX
value. None of the thunks is called; the entry reaches GetTickCount through its slot:

  0x140001000  entry  sub rsp,0x28; call qword ptr [slot0]; add rsp,0x28; ret
  0x140001020  thunk  jmp qword ptr [slot0]   GetTickCount
  0x140001026  thunk  jmp qword ptr [slot1]   Sleep
  0x14000102c  thunk  jmp qword ptr [slot2]   ExitProcess
  0x180002060  slot0..slot2 (KERNEL32.dll)
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECTION_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x1000
IDATA_RVA = 0x40002000
TABLE_RVA = 0x1020
IMPORTS = [b"GetTickCount", b"Sleep", b"ExitProcess"]


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_idata():
    data = bytearray(0x200)
    int_rva = IDATA_RVA + 0x30
    iat_rva = IDATA_RVA + 0x60
    cursor = IDATA_RVA + 0x90
    name_rvas = []
    for name in IMPORTS:
        entry = b"\0\0" + name + b"\0"
        data[cursor - IDATA_RVA:cursor - IDATA_RVA + len(entry)] = entry
        name_rvas.append(cursor)
        cursor += (len(entry) + 1) & ~1
    dll = b"KERNEL32.dll\0"
    data[cursor - IDATA_RVA:cursor - IDATA_RVA + len(dll)] = dll
    struct.pack_into("<IIIII", data, 0, int_rva, 0, 0, cursor, iat_rva)
    for k, rva in enumerate(name_rvas):
        struct.pack_into("<Q", data, int_rva - IDATA_RVA + 8 * k, rva)
        struct.pack_into("<Q", data, iat_rva - IDATA_RVA + 8 * k, rva)
    return bytes(data), iat_rva


def build_text(iat_rva):
    text = bytearray(b"\xcc" * 0x100)
    body = bytearray(b"\x48\x83\xec\x28")
    body += b"\xff\x15" + rel32(TEXT_RVA + len(body), 6, iat_rva)
    body += b"\x48\x83\xc4\x28\xc3"
    text[0:len(body)] = body
    for k in range(len(IMPORTS)):
        at = TABLE_RVA + 6 * k
        text[at - TEXT_RVA:at - TEXT_RVA + 6] = b"\xff\x25" + rel32(at, 6, iat_rva + 8 * k)
        assert text[at - TEXT_RVA + 5] == 0x40
    return bytes(text)


def build():
    idata, iat_rva = build_idata()
    text = build_text(iat_rva)
    headers_size = FILE_ALIGN

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)
    image = bytearray(dos)
    image += b"PE\0\0"
    image += struct.pack("<HHIIIHH", 0x8664, 2, 0, 0, 0, 240, 0x0022)
    optional = bytearray()
    optional += struct.pack(
        "<HBBIIIII", 0x20B, 14, 0, len(text), len(idata), 0, TEXT_RVA, TEXT_RVA
    )
    optional += struct.pack("<Q", IMAGE_BASE)
    optional += struct.pack(
        "<IIHHHHHHIIIIHHQQQQII",
        SECTION_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
        IDATA_RVA + SECTION_ALIGN, headers_size, 0, 3, 0x8160,
        0x100000, 0x1000, 0x100000, 0x1000, 0, 16,
    )
    directories = [(0, 0)] * 16
    directories[1] = (IDATA_RVA, 40)
    directories[12] = (iat_rva, 8 * (len(IMPORTS) + 1))
    for rva, size in directories:
        optional += struct.pack("<II", rva, size)
    assert len(optional) == 240
    image += optional

    def section(name, virtual_size, rva, raw_size, raw_offset, flags):
        return name.encode().ljust(8, b"\0") + struct.pack(
            "<IIIIIIHHI", virtual_size, rva, raw_size, raw_offset, 0, 0, 0, 0, flags
        )

    image += section(".text", len(text), TEXT_RVA, FILE_ALIGN, headers_size, 0x60000020)
    image += section(".idata", len(idata), IDATA_RVA, FILE_ALIGN, headers_size + FILE_ALIGN, 0xC0000040)
    image += bytes(headers_size - len(image))
    image += text.ljust(FILE_ALIGN, b"\0")
    image += idata.ljust(FILE_ALIGN, b"\0")
    return bytes(image)


if __name__ == "__main__":
    output = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pe_rexthunk_far_x86_64.exe")
    with open(output, "wb") as fixture:
        fixture.write(build())
    print(f"wrote {output} ({os.path.getsize(output)} bytes)")
