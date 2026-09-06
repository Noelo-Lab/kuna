#!/usr/bin/env python3
"""Generate `noshdr_x86_64`: a minimal ELF64 PIE that carries **no section table**
at all (RE-need `zero-function-sizes-make`, round-4 crackme KataVM_L1).

`e_shoff`/`e_shnum`/`e_shstrndx` are zero, so every section-keyed reader sees an
empty world and only the three program headers say where anything lives:

    PT_LOAD  0x000..0x0e8  R    the headers themselves
    PT_LOAD  0x100..0x116  R+E  two functions
    PT_LOAD  0x120..0x130  R+W  data

    0x100  55                 push rbp          <- e_entry
    0x101  48 89 e5           mov  rbp,rsp
    0x104  e8 07 00 00 00     call 0x110
    0x109  5d                 pop  rbp
    0x10a  c3                 ret
    0x110  b8 2a 00 00 00     mov  eax,0x2a
    0x115  c3                 ret

The two entries make both clip arms visible: 0x100 stops at its neighbour (16
bytes) and 0x110 at the end of the executable segment (6). The non-executable
segments are what keeps the fallback honest -- a reader that took any load
segment would hand the data at 0x120 a body too.

    python3 noshdr_x86_64.py noshdr_x86_64
"""
import struct
import sys

CODE_VA, DATA_VA = 0x100, 0x120
CODE = bytes([
    0x55,                                # push rbp
    0x48, 0x89, 0xe5,                    # mov  rbp,rsp
    0xe8, 0x07, 0x00, 0x00, 0x00,        # call 0x110
    0x5d,                                # pop  rbp
    0xc3,                                # ret
    0x90, 0x90, 0x90, 0x90, 0x90,        # padding to 0x110
    0xb8, 0x2a, 0x00, 0x00, 0x00,        # mov  eax,0x2a
    0xc3,                                # ret
])
DATA = b"kuna sectionles"
EHSIZE, PHENTSIZE, PHNUM = 64, 56, 3


def phdr(vaddr: int, size: int, flags: int) -> bytes:
    # p_offset == p_vaddr keeps every segment congruent modulo p_align without
    # padding the file out to a page per segment.
    return struct.pack("<IIQQQQQQ", 1, flags, vaddr, vaddr, vaddr, size, size, 0x1000)


def build() -> bytes:
    image = bytearray(DATA_VA + len(DATA) + 1)
    image[CODE_VA:CODE_VA + len(CODE)] = CODE
    image[DATA_VA:DATA_VA + len(DATA)] = DATA
    image[0:EHSIZE] = struct.pack(
        "<16sHHIQQQIHHHHHH",
        b"\x7fELF\x02\x01\x01" + b"\x00" * 9,
        3, 62, 1,                         # ET_DYN, EM_X86_64, EV_CURRENT
        CODE_VA, EHSIZE, 0, 0,            # e_entry, e_phoff, e_shoff = 0, e_flags
        EHSIZE, PHENTSIZE, PHNUM,
        0, 0, 0,                          # e_shentsize/e_shnum/e_shstrndx = 0
    )
    headers = phdr(0, EHSIZE + PHENTSIZE * PHNUM, 4) \
        + phdr(CODE_VA, len(CODE), 5) \
        + phdr(DATA_VA, len(DATA) + 1, 6)
    image[EHSIZE:EHSIZE + len(headers)] = headers
    return bytes(image)


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "noshdr_x86_64"
    with open(out, "wb") as fh:
        fh.write(build())
