#!/usr/bin/env python3
"""Generate `pe_cookiecheck_x86_64.exe` -- a minimal PE32+ whose `main` is
compiled with MSVC `/GS`, so it ends by calling `__security_check_cookie` and
returns the zero it placed in EAX *before* that call (RE-need
`main-returns-invented-cookie`, round-6 challenge 69b7492049fa49a2a260218f).

`x86-64-win.cspec` lists RAX in `<killedbycall>`, so without
`option calleeretpreserves` the zero is dead and kuna emits
`return sub_140001080(v2);` -- a return value the machine never computes.  The
checker body here is `__security_check_cookie` verbatim, including the direct JMP
its failure path takes into a `__fastfail` stub, because that `int 0x29` is what
makes the callee-body write probe declare itself incomplete unless it knows the
interrupt never returns.  The CRT startup is `crtmain_x86_64.py`'s, so
`entrymainproto` gives `main` its argc/argv/envp prototype and the RET keeps a
return value to lose.  No toolchain on this host links a Windows image, so the PE
is assembled here byte by byte.  Regenerate with:

    python3 pe_cookiecheck_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  main       /GS prologue, xor eax,eax, call the cookie check
    0x140001030  startup    sub rsp,0x28                <- AddressOfEntryPoint
                            call venv_env   ; mov rdi,rax
                            call venv_argv  ; mov rbx,[rax]
                            call venv_argc  ; mov r8,rdi ; mov rdx,rbx ; mov ecx,[rax]
                            call main
                            add rsp,0x28 ; ret
    0x140001060  venv_env   jmp [rip+..]  -> IAT[0] _get_initial_narrow_environment
    0x140001066  venv_argv  jmp [rip+..]  -> IAT[1] __p___argv
    0x14000106c  venv_argc  jmp [rip+..]  -> IAT[2] __p___argc
    0x140001080  cookiechk  __security_check_cookie, tail-jumping to
    0x1400010c0  fastfail   mov ecx,2 ; int 0x29
  .rdata RVA 0x2000   import descriptor + INT + IAT + names + __security_cookie
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200

TEXT_RVA = 0x1000
RDATA_RVA = 0x2000

MAIN = 0x1000
STARTUP = 0x1030
VEN_ENV, VEN_ARGV, VEN_ARGC = 0x1060, 0x1066, 0x106C
COOKIECHK = 0x1080
FASTFAIL = 0x10C0
TEXT_SIZE = 0x100

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
    imp_size = len(blob)
    cookie_rva = RDATA_RVA + len(blob)
    blob += struct.pack("<Q", 0x0123456789ABCDEF)             # __security_cookie
    iat_rvas = [RDATA_RVA + iat_off + 8 * i for i in range(n)]
    return (bytes(blob), iat_rvas, RDATA_RVA + desc_off, imp_size,
            RDATA_RVA + iat_off, 8 * (n + 1), cookie_rva)


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))



def build_text(iat_rvas, cookie_rva):
    t = bytearray(TEXT_SIZE)

    # main: the MSVC /GS prologue and epilogue around an empty body.
    p = MAIN
    m = bytearray()
    m += bytes([0x48, 0x83, 0xEC, 0x28])                          # sub rsp,0x28
    m += b"\x48\x8B\x05" + rel32(p + len(m), 7, cookie_rva)       # mov rax,[__security_cookie]
    m += bytes([0x48, 0x33, 0xC4])                                # xor rax,rsp
    m += bytes([0x48, 0x89, 0x44, 0x24, 0x20])                    # mov [rsp+0x20],rax
    m += bytes([0x33, 0xC0])                                      # xor eax,eax   <- what main returns
    m += bytes([0x48, 0x8B, 0x4C, 0x24, 0x20])                    # mov rcx,[rsp+0x20]
    m += bytes([0x48, 0x33, 0xCC])                                # xor rcx,rsp
    m += b"\xE8" + rel32(p + len(m), 5, COOKIECHK)                # call __security_check_cookie
    m += bytes([0x48, 0x83, 0xC4, 0x28])                          # add rsp,0x28
    m += bytes([0xC3])                                            # ret
    assert MAIN + len(m) <= STARTUP, hex(MAIN + len(m))
    t[MAIN - TEXT_RVA:MAIN - TEXT_RVA + len(m)] = m

    # startup: crtmain_x86_64.py's __scrt_common_main_seh, verbatim.
    body = bytearray()
    body += bytes([0x48, 0x83, 0xEC, 0x28])                       # sub rsp,0x28
    body += b"\xE8" + rel32(STARTUP + len(body), 5, VEN_ENV)
    body += bytes([0x48, 0x8B, 0xF8])                             # mov rdi,rax
    body += b"\xE8" + rel32(STARTUP + len(body), 5, VEN_ARGV)
    body += bytes([0x48, 0x8B, 0x18])                             # mov rbx,[rax]
    body += b"\xE8" + rel32(STARTUP + len(body), 5, VEN_ARGC)
    body += bytes([0x4C, 0x8B, 0xC7])                             # mov r8,rdi
    body += bytes([0x48, 0x8B, 0xD3])                             # mov rdx,rbx
    body += bytes([0x8B, 0x08])                                   # mov ecx,[rax]
    body += b"\xE8" + rel32(STARTUP + len(body), 5, MAIN)
    body += bytes([0x48, 0x83, 0xC4, 0x28, 0xC3])                 # add rsp,0x28 ; ret
    assert STARTUP + len(body) <= VEN_ENV, hex(STARTUP + len(body))
    t[STARTUP - TEXT_RVA:STARTUP - TEXT_RVA + len(body)] = body

    for ven, iat in zip((VEN_ENV, VEN_ARGV, VEN_ARGC), iat_rvas):
        stub = b"\xFF\x25" + rel32(ven, 6, iat)
        t[ven - TEXT_RVA:ven - TEXT_RVA + 6] = stub

    # __security_check_cookie, verbatim: the returning path writes RCX and the
    # flags and never touches RAX; the failure path leaves by a direct JMP.
    c = bytearray()
    c += b"\x48\x3B\x0D" + rel32(COOKIECHK + len(c), 7, cookie_rva)  # cmp rcx,[cookie]
    c += bytes([0x75, 0x10])                                      # jnz  +0x10
    c += bytes([0x48, 0xC1, 0xC1, 0x10])                          # rol  rcx,0x10
    c += bytes([0x66, 0xF7, 0xC1, 0xFF, 0xFF])                    # test cx,0xffff
    c += bytes([0x75, 0x01])                                      # jnz  +1
    c += bytes([0xC3])                                            # ret
    c += bytes([0x48, 0xC1, 0xC9, 0x10])                          # ror  rcx,0x10
    c += b"\xE9" + rel32(COOKIECHK + len(c), 5, FASTFAIL)          # jmp  __report_gsfailure
    assert COOKIECHK + len(c) <= FASTFAIL, hex(COOKIECHK + len(c))
    t[COOKIECHK - TEXT_RVA:COOKIECHK - TEXT_RVA + len(c)] = c

    # __report_gsfailure, reduced to the __fastfail it ends in.
    f = bytes([0xB9, 0x02, 0x00, 0x00, 0x00, 0xCD, 0x29, 0xC3])   # mov ecx,2 ; int 0x29 ; ret
    t[FASTFAIL - TEXT_RVA:FASTFAIL - TEXT_RVA + len(f)] = f
    return bytes(t)


def build():
    rdata, iat_rvas, imp_rva, imp_size, iat_rva, iat_size, cookie_rva = build_rdata()
    text = build_text(iat_rvas, cookie_rva)

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
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pe_cookiecheck_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
