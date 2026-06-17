# ModuloAlt — RuleDoubleArithShift clone-gate fix (w10-moduloalt) — ACCEPT, +3

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-moduloalt` @ `e71471a`, base
a0844ea/476). Verdict: **ACCEPT**.

## Root + fix

`RuleDoubleArithShift::clone_rule` (`ruleaction_2.rs:1009`) is registered under runtime group
`"analysis"` (C++ `new RuleDoubleArithShift("analysis")`, coreaction.cc:5808; rust
universalaction.rs:421), but its clone gate tested `grouplist.contains("doublearithshift")` — the
per-op PLACEHOLDER name, which is NOT a member of the `"decompile"` grouplist (action.rs:1573,
which contains `"analysis"`). So the rule was dropped from every cloned `ActionPool` and
`apply_op` never ran (probe: 0 entries). Consequence chain: `RuleDivOpt` (ruleaction_6.rs:2131)
emits the signed-division sign correction with a redundant `(x s>>31) s>>31`; the double-shift was
never collapsed, leaving `div + (x s>>31) - (x s>>31)` un-cancelled, which blocked `RuleModOpt`
(ruleaction_7.rs:794) from reconstructing `x % C`. Fix mirrors the already-fixed sibling
`RuleDoubleShift` (ruleaction_2.rs:865, same bug class): gate on
`contains("analysis") || contains("doublearithshift")`.

## Effect

476 → 479. Gained: ModuloAlt #2 (`a0 % 3`), ModuloAlt #4 (`a0 % 6`), If/Switch #4 (bonus — the
same shift-collapse was load-bearing there). The ~150 Signed/Unsigned Division/Modulo + Modulo
#1-40 assertions all still pass.

## Gate (re-run at integration onto 476)

- `cargo test --workspace` (debug): 0 failures (B0 `universalaction_listing` byte-equal — the
  fix is a clone-gate predicate, registration order/positions untouched).
- datatests: `[675, 479]`; passing-set diff vs base 476: **regressed-set EMPTY** (gained the 3).
- switch cluster unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## Follow-up flagged (not in this wave)

`RuleConcatShift` (ruleaction_2.rs:1064) has the IDENTICAL mis-gating (`contains("concatshift")`
for an `"analysis"`-grouped rule, C++ coreaction.cc:5809) — left untouched to keep this change
minimal/zero-regression; a one-line candidate if a future test exercises `concat(V,W) >> c`.

ModuloAlt #3 (`a0 % 4`) is a separate root: a power-of-2 signed modulo via `CMOVNS` that rust
lifts to a CBRANCH+MULTIEQUAL; `RuleSignMod2nOpt` (ruleaction.cc:8688) can't fire until the
conditional-move branch is eliminated (ActionConditionalExe/blockaction — Chain B territory).

## No special-casing

A general clone-gate predicate fix matching the sibling rule; no magic-constant or value
hardcoding.
