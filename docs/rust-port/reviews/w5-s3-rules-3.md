# w5-s3-rules-3
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier, round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (465 lib + all integration suites pass; 0 failed). `cargo clippy -p kuna-decomp -- -D warnings` -> clean.

## Scope
decompiler/cpp/ruleaction.cc 2819-4292 — 22 Rule classes (RuleBooleanDedup ..
RuleAddMultCollapse) + declarations. Rust: rust/crates/kuna-decomp/src/ruleaction_3.rs
(the only behavioral file in the diff; dtype.rs is the shared W5 dtype-expand scaffold,
out of this item's scope and reviewed under w5-dtype-expand).

## Hunt list
- **Signedness**: clean. The `int4`-vs-`uintb` comparisons (`val < 8*size` in
  RuleTrivialShift :874; `val != 8*size-1` in RuleSignShift :918 / RuleTestSign :1009)
  cast the int4 RHS `as uintb` exactly mirroring the C++ promotion; `val >= 32` in
  RuleShift2Mult :1142 keeps the C++ `int4 val` truncation (see F2).
- **Integer widths**: clean. `int4 val = constvn->getOffset()` (RuleShift2Mult :1141),
  `int4 sa`/`int4 rsa` (RuleShiftPiece :1248/:1282) all reproduce the C++ int4
  truncation. `eval_binary` (:1795) passes `(c0size, c0size, ...)` — matches the C++
  `evaluateBinary(c[0]->getSize(),c[0]->getSize(),...)` double-size argument.
- **Wrapping**: clean. RuleIndirectCollapse big-endian SUBPIECE trunc uses
  `wrapping_add`/`wrapping_sub` (:640-644) matching C++ unsigned overflow;
  RuleCarryElim `off.wrapping_neg() & calc_mask(vn2.size)` (:1562) matches
  `(-off)&calc_mask`. (One bare `<<` in RuleShift2Mult — see F2.)
- **Comparator totality**: n/a — no comparators/BTree ordering in this batch.
- **Iteration-order provenance**: clean. Every descend walk uses `descend_snapshot`
  (a Vec snapshot of the C++ `list` descend order; `funcdata_varnode.rs:463`) — used in
  RuleSignShift, RuleTestSign::find_comparisons, RuleShift2Mult; the C++
  `op = *iter++` cursor-before-mutate idiom is preserved by snapshotting before
  mutation. No HashMap/HashSet anywhere (grep clean).
- **Off-by-one / do-while / reverse iterators**: clean. RuleShift2Mult's
  `for(;;){ probe arithop; if desc==end break; arithop=*desc++; }` is ported as a
  `loop` that probes `arithop` first then steps `desc_idx` (:1152-1170) — at-least-once
  on the def-of-in0 probe, then walks the snapshot, exactly the C++ shape. No reverse
  iterators in scope.
- **Erase-while-iterating**: clean. RuleTestSign collects `compareOps` first then
  rewrites (snapshot-then-mutate, matching C++ which builds `compareOps` via
  `findComparisons` before the rewrite loop). totalReplace is the funcdata-owned
  helper (out of scope).
- **Exception->Result parity**: clean. C++ `throw LowlevelError("Self-defined
  varnode")` in RulePropagateCopy -> `panic!` (:1446) (internal invariant, never a
  recover path). RuleCollapseConstants' C++ `try/catch(LowlevelError)` around
  `op->collapse` is the W6 seam (the whole collapse is deferred), so the catch site is
  not yet reachable.
- **AND-rule specifics** (transposed conditions / swapped slots / inverted returns /
  missing early-outs): re-derived rule-by-rule for all 17 fully-ported rules. All slot
  indices, boolean inversions (`negate=!negate` in BooleanNegate, `sgn=-sgn` in
  TestSign), early-outs, and the `multop2 = (multop1==in0.def)?in1.def:in0.def`
  selection in RuleBoolZext match C++ exactly. The `1-i`/`1-zside` index arithmetic in
  RuleAddMultCollapse / RuleSborrow / RuleScarry trivial+seam boundaries is correct.

## findings
- F1 (loss, accepted -> LOSS-056): five rules are cross-wave-seamed to no-op (entirely
  or on their non-trivial path) where the rewrite reaches an unported subsystem:
  RuleBooleanDedup (BooleanMatch::evaluate), RuleMultiCollapse
  (functionalEquality/cseFindInBlock), RuleSborrow / RuleScarry (AddExpression — only
  the trivial `*(V,0)=>false` case ported), RuleCollapseConstants (op->collapse),
  RuleTransformCpool (cpool store + updateType); plus RuleIndirectCollapse's STORE-guard
  arm (getStoreGuard, more conservative) and three minor symbol/print-hint annotations
  (XorCollapse/AddMultCollapse copySymbolIfValid, PropagateCopy setCopyImmed). Verified
  legitimate: `expression.rs` is a 1-line stub, the cpool store / op->collapse /
  store-guard surfaces are unwired. Each seam returns the C++ guard's conservative
  value (0) BEFORE any graph mutation (no partial state). Same accepted pattern as
  LOSS-053/054/055.
     cpp: decompiler/cpp/ruleaction.cc:2839,3299,3413,3492,3884,3922,3224
     rust: rust/crates/kuna-decomp/src/ruleaction_3.rs:287,747,789,833,1355,1393,672
- F2 (minor, informational — inherited C++ behavior, NOT a port-introduced logic
  divergence; not ledgered): RuleShift2Mult `let val = getOffset() as int4; if val>=32
  return 0; ... 1u64 << val` (:1141,:1176). For a shift-amount constant with the high
  int4 bit set (e.g. 0xFFFFFFFF), C++ `int4 val` is negative, passes `val>=32`, and
  computes `((uintb)1)<<val` which is UB; the Rust `1u64 << val` panics in debug. This
  mirrors the *identical* missing guard in C++ (both are broken on this malformed
  input), is unreachable for well-formed INT_LEFT shift amounts, and matches the C++
  guard structure verbatim — so it is faithful, not a divergence to fix. Flagged only
  for the record.
     cpp: decompiler/cpp/ruleaction.cc:3747-3766
     rust: rust/crates/kuna-decomp/src/ruleaction_3.rs:1141,1176

## Mechanical pass
- grep (added Rust): no `todo!`, no `unimplemented!`, no `HashMap`/`HashSet`, no
  `sort_unstable`. `as` casts (10 in production code) each carry an adjacent
  `// <C++ expr>` comment establishing the int4/uintb origin (faithful truncation);
  none are unjustified.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean (exit 0).
- port-audit: crate `port-audit` does not exist in this tree (optional check, skipped).

## Adversarial tests (committed on branch, commit 01f60fa)
Seven tests added to `ruleaction_3.rs`'s `#[cfg(test)] mod tests` (the item's crate;
the in-module harness `build_fd`/`mk_op`/`def_op`/`new_unique_out` is `pub(crate)`-gated
so the graph builders are reachable only here, not from an external `tests/` crate),
each prefixed `w5_s3_rules_3_`, all green:
- `w5_s3_rules_3_bool_zext_and_branch_rewrites_to_zext_booland` — RuleBoolZext INT_AND
  branch (the most complex rewrite: multop2 slot-selection, INT_AND->BOOL_AND mapping,
  unextended-boolean operands, zext-result re-extension).
