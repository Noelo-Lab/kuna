#!/usr/bin/env python3
"""Generate `pe_addrtaken_x86_64.exe` — a minimal PE32+ whose functions are
reached only through a pointer the program takes, plus the look-alikes that are
not functions (GH-1039, `addrtakenentry`).

Shape of the issue's witness (a stripped zig/MinGW x64 build): a registration
stub stores `lea rax,[handler]` to a global and the handler is called through
that global, and a `.data` table of relocated handler pointers sits in front of
the first `RUNTIME_FUNCTION`. The leaves have no `.pdata` record, so no entry
oracle names them and the bounder folds each into the function before it.

Layout (ImageBase 0x140000000; `.pdata` describes only `_start`):

  .text   RVA 0x1000, executable
    0x140001000  pre_a     lea eax,[rcx+rcx*2] ; add eax,1 ; ret
                           ^ named only by the relocated .rdata word below,
                             before the first RUNTIME_FUNCTION: a function
    0x140001010  pre_b     lea eax,[rcx*8] ; sub eax,ecx ; ret
                           ^ same, chained after pre_a
    0x140001020  str16     "hr-HR" UTF-16, zero padded
                           ^ relocated and lea'd, before the first record:
                             the jormungandr data shape, NOT a function
    0x140001030  register  lea rax,[handler] ; mov [g_sink],rax ; ret
                           ^ called by _start
    0x140001040  handler   lea eax,[rcx+rcx*4] ; lea eax,[rcx+rax*2] ;
                           add eax,-4 ; ret
                           ^ named only by register's lea: a function
    0x140001050  runon     xor eax,eax, then straight into _start
                           ^ lea'd, but runs into other code: NOT a function
    0x140001052  _start    the only .pdata record; calls register, the
                           handler through g_sink, pre_a through the table and
                           interp, and lea's runon, handler+3 (an interior
                           address), desc and blob
    0x1400010a0  desc      01 00 00 00 2c c7 00 00 c2 00 00, a MinGW
                           delay-import descriptor's shape: lea'd after a ret,
                           decodes to add [rax],eax ; add [rax],al ; ... ;
                           ret 0. NOT a function
    0x1400010b0  interp    sub rsp,0x28 ; lea rax,[label] ; jmp rax — called
                           by _start
    0x1400010c0  label     add rsp,0x28 ; ret — lea'd by interp, right after
                           its dispatch jump, and tears down interp's frame:
                           gcc's computed-goto return block, NOT a function
    0x1400010d0  payload   xor eax,eax ; ret — relocated, but past the last
                           record: left out, as before
    0x1400010e0  blob      a UTF-16 string lea'd by _start: NOT a function
  .rdata  RVA 0x2000   pre_a, pre_b, str16, payload (all DIR64-relocated)
  .pdata  RVA 0x3000   one record for _start
  .xdata  RVA 0x4000   one UNWIND_INFO (UWOP_ALLOC_SMALL 0x28)
  .reloc  RVA 0x5000   the four DIR64 entries
  .data   RVA 0x6000   g_sink

No Windows toolchain on this host, so the PE is assembled byte by byte (the
pattern of `pe_reloctable_x86_64.py`). Regenerate with:

    python3 pe_addrtaken_x86_64.py
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
DATA_RVA = 0x6000

PRE_A = 0x1000
PRE_B = 0x1010
STR16 = 0x1020
REGISTER = 0x1030
HANDLER = 0x1040
RUNON = 0x1050
START = 0x1052
DESC = 0x10A0
INTERP = 0x10B0
LABEL = 0x10C0
PAYLOAD = 0x10D0
BLOB = 0x10E0
G_SINK = DATA_RVA
TABLE = RDATA_RVA


def rel32(at_end, target):
    return struct.pack("<i", target - at_end)


def build_text():
    t = bytearray()
    labels = {}

    def at(rva):
        assert TEXT_RVA + len(t) <= rva, hex(rva)
        while TEXT_RVA + len(t) < rva:
            t.append(0xCC)

    def emit(b):
        t.extend(b)

    def here():
        return TEXT_RVA + len(t)

    at(PRE_A)
    emit(b"\x8d\x04\x49")                       # lea eax,[rcx+rcx*2]
    emit(b"\x83\xc0\x01")                       # add eax,1
    emit(b"\xc3")                               # ret
    at(PRE_B)
    emit(b"\x8d\x04\xcd\x00\x00\x00\x00")       # lea eax,[rcx*8]
    emit(b"\x29\xc8")                           # sub eax,ecx
    emit(b"\xc3")                               # ret
    at(STR16)
    emit("hr-HR".encode("utf-16-le") + b"\0\0")
    while here() < REGISTER:
        emit(b"\0")
    at(REGISTER)
    emit(b"\x48\x8d\x05" + rel32(here() + 7, HANDLER))         # lea rax,[handler]
    emit(b"\x48\x89\x05" + rel32(here() + 7, G_SINK))          # mov [g_sink],rax
    emit(b"\xc3")                                              # ret
    while here() < HANDLER:
        emit(b"\x90")
    emit(b"\x8d\x04\x89")                       # lea eax,[rcx+rcx*4]
    emit(b"\x8d\x04\x41")                       # lea eax,[rcx+rax*2]
    emit(b"\x83\xc0\xfc")                       # add eax,-4
    emit(b"\xc3")                               # ret
    at(RUNON)
    emit(b"\x31\xc0")                           # xor eax,eax (falls into _start)
    assert here() == START
    emit(b"\x48\x83\xec\x28")                   # sub rsp,0x28
    emit(b"\xe8" + rel32(here() + 5, REGISTER))                # call register
    emit(b"\x48\x8d\x0d" + rel32(here() + 7, STR16))           # lea rcx,[str16]
    emit(b"\x48\x8d\x15" + rel32(here() + 7, RUNON))           # lea rdx,[runon]
    emit(b"\x4c\x8d\x05" + rel32(here() + 7, HANDLER + 3))     # lea r8,[handler+3]
    emit(b"\x4c\x8d\x0d" + rel32(here() + 7, BLOB))           # lea r9,[blob]
    emit(b"\x48\x8d\x05" + rel32(here() + 7, DESC))           # lea rax,[desc]
    emit(b"\xb9\x05\x00\x00\x00")                              # mov ecx,5
    emit(b"\xff\x15" + rel32(here() + 6, G_SINK))              # call [g_sink]
    emit(b"\xff\x15" + rel32(here() + 6, TABLE))               # call [table]
    emit(b"\xe8" + rel32(here() + 5, INTERP))                   # call interp
    emit(b"\x48\x83\xc4\x28")                   # add rsp,0x28
    emit(b"\xc3")                               # ret
    labels["start_end"] = here()
    at(DESC)
    emit(bytes([0x01, 0x00, 0x00, 0x00, 0x2c, 0xc7, 0x00, 0x00, 0xc2, 0x00, 0x00]))
    at(INTERP)
    emit(b"\x48\x83\xec\x28")                   # sub rsp,0x28
    emit(b"\x48\x8d\x05" + rel32(here() + 7, LABEL))          # lea rax,[label]
    emit(b"\xff\xe0")                           # jmp rax
    at(LABEL)
    emit(b"\x48\x83\xc4\x28")                   # add rsp,0x28
    emit(b"\xc3")                               # ret
    at(PAYLOAD)
    labels["payload"] = here()
    emit(b"\x31\xc0\xc3")                       # xor eax,eax ; ret
    at(BLOB)
    emit("blob".encode("utf-16-le") + b"\0\0")
    return bytes(t), labels


def build():
    text, labels = build_text()
    rdata = b"".join(struct.pack("<Q", IMAGE_BASE + rva)
                     for rva in (PRE_A, PRE_B, STR16, labels["payload"]))
    pdata = struct.pack("<III", START, labels["start_end"], XDATA_RVA)
    xdata = bytes([0x01, 0x04, 0x01, 0x00, 0x04, 0x42, 0x00, 0x00])
    reloc = struct.pack("<II", RDATA_RVA, 8 + 2 * 4)
    for off in range(0, 32, 8):
        reloc += struct.pack("<H", (10 << 12) | off)
    data = bytes(8)

    nsec = 6
    opt_size = 240
    hdr_size = 0x40 + 4 + 20 + opt_size + 40 * nsec

    def align(n):
        return (n + FILE_ALIGN - 1) // FILE_ALIGN * FILE_ALIGN

    headers_sz = align(hdr_size)
    blobs = [(".text", text, TEXT_RVA, 0x60000020), (".rdata", rdata, RDATA_RVA, 0x40000040),
             (".pdata", pdata, PDATA_RVA, 0x40000040), (".xdata", xdata, XDATA_RVA, 0x40000040),
             (".reloc", reloc, RELOC_RVA, 0x42000040), (".data", data, DATA_RVA, 0xC0000040)]
    offs = []
    off = headers_sz
    for _, blob, _, _ in blobs:
        offs.append(off)
        off += align(len(blob))
    image_sz = DATA_RVA + SECT_ALIGN

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)
    b = bytearray(dos)
    b += b"PE\0\0"
    b += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)
    opt = bytearray()
    opt += struct.pack("<HBBIIIII", 0x20B, 14, 0, len(text), len(rdata) + len(data), 0, START, TEXT_RVA)
    opt += struct.pack("<Q", IMAGE_BASE)
    opt += struct.pack("<IIHHHHHHIIIIHHQQQQII",
                       SECT_ALIGN, FILE_ALIGN, 6, 0, 0, 0, 6, 0, 0,
                       image_sz, headers_sz, 0, 3, 0x8160,
                       0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    dirs = [(0, 0)] * 16
    dirs[3] = (PDATA_RVA, len(pdata))
    dirs[5] = (RELOC_RVA, len(reloc))
    for rva, sz in dirs:
        opt += struct.pack("<II", rva, sz)
    assert len(opt) == opt_size
    b += opt
    for (name, blob, rva, chars), roff in zip(blobs, offs):
        b += name.encode().ljust(8, b"\0") + struct.pack(
            "<IIIIIIHHI", len(blob), rva, align(len(blob)), roff, 0, 0, 0, 0, chars)
    b += bytes(headers_sz - len(b))
    for _, blob, _, _ in blobs:
        b += blob.ljust(align(len(blob)), b"\0")
    return bytes(b)


if __name__ == "__main__":
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pe_addrtaken_x86_64.exe")
    with open(out, "wb") as f:
        f.write(build())
    print(f"wrote {out} ({os.path.getsize(out)} bytes)")
