# diffverify — triaging a disagreement: spec, emulator, evaluator or harness

A difference that diffverify reports is a fact about two executions. It is not yet a
fact about which of them is wrong. This page is the procedure for deciding, and the rule
that goes with it:

> **Do not call a difference a SLEIGH spec error until the reference it was compared
> against, kuna-verify's own evaluator and the harness have each been ruled out.**

Every Tier L or Tier C difference has at least five possible causes, and this work has
already met each of them.

## 1. The five suspects

| suspect | what it is | how it shows |
|---|---|---|
| **The SLEIGH spec** | the vendored `.sinc`/`.slaspec` turns the instruction into the wrong p-code | kuna's lift computes something the architecture manual defines otherwise |
| **The reference emulator** | Unicorn (QEMU's TCG as a library) in Tier L, or any other emulator used as the oracle | the oracle computes something the manual or the hardware does not |
| **kuna-verify's evaluator** | `value::eval`, `x87.rs` and the op semantics both sides of Tier C share | one p-code op evaluates differently from what the op means (NaN, division by zero, shifts past the width, `TRUNC` out of range) |
| **The harness** | the Tier L driver (`scripts/diffverify/tier_l.py`, `kuna-verify step`) or the Tier C model (state, stubs, calls run for real, returns) | inputs the two sides did not get alike, or a run the model follows differently from a CPU |
| **The architecture itself** | the manual leaves the result undefined, UNPREDICTABLE or implementation-defined | two correct implementations disagree, and neither is wrong |

The spec is one row of five. A report that names it has to say why the other four are
not the cause.

## 2. What this work has met

Each suspect has produced a difference that, at first sight, looked like another one.

**Spec errors (confirmed).** Three, all in Tier L (`tier-l-results.md` §2):
- AArch64 `ldrsh`/`ldrsb` into a `W` register sign-extend into all 64 bits of `X`. The
  Arm ARM's pseudocode sign-extends to the register size, 32 bits for a `W` register,
  and a write to a `W` register zeroes the top half of `X`. Confirmed by running the
  instruction on an AArch64 CPU. Already reported upstream as
  NationalSecurityAgency/ghidra#6471.
- Thumb `stmia Rn!` with `Rn` in the register list skips its writeback: the spec reuses
  LDM's `Rn_exclaim_WB` rule. Checked against the ARMv7-M pseudocode and Unicorn; no
  Cortex-M hardware was run. NationalSecurityAgency/ghidra#10273.
- Thumb-2 `ands`, `bics`, `mvns` with a shifted or expanded operand leave `C` unchanged
  where the pseudocode sets it to the shifter's carry-out. Checked against the ARMv7-M
  pseudocode and Unicorn; no hardware. NationalSecurityAgency/ghidra#10274.

**Under-specified p-code (not errors in the same sense).** VFP `vcvt` out of range, `vfma`
and `vfnma`, and the default NaN of `vsqrt` (`tier-l-results.md` §2): the spec's p-code
cannot say what the architecture defines (`TRUNC` leaves out-of-range undefined; p-code
has no fused multiply-add; NaN payloads are not modelled). These are reported as gaps,
not as errors.

**A spec gap kept apart.** x86's `AF`: Ghidra's x86 spec does not compute the auxiliary
carry for most arithmetic. No decompiled C reads it, so Tier L counts it in its own
column instead of as a difference.

**The evaluator.** A `DIVSS` with a NaN operand came back as the host's default NaN
instead of the operand, quieted, as SSE and ARM return it. The spec was right; the fix
was in `value.rs`.

**The Tier L driver.** Before any of these were fixed, each looked like a difference:
- a random 8-byte register used as a pointer held a non-canonical address, which the
  x86-64 machine faults on (`--canonical` keeps them below 2^47);
- Unicorn's instruction prefetch read code bytes kuna never read (fetch accesses are no
  longer counted as reads);
- an access to unmapped memory for a write was counted as a read (only `READ_UNMAPPED`
  counts);
- a Thumb instruction inside an IT block runs under ITSTATE, which a single instruction
  run alone does not carry (left out and counted);
- flags the Intel SDM leaves undefined after an instruction differed (left out by
  mnemonic).

**The Tier C model.** On real binaries (`c4-decbench-results.md`), harness defects
produced `DIFFERS` that were not kuna's:
- an RTOS context switch: the callee loads another thread's stack pointer and returns
  into that thread;
- libgcc's soft-float routines (`__aeabi_ddiv`), which reach a shared tail with `bl`
  and return straight to their caller's caller: the return was taken for a context
  switch and the call got the stub;
- a call `kuna decompile` knows does not return only through the Listing (`listing on`,
  which the CLI turns on and kuna-verify did not), or through an `error (1, ...)`
  `CALL_RETURN` override, or because the compiler left only padding after it;
