#!/usr/bin/env python3
"""Rebuild the `macho_ptrslots_offset_arm64` fixture from `macho_ptrslots_arm64`.

`ld64.lld` writes arm64 chained fixups as `DYLD_CHAINED_PTR_64` (format 2), whose
rebase target is a full vmaddr. Apple's `ld64` writes `DYLD_CHAINED_PTR_64_OFFSET`
(format 6), whose target is an offset from the image base: GH #1333's
`audit_func_ptr` slot reads `10 04 00 00 00 00 00 00` on disk, the word `0x410`.
This rewrites each chained-starts segment's `pointer_format` from 2 to 6 and
each rebase word's target to `vmaddr - imagebase`, keeping its `next` and
`high8` fields, so the image has Apple's on-disk layout.

    python3 macho_ptrslots_offset_arm64.py macho_ptrslots_arm64 macho_ptrslots_offset_arm64

Nothing moves: only the format fields and the rebase words change, in place.
"""
import struct
import sys

LC_SEGMENT_64 = 0x19
LC_DYLD_CHAINED_FIXUPS = 0x80000034
PTR_64, PTR_64_OFFSET = 2, 6


def segments_and_fixups(d):
    ncmds = struct.unpack_from("<I", d, 16)[0]
    off, segs, fixups = 32, [], None
    for _ in range(ncmds):
        cmd, cmdsize = struct.unpack_from("<II", d, off)
        if cmd == LC_SEGMENT_64:
            vmaddr, vmsize, fileoff, filesize = struct.unpack_from("<QQQQ", d, off + 24)
            segs.append((vmaddr, fileoff, filesize))
        elif cmd == LC_DYLD_CHAINED_FIXUPS:
            fixups = struct.unpack_from("<II", d, off + 8)
        off += cmdsize
    return segs, fixups


def to_offset_format(data: bytes) -> bytes:
    d = bytearray(data)
    segs, fixups = segments_and_fixups(d)
    if fixups is None:
        raise SystemExit("no LC_DYLD_CHAINED_FIXUPS in the source image")
    imagebase = min(v for v, _, size in segs if size)
    blob = fixups[0]
    starts = blob + struct.unpack_from("<I", d, blob + 4)[0]
    seg_count = struct.unpack_from("<I", d, starts)[0]

    def fileoff(va):
        for vmaddr, off, size in segs:
            if vmaddr <= va < vmaddr + size:
                return off + va - vmaddr
        raise SystemExit(f"chain slot {va:#x} is in no segment")

    rewritten = 0
    for i in range(seg_count):
        info = struct.unpack_from("<I", d, starts + 4 + 4 * i)[0]
        if info == 0:
            continue
        seg = starts + info
        page_size, fmt = struct.unpack_from("<HH", d, seg + 4)
        segment_offset = struct.unpack_from("<Q", d, seg + 8)[0]
        page_count = struct.unpack_from("<H", d, seg + 20)[0]
        if fmt != PTR_64:
            raise SystemExit(f"segment {i} uses pointer format {fmt}, not DYLD_CHAINED_PTR_64")
        struct.pack_into("<H", d, seg + 6, PTR_64_OFFSET)
        for page in range(page_count):
            start = struct.unpack_from("<H", d, seg + 22 + 2 * page)[0]
            if start == 0xFFFF:
                continue
            va = imagebase + segment_offset + page * page_size + start
            while True:
                at = fileoff(va)
                raw = struct.unpack_from("<Q", d, at)[0]
                nxt = (raw >> 51) & 0xFFF
                if not raw >> 63:
                    target = raw & 0xF_FFFF_FFFF
                    raw = (raw & ~0xF_FFFF_FFFF) | ((target - imagebase) & 0xF_FFFF_FFFF)
                    struct.pack_into("<Q", d, at, raw)
                    rewritten += 1
                if nxt == 0:
                    break
                va += nxt * 4
    if rewritten == 0:
        raise SystemExit("no rebase to rewrite")
    return bytes(d)


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    with open(src, "rb") as fh:
        out = to_offset_format(fh.read())
    with open(dst, "wb") as fh:
        fh.write(out)
