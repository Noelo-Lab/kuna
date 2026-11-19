#!/usr/bin/env python3
"""Rebuild the `macho_arm64e_vcall` fixture from an `ld64.lld -arch arm64` link of
`macho_arm64e_vcall.s`.

The same conversion as `macho_arm64e_auth.py` (ld64.lld cannot link arm64e):
the header becomes `CPU_SUBTYPE_ARM64E | CPU_SUBTYPE_PTRAUTH_ABI` and every
`DYLD_CHAINED_PTR_64` rebase becomes a `DYLD_CHAINED_PTR_ARM64E_USERLAND24`
one. Each `__const` rebase is signed the way arm64e signs a vtable slot: key
IA, address-diversified, and a per-slot diversifier, slot 0 `0x0f0f` (`f`)
and slot 1 `0x0a0a` (`g`). A slot is its position in a run of consecutive
fixups.

    clang -target arm64-apple-macos11 -march=armv8.5-a -c macho_arm64e_vcall.s -o v.o
    ld64.lld -arch arm64 -platform_version macos 11.0 11.0 -fixup_chains \\
             -e _main -o v_arm64 v.o
    python3 macho_arm64e_vcall.py v_arm64 macho_arm64e_vcall
"""
import struct
import sys

from macho_arm64e_auth import CPU_SUBTYPE_ARM64E_PTRAUTH, PTR_64, ARM64E_USERLAND24, layout

SLOT_DIVERSITY = [0x0F0F, 0x0A0A]


def convert(data):
    d = bytearray(data)
    struct.pack_into("<I", d, 8, CPU_SUBTYPE_ARM64E_PTRAUTH)
    segs, sects, (blob_off, _blob_size) = layout(d)
    base = min(v for v, _f, size in segs if size)

    def file_off(va):
        return next(f + va - v for v, f, size in segs if v <= va < v + size)

    def section(va):
        return next(name for name, a, size in sects if a <= va < a + size)

    starts = blob_off + struct.unpack_from("<I", d, blob_off + 4)[0]
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
            run_start, prev = va, None
            while True:
                at = file_off(va)
                raw = struct.unpack_from("<Q", d, at)[0]
                nxt = (raw >> 51) & 0xFFF
                assert nxt % 2 == 0 and not raw >> 63
                if prev is None or va != prev + 8:
                    run_start = va
                target = (raw & 0xF_FFFF_FFFF) - base
                word = target | ((nxt // 2) << 51)
                if section(va) == "__const":
                    div = SLOT_DIVERSITY[(va - run_start) // 8]
                    word = target | (div << 32) | (1 << 48) | ((nxt // 2) << 51) | (1 << 63)
                struct.pack_into("<Q", d, at, word)
                if nxt == 0:
                    break
                prev = va
                va += nxt * 4
    return bytes(d)


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    with open(dst, "wb") as f:
        f.write(convert(open(src, "rb").read()))
