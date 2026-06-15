# w10-jts-chain
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
gate: `cd /tmp/kuna-jts-chain/rust && cargo test --workspace` -> 3291 passed / 0 failed
      (3288 pre-existing + 3 verifier adversarial); `cargo clippy --workspace --lib
      -D warnings` -> clean; C++ oracle `python -m kuna.run_tests --all --baseline
      docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK (untouched).

## Scope

End-to-end jump-table recovery + switch structuring + emit, built on the landed
EmulateFunction value-flow emulator.  Two commits (bfbec70 model/clone/stage/install,
fb3f674 structuring/emit).  Diff is `rust/`-only — zero C++/specs touched, so the
C++ oracle is untouched by construction (verified: only 3 file groups under
`rust/crates/kuna-decomp/` change).

## Faithfulness (verified line-by-line vs the C++ in scope)

- `JumpBasic::recoverModel` (jumptable.cc:1565) — findDeterminingVarnodes →
  findNormalized → maxtablesize bound → markFoldableGuards.  FAITHFUL
  (rust jumptable.rs:2092).
- `JumpBasic::buildAddresses` (jumptable.cc:1588) — the emulation-driven index
  enumeration over `EmulateFunction::emulatePath`, `funcptr_align` mask,
  `addressToByte`.  FAITHFUL (rust:2115).  `loadcounts`/`loadpoints` plumbed.
- `JumpBasic::buildLabels` + `backup2Switch` (jumptable.cc:1660/473) — the REVERSE
  emulation over `OpBehavior::recoverInput{Unary,Binary}`, `getEvalType()`
  binary/unary dispatch (mutually-exclusive flag bits → bit-test order == C++
  switch), `isReversible`→NO_LABEL, the truncation early-break + trailing fill.
  FAITHFUL (rust:2241/2284).  This is the real case-target recovery — NOT a lookup.
- `JumpBasic::analyzeGuards` / `findNormalized` / `findUnnormalized` /
  `findSmallestNormal` / `calcRange` (jumptable.cc:1062/1222/1616/1181/1136) —
  FAITHFUL (rust:1898/1996/2167/1843/1799).
- `CircleRange::pullBack(op)` (rangeutil.cc:1022) — the full unary/binary/SUBPIECE-
  zero-nzmask special case + the trailing nzmask intersect.  FAITHFUL
  (rust jumptable.rs:1463; new `CircleRange::set_mask`, the only rangeutil add).
- `RuleRangeMeld::applyOp` (ruleaction.cc:1358) — was a SEAM SHELL (returned 0);
  now a faithful transcription: pullBack(sub1/sub2), BOOL_NEGATE extra pull-back,
  functionalEquality size-realign, isHeritageKnown, intersect/circleUnion,
  translate2Op, the restype 0/1/2/3 rewrites.  FAITHFUL (ruleaction_1.rs:2364).
  Collapses `(V<c)||(V==c) => V<=c`, the guard the index-range model reads back —
  the load-bearing unblock (verified: `switchmulti` only recovers because of it).
- `Funcdata::cloneOp` / `cloneVarnode` / `cloneOpFrom` (funcdata_op.cc:616,
  funcdata_varnode.cc:254) — was a SEAM (errored); now closed.  The 10-flag
  clonable set is VERBATIM (funcdata_varnode.cc:262-266).  FAITHFUL.
- `opSetOpcode` OpCode→TypeOp resolver (`w6_type_op`) — the too-narrow 7-opcode
  fallback (bare flagless `binary`) now routes through the canonical
  `typeop::type_op_for` table (verbatim typeop.cc opflags).  This is a REAL fix:
  `INT_EQUAL`/`BOOL_OR` now carry `booloutput`, so `RuleRangeMeld::isBoolOutput()`
  sees the cloned guard.  The corrected spacebase unit test confirms it — STORE
  is `special` (typeop.cc), so `annotateRawStackPtr` SKIPs it
  (`getEvalType()==special && !isCall()`, varmap.cc:396); the old test passed for
  the WRONG reason on the buggy flagless-binary STORE.  Not a mask — a genuine
  latent-bug fix.
- `Funcdata::truncatedFlow` (clone half) / `stageJumpTable` / `recoverJumpTable` /
  `linkJumpTable` / `installJumpTable` / `earlyJumpTableFail` (funcdata_block.cc) —
  FAITHFUL (funcdata_block.rs:1034/1151/1100/974/985/1270).
- `FlowInfo::collectEdges` BRANCHIND arm + `recoverJumpTables` + the generateOps
  recovery loop + `truncateIndirectJump` (flow.cc:935/1429/789/745) — FAITHFUL
  (flow.rs:1328/1694/1467/1739).  The setMark/clearMark dedup, per-target edge,
  per-table newAddress re-fill — all transcribed.
- `BlockGraph::newBlockSwitch` / `grabCaseBasic` / `addCase` / `getExitLeaf` +
  `BlockSwitch::finalizePrinting` + `CaseOrder::compare` (block.cc:2043/3575/3546/
  3607, block.hh:921) — FAITHFUL (block.rs:2043/2091/2147/1288, funcdata_block.rs:
  82/97).  `IndexPair` Ord == C++ `operator<` (position,index); the linear scan over
  the SORTED `block2addr` reproduces `equal_range(compareByPosition)` group order,
  so multi-label `case 4:5:10:` ordering is correct.  Comparator is total
  (unsigned label, then int4 depth).  `stable_sort` == `sort_by` (stable).
- `PrintC::emitBlockSwitch` / `emitSwitchCase` / `opBranchind` (printc.cc:3470/3278/
  602) — FAITHFUL (printc.rs:2056/2133/opBranchind arm).  `getNumLabels`/`getLabel`
  inlined as the real `num_indices_by_block`/`get_label_by_index(get_index_by_block)`
  JumpTable chain — labels come from the recovered table, not constants.
- `ActionBlockStructure`'s bblocks-switch→jumpvec precompute (ADR 0001 per-arena
  ids, no Funcdata back-pointer in sblocks) — a justified adaptation, BTreeMap
  (ordered), honest "no table → leave unstructured" fallback.

## No special-casing (REJECT criterion — clean)

- `grep` over added `src/` lines: NO function-name / address / case-value /
  jump-target / table-byte hardcoding.  The magic bytes `0xa1ffffff` / `0xbad1abe1`
  appear ONLY in `tests/verify_w10_jts_chain.rs` (as negative assertions), NEVER in
  `src/`.  Switch fn names (`switchind`/`switchmulti`/…) appear NOWHERE in `src/`.
- The only 4+-digit hex literals in added src are the verbatim C++ jumptable.cc
  constants `0x10000` (jumptable.cc:1166) and `0xffff` (jumptable.cc:1742/2470).
- Recovery proven DATA-DRIVEN across THREE distinct switches through ONE code path:
  switchind → {0,1,2,3,4,5,10}, switchmulti → {0..6}, switchloop → {1,2,3,4,...},
  each with distinct real bodies.  A hardcoded value could not yield all three.

## Recovery is REAL (verified against the C++ oracle)

The C++ B5 oracle (switchind.xml stringmatches) pins `case 0:`..`case 5:`,
`case 10:`, `switch(val)`.  The Rust render (observed via the verifier tests)
produces exactly `case 0:`/`1:`/`2:`/`3:` then the collapsed `case 4: case 5:
case 10:` arm — the SAME label set, from genuine emulation + reverse-emulation.
switchmulti renders `<= 6`-guarded switch with 0..6; switchloop renders a
loop-nested switch with real `v? = v? + N` arm bodies.  +14 positive datatest
assertions (51→65), all REAL (the corpus stringmatch tally lists each switchind
`case N:` as a genuine-content positive pass).

## Hunt list

- Signedness: clean — index `as usize` casts over containers; `label` unsigned
  (uintb) in both, `depth` int4; no signed/unsigned compare hazard found.
- Integer widths: clean — `0x10000`/`0xffff` are uintb thresholds == C++.
- Wrapping: clean — `wadd`/`wsub`/`wrapping_*` used on the offset diffs
  (buildAddresses far-address check); no debug-panic path found.
- Comparator totality: clean — `IndexPair::cmp` and `CaseOrder::compare` are total
  orders matching C++ `operator<`.
- Iteration-order provenance: clean — NO HashMap/HashSet in added src; the
  switch→jumpvec map and complex-blocks set are BTreeMap/BTreeSet (ordered);
  `block2addr` sorted by IndexPair Ord; dead-list clone preserves SeqNum order.
- Off-by-one / do-while: clean — buildLabels truncation early-break + trailing
  fill match C++; the `for(j<maxpullback)` and `for(i<maxbranch)` bounds match.
- Erase-while-iterating: clean — collectEdges mark-cleanup walks the contiguous
  tail range it just pushed (== C++ backward walk that stops at `!=op`); trial
  table `remove(last)` leaves jumpvec in the C++ failure state.
- Exception→Result partial-state: see F1 (addCase throw → silent (0,false)) and
  the "Bad partial clone" throw→FailNormal note below.

## Mechanical pass

- `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in added src: NONE.
- bare `as`: all are `int4 as usize` container-index casts (no `// cast:` comment),
  consistent with the established crate convention (jumptable.rs carries 0
  `// cast:` across its whole body; the `// cast:` convention is reserved for
  semantically-interesting signed/unsigned reinterpretations, applied selectively
  across all prior ACCEPTed waves).  Not a finding under the de-facto convention.
