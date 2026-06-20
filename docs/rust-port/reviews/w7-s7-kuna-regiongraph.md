# w7-s7-kuna-regiongraph
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8-1m (independent verifier)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> regiongraph lib tests 18/18 pass; new verifier suite 6/6 pass; whole-crate run has ONE failure `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` which is a missing `.sla` build artifact (gitignored, absent in the worktree), pre-existing on the `rust-port` base (test last touched commit b138730, before this branch) and unrelated to this item's diff. The diff touches only `kuna_regiongraph.rs` (+1971) and one `lib.rs` line (`pub mod kuna_regiongraph;`).

## Scope
C++ in scope: `decompiler/cpp/kuna_regiongraph.{cc,hh}` — the kuna-owned graph
substrate for the angr `RegionIdentifier` (S7): `KunaRegionNode`, `KunaNodeOrder`,
`KunaRegionGraph`, `kunaDfsBackEdges`, `kunaDfsPostorderDeterministic`,
`kunaQuasiTopoSort` (+`_append_scc`, panic mode), `kunaSubgraphBetweenNodes`,
`kunaImmediateDominators`, `kunaDominates`, `KunaIncrementalDominators`.

## Hunt list
- **Signedness.** CLEAN. `KunaNodeOrder` compares `uintb addr` then `uint4 ident`;
  the Rust `KunaNodeKey` derives `Ord` over `(u64 addr, u32 ident, KunaNodeId id)` —
  unsigned throughout. Adversarial test `sorted_succs_is_unsigned_near_u64_max`
  exercises `u64::MAX` / high-bit-set addresses and confirms unsigned ordering in
  both `get_sorted_succs` and `kuna_dfs_postorder_deterministic`. Loop indices that
  were `int4` in C++ are kept `int4` (e.g. Tarjan `counter`, `tarjan_low`) where the
  algorithm relies on a signed sentinel `-1`; container-size compares use `usize`/
  `len()` faithfully. No `-Wno-sign-compare` ambiguity survives.
- **Integer widths.** CLEAN. `uintb->u64`, `uint4->u32`, `int4->i32` per ADR 0003.
  No C++ `int4*int4`-into-`intb` promotions exist in this file. The one width-sensitive
  spot — the `_sort_edge` address sum — is computed in `u64` with explicit carry
  detection (see Wrapping).
- **Wrapping.** CLEAN. The sole wrap site is `KunaEdgeSum`/`kuna_edge_sum_cmp`
  (`suma = a0 + a1; carrya = (suma < a0)`), the deliberate 2^64-carry probe; the Rust
  uses `a0.wadd(a1)` (`wrapping_add` via the `Wrap` trait) and reproduces the C++
  `(carry, low64)` lexicographic compare exactly (cc:317-327 vs rs:638-657), including
  the `if (carrya != carryb) return carryb` -> `if carryb { Less } else { Greater }`
  mapping. All other arithmetic is index/degree counting that cannot wrap on realistic
  graphs.
- **Comparator totality.** CLEAN. `KunaNodeKey`'s derived `Ord` is total (lexicographic
  on `(addr, ident, id)`, all integers); `ident` is a unique creation index so the `id`
  field never actually tiebreaks (matches the C++ `set<...,KunaNodeOrder>` where equal
  `(addr,ident)` is indistinguishable). `kuna_edge_sum_cmp` is a strict weak order
  (lexicographic on `(carry, sum)`). No float comparisons.
- **Iteration-order provenance.** CLEAN except the single LOSS below. Every C++
  `NodeMap` iteration (`KunaNodeOrder`) -> Rust `BTreeMap<KunaNodeKey,_>` (same order):
  `getNodes`, `allEdges`, `buildReversed`, `induced`, the prune loop in
  `subgraph_between_nodes`. `getSuccs`/`getPreds` insertion-order vectors -> `Vec`
  (preserved). "Membership-only" pointer sets -> `BTreeSet<KunaNodeId>` keyed by raw id
  (never iterated into output). The ONE divergence: C++ `doms`/`dfs`/`invertedDomTree`
  are `map<ptr,...>` with the DEFAULT comparator = raw pointer order (NOT `KunaNodeOrder`
  — only `NodeMap` is `KunaNodeOrder`); the Rust port iterates these in `KunaNodeOrder`.
  Proven output-equivalent (set-semantic consumers) — see F1 / LOSS-097.
- **Off-by-one / do-while / reverse iterators.** CLEAN. C++ reverse `for(i=size-1;i>=0;--i)`
  loops (reverse-postorder dom pass, `_append_scc` backward scan, postorder expansion)
  -> `for i in (0..len).rev()` (same elements, same direction). The Tarjan/condensation
  insertion sorts transcribe the C++ `j>=0 && less(...)` bounds exactly (rs:787-806,
  928-936). No `do/while`. `kuna_shortest_path_length` BFS `pos`/`queue` index walk
  matches C++ exactly.