- a call whose result kuna keeps in a joined register pair (`r1:r0`), which the replay
  did not read back.

And the architecture: ARM and AArch64 integer division by zero gives 0, where C leaves
`/` undefined; Tier C declares it (`DIVERGES(divide-by-zero)`) instead of reporting it.

## 3. The checklist

Work through it in order, and keep the answers: the report quotes them.

1. **Reduce it.** One instruction and one state for Tier L (`kuna-verify step` prints
   the state as JSON); for Tier C, the smallest function and input (`--seed`, `--trace`),
   and a C source compiled with `clang --target=...` when one reproduces it.

2. **Rule out the harness.**
   - Did both sides get every input? Tier L leaves out instructions that read a register
     outside the named set; check that the instruction does not read one anyway
     (a flag, a system register, FPSCR/MXCSR, the x87 control word).
   - Is execution state part of the input? IT blocks, Thumb or ARM mode, `FPSCR.DN`,
     `FPSCR.FZ`, rounding mode, privilege.
   - Is the memory the same? Mapped ranges, read-only ranges, what Unicorn prefetches.
   - For Tier C: does the machine side follow the code as a CPU would? Look at the trace
     for calls run for real, stubs, returns that do not land after their call, a stack
     pointer that jumps, a call the machine makes that kuna's C does not (or the reverse)
     next to a no-return callee.
   - Is the kuna output checked the one `kuna decompile` prints? Run `kuna decompile` on
     the same function and read the C. If they differ, the options differ.

3. **Rule out the evaluator.** Print the p-code (`kuna raw-pcode BINARY ADDR`) and
   evaluate the ops by hand from the state. Pay attention to ops whose meaning has edge
   cases: `FLOAT_*` with NaN or infinity operands, `TRUNC`, `INT_DIV`/`INT_SDIV`/`INT_REM`
   by zero, shifts by the width or more, `INT_SCARRY`/`INT_SBORROW`, `PIECE`/`SUBPIECE`
   on non-byte boundaries. If the hand result differs from kuna-verify's, the evaluator is
   wrong.

4. **Rule out the reference emulator.**
   - Record its version (Tier L uses Unicorn 2.1.2) and look for known issues with the
     instruction in its tracker and in QEMU's.
   - Run a second reference where one is available: QEMU user mode
     (`qemu-aarch64`, `qemu-arm`) with a small static test program, or another emulator.
     Two independent references agreeing is evidence; one is not.
   - Remember that Unicorn is QEMU's TCG from an older QEMU, and that TCG implements what
     the architecture defines, not always what a given CPU does with what it leaves
     undefined.

5. **Read the manual.** The pseudocode, not the prose: the Intel SDM's operation
   section, the Arm ARM (DDI 0487) for A64 and A32/T32 on ARMv8, the ARMv7-M ARM
   (DDI 0403) for Cortex-M, the ARMv7-A/R ARM (DDI 0406). If the result is UNDEFINED,
   UNPREDICTABLE, CONSTRAINED UNPREDICTABLE or implementation-defined, stop: it is the
   architecture's row of §1, and the report says so.

6. **Run it on hardware** whenever two references disagree, the manual's reading is not
   unambiguous, or the finding is going upstream. A few lines of assembly that set the
   state, run the instruction and print the result are enough. Say which CPU. Where no
   hardware was run (the Thumb findings above), the report says that too.

7. **Check upstream.** Compare the vendored spec with Ghidra at `GHIDRA_REV` and at its
   current head (`docs/history.md`), and search Ghidra's issues and pull requests. The
   AArch64 finding was already NationalSecurityAgency/ghidra#6471.

8. **Only then name the cause,** with one of these labels:

| label | means | needs |
|---|---|---|
| `spec error` | the vendored spec computes what the manual defines otherwise | 5, and 6 or two references from 4 agreeing |
| `spec gap` | p-code cannot express what the architecture defines (fused ops, NaN payloads, out-of-range conversions) | 5 |
| `architecture-undefined` | the manual does not define the result | 5 |
| `emulator` | the reference disagrees with the manual or the hardware | 5, and 6 or a second reference |
| `evaluator` | kuna-verify evaluates a p-code op wrongly | 3 |
| `harness` | the driver or the model gave the sides different inputs or ran the code differently | 2 |
| `kuna bug` (Tier C) | the decompiled C computes something the machine code does not | 2 and 3, and the C read by hand |

## 4. Reporting

A report of a disagreement, in an issue, a PR or a results file, states:
- the instruction or function, the binary and its source (dataset path, sha256);
- the state or input that shows it, and both results;
- the label from §3, and which steps of the checklist were done: the manual section
  read, the references run (with versions), the hardware run (with the CPU), or that
  none was;
- for a `spec error`, the line of the `.sinc` that produces the wrong p-code.

A finding without that chain is reported as a difference, not as an error of any one
party.
