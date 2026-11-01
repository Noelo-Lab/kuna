"""Keep one external nlist entry of a linked arm64 Mach-O dylib, so the export
trie names symbols the symbol table no longer has (a partially stripped image).

usage: python3 macho_trie_partial_arm64.py IN OUT KEEP
"""
import struct
import sys

src, dst, keep = sys.argv[1], sys.argv[2], sys.argv[3].encode()
b = bytearray(open(src, 'rb').read())
assert struct.unpack_from('<I', b, 0)[0] == 0xFEEDFACF
ncmds = struct.unpack_from('<I', b, 16)[0]
off, symtab, dysymtab = 32, None, None
for _ in range(ncmds):
    cmd, size = struct.unpack_from('<II', b, off)
    if cmd == 0x2:
        symtab = off
    elif cmd == 0xB:
        dysymtab = off
    off += size
symoff, nsyms, stroff, _ = struct.unpack_from('<IIII', b, symtab + 8)
assert struct.unpack_from('<I', b, dysymtab + 60)[0] == 0, 'indirect symbols present'
entries = [bytes(b[symoff + 16 * i:symoff + 16 * i + 16]) for i in range(nsyms)]
name = lambda e: bytes(b[stroff + struct.unpack_from('<I', e, 0)[0]:]).split(b'\0')[0]
kept = [e for e in entries if name(e) == keep]
assert len(kept) == 1, kept
b[symoff:symoff + 16 * nsyms] = kept[0] + bytes(16 * (nsyms - 1))
struct.pack_into('<I', b, symtab + 12, 1)
struct.pack_into('<IIIIII', b, dysymtab + 8, 0, 0, 0, 1, 1, 0)
open(dst, 'wb').write(b)
