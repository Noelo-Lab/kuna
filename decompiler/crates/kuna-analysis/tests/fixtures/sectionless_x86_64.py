#!/usr/bin/env python3
"""Generate `sectionless_x86_64`: an ELF64 PIE with NO section header table
whose one function forms a RIP-relative reference to a read-only string
(RE-need `sectionless-elf-loses-string`, round-4 challenge 605443e333c5d42c3d016f59).

The reported binary (`KataVM_L1`) is a 28682-byte PIE carrying `e_shoff=0` and
thirteen intact program headers. `kuna disassemble` decodes `LEA RDI,[0x6b22]`
and `kuna xrefs --from` recovers the CALL edges around it, but `kuna strings
--filter Correct` answers `xrefs_count 0, functions []`: the reference walk
classifies data operands against the section table, which is not there.

This is the same shape at minimum size -- one `PF_X` `PT_LOAD` holding the
function, one `PF_R` `PT_LOAD` holding the string:

    0x1000  55                       push rbp
    0x1001  48 89 e5                 mov  rbp,rsp
    0x1004  48 8d 3d f5 0f 00 00     lea  rdi,[rip+0xff5]   ; 0x2000
    0x100b  5d                       pop  rbp
    0x100c  c3                       ret

    0x2000  "\n[+] Correct!"

    python3 sectionless_x86_64.py sectionless_x86_64
"""
import struct
import sys

EHSIZE, PHENTSIZE, PHNUM = 64, 56, 2
CODE_VADDR, RODATA_VADDR = 0x1000, 0x2000

CODE = bytes([
    0x55,                                       # push rbp
    0x48, 0x89, 0xe5,                           # mov  rbp,rsp
    0x48, 0x8d, 0x3d, 0xf5, 0x0f, 0x00, 0x00,   # lea  rdi,[rip+0xff5]
    0x5d,                                       # pop  rbp
    0xc3,                                       # ret
])
RODATA = b"\n[+] Correct!\x00"


def build() -> bytes:
    image = bytearray(RODATA_VADDR + len(RODATA))
    image[CODE_VADDR:CODE_VADDR + len(CODE)] = CODE
    image[RODATA_VADDR:RODATA_VADDR + len(RODATA)] = RODATA
    image[0:EHSIZE] = struct.pack(
        "<16sHHIQQQIHHHHHH",
        b"\x7fELF\x02\x01\x01" + b"\x00" * 9,
        3, 62, 1,                               # ET_DYN, EM_X86_64, EV_CURRENT
        CODE_VADDR, EHSIZE, 0, 0,               # e_entry, e_phoff, e_shoff=0, e_flags
        EHSIZE, PHENTSIZE, PHNUM,               # e_ehsize, e_phentsize, e_phnum
        64, 0, 0,                               # e_shentsize, e_shnum=0, e_shstrndx=0
    )
    phdrs = b"".join(
        struct.pack("<IIQQQQQQ", 1, flags, vaddr, vaddr, vaddr, size, size, 0x1000)
        for vaddr, size, flags in (
            (CODE_VADDR, len(CODE), 5),         # PT_LOAD, PF_R | PF_X
            (RODATA_VADDR, len(RODATA), 4),     # PT_LOAD, PF_R
        )
    )
    image[EHSIZE:EHSIZE + len(phdrs)] = phdrs
    return bytes(image)


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "sectionless_x86_64"
    with open(out, "wb") as fh:
        fh.write(build())
