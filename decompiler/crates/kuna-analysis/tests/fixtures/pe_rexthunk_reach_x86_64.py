#!/usr/bin/env python3
"""Generate a minimal PE32+ whose real import thunks follow a REX-valued byte and are reached without a rel32 branch.

Each linker thunk directly follows a function ending in `jmp qword ptr [rax+0x48]`
(`48 8B 01 48 FF 60 48`), so the byte before its `FF` is `48`. Each is reached through
one shape only: a `.rdata` or `.data` pointer, `mov rax, imm64; call rax`, a function
table, a pointer inside executable `.text`, a `.pdata` record, an export, an `E9` stub,
a short jump, or a pointer tail jump. `f_imm64_noret_last` ends in a call of the
`ExitProcess` thunk and `f_noret_last` in a call through the `.text` pointer, each
directly followed by another function. `FatalExit` is a control thunk after `CC`.
Pass a hex byte (for example `cc`) as the second argument to replace the `48`.
"""
import struct, sys

BASE = 0x140000000
FA = 0x200
TEXT, RDATA, DATA, PDATA, EDATA, RELOC = 0x1000, 0x2000, 0x3000, 0x4000, 0x5000, 0x6000
IMPORTS = [b"ExitProcess", b"ExitThread", b"GetTickCount", b"Sleep", b"GetLastError",
           b"GetCurrentProcessId", b"GetCurrentThreadId", b"Beep", b"GetVersion", b"FatalExit", b"RaiseFailFastException"]
PAD_PREV = int(sys.argv[2], 16) if len(sys.argv) > 2 else 0x48

def rel32(here, size, target):
    return struct.pack("<i", target - (here + size))

class Img:
    def __init__(s):
        s.text = bytearray(b"\xcc" * 0x600)
        s.relocs = []
    def put(s, rva, code):
        s.text[rva - TEXT:rva - TEXT + len(code)] = code

