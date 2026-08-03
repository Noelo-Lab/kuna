# funcptralign - the cspec-declared function-pointer alignment was never read

Route B (decbench), NOVEL pool. Cluster record:
`docs/decbench/triage/novel-code-ptr-cluster.md` (landed on main in #247).
Witness case `O2-noinline-crazyflie-cf2-dwHandleInterrupt`, `cf2.elf` @
`0x804001c`.

## The symptom

Every ARM/Thumb indirect call in the corpus was emitted with an ISA-mode
bit-clear the source never performs, and the mask also cost the callee its type:

```c
/* kuna, before */
if ((dwIsClockProblem(a0)) && (*(uint4 *)(a0 + 0x44)))
  (*(code *)(*(uint4 *)(a0 + 0x44) & 0xfffffffe))(a0);

/* ghidra */
if ((iVar2 != 0) && (*(code **)(param_1 + 0x44) != (code *)0x0)) {
  (**(code **)(param_1 + 0x44))(param_1);
}
```

The source (`crazyflie/O2-noinline/compiled/libdw1000.i`, `dwHandleInterrupt`)
does no bit-clear at all - the fields are `dwHandler_t` function pointers:

```c
if(dwIsClockProblem(dev) && dev->handleError != 0) {
  (*dev->handleError)(dev);
}
```

So the C asserted an operation the program does not perform, *and*, because the
mask stood between the `CALLIND` and its pointer operand, the pointer-to-code
data-type never back-propagated onto the LOAD - the field stayed `*(uint4 *)`
and every call site needed an extra `(code *)` cast that ghidra does not need.

## Root cause

SLEIGH lowers every Thumb indirect call through `BXWritePC`
(`specs/Ghidra/Processors/ARM/data/languages/ARM.sinc:221`):

```
macro BXWritePC(addr) {
   SetThumbMode((addr & 0x1) != 0);
   local tmp = addr & 0xfffffffe;
   pc = tmp;
}
```

so `blx r3` lifts to `AND target, 0xfffffffe` -> `CALLIND`. (PR #231 erased the
first line of that macro, the `setISAMode` CALLOTHER; this is the second line,
which #231 did not touch.)

Upstream Ghidra erases that AND in `RuleFuncPtrEncoding`
(`decompiler/cpp/ruleaction.cc:9928`). kuna's port of the rule body is a
line-for-line transcription and is registered in the pool
(`infra/universalaction.rs:515`, `funcptrencoding`) - but its **only** input was
a hard-coded stub, in
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs`:

```rust
/// `data.getArch()->funcptr_align` (C++).  // STUB(W4)
fn funcptr_align(_data: &Funcdata) -> int4 {
    0
}
```

and the rule's first two lines are `let align = funcptr_align(data); if align ==
0 { return 0; }`. **The rule could never fire on any architecture.**

The stub's premise ("the W4 `Architecture` skeleton does not carry
`funcptr_align`") is stale: `infra/architecture.rs:244` declares the field,
`:2105 decode_funcptr_align()` parses `<funcptr align="N"/>` into the bit
position, `:3902` calls it during `parse_compiler_config`, and `:1869` copies it
into the ArchSeam that `Funcdata` reads (`substrate/context.rs:797`). Two other
consumers already read it live - `p2_lift/jumptable.rs:3661` and
`p3_dataflow/ruleaction_6.rs:609` (`RulePtrsubUndo::preserve_thumb_funcptr`,
GH-8471/DIV-2, whose stage test `tests/stages/gh8471-thumbfuncptr.xml` passes
today and is reachable **only** through a non-zero `funcptr_align`).

## The fix

One accessor:

```rust
fn funcptr_align(data: &Funcdata) -> int4 {
    data.get_arch().funcptr_align
}
```

## Scope: this is NOT ARM-only

Every cspec that carries `<funcptr>` becomes live. In the vendored specs:

| cspecs | `align` | `funcptr_align` (bit position) | mask erased |
|---|---|---|---|
| ARM, ARM_apcs, ARM_v45, ARM_win | 2 | 1 | `& ~1` (Thumb mode bit) |
| AARCH64, _apple, _golang, _ilp32, _win | **4** | **2** | `& ~3` (word alignment) |
| 9 MIPS cspecs | 2 | 1 | `& ~1` (MIPS16 mode bit) |
| 4 Loongarch, 8051 keil-cx51 | 2 | 1 | `& ~1` |

### AARCH64 `align=4` - verified explicitly

Three independent checks:

1. **Upstream does not special-case `align`.** `RuleFuncPtrEncoding::applyOp` in
   Ghidra 12.1 is a single code path: `slide = ~0 << align; if ((testmask &
   slide) == val) { opRemoveInput; opSetOpcode(COPY); }`. There is no `align ==
   1` branch. kuna's port is the same arithmetic, so honouring `align = 2` is
   not a kuna choice - it is upstream's declared contract, and the comparison is
   **exact** (a `& ~3` matches, a `& ~1` does not).
2. **AArch64's own lifting emits no mask.** `AARCH64base.sinc:887`,
   `:blr Rn_GPR64 { pc = Rn_GPR64; x30 = inst_start + 4; call [pc]; }` - no AND.
   The rule is therefore inert on ordinary AArch64 code and only fires on a mask
   the *program itself* wrote.
3. **Measured against stock Ghidra 12.1 headless** on a purpose-built AArch64
   probe (`clang --target=aarch64-linux-gnu -O1 -ffreestanding -c`), one function
   with a plain `blr x8` and one that really masks
   (`and x8, x8, #0xfffffffffffffffc; blr x8`):

   | | `handle` (no mask) | `handle_masked` (`& ~3`) |
   |---|---|---|
   | kuna before | `(**(code **)&a0[2])()` | `(*(code *)(*(uint8 *)&a0[2] & 0xfffffffffffffffc))(*a0)` |
   | kuna after | `(**(code **)&a0[2])()` | `(**(code **)&a0[2])(*a0)` |
   | **Ghidra 12.1** | `(**(code **)(param_1 + 2))()` | `(**(code **)(param_1 + 2))(*param_1)` |

   kuna's post-fix rendering matches stock Ghidra on both. The align=4 arm is
   also pinned at the rule level by a new unit test
   (`funcptrencoding_strips_aarch64_word_alignment`), which additionally asserts
   that a `& ~1` on an align=2 architecture is **declined**.

MIPS is the same shape as ARM: `mips.sinc:925 JXWritePC` under `ISA_VARIANT`
(defined by every `mips32*`/`mips64*` slaspec) does `tmp = -2 & addr`, so `jalr`
also carries the mask. The two MIPS fixtures in the tree (`mips_gp_le32`,
`plt_mips32`) emit no masked indirect call, and the six `MIPS:*:32:default`
datatests are unchanged (675/675).

## Owning phase

**P3 - definition web**, the rule pool (`docs/spec/03-ssa-and-simplification.md`
section 3.2). One module, one function body:
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_8.rs`.

## Why no option

`CLAUDE.md`: *"a strict bug fix that only corrects wrong output needs no flag"*.
All four tests apply here:

- The *processor cspec*, not kuna, declares `<funcptr align="N"/>`; the width is
  not a kuna judgment call.
- Upstream honours it unconditionally, and kuna's rule body is already a
  line-for-line port - this finishes a stubbed port rather than adding policy.
  It **removes** a divergence.
- The emitted C stated an operation the program does not perform.
- The ablation is clean: `kuna test --all --baseline docs/baseline.json` stays
  **PARITY OK 675/675** with the fix live, and `make test-stages` stays PARITY
  OK, with **no re-pin and no per-test opt-out**.

It is recorded as a DIV row because that is the registry's convention for
no-flag bug fixes (DIV-43...DIV-53 are all of this shape).

## Nothing in the catalog covered it

The triage swept all 84 non-default settings of the 83-row catalog on `cf2.elf
--addr 0x804001c`, counting `0xfffffffe`: every one returned 6, the baseline.
`--mode aggressive` = 6; `--mode fast`/`reliable` = 0 only because both degrade
that Cortex-M function to an empty body.
