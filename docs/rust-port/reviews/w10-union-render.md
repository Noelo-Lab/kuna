# w10-union-render
verdict: REJECT
verifier: opus-4.8 (independent)
date: 2026-06-15
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (2667 passed, 0 failed); whole-suite `cargo test --workspace --no-fail-fast` -> 3520 passed, 0 failed

## Summary

Faithful, careful transcription of `ScoreUnionFields::run`/`runOneLevel`/
`computeBestIndex`/`newTrials*`/`scoreTrialDown`/`scoreTrialUp` + the three
constructors (`unionresolve_run.rs`), `Funcdata::resolveInFlow`
(`funcdata_resolveflow.rs`), and the printc `op_ptrsub` union arm (`printc.rs`).
No special-casing. No reserved files touched. No parity regression. C++ oracle
untouched.

BUT the `scoreTrialUp` opcode partition drops one whole switch arm: a union
up-trial whose Varnode is defined by `CPUI_PTRADD` scores `-10` (the driver
`_ =>` default) instead of the C++ `{10, 2, -5, 1}` by metatype. This is an
undocumented (not a cited loss) unfaithful transcription in the core scoring
engine that is the heart of this item. REJECT with F1 as the work order.

## The mandatory hunt list

- Signedness: clean. The one in/out comparison `align as int8 == in2off as int8`
  (`unionresolve_run.rs:718`, PTRADD-DOWN arm) casts both to signed `int8`; C++
  (`-Wno-sign-compare`) promotes the `int4 alignSize` to `uintb` and compares
  unsigned. Agrees for all realistic (small positive align + small positive
  offset) values; no observed divergence. Noted, not a finding.
- Integer widths: clean. `sign_extend(voff as i64, vsize*8-1)` (`:615`) mirrors
  C++ `sign_extend(vn->getOffset(), vn->getSize()*8-1)` with a `// cast:` note;
  `el_size = mv.get_offset() as int4` (`:660`) == C++ `(int4)multVn->getOffset()`.
- Wrapping: clean. The only arithmetic on offset-lineage values is the
  `downChain` loop (`while off != 0`) which delegates to the (already-verified)
  `down_chain`; no raw `+`/`-` on `uintb` lineage in the new code.
- Comparator totality: clean. `compute_best_index` (leaf, W6) is strict `>` with
  lowest-index tie-break == C++ `computeBestIndex`. `VisitMark` ordering only
  feeds set membership, never a score.
- Iteration-order provenance: clean. C++ `list<Trial> trialCurrent/trialNext`
  (insertion order) -> Rust `Vec<Trial>` (insertion order); `run()` swap +
  `trialNext.clear()` faithfully mirrored. `visited` C++ `set<VisitMark>` ->
  `BTreeSet<(VarnodeId,int4)>`; only "already inserted?" is observable, so the
  pointer-vs-id order is unobservable. `vn->beginDescend()` order is reproduced
  by `descend_iter` (shared W6 surface, out of scope).
- Off-by-one / do-while / reverse iterators: clean. No `do{}while`, no reverse
  iterators in scope. `for i in 0..numFields` + `i+1` indexing matches C++.
- Erase-while-iterating: n/a. The scorer never erases from a container during
  traversal; `scoreTrial*` append only to `trialNext`. `run_one_level`'s
  `std::mem::take` snapshot + restore is faithful (scoring never mutates
  `trial_current`).
- Exception -> Result partial-state: F1 below is the one place the C++ control
  flow is not reproduced (a dropped switch arm, not a `?` mismatch). Separately,
  the printc union no-resolve path replaces C++ `throw LowlevelError` + `clear()`
  with the generic `op_func_ir` fallback (17 existing precedents in printc.rs) —
  a benign, non-special-casing divergence; noted, not a finding.

