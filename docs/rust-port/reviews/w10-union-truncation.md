# w10-union-truncation
verdict: ACCEPT
verifier: opus-4.8 (independent, round 1)
date: 2026-06-16
branch: rport/w10-union-truncation @ 0153b1b (base rust-port @ 13ed4c3)
gate: `cd rust && cargo test --workspace --no-fail-fast` -> 0 failed (3585 tests
      listed; 3548 result-line sum, 37 ignored);
      `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean;
      `cargo clippy -p kuna-console --test verify_w10_union_truncation -- -D warnings` -> clean

## Scope of the diff

rust/ only, 3 source files (no test files added by the porter):
- `funcdata_resolveflow.rs` (+241): the real `Funcdata::resolve_truncation`,
  `resolve_truncation_union`, `find_truncation`, and the rewired
  `resolve_in_flow_partial` container walk.
- `coreaction_casts.rs` (+34): `get_output_token_subpiece` routes a union /
  partial-union receiver through `resolve_truncation` (scores+writes) instead of
  the read-only struct `find_truncation`.
- `dtype.rs` (+/-): doc + the `Datatype::{find,resolve}_truncation` union arms
  flipped from `SEAM(W6)` stubs to hard guards ("must route through Funcdata::*").

`git diff rust-port...HEAD --stat -- decompiler/ specs/ docs/baseline.json
docs/baseline-stages.json` is EMPTY -> the C++ decompiler tree, the SLEIGH specs,
and the recorded oracle are byte-identical to rust-port, so the C++ oracle's
675/675 PARITY OK is preserved by construction (not re-run; unchanged inputs).

No reserved file touched: the changed set is exactly the 3 files above.
`coreaction_infertypes.rs`, `architecture.rs`, `fspec.rs`,
`funcdata_spacebase.rs`, `typeop.rs`, `varmap.rs` are ABSENT from the diff.
`dtype.rs` IS edited but only the `find_truncation`/`resolve_truncation` methods
(the union/partial-union arms) — `git diff ... -- dtype.rs | grep -i spacebase`
is empty, so the reserved `TypeSpacebase`/spacebase-pointer arms are untouched.
`printc.rs` is NOT in the diff (the held `op_subpiece_ir` subpiece-cast arm is
untouched).

## Faithfulness (the core claim)

Transcribed statement-for-statement against the C++ in scope:
- `TypeUnion::resolveTruncation` (type.cc:2569-2605) -> `resolve_truncation_union`:
  cache lookup -> address-based fallback (`setUnionField` then reuse `res`) ->
  the `if (res != 0) { fieldNum>=0 -> field }` arm -> the `else if SUBPIECE &&
  slot==1` ctor-2 scorer -> the `else` ctor-3 scorer.  The C++ subtlety that a
  cached `res` with `fieldNum < 0` falls THROUGH the if/else-if/else to
  `return 0` (the else-arms run ONLY when `res==0`) is reproduced exactly: when
  `have_cached` is `Some(field_num)` and `field_num < 0` the port returns
  `Ok(None)` without scoring (funcdata_resolveflow.rs, "res != 0 but fieldNum<0").
- `TypePartialUnion::resolveTruncation` (type.cc:2994-2998) -> the PartialUnion
  arm: `container->resolveTruncation(off + offset, ...)`.
- `TypeUnion::findTruncation` (type.cc:2613-2627) -> the Union arm: cached-only,
  `newoff = off - field.offset`, `if (newoff + sz > field->type->getSize())
  return 0`.  Widths match (int8 + int4 vs int4, all promoted to signed int8).
- `TypePartialUnion::findTruncation` (type.cc:2880-2884) -> delegate to container.
- `TypeStruct::findTruncation` (type.cc:1878-1892) -> the `_ =>
  ct.find_truncation(...)` struct field-walk fall-through (pre-existing, faithful).
- `TypePartialUnion::resolveInFlow` walk (type.cc:2945-2975) -> `resolve_in_flow_
  partial`: the `while(curType && getSize()>size)` loop with the PARTIALUNION arm
  (`curOff += getOffset()`, `getParentUnion()->resolveTruncation`, then
  `getExactPiece(field->type, curOff, size)` using **curOff not newOff** — port
  discards `_newoff` and uses `cur_off`), the UNION arm (`resolveTruncation`
  writes newoff INTO curOff, then `getExactPiece(..., cur_off, ...)` — port
  captures `newoff` into `cur_off`), the "should never reach here" else, and the
  `curType==0 || getSize()!=size -> stripped; updateUnionField` tail.
- ctors 2/3 (`new_subpiece`/`new_truncation`, unionresolve.cc:1161/1195) are
  pre-existing (W10 union-scoring) and were re-confirmed faithful here.

The deliberate, documented divergence: C++ `TypeOpSubpiece::getOutputToken`
(typeop.cc:2144-2161) calls READ-ONLY `findTruncation`, relying on
`propagateType` (ActionInferTypes) having already written the slot-1 edge via
`resolveTruncation`.  In the port `propagateType` runs through an immutable
`&Funcdata` and cannot write the cache, so `get_output_token_subpiece`
(ActionSetCasts, `&mut`) is the first reader and drives the SAME scorer (ctor 2,
op==SUBPIECE && slot==1, same offset) — the resolved field is provably identical;
the cache write merely shifts plane (infer -> cast), and both render `print C`
afterward.  E2E (below) confirms `b.bval1`/`a.aval2`/`a.aval1` all resolve.

## REAL-PARITY check (the new wins are genuine, not stubs)

Drove the *actual* partialunion.xml script through the console (`dec` + `print C`)
in the verifier test `verify_w10_union_truncation.rs`:
- `partialunion`: emits `globvar.b.bval1 = val;` (oracle #1) AND
  `return globvar.a.aval2;` (oracle #2) — a float4 store resolves to the float4
  field `b.bval1` (offset 0), an int4 read resolves to the int4 `a.aval2`
  (offset 4): DIFFERENT arms/offsets from the SAME engine on the truncated flow.
- `partial1`: emits `globvar.a.aval2 = a1;` (#3) and `globvar.a.aval1 = a1 + 7;`
  (#4) — int stores landing on `a.aval2`/`a.aval1` by offset.
The pre-port behavior (the SEAM stub that fell straight to `stripped`) could NOT
produce any of these member accesses; the field-resolution is real.

## No regression

- Whole workspace: 0 failures (>> the 307 floor; 3585 tests).  The porter added
  no test files (3 src files only), so the passing SET strictly grows by the +3
  verifier tests; nothing was removed and nothing flipped pass->fail.
- Byte-identical guards intact: `print_b5_boolless` (incl.
  `w10_boolless_full_byte_parity_modulo_type_inference`) -> 11 ok; the
  struct/union/merge-facing/input-params binaries (`verify_w10_struct_corpus` 19,
  `verify_w10_input_params` 4, `verify_w10_merge_facing` 3,
  `verify_w10_partial_types_console` 6, unionresolve unit 17) -> all ok.  The two
  changed call sites (`get_output_token_subpiece`, `resolve_in_flow_partial`) did
  not perturb the struct (non-union) path: struct corpus still green.
- C++ oracle: inputs byte-identical -> 675/675 PARITY OK preserved by
  construction.

## The mandatory hunt list

- Signedness: clean.  Every comparison is C++-faithful: `cur.get_size() as int4
  <= size` (int4 vs int4 == C++ `getSize() > size`); `newoff + sz as int8 >
  get_size() as int8` (C++ `newoff + sz > getSize()`, all promoted to signed
  int8); `field_num >= 0` (== C++ `getFieldNum() >= 0`).
- Integer widths: clean.  See F1 — all `as int4`/`as int8` casts reproduce a C++
  implicit narrow/widen (curOff int8 -> getExactPiece int4; byteOff int4 ->
  resolveTruncation int8; int8 offset -> ScoreUnionFields int4 param).  Each
  picks the C++ result, none the "fixed" one.
- Wrapping: clean.  The only arithmetic on the truncation path is `offset - foff`
  / `off - field.offset` (small int4 field offsets subtracted from an int8 byte
  offset, never wrap-prone) and `cur_off += getOffset()` (int4 union-field offset,
  bounded by type size).  No `uintb`-lineage arithmetic in the new code.
- Comparator totality: N/A — no comparator added.  The scorer's `compute_best_
  index` (used by ctors 2/3) is pre-existing and already verified total.
- Iteration-order provenance: the `resolve_in_flow_partial` `while` walk mirrors
  the C++ `while` loop exactly (a single linear container hop per iteration, no
  container traversal).  No HashMap/HashSet introduced (`union_map` is a BTreeMap,
  pre-existing).  Grep for HashMap/HashSet in the added lines: none.
- Off-by-one / do-while / reverse iterators: the `while let Some(cur) =
  cur_type.clone() { if size<= break }` is a faithful translation of the C++
  `while(curType!=0 && getSize()>size)` (pre-test loop, may run zero times); the
  `cur_off = 0` reset after each arm matches.  No do-while, no reverse iterators.
- Erase-while-iterating: N/A (no container mutation during traversal).
- Exception -> Result partial-state parity: the `?` points
  (`get_exact_piece`, `types_rc`, the scorer ctors) propagate before any
  cache write that C++ would not also have skipped; on the partial-union miss the
  port writes `stripped` via `update_union_field` exactly as C++
  `updateUnionField(this,op,slot,curType)` does.

## Mechanical pass

- grep added lines for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`: NONE.
- bare `as` casts: present (see F1), none change behavior.
- clippy `-p kuna-decomp --lib -D warnings`: clean.

