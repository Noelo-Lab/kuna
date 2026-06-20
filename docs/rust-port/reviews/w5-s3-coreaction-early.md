# w5-s3-coreaction-early
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier, round 2 — structural separation honored; porter notes/transcript not read)
date: 2026-06-12
gate: `cd rust && cargo test -p kuna-decomp` -> PASS for this item.
      - lib: 494 passed / 0 failed (includes the 13 in-module `coreaction_early` tests).
      - `tests/verify_w5_s3_coreaction_early.rs` (round-1 verifier set): 6 passed / 0 failed
        — `startcleanup_apply_snapshots_clean_up_index` (the F1 divergence trace) now PASSES,
        confirming the F1 repair.
      - `tests/verify_w5_s3_coreaction_early_r2.rs` (round-2 verifier set): 4 passed / 0 failed.
      - pre-existing, out-of-scope: `tests/corpus_bootstrap.rs::ldefs_pspec_cspec_parse_matrix`
        fails with "Could not find .sla file for x86:LE:64:default" — a missing build
        artifact (only RISCV/HCS08/ARM/etc. `.sla` present in this tree, no x86). The item
        commits touch only `coreaction_early.rs` + its tests and do not touch any `.sla`
        generation; this failure reproduces independent of the item.
      `cargo clippy -p kuna-decomp --lib`: clean. `--test verify_w5_s3_coreaction_early`: clean.
      `--test verify_w5_s3_coreaction_early_r2`: clean. `port-audit` crate: not present in this
      workspace (mechanical step N/A).

## Round-2 status

Round 1 REJECTed on a single MAJOR finding, **F1**: `ActionStartCleanUp::apply` no-op'd a
*realized* side effect (`Funcdata::start_clean_up`, which snapshots `clean_up_index =
vbank.getCreateIndex()`) behind a factually incorrect "not in the merged tree" seam comment.
The repair commit (`062a8b0`) replaced the no-op body with `data.start_clean_up(); 0` and
removed the false comment. **F1 is fixed and re-verified** two ways:
  - round-1 trace test `startcleanup_apply_snapshots_clean_up_index` now passes;
  - round-2 test `r2_startcleanup_snapshots_live_create_index` independently proves the body
    snapshots the *live* create index at call time and re-snapshots forward on a second apply
    (the round-1 no-op body would pin the index at the constructor default `0` forever).

`Funcdata::start_clean_up` (funcdata.rs:429-431, `clean_up_index = vbank.get_create_index()`)
is faithful to C++ `startCleanUp` (funcdata.hh:194). No regressions introduced by the repair
(the repair diff is two lines in one body).

## Scope reviewed

`decompiler/cpp/coreaction.{cc,hh}` (blob shas match the checklist: cc `28f6157f…`,
hh `bc278439…`), the 15 early-plane Action classes the item ports, in C++ definition order:
`ActionStart`, `ActionStop`, `ActionStartCleanUp`, `ActionStartTypes`, `ActionConstbase`,
`ActionSpacebase`, `ActionHeritage`, `ActionNonzeroMask`, `ActionVarnodeProps`,
`ActionUnreachable`, `ActionDoNothing`, `ActionLateDoNothing`, `ActionRedundBranch`,
`ActionDeterminedBranch`, `ActionNormalizeSetup`. Rust:
`rust/crates/kuna-decomp/src/coreaction_early.rs` (+ `coreaction_early/tests.rs`).

Boundary note (not a finding): the checklist `notes` say "ActionStart..ActionExtraPopSetup",
but this is "coreaction **1/5**" (the checklist itself states `coreaction.cc` is split into 5
stage-aligned items) and the porter drew the cut at the next class `ActionDeadCode` — the
first to need `FuncCallSpecs`/`getCallSpecs`/prototype-call-spec plane (W6/W7). The classes
between (`ActionStackPtrFlow`, `ActionConstantPtr`, the Merge family, `ActionMarkExplicit/
Implied`, `ActionNameVars`, `ActionDeadCode`, `ActionConditionalConst`, `ActionSwitchNorm`,
`ActionPrototypeTypes`…`ActionExtraPopSetup`) are explicitly enumerated as deferred in the
module docstring. Defensible fractional-port boundary at a clean dependency line; reported
for the wave assembler, not held against the verdict (consistent with round 1).

