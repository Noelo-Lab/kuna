#!/usr/bin/env python3
"""Generate `widestrings_x86_64.exe` — a minimal PE32+ whose entry passes a wide
(UTF-16LE) string to `LoadLibraryW` and an ASCII one to `GetProcAddress`.

This is the shape crackmes.one/65acadf3eef082e477ff5ede presents, reduced to the
smallest image that reproduces it: the wide `L"ntdll.dll"` is `6e 00 74 00 …`, so
a 1-byte string scan reads it as the one-character string `"n"` and the emitted C
said `LoadLibraryW("n")`. The ASCII literal is here as the control — the 1-byte
width must still own it. No toolchain on this host links a Windows image, so the
PE is assembled here byte by byte. Regenerate with:

    python3 widestrings_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  entry   sub rsp,0x28                    <- AddressOfEntryPoint
                         lea rcx,[0x140002100]           <- L"ntdll.dll"
                         call [rip+..] -> IAT[0] LoadLibraryW
                         mov rcx,rax
                         lea rdx,[0x140002120]           <- "NtQueryInformationProcess"
                         call [rip+..] -> IAT[1] GetProcAddress
                         add rsp,0x28 ; ret
  .rdata RVA 0x2000   import descriptor + INT + IAT + names
         RVA 0x2100   L"ntdll.dll"  (9 units + the 0x0000 terminator)
         RVA 0x2120   "NtQueryInformationProcess" (25 chars + the NUL)
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200

TEXT_RVA = 0x1000
RDATA_RVA = 0x2000

ENTRY = 0x1000
WIDE_RVA = 0x2100                      # L"ntdll.dll"
ASCII_RVA = 0x2120                     # "NtQueryInformationProcess"

IMPORTS = ["LoadLibraryW", "GetProcAddress"]
DLL = b"KERNEL32.dll\0"
WIDE_TEXT = "ntdll.dll"
ASCII_TEXT = b"NtQueryInformationProcess\0"


def build_rdata():
    """Import descriptor + INT + IAT + name blobs, then the two literals."""
    n = len(IMPORTS)
    desc_off = 0                       # one descriptor + a null terminator
    int_off = desc_off + 20 * 2
    iat_off = int_off + 8 * (n + 1)
    names_off = iat_off + 8 * (n + 1)
    blob = bytearray()
    name_rvas = []
    tail = bytearray()
    for nm in IMPORTS:
        name_rvas.append(RDATA_RVA + names_off + len(tail))
        tail += struct.pack("<H", 0) + nm.encode() + b"\0"
        if len(tail) % 2:
            tail += b"\0"
    dll_rva = RDATA_RVA + names_off + len(tail)
    tail += DLL

    blob += struct.pack("<IIIII", RDATA_RVA + int_off, 0, 0, dll_rva, RDATA_RVA + iat_off)
    blob += bytes(20)                                        # null descriptor
    for r in name_rvas:
        blob += struct.pack("<Q", r)
    blob += struct.pack("<Q", 0)                             # INT terminator
    for r in name_rvas:
        blob += struct.pack("<Q", r)                         # IAT mirrors the INT
    blob += struct.pack("<Q", 0)
    blob += tail

    # The literals, at their fixed RVAs (the import blob must not reach them).
    assert len(blob) <= WIDE_RVA - RDATA_RVA, len(blob)
    blob = blob.ljust(WIDE_RVA - RDATA_RVA, b"\0")
    for ch in WIDE_TEXT:
        blob += struct.pack("<H", ord(ch))
    blob += struct.pack("<H", 0)
    blob = blob.ljust(ASCII_RVA - RDATA_RVA, b"\0")
    blob += ASCII_TEXT
    blob += b"\0" * 4

    iat_rvas = [RDATA_RVA + iat_off + 8 * i for i in range(n)]
    return bytes(blob), iat_rvas, RDATA_RVA + desc_off, 40, RDATA_RVA + iat_off, 8 * (n + 1)


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_text(iat_rvas):
    body = bytearray()

    def here():
        return ENTRY + len(body)

    body += bytes([0x48, 0x83, 0xEC, 0x28])                       # sub rsp,0x28
    body += b"\x48\x8D\x0D" + rel32(here(), 7, WIDE_RVA)          # lea rcx,[wide]
    body += b"\xFF\x15" + rel32(here(), 6, iat_rvas[0])           # call [LoadLibraryW]
    body += bytes([0x48, 0x8B, 0xC8])                             # mov rcx,rax
    body += b"\x48\x8D\x15" + rel32(here(), 7, ASCII_RVA)         # lea rdx,[ascii]
    body += b"\xFF\x15" + rel32(here(), 6, iat_rvas[1])           # call [GetProcAddress]
    body += bytes([0x48, 0x83, 0xC4, 0x28, 0xC3])                 # add rsp,0x28 ; ret
    return bytes(body)


def build():
    rdata, iat_rvas, imp_rva, imp_size, iat_rva, iat_size = build_rdata()
    text = build_text(iat_rvas)

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)

    nsec = 2
    opt_size = 240
    hdr_size = 0x40 + 4 + 20 + opt_size + 40 * nsec
    headers_sz = (hdr_size + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    text_off = headers_sz
    text_sz = (len(text) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    rdata_off = text_off + text_sz
    rdata_sz = (len(rdata) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    image_sz = RDATA_RVA + (len(rdata) + SECT_ALIGN - 1) // SECT_ALIGN * SECT_ALIGN

    b = bytearray(dos)
    b += b"PE\0\0"
    b += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)
    opt = bytearray()
    opt += struct.pack("<HBBIIIII", 0x20B, 14, 0, len(text), len(rdata), 0, ENTRY, TEXT_RVA)
    opt += struct.pack("<Q", IMAGE_BASE)
    opt += struct.pack("<IIHHHHHHIIIIHHQQQQII",
                       SECT_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
                       image_sz, headers_sz, 0, 3, 0x8160,
                       0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = [(0, 0)] * 16
    dirs[1] = (imp_rva, imp_size)
    dirs[12] = (iat_rva, iat_size)
    for rva, sz in dirs:
        opt += struct.pack("<II", rva, sz)
    assert len(opt) == opt_size, len(opt)
    b += opt

    def sect(name, vsz, rva, rsz, roff, chars):
        return (name.encode().ljust(8, b"\0")
                + struct.pack("<IIIIIIHHI", vsz, rva, rsz, roff, 0, 0, 0, 0, chars))

    b += sect(".text", len(text), TEXT_RVA, text_sz, text_off, 0x60000020)
    b += sect(".rdata", len(rdata), RDATA_RVA, rdata_sz, rdata_off, 0x40000040)
    b += bytes(headers_sz - len(b))
    b += text.ljust(text_sz, b"\0")
    b += rdata.ljust(rdata_sz, b"\0")
    return bytes(b)


if __name__ == "__main__":
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "widestrings_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
