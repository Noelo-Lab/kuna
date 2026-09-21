#!/usr/bin/env python3
"""Rebuild the `macho_dup_main` fixture from `macho_imports`.

`macho_dup_main` is `macho_imports` with two symbols spelled the same: the
`_compute` entry's string index is repointed at `_main`'s, so `0x1000005a0` and
`0x1000005b0` both answer to `_main`. That is the shape two `static` functions
of the same name in different translation units produce in a linked image, and
it is the selector-ambiguity witness: the name identifies two entries and only
an address separates them, because a LINKED image has no section-qualified
selector to offer.

    python3 macho_dup_main.py macho_imports macho_dup_main

Nothing moves. One 4-byte `n_strx` field is rewritten in place, so every offset,
the symbol count and the file size are the source image's.
"""
import os
import struct
import sys

LC_SYMTAB = 0x2


def duplicate_main(data: bytes) -> bytes:
    d = bytearray(data)
    ncmds = struct.unpack_from("<I", d, 16)[0]
    off, symtab = 32, None
    for _ in range(ncmds):
        cmd, cmdsize = struct.unpack_from("<II", d, off)
        if cmd == LC_SYMTAB and symtab is None:
            symtab = off
        off += cmdsize
    if symtab is None:
        raise SystemExit("no LC_SYMTAB in the source image")
    symoff, nsyms, stroff, _strsize = struct.unpack_from("<IIII", d, symtab + 8)

    def name(j):
        n_strx = struct.unpack_from("<I", d, symoff + j * 16)[0]
        return bytes(d[stroff + n_strx: d.index(b"\0", stroff + n_strx)])

    main = next(j for j in range(nsyms) if name(j) == b"_main")
    other = next(j for j in range(nsyms) if name(j) == b"_compute")
    struct.pack_into("<I", d, symoff + other * 16,
                     struct.unpack_from("<I", d, symoff + main * 16)[0])
    return bytes(d)


if __name__ == "__main__":
    here = os.path.dirname(os.path.abspath(__file__))
    src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(here, "macho_imports")
    dst = sys.argv[2] if len(sys.argv) > 2 else os.path.join(here, "macho_dup_main")
    with open(dst, "wb") as f:
        f.write(duplicate_main(open(src, "rb").read()))
