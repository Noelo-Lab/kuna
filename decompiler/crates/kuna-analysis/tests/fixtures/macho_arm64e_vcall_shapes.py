#!/usr/bin/env python3
"""Rebuild the `macho_arm64e_vcall_shapes` fixture from an `ld64.lld -arch arm64`
link of `macho_arm64e_vcall_shapes.s`.

The same conversion as `macho_arm64e_auth.py` (ld64.lld cannot link arm64e):
the header becomes `CPU_SUBTYPE_ARM64E | CPU_SUBTYPE_PTRAUTH_ABI` and every
`DYLD_CHAINED_PTR_64` rebase becomes a `DYLD_CHAINED_PTR_ARM64E_USERLAND24`
one, signed per `SIGNING` (symbol, offset -> key, address diversity,
diversifier); every other rebase stays plain.

    clang -target arm64-apple-macos11 -march=armv8.5-a -c macho_arm64e_vcall_shapes.s -o s.o
    ld64.lld -arch arm64 -platform_version macos 11.0 11.0 -fixup_chains \\
             -e _main -o s_arm64 s.o
    python3 macho_arm64e_vcall_shapes.py s_arm64 macho_arm64e_vcall_shapes
"""
import struct
import sys

from macho_arm64e_auth import CPU_SUBTYPE_ARM64E_PTRAUTH, PTR_64, ARM64E_USERLAND24, layout, load_commands

LC_SYMTAB = 0x2
KEY_IA, KEY_DA = 0, 2

SIGNING = {("_vtable_C", 0x10 + 8 * i): (KEY_IA, 1, 0x1111) for i in range(32)}
SIGNING.update({
    ("_vtable_C", 0x110): (KEY_IA, 1, 0x3232),
    ("_gslot", 0): (KEY_IA, 1, 0x6060),
    ("_cslot", 0): (KEY_IA, 0, 0x7070),
    ("_wslot", 0): (KEY_IA, 1, 0x6161),
    ("_swift_meta_prefix", 0): (KEY_IA, 1, 0xBBBF),
    ("_swift_meta", 0x40): (KEY_DA, 1, 0xAE86),
    ("_swift_meta", 0x50): (KEY_IA, 1, 0x5151),
    ("_swift_meta", 0x58): (KEY_IA, 1, 0x5252),
    ("_swift_meta", 0x60): (KEY_IA, 1, 0x5353),
})


def symbols(d):
    for cmd, off in load_commands(d):
        if cmd == LC_SYMTAB:
            symoff, nsyms, stroff = struct.unpack_from("<III", d, off + 8)
            for i in range(nsyms):
                strx, _t, _s, _desc, value = struct.unpack_from("<IBBHQ", d, symoff + 16 * i)
                name = bytes(d[stroff + strx:]).split(b"\0")[0].decode()
                yield name, value


def convert(data):
    d = bytearray(data)
    struct.pack_into("<I", d, 8, CPU_SUBTYPE_ARM64E_PTRAUTH)
    segs, _sects, (blob_off, _blob_size) = layout(d)
    base = min(v for v, _f, size in segs if size)
    at = dict(symbols(d))
    signing = {at[name] + off: sig for (name, off), sig in SIGNING.items()}

    def file_off(va):
        return next(f + va - v for v, f, size in segs if v <= va < v + size)

    starts = blob_off + struct.unpack_from("<I", d, blob_off + 4)[0]
    signed = 0
    for i in range(struct.unpack_from("<I", d, starts)[0]):
        seg_info = struct.unpack_from("<I", d, starts + 4 + 4 * i)[0]
        if not seg_info:
            continue
        p = starts + seg_info
        page_size, fmt, seg_offset = struct.unpack_from("<HHQ", d, p + 4)
        page_count = struct.unpack_from("<H", d, p + 20)[0]
        assert fmt == PTR_64, fmt
        struct.pack_into("<H", d, p + 6, ARM64E_USERLAND24)
        for page in range(page_count):
            start = struct.unpack_from("<H", d, p + 22 + 2 * page)[0]
            if start == 0xFFFF:
                continue
            va = base + seg_offset + page * page_size + start
            while True:
                off = file_off(va)
                raw = struct.unpack_from("<Q", d, off)[0]
                nxt = (raw >> 51) & 0xFFF
                assert nxt % 2 == 0 and not raw >> 63
                target = (raw & 0xF_FFFF_FFFF) - base
                word = target | ((nxt // 2) << 51)
                if va in signing:
                    key, addr_div, div = signing[va]
                    word = target | (div << 32) | (addr_div << 48) | (key << 49) | ((nxt // 2) << 51) | (1 << 63)
                    signed += 1
                struct.pack_into("<Q", d, off, word)
                if nxt == 0:
                    break
                va += nxt * 4
    assert signed == len(signing), (signed, len(signing))
    return bytes(d)


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    with open(dst, "wb") as f:
        f.write(convert(open(src, "rb").read()))
