# w10-union-render
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent, round 2)
date: 2026-06-16
gate: `cd rust && cargo test --workspace --no-fail-fast` -> 3526 passed, 0 failed;
      `cargo test -p kuna-decomp` -> 2673 passed, 0 failed;
      `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean

## Round-2 disposition

Round 1 REJECTed on **F1**: `scoreTrialUp` had no `CPUI_PTRADD` arm, so a union
field defined by `CPUI_PTRADD` fell into the driver `_ => -10` instead of the C++
`{10,2,-5,1}`-by-metatype scoring (`unionresolve.cc:900-912`).  Commit `901ccef`
on this branch adds the PTRADD-up arm **and** the SUBPIECE-up arm
(`unionresolve.cc:890-898`).  Both now match the C++ statement-for-statement.
The round-1 repro `w10_ptradd_up_ptr_field_scores_positive_like_cpp` (which FAILED
on the round-1 branch) now PASSES; the F1 defect is genuinely resolved (not
suppressed).  No new defects; the residual gaps are honestly-disclosed faithful
losses, hence ACCEPT-WITH-LOSSES rather than ACCEPT.

## Scope of the diff

rust/ only (porter commits: 7 src + 2 test files; +1 verifier test file).
`git diff rust-port...rport/w10-union-render -- decompiler/ specs/` is EMPTY — the
C++ decompiler tree and SLEIGH specs are byte-identical to rust-port, so the C++
oracle's 675/675 PARITY OK is preserved by construction.  No reserved file is
touched: confirmed `funcdata_varnode.rs`, the ActionVarnodeProps coreaction file,
`merge.rs`, `variable.rs`, the printc float-constant arms, and the ruleaction
SBORROW/bitfield families are all absent from the diff (`printc.rs` IS edited but
ONLY the union/implied-field render — grep for `INFINITY|NAN|FLOAT_UNKNOWN|
float_class|func_|BADTYPE` over the added lines returns only `op_func_ir`
fallback calls, no float-render code).

## The mandatory hunt list

- **Signedness:** clean (3 deliberate casts, all C++-faithful).
  - `align as int8 == in2off as int8` (`unionresolve_run.rs:718` PTRADD-down,
    `:838` PTRADD-up): C++ `-Wno-sign-compare` promotes `int4 alignSize` to
    `uintb` and compares unsigned; Rust casts both to signed `int8`.  Agrees for
    all realistic (non-negative align + non-negative offset) values; `getAlignSize`
    never returns negative.  Inline comment present.  Noted, not a finding.
  - `align == v.get_offset() as int4` (`score_add_up`, `:883`): faithful to the
    explicit C++ `(int4)def->getIn(1)->getOffset()` cast.
  - `v.get_offset() >= base_size as uintb` (`test_array_arithmetic`, `:204/221/232`)
    == C++ `vn->getOffset() >= result.baseType->getSize()` (uintb vs int4→uintb);
    exact for non-negative size.
- **Integer widths:** clean.  `vec![..; (num_fields + 1) as usize]`, `[i+1]`
  indexing, `score_index as usize` — all index/size casts on `int4` C++ operands.
- **Wrapping:** clean.  No raw `+`/`-` on `uintb`-lineage values in the new code;
  the only arithmetic is `scores[idx] += score` (int4 accumulation, C++ identical)
  and the down-chain delegations (already-verified leaf surface).
- **Comparator totality:** clean.  `compute_best_index` (leaf) is strict `>` with
  lowest-index tie-break == C++ `computeBestIndex` (`unionresolve.cc:1050-1063`),
  fenced by `compute_best_index_ties_keep_lowest`.  `VisitMark` derives `Ord`
  (total) but only feeds `BTreeSet` membership (`insert().second`), never a score
  — pointer-vs-id order unobservable.
- **Iteration-order provenance:** clean.  C++ `list<Trial> trialCurrent/trialNext`
  (insertion order) -> Rust `Vec<Trial>` (insertion order); `run()` swap +
  `trial_next.clear()` faithfully mirrored.  `run_one_level`'s `std::mem::take`
  snapshot + restore is sound: `score_trial_down/up` push ONLY to `trial_next`
  (verified `new_trials_down`/`new_trials` both append to `trial_next`), never to
  `trial_current`, so iterating a moved-out snapshot reproduces the C++
  const-iterator exactly.  C++ `set<VisitMark>` -> `BTreeSet`, no `HashMap`/
  `HashSet` anywhere (grep clean).  `vn->beginDescend()` order reproduced by
  `descend_iter` over `Vec` (shared W6 surface, out of scope).
- **Off-by-one / do-while / reverse iterators:** clean.  No `do{}while`, no reverse
  iterators.  `for i in 0..numFields` + `i+1` indexing == C++.  `for pass in
  0..MAX_PASSES` with `pass+1==MAX_PASSES` last-pass test == C++.
- **Erase-while-iterating:** n/a.  The scorer never erases during traversal.
- **Exception -> Result partial-state:** the printc union no-resolve / non-zero
  PTRSUB-offset paths replace C++ `throw LowlevelError` (+`clear()`) with the
  generic `op_func_ir` functional render so output stays parseable; this matches
  existing printc precedents and never special-cases — benign, noted, not a
  finding.  `resolve_in_flow` `?`-propagation points carry no mid-mutation state.

## findings

  - F1 (correctness, BLOCKER) — **RESOLVED in round 2** (commit `901ccef`).  The
    `scoreTrialUp` `CPUI_PTRADD` arm is now present and matches C++:
       rust: rust/crates/kuna-decomp/src/unionresolve_run.rs:819-853
       cpp:  decompiler/cpp/unionresolve.cc:900-912
    plus the SUBPIECE-up arm (rust :798-818 == cpp :890-898).  Verified by the
    round-1 repro (now PASS) and three round-2 sub-branch tests (below).

## Faithfulness (re-confirmed)

- `run`/`run_one_level`/`compute_best_index`/`new_trials`/`new_trials_down`:
  statement parity with C++ (`unionresolve.cc:297-1085`).
- `score_trial_down`: pure/driver split complete — every non-pure arm
  (COPY/MULTIEQUAL/INDIRECT, LOAD, STORE inslot 1/2, CALL*/CALLIND/RETURN,
  INT_ADD/INT_SUB/PTRSUB, SUBPIECE, PTRADD) present; rest is the leaf pure table.
- `score_trial_up`: NOW complete — COPY/MULTIEQUAL/INDIRECT, LOAD, CALL*, INT_ADD/
  INT_SUB/PTRSUB, **SUBPIECE**, **PTRADD**; the F1 holes are filled.
- 3 constructors (`new_edge`/`new_subpiece`/`new_truncation`), `test_simple_cases`/
  `test_array_arithmetic`, `score_add_down`/`score_ptradd_down`/`score_add_up`,
  `score_constant_fit`, `subpiece_composite_byte_offset`, `finalize_exact_piece`:
  faithful (cross-checked against C++ source line-by-line).
- `dtype.rs`: `TypeUnion` now constructed with `needs_resolution` (C++ `TypeUnion()`
  ctor, type.hh:625) — the load-bearing flip that makes every union resolvable;
  applies to ALL unions, not a special case.
- `funcdata_resolveflow.rs`: `resolve_in_flow` dispatch (TypePointer/TypeUnion/
  TypeArray/TypeStruct/TypePartialUnion overrides) + `score_single_component`
  faithful.  The pointer/union arms are unified; the one C++ asymmetry (TypePointer
  caches the rebuilt `resolve`, TypeUnion caches `*res`) is collapsed to the
  rebuilt resolution — same `fieldNum`, equivalent cached datatype; address-based
  path rarely hit.  Acceptable.
- `coreaction_casts.rs`: the two cast-plane callers now route the real
  `Funcdata::resolve_in_flow` driver (was the W6 SEAM stub `Datatype::
  resolve_in_flow` that ERRORED for unions and was silently swallowed by `let _`).
  This is the enabling change.  The old `Datatype::resolve_in_flow` stub remains
  but is now referenced only by its own dtype.rs unit tests (no production caller)
  — minor dead-ish surface, harmless, not a finding.
- printc union arm (`op_ptrsub`): reads `fd.get_union_field(ptype, op, -1)` ->
  `ct.get_field(resUnion.getFieldNum())`; field name/type/id come from the SCORED
  union, not hardcoded.  `push_implied_field_ir` (the value-member `.field` render)
  is faithful to printc.cc:2161-2192 and gated on the real `Varnode::
  has_implied_field` flag.

## NO special-casing

Grepped every production file for field-name / type-name / address /
function-name hardcoding (`field0|intfield|bval|myint|union_datatype|0x40...`,
filtered for the union logic): **none**.  The only string literals are pre-existing
printc keyword/opcode tables (untouched lines) and synthetic addresses in the test
fixtures.  The rendered field is always `ct.get_field(field_num)` where `field_num`
is the scored resolution.  Confirmed.

## NO regression (passing-set diff)

Built the merge-base (`37167d5`) in a parallel worktree and ran
`cargo test --workspace --no-fail-fast` on both trees.  The base worktree shows 81
*environmental* failures (it lacks the built `.sla` specs + the C++ oracle binary
the corpus/golden tests need) — these are not real base regressions.  The
definitive check is the passing-SET diff:
  - `comm -23 base_pass branch_pass` (tests passing on base but NOT on branch) ==
    **EMPTY** — zero tests dropped.
  - branch adds the new union tests + (environmentally) the spec/oracle-dependent
    tests that only pass where the specs are built.
Byte-identical anchors `boolless` (b3/b4/print/positive-datatest), `readstruct`
(symbol-naming body), `condconst_conn` (dc/eob/implied byte-identical), and
`corpus_functions_byte_compare_against_cpp_oracle` all PASS on the branch.
clippy `-p kuna-decomp --lib -- -D warnings` clean (fresh compile).  C++ tree
byte-identical -> oracle 675/675 PARITY OK untouched.

## Real-parity of the new passes

- `verify_w10_union_scoring.rs` (3 tests): genuinely drive `ScoreUnionFields::
  new_edge` + `resolve_in_flow` on a live Funcdata graph — a union read DOWN into
  FLOAT_ADD resolves to the float field (idx 1), into INT_ADD to the int field
  (idx 0), via real scoring + cache write; the field is the scorer's choice, NOT
  hardcoded.  Non-vacuous.
- `verify_w10_union_render_ptradd_up.rs` (3 tests, round-1): the F1 repro now
  PASSES (fieldNum 1 via the +10 PTRADD-up arm); control + boundary PASS.
- `verify_w10_union_render_r2.rs` (3 tests, round-2, this verdict): pin the
  PTRADD-up SUB-branches the round-1 repro did not cover — align-mismatch (+2,
  still wins), pointer-beats-float (the -5 doesn't-fit branch), and the SUBPIECE-up
  likely-truncation arm (+3).  All assert the C++ outcome and PASS.

## adversarial tests (round 2, committed `cf9b570`)
  rust/crates/kuna-decomp/tests/verify_w10_union_render_r2.rs:
  - w10r2_ptradd_up_align_mismatch_still_scores_positive (PASS — +2 branch)
  - w10r2_ptradd_up_ptr_field_beats_float_field (PASS — isolates the -5 branch)
  - w10r2_subpiece_up_int_field_scores_likely_truncation (PASS — SUBPIECE-up +3)

## losses (ACCEPT-WITH-LOSSES)

Faithful PARTIAL, honestly disclosed in-code, zero regression:
  - L-W10UR-1 (partial-union truncation walk): `resolve_in_flow_partial` takes the
    C++ `stripped` fall-through (type.cc:2971-2973) instead of wiring
    `TypeUnion::resolveTruncation` (the SUBPIECE / implied-truncation scorer
    constructors).  `// SEAM(W10 partial-truncation)`.  `partialunion.xml`'s simple
    accesses resolve through the cache the cast plane already populated.
  - L-W10UR-2 (implied value-member render): `push_implied_field_ir` is the
    faithful port of printc.cc:2161-2192 but is INERT until a merge-stage
    HighVariable retains the needs-resolution union type at print time (a surface
    owned elsewhere); it never changes a render today (gated on
    `has_implied_field`) and lights up the `glob.intfield` / `(ptr->value).myint`
    value-member renders once that retention lands.
  - L-W10UR-3 (B5 end-to-end not asserted): `union_datatype.xml` (the B5 oracle
    with the real `ptr[i].ifield` / `glob.intfield` / `(ptr->value).myfloat`
    renders) is NOT yet wired into a Rust datatest harness, so end-to-end B5
    parity is asserted on neither side.  The PTRSUB `ptr->field` render PATH is
    wired and faithful; the assertion fence is future work.
These are recorded in docs/rust-port/losses.md.
