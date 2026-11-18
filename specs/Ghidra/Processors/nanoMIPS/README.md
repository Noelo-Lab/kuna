# nanoMIPS

SLEIGH processor module for **nanoMIPS32**, the instruction set of MIPS I7200
cores. Language ids `nanoMIPS:LE:32:default` and `nanoMIPS:BE:32:default`; an
ELF with `e_machine` 249 selects one by its byte order.

nanoMIPS is a separate encoding, not a MIPS32 mode. An instruction is one, two
or three 16-bit units, each stored in the image's byte order:

```
unit 0   bits 15..10 = major opcode; bit 12 set means a 16-bit instruction
32-bit   unit 0 = instruction bits 31..16, unit 1 = bits 15..0
48-bit   unit 0 = opcode, register and function; unit 1 = low half and
         unit 2 = high half of a 32-bit immediate
```

Register numbering differs from MIPS32: `a0`..`a7` are 4..11, `t0`..`t3` are
12..15, and `t4`, `t5` are 2, 3. The compiler spec is the p32 ABI.

## What is decoded

The nanoMIPS32 integer instruction set in all three lengths, including the
compressed register encodings (`gpr3`, `gpr4`, their zero-reading store
variants and the `MOVEP` pairs), `SAVE` / `RESTORE` / `RESTORE.JRC`,
`LWM` / `SWM` and the unaligned forms, the PC-relative `LAPC` / `LWPC` /
`SWPC` / `ALUIPC`, the compact branches, `BRSC` / `BALRSC`, the CP0, TLB,
interrupt and exception-return instructions, `LL` / `SC` and the paired forms,
the MCU `ASET` / `ACLR` / `IRET`, and the EVA loads and stores.

`ROTX` lifts to arithmetic for the five operand sets that have assembler names
(`BITREVW`, `BITREVH`, `BITREVB`, `BYTEREVW`, `BYTEREVH`); any other operand
set is the `rotx` user operation.

## What is not

The floating-point (CP1), DSP, MT, CP2 and nanoMIPS64 instruction groups have
no constructors. Decoding stops at the first such instruction.

## How it was checked

- Listing: every instruction of the toolchain's little-endian soft-float
  `libc`, `libm` and `libgcc` (85,262 instructions), and of a hand-written file
  with one line per remaining form in both byte orders, against the GNU
  disassembler. The only
  differences are notational (`aluipc` prints the resolved page, coprocessor
  registers print as numbers, register runs print in full).
- Semantics: `decompiler/crates/kuna-sleigh/tests/nanomips_core.rs` runs a
  compiled workload and a hand-written routine through the p-code emulator and
  compares the results with a native build and with the manual's definitions.