## findings

  - F1 (correctness, BLOCKER): `scoreTrialUp` has no `CPUI_PTRADD` arm. The
    Rust up-pure table correctly returns `None` for PTRADD (it needs the live
    read `def->getIn(2)->getOffset()`), routing it to the driver `match` — but
    the driver `match` has no PTRADD case, so a PTRADD-defined up-trial falls
    into `_ => score = -10`. C++ scores it `+10` (align match) / `+2` (PTR),
    `-5` (ARRAY/STRUCT/UNION/CODE/FLOAT), or `+1` (else). Wrong sign for a
    pointer field. Affects exactly the `ptr[i].field` array-of-union accesses
    (`union_datatype.xml` Union #1/#2) — though that corpus is not yet wired
    into a Rust test, so no current assertion catches it. Undocumented (not in
    the item's declared losses).
       cpp: decompiler/cpp/unionresolve.cc:900-912
       rust: rust/crates/kuna-decomp/src/unionresolve_run.rs:767-801 (missing
             CPUI_PTRADD arm; `_ =>` at :798); the deferral is
             rust/crates/kuna-decomp/src/unionresolve.rs:1342-1344
    Fix: add to the `score_trial_up` driver `match`:
       CPUI_PTRADD => {
         if meta == TYPE_PTR {
           let align = trial.fit_type.get_ptr_to().map(|p| p.get_align_size()).unwrap_or(-1);
           let in2off = def->getIn(2)->getOffset();
           score = if align as int8 == in2off as int8 { 10 } else { 2 };
         } else if ARRAY|STRUCT|UNION|CODE|FLOAT { -5 } else { 1 }
       }
    Repro: tests/verify_w10_union_render_ptradd_up.rs ::
    w10_ptradd_up_ptr_field_scores_positive_like_cpp FAILS on the branch
    (expects fieldNum 1, branch returns 0); the control test
    w10_int_field_up_through_int_add_resolves_control PASSES, isolating the
    defect to the missing PTRADD-up arm.

## What IS faithful / correct (for the re-port)

- `run`/`runOneLevel`/`computeBestIndex`/`newTrials`/`newTrialsDown`: statement
  parity with the C++.
- `scoreTrialDown`: the pure/driver split is complete — every non-pure down arm
  (COPY/MULTIEQUAL/INDIRECT, LOAD, STORE, CALL*/CALLIND/RETURN, INT_ADD/INT_SUB/
  PTRSUB, SUBPIECE, **PTRADD-down**) IS present in the driver match; the rest is
  the verified pure table. PTRADD-down (`:561` -> `score_ptradd_down`) is the
  one the up-direction is missing.
- The three constructors, `testSimpleCases`/`testArrayArithmetic`,
  `score_add_down`/`score_ptradd_down`/`score_add_up`,
  `subpiece_composite_byte_offset`, `finalize_exact_piece`: faithful.
- `dtype.rs`: `TypeUnion` now constructed with `needs_resolution` (C++
  `TypeUnion()` ctor) — load-bearing flip, correct.
- `funcdata_resolveflow.rs`: `resolve_in_flow` dispatch + `score_single_component`
  faithful; two declared losses (partial-union truncation walk; non-existent in
  test scope) acceptable as cited.
- printc union arm: reads `fd->getUnionField(ptype, op, -1)` -> `getField(num)`;
  field name/type/id come from the scored union, NOT hardcoded. No special-casing.

## NO special-casing

Grepped the production diff for field-name / type-name / address / function-name
hardcoding: none. The only string literals are in the test file ("ram" space,
op mnemonics). The rendered field is `ct.get_field(field_num)` where `field_num`
is the scored resolution. Confirmed.

## NO regression (passing-set diff)

Built the merge-base (`37167d5`) in a parallel worktree with the same 148 `.sla`
specs symlinked in (the C++/spec tree is byte-identical — Rust-only diff), ran
`cargo test --workspace --no-fail-fast` on both:
  - BASE: 3517 passed, 0 failed
  - BRANCH: 3520 passed, 0 failed
  - `comm` of the passing test-name sets: 0 tests dropped (no regression);
    +3 new tests, all from `verify_w10_union_scoring.rs`.
Byte-identical anchors `boolless`/`readstruct`/`condconst_conn` and
`corpus_functions_byte_compare_against_cpp_oracle` pass in both. clippy
`-p kuna-decomp --lib` clean (0 warnings). C++ oracle tree unchanged ->
675/675 PARITY OK preserved.

## Real-parity of the new passes

`verify_w10_union_scoring.rs`'s 3 tests genuinely drive `ScoreUnionFields::
new_edge` + `resolve_in_flow`: a union read DOWN into FLOAT_ADD resolves to the
float field (idx 1), into INT_ADD to the int field (idx 0), via real scoring +
cache write — non-vacuous, field derived from scoring. They do NOT exercise the
F1 PTRADD-up path (all down-flow). `union_datatype.xml` (the B5 oracle with the
real `ptr->field` renders) is not wired into any Rust test yet, so end-to-end B5
parity is not asserted on either side.

## adversarial tests
(landed in rust/crates/kuna-decomp/tests/verify_w10_union_render_ptradd_up.rs)
  - w10_ptradd_up_ptr_field_scores_positive_like_cpp (FAILS — the F1 repro)
  - w10_int_field_up_through_int_add_resolves_control (PASSES — isolates F1)
  - w10_ptradd_up_missing_inputs_does_not_panic (PASSES — boundary)

## losses
None accepted. F1 is a defect, not a loss; sends the item back to `todo`.
