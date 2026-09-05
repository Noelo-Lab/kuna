#!/usr/bin/env python3
"""Generate `crtmain_x86_64.exe` — a minimal PE32+ carrying the MSVC UCRT
`__scrt_common_main_seh` call to `main`, with a `main` that ignores its arguments.

This is the shape crackmes.one/65acadf3eef082e477ff5ede presents, reduced to the
smallest image that reproduces it: the startup fetches argc/argv/envp through the
three named UCRT accessors and calls `main`, while `main` never reads rcx/rdx/r8,
so body-driven parameter recovery finds nothing and declares it `void(void)`. No
toolchain on this host links a Windows image, so the PE is assembled here byte by
byte. Regenerate with:

    python3 crtmain_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  main       mov eax,0x2a ; ret          <- ignores its arguments
    0x140001010  startup    sub rsp,0x28                <- AddressOfEntryPoint
                            call venv_env   ; mov rdi,rax
                            call venv_argv  ; mov rbx,[rax]
                            call venv_argc  ; mov r8,rdi ; mov rdx,rbx ; mov ecx,[rax]
                            call main
                            add rsp,0x28 ; ret
    0x140001040  venv_env   jmp [rip+..]  -> IAT[0] _get_initial_narrow_environment
    0x140001046  venv_argv  jmp [rip+..]  -> IAT[1] __p___argv
    0x14000104c  venv_argc  jmp [rip+..]  -> IAT[2] __p___argc
  .rdata RVA 0x2000   import descriptor + INT + IAT + names
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200

TEXT_RVA = 0x1000
RDATA_RVA = 0x2000

MAIN = 0x1000
STARTUP = 0x1010
VEN_ENV, VEN_ARGV, VEN_ARGC = 0x1040, 0x1046, 0x104C

IMPORTS = ["_get_initial_narrow_environment", "__p___argv", "__p___argc"]
DLL = b"api-ms-win-crt-runtime-l1-1-0.dll\0"


def build_rdata():
    """Import descriptor + INT + IAT + name blobs. Returns (bytes, iat_rva[])."""
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
    iat_rvas = [RDATA_RVA + iat_off + 8 * i for i in range(n)]
    return bytes(blob), iat_rvas, RDATA_RVA + desc_off, len(blob), RDATA_RVA + iat_off, 8 * (n + 1)


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_text(iat_rvas):
    t = bytearray(0x60)
    # main: mov eax,0x2a ; ret
    t[MAIN - TEXT_RVA:MAIN - TEXT_RVA + 6] = bytes([0xB8, 0x2A, 0, 0, 0, 0xC3])
    p = STARTUP - TEXT_RVA
    body = bytearray()
    body += bytes([0x48, 0x83, 0xEC, 0x28])                       # sub rsp,0x28
    here = STARTUP + len(body)
    body += b"\xE8" + rel32(here, 5, VEN_ENV)
    body += bytes([0x48, 0x8B, 0xF8])                             # mov rdi,rax
    here = STARTUP + len(body)
    body += b"\xE8" + rel32(here, 5, VEN_ARGV)
    body += bytes([0x48, 0x8B, 0x18])                             # mov rbx,[rax]
    here = STARTUP + len(body)
    body += b"\xE8" + rel32(here, 5, VEN_ARGC)
    body += bytes([0x4C, 0x8B, 0xC7])                             # mov r8,rdi
    body += bytes([0x48, 0x8B, 0xD3])                             # mov rdx,rbx
    body += bytes([0x8B, 0x08])                                   # mov ecx,[rax]
    here = STARTUP + len(body)
    body += b"\xE8" + rel32(here, 5, MAIN)
    body += bytes([0x48, 0x83, 0xC4, 0x28, 0xC3])                 # add rsp,0x28 ; ret
    assert STARTUP + len(body) <= VEN_ENV, hex(STARTUP + len(body))
    t[p:p + len(body)] = body
    for ven, iat in zip((VEN_ENV, VEN_ARGV, VEN_ARGC), iat_rvas):
        stub = b"\xFF\x25" + rel32(ven, 6, iat)
        t[ven - TEXT_RVA:ven - TEXT_RVA + 6] = stub
    return bytes(t)


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
    opt += struct.pack("<HBBIIIII", 0x20B, 14, 0, len(text), len(rdata), 0, STARTUP, TEXT_RVA)
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
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "crtmain_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
