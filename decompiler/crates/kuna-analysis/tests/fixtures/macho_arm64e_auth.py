#!/usr/bin/env python3
"""Rebuild the `macho_arm64e_auth` fixture from an `ld64.lld -arch arm64` link of
`macho_arm64e_auth.s`.

ld64.lld cannot link arm64e, so the arm64 image it writes is converted in
place: the header becomes `CPU_SUBTYPE_ARM64E | CPU_SUBTYPE_PTRAUTH_ABI`, and
every `DYLD_CHAINED_PTR_64` chain is re-encoded as
`DYLD_CHAINED_PTR_ARM64E_USERLAND24` (`mach-o/fixup-chains.h`). The chains keep
their slots; only the entry layout and the stride change (4 to 8 bytes). The
`__auth_ptr` bind and `__const[0..1]` become authenticated entries, the way
an arm64e link signs a function pointer:

    __const[0]    auth rebase -> _helper, key IA, address-diversified, 0x1234
    __const[1]    auth bind   -> _ext_func, key IA
    __const[2]    bind        -> _ext_data, addend 16
    __auth_ptr[0] auth bind   -> _ext_func, key IA
    __data[0]     rebase      -> _helper2

    clang -target arm64-apple-macos11 -march=armv8.5-a -c macho_arm64e_auth.s -o a.o
    ld64.lld -arch arm64 -platform_version macos 11.0 11.0 -fixup_chains \
             -undefined dynamic_lookup -e _main -o a_arm64 a.o
    python3 macho_arm64e_auth.py a_arm64 macho_arm64e_auth
"""
import struct
import sys

LC_SEGMENT_64 = 0x19
LC_DYLD_CHAINED_FIXUPS = 0x80000034
CPU_SUBTYPE_ARM64E_PTRAUTH = 0x80000002
PTR_64 = 2
ARM64E_USERLAND24 = 12
KEY_IA = 0


def load_commands(d):
    ncmds = struct.unpack_from("<I", d, 16)[0]
    off = 32
    for _ in range(ncmds):
        cmd, size = struct.unpack_from("<II", d, off)
        yield cmd, off
        off += size


def layout(d):
    segs, sects, fixups = [], [], None
    for cmd, off in load_commands(d):
        if cmd == LC_SEGMENT_64:
            vmaddr, _vmsize, fileoff, filesize = struct.unpack_from("<4Q", d, off + 24)
            nsects = struct.unpack_from("<I", d, off + 64)[0]
            segs.append((vmaddr, fileoff, filesize))
            for i in range(nsects):
                s = off + 72 + 80 * i
                name = bytes(d[s:s + 16]).rstrip(b"\0").decode()
                addr, size = struct.unpack_from("<2Q", d, s + 32)
                sects.append((name, addr, size))
        elif cmd == LC_DYLD_CHAINED_FIXUPS:
            fixups = struct.unpack_from("<II", d, off + 8)
    return segs, sects, fixups


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
            while True:
                at = file_off(va)
                raw = struct.unpack_from("<Q", d, at)[0]
                nxt = (raw >> 51) & 0xFFF
                assert nxt % 2 == 0
                struct.pack_into("<Q", d, at, encode(raw, nxt // 2, base, section(va), va))
                if nxt == 0:
                    break
                va += nxt * 4
    return bytes(d)


def encode(raw, nxt, base, section, va):
    if raw >> 63:
        ordinal, addend = raw & 0xFFFFFF, (raw >> 24) & 0xFF
        word = ordinal | (nxt << 51) | (1 << 62)
        if section == "__auth_ptr" or (section == "__const" and va & 0xF == 8):
            return word | (KEY_IA << 49) | (1 << 63)
        return word | (addend << 32)
    target = (raw & 0xF_FFFF_FFFF) - base
    if section == "__const":
        return target | (0x1234 << 32) | (1 << 48) | (KEY_IA << 49) | (nxt << 51) | (1 << 63)
    return target | (nxt << 51)


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    with open(dst, "wb") as f:
        f.write(convert(open(src, "rb").read()))
