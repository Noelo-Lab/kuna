# Relative pointers buildLocaltypes seed (w10-relative-pointers) — ACCEPT, +4

Merge of `worktree-agent-a3f4a8c7cbcff7830` @ `05331f5`, base `bb21c8b`/624 → **628**. ACCEPT.

## Root + fix
`ptrrel`'s non-type-locked stack Varnodes floated to `int8`/plain `TYPE_PTR` instead of the formal
`myptroff` `TypePointerRel`, so `RuleStructOffset0`'s relative arm (ruleaction_5.rs:1277,
`is_formal_pointer_rel`) never fired → `*ptrrel` LOAD / `ptrrel[k]` PTRADDs were never rewritten into
PTRSUB-into-parent. Root = the unported `ActionInferTypes::buildLocaltypes` type-locked-symbol seed
(coreaction.cc:5275-5281).
- `varmap.rs:1722` — `ScopeLocal::build_localtype_seed` (covering entry via `find_overlap`=queryProperties
  → `getExactPiece(symbol.getType(), curOff, size)` for a type-locked owning Symbol, with the C++
  null/UNKNOWN float-fallback).
- `coreaction_infertypes.rs:308` — `build_localtypes` consults the seed for non-type-locked live
  Varnodes before falling back to `get_local_type` (verbatim C++ arm).

## Effect / gate
628 (+4: Relative pointers #1/#2/#3 + No for-loop alias #1 bonus). datatests `[675,628]`, regressed-set
EMPTY; cargo --no-fail-fast 0-fail; oracle PARITY OK; C++/docs byte-untouched. Relative pointers #7 =
RuleConditionalMove float-NaN-compare (LOSS-074, separate). No special-casing.
