---
item: w7-s7-kuna-regionid
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8-1m (independent verifier)
date: 2026-06-13
round: 1
gate: `cd rust && cargo test -p kuna-decomp` -> regionid lib tests 3/3 pass; new verifier suite 6/6 pass (`tests/verify_w7_s7_kuna_regionid.rs`); the regiongraph dependency suite 6/6 pass. Whole-crate run has ONE failure, `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`, a missing `.sla` build artifact (gitignored, absent in this worktree), pre-existing on the `rust-port` base and unrelated to this item's diff. The diff touches only `kuna_regionid.rs` (+2184), `kuna_regiongraph.rs` (+1971, the dependency item with its own verdict), one `lib.rs` line (`pub mod kuna_regionid;`), and the two verifier test files.
---

## Scope

C++ in scope: `decompiler/cpp/kuna_regionid.{cc,hh}` — the kuna-owned region
identification analysis (S7), a faithful ANALYSIS-ONLY port of angr's
`RegionIdentifier` + the read-only subset of `GraphRegion`. The graph substrate
(`kuna_regiongraph.{cc,hh}`) is the **separate** prior item
`w7-s7-kuna-regiongraph` (its own verdict file); it is treated here as reviewed
foundation and only spot-checked for the contracts regionid relies on
(`NodeMap` = `map<ptr,Adjacency,KunaNodeOrder>` -> `BTreeMap<KunaNodeKey,_>`;
insertion-ordered `preds`/`succs` vectors; `idom`/`dominates`/`df`/
`graph_updated` semantics).

Rust: `rust/crates/kuna-decomp/src/kuna_regionid.rs`.

## Re-derivation summary

The port transcribes the C++ structure-for-structure. `KunaRegionNode *` ->
`KunaNodeId` arena id (ADR 0001); `KunaGraphRegion *` -> `RegionPayloadId` index
into `region_pool`; every C++-iterated container keyed on `KunaNodeOrder` ->
`BTreeSet/BTreeMap<KunaNodeKey>` (ADR 0002); "membership only" pointer sets ->
`BTreeSet<KunaNodeId>` keyed by raw id (never iterated into output). All five
phases re-derived against the C++: `pickConnectedComponent`, `makeSupergraph`,
the cyclic loop (`findLoopHeaders` / `findInitialLoopNodes` / `refineLoop` /
`makeCyclicRegion` / `abstractCyclicRegion`), the acyclic loop (`checkRegion` /
`computeRegion` / `makeAcyclicRegion` / `abstractAcyclicRegion` /
`replaceRegionInGraph` / `backpatch_secondary`), and the outputs
(`buildRegionsByBlockAddrs` / `walkBlocks`).

## Hunt list (all explicitly checked)

- **Signedness.** CLEAN. `chosen` widened to `i64` with the same `< 0` sentinel
  as C++ `int4 chosen=-1` (component indices are small; no truncation). Length
  comparisons that the C++ does as `(int4)size > 2` / `== maxSize` are
  transcribed as `len() as int4` with the identical truncation semantics (rs
  705, 949-960) — the port chose the C++ result, not a "fixed" `usize` compare.
  Loop-index vs container-size compares all use `usize`/range iterators.
- **Integer widths.** CLEAN. `uintb->u64`, `int4->i32`, `int8->i64` (ADR 0003).
  `kuna_guard_cap` computes `2*n*n+64` in `int8` exactly as the C++ comment
  pins (rs 56-59) — the 32-bit-overflow trap the C++ adversarial review noted is
  preserved in 64-bit. Dummy node addr `!0u64` == C++ `~((uintb)0)` (rs 1723).
- **Wrapping.** CLEAN / N/A in regionid. The only address arithmetic
  (`edge_sum`) lives in the regiongraph dependency (`wadd`, reviewed there).
  regionid does no arithmetic on `uintb` values beyond equality/order.
