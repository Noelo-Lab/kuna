# w5-s3-rules-5
verdict: ACCEPT-WITH-LOSSES
verifier: claude (independent verifier, round 2)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (458 lib + every verifier
      suite green; the item's `verify_w5_s3_rules_5` = 8 passed, incl. the 3
      round-2 adversarial tests). exit 0.
branch: worktree-wf_a6862008-41f-9 (this item's source diff is `ruleaction_5.rs`;
        the branch diff also carries the sibling `w5-dtype-expand` commits —
        `dtype.rs` / `verify_w5_dtype_expand.rs` are NOT in this item's scope and
        are reviewed under `w5-dtype-expand.md`). Round-2 tests committed 3aa7217.

scope: decompiler/cpp/ruleaction.cc 5527-6930 — 9 Rule classes
  (RuleBoolNegate, RuleLess2Zero, RuleLessEqual2Zero, RuleSLess2Zero, RuleEqual2Zero,
   RuleEqual2Constant, RulePtrArith, RuleStructOffset0, RulePushPtr) + the AddTreeState
  pointer-arithmetic engine + the RulePtrArith/RulePushPtr static helpers + their
  declarations in ruleaction.hh:1009-1099.
  Six comparison rules are FULLY PORTED; the three pointer rules
  (RulePtrArith/RuleStructOffset0/RulePushPtr) and ALL of AddTreeState are SEAM-stubbed
  to a no-op `applyOp` returning 0 (W6 type-facing + W3-varnode op-creation surfaces) —
  LOSS-069.

## round-1 -> round-2 delta
  Round 1 REJECTed on F1 (MAJOR): `RuleBoolNegate` flipped its def op-code via
  `get_booleanflip`, whose result was resolved through a `type_op_seam` that covered
  only {COPY, INT_EQUAL, INT_NOTEQUAL, BOOL_NEGATE}; the eight other flip codes
  (INT_SLESS<->INT_SLESSEQUAL, INT_LESS<->INT_LESSEQUAL, FLOAT_EQUAL/NOTEQUAL/LESS/
  LESSEQUAL) hit the `other =>` flags-0 fallback and lost `binary|booloutput`.
  **F1 IS FIXED.** `type_op_seam` (ruleaction_5.rs:91-130) now matches all 12
  `get_booleanflip` outputs with their verbatim typeop.cc opflags (the six `*LESS*`
  forms `binary|booloutput`, FLOAT_EQUAL/NOTEQUAL `binary|booloutput|commutative`,
  BOOL_NEGATE `unary|booloutput`, COPY `unary|nocollapse`). The round-1 failing tests
  `boolnegate_sless_flip_keeps_booloutput_flag` / `boolnegate_less_flip_keeps_booloutput_flag`
  now PASS (re-run, green), and the `..._equal_flip_..._control` still passes.

## hunt list
- signedness: clean. `int4` size operands vs `uintb` offsets handled deliberately
  (`calc_mask(get_size())`, `get_offset()==0`); the `rvn_size + (in1_off as int4)` /
  `lvn_size + (in1_off as int4)` SUBPIECE checks (ruleaction_5.rs:505,:631) transcribe
  the C++ `(int4)getOffset()` truncating cast exactly. No silent sign-compare slip.
- integer widths: clean for realistic inputs. `(lvn_size as u64) * 8 - 1` (:570) widens
  to u64 before the multiply where C++ computes `lvn->getSize()*8-1` in int4 then promotes
  for the `uintb` compare; identical for the only reachable sizes (lvn is the `-1`
  constant, size>=1). `calc_mask`/`uintb_negate` width-mapping matches (address.rs:1624/1719
  == address.cc:894/925, size>8 -> full mask).
- wrapping: clean for the six ported rules. `uintb_negate(off.wrapping_sub(1), size)` (:783),
  `cvn_off.wrapping_sub(...)` (:919), `cvn_off.wrapping_neg()` (:933), `!cvn_off` (:936) all
  match C++ uintb wrap; verified at the c==0 underflow boundary
  (`equal2zero_const_form_wrap_boundary_c_is_zero` PASSES). EXCEPTION: the two INT_AND
  sign-bit shifts use a BARE `>>` -> F1/LOSS-068 below.