## findings

  - F1 (low / style): the new `as int4`/`as int8` casts in
    funcdata_resolveflow.rs and coreaction_casts.rs carry no `// cast:`
    justification comment (verification.md "Mechanical pass" asks each bare `as`
    to be annotated).  All are faithful C++ implicit conversions (curOff int8 ->
    getExactPiece int4 narrowing; byteOff int4 -> resolveTruncation int8
    widening; int8 offset -> ScoreUnionFields int4 param), verified one-by-one
    above.  Style nit, not a correctness defect — does not block ACCEPT.
       cpp: decompiler/cpp/type.cc:2956,2964 (getExactPiece int4 narrow);
            decompiler/cpp/typeop.cc:2152,2181 (byteOff widen);
            decompiler/cpp/unionresolve.cc:1161,1195 (int4 offset param)
       rust: rust/crates/kuna-decomp/src/funcdata_resolveflow.rs (resolve_in_flow_
             partial / resolve_truncation_union cast sites);
             rust/crates/kuna-decomp/src/coreaction_casts.rs (get_output_token_
             subpiece: `byte_off as int8`)

## adversarial tests (land with verdict, in kuna-console)

rust/crates/kuna-console/tests/verify_w10_union_truncation.rs:
  - partialunion_float_write_and_int_read_resolve_to_correct_fields
      (T1+T3: float store -> b.bval1, int read -> a.aval2; real cache write+read)
  - partial1_int_writes_resolve_to_a_aval_fields_by_offset
      (T2: a.aval2 @off4 and a.aval1 @off0 from the same engine, offset-driven)
  - unrelated_function_has_no_union_field_tokens
      (T2 negative: condconst carries none of globvar/bval1/aval1/aval2 —
       anti-special-casing, the resolution fires only on union flow)
All 3 PASS on the branch (driving the real partialunion.xml console script).

losses: none (faithful full port; the only divergence — getOutputToken driving
the scorer from the cast plane — is behavior-preserving, proven by E2E parity,
so it is not a loss).  Note: the pre-existing `finalize_exact_piece` getExactPiece
-null LOSS belongs to w10-union-scoring, not this item.
