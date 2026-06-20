# w7-s7-blockaction
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, structural)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> lib suite 1217/1217 PASS (incl. 13
  blockaction::tests); verifier suite `verify_w7_s7_blockaction` 8/8 PASS;
  `cargo clippy -p kuna-decomp --lib -- -D warnings` clean. One unrelated
  pre-existing failure: `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`
  (panics needing an x86 `.sla` it cannot resolve) — present on the `rust-port`
  base, NOT in this item's diff (diff touches only blockaction.rs, blockaction/
  tests.rs, lib.rs), an environment/build-artifact issue independent of S7.

blob-sha: matches checklist (cc b1bec9b…, hh 9f4cb5a…), re-verified by
`git hash-object`. Mechanical greps: no `todo!`/`unimplemented!`/`HashMap`/
`HashSet`/`sort_unstable` in blockaction.rs. (No `port-audit` crate exists in
this tree; the `cargo run -p port-audit` mechanical step is N/A.)

## hunt list
- Signedness: CLEAN. Block API (`size_out`/`size_in`/`get_index`/`get_size`/
  `get_visit_count`) is uniformly `int4` (i32) and indexers take `int4`, so the
  C++ `int4`-indexed loops port literally. The two signed-reverse loops
  (`LoopBody::update` rs:271 `size_out()-1 .. while i>=0`; `label_exit_edges`/
  `emit_likely_edges` `tails.len() as int4 - 1 .. while i>=0`) keep `int4`, so
  an empty container yields `-1` and the loop is skipped — no usize underflow.
  `compare_head` is a signed 3-way (verified for negative indices). Adversarial
  test `compare_head_is_signed_three_way_and_drives_search`.
- Integer widths: CLEAN. `int4`=i32, `uint4`=u32, `uintb`=u64 per ADR-0003;
  `F_ACTIVE`/`F_TERMINAL` are `uint4` flag masks matching C++. No promotion-
  width traps (no `int4*int4`-into-`intb` arithmetic in this file).
- Wrapping: CLEAN. The only arithmetic on counts/indices is `+= 1`/`-= 1` on
  values bounded by container sizes (`pathout`, `depth`, `edgelump`,
  `visit_count`, `siblingedge`) — no `uintb`-lineage wrapping arithmetic.
- Comparator totality: CLEAN. `compare_ends` (head idx, then tail[0] idx) and
  `bad_edge_less` (exitproto idx, then branchpoint-top idx, then pathout) are
  strict total orders (the tying triples are unique per record here); used with
  Rust stable `sort_by`. `compare_final` is the `compareFinal` predicate, not a
  sort key. No float compares anywhere. Adversarial test
  `compare_ends_orders_by_head_then_tail_and_is_antisymmetric`.
- Iteration-order provenance: CLEAN. C++ `list<LoopBody>` insertion order is
  preserved by the compaction in `order_loop_bodies` (keep-then-remap over
  `0..loopbody.len()`); `looporder` is the `std::sort`-by-`compare_ends` vector,
  ported as a `Vec<usize>` sorted by the same comparator (the C++ uses
  non-stable `std::sort` but the key is unique pre-merge, so stable/unstable is
  irrelevant). The active list (`activetrace` Vec<usize>) reproduces the C++
  `list<BlockTrace*>` push_back/erase order; the `activeiter` is realized by
  value-search (`active_pos`). No `HashMap`/`HashSet` in ported logic.
