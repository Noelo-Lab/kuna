# w5-s3-rules-1
verdict: ACCEPT
verifier: claude-opus (independent verifier, round 2)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (482 lib unit + all integration;
      verify_w5_s3_rules_1: 15 passed, incl. 3 new round-2 tests)
clippy: `cargo clippy -p kuna-decomp -- -D warnings` -> clean
port-audit: n/a (no port-audit harness present in this tree)

## Scope

ruleaction.cc lines 1-1443, the 25 classes RuleEarlyRemoval..RuleRangeMeld
(getMultCoeff/minMaxUse/acceptableSize shared statics included), ported into
`rust/crates/kuna-decomp/src/ruleaction_1.rs` (+ `ruleaction_1/tests.rs`). The
branch's dtype.rs / verify_w5_dtype_expand.rs changes are the prior
w5-dtype-expand item, not in scope. Port commit: 06acb60; round-2 repair: dc30546.

14 rules fully ported (Bxor2NotEqual, OrMask, AndMask, OrConsume, OrCollapse,
AndOrLump, NegateIdentity, Piece2Zext, Piece2Sext, TermOrder, LessOne,
ShiftBitops, RightShiftAnd, HighOrderAnd) + the 3 shared statics; the other 11
are seam-deferred at a missing Funcdata/expression/rangeutil helper, each
returning 0 after every portable early-out guard is transcribed. The seams are
pre-existing cross-wave seams (W3 output-creation, W5 join-record, W6
TypeFactory, expression/rangeutil) documented in the module header; they are
in-scope conservative declines, not new losses.

## Round-2 disposition (F1 fixed)

Round 1 REJECTed on one BLOCKER (F1): `RuleRightShiftAnd::apply_op` used raw
`u64 >> i32` on a data-derived p-code shift count that can be >= 64 (or negative
after the `(int4)` truncation), panicking in debug ("attempt to shift right with
overflow") where the C++ raw `>>` is defined (x86-64 masks the count to 6 bits and
the rule applies). Repair commit dc30546 routes both `>> sa` sites through `wshr`
(`wrapping_shr`, count taken mod 64).

Re-derivation of the fix (independent):
- C++ `maskVn->getOffset() >> sa` / `calc_mask(rootVn->getSize()) >> sa`, `sa` an
  `int4`. On the x86-64 reference build a 64-bit `shr/sar` reads the count register
  and masks it to the low 6 bits, so `>> 64 == >> 0` and a negative `sa` masks to
  `sa & 0x3F`.
- Rust `get_offset().wshr(sa as u32)`. `wshr` for `u64` = `wrapping_shr`, i.e.
  `count % 64` (verified at `kuna-base/src/types.rs:129`). For `sa in [0,63]` this
  is exactly `val >> sa`. For `sa == 64` -> count 0. For a negative `sa` (e.g. -64
  -> `0xFFFFFFC0 as u32`), `% 64 == 0` — identical to the x86 mask. The two sides
  agree on every count.
- Correctly chose `wshr` (x86 raw-`>>` mask) and NOT `pcode_right` — the C++ rule
  uses raw `>>`, whereas `pcode_right`/`pcode_left` carry a `>= 64 -> return 0`
  guard. The distinction is real and the porter respected it (confirmed by the new
  R2-3 test below).

The flipped test `w5s3_rightshiftand_large_shift_masks_count_and_applies` now pins
the applied C++ outcome (returns 1, slot0 -> rootVn) for the formerly-panicking
`sa == 64` case. No LOSS recorded (a fixed blocker is not an accepted divergence).

## Hunt list (re-run, independent)

- **Signedness**: clean. `size > SIZEOF_UINTB`, `min_byte >/< SIZEOF_UINTB` mirror
  C++ `int4 vs sizeof(uintb)` mixed-sign compares; operands are provably >= 0.
  `RulePiece2Sext` `int4 n = (int4)getOffset()` then `n != 8*xsize-1` matches the
  C++ truncate-then-compare. `min_max_use` `min = (int4)getOffset()`,
  `min+outsize-1` are int4 like the C++.
- **Integer widths**: clean. int4->i32, uintb->u64, uintm->u32; `8*xsize-1`,
  `min+outsize-1`, `8*min_byte` are i32/int4 like the C++ int4.
- **Wrapping**: clean (F1 fixed). `RuleRightShiftAnd` both shift sites now `wshr`.
  `RuleHighOrderAnd` `(val-1)|val` -> `val.wsub(1)|val` (correct at val==0, both
  for size<8 decline and size==8 pass). `RulePullsubMulti` `calc_mask(new_size)
  << (8*min_byte)` guarded by the `min_byte < SIZEOF_UINTB` branch so `8*min_byte`
  in [0,56] — in range. `RuleShiftBitops` routes through `pcode_left`/`pcode_right`
  (the `>=64 -> 0` helper), faithfully ported in kuna-base.