- clippy `--workspace --lib -D warnings`: clean.

## Findings (all faithful-port nits or DECLARED seams; none blocking)

- F1 (low): `make_case` returns `(outindex=0, isdefault=false)` when `getInIndex`
  is -1, where C++ `addCase` THROWS `LowlevelError("Case block has become detached
  from switch")` (block.cc:3558).  Behavioral divergence only on a degenerate
  detached-case input (unreachable for a well-formed switch).
       cpp: decompiler/cpp/block.cc:3558
       rust: rust/crates/kuna-decomp/src/block.rs:2156
- F2 (low/cosmetic): `emit_block_switch` closes with `close_brace_indent` paired to
  `open_brace_indent`, where C++ `emitBlockSwitch` does `openBrace(...)` then a
  plain `print(CLOSE_CURLY)` (printc.cc:3486/3508).  Brace-emit primitive
  difference; gate-invisible (the oracle pins `case`/`switch` strings, not exact
  brace indentation; corpus tally + verifier tests pass).
       cpp: decompiler/cpp/printc.cc:3508
       rust: rust/crates/kuna-decomp/src/printc.rs (emit_block_switch close)
- F3 (loss, LOSS-167): `JumpBasic::foldInGuards` is SEAMED (returns false), so the
  switch's range guard is not folded into the BlockSwitch — switchind renders as
  `if (v1<=10){ switch(...) }` with the default body OUTSIDE the switch, where the
  C++ oracle folds it to `switch{ case...; default: }`.  The `default:` arm and the
  guard removal are deferred; the recovered labels/bodies are correct.  Declared.