- `w5_s3_rules_3_test_sign_minus_one_equal_to_sless` — RuleTestSign sgn=-1 via the
  raw `==calc_mask` offset (no complement) -> SLESS.
- `w5_s3_rules_3_test_sign_zero_equal_to_slessequal` — RuleTestSign sgn=1 via 0/EQUAL
  -> SLESSEQUAL (`0 s<= V`), the untested-by-porter arm.
- `w5_s3_rules_3_shiftpiece_exact_piece` — RuleShiftPiece concatsize==out*8 in-place
  PIECE(V,W).
- `w5_s3_rules_3_shiftpiece_swap_when_left_in_slot1` — RuleShiftPiece swap branch
  (INT_LEFT in slot1).
- `w5_s3_rules_3_shiftpiece_zext_tail` — RuleShiftPiece concatsize<out*8 inserts a new
  PIECE op and rewrites the original to ZEXT(newpiece).
- `w5_s3_rules_3_addmult_three_term_requires_input_spacebase` — RuleAddMultCollapse
  3-term branch conservative side (non-spacebase base -> no rewrite). (The fire side
  needs the `spacebase` flag, which has no public setter at W5; its rewrite is
  re-derived against C++ in the hunt-list and pinned indirectly by the nested-constant
  in-file tests.)

losses: LOSS-056
