# w10-structure-printbody
verdict: ACCEPT
verifier: claude-opus-4-8[1m] (independent verifier)
date: 2026-06-13
round: 2 (Round-1 was REJECT/vacuous; this re-port closes the gap)
branch: rport/w10-structure-printbody (cc9420b; verifier tests added at 0d4d7ac in /tmp/kuna-struct)
gate: rust-engine datatest count rises **with real positive parity** — boolless's
      min=1/max=1 assertion `if (dat_52 <= 10)` now PASSES by genuine parity (was
      the exact Round-1 REJECT condition). Corpus stringmatch tally over all 83
      datatests: **2 REAL positive (min>=1) passes** (Round-1 had 0), 39 negative,
      631 fail-positive (the still-unported deeper layers). cpp oracle untouched
      675/675. See gate evidence below.

## What changed since Round 1 (REJECT -> ACCEPT)

Round 1 REJECTed for one reason only: the count rise was vacuous — all 19 new
rust datatest passes were `min=0/max=0` negative ("must-NOT-appear") matches; 0
real positive assertions passed, and boolless's own positive `if (dat_52 <= 10)`
FAILED (rust emitted `if (10 < dat_52)` from un-collapsed IR). The port code was
already judged FAITHFUL and NOT special-cased; the deliverable (real parity) was
just absent.

Round 2 closes the *analysis layer* Round-1 named as the honest work order (NOT
the printer). It un-seams four real decompiler passes so the IR collapses
`INT_LESS(#0xa, i0x52)` into a genuine `INT_LESSEQUAL(i0x52, #0xa)` before
printing:

1. **`Funcdata::opNormalizeFlip`** (funcdata_op.cc:1337) — line-for-line
   transcription: `loneDescend` guard, `get_booleanflip`, `opSetOpcode` via the
   canonical op-info table, conditional `opSwapInput`, `flipFlag(boolean_flip)`,
   then `replaceLessequal` for the LESSEQUAL/SLESSEQUAL result.
2. **`Funcdata::opFlipCondition`** (funcdata.hh:498) + `RuleCondNegate` wiring
   (ruleaction_4.rs) — the previously-`// SEAM(W3-block)` early-returns are now
   live calls.
3. **`Funcdata::removeBranch`/`branchRemoveInternal`/`opZeroMulti`**
   (funcdata_block.cc:195/213/238) — faithful ports, wired into
   `ActionRedundBranch` (coreaction_early.rs) so the degenerate same-target
   CBRANCH is removed.
4. **`resolve_typeop`** (condexe.rs) — the W6 op-info seam is closed by routing
   `opSetOpcode` through the already-ported `typeop::type_op_for` (= C++
   `glb->inst[opc]`); the `inst[]` table was ported in W6, not fabricated here.
5. **`present_lessequal`** wired into `Architecture` (seams.rs, default-on per
   `architecture.cc:1450` DIV-2) and `ActionPresentCompareForm` gated on it
   (kuna_compareform.rs:111, matching `kuna_compareform.cc:70`) — the kuna
   stage-model action that renders the `< 0xb` canonical form back as `<= 10`,
   exactly as the cpp oracle (whose datatest baseline is DIV-2-repinned).
6. **`BlockBasic::negateCondition` data-flow half** realized:
   `CollapseStructure::negate_condition_rec` ports the C++ recursive subtype
   dispatch (BlockCopy/BlockList/BlockCondition/base, block.cc:294/2355/3015/3071
   + block.hh:547) — topology `swapEdges` in place, the leaf CBRANCH op-flag flip
   deferred to `pending_flips` and applied (XOR-reduced) after collapse via
   `block_basic_negate_lastop`. This supersedes (does not break) the documented
   LOSS-100 deferral.

## Why ACCEPT (the three load-bearing proofs)

**(A) Real parity, not vacuous (the Round-1 REJECT condition is gone).**
Independent corpus harness (`verify_w10_struct_corpus.rs`, committed 0d4d7ac):
evaluates every `<stringmatch min max>` over the rust engine's `decompile_func`+
`print_c` output across all 83 datatests, the same `min<=hits<=max` scoring the
C++ harness uses. Result: **PASS positive (min>=1, REAL content) = 2**
(`boolless::if (dat_52 <= 10)` and `retspecial::return;`, both min=1/max=1),
PASS negative = 39, FAIL positive = 631. Round-1 had 0 real positive passes;
Round-2 has 2. The headline assertion the item is named for is one of them.