- F4 (loss, LOSS-167): switchhide (hidden/multistage table) is unrecovered —
  `checkMultistageJumptables` + the generateOps outer do/while are SEAM(W4); it
  falls back gracefully (CALLIND, no crash).  Declared.
- F5 (loss, LOSS-167): symbol/markup propagation dropped (`constMarkup`/
  `copySymbolIfValid` in pullBack + RuleRangeMeld; `fd->warning` calls in
  buildLabels/recoverJumpTables).  Cosmetic.
- N1 (note, not a finding): C++ `stageJumpTable` throws `LowlevelError("Bad partial
  clone")` (propagated up, caught in recoverJumpTables) where the Rust returns
  `FailNormal`.  End state identical (no table installed); the throw site is a
  degenerate internal-error case.
- N2 (note): C++ reuses ONE `partial` Funcdata across a tablelist in a single
  recoverJumpTables call (stageJumpTable's `if(!isJumptableRecoveryOn())` guard);
  the Rust rebuilds a fresh partial per BRANCHIND.  Perf-only; per-table recovery
  is independent, so correctness is unaffected (one-table-per-fn corpus identical).

## Adversarial tests (committed to the branch, e61f9de)

In `rust/crates/kuna-decomp/tests/verify_w10_jts_chain.rs` (3 NEW, distinct from
the porter's 4), all passing:
- `verifier_switchind_multilabel_arm_is_in_oracle_order` — the `case 4:5:10:` arm
  is in block2addr/IndexPair sort order and shares one body (probes getIndexByBlock
  ordering + comparator totality).
- `verifier_switchloop_third_distinct_switch_recovers_real_body` — a THIRD distinct
  switch recovers its own labels {1,2,...} + real arm bodies (widens the
  no-hardcoding proof to 3 functions).
- `verifier_switchind_label_set_matches_oracle_cardinality` — exactly the oracle
  labels {0,1,2,3,4,5,10}, NO phantom `case 6/7/8/9/11/12:` (rules out a
  memoized/hardcoded label injection).

## Byte-parity (REJECT criterion — intact)

`w10_boolless_full_byte_parity_modulo_type_inference`,
`w10_eob_condconst_conn_forms_andand_byte_identical`, the readstruct symbol-naming
parity test — all GREEN.  No prior byte-parity function regressed.

losses: LOSS-167 (F3/F4/F5 — declared, bounded sub-behaviors of a chain that
otherwise delivers REAL end-to-end recovery: 5/6 switch files render a real
`switch` with REAL recovered labels; +14 positive datatest assertions).
