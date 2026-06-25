# [PROPOSAL] optimized-memcpy-6301a9 — recover unrolled-guard jump tables (MSVC optimized memcpy)

**Status:** proposal (large feature — needs human go/no-go before implementation)
**angr testcase:** `test_decompiling_optimized_memcpy` :: `0x42cca0`
**Binary:** `i386/windows/736cb27201273f6c4f83da362c9595b50d12333362e02bc7a77dd327cc6b045a`
**Proposed option:** `unrolledguard on|off`

## The problem

On the MSVC CRT optimized memcpy (`0x42cca0`, Duff's device), angr recovers **7**
jump-table switches; kuna recovers **1** and degrades the other **6** to
`/* WARNING: Treating indirect jump as call */`. Full analysis + side-by-side:
[`analysis.md`](analysis.md) / [`angr-vs-kuna.txt`](angr-vs-kuna.txt).

The six failing tables share one root cause (see analysis): their bound comes from a
Duff's-device **unrolled/duplicated `cbranch` guard** at a block-merge point, and
kuna's `JumpBasic::check_unrolled_guard` (`s2_lift/jumptable.rs:2075`) is a documented
`// SEAM(structuring)` **stub**. The value-set / index-transform / table-emulation
path is already ported and works — only the unrolled-guard bound is missing.

## angr reference

angr's `JumpTableResolver` (in `angr/analyses/cfg`) resolves these as bounded jump
tables; the structurer then emits `switch` per table. The kuna-equivalent is Ghidra's
`JumpBasic::checkUnrolledGuard` / `checkCommonCbranch` (`jumptable.cc:1257`-ish) plus
the `BlockBasic` SSA helpers it depends on.

## Why this is LARGE (Hard Rule 7)

A decider subagent investigated and confirmed **large**:

- It is **not** a self-contained `kuna_*.rs` Action/Rule. The fix edits S2
  jumptable-recovery core (`analyzeGuards`/`findNormalized` in `jumptable.rs`) **beyond
  a single gated early-return**.
- It requires **new infrastructure**: porting the unported `BlockBasic` SSA helpers
  `findMultiequal` / `noInterveningStatement` / `unblockedMulti` / `liftVerifyUnroll`
  (all `// SEAM(W7)` in `substrate/block.rs`).
- It touches **>3 ported-core anchor files** (`jumptable.rs`, `block.rs`, `funcdata`,
  plus option/arch wiring) and is not cleanly gateable default-off (it changes core
  recovery, not an opt-in rendering pass).
- The rejected alternative — a `kuna_loweredswitch.rs`-style *manufacturing* Action
  that reads the absolute tables from the loadimage — is **even larger**: it duplicates
  working recovery, and the Duff's-device targets are mid-block, forcing CFG block
  splitting plus its own bounds inference.

## Proposed implementation plan (multi-step)

1. **Port the W7 `BlockBasic` SSA helpers** `findMultiequal` / `noInterveningStatement`
   / `unblockedMulti` / `liftVerifyUnroll` in `substrate/block.rs` (faithful C++
   transcription; standard W7 SSA-helper effort).
2. **Port `checkCommonCbranch` + fill `check_unrolled_guard`** in `jumptable.rs` to
   detect the duplicated guard across the unrolled merge and contribute the bounding
   range constraint into `analyzeGuards`.
3. **Gate** behind a new arch flag + `option unrolledguard on|off`
   (`change_kind = structure-recovery`), default-off while developing; correct the
   stale `jumptable.rs` module-header SEAM note that misdescribes the chain as unported.
4. **Stage test** `tests/stages/ghangr-optimized-memcpy-6301a9.xml`: the bytechunk of
   `0x42cca0` + its in-`.text` tables; pass 1 (`option unrolledguard off`) asserts the
   `Treating indirect jump as call` degradation and `switch(` count == 1; pass 2
   (default/on) asserts the recovered switches (target: 7, matching angr).
5. **Ablation + speed**: `kuna test --all --baseline docs/baseline.json` must stay
   PARITY OK; measure decompile wall-time off vs on (jump-table recovery + extra SSA
   walks can be non-trivial; if over the speed budget, ship opt-in).

## Speed / risk assessment

- **Risk (correctness):** editing core jump-table recovery can regress the existing 1
  recovered switch and the broader datatest jump-table corpus. `make test` /
  `make test-stages` parity is the hard gate. Validate the bound across all six
  transformed tables (negative stride `-4`, offset `(a2-4)*4`, masks `&3`).
- **Risk (scope):** the W7 SSA helpers are reused well beyond this case, so the port
  should be done carefully/faithfully, not pattern-matched to this one function.
- **Speed:** the extra guard/SSA analysis runs only on functions with `size_in()>1`
  guard blocks; expected to be modest, but must be measured (Hard Rule 6).

## Recommended option name

`unrolledguard` (the construct is "jump table bounded by an unrolled/duplicated cbranch
guard"), `change_kind = structure-recovery`, `source_decompiler = angr`,
`inspiration = "test_decompiling_optimized_memcpy; JumpBasic::checkUnrolledGuard / angr JumpTableResolver; 0x42cca0"`.

---
🤖 Generated with [Claude Code](https://claude.com/claude-code)
