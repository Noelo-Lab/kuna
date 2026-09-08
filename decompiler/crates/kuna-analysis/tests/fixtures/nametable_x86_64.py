#!/usr/bin/env python3
"""Generate `nametable_x86_64`: an ELF64 PIE whose `.rodata` holds a
length-prefixed name table -- printable identifiers separated by their own length
byte, with no NUL anywhere (RE-need `string-inventory-include-non`, round-8
challenge 673da52e9b533b4c22bd2eeb).

The reported binary is a 977264-byte Node SEA bundle whose `.rodata` opens with

    000b2020: 43c7 050c 6f75 742e 6a73 0673 7464 046f  C...out.js.std.o
    000b2030: 7312 5f30 7838 6563 3662 3312 5f30 7833  s._0x8ec6b3._0x3

-- `\\x0c` `out.js` `\\x06` `std` `\\x04` `os` `\\x12` `_0x8ec6b3`, each tag being
the name's length doubled. `strings -a` reads 635 of them; `kuna strings
--section .rodata --filter '_0x|out.js' --min-length 4` answered `count 0`,
because the matcher took only NUL-ended runs.

This is that shape at minimum size, plus one ordinary NUL-terminated literal as
the control -- `--termination nul` must still find that one and only that one:

    0x1000  55                       push rbp
    0x1001  48 89 e5                 mov  rbp,rsp
    0x1004  48 8d 3d 1b 10 00 00     lea  rdi,[rip+0x101b]  ; 0x2026, "Correct serial!"
    0x100b  5d                       pop  rbp
    0x100c  c3                       ret

    0x2000  \\x0cout.js\\x06std\\x12_0x8ec6b3\\x12_0x3f5c60\\x11
    0x2026  "Correct serial!\\x00"

    python3 nametable_x86_64.py nametable_x86_64
"""
import struct
import sys

EHSIZE, PHENTSIZE, PHNUM, SHENTSIZE = 64, 56, 2, 64
CODE_VADDR, RODATA_VADDR = 0x1000, 0x2000

NAMES = b"\x0cout.js\x06std\x12_0x8ec6b3\x12_0x3f5c60\x11"
LITERAL = b"Correct serial!\x00"
RODATA = NAMES + LITERAL
LITERAL_VADDR = RODATA_VADDR + len(NAMES)

CODE = bytes([
    0x55,                                       # push rbp
    0x48, 0x89, 0xe5,                           # mov  rbp,rsp
    0x48, 0x8d, 0x3d,                           # lea  rdi,[rip+disp32]
]) + struct.pack("<i", LITERAL_VADDR - (CODE_VADDR + 11)) + bytes([
    0x5d,                                       # pop rbp
    0xc3,                                       # ret
])

SECTIONS = [b"", b".text", b".rodata", b".shstrtab"]


def build() -> bytes:
    shstrtab = b"\x00".join(SECTIONS) + b"\x00"
    shstr_off = RODATA_VADDR + len(RODATA)
    shoff = (shstr_off + len(shstrtab) + 7) & ~7

    image = bytearray(shoff + SHENTSIZE * len(SECTIONS))
    image[CODE_VADDR:CODE_VADDR + len(CODE)] = CODE
    image[RODATA_VADDR:RODATA_VADDR + len(RODATA)] = RODATA
    image[shstr_off:shstr_off + len(shstrtab)] = shstrtab
    image[0:EHSIZE] = struct.pack(
        "<16sHHIQQQIHHHHHH",
        b"\x7fELF\x02\x01\x01" + b"\x00" * 9,
        3, 62, 1,                               # ET_DYN, EM_X86_64, EV_CURRENT
        CODE_VADDR, EHSIZE, shoff, 0,           # e_entry, e_phoff, e_shoff, e_flags
        EHSIZE, PHENTSIZE, PHNUM,               # e_ehsize, e_phentsize, e_phnum
        SHENTSIZE, len(SECTIONS), 3,            # e_shentsize, e_shnum, e_shstrndx
    )
    phdrs = b"".join(
        struct.pack("<IIQQQQQQ", 1, flags, vaddr, vaddr, vaddr, size, size, 0x1000)
        for vaddr, size, flags in (
            (CODE_VADDR, len(CODE), 5),         # PT_LOAD, PF_R | PF_X
            (RODATA_VADDR, len(RODATA), 4),     # PT_LOAD, PF_R
        )
    )
    image[EHSIZE:EHSIZE + len(phdrs)] = phdrs

    def shdr(name, sh_type, flags, addr, off, size, align):
        return struct.pack(
            "<IIQQQQIIQQ",
            shstrtab.index(name + b"\x00") if name else 0,
            sh_type, flags, addr, off, size, 0, 0, align, 0,
        )

    shdrs = b"".join([
        shdr(b"", 0, 0, 0, 0, 0, 0),                                        # SHT_NULL
        shdr(b".text", 1, 0x6, CODE_VADDR, CODE_VADDR, len(CODE), 16),      # ALLOC|EXEC
        shdr(b".rodata", 1, 0x2, RODATA_VADDR, RODATA_VADDR, len(RODATA), 1),
        shdr(b".shstrtab", 3, 0, 0, shstr_off, len(shstrtab), 1),
    ])
    image[shoff:shoff + len(shdrs)] = shdrs
    return bytes(image)


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "nametable_x86_64"
    with open(out, "wb") as fh:
        fh.write(build())
