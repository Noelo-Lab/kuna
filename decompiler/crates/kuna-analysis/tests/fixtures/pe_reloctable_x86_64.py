#!/usr/bin/env python3
"""Generate `pe_reloctable_x86_64.exe` — a minimal PE32+ carrying an
address-taken function that no other entry oracle can see, plus the two
negatives the oracle has to decline.

The shape of the witness (a crackmes.one x64 VM interpreter): a handler table in
`.rdata` holds absolute pointers to the VM's handlers, the dispatcher reaches
them with `LEA RCX,[handler]` rather than a `CALL`, and MSVC left the leaf
handlers out of `.pdata`.  Entry point, exports, TLS callbacks, `.pdata` and the
recursive-descent call sweep therefore all miss them, and the function bounder
folds each into the function ahead of it.

The image's own base-relocation table is what names those words as addresses.

Layout (ImageBase 0x140000000):

  .text   RVA 0x1000, executable
    0x140001000  _start    sub rsp,0x28 ; xor ecx,ecx ; call dispatch
                           add rsp,0x28 ; ret          <- AddressOfEntryPoint
    0x140001020  dispatch  sub rsp,0x28 ; test ecx,ecx ; jz label
                           mov eax,1 ; add rsp,0x28 ; ret
    0x140001040  label     mov eax,2 ; add rsp,0x28 ; ret
                           ^ INTERIOR to dispatch's .pdata record: a jump-table
                             destination, NOT a function start
    0x140001060  handler   mov byte [rcx+0x48],0 ; ret
                           ^ named by NOTHING: no .pdata record, no call site,
                             reached only through the .rdata pointer below
    0x140001080  tail      xor eax,eax ; ret
                           ^ .pdata again, so the handler sits in a GAP between
                             two described bodies rather than past the last one
    0x1400010a0  payload   "h\0i\0" — data merged into the executable section,
                           past everything .pdata vouches for (the jormungandr
                           shape, where a UTF-16 locale table sits below the
                           first RUNTIME_FUNCTION)
  .rdata  RVA 0x2000, NOT executable — the pointer table, all four DIR64-relocated
    +0x00  0x140001060   -> handler   the one candidate that must be discovered
    +0x08  0x140001040   -> label     interior to a body: must be declined
    +0x10  0x140002000   -> itself    not executable: must be ignored
    +0x18  0x1400010a0   -> payload   executable but unvouched: must be declined
  .pdata  RVA 0x3000   {0x1000,0x1010} _start, {0x1020,0x1060} dispatch and
                       {0x1080,0x1088} tail.  Deliberately NO record for handler.
  .xdata  RVA 0x4000   one ordinary UNWIND_INFO (flags = 0)
  .reloc  RVA 0x5000   one block over page 0x2000 with the three DIR64 entries

No Windows toolchain on this host, so the PE is assembled byte by byte (same
pattern as `pe_chainedunwind_x86_64.py`).  Regenerate with:

    python3 pe_reloctable_x86_64.py
"""
import os
import struct

IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200

TEXT_RVA = 0x1000
RDATA_RVA = 0x2000
PDATA_RVA = 0x3000
XDATA_RVA = 0x4000
RELOC_RVA = 0x5000

START = 0x1000
START_END = 0x1010
DISPATCH = 0x1020
LABEL = 0x1040
DISPATCH_END = 0x1060
HANDLER = 0x1060
HANDLER_END = 0x1065
TAIL = 0x1080
TAIL_END = 0x1088
PAYLOAD = 0x10A0

UW_RVA = XDATA_RVA
IMAGE_REL_BASED_ABSOLUTE = 0
IMAGE_REL_BASED_DIR64 = 10


def build_text():
    t = bytearray(0xB0)

    def put(rva, b):
        off = rva - TEXT_RVA
        t[off:off + len(b)] = b
        return rva + len(b)

    # ---- _start [0x1000, 0x1010) --------------------------------------------
    p = START
    p = put(p, bytes([0x48, 0x83, 0xEC, 0x28]))          # sub  rsp,0x28
    p = put(p, bytes([0x33, 0xC9]))                      # xor  ecx,ecx
    disp = DISPATCH - (p + 5)
    p = put(p, b"\xE8" + struct.pack("<i", disp))        # call dispatch
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))          # add  rsp,0x28
    p = put(p, bytes([0xC3]))                            # ret
    assert p == START_END, hex(p)

    # ---- dispatch [0x1020, 0x1060), with LABEL inside it --------------------
    p = DISPATCH
    p = put(p, bytes([0x48, 0x83, 0xEC, 0x28]))          # sub  rsp,0x28
    p = put(p, bytes([0x85, 0xC9]))                      # test ecx,ecx
    disp = LABEL - (p + 6)
    p = put(p, b"\x0F\x84" + struct.pack("<i", disp))    # jz   label
    p = put(p, bytes([0xB8, 0x01, 0x00, 0x00, 0x00]))    # mov  eax,1
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))          # add  rsp,0x28
    p = put(p, bytes([0xC3]))                            # ret
    while p < LABEL:
        p = put(p, bytes([0xCC]))                        # int3 (pad)
    assert p == LABEL, hex(p)
    p = put(p, bytes([0xB8, 0x02, 0x00, 0x00, 0x00]))    # mov  eax,2
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))          # add  rsp,0x28
    p = put(p, bytes([0xC3]))                            # ret
    while p < DISPATCH_END:
        p = put(p, bytes([0xCC]))                        # int3 (pad)

    # ---- handler [0x1060, 0x1065): the address-taken leaf -------------------
    assert p == HANDLER, hex(p)
    p = put(p, bytes([0xC6, 0x41, 0x48, 0x00]))          # mov  byte [rcx+0x48],0
    p = put(p, bytes([0xC3]))                            # ret
    assert p == HANDLER_END, hex(p)

    # ---- tail [0x1080, 0x1088): described again, so handler is in a gap ------
    p = TAIL
    p = put(p, bytes([0x33, 0xC0]))                      # xor  eax,eax
    p = put(p, bytes([0xC3]))                            # ret

    # ---- payload [0x10a0, ...): data living in the executable section --------
    put(PAYLOAD, "hi".encode("utf-16-le"))
    return bytes(t)


