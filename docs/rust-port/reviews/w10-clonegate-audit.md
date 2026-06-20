# clone-gate audit — 17 mis-gated rules (w10-clonegate-audit) — ACCEPT, +2

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-clonegate-audit` @ `3b99115`, base
b54ffbc/479). Verdict: **ACCEPT**. Systematic sweep of the bug class first found in ModuloAlt.

## Bug class

A Rule registered under a runtime GROUP (e.g. `new RuleX("analysis")`, coreaction.cc:5787-5883)
whose Rust `clone_rule` clone-gate tested `grouplist.contains("<placeholder>")` — the per-op
PLACEHOLDER name (the rule's own short name) — instead of the registration group. The placeholder
name is NOT a member of the `"decompile"` grouplist (action.rs:1573), so the rule was DROPPED from
every cloned ActionPool and its `apply_op` NEVER ran.

## 17 rules fixed (all in ruleaction_2.rs, all `"analysis"`-grouped)

RuleFloatRange, RuleAndCommute, RuleAndPiece, RuleAndZext, RuleAndCompare, RuleDoubleSub,
RuleConcatShift, RuleLeftRight, RuleShiftCompare, RuleLessEqual, RuleLessNotEqual, RuleTrivialBool,
RuleZextEliminate, RuleSlessToLess, RuleZextSless, RuleBitUndistribute, RuleBooleanUndistribute.
Each gate → `contains("analysis") || contains("<placeholder>")`. (RuleConcatShift was the known
remaining instance flagged by the ModuloAlt wave; the other 16 are newly found.) Every other
literal-string clone gate in the audited files was cross-checked CORRECT (gate literal ==
registration group): RuleLoadVarnode/RuleStoreVarnode→stackvars, RuleSegment→segment,
RulePtrFlow→subvar, RuleConditionalMove→conditionalexe, RuleFloatCast/RuleIgnoreNan→floatprecision,
etc. (ruleaction_5.rs deferred to a separate audit — was owned by a concurrent wave.)

## Effect

479 → 481. Gained: Bitfields #2, Signed byte #4. The other 15 activated rules are gain-or-neutral
(correctness fidelity — they now fire as C++ does); none regressed.

## Gate (re-run at integration onto 479)

- `cargo test --workspace --no-fail-fast`: 0 failures (B0 `universalaction_listing` byte-equal —
  only clone-gate predicates changed, registration order untouched).
- datatests: `[675, 481]`; passing-set diff vs base 479: **regressed-set EMPTY** (gained the 2).
- switch cluster 8/16/3 unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## No special-casing

A general clone-gate predicate fix (group || placeholder) matching each rule's registration; no
value/name hardcoding. NOTE: `ruleaction_5.rs` (RulePtr family) still needs the same audit.

## No body bugs

Every activated rule was gain-or-neutral — none had to be excluded for a separate body bug,
indicating the rule bodies were faithfully ported and only the clone-gate predicate was wrong.