**(B) The `<=` is produced by REAL IR analysis, NOT a special-cased printer
(FOCUS 2/3).** Direct IR dump of the analyzed boolless (verifier-run
`zz_dump_boolless_cfg`): the IR carries
`u0x00000600:1 = CPUI_INT_LESSEQUAL(i0x52:1, #0xa:1)` — a genuine op produced by
the un-seam chain — and the printer renders it as `if (dat_52 <= 10)`. No raw
`INT_LESS` survives. A printer hardcoded to the oracle would emit `<= 10` over a
raw `INT_LESS(#0xa, ..)`; this one renders whatever the IR holds. The committed
tripwire `w10_boolless_lessequal_comes_from_real_ir_not_printer` asserts exactly
this (INT_LESSEQUAL present AND INT_LESS absent in the alive op list) and PASSES.
The collapse is also bootstrap-independent (my corpus harness uses a *different*
bootstrap than the boolless test and still scored the positive match) — a
special-case would be bootstrap-sensitive.

**(C) FOCUS 2 grep: no special-casing in any production source file.** Grep of
the full branch diff (`rust-port...cc9420b`) restricted to `crates/*/src/*` for
`boolless`/`dat_52`/`0xa00*`/`name == "..."`/`hardcod`/`special-case`: **empty**.
Every such token lives only in test files (`run_full("boolless",..)`, asserts,
the `CPP_B5_ORACLE` constant) or doc comments. Zero in production control flow.

## Faithfulness of the named modules (FOCUS 1)

- **ActionBlockStructure / CollapseStructure::negate_condition_rec** — faithful
  port of `FlowBlock::negateCondition` subtype dispatch. BlockCopy pushes its
  `copy` (bblocks BlockBasic) to pending_flips + swaps the structured node's
  edges; BlockList recurses into last block; BlockCondition distributes NOT to
  both children + `flip_condition_opcode`; base swaps edges. The deferred
  pending_flips → `block_basic_negate_lastop` reproduces all three C++ BlockCopy
  effects (boolean_flip + fallthru_true op-flag flip, bblocks `swap_edges`,
  structured `swap_edges`). The XOR-reduce is sound: both `flipFlag` (XOR) and
  `swapEdges` (involution) are self-inverse, and the deferred half is bblocks-only
  (no influence on the structured collapse decisions, which use the in-place
  structured `swap_edges`). Applying pending_flips even on the collapse_all Err
  path matches C++ in-place semantics (flips committed before an abort persist).
- **merge HighVariable formation (ActionMarkExplicit/MarkImplied)** — unchanged
  from Round-1's already-faithful judgment (coreaction.cc:3105-3185 / :3519,
  check order + max_implied_ref=2 + BTree-ordered iter_loc/iter_def). Not
  re-touched in Round 2.
- **printc body driver (emitBlock{Graph,Copy,Basic,Ls,If}/emitStatement/
  emitExpression/docFunction)** — unchanged from Round-1's faithful judgment
  (printc.cc:2790-3098); the printer still renders only what the IR hands it,
  proven by (B).
- **opNormalizeFlip / opFlipCondition / removeBranch / branchRemoveInternal /
  opZeroMulti** — line-for-line transcriptions verified against
  funcdata_op.cc:1337, funcdata.hh:498, funcdata_block.cc:195/213/238.
  `branchRemoveInternal`'s MULTIEQUAL loop uses a `bb_ops` **Vec snapshot** (head-
  to-tail walk) so the mutating `opRemoveInput`/`opZeroMulti` inside the loop
  cannot disturb iteration; the snapshot visits the same leading MULTIEQUAL run
  the C++ live `beginOp()..break-on-non-MULTIEQUAL` loop does (op_zero_multi never
  reorders the block list).

## The mandatory hunt list (Round-2 changes)

- **Signedness**: CLEAN. New params `num`/`blocknum` are `int4` edge indices
  passed through to `get_out`/`get_in_index` (int4 in C++); no signed/unsigned
  mix. `nin` compared by equality only.
- **Integer widths**: CLEAN. No new width-narrowing arithmetic; opcode lookups go
  through the canonical `type_op_for` table.
- **Wrapping**: CLEAN. `opZeroMulti` uses `nin == 0`/`nin == 1` equality (no
  arithmetic); `branchRemoveInternal` does no arithmetic on indices.
- **Comparator totality**: N/A. No new comparator; no `partial_cmp`/float
  `unwrap` in the diff (the one `.unwrap()` is a test assertion).
- **Iteration-order provenance**: CLEAN. `pending_flips` XOR-reduce uses a
  `BTreeMap` (deterministic); `bb_ops` is a head-to-tail Vec snapshot
  (block-list order). No HashMap/HashSet in the diff.