- **Comparator totality**: n/a (no comparators in this batch).
- **Iteration-order provenance**: clean. `RuleNegateIdentity`, `min_max_use`
  iterate `descend_iter()` (the C++ `list<PcodeOp*>` descend list, insertion
  order); collect-into-Vec-then-iterate preserves order, and both either return on
  first match or only read, so the snapshot is equivalent. No HashMap/HashSet
  anywhere (grep clean).
- **Off-by-one / do-while / reverse iterators**: clean. `RuleShiftBitops` post-loop
  `getIn(1-i)` (binary ADD/XOR/OR), `RuleHighOrderAnd`/`RuleAndDistribute`
  `for i in 0..2`+`getIn(1-i)`, `RuleAndDistribute` manual `while i<2` with `i+=1`
  on every `continue` and bare `break` on the AND-cancel/trivial checks (mirrors the
  C++ for-loop ++i vs break-exit exactly). `acceptable_size` chain faithful.
- **Erase-while-iterating**: n/a in the ported set — `RuleNegateIdentity` mutates
  the first matching descendant then returns immediately (no further traversal);
  the rules that erase during traversal (replaceDescendants, findSubstitute) are
  seam-deferred.
- **Exception->Result parity**: n/a — the only C++ throw sites
  (`replaceDescendants`/`buildSubpiece` LowlevelError) live behind seams.
- **Rule-specific (transposed conds / swapped slots / inverted returns / missing
  early-outs)**: clean. RuleAndMask first-two-branch fold (`andmask==0 ||
  (andmask&consume)==0`, both -> const 0) behavior-preserving; consume hoist
  side-effect-free. RuleOrConsume slot 0/1 order preserved. RuleTermOrder swap
  direction (`vn1_const && !vn2_const` -> swap 0,1) correct. RuleShiftBitops/
  RuleRightShiftAnd shift idioms NOT transposed (R2-3). Re-derived per rule.

## Mechanical pass

- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable/panic!/unreachable! in
  ruleaction_1.rs + tests: **none**.
- bare `as` casts: 7, all `... .get_offset() as int4` / `... as int4 * 8` / `sa as
  u32` mirroring a C++ `(int4)` cast (or the x86-mask shift count) on the
  immediately-preceding commented line. Faithful; lack a literal `// cast:` token
  (minor style nit per verification.md, not a correctness finding — the C++ source
  line is the justification).
- raw shift operators: only `calc_mask(new_size) << (8*min_byte)` at :1504, guarded
  in range; all other shifts route through `wshr`/`pcode_left`/`pcode_right`.
- clippy -D warnings: clean.

## findings

  (none standing) — round-1 F1 (BLOCKER) resolved by dc30546; re-derived above and
  re-tested green. No new findings.

## adversarial tests (rust/crates/kuna-decomp/tests/verify_w5_s3_rules_1.rs)

Round-1 set (12, retained; the F1 panic test flipped to the applied-outcome test):
- w5s3_rightshiftand_large_shift_masks_count_and_applies (F1 fix, sa==64 applies)
- w5s3_rightshiftand_shift_63_in_range_applies
- w5s3_shiftbitops_mult_or_picks_one_minus_i_operand
- w5s3_shiftbitops_mult_odd_coeff_declines
- w5s3_shiftbitops_mult_zero_coeff_declines
- w5s3_highorderand_val_zero_wraps_and_declines
- w5s3_highorderand_clean_submask_passes_form_then_nzmask_declines
- w5s3_highorderand_dirty_mask_declines_at_form_check
- w5s3_lessone_less_val1_becomes_equal_zero
- w5s3_lessone_lessequal_val0_keeps_slot1
- w5s3_lessone_less_val2_declines
- w5s3_negateidentity_skips_nonmatching_then_collapses_match

Round-2 set (3 new, commit 819d830):
- w5s3r2_rightshiftand_negative_sa_masks_like_x86  (negative sa -> wshr mod 64,
  no panic, applies — the F1 fix's riskiest corner)
- w5s3r2_rightshiftand_large_shift_full_mismatch_declines  (masked nonzero count,
  full!=mask -> decline; no false apply)
- w5s3r2_shiftbitops_left_shift_64_uses_pcode_left_returns_zero  (pcode_left vs
  wshr non-transposition: INT_LEFT sa==64 -> nzm clears -> applies)

## losses

none.

## disposition

ACCEPT — the round-1 BLOCKER (F1) is fixed correctly and confirmed by independent
re-derivation + 3 new adversarial tests; the full gate is green and clippy is
clean. The 14 fully-ported rules and 3 statics re-derive faithfully against the
C++; the 11 seam declines are in-scope conservative no-changes against existing
cross-wave seams (no loss). No findings stand.
