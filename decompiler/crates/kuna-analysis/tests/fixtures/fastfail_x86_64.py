#!/usr/bin/env python3
"""Generate `fastfail_x86_64.exe` -- a minimal PE32+ whose entry guards a Win32
call with the MSVC `__fastfail` idiom (`mov ecx,5 ; int 0x29`).

This is the shape crackmes.one/6a3822888a86e4c2c55254fe presents at
`0x140002d50`, reduced to the smallest image that reproduces it. x86 SLEIGH lifts
`INT imm8` to a `call` with no matching push, so the compiler spec's `extrapop`
raises RSP by 8 at the interrupt; where the two paths join the frame stops being a
constant offset from the spacebase, and the emitted C says so -- the stack locals
become offsets off a `char *` and the `CALL`'s own return-address push survives as
`*(unsigned long long *)&v6[-8] = 0x140001048;`. No toolchain on this host links a
Windows image, so the PE is assembled here byte by byte. Regenerate with:

    python3 fastfail_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  entry   sub rsp,0x48                    <- AddressOfEntryPoint
                         mov [rsp+0x40],rcx              ; spill the parameter
                         mov rax,[rsp+0x40]
                         mov rax,[rax]                   ; the algorithm handle
                         mov [rsp+0x38],rax              ; ... into a local
                         test rax,rax
                         jne  0x140001027
                         mov  ecx,5                      ; FAST_FAIL_LEGACY_GS_VIOLATION
                         int  0x29                       ; __fastfail(5), never returns
                         mov  rcx,[rsp+0x38]             ; <- the join
                         lea  rdx,[rsp+0x30]
                         lea  r8,[rsp+0x28]
                         mov  r9d,0x20
                         call [rip+..] -> IAT[0] BCryptGenerateSymmetricKey
                         add rsp,0x48 ; ret
  .rdata RVA 0x2000   import descriptor + INT + IAT + names (bcrypt.dll)
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200

TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY = 0x1000

IMPORTS = ["BCryptGenerateSymmetricKey"]
DLL = b"bcrypt.dll\0"


def build_rdata():
    """Import descriptor + INT + IAT + name blobs."""
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
    return bytes(blob), iat_rvas, RDATA_RVA + desc_off, 40, RDATA_RVA + iat_off, 8 * (n + 1)


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_text(iat_rvas):
    body = bytearray()

    def here():
        return ENTRY + len(body)

    body += bytes([0x48, 0x83, 0xEC, 0x48])                       # sub rsp,0x48
    body += bytes([0x48, 0x89, 0x4C, 0x24, 0x40])                 # mov [rsp+0x40],rcx
    body += bytes([0x48, 0x8B, 0x44, 0x24, 0x40])                 # mov rax,[rsp+0x40]
    body += bytes([0x48, 0x8B, 0x00])                             # mov rax,[rax]
    body += bytes([0x48, 0x89, 0x44, 0x24, 0x38])                 # mov [rsp+0x38],rax
    body += bytes([0x48, 0x85, 0xC0])                             # test rax,rax
    body += bytes([0x75, 0x07])                                   # jne +7 (past the fastfail)
    body += bytes([0xB9, 0x05, 0x00, 0x00, 0x00])                 # mov ecx,5
    body += bytes([0xCD, 0x29])                                   # int 0x29
    body += bytes([0x48, 0x8B, 0x4C, 0x24, 0x38])                 # mov rcx,[rsp+0x38]
    body += bytes([0x48, 0x8D, 0x54, 0x24, 0x30])                 # lea rdx,[rsp+0x30]
    body += bytes([0x4C, 0x8D, 0x44, 0x24, 0x28])                 # lea r8,[rsp+0x28]
    body += bytes([0x41, 0xB9, 0x20, 0x00, 0x00, 0x00])           # mov r9d,0x20
    body += b"\xFF\x15" + rel32(here(), 6, iat_rvas[0])           # call [BCryptGenerate..]
    body += bytes([0x48, 0x83, 0xC4, 0x48, 0xC3])                 # add rsp,0x48 ; ret
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
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "fastfail_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
