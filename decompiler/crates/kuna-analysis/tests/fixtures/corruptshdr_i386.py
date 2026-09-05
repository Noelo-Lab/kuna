#!/usr/bin/env python3
"""Generate `corruptshdr_i386`: a minimal ELF32 whose section table is garbage
while its program headers are intact (RE-need `corrupt-elf-section-table`,
round-3 challenge 5ab77f6633c5d40ad448cc64).

The reported binary carried `e_shoff=57005` (0xDEAD), `e_shnum=57007` and
`e_shstrndx=47806` in a 161156-byte file -- a section table 2 MB past EOF -- and
nine intact program headers. `readelf -h -l` warns and then prints the entry
point and both LOAD segments; `object::File::parse` rejects the whole file, so
every kuna surface exited 1. Those three header values are reproduced verbatim
here over two functions, `_start` and the leaf it calls:

    08048054  55                push ebp
    08048055  89 e5             mov  ebp,esp
    08048057  6a 23             push 0x23
    08048059  e8 05 00 00 00    call 0x8048063
    0804805e  c9                leave
    0804805f  c3                ret

    08048063  8b 44 24 04       mov  eax,[esp+4]
    08048067  83 c0 07          add  eax,7
    0804806a  c3                ret

    python3 corruptshdr_i386.py corruptshdr_i386
"""
import struct
import sys

BASE = 0x8048000
EHSIZE, PHENTSIZE = 52, 32
ENTRY = BASE + EHSIZE + PHENTSIZE          # 0x8048054, right after the headers

# The three corrupt values, copied from the reported image.
E_SHOFF, E_SHNUM, E_SHSTRNDX = 57005, 57007, 47806

CODE = bytes([
    0x55,                                 # push ebp
    0x89, 0xe5,                           # mov  ebp,esp
    0x6a, 0x23,                           # push 0x23
    0xe8, 0x05, 0x00, 0x00, 0x00,         # call 0x8048063
    0xc9,                                 # leave
    0xc3,                                 # ret
    0x90, 0x90, 0x90,                     # pad to the callee
    0x8b, 0x44, 0x24, 0x04,               # mov  eax,[esp+4]
    0x83, 0xc0, 0x07,                     # add  eax,7
    0xc3,                                 # ret
])


def build() -> bytes:
    off = ENTRY - BASE
    image = bytearray(off + len(CODE))
    image[off:off + len(CODE)] = CODE
    image[0:EHSIZE] = struct.pack(
        "<16sHHIIIIIHHHHHH",
        b"\x7fELF\x01\x01\x01" + b"\x00" * 9,
        2, 3, 1,                          # ET_EXEC, EM_386, EV_CURRENT
        ENTRY, EHSIZE, E_SHOFF, 0,        # e_entry, e_phoff, e_shoff, e_flags
        EHSIZE, PHENTSIZE, 1,             # e_ehsize, e_phentsize, e_phnum
        40, E_SHNUM, E_SHSTRNDX,          # e_shentsize, e_shnum, e_shstrndx
    )
    image[EHSIZE:EHSIZE + PHENTSIZE] = struct.pack(
        "<IIIIIIII",
        1, 0, BASE, BASE,                 # PT_LOAD, offset 0, vaddr/paddr
        len(image), len(image),
        5, 0x1000,                        # PF_R | PF_X
    )
    return bytes(image)


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "corruptshdr_i386"
    with open(out, "wb") as fh:
        fh.write(build())
