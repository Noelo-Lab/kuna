# w10-union-scoring
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent)
date: 2026-06-15
branch: rport/w10-union-scoring (round 2)
gate: cargo test --workspace (debug) -> 3429 passed / 0 failed (3425 base-of-branch
      + 4 new verifier r2 fences). cargo clippy --lib -p kuna-decomp -> clean (also
      --tests clean for the new file). Rust decomp_test_dbg datatests -> 668 applied
      / 187 passing (>= 187 ✓). C++ oracle (kuna.run_tests --all --baseline
      docs/baseline.json) -> 207/207 unit + 675/675 datatest, PARITY OK (untouched,
      rust-only diff). byte-parity fences print_b5_boolless 11/0 + printc_parity 1/0
      green.

## Verdict rationale (round 2)

The round-1 REJECT had exactly ONE blocker (F1): the branch edited a reserved file,
`dtype.rs::get_type_union_impl`, to add `flags::needs_resolution`. Round 2 (commit
2d81595) reverted dtype.rs to base **byte-for-byte** (blob `5ce1563`, confirmed via
two-dot `git diff rust-port rport/w10-union-scoring -- dtype.rs` = empty). Fleet
discipline is now satisfied: NO net change to dtype.rs or printc.rs. The
`ScoreUnionFields` driver + the `resolveInFlow` cache population remain a faithful,
complete, verbatim transcription with no special-casing, no fabricated passes, and
zero regression on the shared cast/merge plane. That is a **faithful PARTIAL with
zero regression** -> ACCEPT-WITH-LOSSES per the gate rule.

The cost of clearing the blocker: reverting dtype.rs dropped the load-bearing
`needs_resolution` flag, so the scorer is now **dormant on the real corpus** (it
fires only for fixtures that set the flag themselves). This is the central loss
(LOSS-184) and is the correct tradeoff — re-adding the dtype.rs edit would itself be
a categorical REJECT. The feature is live-ready and activates with no further change
to this branch once the reserved-file owner lands the one-line flag.

## Findings

- F1 (RESOLVED — was the round-1 BLOCKER): dtype.rs is reverted to base
  byte-for-byte; the two-dot diff touches no reserved file. Cleared.
       was-cpp: decompiler/cpp/type.hh:625 ; was-rust: dtype.rs:4763

- F2 (LOW, accepted as cited loss): `finalize_exact_piece` keeps the prior
  `resolve` when `get_exact_piece` returns `None`, whereas C++ assigns
  unconditionally (can cache nullptr, unionresolve.cc:1183/1222). Round 2 added a
  full doc-comment citing why this is unobservable (SUBPIECE always hits the
  perfect-size early return; the implied-truncation consumer reads
  `getField(fieldNum)`, not `getDatatype()`). Accepted -> LOSS-184(a).
       cpp: decompiler/cpp/unionresolve.cc:1183, 1222
       rust: rust/crates/kuna-decomp/src/unionresolve_run.rs:1223 (finalize_exact_piece)

- F3 (LOW, info): the FLOAT-constant arm IS wired live (architecture.rs:703
  shares `OwnedFloatFormats` through `build_arch_handle`; the scorer's
  `get_arch().float_format(size)` resolves on the real lift path, None only for
  hand-built fixtures). The unionresolve_run.rs doc-comments at :303-352/:335-341
  still call float-format a residual loss — STALE doc, not a behavior gap.
       rust: rust/crates/kuna-decomp/src/unionresolve_run.rs:303-352

- F4 (info — the central PARTIAL): the scorer is DORMANT on the real corpus and
  the activation flag is currently in NO landed branch. Reverting dtype.rs dropped
  `flags |= needs_resolution`; without it, real unions never enter `resolveInFlow`,
  so the passing SET is identical (187->187, `comm` both empty) and the 33 Union +
  5 Partial-union datatests stay FAIL on both base and branch. VERIFIER CHECK: the
  named owner `rport/w10-bitfield` (dtype.rs:4984) ALSO sets only `type_incomplete`
  — it does NOT carry the flag today, so the 38-assertion flip is blocked on a
  one-line edit no landed branch contains. Faithful and zero-regression, but the
  orchestrator must route that one line to whichever wave owns the reserved
  `dtype.rs` union builder. -> LOSS-184.
       cpp: decompiler/cpp/type.hh:625 (TypeUnion(void) sets needs_resolution)
       rust: rust/crates/kuna-decomp/src/dtype.rs:4768 (sets only type_incomplete, == base)

- F5 (info, unobservable): TypeUnion address-based-hit caches the freshly-built
  `resolve` (the TypePointer form, :1325) rather than the source `*res` (:2551);
  same fieldNum, and for a plain union the resolve datatype (`getDepend(fieldNum)`)
  is identical to what the source record computed. -> LOSS-184(b).
       cpp: decompiler/cpp/type.cc:2551
       rust: rust/crates/kuna-decomp/src/funcdata_union.rs:430

## Hunt list

- Signedness: clean. Field-count loops over int4; `scores[(i+1) as usize]`; the
  `getOffset() >= base_size as uintb` array-arith checks mirror the C++ `uintb >=
  int4` promotion; `score_add_down` `off >= max_length as int8` and `vsize*8-1`,
  `score_ptradd_down` `align as int8 == in2off as int8` reproduce the C++
  int8/int4 promotions; `score_add_up`/`el_size` use the explicit C++ `(int4)`
  casts.
