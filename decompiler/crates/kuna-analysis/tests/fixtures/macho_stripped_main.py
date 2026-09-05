#!/usr/bin/env python3
"""Rebuild the `macho_stripped_main` fixture from `macho_imports`.

`macho_stripped_main` is `macho_imports` with the defined-symbol names taken
away - what `strip` leaves behind on a Mach-O executable: no defined-symbol
names and no export trie, only `__mh_execute_header` and the undefined imports.
It is the `machomain` witness: `LC_MAIN` still states `entryoff = 0x5b0`, so the
program's `main` (0x1000005b0) is recoverable from the load command alone, while
nothing in the symbol stream names it any more. `macho_imports` itself is the
NEGATIVE case for the same pass - its `_main` is a real symbol and wins.

    python3 macho_stripped_main.py macho_imports macho_stripped_main

Every offset and the file size are preserved - the symbol table is rewritten in
place (survivors first, `nsyms` shortened), the export trie is emptied by
zeroing its LC_DYLD_INFO_ONLY (offset, size) pair, and LC_DYSYMTAB's index
windows and the indirect-symbol table are renumbered onto the new slots.
"""
import os
import struct
import sys

LC_SEGMENT_64 = 0x19
LC_SYMTAB = 0x2
LC_DYSYMTAB = 0xB
LC_DYLD_INFO_ONLY = 0x80000022
LC_DYLD_INFO = 0x22

KEEP = (b"__mh_execute_header", b"_printf", b"dyld_stub_binder")


def strip(data: bytes) -> bytes:
    d = bytearray(data)
    ncmds = struct.unpack_from("<I", d, 16)[0]
    off = 32
    cmds = {}
    for _ in range(ncmds):
        cmd, cmdsize = struct.unpack_from("<II", d, off)
        cmds.setdefault(cmd, off)
        off += cmdsize

    # 1. Empty the export trie (a stripped executable exports nothing kuna can
    #    read a defined-function name out of).
    for c in (LC_DYLD_INFO_ONLY, LC_DYLD_INFO):
        if c in cmds:
            struct.pack_into("<II", d, cmds[c] + 40, 0, 0)

    # 2. Rewrite the symbol table: survivors first, in (local, extdef, undef)
    #    order, then shorten `nsyms`.
    sym_off = cmds[LC_SYMTAB]
    symoff, nsyms, stroff, _strsize = struct.unpack_from("<IIII", d, sym_off + 8)
    entries = []
    for j in range(nsyms):
        e = symoff + j * 16
        n_strx, n_type, n_sect, n_desc, n_value = struct.unpack_from("<IBBHQ", d, e)
        name = bytes(d[stroff + n_strx: d.index(b"\0", stroff + n_strx)])
        entries.append((j, name, (n_strx, n_type, n_sect, n_desc, n_value)))
    survivors = [e for e in entries if e[1] in KEEP]
    # N_EXT|N_SECT defined symbols first, then the N_UNDF imports.
    survivors.sort(key=lambda e: (e[2][1] & 0xE) != 0xE)
    remap = {}
    for new, (old, _name, rec) in enumerate(survivors):
        struct.pack_into("<IBBHQ", d, symoff + new * 16, *rec)
        remap[old] = new
    for j in range(len(survivors), nsyms):
        struct.pack_into("<IBBHQ", d, symoff + j * 16, 0, 0, 0, 0, 0)
    struct.pack_into("<I", d, sym_off + 12, len(survivors))

    # 3. Renumber LC_DYSYMTAB's index windows and the indirect-symbol table.
    dy = cmds[LC_DYSYMTAB]
    ndef = sum(1 for e in survivors if (e[2][1] & 0xE) == 0xE)
    struct.pack_into(
        "<IIIIII", d, dy + 8,
        0, 0,                       # ilocalsym, nlocalsym
        0, ndef,                    # iextdefsym, nextdefsym
        ndef, len(survivors) - ndef,  # iundefsym, nundefsym
    )
    indirect_off, nindirect = struct.unpack_from("<II", d, dy + 8 + 48)
    for k in range(nindirect):
        at = indirect_off + 4 * k
        old = struct.unpack_from("<I", d, at)[0]
        struct.pack_into("<I", d, at, remap.get(old, old))
    return bytes(d)


if __name__ == "__main__":
    here = os.path.dirname(os.path.abspath(__file__))
    src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(here, "macho_imports")
    dst = sys.argv[2] if len(sys.argv) > 2 else os.path.join(here, "macho_stripped_main")
    out = strip(open(src, "rb").read())
    with open(dst, "wb") as f:
        f.write(out)
