# w10-bitfield
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-15
branch: rport/w10-bitfield (a17a467; verifier test commit 22cc48c)

gate: `cd rust && cargo test -p kuna-decomp` -> ok, 0 failed (incl. the 10 new
      porter tests: 5 in `bitfield/tests.rs`, 5 in `dtype.rs#tests`).
      Whole-suite `cargo test --workspace` -> 3433 passed / 0 failed.
      Datatest e2e `./target/debug/decomp_test_dbg -sleighpath ../specs -path
      ../decompiler/datatests datatests` -> 668 applied / **187 passing**, byte-
      identical to the rust-port base (also 668 / 187 — no regression, no
      fabricated pass). `bitfields.xml`/`bitfields2.xml` still apply **0** tests
      (the IR-mutation + printc seam is unported, recorded as LOSS-092 — this
      wave delivers the prerequisite type-query layer, not the end-to-end
      rendering). C++ oracle `python -m kuna.run_tests --all --baseline
      docs/baseline.json` -> 207/207 + 675/675 PARITY OK, byte-untouched (no
      C++ edited). `cargo clippy -p kuna-decomp --lib` clean; the new test file
      `verify_w10_bitfield.rs` clippy-clean.

## Scope of the diff

3 Rust files, +632/-16: `bitfield.rs` (the `BitFieldTransform` base: ctor +
`establish_fields`), `bitfield/tests.rs` (5 porter unit tests), `dtype.rs` (the
type-query layer: `BitFieldTriple`, `compareMaxByte` x2, `upper_bound_idx`,
`collect_bit_fields`, `has_bit_fields_in_range`, `TypeBitField::bits`, + 5 unit
tests). This wave ports the prerequisites LOSS-092's restoration criteria named
("when the W6 type lattice — `collectBitFields`/`TypePartialStruct`/relative-
`getPtrInto` — land, port both transform engines"); the two transform subclasses,
the `apply()`/`doTrace`/`foldLoad`/`foldPtrsub` IR-mutation bodies, the six rule
`applyOp` bodies, and the printc `pushBitfield`/`checkBitFieldMember` rendering
remain seamed (still SEAM in the module docs).

## Faithfulness — verbatim against C++

- `BitFieldTransform::new` vs bitfield.cc:96-116: TYPE_STRUCT sets
  parentStruct+initialOffset; TYPE_PARTIALSTRUCT unwraps via `get_partial_base`
  (== C++ `getParent()`, both return `container`) + `get_partial_offset` (==
  `getOffset()`) and folds `off + part_off`. The `Option` guard is a no-op for a
  real PARTIALSTRUCT (always `Some`), so equivalent to the C++ unconditional
  `getMetatype()` check. FAITHFUL.
- `establish_fields` vs bitfield.cc:57-91: `vnBitSize = vn_size*8`; the
  `bitrange` ctor, `collectBitFields(0,…,initialOffset,vn_size)`, the sort, the
  `translateLSB`/`fieldEnd`/clip-to-`vnBitSize`/leading-hole/`overlapTest`
  code∈{0,3} field-vs-hole branch/trailing-hole — transcribed line-by-line. The
  `code != 0 && code != 3` else-branch emits a hole `field_end - pos`, matching
  the C++ `else { if (followHoles) …(pos,(fieldEnd-pos)); }`. FAITHFUL.
- `BitFieldTriple::compare` vs type.cc:932-949: endianness keyed off op1;
  `byteOff = offset + bits.byteOffset`; BE returns `byteOff1 > byteOff2`, LE
  returns `<`; lsb tie-break `<`; equal -> false. FAITHFUL.
- `collect_bit_fields` / `has_bit_fields_in_range` vs type.cc:1804-1852:
  `upper_bound(compareMaxByte)` start, the `BitRange(offset,sz,isBigEndian)` of
  the start element, the `overlapTest` code 1=break / -1=continue / else=collect,
  then the field-loop `upper_bound` + `offset>=offset+sz` break + nested-struct
  metatype/hasBitfields gate + recursion `(base+off, offset-off, sz)`. FAITHFUL.
- `compareMaxByte` x2 vs type.hh:322-324 / 339-341: `off < field.offset +
  field.type->getSize()` and `off < bits.byteOffset + bits.byteSize`. FAITHFUL.
- `upper_bound_idx` is a verbatim `std::upper_bound(first,last,value,comp)`
  binary search (`comp(value,element)`), returning the first index where `comp`
  is true. FAITHFUL.
- `BitFieldNodeState::follow_field` (pre-existing W6) correctly ANDs
  `is_most_significant()` into the sign-extension flag, matching
  `isSignExtended = (type==TYPE_INT) && bitsField.isMostSignificant()`
  (bitfield.cc:26). The `is_int` bool passed in is exactly
  `field_type.get_metatype() == TYPE_INT`. FAITHFUL.

## No special-casing

Greps for function-name / address / value / type-name / field-name hardcoding in
production code: NONE. `myfoo`/`field3`/`sfield4`/`fieldb` appear only inside test
fixtures (legitimate models of `bitfields.xml`'s `struct myfoo`), never in
`bitfield.rs`/`dtype.rs` logic. No magic-value short-circuits.

## Fleet discipline

This IS the bitfield wave; editing `dtype.rs` is permitted. It does NOT touch
`printc.rs` (the `pushBitfield` rendering stays a documented seam). Only
`bitfield.rs` + `dtype.rs` + their tests changed. CLEAN.

## No regression

Datatest passing SET byte-identical (187 -> 187, no function flipped either way;
bitfield datatests apply 0 both sides). The new functions have no live caller
outside `establish_fields`'s internal use + tests (grep-verified), so the cast/
merge/print plane is untouched — consistent with the unchanged 187 and the
byte-untouched C++ oracle. parity-fence `catalog_bytecompat` intact (5/5).

## Hunt list

- Signedness: clean. All added arithmetic is `int4` (i32) over byte/bit struct
  offsets, matching the C++ `int4`; the only `int4`-vs-`size()` C++
  sign-compare (`establishFields`'s `for(int4 i=0;i<overlap.size();++i)`) is
  ported as a by-reference iterator (`for triple in &overlap`), no index.
  `upper_bound_idx` uses `usize` indices bounded by `slice.len()`.
- Integer widths: clean. No `int4*int4 -> intb` promotions; all results stay
  `int4`. No width-narrowing.
- Wrapping: clean. The `int4` `+`/`-`/`*` on tiny struct offsets mirrors C++
  `int4` (signed; C++ overflow is UB, practically never hit) — plain arithmetic
  is the faithful mapping; a `wrapping_*` here would be LESS faithful.
- Comparator totality: clean. `BitFieldTriple::compare` is a strict-weak order
  (irreflexive, asymmetric — verified by adversarial test F-1). `compareMaxByte`
  is a partition predicate for `upper_bound`, not an ordering. No float cmp.
- Iteration-order provenance: clean. `bitfield`/`field` are `Vec`s iterated by
  slice (declared field order, == C++ vector order); `overlap` is sorted then
  iterated. No HashMap/HashSet anywhere in the added code.
- Off-by-one / do-while / reverse iterators: clean. `upper_bound_idx` is a
  verbatim transcription of libstdc++ `upper_bound`; the half-open
  `offset>=offset+sz` field break and the `compareMaxByte` `<` boundary are
  exact (verified by adversarial tests F-2/F-3). No do-while, no reverse iter.
- Erase-while-iterating: N/A (no mutation during traversal; `res` is append-only).
- Exception -> Result partial-state: N/A (no `?`/Result; the defensive
  `if let Some(parent)` in `establish_fields` replaces a path C++ never reaches
  with a null `parentStruct`, documented in the porter test).

## Mechanical pass

- `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare-`as` in added
  lines: NONE (grep-verified on the diff). `overlap.sort_by` is stable; C++ uses
  unstable `std::sort` — for the strict-weak `BitFieldTriple::compare`, ties
  occur only for two bitfields at the same byteOffset+leastSigBit, which a valid
  struct cannot have, so stable-vs-unstable is unobservable (a safe superset).
- `cargo clippy -p kuna-decomp --lib` -> clean. (`--tests` surfaces a
  pre-existing `overly_complex_bool_expr` error at heritage.rs:3122, an untouched
  W5 file unrelated to this wave — same family the LOSS-092 review already noted
  pre-exists on rust-port.)

## Adversarial tests (rust/crates/kuna-decomp/tests/verify_w10_bitfield.rs, 22cc48c)

- `w10bf_triple_compare_bigendian_reverses_byte_significance` — the BE arm the
  porter's diff under-tested: higher byte-offset container sorts first under BE
  (opposite of LE); comparator irreflexive/asymmetric; the `offset` term folds
  into the container byte offset.
- `w10bf_collect_upper_bound_skips_container_ending_at_offset` — `compareMaxByte`
  `upper_bound` boundary: a container ending exactly AT `offset` is skipped; the
  one ending one byte later is the start; `has_bit_fields_in_range` agrees at
  every boundary incl. the empty `[2,0)`.
- `w10bf_collect_nested_offset_shift_and_halfopen_break` — nested-struct
  recursion reports container offset `base+field.offset` and shifts the query by
  `field.offset`; the `curField.offset >= offset+sz` break is half-open (a nested
  struct starting AT the range end is excluded).
- `w10bf_establish_fields_bigendian_three_fields_no_holes` — BE establishFields:
  three fully-contained fields, no holes with follow_holes=false; the lone
  TYPE_INT field not at the container top is NOT sign-extended; the bool top
  field is never sign-extended.
- `w10bf_establish_fields_leading_and_trailing_holes_gated` — a mid-byte field
  emits leading [0,3) + trailing [5,8) holes when follow_holes=true, and ALL
  holes are suppressed when false.

All 5 pass on the branch; clippy-clean.

losses: LOSS-092 (the two `BitFieldInsert/PullTransform` engines, the six rule
`applyOp` bodies, and `foldLoad`/`foldPtrsub`/`doTrace`/`apply` IR-mutation stay
unported), extended by this wave to also cover the relative-pointer
`getPtrInto` case and the printc `pushBitfield`/`checkBitFieldMember` rendering
(both already named in the bitfield.rs module-doc SEAM(W10) and consistent with
LOSS-066). No NEW divergence introduced — the delivered type-query layer +
`BitFieldTransform` base is a faithful, fully-tested transcription; the residual
is the same end-to-end seam already ledgered. A faithful PARTIAL with zero
regression -> ACCEPT-WITH-LOSSES.
