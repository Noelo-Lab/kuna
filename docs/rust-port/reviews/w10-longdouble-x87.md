# w10-longdouble-x87

verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (Opus 4.8 [1m], round 1)
date: 2026-06-17
branch: rport/w10-longdouble-x87 @ b8a26af (one commit over merge-base 9fdbef0 = rust-port HEAD)

## Decision

ACCEPT-WITH-LOSSES. The wave ports the C++ `ActionFuncLink::funcLinkInput` stack/JOIN
(x87/float10) parameter reassembly and the `ActionMarkExplicit::baseExplicit` addrtied
SUBPIECE/INT_ZEXT/PIECE fall-throughs BYTE-FAITHFULLY. It is not special-cased (independently
disproven via an integer-args parity test), passes the full workspace test suite, and carries
ZERO scored regression (independent full-suite diff: passing set byte-identical, regressed set
EMPTY). It is a faithful PARTIAL: the previously-DROPPED float10 CALL argument is now BUILT and
the render is reported, but not yet folded to the byte-clean oracle render — disclosed, pinned,
and recorded as LOSS-223. No REJECT trigger is met.

## Scope (full changed-file set)

`git diff rust-port...rport/w10-longdouble-x87` = exactly 5 files, Rust-only (+934/-58):
  - rust/crates/kuna-decomp/src/funcdata_op.rs            (+130) — `newSpacebasePtr` (funcdata.cc:273),
    `createStackRef` (funcdata_op.cc:459), `opStackLoad` (funcdata_op.cc:541)
  - rust/crates/kuna-decomp/src/coreaction_protos.rs      (+167/-1) — `funcLinkInput` IPTR_SPACEBASE +
    IPTR_JOIN (x87/float10 PIECE-recombine) arms (coreaction.cc:1514-1556)
  - rust/crates/kuna-decomp/src/coreaction_cleanup.rs     (+479) — `compare_order_ro` (op.cc:808),
    `find_root` (PieceNode::findRoot op.cc:854), `base_explicit` addrtied fall-throughs
    (coreaction.cc:3120-3146) + 4 hand-built-IR adversarial unit tests
  - rust/crates/kuna-decomp/tests/verify_w10_callarg_piece.rs       (+/-) — AT4 flipped to built-positive
  - rust/crates/kuna-harness/tests/verify_w10_longdouble_x87.rs     (+175, new) — AT1/AT2/AT3

No C++ / spec / python / baseline byte touched (`git diff --name-only` has zero non-rust
entries) -> the recorded C++ oracle 675/675 PARITY OK is structurally untouched
(independently re-confirmed: C++ `decomp_test_dbg` full suite reports 675 Success / 0 FAIL).

## Reserved-file check — CLEAN

Each reserved file is ABSENT from the changed-file set: coreaction_infertypes.rs, typeop.rs
(the concurrent enum-truncation wave), dynamic.rs, printc.rs, database.rs. (`coreaction_protos.rs`
and `funcdata_op.rs` are edited but are NOT on the reserved list.) The charter's premise that the
diff "edits ONLY subflow.rs/flow.rs/coreaction_cleanup.rs" does not match the implementation (the
port lives in funcLinkInput/funcdata factories + base_explicit, not subflow/flow) — but the
reserved-list constraint is independent of that premise and is satisfied. f0-flag-v2's
`mark_output_storage_addr_tied` (coreaction_cleanup.rs:179) is NOT modified by the diff (the diff
adds code at 519+/685+ only); all 5 `verify_w10_f0flag_v2_untie` tests pass. B0
(`universalaction_listing::decompile_tree_dump_is_byte_equal_to_oracle`) PASSES — the action
schedule is byte-identical to the oracle (no node added).

## Faithfulness — byte-faithful to the C++ (line-checked against oracle)

- `funcLinkInput` (coreaction_protos.rs vs coreaction.cc:1490-1557): the IPTR_SPACEBASE arm
  (opStackLoad + opInsertInput + setSpacebasePlaceholder/spacebase=0), the IPTR_JOIN arm
  (findJoin -> index pick from getPiece(0)/getPiece(last) IPTR_SPACEBASE -> stripJoinPiece ->
  opStackLoad of the stack piece + newVarnode of the remainder + CPUI_PIECE concat with the
  index-ordered slots + newUniqueOut + opInsertBefore + opInsertInput), the index<0 fall-through
  to the plain newVarnode insert, and the trailing `if (spacebase) createPlaceholder` guard — all
  transcribed line-for-line. The only residual is `createPlaceholder` (a pre-existing fspec.rs
  seam) which the locked-stack-param path never reaches (it clears `spacebase`).