- **Erase-while-iterating.** CLEAN. `remove_node` clones the pred/succ lists before
  mutating neighbors (rs:298,309), matching the C++ which reads `(*iter).second.preds`
  while erasing from *other* nodes' lists. The `subgraph_between_nodes` dangling-prune
  collects `to_remove` then removes (rs:1162-1182) — same fixpoint as the C++
  `for(;;){ collect; if empty break; remove }`. `graph_updated`'s frontier patch
  (`dfs.values_mut()` remove+insert, rs:1517-1521) is set-semantic, order-independent,
  and mirrors cc:921-928. The `inverted_dom_tree` list erase-by-position (rs:1534) with
  the `found` invariant check matches cc:943-947.
- **Exception -> Result parity.** CLEAN. Every C++ `throw LowlevelError` -> a `?`-propagated
  `Err(KunaError::lowlevel(...))` with the same message and the same call site
  (sizeIn/Out/getPreds/getSuccs not-in-graph; subgraph source/frontier not-in-graph;
  "no head"; "not mutually reachable"; "broken dom chain"; "inverted dom tree
  inconsistent"; the four `verify` divergence messages). These are all invariant
  violations on malformed input; no caller in scope relies on partial-mutation state
  past a throw (the throwing functions build fresh `res`/`subgraph` locals).
- **This wave's extra (HighVariable merge / Cover-intersection / varmap / block-structuring
  schema).** N/A — this item is the pure graph substrate; none of those subsystems are
  touched here. The relevant wave-specific concern (dominator-tree / region collapse
  ordering) is covered by the iteration-order and `graph_updated` analysis above.

## Mechanical pass
- grep: zero `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`. `unwrap()`
  occurrences are all on structurally-guaranteed `stack.last()` / `get_mut(existing key)`
  / `next()` on a known-nonempty container — re-derived as infallible. `sort_by` (stable)
  used, not `sort_unstable`.
- bare `as` casts: ~80, all `usize`<->`int4`/`u32` index & length conversions inherent
  to the arena-id model (C++ uses native `int4` indices / pointers). None carry a `// cast:`
  comment (minor style gap vs the protocol), but each is a mechanical index/len bridge with
  no truncation risk on realistic graph sizes — not a correctness finding. Noted as MINOR.
- `cargo clippy -p kuna-decomp -- -D warnings`: CLEAN (no warnings).
- item gate: regiongraph lib 18/18 + verifier 6/6 green; the only red is the
  environmental `.sla` `corpus_bootstrap` failure documented in the gate line.

## findings
- F1 (LOSS, not blocking): incremental-dominator side maps (`doms`/`dfs`/
  `invertedDomTree`) iterate in `KunaNodeOrder`, where C++ iterates in raw pointer
  order (default `std::less<ptr>`). Output-equivalent (every consumer is set-semantic
  on node identity); a deliberate determinism improvement over the C++. Cited as
  LOSS-097.
       cpp: decompiler/cpp/kuna_regiongraph.cc:843-875
       rust: rust/crates/kuna-decomp/src/kuna_regiongraph.rs:1363-1432
- F2 (MINOR, advisory): the numerous bare `as` index/length casts lack the protocol's
  `// cast:` justification comments. No truncation/correctness risk; recommend a future
  cleanup pass adds the markers.
       cpp: decompiler/cpp/kuna_regiongraph.cc (int4 indices throughout)
       rust: rust/crates/kuna-decomp/src/kuna_regiongraph.rs:199,370,703,821,... (passim)

## adversarial tests
Added `rust/crates/kuna-decomp/tests/verify_w7_s7_kuna_regiongraph.rs` (6 tests, all pass):
- `sorted_succs_is_unsigned_near_u64_max` — `KunaNodeOrder` is unsigned at `u64::MAX` /
  high-bit addresses (signedness; sorted-succs + postorder largest-first).
- `append_scc_loop_head_distance_tie_last_wins` — `_append_scc` loop-head tie-break picks
  the LAST successor in `KunaNodeOrder` (angr `{v:k}` overwrite-on-collision); K3-cycle
  oracle re-derived, head == highest-addr member.
- `quasi_topo_real_self_loop` — a real `src==dst` self-loop edge collapses to a
  present-but-edgeless id and the node appears exactly once in topo order.
- `graph_updated_multi_dominatee_matches_scratch` — incremental dominator patch with
  multiple dominatees re-pointed at once == independent from-scratch `immediate_dominators`
  + `verify()`; exercises the LOSS-097 pointer-vs-KunaNodeOrder divergence and the dfs-patch
  path (touches `df` first).
- `subgraph_between_nodes_prunes_unreachable_cycle` — a self-sustaining dead 2-cycle that
  cannot reach the frontier is gated out by the reverse-BFS `canReach` (the dangling-prune
  alone would never delete it), incl. `includeFrontier=false`.
- `post_dominators_diamond_merge` — post-dominators (reversed-edge + sink root) of a diamond:
  the merge/exit post-dominates the entry; `verify()` on the reversed orientation.

## losses
LOSS-097 (the `KunaNodeOrder`-vs-pointer-order iteration divergence in the incremental
dominator side maps — output-equivalent, determinism improvement).