- comparator totality: n/a (no `operator<` / sort / BTree in scope).
- iteration-order provenance: clean. `descend_snapshot` (funcdata_varnode.rs:463, pre-existing
  W3, NOT in this diff) yields C++ `list` order exactly. RuleBoolNegate takes ONE snapshot
  and reuses it for both the all-negates check and the set-to-COPY loop; setting a descendant
  to COPY does not remove it from `vn`'s descend list (a COPY still reads vn), so the snapshot
  stays valid — matches C++ re-iterating the live list. RuleEqual2Zero / RuleEqual2Constant
  descend loops are read-only. No HashMap/HashSet anywhere in the item.
- off-by-one / do-while / reverse iterators: n/a for the six ported rules (no do-while). The
  one do-while in scope (`RulePushPtr::duplicateNeed`, ruleaction.cc:6845) is in the
  SEAM-stubbed set (LOSS-069).
- erase-while-iterating: n/a in the ported rules (the erase/destroy-heavy bodies are all in
  the SEAM-stubbed AddTreeState/RulePushPtr).
- exception -> Result parity: clean. The six ported rules have no C++ throw site; the
  `op_set_input` calls return `KunaResult` and the port `.expect()`s them — these cannot fail
  for the in-bounds slots used, matching C++ which has no throw there.
- AND-rule hunt (transposed conds, swapped slots, inverted returns, missing early-outs): clean.
  Slot indices in RuleSLess2Zero (PIECE most-significant = `getIn(0)`; SUBPIECE hi = `getIn(1)`),
  the two-branch lvn/rvn-constant split, `get_hi_bit` totality (single-high-bit mask vs nzmask),
  RuleEqual2Zero's posvn/negvn/unnegvn assignment in both the const and INT_MULT(-1) forms, and
  every `return 0`/`return 1` early-out transcribe the C++ verbatim. The "0 == addvn" operand
  selection (:734-744) and RuleEqual2Constant's ADD/MULT/NEGATE newconst arms (:910-940) match.

## findings
  - F1 (MINOR, divergence -> LOSS-068): RuleSLess2Zero's two INT_AND arms compute the
    sign-bit fetch with a BARE shift `mv.get_offset() >> (8 * avn_size - 1)`
    (ruleaction_5.rs:543 for `-1 s< (avn & mask)`, :666 for `(avn & mask) s< 0`).
    Neither arm guards `avn_size <= 8` (the SUBPIECE arms DO, at :498/:624). For an
    `avn` strictly larger than 8 bytes, `8*avn_size-1 >= 64`, and `u64 >> 127` PANICS
    in a debug build ("attempt to shift right with overflow") where C++ `uintb >> 127`
    is UB-but-survives (x86 masks the count to 63; the rule fails the `& 1` test and
    returns 0). ADR-0003 mandates `wshr` for any shift whose count can leave [0,width);
    this is a `wshr` violation, in the same family as LOSS-055 (RuleZextEliminate) and
    LOSS-064 (RuleSubvarShift), which the project ledgers rather than blocks. Output-
    identical on all machine-word-sized operands (`avn <= 8`); reachable only on a
    signed comparison of a > 64-bit value (e.g. int128 / vector-lane spec).
       cpp: decompiler/cpp/ruleaction.cc:5767 (-1 s< arm), :5839 (s< 0 arm)
       rust: rust/crates/kuna-decomp/src/ruleaction_5.rs:543, :666
    repro: rust/crates/kuna-decomp/tests/verify_w5_s3_rules_5.rs
           ::sless2zero_int_and_oversize_avn_shift_panics  (PASSES via catch_unwind;
             trace `ruleaction_5.rs:543:36 attempt to shift right with overflow`),
           ::sless2zero_int_and_size8_fires_no_panic / ::sless2zero_int_and_size8_lhs_form_fires
             (controls: size-8 `>> 63` fires both arms cleanly).

  - F2 (MINOR, observation): three bare `as` casts lack the protocol's `// cast:`
    justification comment. `in1_off as int4` (ruleaction_5.rs:505,:631) transcribe
    C++ `(int4)getOffset()`; `lvn_size as u64` (:570) widens a size before a u64
    multiply. All are correct for reachable inputs but carry only a transcription `//`
    comment, not the `// cast:` form. Not output-affecting; flagged for the record.

  - F3 (TRIVIAL, observation): `use kuna_base::address::{Address, SeqNum}` —
    `SeqNum` is unused (`warning: unused import` at ruleaction_5/tests.rs:15:35). The
    only clippy warning in this item's files; the ported source `ruleaction_5.rs` is
    clippy-clean. Cosmetic.