- **Off-by-one / do-while / reverse iterators**: CLEAN. The MULTIEQUAL loop is a
  forward `for op in snapshot` with `break` on the first non-MULTIEQUAL, matching
  the C++ forward loop. No do-while introduced.
- **Erase-while-iterating**: CLEAN. `branchRemoveInternal` iterates a Vec
  snapshot; `op_zero_multi`/`op_remove_input` mutate ops in place without
  reordering the block list, so snapshot == live visited set.
- **Exception -> Result partial-state parity**: ACCEPT (was Round-1 F2). The
  `collapse_all()` Err arm still returns 0, but this is the **documented
  switch-node-construction loss** (losses.md, the `rule_block_switch` `?` abort)
  — `collapse_all` returns Ok for boolless and every now-passing corpus function
  (no switch). No panic from any Round-2 path across all 83 datatests (verified).
  pending_flips are applied before the Err check, matching C++ in-place flips.

## Mechanical pass

- grep HashMap/HashSet/todo!/unimplemented!/sort_unstable in the Round-2 diff:
  CLEAN (none).
- bare `as` casts in the Round-2 production diff: NONE. The three `as` casts in
  the diff are all in `block/tests.rs` (test-only index<->int4 / len-as-usize).
- cargo clippy -p kuna-decomp --tests: 1 error + warnings, **0 in any file this
  branch touches** — the single error is `heritage.rs:2586` (a pre-existing
  `assert!(... || true)` in an untouched test file; heritage.rs is NOT in this
  branch's diff). The diff's own code (incl. my verifier test) is clippy-clean.
- cargo test --workspace: **3162 passed / 0 failed** (incl. the 5 committed w10
  tests + my verifier corpus harness).
- C++ oracle (main tree, byte-identical cpp; branch makes ZERO changes to
  decompiler/cpp/ or specs/): **207/207 unit + 675/675 datatest, PARITY OK**.

## Findings

- F1 (resolved, was Round-1 blocker): real positive parity achieved —
  `if (dat_52 <= 10)` passes by genuine IR collapse (INT_LESSEQUAL in the analyzed
  IR), 2 real positive corpus passes vs Round-1's 0.
- F2 (informational / accepted loss): `collapse_all()` Err -> return 0 is the
  documented switch-node-construction loss (losses.md `rule_block_switch` abort);
  unreachable for every now-passing corpus function. No new loss id needed.
- F3 (nit, pre-existing): a couple bare `as` casts in test code lack `// cast:`
  comments. Test-only, not a port-correctness issue.

## Adversarial / verifier tests

Committed on the branch (porter, 8eddeda/cc9420b) — re-run, all PASS:
- block::tests::w10_build_copy_from_remaps_edges_cross_arena
- block::tests::w10_build_copy_from_empty_source
- print_b5_boolless::w10_boolless_lessequal_comes_from_real_ir_not_printer
  (anti-special-casing tripwire: INT_LESSEQUAL present AND INT_LESS absent)
- print_b5_boolless::w10_boolless_positive_datatest_assertion_now_real
- print_b5_boolless::w10_boolless_if_body_contains_assignment

Added by this verifier (committed 0d4d7ac in /tmp/kuna-struct):
- verify_w10_struct_corpus::verify_w10_corpus_stringmatch_tally — runs the rust
  engine over all 83 datatests and tallies REAL positive vs vacuous negative
  passes (the independent reproduction of the Round-1 measurement); asserts
  >= 1 real positive pass. PASS (2 real positives).

## Gate evidence

- corpus stringmatch tally (rust engine, all 83 datatests, 675 assertions):
  PASS positive (min>=1) = 2; PASS negative (min=0/max=0) = 39; FAIL positive =
  631; FAIL negative = 3.  Round-1 PASS positive was 0.
- boolless analyzed IR: `CPUI_INT_LESSEQUAL(i0x52:1, #0xa:1)` (real op), renders
  `if (dat_52 <= 10) { ACC = 1; } return ACC;` — assignment nested in if-body.
- no panic from any Round-2 code path (opNormalizeFlip/opFlipCondition/
  RuleCondNegate/opZeroMulti/branchRemoveInternal/negate_lastop) across the corpus.
- cargo test --workspace: 3162 passed / 0 failed.
- C++ oracle: 207/207 unit + 675/675 datatest, PARITY OK (cpp untouched).

losses: none newly required (ACCEPT). The collapse_all Err path is the
already-documented switch-construction loss; LOSS-100 (negateCondition deferral)
is partially superseded by this re-port but left in place (append-only). MAIN-tree
losses.md max id at review time: LOSS-136.
