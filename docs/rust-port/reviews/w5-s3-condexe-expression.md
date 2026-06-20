# w5-s3-condexe-expression
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier subagent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (473 lib + integration bins, incl. 6 new verifier tests; 0 failed)

## Scope

Port item commit `ab1b2da` only (the branch is stacked on `w5-dtype-expand`;
`dtype.rs` / `verify_w5_dtype_expand.rs` belong to that separate item and were
excluded from this review). In-scope diff: `condexe.rs` (+1336), `expression.rs`
(+1111), and their `tests.rs`. cpp_blob_sha for all four modules verified against
the working tree — no drift.

## Re-derivation (function-by-function)

expression.cc — `BooleanMatch::evaluate` (incl. the depth/op1==null ordering, the
BOOL_NEGATE flip, the three-way bool-binary mix, the direct-compare `else`, and
the `get_booleanflip` reorder path), `sameOpComplement`, `varnodeSame`,
`BooleanExpressionMatch::verifyCondition`, `TermOrder::collect`/`sortTerms`/
`additiveCompare`, `Varnode::termOrder` (local W3 seam), `functionalEqualityLevel0`/
`functionalEqualityLevel`/`functionalEquality`/`functionalDifference`,
`rootPointer`/`pointerEquality`, `TraverseNode::isAlternatePathValid`, `PcodeOpNode`:
all re-derived line-by-line against the Rust and found faithful.

condexe.cc — `buildHeritageArray`, `testIBlock`, `findInitPre`, `verifySameCondition`,
`testMultiRead`, `testOpRead`, `findPullback`, `pullbackOp`, `getNewMulti`,
`resolveRead`, `resolveIblockRead`, `getMultiequalRead`, `getReplacementRead`,
`doReplacement`, `testRemovability`, `verify`, `trial`, `execute`,
`ActionConditionalExe::apply`, `RuleOrPredicate::*` (`discoverZeroSlot`/
`discoverCbranch`/`discoverPathIsTrue`/`discoverConditionalZero`/`checkSingle`/
`applyOp`): all re-derived. Verify/discovery path is exact; mutation path is exact
modulo the documented seams (LOSS-067).

## Mandatory hunt list

- **Signedness**: clean. `1-constslot`, `1-prea_inslot`, `1-posta_outslot`,
  `1-camethruposta_slot`, `1-zeroSlot` all operate on 0/1 ints (no underflow);
  `inrev == posta_outslot` is int4==int4; `int4` loop indices vs `num_input()`
  (int4) are like-typed. Adversarially tested the signed `sameOpComplement` corner.
- **Integer widths**: clean. int4/uintb mappings consistent; no implicit-promotion
  divergence. `terms.len() as int4`, `pullback.len() as int4`, index `as usize`
  are arena-index conversions, faithful.
- **Wrapping**: clean. `sameOpComplement` `val1 + 1` -> `val1.wrapping_add(1)`
  (expression.cc:77); `rootPointer` offset accumulation -> `wrapping_add`
  (matches uintb). Adversarially tested the `val1+1` wrap-to-0 corner.
- **Comparator totality**: clean. `additiveCompare`/`sort_terms` reconstructs a
  full `Ordering` from the total `termOrder` (const-ness then address) and is
  consistent with the C++ `<` predicate. `PcodeOpNode::less` (SeqNum time then
  slot) and `compareOrder` are total. No `partial_cmp().unwrap()` on floats.
  Note: C++ `std::sort` is unstable; Rust uses stable `sort_by` — a valid
  realization since the C++ tie order is unspecified (informational, not a finding).
- **Iteration-order provenance**: clean. `replacement` is `BTreeMap<int4,_>` keyed
  by block index = C++ `std::map<int4,_>` key order (lookup-only, never iterated);
  `pullback` is index-keyed `Vec`; `descend_iter().collect()` snapshots preserve
  C++ `list` order before mutate. **No HashMap/HashSet anywhere** (grep clean).
- **Off-by-one / do-while / reverse iterators**: clean. `verify`'s reverse op
  scan (skip branch, iterate predecessor->head inclusive) and `execute`'s do-while
  (snapshot-predecessor-before-destroy, at-least-once) both match C++ exactly.