## SEAM verification (RulePtrArith / RuleStructOffset0 / RulePushPtr + AddTreeState) -> LOSS-069
  Correctly deferred. Every body in this set reads W6 type-facing surfaces
  (`getTypeReadFacing`, `TypePointer`/`TypePointerRel`, `getSubType`, `nearestArrayedComponent*`,
  `numElements`, `evaluateThruParent`, `byteToAddress*`) and/or W3-varnode op-creation factories
  with outputs (`newOpBefore`/`newUniqueOut`/`newVarnodeOut`/`opSetOutput`/`opSetAllInput`/
  `distributeIntMultAdd`/`collapseIntMultMult`/`inheritUnionField`/`forceFacingType`/`warning`),
  none of which exist in the merged W5 IR (`Funcdata::op_set_output` is itself an `Err` stub,
  LOSS-035/036). Ported structurally (name/`get_op_list`/group-filtered `clone_rule`, registered
  in `specs()` with the `typerecovery` group); `applyOp` is a no-op `0` by design, pinned by
  `pointer_rules_are_noop_stubs`. Ledgered as LOSS-069 per the LOSS-035/036/067 precedent.

## mechanical pass
  - grep (item files): no todo!/unimplemented!/HashMap/HashSet/sort_unstable/panic!. 3 bare `as`
    casts -> F2.
  - `cargo clippy -p kuna-decomp --lib`: `ruleaction_5.rs` clean (0 warnings). The crate's
    other warnings are all in pre-existing, out-of-scope test files (verify_w3_*/verify_w4_*);
    the only in-item warning is the unused `SeqNum` import -> F3 (trivial).
  - `cargo test -p kuna-decomp`: PASS (458 lib + all verifier suites; item suite 8/8).

## registration / groups
  Verified against coreaction.cc::universalAction: the six comparison rules group "analysis"
  (RuleLess2Zero/LessEqual2Zero/SLess2Zero/Equal2Zero/Equal2Constant at 5824-5828, RuleBoolNegate
  at 5877); the three pointer rules group "typerecovery" (RulePushPtr/RuleStructOffset0/RulePtrArith
  at 5935-5937). `specs()` groups match exactly; `clone_rule` filters with
  `grouplist.contains(self.group)` as C++ `clone`. The module note correctly defers the *exact*
  in-pool registration ORDER (BoolNegate-after-comparisons; pointer pool reversed) to the W8
  builder — `specs()` preserves definition order with correct groups, the established
  ruleaction_N convention. Not a finding. Rule `name()` strings are W8 bookkeeping (RuleSpec
  carries no name); not a finding.

## adversarial tests added (round 2)
  rust/crates/kuna-decomp/tests/verify_w5_s3_rules_5.rs (branch commit 3aa7217):
  - sless2zero_int_and_oversize_avn_shift_panics      (PASS — F1/LOSS-068 panic trace)
  - sless2zero_int_and_size8_fires_no_panic           (PASS — size-8 -1 s< control)
  - sless2zero_int_and_size8_lhs_form_fires           (PASS — size-8 s< 0 control)
  (the round-1 tests boolnegate_{sless,less}_flip_keeps_booloutput_flag — once failing —
   are retained and now PASS, confirming the F1 fix.)

## decision
  ACCEPT-WITH-LOSSES. The sole round-1 blocker (F1, RuleBoolNegate flag loss) is fixed and
  re-verified green. The six comparison rules are a faithful statement-for-statement port
  (signedness/wrapping/iteration-order/slot-index hunt all clean). The two remaining
  divergences are accepted, ledgered losses: LOSS-068 (the un-guarded INT_AND sign-bit shift
  — debug panic vs C++ x86-masked UB on > 8-byte operands, same family as the already-accepted
  LOSS-055/064) and LOSS-069 (the three pointer rules + AddTreeState SEAM-stubbed pending the
  W6 type lattice + W3-varnode op-creation API, per the LOSS-035/036/067 deferral pattern).
  No blocker/major stands.
  losses: LOSS-068, LOSS-069 (appended to MAIN-tree docs/rust-port/losses.md).
