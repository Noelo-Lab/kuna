# w6-s5-double
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent verifier)
date: 2026-06-12 (round 2)
gate: `cd rust && cargo test -p kuna-decomp` -> double scope GREEN: lib 951 pass
      (all `double::tests::*`), `verify_w6_s5_double` 5/5. The ONE failure
      (`corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`: "Could not find .sla
      file for x86:LE:64:default") is a pre-existing, out-of-scope ENV gap — the
      gitignored built x86 `.sla` is absent in the worktree (present in the main
      tree; needs `make specs`). It is on the rust-port base, does not reference
      double/SplitVarnode, and `double.rs` is the only changed source file.

## Round 1 -> Round 2
Round 1 REJECTed on **F1**: `Equal2Form::replace` and `LessThreeWay::set_bool_op`
ran the prepare step (`is_whole_feasible`, which mutates `whole`/`defblock`/
`defpoint`) on a *throwaway clone* and then the rewrite step on a *second fresh
clone*, discarding the discovery and fabricating a redundant/mis-located concat
whole (wrong IR). **Round 2 FIXES F1**, and I independently re-derived and
confirmed the fix (see below). No new blocker found; the residual is the
already-documented conservative cross-wave seam family, now ledgered as LOSS-084.

## F1 — FIXED, independently re-verified
C++ oracle: `Equal2Form::replace` calls `prepareBoolOp(in,param2,..)` which mutates
the class MEMBERS `in`/`param2`; `applyRule` then calls `replaceBoolOp(..,in,param2,..)`
on those SAME members (double.cc:1926/1933, :1978). `LessThreeWay::setBoolOp` ->
`prepareBoolOp(in,in2,..)` mutates members; `applyRule` -> `createBoolOp(..,in,in2,..)`
reuses them (double.cc:2419-2425, :2487-2490). `findCreateWhole` reuses an existing
whole via `if (whole != 0) return;` (double.cc:512).

Round-2 fix (verified faithful): `replace`/`set_bool_op` now thread ONE clone pair
through `prepare_bool_op` and persist it back into `self.in_`/`self.param2`
(`self.in2`) — double.rs:2703-2718, 3414-3426 — and `apply_rule` reuses the
persisted members in `replace_bool_op`/`create_bool_op` (double.rs:2771, 3504-3514).
`find_create_whole`'s `if self.whole.is_some() { return }` (double.rs:973) then
reuses the discovered whole. I audited EVERY prepare->create site:
- AddForm/SubForm/MultForm/ShiftForm: `prepare_*` on `&mut self.<member>` directly
  (persists), then clone for `create_*` (double.rs:2159/2344/2510/4309/3905/3931).
- Equal1Form/Equal3Form/LessConstForm/PhiForm/IndirectForm: prepare+create/replace
  on the SAME local pair within one scope, written back (double.rs:2615-2650/
  2861-2867/3597-3613/4456-4462/4590-4596).
No other clone-discard site remains. The clone-into-locals pattern is forced by the
borrow checker (two `&mut` fields of `self`), but the prepared state is always
written back before/after the create step — behaviorally equivalent to the C++
member mutation. The three porter F1-fix regression tests pass.

## hunt list
- **Signedness**: clean. `(uintb)(w.size()-h.size())` int4-subtract-then-cast faithful
  (double.rs:488,691,1327); `val >= 8*lo.size()` int4->uintb promotion preserved
  (:3810); `getSeqNum().getOrder()` compared as `u32` (unsigned). No int4-index-vs-size.
- **Integer widths**: clean. uintb=u64, int4=i32, SIZEOF_UINTB=8==sizeof(uintb);
  `exceedsConstPrecision` (`wholesize > 8`) faithful (:464).
- **Wrapping**: clean. `adjacent_offsets` `wrapping_add` (:1142,1176). `checkForCarry`
  `(!negconst)&calc_mask` (:1959). `normalize_hi/mid/lo` `wrapping_add(inc as uintb)` /
  `(-1i64) as uintb` reproduce the C++ `int4 -1`->uintb promotion-and-wrap (:3156,3213,3262);
  `wshl/wshr`=wrapping shift=x86 `<<(n&63)`==C++ shift-UB-on-x86. `verify_shift_amount`'s
  `8*losize - val` plain `-` is guarded by a prior `val >= 8*losize` reject (no underflow).
- **Comparator totality**: N/A — no operator</sort/partial_cmp/BTree in double.rs (clean).
- **Iteration-order provenance**: clean. Every loop is `descend_ops(vn)` (C++
  `list<PcodeOp*> descend` insertion order, reproduced by the SmallVec descend list) or
  `0..numInput`. `whole_list`/`find_copies`/`applyRuleIn`/`attemptMarking`/`findHiMatch`
  preserve order + break-on-first-unique. No HashMap/HashSet.
- **Off-by-one / do-while / reverse iters**: clean. No do-while/rbegin. `findDefinitionPoint`
  TWO-PHASE cross-block dominance (hi-block dominated by lo-block, then reverse) faithful
  (:768-790 == double.cc:346-362). `findWholeBuiltFromPieces` earliest-by-order faithful.
- **Erase-while-iterating**: clean. `applyRuleIn` snapshots the descend list before the
  form dispatch; forms mutate only on success then return 1 (`op_is_dead` guard is inert).
  `reassignIndirects` two-pass mark/destroy then clear/move faithful (:5244-5271).
- **Exception -> Result partial-state**: clean. All seam declines bail BEFORE any op-graph
  mutation (prepare* are read-only probes; create* run only after success). C++ throws map
  to Rust panic! on the same unreachable-on-valid-SSA conditions.
- **(this wave) Datatype::compare / ParamEntry/ParamTrial sort / TypeFactory cache**: N/A —
  double.cc contains none of these.

## findings
  - F1 (was BLOCKER, now FIXED): clone-discard whole-feasibility in Equal2Form/
    LessThreeWay. Round-2 fix re-verified faithful; no residual.
       cpp: decompiler/cpp/double.cc:1926,1978,2419-2425,2487-2490
       rust: rust/crates/kuna-decomp/src/double.rs:2703-2718,2771,3414-3426,3504-3514
  - F2 (loss, LOSS-084): W6 `op_is_arith_or_float`->false disables BOTH
    `RuleDoubleIn` and `RuleDoubleOut::attemptMarking` for the dominant written/
    PIECE-read whole (the `TypeOp::addlflags` arith/float word is absent from the W3
    `TypeOp` skeleton). Recovery never STARTS for that shape — conservative decline
    (C++ marks + returns 1). Single most behavior-shrinking loss; pinned by my
    `..._arith_whole_marking_disabled_by_w6_seam`.
       cpp: decompiler/cpp/double.cc:3238-3240,3318-3320
       rust: rust/crates/kuna-decomp/src/double.rs:4733-4736,4788,4912
  - F3 (loss, LOSS-084): W4 `getSpaceFromConst`/`getOpFromConst`/`combineInputVarnodes`
    seams make `testContiguousPointers`->None (so `RuleDoubleLoad`/`RuleDoubleStore`
    decline at their first reachable line) and `RuleDoubleOut`'s input-collapse
    decline. Full transform bodies are transcribed below each seam; bails are
    read-only (no partial state). `set_opcode_seam` W6 flag-word=0 is benign for
    dispatch.
       cpp: decompiler/cpp/double.cc:758-759,3353,3473,3530,3087,3598
       rust: rust/crates/kuna-decomp/src/double.rs:1190-1193,1208,4875-4881,4487-4491,5126-5129
  - F4 (loss, LOSS-084, INERT today): the C++ `Varnode::getSymbolEntry` symbol-mismatch
    rejects are OMITTED in TWO places — `is_addr_tied_contiguous` (double.cc:795-803)
    and `RuleDoubleOut::attempt_marking` (double.cc:3306-3313). "Less conservative"
    (could accept a pair C++ rejects), BUT VERIFIED INERT: the Varnode model in this
    tree has NO symbol-entry surface at all (grep: no `getSymbolEntry`/`SymbolEntry`/
    `mapentry` in varnode.rs), so C++-at-this-maturity would also see null entries and
    skip the rejects — both return the same result for every input today. The
    `attempt_marking` site is additionally masked by F2 (the arith/float loop declines
    first); the `is_addr_tied_contiguous` site is reachable via CopyForceForm (live)
    only once F2's marking is restored. Must be restored together with the W4 symbol
    table.
       cpp: decompiler/cpp/double.cc:795-803,3306-3313
       rust: rust/crates/kuna-decomp/src/double.rs:1269-1275,4903-4907

## adversarial tests (rust/crates/kuna-decomp/tests/verify_w6_s5_double.rs, 5/5 pass)
  - verify_w6_s5_double_arith_whole_marking_disabled_by_w6_seam (F2 regression guard:
    pins the W6-seam decline against the C++ oracle that marks + returns 1)
  - verify_w6_s5_double_free_whole_rejected_before_seam (!isWritten guard, double.cc:3232)
  - verify_w6_s5_double_non_half_truncation_rejected (offset*2!=whole.size, double.cc:3228)
  - verify_w6_s5_double_reset_sets_recovery_flag (setDoublePrecisRecovery, double.cc:3201)
  - verify_w6_s5_double_exceeds_const_precision_boundary (sizeof(uintb)==8 off-by-one)
(The porter's round-1 F1-proof tests live in `double.rs mod tests`:
clonediscard_fabricates_redundant_whole, find_create_whole_reuses..., equal2form/
lessthreeway_threads_prepared_whole, adjacent_offsets_wraps..., check_for_carry_negconst_masking.)

## mechanical pass
  - greps: no todo!/unimplemented!/HashMap/HashSet/sort_unstable/partial_cmp/BTreeMap in
    double.rs. Every numeric bare `as` cast has a verified C++ counterpart; the rest are doc text.
  - `cargo clippy -p kuna-decomp` — clean (0 warnings).
  - `port-audit` — NOT present in this tree (`package port-audit not found`); skipped.

## disposition
ACCEPT-WITH-LOSSES. F1 (the round-1 blocker) is fixed and re-verified; the ported logic
(discovery, feasibility, whole-creation, all 14 forms, the op-builders) is faithful and
independently re-derived. The remaining divergences (F2/F3/F4) are conservative /
currently-inert cross-wave seams, all facets of the appended **LOSS-084**.
