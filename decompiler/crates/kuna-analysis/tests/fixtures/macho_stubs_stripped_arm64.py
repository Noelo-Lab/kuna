#!/usr/bin/env python3
"""Write macho_stubs_stripped_arm64: chkstkdarwin_macho_arm64 with the indirect-symbol
entries of its `__stubs` and `__got` sections overwritten with
INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS, as `strip` leaves them when it drops the
undefined nlist the entries indexed. The chained-fixups bind on the `__got` slot stays.

    python3 macho_stubs_stripped_arm64.py
"""
import struct, sys
from pathlib import Path

LC_DYSYMTAB = 0xB
LOCAL_ABS = 0x80000000 | 0x40000000

here = Path(__file__).resolve().parent
src = bytearray((here / "chkstkdarwin_macho_arm64").read_bytes())
ncmds = struct.unpack_from("<I", src, 16)[0]
off = 32
for _ in range(ncmds):
    cmd, size = struct.unpack_from("<II", src, off)
    if cmd == LC_DYSYMTAB:
        indirectsymoff, nindirectsyms = struct.unpack_from("<II", src, off + 56)
        for i in range(nindirectsyms):
            struct.pack_into("<I", src, indirectsymoff + 4 * i, LOCAL_ABS)
        break
    off += size
else:
    sys.exit("no LC_DYSYMTAB")
(here / "macho_stubs_stripped_arm64").write_bytes(src)
