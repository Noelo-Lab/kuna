#!/usr/bin/env python3
"""Generate `pe_switchdelta_x86_64.exe` - a minimal PE32+ carrying the jump
table shape GH-456 reports as missing from `kuna decompile-graph`'s callee
lists: the MSVC x64 delta table.  The dispatch materializes the IMAGE BASE in
one instruction and indexes a table of 4-byte RVAs in another, so the table base
is in no operand and the entries are not pointers:

    LEA  RDX,[0x140000000]
    MOV  ECX,dword ptr [RDX + RBX*0x4 + 0x2000]
    ADD  RCX,RDX
    JMP  RCX

Each case body holds a direct `CALL case<n>` that is reachable no other way.

`.pdata` gives every function a RUNTIME_FUNCTION so the inventory finds them
without symbols, exactly as it does on the reporter's redtest.exe.

No Windows toolchain on this host, so the PE is assembled byte by byte (same
pattern as `pe_chainedunwind_x86_64.py`).  Regenerate with:

    python3 pe_switchdelta_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  case0     xor eax,eax ; ret
    0x140001010  case1     mov eax,1   ; ret
    0x140001020  case2     mov eax,2   ; ret
    0x140001030  case3     mov eax,3   ; ret
    0x140001040  dispatch  the switch above; case bodies call case0..case3,
                           the epilogue calls [IAT+8] (GetModuleHandleA)
    0x1400010c0  _start    calls dispatch          <- AddressOfEntryPoint
  .rdata RVA 0x2000
    0x2000  jump table: 4 DWORD RVAs of the case bodies
    0x2100  IAT   : Sleep, GetModuleHandleA
    0x2200  import descriptors
    0x2240  ILT
    0x2280  hint/name entries
    0x2300  "KERNEL32.dll"
  .pdata RVA 0x3000   one RUNTIME_FUNCTION per function
  .xdata RVA 0x4000   one shared UNWIND_INFO
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

TABLE_RVA = 0x2000          # the switch table, indexed off the image base
IAT_RVA = 0x2100
IMPDESC_RVA = 0x2200
ILT_RVA = 0x2240
HINTNAME_RVA = 0x2280
DLLNAME_RVA = 0x2300

CASE = [0x1000, 0x1010, 0x1020, 0x1030]
DISPATCH = 0x1040
START = 0x10C0
TEXT_END = 0x10E0


def build_text():
    """Returns (bytes, {label: rva}, case_body_rvas)."""
    t = bytearray(TEXT_END - TEXT_RVA)

    def put(rva, b):
        off = rva - TEXT_RVA
        t[off:off + len(b)] = b
        return rva + len(b)

    # ---- case0..case3: four trivial leaves --------------------------------
    put(CASE[0], bytes([0x33, 0xC0, 0xC3]))                    # xor eax,eax ; ret
    for i in (1, 2, 3):
        put(CASE[i], bytes([0xB8, i, 0, 0, 0, 0xC3]))          # mov eax,i ; ret

    # ---- dispatch ---------------------------------------------------------
    p = DISPATCH
    p = put(p, bytes([0x53]))                                  # push rbx
    p = put(p, bytes([0x8B, 0xD9]))                            # mov  ebx,ecx
    p = put(p, bytes([0x83, 0xFB, 0x03]))                      # cmp  ebx,3
    ja_at = p
    p += 6                                                     # ja default (patched)
    # lea rdx,[rip+d] -> IMAGE_BASE (RVA 0); MSVC's `lea rdx,[__ImageBase]`
    lea_at = p
    p = put(p, bytes([0x48, 0x8D, 0x15]) + struct.pack("<i", 0 - (lea_at + 7)))
    # mov ecx,dword ptr [rdx+rbx*4+TABLE_RVA]
    p = put(p, bytes([0x8B, 0x8C, 0x9A]) + struct.pack("<I", TABLE_RVA))
    p = put(p, bytes([0x48, 0x03, 0xCA]))                      # add  rcx,rdx
    p = put(p, bytes([0xFF, 0xE1]))                            # jmp  rcx

    # four case bodies, each `call case<i>` then a jump to the shared epilogue.
    # Nothing else in the image reaches them.
    bodies, exits = [], []
    for i in range(4):
        while p % 16:
            p = put(p, bytes([0xCC]))                          # int3 padding
        bodies.append(p)
        p = put(p, b"\xE8" + struct.pack("<i", CASE[i] - (p + 5)))   # call case<i>
        exits.append(p)
        p += 5                                                 # jmp epilogue (patched)

    while p % 16:
        p = put(p, bytes([0xCC]))
    default_rva = p
    p = put(p, bytes([0x33, 0xC0]))                            # xor eax,eax
    epilogue = p
    p = put(p, bytes([0xFF, 0x15])
            + struct.pack("<i", (IAT_RVA + 8) - (p + 6)))      # call [GetModuleHandleA]
    p = put(p, bytes([0x5B]))                                  # pop rbx
    p = put(p, bytes([0xC3]))                                  # ret
    dispatch_end = p
    assert dispatch_end <= START, (hex(dispatch_end), hex(START))

    put(ja_at, b"\x0F\x87" + struct.pack("<i", default_rva - (ja_at + 6)))
    for at in exits:
        put(at, b"\xE9" + struct.pack("<i", epilogue - (at + 5)))

    # ---- _start -----------------------------------------------------------
    p = START
    p = put(p, bytes([0x48, 0x83, 0xEC, 0x28]))                # sub rsp,0x28
    p = put(p, bytes([0xB9, 0x02, 0x00, 0x00, 0x00]))          # mov ecx,2
    p = put(p, b"\xE8" + struct.pack("<i", DISPATCH - (p + 5)))  # call dispatch
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))                # add rsp,0x28
    p = put(p, bytes([0xC3]))                                  # ret
    assert p <= TEXT_END, hex(p)
    return bytes(t), bodies, dispatch_end


def build_rdata(bodies):
    r = bytearray(0x400)

    def put(rva, b):
        off = rva - RDATA_RVA
        r[off:off + len(b)] = b

    # the switch table: four DWORD RVAs, MSVC style (target = ImageBase + entry)
    put(TABLE_RVA, b"".join(struct.pack("<I", b) for b in bodies))

    names = [(0, b"Sleep"), (0, b"GetModuleHandleA")]
    hn_rvas = []
    off = HINTNAME_RVA
    for hint, nm in names:
        hn_rvas.append(off)
        blob = struct.pack("<H", hint) + nm + b"\0"
        if len(blob) % 2:
            blob += b"\0"
        put(off, blob)
        off += len(blob)

    thunks = b"".join(struct.pack("<Q", rva) for rva in hn_rvas) + struct.pack("<Q", 0)
    put(IAT_RVA, thunks)
    put(ILT_RVA, thunks)
    put(DLLNAME_RVA, b"KERNEL32.dll\0")
    put(IMPDESC_RVA, struct.pack("<IIIII", ILT_RVA, 0, 0, DLLNAME_RVA, IAT_RVA)
        + struct.pack("<IIIII", 0, 0, 0, 0, 0))
    return bytes(r)


def build_pdata(dispatch_end):
    uw = XDATA_RVA
    e = bytearray()
    for start, end in [(CASE[0], CASE[0] + 3), (CASE[1], CASE[1] + 6),
                       (CASE[2], CASE[2] + 6), (CASE[3], CASE[3] + 6),
                       (DISPATCH, dispatch_end), (START, TEXT_END)]:
        e += struct.pack("<III", start, end, uw)
    return bytes(e)


def build_xdata():
    # version 1, flags 0, SizeOfProlog 0, 0 unwind codes
    return bytes([0x01, 0x00, 0x00, 0x00])


def build():
    text, bodies, dispatch_end = build_text()
    rdata = build_rdata(bodies)
    pdata = build_pdata(dispatch_end)
    xdata = build_xdata()

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)

    nsec = 4
    opt_size = 240
    hdr_size = 0x40 + 4 + 20 + opt_size + 40 * nsec
    headers_sz = (hdr_size + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    offs, cur = {}, headers_sz
    for name, blob in (("text", text), ("rdata", rdata), ("pdata", pdata), ("xdata", xdata)):
        offs[name] = cur
        cur += (len(blob) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
    image_sz = XDATA_RVA + SECT_ALIGN

    b = bytearray(dos)
    b += b"PE\0\0"
    b += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)
    opt = bytearray()
    opt += struct.pack("<HBBIIIII", 0x20B, 14, 0, len(text),
                       len(rdata) + len(pdata) + len(xdata), 0, START, TEXT_RVA)
    opt += struct.pack("<Q", IMAGE_BASE)
    opt += struct.pack("<IIHHHHHHIIIIHHQQQQII",
                       SECT_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
                       image_sz, headers_sz, 0, 3, 0x8160,
                       0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = [(0, 0)] * 16
    dirs[1] = (IMPDESC_RVA, 40)              # IMPORT
    dirs[3] = (PDATA_RVA, len(pdata))        # EXCEPTION
    dirs[12] = (IAT_RVA, 24)                 # IAT
    for rva, sz in dirs:
        opt += struct.pack("<II", rva, sz)
    assert len(opt) == opt_size, len(opt)
    b += opt

    def sect(name, vsz, rva, blob, chars):
        rsz = (len(blob) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN
        return (name.encode().ljust(8, b"\0")
                + struct.pack("<IIIIIIHHI", vsz, rva, rsz, offs[name.lstrip(".")],
                              0, 0, 0, 0, chars))

    b += sect(".text", len(text), TEXT_RVA, text, 0x60000020)
    b += sect(".rdata", len(rdata), RDATA_RVA, rdata, 0x40000040)
    b += sect(".pdata", len(pdata), PDATA_RVA, pdata, 0x40000040)
    b += sect(".xdata", len(xdata), XDATA_RVA, xdata, 0x40000040)
    b += bytes(headers_sz - len(b))
    for blob in (text, rdata, pdata, xdata):
        b += blob.ljust((len(blob) + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN, b"\0")
    return bytes(b)


if __name__ == "__main__":
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "pe_switchdelta_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