- Off-by-one / do-while / reverse iterators: CLEAN. All three do-while loops
  (`markPath`, `distance`, `processExitConflict`'s inner) port to `loop`+break /
  `while let Some` keeping at-least-once semantics. `LoopBody::find` binary
  search ports the `int4 max=size-1` / `min<=max` / `min=mid+1` / `max=mid-1`
  boundary exactly (empty → `max=-1` → returns None). `emit_likely_edges` reverse
  tail loop + holdin/holdout deferral matches blockaction.cc:386-411 edge-for-
  edge (the `iter==enditer && outbl==exitblock` hold, and emitting the hold edge
  before tail[0]'s back-edges). Adversarial tests
  `find_empty_lookup_returns_none_no_underflow_panic`,
  `find_single_element_hit_and_miss`, `find_three_elements_walks_both_branches`.
- Erase-while-iterating: CLEAN. `orderLoopBodies`' erase of subsumed loopbodies
  (cc:1159-1167) is ported as collect-keep + index remap (correct surviving
  order). `mergeIdenticalHeads` `resize()` → `truncate()` with the same skip/
  swap-down semantics (the unconditional `looporder[0]` read is safe under the
  `!loopbody.empty()` caller guard). `pushBranches`' active-list erase via
  `remove_active` (`Vec::remove` + value-search reposition) matches the C++
  list-iterator stability: the miss path never removes (Vec indices stable), and
  retire/open/removeTrace each recompute `current_activeiter` fresh exactly as
  C++ returns a fresh iterator. `check_exit_block` advances `next` before the
  (read-only) body, matching `++iter` placement. Adversarial tests
  `merge_identical_heads_{accumulates_tails_and_truncates,no_merge_is_identity,
  all_same_head_collapses_to_one}`.
- Exception -> Result parity: CLEAN/accounted. `selectGoto`'s
  `throw LowlevelError("Could not finish collapsing block structure")` →
  `Err(KunaError::lowlevel(...))` after the same `clipExtraRoots()` guard, same
  partial state (gotos already marked on the graph). The two NEW `Err` exits
  (`new_block_switch_seam`, `ConditionalJoin::execute`) are seam markers, not
  C++ catch sites — recorded as LOSS-100/LOSS-101 (the `?` propagation aborts
  the collapse for switch CFGs, which is the documented seam, not a parity bug).

## findings
- (no blockers, no majors)
- F1 (informational, NOT a finding): bare `as` casts at blockaction.rs
  463/502/629/1048/1301 lack a `// cast:` comment. All are `int4 -> usize`
  conversions on values non-negative by construction (counts, ≥0 loop indices,
  ≥0 `pathout`), faithful to C++ `int4` indexing. The `// cast:` convention has
  0 occurrences in the already-accepted adjacent `block.rs`/this file, so
  flagging it would be inconsistent with the established convention; noted only.

## losses (cited; appended by this verifier to MAIN losses.md)
- LOSS-100: `negateCondition` ports the edge-swap (topology) only; the
  `BlockCopy`->`BlockBasic` CBRANCH op-flag flip and the change-count are
  deferred (so `dataflow_changecount` is always 0 in the standalone path). The
  collapse *decisions* (true/false out ordering) are faithful.
- LOSS-101: `isComplex` returns the `FlowBlock` base default (`true`), not the
  `BlockCopy`->`BlockBasic` statement count. Observable: `ruleBlockOr` never
  fires; `ruleBlockWhileDo` always uses overflow syntax (and flips the
  `(i==0)!=overflow` negate choice).
- LOSS-102: `ruleBlockSwitch` ports the full match/exit/skip-edge decision but
  `newBlockSwitch` surfaces as `Err` (needs getExitLeaf + grabCaseBasic, W4).
- LOSS-103: `ConditionalJoin::execute` surfaces as `Err`; only the (read-only)
  match path is ported (needs opSetOutput + opSetOpcode).
- LOSS-104: the seven structuring `Action`s are shells/partial no-ops pending
  block.rs/funcdata seams (notably `ActionBlockStructure` stops at the
  cross-arena `buildCopy` seed and never runs the otherwise-fully-ported
  `CollapseStructure`).

## adversarial tests added
rust/crates/kuna-decomp/tests/verify_w7_s7_blockaction.rs (8 tests, all pass):
- find_empty_lookup_returns_none_no_underflow_panic
- find_single_element_hit_and_miss
- find_three_elements_walks_both_branches
- compare_head_is_signed_three_way_and_drives_search
- merge_identical_heads_accumulates_tails_and_truncates
- merge_identical_heads_no_merge_is_identity
- merge_identical_heads_all_same_head_collapses_to_one
- compare_ends_orders_by_head_then_tail_and_is_antisymmetric

## rationale
The CFG-topology core that this item is *about* — `FloatingEdge`, `LoopBody`
(find/merge/extend/findExit/orderTails/labelExitEdges/emitLikelyEdges), the whole
`TraceDAG` (branchpoint/blocktrace arena, active-list iterator realization,
bad-edge scoring, retire/open/removeTrace), and `CollapseStructure`'s collapse
loop + the eight-rule schema-precedence cascade + the deferred ifNoExit/
caseFallthru pass — is transcribed faithfully (verified edge-for-edge against the
C++) and is exercised by 13 porter tests + 8 verifier tests covering straight-
line/diamond/proper-if/while-do/do-while/inf-loop/nested-loop/irreducible/cross-
edge CFGs and the LoopBody comparators/binary-search. The `ConditionalJoin` match
path is also fully ported. The divergences are all genuine W7/W4/W8-boundary
seams (op-flag flips, statement-count isComplex, JumpTable switch factory, op-
bank join execute, cross-arena buildCopy + print/label passes), each now a cited
losses.md entry. No blocker or major stands -> ACCEPT-WITH-LOSSES.