- `opStackLoad`/`createStackRef`/`newSpacebasePtr` (funcdata_op.rs vs funcdata_op.cc:459/541,
  funcdata.cc:273): exact op construction, INT_ADD base offset via byteToAddress, LOAD with
  newVarnodeSpace(getContain()), opInsertAfter(addout->getDef()). The SEGMENTOP rewrite is the
  segmented-arch branch (segdef!=0); the datatest stack space is not segmented (W8 userops seam,
  unreachable here).
- `base_explicit` addrtied block (coreaction_cleanup.rs vs coreaction.cc:3120-3146): the SUBPIECE
  `overlapJoin == getIn(1)->getOffset()` copymarker pre-check (return -1), the loneDescend
  dispatch, the INT_ZEXT `(!isAddrTied)||(0!=contains)` arm with the implied fall-through, the
  PIECE `findRoot`/`isPartialRoot` arm with the implied fall-through, and the `else return -1` —
  exactly. `overlap_join`/`contains`/`is_partial_root`/`is_proto_partial`/`is_addr_tied` are all
  pre-existing faithful ports, not stubs.
- `find_root` (PieceNode::findRoot op.cc:854): the forward PIECE walk with the big-endian
  slot fixup (`isBigEndian() == (slot==1)`), the `addr + getIn(1-slot)->getSize()` offset, the
  join-address `renormalize`, the `addr == vn->getAddr()` match, and the compareOrder earliest-op
  tiebreak — faithful. `compare_order_ro` is a sound read-only mirror of `PcodeOp::compareOrder`
  (op.cc:808): same-parent execution-order compare, else the lowest-common-dominator pick.
  The C++ delegates the cross-block case to `FlowBlock::findCommonBlock` (block.cc), a
  transiently-mutating (mark/clear, restored) LCA walk; the immediate-dominator relation is a
  tree, so the first intersection of the two dominator chains is the unique LCA — the
  no-mutation set-then-walk computes the identical block. `op->compareOrder(pieceOp)` truthiness
  is preserved as `!= 0`, argument order maps correctly.

## Special-casing hunt — CLEAN

Whole-line grep of every added production line across funcdata_op.rs / coreaction_protos.rs /
coreaction_cleanup.rs for float10/TYPE_FLOAT/metatype/function-name (writeLongDouble/dldlll/
passmany)/value (==N)/magic-offset keying: ALL EMPTY in production code. The only hex-offset hits
are storage addresses inside the `#[cfg(test)]` hand-built-IR fixtures (coreaction_cleanup.rs
mod tests @ line 2005+) — IR geometry, not corpus keying. Zero string literals in the added
production code; the decision keys only on space-type (IPTR_SPACEBASE/IPTR_JOIN), opcode
(SUBPIECE/INT_ZEXT/PIECE), and addrtied/CONCAT geometry.

INDEPENDENT disproof of special-casing: AT2 (`dldlll`, mixfloatint.xml) — the SAME opStackLoad
IPTR_SPACEBASE arm rebuilds two pure-INTEGER stack arguments that the baseline DROPPED:
  - baseline: `dldlll(7.0,a0,8.0,a0 + 1)`  (stack args gone)
  - branch:   `dldlll(7.0,a0,8.0,a0 + 1,a0 + 2,a0 + 3)`  (stack ints recovered)
A float10-keyed implementation could not recover these. The base_explicit unit test
`adversarial_no_special_casing_integer_piece_same_decision` drives identical PIECE geometry with
integer (non-float) types to the SAME implied verdict.

## Real parity (independently reproduced vs C++ oracle B5)

Dumped longdouble.xml (KUNA_DUMP) from the branch `decomp_test_dbg`, the baseline (rust-port HEAD),
and the C++ oracle:
  - C++ oracle B5: `writeLongDouble(ldarr,x); writeLongDouble(ldarr,ptrldstr->a);
    writeLongDouble(ldarr,ptrldstr->b); writeLongDouble(ldarr,firstval.a);` (clean — passes #3/#5/#6/#11).
  - baseline (rust): `writeLongDouble(ldarr);` x4 — the float10 second arg DROPPED.
  - branch (rust): `writeLongDouble(ldarr,(undefined10)CONCAT(z,CONCAT(v1,x)));` /
    `...,(undefined10)CONCAT(v3,*(xunknown8 *)(...)));` / `...,v2);` /
    `...,(undefined10)CONCAT(v2,firstval));` — the arg is now BUILT (CONCAT present, not dropped).
  - longdouble #3/#5/#6/#11 FAIL on BOTH baseline and branch (the clean render needs the deeper
    stack-slot SSA-coherence seam) -> the scored set does not move. This is the disclosed PARTIAL
    (LOSS-223): CONCAT built, render reported, not yet folded clean.

