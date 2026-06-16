# w10-pushpartialsymbol
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Opus 4.8)
date: 2026-06-16
gate: cd rust && cargo test -p kuna-decomp  +  python -m kuna.run_tests --all --engine rust  ->  rust datatests 307/675 (+9 vs base 298, zero regressions); cargo test --workspace 0 failed; C++ oracle 207/207 + 675/675 PARITY OK

## Summary

Faithful port of the STRUCT/UNION arms of `PrintC::pushPartialSymbol`
(printc.cc:2019-2141) plus the `PrintC::opSubpiece` field-extraction entry
(printc.cc:863-898) and the `RuleSubRight` `opMarkSpecialPrint` IR-side enabler
(ruleaction.cc). Delivers **9 real new datatest passes** under the Rust engine
(`Union #5/#6/#7/#12/#15/#16`, `Concat #11/#12`, `Implied Fields #2`) with
**ZERO regression** to the previously-passing set. Verdict ACCEPT-WITH-LOSSES:
the array / allowCast-SUBPIECE / artificial-field arms of `pushPartialSymbol`
and the non-special SUBPIECE cast-selection are honest, corpus-inert
narrowings, cited as LOSS-202.

## FOCUS findings

1. FAITHFULNESS — CLEAN. The STRUCT arm (`find_truncation` ->
   `getFieldIter`/`noff`/span-check) and the UNION arm (the `getUnionResolution`
   BTreeMap cache read + `newoff = off - field->offset` + the
   `newoff+sz > field->type->getSize()` span guard) reproduce
   `Datatype::findTruncation` (type.cc:1878-1892 / 2613-2627) exactly;
   `subpiece_byte_offset_for_composite` reproduces
   `TypeOpSubpiece::computeByteOffsetForComposite` (typeop.cc:2197) by endianness;
   the `off==0 && (sz==0 || (sz==getSize() && (!needsResolution||TYPE_PTR)))`
   break and the RPN push order match printc.cc:2033-2140. The new path is
   GUARDED: `push_partial_symbol_ir` returns `false` on a whole-symbol cover, so
   a non-partial-symbol read falls straight through to the bare-name render
   byte-unchanged (verified: `fillin`'s member lines byte-match the C++ oracle and
   carry no `glob.`/`val.` token; `boolless`/`readstruct`/`condconst` render
   BYTE-IDENTICAL rust==cpp).

2. NO SPECIAL-CASING — CLEAN. The only `intfield`/`val.c`/`bval1` strings in the
   diff are in doc-comments; ZERO string literals in the new executable code.
   Field/symbol names come from the walked type's field table and the `kuna_name`
   binding — verified type-driven and byte-identical to the C++ oracle for
   `printglob` (`glob.intfield`/`.floatfield`/`.uintfield`) and `testvalue`
   (`val.myptr`/`val.mydouble`), with no cross-function leak (A3).

3. NEW PASSES ARE REAL PARITY — CONFIRMED. The +9 gain was computed by diffing
   the full failing SET base(298) vs branch(307): `comm -13` = 9 gains,
   `comm -23` = empty (zero regressions). The member renders were byte-compared
   per function against the C++ oracle B5 (`KUNA_DUMP=1`): `glob.intfield / 3`,
   `glob.floatfield + 11.0`, `glob.uintfield >> 3`, `*val.myptr + 3`,
   `val.mydouble + 33.0` are token-for-token identical. (The whole-function body
   still differs only in if/else-if flattening and for-loop reroll — separate
   waves held in `blockaction.rs` — never in any member token this item ships.)

4. NO REGRESSION — CONFIRMED (CRITICAL). `push_vn_explicit_ir` change is guarded
   so the universal explicit-render path is byte-unchanged for non-partial reads.
   `boolless`+`readstruct`+`condconst` BYTE-IDENTICAL rust==cpp. Full passing SET
   strictly superset of base (`comm -23` of failing sets empty). The diff edits
   only `printc.rs` + `ruleaction_6.rs` (+ the verifier test) — NO reserved file
   (`coreaction_stackptr.rs`, `coreaction_infertypes.rs`, the deadcode file,
   `jumptable.rs`, `funcdata_spacebase.rs`, `blockaction.rs`) touched.
   `cargo test --workspace` green (0 failed). C++ oracle 207/207 + 675/675
   PARITY OK untouched (rust-only diff).

## Hunt list

- Signedness: clean — `off`/`newoff` int8, `sz` int4, matching C++ `int8`/`int4`;
  `newoff + sz > getSize()` is signed-int8 in both.
- Integer widths: clean — `as int8`/`as uint4` are width conversions matching
  C++ `int8 byteOff` / `(uint4)offset` / `(int4)getOffset()`.
- Wrapping: clean — all arithmetic signed int8 (`off - foff`, `boff += sym_off`);
  no unsigned wrap site, no `wrapping_*` needed or misused.
- Comparator totality: n/a — no comparator ported.
- Iteration-order provenance: clean — `union_map` is a BTreeMap (C++ `map` order),
  READ-only here; the type walk is single-field-per-level deterministic; no
  HashMap/HashSet in ported logic.
- Off-by-one / do-while / reverse iterators: clean — `while(ct != 0)` ->
  `while let Some(cur)` (pre-condition, matches); the reverse `for(i=size-1;..)`
  object_member push == forward push since all tokens are `object_member`.
- Erase-while-iterating: n/a — no container mutation during traversal.
- Exception -> Result partial-state: clean — `find_truncation` Err is treated as
  "no field" (Ok(None) equivalent), the C++ union override returns null there.

## Mechanical pass

- grep new code: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/
  `unwrap()`/`panic!`; bare `as int8`/`as uint4` are width conversions matching
  C++ casts.
- grep for hardcoded field/type/symbol/address names in executable lines: NONE
  (matches are doc-comments only); ZERO string literals in the new code.
- `cargo clippy -p kuna-decomp --lib` clean; `--test verify_w10_pushpartialsymbol`
  clean.
- reserved-file check: none touched.

## Adversarial tests (committed to the branch, commit 77ced72)

rust/crates/kuna-decomp/tests/verify_w10_pushpartialsymbol.rs:
  - a1_printglob_struct_member_render_byte_matches_cpp_oracle (struct walk,
    byte-parity of the member lines vs the C++ oracle B5)
  - a1_testvalue_union_member_render_byte_matches_cpp_oracle (union resolution
    cache walk, `val.myptr`/`val.mydouble` byte-parity)
  - a2_non_partial_symbol_function_byte_unchanged (the GUARD: `fillin` carries no
    spurious member token, member lines byte-unchanged vs oracle)
  - a3_member_names_are_type_driven_no_cross_function_leak (data-driven, no
    cross-function field-name leak)
All 4 PASS on the branch.

losses: LOSS-202 (the ARRAY arm, the allowCast SUBPIECE-cast arm, the
`!succeeded` artificial-field name, and the gated non-special SUBPIECE
cast-selection are unported / dormant; each degrades to the bare-name or
functional render, 0 corpus passes AND 0 regressions today).
