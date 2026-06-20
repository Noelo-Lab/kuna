# Union Family-3 inheritUnionField / resolveInFlow (w10-union-inherit-field) — ACCEPT, +3

Merge of `rport/w10-union-inherit-field` @ `d8bee70`, base `8c88a1d`/614 → **617**. ACCEPT.

## Root + fix
The in-flow union resolution cache was never populated, so `RulePtrArith`'s pointer-slot scan saw the
union-member load as `TYPE_UNION` (needsResolution) and declined the offset-4 PTRSUB fold. The producer
(`resolveInFlow` arm of `propagateTypeEdge`) was missing and consumers used the bare-Varnode facing stub.
- `coreaction_infertypes.rs:398` — `propagate_type_edge` runs `resolve_in_flow` on a needs-resolution
  `alttype` (coreaction.cc:5335-5341).
- `addtreestate.rs:100/739/773` — `vn_type_read_facing` + PTRADD `forceFacingType`/`inheritUnionField`
  + PTRSUB `inheritUnionField` (ruleaction.cc:6524-6541).
- `ruleaction_5.rs` (RuleStructOffset0 inheritUnionField arms, ruleaction.cc:6741/6779) +
  `ruleaction_6.rs:373/955` (RulePtraddUndo/RuleSubRight — fixes the RulePtrArith↔RulePtraddUndo
  oscillation the unresolved bare stub caused).

## Effect / gate
617 (+3: Union #4/#25/#27). datatests `[675,617]`, regressed-set EMPTY; cargo --no-fail-fast 0-fail;
oracle PARITY OK; C++/docs byte-untouched. Union #14/#17 remain (Family-2 `longField`, separate).
No special-casing.
