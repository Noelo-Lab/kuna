# RuleAddUnsigned getTypeReadFacing arm (w10-fpconvert) — ACCEPT, +3

Wave (2026-06-18) integrated at rust-port (merge of `rport/w10-fpconvert` @ `c595216`, base
219f74a/482). Verdict: **ACCEPT**.

## Root + fix

`RuleAddUnsigned::apply_op` (`ruleaction_6.rs:629`) was stubbed — it bailed at the
`seam_type_read_facing` (W6 `getTypeReadFacing`) seam, so `INT_ADD(V, 0xff..)` never became
`V - 0x00..`. The read-facing type resolver (`Funcdata::vn_type_read_facing`,
funcdata_facing.rs:135) and the commit helpers (`op_set_opcode_code`→`type_op_for(INT_SUB)`,
`new_constant`, `copy_symbol`, `op_set_input`) have all since landed. Ported faithfully from
`ruleaction.cc:7215` (quarter-mask, named-equate name-lock, enum `hasNamedValue` guards included)
→ `(fval - 0x10)` for floatconv #1.

## Effect

482 → 485. Gained: Floating-point convert #1, Promotion on compare #1, **Switch Multi #3**
(bonus — switchmulti is now **9/9**, fully recovered). The fix is general (any
INT_ADD-with-high-constant → INT_SUB where the read-facing type is signed), hence the cross-family
gains.

## Gate (re-run at integration onto 482)

- `cargo test --workspace --no-fail-fast`: 0 failures.
- datatests: `[675, 485]`; passing-set diff vs base 482: **regressed-set EMPTY** (gained the 3).
- switch cluster: switchind 16/16, **switchmulti 9/9**, switchhide 3/4 — improved (no regression).
- oracle **PARITY OK**; C++ tree byte-untouched.

## Remaining (floatconv #2/#3 — separate root)

The unsigned-64→double idiom collapse (`(int8)v1 < 0 ? (float8)v1 : (float8)(int8)v1` →
`(float8)(lval - 0x10)`) needs `RuleInt2FloatCollapse::apply_op` (ruleaction_8.rs:420-425, stubbed
at `FlowBlock::findCondition` + `newUniqueOut` seams; C++ ruleaction.cc:9878, sibling
RuleUnsigned2Float ruleaction.cc:9810). A separate ruleaction_8 wave.

## No special-casing

Faithful RuleAddUnsigned port matching C++; no value hardcoding.
