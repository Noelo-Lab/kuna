# diffverify — analysis

Why kuna needs a standing semantic check, which bugs it can and cannot see, and what it
can be built from.

## 1. Wrong-value bugs are fixed one at a time, and each fix builds its own oracle

Fix commits since 2026-06-01 that carry a phase tag (`fix(pN)` in the subject; newer
commits often omit the tag, so these are lower bounds): P2 32, P3 31, P4 25, P9 24,
P6 12, others 14.

Each fix proved itself with an oracle written for that PR:

- 84 test files under `decompiler/crates/kuna-cli/tests/` invoke a C compiler, most of
  them to compile the printed C and compare it with the source. `zext_pair_returns.rs`
  (#794) compares printed and original over 289 input pairs per target.
- 5 of them run the result under `qemu-user` (`x87_native_returns.rs`, `call_wide.rs`,
  `mips_elf_selection.rs`, …), and
  `docs/features/conditional-register-lifetimes/verify_arm.py` runs a Python Unicorn
  model by hand.

Each oracle answers one question for one fixture and never runs on anything else.

## 2. What a function-level check can see

The 56 P3 and P4 fixes, classified by subject line and PR description. These are
approximate: a subject line is not a root cause.

| class | count | examples | visible to a function-level check? |
|---|---:|---|---|
| **body rewrite**: a rule changes a value, drops a store, or drops a call | ~15 | #777 wide constants with oversized shifts; #801 a call dropped after stack byte stores; #893 a store into the caller's home slot dropped; #533 a loop counter's write-back dropped; #251 `RuleSubRight` truncation | yes, by comparing outputs and stores |
| **prototype/ABI claim**: wrong parameters, wrong return width or signedness, halves swapped | ~29 | #794, #763, #855, #877, #875, #828, #229 | only if the decompiled side is read **through the prototype it claims**, on inputs and outputs (§3) |
| **type, print, naming, crash, flow** | ~12 | #756, #677 narrow reads printed wide; #639 naming; #562 panic; #1501 jump tables | no |

So the check sees roughly 15 bugs with a plain comparison and roughly 44 with ABI
projection. It sees none of the type and printing class. That class, plus P6 merges and
P9, needs the printed-C layer.

## 3. The representative bug, and why projection is required

#794 (`fix(p3)`): on 32-bit ARM, `u64 ins16(unsigned a, unsigned short b) { return ((a + 1)
<< 16) | b; }` printed as `int ins16(int a0, unsigned int a1)`. The machine returns `r1:r0`
with `r1 = 0`. `RuleSubvarZext` cut the `ZEXT` feeding the RETURN, so the decompiled
function claims to return a signed 32-bit word.

Comparing `r0` with `r0` says the two agree. The disagreement appears only when the
claimed `int` return is mapped back to the ABI the way a caller would read it, which is
sign-extended into `r1:r0`. On `a0 = 0x7fff` the machine gives `0x0000000080000000` and
the claim gives `0xffffffff80000000`.

The same holds on the input side. A recovered prototype that drops a parameter (#875)
leaves the body reading a register no caller sets. The check has to feed such reads a
value the machine side does not get.

## 4. Why equivalence is the hard part

Lifted machine code and the decompiled function describe the same computation in
different vocabularies. The encoding has to bridge each gap explicitly, or it reports
differences that are not bugs:

- **Calls.** The machine passes the whole register file; the decompiled call passes the
  recovered arguments. INDIRECT ops encode what kuna assumes a call may change.
- **Stack.** Concrete `sp`-relative slots on one side; locals on the other, some of whose
  stores kuna deliberately drops as dead.
- **Deliberate divergence.** Some defaults trade fidelity for readability on purpose.
  #794 keeps trimming x86-64 `RDX:RAX` because a zeroed `RDX` is usually
  `-fzero-call-used-regs`. Each of these must be declared, keyed by option, or it shows
  up as a bug.

A checker drowning in false differences gets ignored. This drives two design choices:
concrete execution before symbolic proof, and a declared-divergence list from day one.

## 5. Pieces kuna already has

- **Machine-side emulator.** `kuna-sleigh/src/emulate.rs` has `EmulatePcodeCache`, which
  runs SLEIGH p-code over the load image.
- **Graph-side emulator.** `p2_lift/kuna_emulatefunction.rs` has
  `EmulateFunction`/`EmulatePcodeOp`, which runs ops over a `Funcdata` syntax tree with a
  varnode-keyed state. It is single-path and externally driven today, for jump-table
  recovery, and is the base for an interpreter over the decompiled function.
- **Dev-only home.** `kuna-harness` already hosts differential tooling and is never
  shipped.
- **Rule control.** Rules are named, keep `count_apply` statistics, and can be switched
  off with `disable_rule(specify)` (`infra/action.rs`). A failing function can be bisected
  down to the rules that fired.
- **Phase hooks.** Draft #1699 (`HookRegistry::on_phase`) would add P2, Band B and P6
  snapshot points. It is useful for localizing a difference, but the first slice does not
  need it (proposal, Tier C).

## 6. Why not build the solver and emulator

Fission (a sibling Rust decompiler) wrote its own SAT/bit-vector solver and JIT emulator
for the same purpose. The solver's invariant list (`crates/fission-solver/AGENTS.md`) is
mostly soundness bugs found after the fact:

- `x*3 != x*5` came back UNSAT, which is a false equivalence proof;
- `x == 0 ∧ x + x != 0` came back SAT;
- pigeonhole problems came back SAT.

A checker that is wrong this way is worse than none. kuna should use z3 and QEMU, and
spend its effort on the encoding (§4).
