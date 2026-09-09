#!/usr/bin/env python3
"""Generate `armdiscseed_le32` — a stripped non-PIE ARM32 ELF whose only path to
the program body runs through `main`.

The shape of the witness (a stripped ARM crackme): `_start` hands `main` to
`__libc_start_main` in a PC-relative literal pool word, and `main` is the only
caller of everything else. No symbol table, no `.eh_frame`, no `INIT_ARRAY`, so
`collect_entries` recovers `_start` and nothing past it — `main` comes only from
the `armlibcmain` oracle, and the validator `main` calls comes only from
following `main`'s call graph.

No cross toolchain is needed; the ELF is assembled here byte by byte and the A32
bodies are hand-encoded. Regenerate with:

    python3 armdiscseed_le32.py

Layout (one RWX PT_LOAD at 0x10000, file offset == vma - 0x10000):

  .dynsym  @0x100a0   null + UND `__libc_start_main`
  .dynstr  @0x100c0
  .rel.plt @0x100e0   R_ARM_JUMP_SLOT on the GOT slot
  .plt     @0x10150   SHF_EXECINSTR, one decodable stub
  .text    @0x10160   SHF_EXECINSTR
      0x10160  _start:    ldr r0,[pc,#8]   -> the pool word at 0x10170 (= main)
      0x10164             bl 0x10150       -> `__libc_start_main`
      0x10168             bx lr
      0x10170             .word 0x10180    <- `main`, named by no other oracle
      0x10180  main:      stmdb sp!,{r11,lr}
      0x10184             bl 0x10190       <- the validator, named by NOTHING
      0x10188             mov r0,#0
      0x1018c             ldmia sp!,{r11,pc}
      0x10190  validator: stmdb sp!,{r11,lr}
      0x10194             mov r0,#0x2a
      0x10198             ldmia sp!,{r11,pc}
  .got.plt @0x10200   SHF_WRITE
  .dynamic @0x10210
"""
import struct, os

BASE = 0x10000
PLT_VMA = 0x10150
TEXT_VMA = 0x10160
START_VMA = TEXT_VMA
POOL_VMA = 0x10170
MAIN_VMA = 0x10180
VALIDATOR_VMA = 0x10190
TEXT_END = 0x1019C
GOTPLT_VMA = 0x10200
DYNSYM_VMA = 0x100A0
DYNSTR_VMA = 0x100C0
RELPLT_VMA = 0x100E0
DYNAMIC_VMA = 0x10210

R_ARM_JUMP_SLOT = 22
SHT_PROGBITS, SHT_SYMTAB, SHT_STRTAB, SHT_DYNAMIC = 1, 2, 3, 6
SHT_REL, SHT_DYNSYM = 9, 11
SHF_WRITE, SHF_ALLOC, SHF_EXECINSTR = 0x1, 0x2, 0x4


def bl(at, target):
    """`bl <target>` (A32); the ARM PC is two instructions ahead."""
    off = (target - (at + 8)) >> 2
    return 0xEB000000 | (off & 0x00FFFFFF)


def ldr_pc_r0(at, target):
    """`ldr r0,[pc,#imm]` (A32)."""
    imm = target - (at + 8)
    assert 0 <= imm < 0x1000, hex(imm)
    return 0xE59F0000 | imm


def build_text():
    words = {
        0x10160: ldr_pc_r0(0x10160, POOL_VMA),
        0x10164: bl(0x10164, PLT_VMA),
        0x10168: 0xE12FFF1E,  # bx lr
        0x1016C: 0x00000000,
        0x10170: MAIN_VMA,
        0x10174: 0x00000000,
        0x10178: 0x00000000,
        0x1017C: 0x00000000,
        0x10180: 0xE92D4800,  # stmdb sp!,{r11,lr}
        0x10184: bl(0x10184, VALIDATOR_VMA),
        0x10188: 0xE3A00000,  # mov r0,#0
        0x1018C: 0xE8BD8800,  # ldmia sp!,{r11,pc}
        0x10190: 0xE92D4800,  # stmdb sp!,{r11,lr}
        0x10194: 0xE3A0002A,  # mov r0,#0x2a
        0x10198: 0xE8BD8800,  # ldmia sp!,{r11,pc}
    }
    return b"".join(struct.pack("<I", words[a]) for a in range(TEXT_VMA, TEXT_END, 4))


def build_plt():
    # `add ip,pc,#0 ; add ip,ip,#0 ; ldr pc,[ip,#0xa8]!` -> GOT slot 0x10200
    # (the ARM pipeline puts pc at entry+8, so 0x10158 + 0xa8 == 0x10200).
    assert PLT_VMA + 8 + 0xA8 == GOTPLT_VMA
    return struct.pack("<III", 0xE28FC000, 0xE28CC000, 0xE5BCF0A8)


