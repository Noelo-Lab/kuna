#!/usr/bin/env python3
"""Rebuild the `macho_rustlike_symbol` fixture from `macho_imports`.

`macho_rustlike_symbol` is `macho_imports` with its C helper renamed `_compute`
-> `_Runtime`: an ordinary C program carrying one ordinary C function whose name
begins with `R`. Mach-O prepends a platform underscore to every symbol, so that
name is spelled `_Runtime` in the image and shares its first two bytes with the
Rust v0 mangling scheme (`_R<path>`) without being one -- `untime` is not a v0
path. It is the source-language witness: the whole program must still read as C
(`Compiler::Clang` from `LC_BUILD_VERSION`), not as rustc.

    python3 macho_rustlike_symbol.py macho_imports macho_rustlike_symbol

The two names are the same length, so this is a pure string-table edit: every
offset, every size and the file length are unchanged, and no symbol-table,
export-trie or indirect-table index moves.
"""
import os
import struct
import sys

LC_SYMTAB = 0x2

OLD = b"_compute\0"
NEW = b"_Runtime\0"


def rename(data: bytes) -> bytes:
    d = bytearray(data)
    ncmds = struct.unpack_from("<I", d, 16)[0]
    off = 32
    symtab = None
    for _ in range(ncmds):
        cmd, cmdsize = struct.unpack_from("<II", d, off)
        if cmd == LC_SYMTAB and symtab is None:
            symtab = off
        off += cmdsize
    if symtab is None:
        raise SystemExit("no LC_SYMTAB")

    stroff, strsize = struct.unpack_from("<II", d, symtab + 16)
    at = d.index(OLD, stroff, stroff + strsize)
    assert len(NEW) == len(OLD)
    d[at:at + len(NEW)] = NEW
    return bytes(d)


if __name__ == "__main__":
    here = os.path.dirname(os.path.abspath(__file__))
    src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(here, "macho_imports")
    dst = sys.argv[2] if len(sys.argv) > 2 else os.path.join(here, "macho_rustlike_symbol")
    with open(dst, "wb") as f:
        f.write(rename(open(src, "rb").read()))