- **Comparator totality.** CLEAN. `sort_by_node_order` pre-checks every node has
  an order entry (surfacing the C++ comparator's throw as an `Err`) then sorts
  by the total `(int4,int4)` order with a latched error (rs 593-616). The
  multi-exit `keyed.sort_by(|a,b| a.0.cmp(&b.0))` (rs 1067) drops the C++
  `pair<int4,ptr>` pointer tiebreaker; re-derived as genuinely unobservable —
  each exit maps to a UNIQUE `dfs_postorder` index, so ties never occur (a
  determinism improvement: the C++ tiebreaker was raw pointer order). Verified
  by the `cyclic_region_two_exits_postorder_split` test.
- **Iteration-order provenance, per loop.** CLEAN. Every C++ `beginNode()`/
  `getNodes`/`node_keys` loop -> `BTreeMap`/`BTreeSet` `KunaNodeOrder`
  iteration. `getSuccs`/`getPreds`/`inEdges`/`outEdges`/`allEdges` ->
  insertion-ordered `Vec` (preserved). `backpatch_secondary` snapshots
  `graph_with_successors` nodes in `KunaNodeOrder` and reads the **final**
  `successors` set for the successor-pair double loop, matching C++ taking
  `begin()` after the first loop mutates the set (rs 1896-1909 vs cc 1289-1294).
  No `HashMap`/`HashSet` anywhere in the ported logic.
- **Off-by-one / do-while / reverse iterators.** CLEAN. The dangerous C++
  `for(int4 i=snap.size()-1;i>=0;--i)` (cc 910; underflows to -1 when empty,
  guarded by a prior `empty()` break) -> empty-safe `for i in (0..snap.len())
  .rev()` (rs 1262). `widx` BFS-level loop `0..=new_regions.len()` matches C++
  `widx<=newRegions.size()` (the `==size` iteration is the top-level region, rs
  1299). The postdom-climb `while let Some(pdn)` + the two `break` checks +
  `idom(pdn)==Some(pdn)` self-check transcribe cc 1319-1321 exactly.
- **Erase-while-iterating.** CLEAN. `refineLoop` stage 1 snapshots `exit_nodes`
  before mutating (`snap = exit_nodes.iter().copied().collect()`, rs 766) and
  removes via `exit_nodes.remove(&ek)` — same elements as the C++ `vector` snap
  + `set::erase` (cc 513-525). Stage 2's `sorted_exits` index `remove(pos)` and
  the `loop_headers` failed-removal `position()+remove` match the C++
  `vector::erase` sites (rs 853-855, 1275-1279). `make_regions` removes failed
  headers from the LIVE list (not the snapshot), matching cc 918-920.
- **Exception -> Result parity.** Mostly CLEAN; ONE non-blocking parity note
  (F1 below). All C++ `throw LowlevelError` sites map to `Err(...)?`
  propagation: the merge non-block, inconsistent cyclic in/out-edge asserts,
  the start-node / node-order / unreachable-exit-node lookups, and the four
  guard-cap non-convergence traps. The `compute()` `computed` latch and
  empty-graph error are preserved (verified by `compute_idempotent_and_empty
  _errors`).

## Structuring-schema-precedence wave items

- **Cyclic-vs-acyclic precedence.** `make_regions` runs the cyclic fixpoint to
  exhaustion FIRST (outer+inner loops), then the acyclic phase per cyclic
  region body + the residual graph, exactly as cc 897-958. A loop whose body
  still contains an unstructured header returns `None`, is dropped from the live
  header list, and is retried after the inner one is structured (cc 713-719,
  915-921) — verified end-to-end by `nested_loops_inner_structured_first`.
- **HighVariable-merge / Cover-intersection analog (successor-frontier
  precedence).** `abstract_cyclic_region` and `backpatch_secondary` build
  `graph_with_successors` in the same edge order as the C++: subgraph edges,
  then region out-edges, then the successor frontier, then successor-successor
  edges; the `complete_successors` vs `!gc.contains_node(succ)` branch (rs
  1873-1892) matches cc 1274-1285. `node_order[wrapper]` = head's order (cyclic)
  / min over members (acyclic) match cc 880 / 1123-1132.
- **Stack-layout-order analog (node_order determinism).** The single source of
  ordering is `compute_node_order` (quasi-topo) -> `(i,0)` pairs, with wrappers
  inheriting head/min order. No ordering is left to allocation/pointer identity.

## Adversarial tests (`tests/verify_w7_s7_kuna_regionid.rs`, 6/6 pass)

1. `cyclic_region_two_exits_postorder_split` — loop {2,3} with two
   non-absorbable exits (in-degree 2 blocks `refineLoop` stage 1), forcing the
   `refinedExitNodes.size()>1` deterministic postorder normal/abnormal split.
2. `supergraph_chain_collapses_but_walk_expands_all` — a 1->2->3->4->5 chain
   collapses to one `k_multi`, yet the walker expands all five in execution
   order.
3. `nested_loops_inner_structured_first` — inner header structured before the
   outer (the `None`-return retry path); both cyclic regions formed.
4. `self_loop_head_slice_hack` — the `sliceGraph` infinite-self-loop HACK
   (cc 416-417) yields a cyclic region with body exactly {2}.
5. `connected_component_prefers_entry_over_largest` — the entry component wins
   over the strictly larger one (cc 212-219 precedence).
6. `compute_idempotent_and_empty_errors` — `computed` latch + empty-graph error
   parity.

## Findings

- **F1 (parity note, NON-blocking, not a LOSS).** `region_in_edges` /
  `region_out_edges` (rs 1518, 1542) GUARD with `if g.contains_node(head/member)`
  and silently produce no edges when absent, whereas the C++ `regionInEdges`/
  `regionOutEdges` call `getPreds`/`getSuccs`, which **throw** `LowlevelError`
  if the node is not in the graph (regiongraph cc 113-127). Re-derived as
  output-equivalent on every reachable path: at all three call sites
  (`abstractAcyclicRegion`, the secondary recursion, `replaceRegionInGraph`) the
  region head/members are provably still present in the working graph when these
  run, so neither the C++ throws nor the Rust guard fires. The divergence is
  only on an unreachable "impossible" error path (defensive skip vs hard error),
  cannot change any valid region tree, and is therefore recorded as a note, not
  a blocking finding nor a LOSS. Anchor: rs 1508-1554 vs
  decompiler/cpp/kuna_regionid.cc:1072-1093.

## Losses (MAIN-tree losses.md: LOSS-105, LOSS-106)

- **LOSS-105** — `buildFromBlockGraph` (Input A, the read-only decompiler
  `BlockGraph` adapter) and the real-block arm of `endsWithBranchindOrCbranch`
  (`FlowBlock::lastOp()->code()` probe for `CPUI_BRANCHIND`/`CPUI_CBRANCH`) are
  seamed `SEAM(W7)`. The full identifier algorithm IS ported over the synthetic
  input API (`add_synthetic_block`/`add_synthetic_edge`), which is exactly what
  the `testkunaregion.cc` unit tests drive; only the wiring to real
  `FlowBlock`s (a later wave's accessor) is deferred. On the synthetic path the
  C++ also returns `false` (no block), so the seam is behavior-identical to the
  only path this port exercises.
- **LOSS-106** — the three console commands (`IfcKunaRegionTree`,
  `IfcKunaRegionBlocks`, `IfcKunaRegionWalk`) and `KunaGraphRegion::printTree` /
  `KunaRegionIdentifier::printTree` are not ported (the `IfaceDecompCommand`
  console machinery + ostream dumping are a later wave). The underlying data
  they print (`get_regions_by_block_addrs`, the `walk_blocks` visitor surface)
  is fully ported and tested.

## Verdict

ACCEPT-WITH-LOSSES. The algorithm is a faithful, deterministic transcription of
the C++ across all five phases; the hunt list is clean (one non-blocking
error-path parity note, F1); the two losses are the documented `SEAM(W7)`
block-graph/console surface, behavior-identical on every path the port
realizes. No blocker or major stands.