def main():
    image = bytearray(0x400)

    def put(vma, blob):
        off = vma - BASE
        image[off:off + len(blob)] = blob

    dynstr = b"\0__libc_start_main\0"
    put(DYNSTR_VMA, dynstr)
    # .dynsym: the null entry, then the UND import at index 1.
    put(DYNSYM_VMA, struct.pack("<IIIBBH", 0, 0, 0, 0, 0, 0))
    put(DYNSYM_VMA + 16, struct.pack("<IIIBBH", 1, 0, 0, (1 << 4) | 2, 0, 0))
    put(RELPLT_VMA, struct.pack("<II", GOTPLT_VMA, (1 << 8) | R_ARM_JUMP_SLOT))
    put(PLT_VMA, build_plt())
    put(TEXT_VMA, build_text())
    put(GOTPLT_VMA, struct.pack("<I", PLT_VMA))

    dyn = [
        (5, DYNSTR_VMA),          # DT_STRTAB
        (6, DYNSYM_VMA),          # DT_SYMTAB
        (10, len(dynstr)),        # DT_STRSZ
        (11, 16),                 # DT_SYMENT
        (3, GOTPLT_VMA),          # DT_PLTGOT
        (2, 8),                   # DT_PLTRELSZ
        (20, 17),                 # DT_PLTREL = DT_REL
        (23, RELPLT_VMA),         # DT_JMPREL
        (0, 0),                   # DT_NULL
    ]
    dyn_blob = b"".join(struct.pack("<Ii", t, v) for t, v in dyn)
    put(DYNAMIC_VMA, dyn_blob)
    alloc_end = DYNAMIC_VMA - BASE + len(dyn_blob)
    image = image[:alloc_end]

    names = [b"", b".dynsym", b".dynstr", b".rel.plt", b".plt", b".text",
             b".got.plt", b".dynamic", b".shstrtab"]
    shstr = bytearray()
    name_off = {}
    for n in names:
        name_off[n] = len(shstr)
        shstr.extend(n + b"\0")

    shstr_off = len(image)
    blob = bytearray(image) + shstr
    while len(blob) % 4:
        blob.append(0)
    shoff = len(blob)

    # (name, type, flags, vma, size, link, entsize)
    sections = [
        (b"", 0, 0, 0, 0, 0, 0),
        (b".dynsym", SHT_DYNSYM, SHF_ALLOC, DYNSYM_VMA, 32, 2, 16),
        (b".dynstr", SHT_STRTAB, SHF_ALLOC, DYNSTR_VMA, len(dynstr), 0, 0),
        (b".rel.plt", SHT_REL, SHF_ALLOC, RELPLT_VMA, 8, 1, 8),
        (b".plt", SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR, PLT_VMA, 12, 0, 0),
        (b".text", SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA,
         TEXT_END - TEXT_VMA, 0, 0),
        (b".got.plt", SHT_PROGBITS, SHF_ALLOC | SHF_WRITE, GOTPLT_VMA, 4, 0, 0),
        (b".dynamic", SHT_DYNAMIC, SHF_ALLOC | SHF_WRITE, DYNAMIC_VMA,
         len(dyn_blob), 2, 8),
        (b".shstrtab", SHT_STRTAB, 0, 0, len(shstr), 0, 0),
    ]
    shdrs = bytearray()
    for i, (name, typ, flags, vma, size, link, entsize) in enumerate(sections):
        off = 0 if i == 0 else (shstr_off if name == b".shstrtab" else vma - BASE)
        shdrs.extend(struct.pack(
            "<10I", name_off[name], typ, flags, vma, off, size, link, 0,
            4 if vma else 1, entsize))
    blob.extend(shdrs)

    phdrs = b"".join([
        # PT_LOAD, the whole allocated image, R|W|X
        struct.pack("<8I", 1, 0, BASE, BASE, alloc_end, alloc_end, 0x7, 0x1000),
        # PT_DYNAMIC
        struct.pack("<8I", 2, DYNAMIC_VMA - BASE, DYNAMIC_VMA, DYNAMIC_VMA,
                    len(dyn_blob), len(dyn_blob), 0x6, 4),
    ])
    ehdr = struct.pack(
        "<4s5B7x2H5I6H",
        b"\x7fELF", 1, 1, 1, 0, 0,          # 32-bit, LE, v1, SYSV
        2, 40,                                # ET_EXEC, EM_ARM
        1, START_VMA, 52, shoff, 0x5000200,   # v1, entry, phoff, shoff, flags
        52, 32, len(phdrs) // 32, 40, len(sections), len(sections) - 1)
    blob[0:52] = ehdr
    blob[52:52 + len(phdrs)] = phdrs

    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "armdiscseed_le32")
    with open(out, "wb") as f:
        f.write(bytes(blob))
    print("wrote %s (%d bytes)" % (out, len(blob)))


if __name__ == "__main__":
    main()