- Integer widths: clean. int4/int8/uintb/u32 map 1:1; `sign_extend(voff as i64,
  vsize*8-1)` matches the C++ 2-arg `sign_extend(intb,int4)` (address.hh:586) — the
  `voff as i64` is the same implicit reinterpret; the prior-wave body uses wrapping
  shifts.
- Wrapping: clean. Only uintb-lineage arithmetic is the sign_extend reinterpret;
  the down-chain `while off != 0` assigns the returned newoff (no new wrap site).
- Comparator totality: clean. `computeBestIndex` strict `>` keeps the lowest index
  (fenced); `VisitMark(VarnodeId,int4)` derives total `Ord`; visited is a BTreeSet.
- Iteration-order provenance: clean. trialCurrent/trialNext are C++ `list` ->
  `Vec` (insertion order); `run_one_level` take+iterate+restore is faithful (scoring
  only appends to trialNext); `beginDescend()` -> `descend_iter()` over a Vec. NO
  HashMap/HashSet in ported logic.
- Off-by-one / do-while / reverse: clean. `scoreTruncation`'s
  `while(ct && (curOff||sizeMismatch))` inverted to `while is_some()` + early break,
  keeping the `ct==0 -> -10` tail; the down-chain `while off!=0` keeps both guards;
  `run`'s `pass+1==maxPasses` last-pass arm and the maxTrials early-return (restore
  trialCurrent) match.
- Erase-while-iterating: N/A — the scorer reads only; the one cache write happens
  after `into_result()`.
- Exception -> Result partial-state: clean. The `?` points are type-factory
  builders the C++ never throws from; `propagate_type_edge`'s
  `unwrap_or_else(|_| clone(alttype))` leaves the same no-op-on-failure state; the
  two `.expect()` (down-trial has an op; Trial::new_down slot) match C++ unchecked
  derefs that are invariants at those sites.

## Mechanical pass

- todo!/unimplemented!/unreachable!: none in unionresolve_run.rs / funcdata_union.rs.
- HashMap/HashSet/sort_unstable in ported logic: none (one comment mention).
- bare `as` casts: index `as usize`, known-non-negative width conversions, or the
  C++ `(int4)`/`(int8)` mirrors; the one risky reinterpret carries `// cast:`.
- special-casing grep (function names / addresses / field names / magic constants):
  NONE.
- C++ blob-sha: unionresolve.cc `0d7edd1f...` / unionresolve.hh `b01db68c...` match
  the recorded shas — no drift.
- clippy -p kuna-decomp --lib and --tests: clean.

## Integration witness (REAL parity, not fabricated)

The driver is genuinely wired and executes end-to-end: `propagate_type_edge` calls
`resolve_in_flow` on `needsResolution` (coreaction.cc:5336-5341), and the two Casts
sites are converted from the seam `dt.resolve_in_flow()` (Err) to the live
`self.resolve_in_flow(&dt, ...)`. The 11 fences build a live Funcdata, wire ops,
resolve, and assert the cached field index is DATA-DRIVEN (FLOAT_ADD->float,
INT_ADD/INT_MULT->int, CBRANCH->bool, INT_ZEXT->uint-over-int, tie->lowest-index,
size-mismatch exclusion, SUBPIECE-truncation self-match). The only reason no
datatest flips is the dormant `needs_resolution` flag (F4), not a scorer defect.

## Adversarial tests (round 2, added + committed to rport/w10-union-scoring @ 06ae1f3)

rust/crates/kuna-decomp/tests/verify_w10_union_scoring_r2.rs (4 tests, all pass):
  - r2_cbranch_resolves_bool_field_not_whole_union — CBRANCH down arm + the
    `scores[0] -= 1` whole-union penalty; bool field (index 1) wins.
  - r2_int_zext_prefers_uint_over_int_field — INT_ZEXT pure-table arm discriminates
    UINT(+2) from INT(+1); the later index 1 wins the +1 margin.
  - r2_size_mismatch_field_is_excluded — the ctor `scores[i+1] -= 10` size-skip; only
    the size-matching field is trialled (read back via getUnionResolution, since
    getUnionField applies its own size check).
  - r2_subpiece_truncation_finds_matching_field — the SUBPIECE down arm drives
    scoreTruncation's union self-match bonus (+10+5), making the WHOLE union
    (fieldNum -1) win; a same-size sibling field does NOT spuriously win.
  (Two failed on my first guesses; investigation showed the PORT was correct and my
  expectations wrong — getUnionField's size check, and the ctor size-skip excluding
  the int2 field — so the corrected assertions are stronger fences. No port defect.)
The 7 prior fences (verify_w10_union_scoring.rs x3 + _adversarial.rs x4) re-run green.

## losses

LOSS-184 — faithful PARTIAL, zero regression, zero new datatest pass; the scorer is
dormant on the real corpus pending the `needs_resolution` flag landing via the
reserved-file owner (not yet in any branch), plus the field-access print plane; and
the two unobservable internal divergences (F2 getExactPiece-null, F5 address-based
caching). This item also closes the LOSS-138 "thread needsResolution/resolveInFlow"
deferral and the LOSS-127-family "ScoreUnionFields driver SEAM(W10) keystone".