## Mandatory hunt list

- **Signedness** — clean. C++ `int4` loop indices (`i`, `j`, `count`) map to `i32`/`int4`;
  container sizes (`graph.getSize()`, `sizeIn/sizeOut`) are also `int4` in the Rust block API,
  so comparisons stay signed-vs-signed exactly as C++. No `int4` vs `uint4`/`uintb` compare in
  the ported bodies. `ActionDeterminedBranch`'s `val` is `uintb` but only enters a `!= 0` test
  (bool), never an ordered compare.
- **Integer widths** — clean. `uintb`=`u64` for the CBRANCH condition offset; `int4` for
  `num`/indices. No promotion-into-`intb` arithmetic in scope. `clean_up_index`/create-index
  are `uint4`=`u32` on both sides (funcdata.hh:76 ↔ funcdata.rs:137).
- **Wrapping** — clean. No `uintb`-lineage arithmetic beyond `i += 1`/`count += 1` on small
  signed indices and `val != 0`. `ActionRedundBranch`'s `i: i32 = -1` then `i += 1` is the
  faithful image of C++ `i = -1; …; ++i` and cannot under/overflow for realistic block counts.
- **Comparator totality** — N/A (no `operator<`/sort ported in this item).
- **Iteration-order provenance** — clean. Every loop walks `BlockGraph` by index
  (`for i in 0..size` / `while i < size`), mirroring C++ `for(i=0;i<graph.getSize();…)` over the
  same `Vec`-backed block list. No `HashMap`/`HashSet` anywhere (grep clean).
  `early_actions` enumerates the set in C++ definition order (test
  `early_actions_enumerates_full_set_in_cpp_order`).
- **Off-by-one / do-while / reverse iterators** — clean. `ActionRedundBranch`'s inner
  `for(j=1;j<sizeOut;++j)` with the post-loop `j != sizeOut` "all-exits-equal" check is exact
  (round-2 tests `r2_redundbranch_multi_out_distinct_targets_not_removed` and
  `…_all_exits_same_block_reaches_seam_cleanly` pin both the early-out and the all-same arms).
  The `i = -1` scan-reset (restart at block 0) is faithful and tested
  (`redundbranch_scan_reset_collapses_full_chain_in_one_pass`).
  `removing_creates_redundancy`'s `for(count=0;…)` / `if (count==sizeOut) return true` boundary
  is reproduced exactly, incl. the `inbl->sizeOut()==1 → continue` skip. No `do/while`, no
  reverse iterators. Empty-graph boundary (`getSize()==0`) tested
  (`r2_redundbranch_empty_graph_is_noop`).
- **Erase-while-iterating** — N/A in the *realized* paths (the C++ erase-during-walk surfaces
  `removeDoNothingBlock`/`removeBranch` are seamed; see LOSS-078). `ActionRedundBranch`'s
  realized `spliceBlockBasic` mutates the graph mid-walk and the `while i < bblocks_get_size()`
  re-reads the size each turn, matching C++ re-evaluating `graph.getSize()` per iteration.
- **Exception → Result parity** — `ActionRedundBranch` gates the splice on
  `splice_block_basic(bb).is_ok()`; C++ calls `spliceBlockBasic` unconditionally and always
  `count += 1`. The splice `Err`s only when the spliced block ends in a branch op (the
  `opDestroy` W3-op seam inside `splice_block_basic`, funcdata_block.rs:332-338). On a
  trailing-branch block the port skips the splice + scan-reset where C++ would splice. This is
  the `splice_block_basic` surface's own deferral, not new logic in `coreaction_early.rs`, and
  is folded into LOSS-078. The `.expect()` calls in `ActionDeterminedBranch` (op/varnode bank
  lookups, coreaction_early.rs:793/801) are on internally-consistent invariants (the id came
  from `bb_op_tail`/`get_in(1)` which already established existence), not attacker input;
  exercised without panic by `determinedbranch_*` (round 1).

## Findings

None standing. (Round-1 F1 is repaired and re-verified — see Round-2 status.)

### Non-findings verified faithful (re-derived this round)

