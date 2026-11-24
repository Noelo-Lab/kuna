# diffverify — Tier L results (the lift against Unicorn)

Before naming a cause for any difference here, follow `triage.md`: a difference against
Unicorn can be the spec's, Unicorn's, kuna-verify's evaluator's, the driver's or the
architecture's own.

Tier L checks the machine side of Tier C: one instruction's SLEIGH p-code, run with the
op semantics the check uses (`machine::lift`, `value::eval`), against Unicorn (QEMU's
TCG as a library) from the same state (proposal, Tier L).

```sh
python3 scripts/diffverify/tier_l.py x86-64|aarch64|thumb BINARY [--limit N] [--states N]
```

- **kuna side:** `kuna-verify step BINARY --regs R,R.. --flags F,F..` sweeps each
  function and picks a few instructions of each form: the mnemonic, with every number in
  the operands replaced. It runs each instruction from random values of the named
  registers and prints one JSON line with the registers before and after, the memory
  bytes it read and wrote, and the next address. `--canonical` keeps 8-byte registers
  below 2^47, so a pointer among them is one the 48-bit address space can hold.
- **Unicorn side:** the driver maps the instruction bytes and the bytes kuna read, sets
  the same registers and flags, runs one instruction, and compares:
  - the registers;
  - the flags the architecture defines after that instruction (the Intel SDM's undefined
    flags are left out by mnemonic);
  - the bytes written;
  - the next address;
  - any byte Unicorn reads that kuna did not.

Left out and counted:
- instructions that read a register outside the named set (x87, MMX, NEON);
- user ops the step does not run (`syscall`, `cpuid`, coprocessor access);
- Thumb instructions inside an IT block, whose condition comes from ITSTATE, which one
  instruction run alone does not carry.

## 1. Results

Four states per instruction, two instructions per form, from the decbench binaries
(x86-64 and Cortex-M Thumb) and the C++ sample set's AArch64 builds.

| binaries | forms | same | `AF` only | differ | left out |
|---|---:|---:|---:|---:|---:|
| x86-64: `ls`, `bash`, `certtool` (`O2`), `ssh-add` (`O0`) | 6,686 | 5,263 | 1,391 | 0 | 32 |
| Thumb: `RTOSDemo`, `cf2.elf` (`O2`), `ch.elf` (`O0`) | 1,330 | 933 | — | 16 | 381 |
| AArch64: `ninja`, `libsnappy` (`O2`), `libdouble-conversion` (`O0`) | 789 | 697 | — | 8 | 84 |

x86's `AF`: Ghidra's x86 spec does not compute the auxiliary carry in `addflags`,
`subflags` and most other arithmetic; it leaves it as it was. No decompiled C reads it,
so these are counted apart, not as differences.

## 2. What differs

| instructions | forms | kuna | Unicorn (the architecture) | where |
|---|---:|---|---|---|
| AArch64 `ldrsh`/`ldrsb` into a `W` register | 8 | sign-extends into all 64 bits of `X` | sign-extends to 32 bits, zeroes the top 32 | `AARCH64base.sinc`: the 32-bit forms write `Rt_GPR64 = sext(...)` |
| Thumb `stmia Rn!` with `Rn` in the list | 7 | no writeback | `Rn += 4 × count`, always | `ARMTHUMBinstructions.sinc`: `Rn_exclaim_WB`, the LDM rule, is used for STM too |
| Thumb-2 `ands`, `bics`, `mvns` with an expanded immediate or a shifted register | 4 | `C` unchanged | `C` = the shifter's carry-out | `thSBIT_ZN` sets `Z` and `N` only; `ThumbExpandImm12` computes `shift_carry` and nothing reads it |
| VFP `vcvt.s32/u32.f32` out of range | 2 | x86's integer indefinite | saturates | p-code's `TRUNC` leaves it undefined |
| VFP `vfma`, `vfnma` | 2 | one ulp off | fused | the spec multiplies and adds with a rounding between |
| VFP `vsqrt` of a negative | 1 | `0xffc00000` | `0x7fc00000`, ARM's default NaN | the default NaN differs by architecture |

The first three are errors in the vendored SLEIGH specs. Every decompile of those
instructions has them, not only this check:
- After `ldrsh w0, [x1]`, kuna's IR holds a 64-bit sign extension where the machine holds
  a zero-extended 32-bit value, so a later use of `x0` (an address or a 64-bit compare)
  is wrong.
- After a Thumb `stmia`, kuna's base register is too low by the bytes stored.
- After a flag-setting Thumb-2 logical op, a `bcs`/`bcc` reads a stale carry.

The VFP rows are p-code under-specifying the operation. Tier C runs the same semantics on
both sides, so they do not turn into `DIFFERS` there.

One difference found on the way is fixed in the check:
- **A NaN operand to `+ − × ÷`** now comes back quieted, as SSE and ARM (with default-NaN
  mode off) return it, instead of the host's default NaN.

`hlt`, whose spec branches to itself, is left out as a system instruction.
