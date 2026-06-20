# w5-s3-rules-8
verdict: ACCEPT-WITH-LOSSES
verifier: claude (independent verifier, round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (446 unit + all integration suites pass; new verify_w5_s3_rules_8 = 8 passed); `cargo clippy -p kuna-decomp -- -D warnings` -> clean

scope: decompiler/cpp/ruleaction.cc 9804-11032 (12 rules: RuleUnsigned2Float,
RuleInt2FloatCollapse, RuleFuncPtrEncoding, RuleThreeWayCompare,
RulePopcountBoolXor, RulePiecePathology, RuleXorSwap, RuleLzcountShiftBool,
RuleFloatSign, RuleFloatSignCleanup, RuleOrCompare, RuleExpandLoad) +
declarations in ruleaction.hh. Rust: rust/crates/kuna-decomp/src/ruleaction_8.rs.

## Hunt list

- **Signedness**: clean. The risk points are `const_slot:int4` used in
  `op->getIn(1-const_slot)` (RuleThreeWayCompare) — faithful, `get_in` takes
  int4; `form:int4` bit-encoding all-positive (0..23); `int4` bit positions in
  `get_boolean_result`. The `(int)val+1` form value uses `val<=2` so the int
  cast is exact. No signed/unsigned comparison was silently inherited.
- **Integer widths**: clean with one non-physical note (F3). `(int4)getOffset()`
  shift-amount casts (ruleaction.cc:10394/10415/10425) are reproduced as
  `get_offset() as int4` (u64->i32 truncation, exactly C++ `(int4)`).
  `max_return = 8u64 * (in0_size as uintb)` widens before multiply where C++ is
  `8 * int4`; identical for all realizable sizes (F3, non-blocker).
- **Wrapping**: clean. `testCompareEquivalence`'s `a2off+1==a1off` etc. use
  `wrapping_add` (C++ unsigned `getOffset()+1` wraps); `funcptr_align` slide and
  the bit-position `mask <<=`/`>>=` use `wrapping_shl`/checked shift exactly where
  C++ shifts a uintb. No `wrapping_*` that should be `checked_*`.
- **Comparator totality**: n/a (no `operator<`/BTree/sort ported in this batch).
  The `(OpCode)(lessform+1)` LESSEQUAL arithmetic is replaced by an explicit
  `lessequal_after` match — verified against opcodes.hh that SLESS+1=SLESSEQUAL,
  LESS+1=LESSEQUAL, FLOAT_LESS+1=FLOAT_LESSEQUAL.
- **Iteration-order provenance**: clean. Every `descend_ops`/`lone_descend`
  snapshot reads `Varnode::descend` which is a `SmallVec` filled by `push_back`
  in `add_descend`, faithfully reproducing the C++ `list<PcodeOp*>` beginDescend
  insertion order. No HashMap/HashSet anywhere in the ported logic.
- **Off-by-one / do-while / reverse iterators**: clean. `get_boolean_result`'s
  C++ `for(;;)` is a Rust `loop`; the `RuleXorSwap` `for i in 0..2`, the
  RuleThreeWayCompare 24-case switch, and the `1-i`/`1-const_slot` slot math all
  match. No do-while, no reverse iterators in scope.
- **Erase-while-iterating**: clean. RuleOrCompare's C++ live-iterator
  (`++iter` before modifying equalOp) is ported as a `descend_ops` snapshot;
  since the new eq_V/eq_W ops read V/W (not outvn) they never enter outvn's list,
  so the snapshot visits exactly the C++ set in the C++ order. RuleExpandLoad's
  `modifyAndComparison` (`++iter` before modify) is unreached (W6 seam).
- **Exception->Result partial-state parity**: see F2/LOSS-057. The W3
  `op_set_output` seam returns Err mid-build; RuleOrCompare leaves orphan
  eq_V/eq_W ops linked to V/W on abort. This is the LOSS-035 W3-seam partial
  state, made explicit and pinned, not a net-new divergence.

## Mechanical pass

- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`: none.
- bare `as` casts: all 14 are int-width/shift-amount casts that mirror an
  explicit C++ `(int4)`/shift or are provably in-range (bit positions < 64); no
  uncommented surprising cast.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean.
- W6 `resolve_typeop` flag words spot-checked against typeop.cc: COPY
  `unary|nocollapse` (:391), INT_XOR `binary|commutative` (:1410, NOT
  booloutput), BOOL_AND `binary|commutative|booloutput` (:1729), FLOAT_ADD
  `binary|commutative` (:1785), SUBPIECE `binary` (:2118) — all exact. `op_set_opcode`
  -> `set_opcode` clears opcode-derived bits then ORs the new flags, so
  `code()`/`isBoolOutput()` of a rewritten op are correct.

## Findings

- F1 (info, not a defect): RuleThreeWayCompare and RuleXorSwap are fully realized
  and correct end-to-end; RulePopcountBoolXor's body is faithful but cannot
  recognize a boolean leaf until W7 nzmask data is present.
       cpp: decompiler/cpp/ruleaction.cc:10161, :10640
       rust: rust/crates/kuna-decomp/src/ruleaction_8.rs:739, :1200
- F2 (minor, accepted -> LOSS-057): eight of twelve rules are cross-wave-seamed
  (W3 newUniqueOut/opSetOutput, W4 funcptr_align/FuncCallSpecs, W6
  floatSignManipulation/facing-types, W3-block findCondition) and no-op where C++
  rewrites; the seam helpers return the C++ guard's failing value (faithful
  no-op, no invented behavior). RuleOrCompare additionally leaves orphan
  eq_V/eq_W descend links on V/W when it bails at the W3 seam (partial-state
  manifestation of LOSS-035).
       cpp: decompiler/cpp/ruleaction.cc:9853, :9944, :10472, :10757, :10970, :10873
       rust: rust/crates/kuna-decomp/src/ruleaction_8.rs:136, :168, :188, :401, :1153, :1616
- F3 (info, non-physical, no new loss): `max_return = 8u64 * (in0_size as uintb)`
  widens before multiplying where C++ does a 32-bit `8 * int4`; divergent only
  for a Varnode size >= 2^28 bytes (impossible). Analogous in spirit to LOSS-039
  but unreachable; not ledgered.
       cpp: decompiler/cpp/ruleaction.cc:10686
       rust: rust/crates/kuna-decomp/src/ruleaction_8.rs:1262

## Re-derivation highlights (verifier-built oracle)

- RuleThreeWayCompare Form-1 `(z+z)-1` end-to-end: secondary `threeway == 0`
  encodes to form 14 -> case 14 -> INT_EQUAL(avn=b, bvn=a); `threeway s< 0`
  encodes to form 12 -> case 12 -> INT_LESS(avn=b, bvn=a). Both reproduced by the
  port and asserted (operand slots included). The `bvn=getIn(0)/avn=getIn(1)`
  inversion (ruleaction.cc:10205-10206) is preserved.
- testCompareEquivalence a/b-block asymmetry (ruleaction.cc:10008-10035): the
  b-block's OUTER `else return -1` (vs the a-block's INNER else) is faithfully
  reproduced — a LESS+LESSEQUAL pair (twoLessThan=false) with b1!=b2 returns -1,
  not 0; a-block non-adjacent constants return -1; a1+1==a2 returns the res=1
  swap signal.

## Adversarial tests added

rust/crates/kuna-decomp/tests/verify_w5_s3_rules_8.rs (8 tests, all pass,
committed on the branch as b64756e):
- w5s3r8_threeway_form1_eq_zero_collapses_to_int_equal
- w5s3r8_threeway_form1_sless_zero_collapses_to_a_lt_b
- w5s3r8_test_compare_equivalence_bblock_outer_else_returns_minus1
- w5s3r8_test_compare_equivalence_ablock_nonadjacent_const_returns_minus1
- w5s3r8_test_compare_equivalence_ablock_adjacent_swaps_res1
- w5s3r8_lzcount_non_power_of_two_size_blocks_without_mutation
- w5s3r8_lzcount_power_of_two_reaches_w3_seam_and_noops_cleanly
- w5s3r8_orcompare_seam_leaves_partial_descend_links_on_v_and_w

## Losses

LOSS-057 (this item's cross-wave seam ledger entry). No blocker or major
finding stands; the seams are faithful guard-failing no-ops, the two fully
realized rules transform correctly, and the partial-state on abort is the
already-ledgered LOSS-035 W3 seam.
