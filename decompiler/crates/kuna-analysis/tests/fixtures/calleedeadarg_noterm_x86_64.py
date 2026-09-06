#!/usr/bin/env python3
"""Generate `calleedeadarg_noterm_x86_64.exe` — a minimal PE32+ that passes one
snapshot handle to two Win32 imports from a dominating predecessor block.

This is `KeyCheker.exe`'s process-enumeration loop (crackmes.one
640a526833c5d447bc761899) reduced to the smallest image that reproduces its
argument loss. `calleedeadarg` decides whether a callee clobbers the argument
register by decoding the callee's entry, and a PE import's entry address IS its
IAT slot — so the walk reads a pointer as instructions. Each import's hint/name
entry is placed at an RVA ending in `0xF4`, so every IAT slot starts with the
byte the walk sees as `HLT`, whose p-code is a branch to itself: the walk closes
back onto an address it has already visited and ends having recorded no path
terminator at all. `Process32NextW()` and `CloseHandle()` came out with empty
argument lists from that state.

`abort` is imported and called because the veto only reaches an import once the
no-return discovery has forced a re-flow — before that the call is an
unresolved CALLIND with no entry address to decode.

No toolchain on this host links a Windows image, so the PE is assembled here
byte by byte. Regenerate with:

    python3 calleedeadarg_noterm_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  entry   sub rsp,0x38                    <- AddressOfEntryPoint
                         mov ecx,2                       <- TH32CS_SNAPPROCESS
                         call [rip+..] -> CreateToolhelp32Snapshot
                         mov rcx,rax                     <- the leftover result
                         test rax,rax ; jne L_close
                         lea rdx,[rsp+0x20]
                         call [rip+..] -> Process32NextW  (rcx,rdx)
                         add rsp,0x38 ; ret
                 L_close call [rip+..] -> CloseHandle     (rcx)
                         test eax,eax ; jne L_abort
                         add rsp,0x38 ; ret
                 L_abort call [rip+..] -> abort           (no-return)
  .rdata RVA 0x2000   import descriptor + INT + IAT + DLL name
         RVA 0x20F4 + k*0x100   the four hint/name entries
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200

TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
ENTRY = 0x1000

IMPORTS = ["CreateToolhelp32Snapshot", "Process32NextW", "CloseHandle", "abort"]
DLL = b"KERNEL32.dll\0"

# Each hint/name entry sits at its own page-aligned RVA plus this offset, so the
# IAT slot that points at it reads `F4 2x 00 00 00 00 00 00` — `hlt` first.
NAME_LOW_BYTE = 0xF4
NAME_STRIDE = 0x100
NAME_FIRST = 0x2000 + NAME_STRIDE + NAME_LOW_BYTE


def build_rdata():
    """Import descriptor + INT + IAT + the name blobs at their pinned RVAs."""
    n = len(IMPORTS)
    int_off = 20 * 2                                     # one descriptor + a null one
    iat_off = int_off + 8 * (n + 1)
    dll_off = iat_off + 8 * (n + 1)

    name_rvas = [NAME_FIRST + NAME_STRIDE * i for i in range(n)]
    assert all(r & 0xFF == NAME_LOW_BYTE for r in name_rvas)

    blob = bytearray()
    blob += struct.pack("<IIIII", RDATA_RVA + int_off, 0, 0, RDATA_RVA + dll_off,
                        RDATA_RVA + iat_off)
    blob += bytes(20)                                    # null descriptor
    for r in name_rvas:
        blob += struct.pack("<Q", r)
    blob += struct.pack("<Q", 0)                         # INT terminator
    for r in name_rvas:
        blob += struct.pack("<Q", r)                     # the IAT mirrors the INT
    blob += struct.pack("<Q", 0)
    blob += DLL
    assert len(blob) <= NAME_FIRST - RDATA_RVA, len(blob)

    for r, nm in zip(name_rvas, IMPORTS):
        blob = blob.ljust(r - RDATA_RVA, b"\0")
        blob += struct.pack("<H", 0) + nm.encode() + b"\0"

    iat_rvas = [RDATA_RVA + iat_off + 8 * i for i in range(n)]
    return bytes(blob), iat_rvas, RDATA_RVA, 40, RDATA_RVA + iat_off, 8 * (n + 1)


def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))


def build_text(iat_rvas):
    snapshot, next_w, close, abort = iat_rvas
    body = bytearray()

    def here():
        return ENTRY + len(body)

    body += bytes([0x48, 0x83, 0xEC, 0x38])                       # sub rsp,0x38
    body += bytes([0xB9, 0x02, 0x00, 0x00, 0x00])                 # mov ecx,2
    body += b"\xFF\x15" + rel32(here(), 6, snapshot)              # call [CreateToolhelp32Snapshot]
    body += bytes([0x48, 0x8B, 0xC8])                             # mov rcx,rax
    body += bytes([0x48, 0x85, 0xC0])                             # test rax,rax
    jne_close = len(body)
    body += bytes([0x75, 0x00])                                   # jne L_close (patched)
    body += bytes([0x48, 0x8D, 0x54, 0x24, 0x20])                 # lea rdx,[rsp+0x20]
    body += b"\xFF\x15" + rel32(here(), 6, next_w)                # call [Process32NextW]
    body += bytes([0x48, 0x83, 0xC4, 0x38, 0xC3])                 # add rsp,0x38 ; ret

    body[jne_close + 1] = len(body) - (jne_close + 2)             # L_close
    body += b"\xFF\x15" + rel32(here(), 6, close)                 # call [CloseHandle]
    body += bytes([0x85, 0xC0])                                   # test eax,eax
    jne_abort = len(body)
    body += bytes([0x75, 0x00])                                   # jne L_abort (patched)
    body += bytes([0x48, 0x83, 0xC4, 0x38, 0xC3])                 # add rsp,0x38 ; ret

    body[jne_abort + 1] = len(body) - (jne_abort + 2)             # L_abort
    body += b"\xFF\x15" + rel32(here(), 6, abort)                 # call [abort]
    body += bytes([0xCC])                                         # int3
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
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "calleedeadarg_noterm_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
