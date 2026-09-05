#!/usr/bin/env python3
"""Generate `overlapbranch_i386`: a minimal ELF32 carrying the x86 anti-disassembly
overlap the `overlapbranch` option resolves (RE-need
`overlapping-anti-disassembly-sequence`, round-2 crackme pcme0).

A `75 01` short JNZ hops over a junk `e8` lead byte, so a follower that decodes the
fall-through first sees one bogus 5-byte CALL that swallows the branch target:

    0804881c  8b 44 24 04     mov  eax,[esp+4]
    08048820  3c ff           cmp  al,0xff
    08048822  75 01           jne  0x8048825      ; INSIDE the next instruction
    08048824  e8              (junk lead byte)
    08048825  83 c0 07        add  eax,7          ; the real stream
    08048828  03 00           add  eax,[eax]
    0804882a  05 00 dd 8d d9  add  eax,0xd98ddd00
    0804882f  c3              ret

Decoded from 0x8048824 instead, the same bytes read as `call 0xb0c48ac` (outside the
image) followed by `add [0xd98ddd00],al` -- the out-of-image `sub_` call and the
bogus `dat_` global the tester reported, in 20 bytes.

    python3 overlapbranch_i386.py overlapbranch_i386
"""
import struct
import sys

BASE = 0x8048000
ENTRY = 0x804881C
CODE = bytes([
    0x8b, 0x44, 0x24, 0x04,              # mov  eax,[esp+4]
    0x3c, 0xff,                          # cmp  al,0xff
    0x75, 0x01,                          # jne  0x8048825
    0xe8,                                # junk lead byte
    0x83, 0xc0, 0x07,                    # add  eax,7
    0x03, 0x00,                          # add  eax,[eax]
    0x05, 0x00, 0xdd, 0x8d, 0xd9,        # add  eax,0xd98ddd00
    0xc3,                                # ret
])

EHSIZE, PHENTSIZE = 52, 32


def build() -> bytes:
    off = ENTRY - BASE
    image = bytearray(off + len(CODE))
    image[off:off + len(CODE)] = CODE
    ehdr = struct.pack(
        "<16sHHIIIIIHHHHHH",
        b"\x7fELF\x01\x01\x01" + b"\x00" * 9,
        2, 3, 1,                          # ET_EXEC, EM_386, EV_CURRENT
        ENTRY, EHSIZE, 0, 0,
        EHSIZE, PHENTSIZE, 1, 40, 0, 0,
    )
    phdr = struct.pack(
        "<IIIIIIII",
        1, 0, BASE, BASE,                 # PT_LOAD, offset 0, vaddr/paddr
        len(image), len(image),
        5, 0x1000,                        # PF_R | PF_X
    )
    image[0:len(ehdr)] = ehdr
    image[EHSIZE:EHSIZE + len(phdr)] = phdr
    return bytes(image)


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "overlapbranch_i386"
    with open(out, "wb") as fh:
        fh.write(build())
