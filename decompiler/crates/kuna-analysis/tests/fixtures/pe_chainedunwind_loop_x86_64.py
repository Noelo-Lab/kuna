#!/usr/bin/env python3
"""Generate `pe_chainedunwind_loop_x86_64.exe` — a minimal PE32+ whose `.pdata` splits
ONE logical function across two RUNTIME_FUNCTION records, the second carrying
UNWIND_INFO with UNW_FLAG_CHAININFO (0x4) pointing back at the first.

This is the shape MSVC emits for a shrink-wrapped / separated register-save chunk
(GH-403: the reporter's `sub_140002650` falls through at 0x14000270c into a
`MOV [RSP+0xa0],RBX ; MOV [RSP+0xb0],RBP ; MOVAPS [RSP+0x80],XMM6` chunk that
MSVC gave its own chained RUNTIME_FUNCTION).  A chained record is NOT a function
start: Ghidra skips it (`ImageRuntimeFunctionEntries_X86.markup`:142 —
`if (beginAddress != 0 && !unwindInfo.hasChainedUnwindInfo())`).

No Windows toolchain on this host, so the PE is assembled byte by byte (same
pattern as `crtmain_x86_64.py`).  Regenerate with:

    python3 pe_chainedunwind_loop_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  primary   sub rsp,0x28 ; mov eax,ecx ; add eax,1 ; test eax,eax
                           jz  0x140001030            <- cold arm, taken branch
                           add eax,2 .. add eax,6     <- falls through at 0x140001020
    0x140001020  chunk     movaps [rsp+0x10],xmm6     <- CHAINED RUNTIME_FUNCTION begin
                           xor ebp,ebp ; add eax,7
                           add rsp,0x28 ; ret
    0x140001030  cold      mov eax,0x63 ; add rsp,0x28 ; ret
    0x140001040  _start    sub rsp,0x28 ; xor ecx,ecx ; call primary
                           add rsp,0x28 ; ret          <- AddressOfEntryPoint
  .pdata RVA 0x2000
    [0]  {0x1000, 0x1020, 0x3000}   primary   UNWIND_INFO flags=0
    [1]  {0x1020, 0x103a, 0x3008}   chunk     UNWIND_INFO flags=UNW_FLAG_CHAININFO,
                                              chained RUNTIME_FUNCTION -> {0x1000,0x1020,0x3000}
    [2]  {0x1040, 0x1050, 0x3000}   _start    UNWIND_INFO flags=0
  .xdata RVA 0x3000   the two UNWIND_INFO records
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200

TEXT_RVA = 0x1000
PDATA_RVA = 0x2000
XDATA_RVA = 0x3000

PRIMARY = 0x1000
CHUNK = 0x1020
COLD = 0x1030
START = 0x1040
FUNC_END = 0x103A
START_END = 0x1050

UW1_RVA = XDATA_RVA + 0x00     # ordinary UNWIND_INFO (flags = 0)
UW2_RVA = XDATA_RVA + 0x08     # chained UNWIND_INFO (flags = UNW_FLAG_CHAININFO)


def build_text():
    t = bytearray(0x60)

    def put(rva, b):
        off = rva - TEXT_RVA
        t[off:off + len(b)] = b
        return rva + len(b)

    # ---- primary [0x1000, 0x1020): a do-while whose EXIT falls into CHUNK ---
    p = PRIMARY
    p = put(p, bytes([0x48, 0x83, 0xEC, 0x28]))          # sub  rsp,0x28
    p = put(p, bytes([0x33, 0xC0]))                      # xor  eax,eax
    head = p                                             # 0x1006
    for imm in (1, 2, 3, 4, 5):
        p = put(p, bytes([0x83, 0xC0, imm]))             # add  eax,imm
    p = put(p, b"\x3D" + struct.pack("<i", 0x64))        # cmp  eax,0x64
    disp = head - (p + 6)
    p = put(p, b"\x0F\x8C" + struct.pack("<i", disp))    # jl   head  (latch)
    assert p == CHUNK, hex(p)                            # loop EXIT falls into CHUNK

    # ---- the chained chunk [0x1020, 0x103a): MSVC separated register saves ---
    p = put(p, bytes([0x0F, 0x29, 0x74, 0x24, 0x10]))    # movaps [rsp+0x10],xmm6
    p = put(p, bytes([0x33, 0xED]))                      # xor  ebp,ebp
    p = put(p, bytes([0x83, 0xC0, 0x07]))                # add  eax,7
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))          # add  rsp,0x28
    p = put(p, bytes([0xC3]))                            # ret
    assert p == COLD - 1, hex(p)
    p = put(p, bytes([0x90]))                            # nop (pad)

    # ---- the cold arm [0x1030, 0x103a) --------------------------------------
    assert p == COLD, hex(p)
    p = put(p, bytes([0xB8, 0x63, 0x00, 0x00, 0x00]))    # mov  eax,0x63
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))          # add  rsp,0x28
    p = put(p, bytes([0xC3]))                            # ret
    assert p == FUNC_END, hex(p)

    # ---- _start [0x1040, 0x104f) --------------------------------------------
    p = START
    p = put(p, bytes([0x48, 0x83, 0xEC, 0x28]))          # sub  rsp,0x28
    p = put(p, bytes([0x33, 0xC9]))                      # xor  ecx,ecx
    disp = PRIMARY - (p + 5)
    p = put(p, b"\xE8" + struct.pack("<i", disp))        # call primary
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))          # add  rsp,0x28
    p = put(p, bytes([0xC3]))                            # ret
    assert p == START_END, hex(p)
    return bytes(t)


def build_xdata():
    """UW1 (ordinary) at +0, UW2 (UNW_FLAG_CHAININFO) at +8."""
    x = bytearray()
    # UW1: version 1, flags 0, SizeOfProlog 4, 1 unwind code (UWOP_ALLOC_SMALL 0x28)
    x += bytes([0x01, 0x04, 0x01, 0x00])
    x += bytes([0x04, 0x42])          # offset 4, opcode 2 (ALLOC_SMALL) opinfo 4 => 0x28
    x += bytes([0x00, 0x00])          # pad to an even code count
    assert len(x) == 8, len(x)
    # UW2: version 1, flags UNW_FLAG_CHAININFO (0x4) => byte0 = 1 | (4 << 3) = 0x21
    x += bytes([0x21, 0x05, 0x02, 0x00])
    x += bytes([0x05, 0x86])          # offset 5, opcode 8 (SAVE_XMM128) opinfo 6 (xmm6)
    x += struct.pack("<H", 0x0002)    # scaled offset operand (2nd slot)
    # ...followed by the chained RUNTIME_FUNCTION: the PRIMARY entry.
    x += struct.pack("<III", PRIMARY, CHUNK, UW1_RVA)
    assert len(x) == 8 + 20, len(x)
    return bytes(x)


def build_pdata():
    e = bytearray()
    e += struct.pack("<III", PRIMARY, CHUNK, UW1_RVA)        # primary
    e += struct.pack("<III", CHUNK, FUNC_END, UW2_RVA)       # CHAINED - not a function
    e += struct.pack("<III", START, START_END, UW1_RVA)      # _start
    return bytes(e)


def build():
    text = build_text()
    pdata = build_pdata()
    xdata = build_xdata()

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)

    nsec = 3
    opt_size = 240
    hdr_size = 0x40 + 4 + 20 + opt_size + 40 * nsec
    headers_sz = (hdr_size + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    text_off = headers_sz
    text_sz = (len(text) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    pdata_off = text_off + text_sz
    pdata_sz = (len(pdata) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    xdata_off = pdata_off + pdata_sz
    xdata_sz = (len(xdata) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    image_sz = XDATA_RVA + (len(xdata) + SECT_ALIGN - 1) // SECT_ALIGN * SECT_ALIGN

    b = bytearray(dos)
    b += b"PE\0\0"
    b += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)
    opt = bytearray()
    opt += struct.pack("<HBBIIIII", 0x20B, 14, 0, len(text), len(pdata) + len(xdata),
                       0, START, TEXT_RVA)
    opt += struct.pack("<Q", IMAGE_BASE)
    opt += struct.pack("<IIHHHHHHIIIIHHQQQQII",
                       SECT_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
                       image_sz, headers_sz, 0, 3, 0x8160,
                       0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = [(0, 0)] * 16
    dirs[3] = (PDATA_RVA, len(pdata))      # IMAGE_DIRECTORY_ENTRY_EXCEPTION
    for rva, sz in dirs:
        opt += struct.pack("<II", rva, sz)
    assert len(opt) == opt_size, len(opt)
    b += opt

    def sect(name, vsz, rva, rsz, roff, chars):
        return (name.encode().ljust(8, b"\0")
                + struct.pack("<IIIIIIHHI", vsz, rva, rsz, roff, 0, 0, 0, 0, chars))

    b += sect(".text", len(text), TEXT_RVA, text_sz, text_off, 0x60000020)
    b += sect(".pdata", len(pdata), PDATA_RVA, pdata_sz, pdata_off, 0x40000040)
    b += sect(".xdata", len(xdata), XDATA_RVA, xdata_sz, xdata_off, 0x40000040)
    b += bytes(headers_sz - len(b))
    b += text.ljust(text_sz, b"\0")
    b += pdata.ljust(pdata_sz, b"\0")
    b += xdata.ljust(xdata_sz, b"\0")
    return bytes(b)


if __name__ == "__main__":
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "pe_chainedunwind_loop_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