- **F1 fix** (`ActionStartCleanUp::apply` → `data.start_clean_up(); 0`): realized, faithful to
  C++ coreaction.hh:65 + funcdata.hh:194. Pinned by two independent tests.
- Names / groups / flags: every class carries the exact `name()` and ctor flags (`0` except
  `donothing`=`rule_repeatapply`, `normalizesetup`=`rule_onceperfunc`). (`names_groups_and_flags_match_cpp`).
- `reset` overrides for `ActionStartTypes` (`setTypeRecovery(true)`) and `ActionNormalizeSetup`
  (`setNormalization(true)`) faithfully *replace* `Action::reset` without chaining, exactly as
  the C++ inline overrides; `set_type_recovery`/`set_normalization` match the C++ bit-twiddle
  (funcdata.hh:184/190 ↔ funcdata.rs:370/383).
- `ActionStartTypes::apply` once-only `count += 1` is exact (`startTypeRecovery` latch,
  funcdata.cc:180 ↔ funcdata.rs:362).
- `ActionRedundBranch` single-out splice gating (`sizeIn()==1`, `!isEntryPoint()`,
  `!isSwitchOut()`), splice-counts-and-resets-scan, and n-way "all exits to bl" detection:
  realized parts transcribed verbatim; verified by 5 adversarial tests across both rounds.
- `ActionDeterminedBranch` detection (lastOp==CBRANCH, slot-1 constant, `num =
  ((val!=0)!=isBooleanFlip()) ? 0 : 1`): exact; the `get_in(1)` null guards are benign Rust
  additions.
- `ActionLateDoNothing::removing_creates_redundancy`: dense pure graph read transcribed
  line-for-line incl. the `inbl->sizeOut()==1 → continue` skip and the
  break/`count==sizeOut` boundary.
- `ActionDoNothing` `clearDelayedDonothing()` sweep over all blocks: realized and faithful.

## Genuine seams (the accepted losses → LOSS-078)

The remaining bodies no-op because the required primitive is **grep-confirmed absent** from
the merged tree (no `fn` of any visibility): `startProcessing`/`stopProcessing`/`spacebase`/
`opHeritage`/`calcNZMask` (W3-op/W4); `removeUnreachableBlocks`/`removeBranch`/
`removeDoNothingBlock` + `isDoNothing`/`unblockedMulti`/`hasNoImmediateCopy` (W3-block);
`beginLoc`/`endLoc`/`getHeritagePass`/`fillinReadOnly`/`replaceVolatile`/`totalReplaceConstant`
(W3-vn; `total_replace_constant` exists only as a test fixture, funcdata_varnode.rs:888); the
W4 prototype/inject surface for `ActionConstbase`/`ActionNormalizeSetup`. These are defensible
cross-wave deferrals and are ledgered as **LOSS-078**. (Distinct from round-1 F1: F1 was a
realized primitive skipped behind a false seam — fixed.)

## Adversarial tests added

Round 1 (`e9bdc8e`, `tests/verify_w5_s3_coreaction_early.rs`): 6 tests — splice gating
(`…_into_entry_point`, `…_two_in_edges`, `…_scan_reset_collapses_full_chain`), determined-branch
walk, `starttypes_apply_counts_exactly_once_across_repeats`,
`startcleanup_apply_snapshots_clean_up_index` (now PASS).

Round 2 (`534672d`, `tests/verify_w5_s3_coreaction_early_r2.rs`): 4 tests —
`r2_startcleanup_snapshots_live_create_index` (direct F1-fix re-derivation),
`r2_redundbranch_multi_out_distinct_targets_not_removed` (n-way `j!=sizeOut` early-out),
`r2_redundbranch_all_exits_same_block_reaches_seam_cleanly` (duplicate n-way → removeBranch
seam without entering the splice arm), `r2_redundbranch_empty_graph_is_noop` (getSize()==0).
All pass; clippy clean.

## Verdict

ACCEPT-WITH-LOSSES — the round-1 MAJOR finding (F1) is repaired and re-verified; the realized
fraction of every ported body is faithful to the C++ oracle (hunt list clean); the remaining
no-op bodies are genuine cross-wave seams over grep-confirmed-absent primitives, ledgered as
**LOSS-078**. losses: LOSS-078.
