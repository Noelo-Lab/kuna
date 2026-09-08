#!/usr/bin/env python3
"""Generate `dynseg_plt_x86_64`: a **section-table-free** ELF64 PIE that still
imports `puts` and `memcpy` through ordinary `.plt.sec`-shaped stubs (RE-need
`sectionless-elf-import-relocations`, round-4 crackme KataVM_L1).

`e_shoff`/`e_shnum`/`e_shstrndx` are zero, so `.dynsym`, `.dynstr`, `.rela.plt`
and `.plt.sec` are all invisible as sections and only `PT_DYNAMIC` says where
they are -- exactly how the run-time loader finds them, and exactly the state
KataVM_L1 ships in.  Everything the import naming needs is reachable *only*
through the dynamic tags:

    PT_LOAD    0x000..0x23c  R    headers, .dynamic, .dynsym, .dynstr, .rela.plt
    PT_LOAD    0x240..0x290  R+E  two PLT stubs + main
    PT_LOAD    0x300..0x328  R+W  the GOT
    PT_DYNAMIC 0x120..0x1b0       STRTAB/SYMTAB/SYMENT/STRSZ/PLTGOT/PLTREL/
                                  PLTRELSZ/JMPREL

    0x240  endbr64; bnd jmp *0x318(%rip)      <- puts@plt   -> JUMP_SLOT sym 1
    0x250  endbr64; bnd jmp *0x320(%rip)      <- memcpy@plt -> JUMP_SLOT sym 2
    0x260  main: memcpy(buf, "hi", 3); puts(buf); return 0   <- e_entry

The two stubs are 16 bytes apart with a *third* 16-byte-aligned run of code
right after them, so a reader that guessed a PLT extent from the executable
segment instead of correlating decoded GOT targets against the relocation slots
would name `main` as an import too.

    python3 dynseg_plt_x86_64.py dynseg_plt_x86_64
"""
import struct
import sys

DYNAMIC_VA = 0x120
DYNSYM_VA = 0x1b0
DYNSTR_VA = 0x1f8
RELA_VA = 0x208
STR_VA = 0x238
CODE_VA = 0x240
GOT_VA = 0x300

PUTS_SLOT = GOT_VA + 0x18
MEMCPY_SLOT = GOT_VA + 0x20

EHSIZE, PHENTSIZE, PHNUM = 64, 56, 4
R_X86_64_JUMP_SLOT = 7
DYNSTR = b"\0puts\0memcpy\0"


def stub(at: int, slot: int) -> bytes:
    """A CET `.plt.sec` entry: `endbr64; bnd jmp *slot(%rip)`, padded to 16."""
    disp = slot - (at + 11)  # rip after the 7-byte `f2 ff 25 <disp32>`
    return (b"\xf3\x0f\x1e\xfa\xf2\xff\x25" + struct.pack("<i", disp)
            + b"\x0f\x1f\x44\x00\x00")


def main_code(at: int) -> bytes:
    """`char b[8]; memcpy(b, "hi", 3); puts(b); return 0;`"""
    body = bytearray()
    body += b"\x48\x83\xec\x18"                      # sub  rsp,0x18
    body += b"\x48\x8d\x7c\x24\x08"                  # lea  rdi,[rsp+8]
    lea = at + len(body)
    body += b"\x48\x8d\x35" + struct.pack("<i", STR_VA - (lea + 7))  # lea rsi,[rip+"hi"]
    body += b"\xba\x03\x00\x00\x00"                  # mov  edx,3
    call = at + len(body)
    body += b"\xe8" + struct.pack("<i", (CODE_VA + 0x10) - (call + 5))  # call memcpy@plt
    body += b"\x48\x8d\x7c\x24\x08"                  # lea  rdi,[rsp+8]
    call = at + len(body)
    body += b"\xe8" + struct.pack("<i", CODE_VA - (call + 5))          # call puts@plt
    body += b"\x31\xc0"                              # xor  eax,eax
    body += b"\x48\x83\xc4\x18"                      # add  rsp,0x18
    body += b"\xc3"                                  # ret
    return bytes(body)


def phdr(p_type: int, vaddr: int, size: int, flags: int) -> bytes:
    # p_offset == p_vaddr keeps every segment congruent modulo p_align without
    # padding the file out to a page per segment.
    return struct.pack("<IIQQQQQQ", p_type, flags, vaddr, vaddr, vaddr, size, size, 0x1000)


def build() -> bytes:
    code = stub(CODE_VA, PUTS_SLOT) + stub(CODE_VA + 0x10, MEMCPY_SLOT)
    entry = CODE_VA + len(code)
    code += main_code(entry)

    image = bytearray(GOT_VA + 0x28)

    # `.dynsym`: the null symbol plus the two UND imports (st_name only; the
    # PLT resolution reads nothing else).
    for i, off in enumerate([0, 1, 6]):
        struct.pack_into("<IBBHQQ", image, DYNSYM_VA + i * 24, off, 0, 0, 0, 0, 0)
    image[DYNSTR_VA:DYNSTR_VA + len(DYNSTR)] = DYNSTR

    # `.rela.plt`: one JUMP_SLOT per import, naming its GOT slot.
    for i, (slot, sym) in enumerate([(PUTS_SLOT, 1), (MEMCPY_SLOT, 2)]):
        struct.pack_into("<QQq", image, RELA_VA + i * 24, slot,
                         (sym << 32) | R_X86_64_JUMP_SLOT, 0)

    image[STR_VA:STR_VA + 3] = b"hi\0"
    image[CODE_VA:CODE_VA + len(code)] = code

    dyn = [
        (5, DYNSTR_VA),                  # DT_STRTAB
        (6, DYNSYM_VA),                  # DT_SYMTAB
        (10, len(DYNSTR)),               # DT_STRSZ
        (11, 24),                        # DT_SYMENT
        (3, GOT_VA),                     # DT_PLTGOT
        (2, 48),                         # DT_PLTRELSZ
        (20, 7),                         # DT_PLTREL = DT_RELA
        (23, RELA_VA),                   # DT_JMPREL
        (0, 0),                          # DT_NULL
    ]
    for i, (tag, val) in enumerate(dyn):
        struct.pack_into("<QQ", image, DYNAMIC_VA + i * 16, tag, val)

    image[0:EHSIZE] = struct.pack(
        "<16sHHIQQQIHHHHHH",
        b"\x7fELF\x02\x01\x01" + b"\x00" * 9,
        3, 62, 1,                         # ET_DYN, EM_X86_64, EV_CURRENT
        entry, EHSIZE, 0, 0,              # e_entry, e_phoff, e_shoff = 0, e_flags
        EHSIZE, PHENTSIZE, PHNUM,
        0, 0, 0,                          # e_shentsize/e_shnum/e_shstrndx = 0
    )
    headers = (phdr(1, 0, STR_VA + 4, 4)
               + phdr(1, CODE_VA, len(code), 5)
               + phdr(1, GOT_VA, 0x28, 6)
               + phdr(2, DYNAMIC_VA, len(dyn) * 16, 6))
    image[EHSIZE:EHSIZE + len(headers)] = headers
    return bytes(image)


if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else "dynseg_plt_x86_64"
    with open(out, "wb") as fh:
        fh.write(build())
