# w10-bitfield-full (rport/w10-bitfield-activate)
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Opus 4.8)
date: 2026-06-15
branch: rport/w10-bitfield-activate @ 3b19c02 (verifier tests committed on top: 5d3f6f2)
gate: `cd rust && cargo test -p kuna-decomp` -> green (2617 passed / 0 failed, incl. 3 verifier tests);
      `cargo test --workspace` -> 3470 passed / 0 failed;
      `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean;
      C++ oracle: `git diff rust-port...HEAD -- decompiler/cpp/` EMPTY -> 675/675 PARITY OK preserved.

## Summary

The branch activates the bitfield stage: it transcribes `BitFieldInsertTransform`
+ `BitFieldPullTransform` (transforms), the four `RuleBitField*` rule bodies (no
longer `0`-stubs), the `expression.cc` bitfield-expression family, the printc
`opZpull/opSpull/emitBitFieldStore/emitBitFieldExpression/checkBitFieldMember`
render, and the `type.cc` query layer (`findMatchingBitField`/`collectBitFields`/
`hasBitFieldsInRange`/`BitFieldTriple`). The transcription is FAITHFUL with NO
special-casing and ZERO regression. It is ACCEPT-WITH-LOSSES (not ACCEPT) for one
reason: the activation is DORMANT — `bitfield::specs()` is unwired into
universalAction and no committed test demonstrates a `ptr->field` render
end-to-end, so the prompt's "new passes = real parity / ptr->field genuinely
matches" is NOT substantiated (LOSS-193). The new code is a faithful PARTIAL.

## Hunt list

- Signedness: clean. `(int4)getOffset()` casts match C++ verbatim; `handleLessForward`
  slot/`leastSigZeroBits`/`needMaskCheck` logic transcribed exactly (bitfield.cc:1267-1311).
- Integer widths: clean. `SIZEOF_UINTB=8`; `byteSize > sizeof(uintb)` guards preserved
  in every handler; `8*SIZEOF_UINTB` shift bounds preserved.
- Wrapping: clean. `mask -= 1` -> `wrapping_sub(1)` (handleLessForward, constructOriginalValueMask);
  no other unsigned arithmetic can wrap on the bitfield-lineage values.
- Comparator totality: clean AND verified adversarially. `BitFieldTriple::compare`
  (dtype.rs) matches type.cc:932-949; `PullRecord` `operator<` (pull.rs `record_less`)
  matches bitfield.cc:1069-1081 with the correct `SeqNum` tuple `(space_idx, offset, time)`
  (address.hh:154-158). My r1 test proves irreflexivity/antisymmetry/transitivity both endians.
- Iteration-order provenance: clean. `workList` is a `std::list` consumed via `pop_front`;
  Rust uses `Vec::remove(0)` (documented O(n), order-preserving). `pullList.sort()` is a
  STABLE `std::list::sort`; Rust `sort_by` (slice sort) is also stable — load-bearing for
  same-op equal records. No `HashMap`/`HashSet` (grep clean).
- Off-by-one / do-while / reverse iterators: clean. `findMatchingBitField`'s `int4 max =
  len-1 = -1` empty case preserved (loop skipped). `verify*OriginalValue` walk block ops in
  reverse (`(0..final_idx).rev()`), matching the C++ reverse traversal from finalWriteOp.
  `testCompareGroup`'s `do/while` -> a `while cur < len { ...; if rec.read_op != op break }`
  preserving at-least-once over the first record.
- Erase-while-iterating: clean. `testCompareGroup`'s `pullList.erase(iter,curiter)` ->
  `drain(start..cur)` returning `start`; `apply`'s front-consume + in-place equal->normal
  conversion (no removal in `applyCompareRecord`) matches bitfield.cc:1655-1664 exactly.
- Exception -> Result partial-state parity: N/A at this surface (the transforms either
  fully apply or `doTrace` returns false before any mutation; `?`-propagation is on read paths).

## Faithfulness (the four focus areas)

1. TRANSFORMS — FAITHFUL. `doTrace`/`apply`/`processBackward`/`processForward` and every
   back/forward handler are line-for-line (bitfield.cc:57-1017, 1023-1669 vs insert.rs/pull.rs).
   The op-creation order in `apply`/`applyRecord`/`applyCompareRecord` (newOp/newConstant/
   newUnique*/insert-before/after, the loadModel duplication on `state.count>0`, the INT_LEFT
   shift splice with `opSetOutput`, the `opDestroyRecursive` of a dead modOp) is preserved.
2. RULES — FAITHFUL. The four `RuleBitField*::applyOp` bodies (bitfield.cc:1677-1759) are
   transcribed: `getTypeReadFacing`->`getPtrInto`->`hasBitfields`, the `notPrinted` LOAD guard,
   the `vn isWritten && def==INSERT` store guard, then `doTrace`/`apply`. (`RulePullAbsorb`/
   `RuleInsertAbsorb` were not in scope for this review's diff.)
3. EXPRESSION RECOVERY — FAITHFUL. `getStructures`/`recoverStructurePointer`/`getPullField`
   and the three expression records (expression.cc:403-586) match, incl. the `else if` fall-through
   in `recoverStructurePointer` and the partial-struct unwrap/offset fold.
4. PRINTC RENDER — FAITHFUL. `op_pull_ir` (opZpull/opSpull share a body, the sign distinction
   is in the recovered type), `emit_bitfield_store`, `emit_bitfield_expression`, and
   `check_bit_field_member` match printc.cc:378-389,1294-1344,2547-2637 in push order and the
   member(`.`)/pointer(`->`) decision off `field.byte_offset`.

## No special-casing

grep over bitfield.rs / bitfield/*.rs / printc.rs (non-test): NO string-literal field name,
NO hardcoded mask value, NO hardcoded offset/width, NO function-name branch. Offsets/widths
flow from the `TypeStruct` bitfield `TypeField`s (`collectBitFields`/`establishFields`) and
the op const inputs (`in(2)/in(3)` for INSERT, `in(1)/in(2)` for the pulls). NO `todo!`/
`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`panic!` in the new code.