## No-regression — CONFIRMED (independent full-suite diff)

Ran the FULL datatest suite (83 files, 675 assertion lines) on BOTH the baseline binary
(main tree @ 9fdbef0 = rust-port HEAD, rebuilt `--bin decomp_test_dbg`) and the branch binary,
sorted the `Success --` lines, and diffed the passing SETS (not just counts):
  - BASELINE passing = **383** (matches the charter floor exactly).
  - BRANCH passing   = **383**.
  - REGRESSED SET (baseline-pass -> branch-FAIL) = **EMPTY**. No masked regression.
  - GAINED SET = **EMPTY**. The passing sets are byte-identical. +0 net (matches the commit).
  - `cargo test --workspace` (independently re-run): **3648 passed / 0 failed** (zero `[1-9]
    failed`; exit 0).
  - new/updated test binaries: `verify_w10_longdouble_x87` 3/0 (AT1/AT2/AT3 all PASS, all live
    oracle/datatest dumps), `verify_w10_callarg_piece` 4/0 (AT4 flipped to built-positive, PASS),
    `verify_w10_callarg_piece_switchhide_guard` unchanged.
  - `cargo clippy --workspace --lib` clean (zero warnings/errors).
  - B0 `universalaction_listing` 1/0 (action schedule byte-identical to oracle).
  - C++ oracle full suite reports 675 Success / 0 FAIL (untouched).

byte-identity (branch-vs-baseline render dumps, KUNA_DUMP):
  - boolless: IDENTICAL. readstruct (nestedoffset.xml): IDENTICAL. condconst: IDENTICAL.
  - switch datatests: 7/7 IDENTICAL (ifswitch, switchind, switchloop, switchmulti, switchreturn,
    switchhide — switchhide is byte-identical here, the callarg-piece delta is already merged in
    the base).

## Findings (LOSS-class — none is a REJECT trigger)

F1 (LOSS, disclosed+pinned) — the float10 (and stack-int) CALL argument is BUILT but the float10
  render is not folded to the byte-clean oracle form. `writeLongDouble(ldarr)` (dropped) ->
  `writeLongDouble(ldarr,(undefined10)CONCAT(...))` (built) vs oracle `writeLongDouble(ldarr,x)`.
  NOT a scored regression (#3/#5/#6/#11 FAIL on both baseline and branch; passing set byte-
  identical). Directionally CLOSER to the oracle (the arg is reconstructed instead of dropped).
  DISCLOSED + PINNED by `verify_w10_longdouble_x87.rs` AT1 (asserts the arg is built without
  over-asserting the final `x` render) and the file docstring. Recorded as LOSS-223. Restoration
  is automatic when the 10-byte stack-slot SSA-coherence / wide-float merge prerequisites land.

## Adversarial tests (8 committed; >= 3 required)

  - AT1 at1_float10_callarg_concat_built — PASS. `passmany`: `writeLongDouble(ldarr)` gone, second
    arg present (live longdouble.xml dump).
  - AT2 at2_no_special_casing_integer_stack_args_built — PASS. The anti-special-casing guard:
    `dldlll` pure-INTEGER stack args (`a0 + 2`,`a0 + 3`) recovered by the same opStackLoad arm.
  - AT3 at3_register_only_call_unaffected — PASS. boolless register-only call untouched.
  - adversarial_wide_piece_recombine_renders_implied — PASS. Wide PIECE recombine temp stays
    IMPLIED (findRoot reaches a distinct non-partialRoot root -> fall through to desccount).
  - adversarial_non_piece_addrtied_stays_explicit — PASS. addrtied INT_ADD user -> explicit (-1).
  - adversarial_no_special_casing_integer_piece_same_decision — PASS. Identical PIECE geometry,
    integer types -> SAME implied verdict (geometry, not type, decides).
  - adversarial_subpiece_overlapjoin_copymarker_explicit — PASS. SUBPIECE overlapJoin==truncation
    -> copymarker explicit, taken before loneDescend.
  - AT4 at4_widefloat_callarg_concat_built (verify_w10_callarg_piece.rs) — PASS. Flipped from the
    gated negative guard to the built-positive guard.

## Losses (recorded in losses.md)

LOSS-223: float10 CALL-arg built but not folded to the clean render (disclosed+pinned) — see F1.
A faithful PARTIAL (CONCAT built, render reported) with ZERO regression; restoration is automatic
when the stack-slot SSA-coherence prerequisite lands.
