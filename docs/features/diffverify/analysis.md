# diffverify — analysis

Why kuna needs a standing semantic check, and what the existing evidence says about where
it should look first.

## 1. Wrong-value bugs are fixed one at a time, and each fix builds its own oracle

Fix commits since 2026-06-01 that carry a phase tag (`fix(pN)` in the subject; newer
commits often omit the tag, so these are lower bounds):

| phase | fixes | what an oracle would have to compare |
|---|---:|---|
| P2 lift / flow | 32 | one instruction's p-code against the machine |
| P3 dataflow / rules | 31 | p-code before and after the simplification band |
| P4 calls | 24 | the same, with call sites and the prototype in play |
| P9 emit | 24 | the printed C against the binary |
| P6 variables | 12 | the printed C against the binary |
| P1, P5, P8, P0 | 14 | — |

Each of these fixes proved its own correctness with an oracle written for that PR:

- 84 test files under `decompiler/crates/kuna-cli/tests/` invoke a C compiler, most of
  them to compile the printed C and compare it with the source. `zext_pair_returns.rs`
  (#794) compares printed and original over 289 input pairs per target.
- 5 of them run the result under `qemu-user` (`x87_native_returns.rs`, `call_wide.rs`,
  `mips_elf_selection.rs`, …), and
  `docs/features/conditional-register-lifetimes/verify_arm.py` runs a Python Unicorn
  model by hand.

Every one of these oracles answers one question for one fixture and is never run on
anything else. The same machinery, run over the corpus, would have found these bugs
before a user did.

## 2. A representative bug: the one the oracle would catch

#794 (`fix(p3)`): on 32-bit ARM, `u64 ins16(unsigned a, unsigned short b) { return ((a + 1)
<< 16) | b; }` printed as `int ins16(int a0, unsigned int a1)`. The lifted p-code returns
`r1:r0` with `r1 = 0`. `RuleSubvarZext` cut the `ZEXT` feeding the RETURN, so after Band B
the function claims to return a signed 32-bit word. A caller widening the result
sign-extends `0x80000000` to `0xffffffff80000000`.

Two observations shape the design:

1. The bug is visible **without printing C**. The p-code after P2 and the p-code after
   Band B disagree on the 64-bit ABI return value for `a0 = 0x7fff`. One SMT query per
   function finds the counterexample.
2. It is visible only if the comparison reads the after-state **through the prototype the
   decompiler claims**. Comparing just `r0` with `r0` says "equivalent". The observable
   must be the ABI value the printed signature implies: an `int` return sign-extended to
   the pair, compared with the lifted `r1:r0`.

## 3. Pieces kuna already has

- `kuna-sleigh/src/emulate.rs`: `EmulatePcodeCache`, a concrete p-code emulator, already
  used by jump-table recovery (`p2_lift/kuna_emulatefunction.rs`).
- `kuna-harness`: a dev-only crate that is never shipped, already the home of
  differential tooling (`liftdiff`, golden comparison).
- Named rules with `count_apply` statistics and `disable_rule(specify)`
  (`infra/action.rs`), so a failing function can be bisected down to the rules that
  fired.
- Draft #1699 (`HookRegistry::on_phase`): callbacks at P2, Band B and P6 boundaries with
  no cost when nothing is registered. Those are the snapshot points this needs.

## 4. Why not build the solver and emulator

Fission (a sibling Rust decompiler) wrote its own SAT/bit-vector solver and its own JIT
emulator for the same purpose. The solver's invariant list
(`crates/fission-solver/AGENTS.md`) is mostly soundness bugs that were found after the
fact. `x*3 != x*5` came back UNSAT, which is a false equivalence proof.
`x == 0 ∧ x + x != 0` came back SAT. Pigeonhole problems came back SAT. A checker that
is wrong this way is worse than no checker, because its "equivalent" gets trusted. kuna
should use z3 and QEMU, which are maintained and widely exercised, and spend its effort on
the encoding.
