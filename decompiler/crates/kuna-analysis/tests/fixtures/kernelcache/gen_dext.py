#!/usr/bin/env python3
"""Build dext_osclassinfo.macho: a tiny arm64 DriverKit-style executable for
the `iokitclasses` DriverKit class-record pass.

__DATA_CONST,__osclassinfo holds one pointer to an OSClassLoadInformation
record (0xa0 bytes) for class MyDriver (superclass IOService, 0x40 bytes):
its description is an OSClassDescription (0x138 bytes) in __TEXT,__const, its
metaPointer the global gMyDriverMetaClass in __DATA, its New an unnamed
function. Only get_meta (returns gMyDriverMetaClass) and make (tail-calls
New) have symbols. Pointers are plain absolute words.

Run from this directory: python3 gen_dext.py
"""
import struct

B = 0x100000000
TEXT_CODE, TEXT_CONST, CONST, OSCLASSINFO, DATA, LINKEDIT = 0x1000, 0x2000, 0x4000, 0x4100, 0x8000, 0xC000

def u32(*v): return struct.pack("<" + "I" * len(v), *v)
def u64(*v): return struct.pack("<" + "Q" * len(v), *v)
def adrp(rd, pc, target):
    delta = ((B + target) >> 12) - ((B + pc) >> 12)
    return 0x90000000 | ((delta & 3) << 29) | (((delta >> 2) & 0x7FFFF) << 5) | rd

NEW, GET_META, MAKE = TEXT_CODE, TEXT_CODE + 0x10, TEXT_CODE + 0x20
code = {
    NEW: [0x52800000, 0xD65F03C0],
    GET_META: [adrp(8, GET_META, DATA), 0xF9400100 | ((DATA % 0x1000) // 8 << 10), 0xD65F03C0],
    MAKE: [0x14000000 | (((NEW - MAKE) // 4) & 0x3FFFFFF)],
}

desc = bytearray(0x138)
desc[0:4] = u32(0x138)
desc[4:4 + 8] = b"MyDriver"
desc[100:100 + 9] = b"IOService"
info = u64(B + TEXT_CONST, B + DATA) + u32(1, 0x40) + u64(*[0] * 8) + u64(B + NEW) + u64(*[0] * 8)
assert len(info) == 0xA0

strtab = b"\0_get_meta\0_make\0"
syms = u32(1) + bytes([0x0F, 1]) + struct.pack("<H", 0) + u64(B + GET_META)
syms += u32(11) + bytes([0x0F, 1]) + struct.pack("<H", 0) + u64(B + MAKE)
linkedit = syms + strtab

def section(seg, name, addr, size, flags=0):
    return name.ljust(16, b"\0") + seg.ljust(16, b"\0") + u64(B + addr, size) + u32(addr, 0, 0, 0, flags, 0, 0, 0)
def segment(name, addr, size, sections=(), prot=5, fileoff=None):
    off = addr if fileoff is None else fileoff
    body = name.ljust(16, b"\0") + u64(B + addr, size, off, size) + u32(prot, prot, len(sections), 0) + b"".join(sections)
    return u32(0x19, 8 + len(body)) + body

cmds = [
    segment(b"__TEXT", 0, 0x4000, [section(b"__TEXT", b"__text", TEXT_CODE, 0x30, 0x80000400),
                                   section(b"__TEXT", b"__const", TEXT_CONST, 0x138)]),
    segment(b"__DATA_CONST", CONST, 0x4000, [section(b"__DATA_CONST", b"__const", CONST, 0xA0),
                                              section(b"__DATA_CONST", b"__osclassinfo", OSCLASSINFO, 8)], prot=1),
    segment(b"__DATA", DATA, 0x4000, [section(b"__DATA", b"__data", DATA, 8)], prot=3),
    segment(b"__LINKEDIT", LINKEDIT, len(linkedit), prot=1),
    u32(0x2, 24, LINKEDIT, 2, LINKEDIT + len(syms), len(strtab)),
]
image = bytearray(LINKEDIT + len(linkedit))
def place(off, data): image[off:off + len(data)] = data
place(0, u32(0xFEEDFACF, 0x0100000C, 0, 0x2, len(cmds), sum(map(len, cmds)), 0x200085, 0) + b"".join(cmds))
for addr, words in code.items():
    place(addr, u32(*words))
place(TEXT_CONST, bytes(desc))
place(CONST, info)
place(OSCLASSINFO, u64(B + CONST))
place(LINKEDIT, linkedit)
open("dext_osclassinfo.macho", "wb").write(image)
