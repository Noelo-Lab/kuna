# Cortus APS3

SLEIGH processor module for the **Cortus APS3**, the 32-bit RISC core inside the
Atmel/Microchip ATWINC1500 Wi-Fi module. Language id `Cortus:LE:32:APS3`.

There is no public opcode table for this core: Cortus never released one and the
GNU binutils port is licensee-gated. **The instruction set here was recovered
empirically from firmware**, so read the confidence tag on a constructor before
leaning on its mnemonic. Instruction *lengths* and *operand formats* are proven
for all 256 opcodes, so linear sweep and control flow stay correct even where a
mnemonic is a guess.

## The encoding, in three rules

```
len   = 4 if (opcode & 0x08) else 2      opcode = insn[7:0]
class = opcode & 0x07                    rs     = insn[11:8]
store = load + 0x81, and bit 7 = .cc     rd     = insn[15:12]
```

`r0` reads as constant zero, so `[r0 + off]` is an absolute address. `r1` is the
stack pointer and `r15` the link register: `call` writes it, and `ret` is
literally `jmp [r15]`. Arguments start in `r2` and return in `r2`; `r8`..`r14`
are callee-saved, measured in `aps3.cspec`.

Two columns use an opcode narrower than a byte. The direct `call` is a **4-bit**
opcode (low nibble `0xB`) carrying a 17-bit halfword-scaled index **into the
PC's own 256 KiB window**: 17 bits cannot hold a 32-bit address, and an image
based at `0x60000000` resolves every one of its call targets inside its own
extent under that rule and none under a fixed `0x80000000` base. The
shift-immediates are a **7-bit** opcode (`op7 = insn[6:0]`, count in
`insn[11:7]`), so bit 7 of the opcode *byte* carries `imm5[0]`.

The `0x_3` column is the extend/shift column and its high half is the ordinary
`.cc` half, not a separate family of tests: `0x13`/`0x23`/`0x33`/`0x43` are
`sextb`/`sexth`/`extb`/`exth` and `0x93`/`0xa3`/`0xb3`/`0xc3` are the same four
setting flags. They read as tests because `rd` is written to `r0`, which discards
it — which is what 75 of 128 `0x93` sites and 33 of 48 `0xa3` sites do, while
`0xb3` never does.

## Unknown opcodes are values, not `unimpl`

An opcode whose length and operand format are established but whose operation is
not calls `aps3_unknown_op`/`aps3_unknown_op_imm` and assigns the result to `rd`.
`unimpl` is the usual placeholder, but it aborts the enclosing basic block and so
costs the whole function; an opaque value keeps control flow intact and lets
every other statement decompile. On a 147 KiB ATWINC1500 image that is the
difference between 24 poisoned functions and none.

## Loading a raw image

The core has no object format of its own, so images arrive headerless:

```sh
kuna functions fw.bin --raw-image --target Cortus:LE:32:APS3:default \
    --base 0x80000000 --entry 0x8000010c --json
```

`--base` is the virtual address of file offset 0. Firmware for this part puts its
reset vector in the first word, so the entry point is usually
`*(uint32_t *)&file[0]` and the base is that word rounded down to the region
start. Finding the rest of the functions is `rawdiscover`'s job
(`docs/spec/01-program-prep.md` §1.6).
