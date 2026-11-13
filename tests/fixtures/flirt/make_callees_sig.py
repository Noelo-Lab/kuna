#!/usr/bin/env python3
"""Write `callees.sig`, a FLIRT v11 signature for tests/fixtures/fid/callees/prog.

Modules: scale_a, scale_b (whole body as the pattern), and wrap_a / wrap_b,
which share one pattern (the call's rel32 is a wildcard) and differ only in
the name of the site at offset 7.
"""
import struct
from pathlib import Path

PROG = Path(__file__).resolve().parent.parent / "fid" / "callees" / "prog"

def m2(v):
    return bytes([v]) if v < 0x80 else bytes([0x80 | v >> 8, v & 0xFF])

def mb(v):
    if v < 0x80:
        return bytes([v])
    if v < 0x4000:
        return bytes([0x80 | v >> 8, v & 0xFF])
    return bytes([0xC0 | v >> 24, v >> 16 & 0xFF]) + struct.pack(">H", v & 0xFFFF)

def mask(pat):
    n = len(pat)
    bits = sum(1 << (n - 1 - i) for i, b in enumerate(pat) if b is None)
    if n < 0x10:
        return m2(bits)
    if n <= 0x20:
        return mb(bits)
    raise ValueError("pattern too long for this writer")

def module(name, length, sites, more):
    out = mb(length) + mb(0) + struct.pack(">HB", len(name), 0) + name.encode()
    out += bytes([(0x20 if sites else 0) | (8 if more else 0)])
    if sites:
        out += mb(len(sites))
        for off, sname in sites:
            out += mb(off) + struct.pack(">H", len(sname)) + sname.encode() + b"\0"
    return out

def child(pat, mods):
    leaf = m2(0) + bytes([0]) + struct.pack(">H", 0)
    for i, (name, sites) in enumerate(mods):
        leaf += module(name, len(pat), sites, i + 1 < len(mods))
    return m2(len(pat)) + mask(pat) + bytes(b for b in pat if b is not None) + leaf

text = PROG.read_bytes()[0x1000:0x1090]
def body(lo, hi, wild=()):
    return [None if i in wild else text[i] for i in range(lo, hi)]

children = [
    child(body(0x00, 0x14), [("scale_a", [])]),
    child(body(0x20, 0x38), [("scale_b", [])]),
    child(body(0x40, 0x54, wild=range(0x47, 0x4B)), [("wrap_a", [(7, "scale_a")]), ("wrap_b", [(7, "scale_b")])]),
]
tree = m2(len(children)) + b"".join(children)
lib = b"kuna callee fixture"
hdr = b"IDASGN" + bytes([11, 0]) + struct.pack("<IHHHHH", 0xFFFFFFFF, 0xFFFF, 0xFFFF, 0, 0, 0)
hdr += bytes(12) + bytes([len(lib)]) + struct.pack("<HIHHB", 0, 4, 32, 0, 0) + lib
(Path(__file__).resolve().parent / "callees.sig").write_bytes(hdr + tree)