- **Erase-while-iterating**: clean. `doReplacement` re-fetches the first descendant
  each iteration (C++ `iter = vn->beginDescend()` re-fetch); `execute` snapshots
  the predecessor before `op_destroy`. `testRemovability` collects descenders into
  a Vec before iterating (read-only, order preserved).
- **Exception -> Result partial-state parity**: FINDING F1 (loss, not blocker) —
  see below.

## Mechanical pass

- grep `todo!`/`unimplemented!`: none (the `panic!`s are test-helper collision
  asserts). `HashMap`/`HashSet`: none. `sort_unstable`: none. Bare `as`: only
  `int4`<->`usize` arena-index conversions (no `// cast:` needed; semantically
  faithful, no truncation in practice).
- `cargo clippy -p kuna-decomp -- -D warnings`: clean (no output).

## Adversarial tests (committed to branch, `tests/verify_w5_s3_condexe_expression.rs`)

All 6 PASS against the port, confirming parity on the fragile spots:
- `w5_s3_sameop_complement_unsigned_val2_zero_corner_is_uncorrelated` — the
  `val1+1` wrap-to-0 + `val2==0` reject (expression.cc:77-78).
- `w5_s3_sameop_complement_signed_sign_boundary_corner_is_uncorrelated` (+ an
  in-range control) — the signed sign-bit corner (expression.cc:79-83).
- `w5_s3_funceqlevel_commutative_swap_prefers_then_swaps_ordering` — the
  `unmatchsize==2` "prefer original" res2 bookkeeping (expression.cc:698-700).
- `w5_s3_funceqlevel_commutative_locks_first_pair_via_flip` — the `comm2==0`
  leftover-pair rewrite `res2[0]=res2[1]` (expression.cc:694-696).
- `w5_s3_evaluate_double_negate_round_trips_to_same` — flip-applied-twice.
- `w5_s3_evaluate_unwritten_vn1_with_negate_vn2_recurses` — the `op1==0` early-out
  deferred past the vn2 BOOL_NEGATE branch (expression.cc:130-148).

## Findings

- F1 (loss / partial-state divergence, NOT a blocker): `ConditionalExecution::execute`
  destroys iblock ops in a reverse loop and `pullbackOp`/`getNewMulti`/`doReplacement`/
  `RuleOrPredicate::checkSingle`/`applyOp` create an orphan op (and orphan free
  varnode) *before* the `op_set_output?` / `resolve_typeop?` seam `Err` bails. So a
  seam `Err` mid-execute leaves PARTIAL graph mutation, which `apply` then silently
  treats as "no change". C++ completes the transform here. Caused entirely by the
  W3 `opSetOutput` (LOSS-035/036) + W6 `opSetOpcode` op-info seams, which are
  out-of-scope dependencies. Unreachable at this boundary (the action/rule are not
  wired into any `ActionPool`/`universalAction`; configs needing no new-op creation
  succeed cleanly). Surfacing `Err` is the correct conservative choice. -> LOSS-067.
       cpp: decompiler/cpp/condexe.cc:160,198,320,456,637
       rust: rust/crates/kuna-decomp/src/condexe.rs:358,414,579,765,1120,1193
- F2 (loss, NOT a blocker): the `AddExpression` and bitfield-expression family
  (`BitFieldExpression`/`InsertExpression`/`InsertStoreExpression`/`PullExpression`)
  from expression.{cc,hh} are not ported (deref W6 type lattice + W4 symbol surface;
  no in-tree consumer; AddExpression's only consumers are the already-deferred
  LOSS-056 rules). -> LOSS-066.
       cpp: decompiler/cpp/expression.cc:300,403,477,522,551
       rust: rust/crates/kuna-decomp/src/expression.rs:23

## Conclusion

The in-scope algorithmic content — the entire verify/discovery surface of CONDEXE
and the complete boolean-match / functional-equality / term-order / pointer-equality
machinery — is a faithful, total, wrap- and signedness-correct port, independently
re-derived and adversarially tested with parity confirmed. The two divergences (F1,
F2) are documented cross-wave seams with no consumer at this item's boundary, now
ledgered as LOSS-066 and LOSS-067. No blocker or major stands.

losses: LOSS-066, LOSS-067