## No regression / reserved files

- ONLY `rust/` changed; C++ oracle source byte-untouched -> 675/675 PARITY OK preserved.
- No reserved file edited: merge.rs / funcdata_merge.rs / variable.rs / varmap.rs /
  coreaction_cleanup.rs / coreaction_infertypes.rs all unmodified.
- dtype.rs diff is PURELY ADDITIVE (zero deletion lines) — the char/char*-pointer methods
  are untouched; only NEW TypeStruct bitfield methods added (as required).
- Passing-set diff base(rust-port)->branch: regressions EMPTY; 17 test fns added (none removed);
  + my 3 verifier tests. cargo test --workspace 3470/0.

## The loss (why ACCEPT-WITH-LOSSES, not ACCEPT)

The activation is DORMANT and the claimed end-to-end parity is not demonstrated:
- `bitfield::specs()` is referenced NOWHERE in src — the rules are NOT wired into the
  universalAction pass tree, so the transforms cannot fire in a live decompile.
- The decompilation passes are seam stubs (LOSS-130): the IR reaching the printer is raw
  lifted p-code with no recovered bitfield types/symbols, so the rule guards could not fire
  even if wired.
- NO committed test runs a transform `apply()` over a real Funcdata and asserts a `ptr->field`
  render. All bitfield tests exercise the type-query primitive / `establishFields` arithmetic.
  `verify_w10_bitfield_full.rs`'s module doc (lines 20-22) claims such an end-to-end
  `bitfields2.xml` byte-match test exists; it does NOT — the doc overstates the coverage.
This is a faithful PARTIAL with zero regression -> ACCEPT-WITH-LOSSES, cited as LOSS-193.

## Adversarial tests (verifier-authored, committed on the branch @ 5d3f6f2)

- `r1_triple_compare_is_total_strict_weak_order_both_endians` — `BitFieldTriple::compare`
  irreflexivity/antisymmetry/transitivity + std debug-sort totality, both endians.
- `r2_find_matching_be_multibyte_window_narrowing` — `find_matching_bit_field` BE binary
  search across two bytes (5 fields); every field found, partial straddle returns None.
- `r3_collect_upper_bound_boundary_is_strict` — `compareMaxByte` upper_bound boundary is
  strict (`<`): a field ending exactly at the query offset is excluded.
(file: rust/crates/kuna-decomp/tests/verify_w10_bitfield_activate_r1.rs — all 3 pass.)

## losses

- LOSS-193 (appended to docs/rust-port/losses.md): bitfield activation is dormant
  (specs() unwired + LOSS-130 stubbed passes) and the `ptr->field` end-to-end render is
  not demonstrated; faithful PARTIAL, zero regression. Cross-refs LOSS-066 (restoration
  criterion still UNMET) and LOSS-130.