def build_rdata():
    """The pointer table: a real handler, an interior label, a non-code word."""
    return (struct.pack("<Q", IMAGE_BASE + HANDLER)
            + struct.pack("<Q", IMAGE_BASE + LABEL)
            + struct.pack("<Q", IMAGE_BASE + RDATA_RVA)
            + struct.pack("<Q", IMAGE_BASE + PAYLOAD))


def build_pdata():
    e = bytearray()
    e += struct.pack("<III", START, START_END, UW_RVA)
    e += struct.pack("<III", DISPATCH, DISPATCH_END, UW_RVA)
    e += struct.pack("<III", TAIL, TAIL_END, UW_RVA)
    return bytes(e)


def build_xdata():
    # version 1, flags 0, SizeOfProlog 4, 1 unwind code (UWOP_ALLOC_SMALL 0x28)
    return bytes([0x01, 0x04, 0x01, 0x00, 0x04, 0x42, 0x00, 0x00])


def build_reloc():
    """One block over the .rdata page: DIR64 on each of the three table slots."""
    entries = [(IMAGE_REL_BASED_DIR64, 0x00),
               (IMAGE_REL_BASED_DIR64, 0x08),
               (IMAGE_REL_BASED_DIR64, 0x10),
               (IMAGE_REL_BASED_DIR64, 0x18)]
    b = struct.pack("<II", RDATA_RVA, 8 + 2 * len(entries))
    for typ, off in entries:
        b += struct.pack("<H", (typ << 12) | off)
    return b


def build():
    text = build_text()
    rdata = build_rdata()
    pdata = build_pdata()
    xdata = build_xdata()
    reloc = build_reloc()

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)

    nsec = 5
    opt_size = 240
    hdr_size = 0x40 + 4 + 20 + opt_size + 40 * nsec
    headers_sz = (hdr_size + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN

    def align(n):
        return (n + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN

    text_off = headers_sz
    rdata_off = text_off + align(len(text))
    pdata_off = rdata_off + align(len(rdata))
    xdata_off = pdata_off + align(len(pdata))
    reloc_off = xdata_off + align(len(xdata))
    image_sz = RELOC_RVA + (len(reloc) + SECT_ALIGN - 1) // SECT_ALIGN * SECT_ALIGN

    b = bytearray(dos)
    b += b"PE\0\0"
    b += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)
    opt = bytearray()
    opt += struct.pack("<HBBIIIII", 0x20B, 14, 0, len(text),
                       len(rdata) + len(pdata) + len(xdata) + len(reloc),
                       0, START, TEXT_RVA)
    opt += struct.pack("<Q", IMAGE_BASE)
    opt += struct.pack("<IIHHHHHHIIIIHHQQQQII",
                       SECT_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
                       image_sz, headers_sz, 0, 3, 0x8160,
                       0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = [(0, 0)] * 16
    dirs[3] = (PDATA_RVA, len(pdata))       # IMAGE_DIRECTORY_ENTRY_EXCEPTION
    dirs[5] = (RELOC_RVA, len(reloc))       # IMAGE_DIRECTORY_ENTRY_BASERELOC
    for rva, sz in dirs:
        opt += struct.pack("<II", rva, sz)
    assert len(opt) == opt_size, len(opt)
    b += opt

    def sect(name, vsz, rva, rsz, roff, chars):
        return (name.encode().ljust(8, b"\0")
                + struct.pack("<IIIIIIHHI", vsz, rva, rsz, roff, 0, 0, 0, 0, chars))

    b += sect(".text", len(text), TEXT_RVA, align(len(text)), text_off, 0x60000020)
    b += sect(".rdata", len(rdata), RDATA_RVA, align(len(rdata)), rdata_off, 0x40000040)
    b += sect(".pdata", len(pdata), PDATA_RVA, align(len(pdata)), pdata_off, 0x40000040)
    b += sect(".xdata", len(xdata), XDATA_RVA, align(len(xdata)), xdata_off, 0x40000040)
    b += sect(".reloc", len(reloc), RELOC_RVA, align(len(reloc)), reloc_off, 0x42000040)
    b += bytes(headers_sz - len(b))
    for blob in (text, rdata, pdata, xdata, reloc):
        b += blob.ljust(align(len(blob)), b"\0")
    return bytes(b)


if __name__ == "__main__":
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "pe_reloctable_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