def build(out):
    # .rdata: import descriptor, INT, IAT, names; then pointer area
    rdata = bytearray(0x400)
    int_rva, iat_rva, names_rva = RDATA + 0x40, RDATA + 0xA0, RDATA + 0x100
    cur = names_rva
    name_rvas = []
    for nm in IMPORTS:
        e = b"\0\0" + nm + b"\0"
        rdata[cur - RDATA:cur - RDATA + len(e)] = e
        name_rvas.append(cur)
        cur += (len(e) + 1) & ~1
    dll = b"KERNEL32.dll\0"
    rdata[cur - RDATA:cur - RDATA + len(dll)] = dll
    struct.pack_into("<IIIII", rdata, 0, int_rva, 0, 0, cur, iat_rva)
    for k, r in enumerate(name_rvas):
        struct.pack_into("<Q", rdata, int_rva - RDATA + 8 * k, r)
        struct.pack_into("<Q", rdata, iat_rva - RDATA + 8 * k, r)
    slot = {nm: BASE + iat_rva + 8 * k for k, nm in enumerate(IMPORTS)}
    PTR_A = RDATA + 0x300      # .rdata pointer -> thunk ExitProcess
    TBL_D = RDATA + 0x310      # .rdata table [0]=filler [1]=thunk Sleep
    PTR_K = RDATA + 0x320      # .rdata pointer -> control thunk after CC (FatalExit)
    PTR_B = DATA + 0x10        # .data pointer -> thunk ExitThread

    im = Img()
    thunk = {}
    stub_short = None
    # thunk block: each thunk preceded by a function ending in 48 ff 60 48
    at = TEXT + 0x200
    order = [b"ExitProcess", b"ExitThread", b"GetTickCount", b"Sleep", b"GetLastError", b"Beep",
             b"GetCurrentProcessId", b"GetCurrentThreadId", b"GetVersion", b"RaiseFailFastException"]
    for nm in order:
        if nm == b"GetCurrentThreadId":
            im.put(at, b"\x33\xc9\xeb\x07")
            stub_short = at
            at += 4
        vc = bytes([0x48, 0x8B, 0x01, 0x48, 0xFF, 0x60, PAD_PREV])
        im.put(at, vc)
        t = at + len(vc)
        im.put(t, b"\xff\x25" + rel32(BASE + t, 6, slot[nm]))
        thunk[nm] = t
        at = (t + 6 + 15) & ~15
    # control: bare thunk after CC padding, data-only reference
    thunk[b"FatalExit"] = at + 4
    im.put(at + 4, b"\xff\x25" + rel32(BASE + at + 4, 6, slot[b"FatalExit"]))

    funcs = {}
    f = TEXT + 0x40
    def func(name, code):
        nonlocal f
        funcs[name] = f
        im.put(f, code)
        f = (f + len(code) + 15) & ~15
    # f1: call [rip+PTR_A]  (.rdata pointer)
    c = bytearray(b"\x48\x83\xec\x28\x33\xc9")
    c += b"\xff\x15" + rel32(TEXT + 0x40 + len(c), 6, PTR_A)
    c += b"\x48\x83\xc4\x28\xc3"
    func("f_rdata_ptr", c)
    # f2: call [rip+PTR_B]  (.data pointer)
    here = f
    c = bytearray(b"\x48\x83\xec\x28\x33\xc9")
    c += b"\xff\x15" + rel32(here + len(c), 6, PTR_B)
    c += b"\x48\x83\xc4\x28\xc3"
    func("f_data_ptr", c)
    # f3: mov rax, imm64 ; call rax
    here = f
    c = bytearray(b"\x48\x83\xec\x28\x48\xb8")
    im.relocs.append(here + len(c))
    c += struct.pack("<Q", BASE + thunk[b"GetTickCount"])
    c += b"\xff\xd0\x48\x83\xc4\x28\xc3"
    func("f_imm64", c)
    # f4: lea rax,[rip+TBL_D]; mov ecx,0x3e8; call [rax+8]
    here = f
    c = bytearray(b"\x48\x83\xec\x28")
    c += b"\x48\x8d\x05" + rel32(here + len(c), 7, TBL_D)
    c += b"\xb9\xe8\x03\x00\x00\xff\x50\x08\x48\x83\xc4\x28\xc3"
    func("f_table", c)
    # f7: call ilt ; ilt: jmp rel32 thunk (GetCurrentProcessId)
    here = f
    ilt = TEXT + 0x1f0
    c = bytearray(b"\x48\x83\xec\x28")
    c += b"\xe8" + rel32(here + len(c), 5, ilt)
    c += b"\x48\x83\xc4\x28\xc3"
    func("f_ilt", c)
    im.put(ilt, b"\xe9" + rel32(ilt, 5, thunk[b"GetCurrentProcessId"]))
    # f8: jmp short to thunk (GetCurrentThreadId), placed right before its vcall filler
    stub = stub_short
    here = f
    c = bytearray(b"\x48\x83\xec\x28")
    c += b"\xe8" + rel32(here + len(c), 5, stub)
    c += b"\x48\x83\xc4\x28\xc3"
    func("f_short", c)
    # f9: call [rip+PTR_K] control after CC
    here = f
    c = bytearray(b"\x48\x83\xec\x28\x33\xc9")
    c += b"\xff\x15" + rel32(here + len(c), 6, PTR_K)
    c += b"\x48\x83\xc4\x28\xc3"
    func("f_control", c)
    # f10: tail jump via .rdata pointer: jmp [rip+PTR_A]? use GetVersion via rdata ptr at +0x330 with jmp
    PTR_V = RDATA + 0x330
    here = f
    c = bytearray(b"\xff\x25" + rel32(here, 6, PTR_V))
    func("f_tailptr", c)
    # f11: call [rip+PTR_T] with the pointer inside executable .text, then return 1
    PTR_T = TEXT + 0x1e0
    here = f
    c = bytearray(b"\x48\x83\xec\x28\x33\xc9")
    c += b"\xff\x15" + rel32(here + len(c), 6, PTR_T)
    c += b"\xb8\x01\x00\x00\x00\x48\x83\xc4\x28\xc3"
    func("f_textptr", c)
    im.put(PTR_T, struct.pack("<Q", BASE + thunk[b"ExitProcess"])); im.relocs.append(PTR_T)
    # f12: call [rip+PTR_A] (.rdata) then return 1
    here = f
    c = bytearray(b"\x48\x83\xec\x28\x33\xc9")
    c += b"\xff\x15" + rel32(here + len(c), 6, PTR_A)
    c += b"\xb8\x01\x00\x00\x00\x48\x83\xc4\x28\xc3"
    func("f_rdata_ret1", c)
    # f13: noreturn call through the .text pointer as the LAST instruction; f14 follows immediately
    here = f
    c = bytearray(b"\x48\x83\xec\x28\x33\xc9")
    c += b"\xff\x15" + rel32(here + len(c), 6, PTR_T)
    f14 = here + len(c)
    c += b"\x48\x8d\x05" + rel32(f14, 7, RDATA + 0x100) + b"\xc3"
    funcs["f_noret_last"] = here
    funcs["f_next"] = f14
    im.put(here, c)
    f = (f + len(c) + 15) & ~15
    # f15: xor ecx,ecx; mov rax, imm64(ExitProcess thunk); call rax -- last instruction; f16 follows
    here = f
    c = bytearray(b"\x48\x83\xec\x28\x33\xc9\x48\xb8")
    im.relocs.append(here + len(c))
    c += struct.pack("<Q", BASE + thunk[b"ExitProcess"]) + b"\xff\xd0"
    f16 = here + len(c)
    c += b"\xb8\x07\x00\x00\x00\xc3"
    funcs["f_imm64_noret_last"] = here
    funcs["f_next2"] = f16
    im.put(here, c)
    f = (f + len(c) + 15) & ~15
    # entry: call every helper
    entry = TEXT
    c = bytearray(b"\x48\x83\xec\x28")
    for nm in ["f_data_ptr", "f_imm64", "f_table", "f_ilt", "f_short", "f_tailptr", "f_control", "f_textptr", "f_rdata_ret1", "f_noret_last", "f_rdata_ptr"]:
        c += b"\xe8" + rel32(entry + len(c), 5, funcs[nm])
    c += b"\x48\x83\xc4\x28\xc3"
    assert len(c) <= 0x40, len(c)
    im.put(entry, c)
    assert f <= TEXT + 0x1f0, hex(f)

    struct.pack_into("<Q", rdata, PTR_A - RDATA, BASE + thunk[b"ExitProcess"]); im.relocs.append(PTR_A)
    struct.pack_into("<Q", rdata, TBL_D - RDATA, BASE + funcs["f_control"]); im.relocs.append(TBL_D)
    struct.pack_into("<Q", rdata, TBL_D + 8 - RDATA, BASE + thunk[b"Sleep"]); im.relocs.append(TBL_D + 8)
    struct.pack_into("<Q", rdata, PTR_K - RDATA, BASE + thunk[b"FatalExit"]); im.relocs.append(PTR_K)
    struct.pack_into("<Q", rdata, PTR_V - RDATA, BASE + thunk[b"GetVersion"]); im.relocs.append(PTR_V)
    data = bytearray(0x200)
    struct.pack_into("<Q", data, PTR_B - DATA, BASE + thunk[b"ExitThread"]); im.relocs.append(PTR_B)

    # .pdata: one RUNTIME_FUNCTION for the Beep thunk (only reach), unwind info in .rdata
    UNW = RDATA + 0x380
    rdata[UNW - RDATA:UNW - RDATA + 4] = b"\x01\x00\x00\x00"
    pdata = bytearray(0x200)
    struct.pack_into("<III", pdata, 0, thunk[b"Beep"], thunk[b"Beep"] + 6, UNW)
    # .edata: export GetLastError thunk as "MyExport" and RaiseFailFastException thunk by its own name
    edata = bytearray(0x200)
    exps = [(b"MyExport", thunk[b"GetLastError"]), (b"RaiseFailFastException", thunk[b"RaiseFailFastException"])]
    dllname_rva = EDATA + 0x100
    edata[0x100:0x100 + 8] = b"adv.exe\0"
    eat, npt, ord_ = EDATA + 0x40, EDATA + 0x60, EDATA + 0x80
    cur = EDATA + 0x120
    for k, (nm, t) in enumerate(exps):
        struct.pack_into("<I", edata, eat - EDATA + 4 * k, t)
        struct.pack_into("<I", edata, npt - EDATA + 4 * k, cur)
        struct.pack_into("<H", edata, ord_ - EDATA + 2 * k, k)
        edata[cur - EDATA:cur - EDATA + len(nm) + 1] = nm + b"\0"
        cur += len(nm) + 1
    struct.pack_into("<IIHHIIIIIII", edata, 0, 0, 0, 0, 0, dllname_rva, 1, len(exps), len(exps), eat, npt, ord_)
    # .reloc: DIR64 entries grouped by page
    pages = {}
    for r in im.relocs:
        pages.setdefault(r & ~0xFFF, []).append(r & 0xFFF)
    reloc = bytearray()
    for pg in sorted(pages):
        ents = [0xA000 | o for o in sorted(pages[pg])]
        if len(ents) % 2: ents.append(0)
        reloc += struct.pack("<II", pg, 8 + 2 * len(ents)) + b"".join(struct.pack("<H", e) for e in ents)
    reloc = bytes(reloc).ljust(0x200, b"\0")

    secs = [(".text", TEXT, bytes(im.text), 0x60000020), (".rdata", RDATA, bytes(rdata), 0x40000040),
            (".data", DATA, bytes(data), 0xC0000040), (".pdata", PDATA, bytes(pdata), 0x40000040),
            (".edata", EDATA, bytes(edata), 0x40000040), (".reloc", RELOC, reloc, 0x42000040)]
    dos = bytearray(0x40); dos[0:2] = b"MZ"; struct.pack_into("<I", dos, 0x3C, 0x40)
    img = bytearray(dos) + b"PE\0\0"
    img += struct.pack("<HHIIIHH", 0x8664, len(secs), 0, 0, 0, 240, 0x0022)
    opt = bytearray(struct.pack("<HBBIIIII", 0x20B, 14, 0, len(im.text), 0x800, 0, TEXT, TEXT))
    opt += struct.pack("<Q", BASE)
    hdr = 0x400
    opt += struct.pack("<IIHHHHHHIIIIHHQQQQII", 0x1000, FA, 6, 0, 0, 0, 6, 0, 0, 0x7000, hdr, 0, 3, 0x8160,
                       0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = [(0, 0)] * 16
    dirs[0] = (EDATA, 0x200); dirs[1] = (RDATA, 40); dirs[3] = (PDATA, 12); dirs[5] = (RELOC, len(bytes(reloc).rstrip(b"\0")) or 8)
    dirs[12] = (iat_rva, 8 * (len(IMPORTS) + 1))
    for r, sz in dirs:
        opt += struct.pack("<II", r, sz)
    img += opt
    raw = hdr
    for nm, r, blob, fl in secs:
        rs = (len(blob) + FA - 1) & ~(FA - 1)
        img += nm.encode().ljust(8, b"\0") + struct.pack("<IIIIIIHHI", len(blob), r, rs, raw, 0, 0, 0, 0, fl)
        raw += rs
    img += bytes(hdr - len(img))
    for nm, r, blob, fl in secs:
        img += blob.ljust((len(blob) + FA - 1) & ~(FA - 1), b"\0")
    open(out, "wb").write(img)
    print("thunks:", {k.decode(): hex(BASE + v) for k, v in thunk.items()})
    print("funcs:", {k: hex(BASE + v) for k, v in funcs.items()})

if __name__ == "__main__":
    import os
    here = os.path.dirname(os.path.abspath(__file__))
    build(sys.argv[1] if len(sys.argv) > 1 else os.path.join(here, "pe_rexthunk_reach_x86_64.exe"))
