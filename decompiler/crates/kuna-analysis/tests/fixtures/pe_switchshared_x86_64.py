#!/usr/bin/env python3
"""Generate `pe_switchshared_x86_64.exe` - a minimal PE32+ carrying the MSVC
x64 shape GH-1824 reports as missing from `kuna xrefs`: two switches in one
function sharing a single hoisted `LEA RDI,[__ImageBase]`, where every case
body of the first switch ends in a `JMP` to the second dispatch, so a linear
walk back from the second `JMP RAX` never reaches the base:

    LEA  RDI,[0x140000000]
    CMP  EBX,3 ; JA sw2
    MOV  EAX,dword ptr [RDI + RBX*0x4 + 0x2000] ; ADD RAX,RDI ; JMP RAX
    ...case bodies: CALL case<n> ; JMP sw2
    sw2: CMP ESI,3 ; JA out
    MOV  EAX,dword ptr [RDI + RSI*0x4 + 0x2010] ; ADD RAX,RDI ; JMP RAX
    ...case bodies: CALL case<4+n> ; JMP out

Each case body holds a direct `CALL case<n>` that is reachable no other way.

`.pdata` gives every function a RUNTIME_FUNCTION so the inventory finds them
without symbols.

No Windows toolchain on this host, so the PE is assembled byte by byte (same
pattern as `pe_switchdelta_x86_64.py`).  Regenerate with:

    python3 pe_switchshared_x86_64.py

Layout (ImageBase 0x140000000):

  .text  RVA 0x1000
    0x140001000  case0     xor eax,eax ; ret
    0x140001010  case<n>   mov eax,n   ; ret        (n = 1..7, 16 bytes apart)
    0x140001080  dispatch  the two switches above; case bodies call
                           case0..case7, the epilogue calls [IAT+8]
    0x140001160  _start    calls dispatch          <- AddressOfEntryPoint
  .rdata RVA 0x2000
    0x2000  jump table 1: 4 DWORD RVAs of the first switch's case bodies
    0x2010  jump table 2: 4 DWORD RVAs of the second switch's case bodies
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

TABLE_RVA = 0x2000          # the first switch table, indexed off the image base
TABLE2_RVA = 0x2010         # the second, read through the same base
IAT_RVA = 0x2100
IMPDESC_RVA = 0x2200
ILT_RVA = 0x2240
HINTNAME_RVA = 0x2280
DLLNAME_RVA = 0x2300

CASE = [0x1000 + 0x10 * i for i in range(8)]
DISPATCH = 0x1080
START = 0x1160
TEXT_END = 0x1180


def build_text():
    """Returns (bytes, {label: rva}, case_body_rvas)."""
    t = bytearray(TEXT_END - TEXT_RVA)

    def put(rva, b):
        off = rva - TEXT_RVA
        t[off:off + len(b)] = b
        return rva + len(b)

    # ---- case0..case7: eight trivial leaves --------------------------------
    put(CASE[0], bytes([0x33, 0xC0, 0xC3]))                    # xor eax,eax ; ret
    for i in range(1, 8):
        put(CASE[i], bytes([0xB8, i, 0, 0, 0, 0xC3]))          # mov eax,i ; ret

    # ---- dispatch ---------------------------------------------------------
    p = DISPATCH
    p = put(p, bytes([0x53]))                                  # push rbx
    p = put(p, bytes([0x56]))                                  # push rsi
    p = put(p, bytes([0x57]))                                  # push rdi
    p = put(p, bytes([0x8B, 0xD9]))                            # mov  ebx,ecx
    p = put(p, bytes([0x8B, 0xF2]))                            # mov  esi,edx
    # lea rdi,[rip+d] -> IMAGE_BASE (RVA 0); MSVC's hoisted `lea rdi,[__ImageBase]`
    lea_at = p
    p = put(p, bytes([0x48, 0x8D, 0x3D]) + struct.pack("<i", 0 - (lea_at + 7)))
    p = put(p, bytes([0x83, 0xFB, 0x03]))                      # cmp  ebx,3
    ja1_at = p
    p += 6                                                     # ja sw2 (patched)
    # mov eax,dword ptr [rdi+rbx*4+TABLE_RVA]
    p = put(p, bytes([0x8B, 0x84, 0x9F]) + struct.pack("<I", TABLE_RVA))
    p = put(p, bytes([0x48, 0x03, 0xC7]))                      # add  rax,rdi
    p = put(p, bytes([0xFF, 0xE0]))                            # jmp  rax

    # four case bodies, each `call case<i>` then a jump to the second switch.
    bodies, exits = [], []
    for i in range(4):
        while p % 16:
            p = put(p, bytes([0xCC]))                          # int3 padding
        bodies.append(p)
        p = put(p, b"\xE8" + struct.pack("<i", CASE[i] - (p + 5)))   # call case<i>
        exits.append(p)
        p += 5                                                 # jmp sw2 (patched)

    while p % 16:
        p = put(p, bytes([0xCC]))
    sw2 = p
    p = put(p, bytes([0x83, 0xFE, 0x03]))                      # cmp  esi,3
    ja2_at = p
    p += 6                                                     # ja out (patched)
    # mov eax,dword ptr [rdi+rsi*4+TABLE2_RVA]
    p = put(p, bytes([0x8B, 0x84, 0xB7]) + struct.pack("<I", TABLE2_RVA))
    p = put(p, bytes([0x48, 0x03, 0xC7]))                      # add  rax,rdi
    p = put(p, bytes([0xFF, 0xE0]))                            # jmp  rax

    bodies2, exits2 = [], []
    for i in range(4, 8):
        while p % 16:
            p = put(p, bytes([0xCC]))
        bodies2.append(p)
        p = put(p, b"\xE8" + struct.pack("<i", CASE[i] - (p + 5)))   # call case<i>
        exits2.append(p)
        p += 5                                                 # jmp epilogue (patched)

    while p % 16:
        p = put(p, bytes([0xCC]))
    default_rva = p
    p = put(p, bytes([0x33, 0xC0]))                            # xor eax,eax
    epilogue = p
    p = put(p, bytes([0xFF, 0x15])
            + struct.pack("<i", (IAT_RVA + 8) - (p + 6)))      # call [GetModuleHandleA]
    p = put(p, bytes([0x5F]))                                  # pop rdi
    p = put(p, bytes([0x5E]))                                  # pop rsi
    p = put(p, bytes([0x5B]))                                  # pop rbx
    p = put(p, bytes([0xC3]))                                  # ret
    dispatch_end = p
    assert dispatch_end <= START, (hex(dispatch_end), hex(START))

    put(ja1_at, b"\x0F\x87" + struct.pack("<i", sw2 - (ja1_at + 6)))
    put(ja2_at, b"\x0F\x87" + struct.pack("<i", default_rva - (ja2_at + 6)))
    for at in exits:
        put(at, b"\xE9" + struct.pack("<i", sw2 - (at + 5)))
    for at in exits2:
        put(at, b"\xE9" + struct.pack("<i", epilogue - (at + 5)))

    # ---- _start -----------------------------------------------------------
    p = START
    p = put(p, bytes([0x48, 0x83, 0xEC, 0x28]))                # sub rsp,0x28
    p = put(p, bytes([0xB9, 0x02, 0x00, 0x00, 0x00]))          # mov ecx,2
    p = put(p, b"\xE8" + struct.pack("<i", DISPATCH - (p + 5)))  # call dispatch
    p = put(p, bytes([0x48, 0x83, 0xC4, 0x28]))                # add rsp,0x28
    p = put(p, bytes([0xC3]))                                  # ret
    assert p <= TEXT_END, hex(p)
    return bytes(t), bodies + bodies2, dispatch_end


def build_rdata(bodies):
    r = bytearray(0x400)

    def put(rva, b):
        off = rva - RDATA_RVA
        r[off:off + len(b)] = b

    # the two switch tables: DWORD RVAs, MSVC style (target = ImageBase + entry)
    put(TABLE_RVA, b"".join(struct.pack("<I", b) for b in bodies[:4]))
    put(TABLE2_RVA, b"".join(struct.pack("<I", b) for b in bodies[4:]))

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
    leaves = [(CASE[0], CASE[0] + 3)] + [(c, c + 6) for c in CASE[1:]]
    for start, end in leaves + [(DISPATCH, dispatch_end), (START, TEXT_END)]:
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
                       "pe_switchshared_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
