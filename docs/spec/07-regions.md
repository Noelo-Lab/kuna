# 07 — Region identification (angr)

```yaml
Anchors:
  - decompiler/crates/kuna-decomp/src/p7_regions
```

This phase computes kuna's S7 artifact: the **region tree** — a nesting of
single-entry subgraphs over a function's basic blocks, with loops found first
and acyclic regions carved out by dominance. Unlike every other phase, its
lineage is angr, not Ghidra: the folder is a line-faithful port of angr's
`RegionIdentifier` (parenthetical provenance: angr
`analyses/decompiler/region_identifier.py`, `graph_region.py`,
`utils/graph.py`, `utils/doms.py`), and its result type is the read-only
subset of angr's `GraphRegion`. Everything in §7.1–§7.2 is **analysis-only**:
the identifier builds a private copy of the CFG and never mutates p-code,
`bblocks`, or any P0 state. It is also *unscheduled* — no node of the pass
tree (00-overview §0.6) runs it; it is computed on demand by its consumers
(the console/CLI surfaces of §7.4 and the chapter-08 region structurer). The
folder's one scheduled pass is the stack-guard transform of §7.3, which the
schedule places in fullloop's tail. Option defaults and flip guidance for
every option named below live in the generated catalog
([docs/options.md](../options.md)); the registry rows are in
`decompiler/crates/kuna-decomp/phases.toml` and the default divergences are
DIV-12/DIV-14 in `docs/history.md`.

## 7.1 The region graph (angr)

**Why a second graph exists.** Region identification works by *destructively
collapsing* a graph — merging chains into multi-nodes, replacing whole node
sets with a single region wrapper — and the decompiler's own `BlockGraph` must
never be mutated by an analysis. So the phase carries its own substrate:
`decompiler/crates/kuna-decomp/src/p7_regions/kuna_regiongraph.rs
(KunaRegionGraph)`, a mutable digraph in the image of `networkx.DiGraph`, over
nodes pooled in `kuna_regiongraph.rs (KunaNodePool)`. A node
(`kuna_regiongraph.rs (KunaRegionNode)`) is one of four kinds: a leaf `Block`
(a real basic block, or a synthetic address-only test node), a `Multi` (a
merged single-out→single-in chain, angr's `MultiNode`), a `Region` wrapper
carrying a collapsed region payload, or the `Dummy` end node the acyclic phase
fabricates.

**The determinism contract.** angr's algorithm leans on Python dict insertion
order; kuna replaces that with one global strict weak order — compare
`(address, creation-index)` (`kuna_regiongraph.rs (KunaNodeKey)`, the C++
port's `KunaNodeOrder`). Every container the algorithm *iterates* in an
output-affecting way is keyed on that order; membership-only sets are keyed on
the raw arena id and never drive iteration; per-node predecessor/successor
lists keep edge insertion order (the networkx adjacency analog), with a sorted
accessor (`kuna_regiongraph.rs (KunaRegionGraph::get_sorted_succs)`) where the
algorithm needs the canonical order. The consequence is documented rather than
hidden: on equal-address ties the tree can differ from angr's dict-order tree
while remaining a valid region tree. One arithmetic wrinkle is preserved
exactly: angr sorts edges by the arbitrary-precision sum of endpoint
addresses, so the port compares `(carry, low-64-bits)` lexicographically
(`kuna_regiongraph.rs (kuna_edge_sum_cmp)`) to keep the order right even when
an address sum crosses 2^64.

**The utility layer.** Four algorithms, all deterministic ports:

- *Back edges* — iterative DFS reporting every edge into a node still on the
  DFS stack-path, children visited in node order
  (`kuna_regiongraph.rs (kuna_dfs_back_edges)`).
- *Deterministic DFS postorder* — successors pushed ascending so the
  largest-addressed one is explored first, matching angr's
  `dfs_postorder_nodes_deterministic` exactly
  (`kuna_regiongraph.rs (kuna_dfs_postorder_deterministic)`).
- *Quasi-topological sort* (`kuna_regiongraph.rs (kuna_quasi_topo_sort)`) —
  Tarjan SCC condensation, topological order over the condensation, then each
  non-trivial SCC is expanded recursively after picking its loop head
  (`kuna_regiongraph.rs (kuna_append_scc)`): the head is the unique in-SCC
  successor of the latest already-ordered node, or on a multi-way entry the
  candidate with the smallest total pairwise shortest-path distance (ties
  resolved last-in-address-order, matching angr's dict-inversion semantics).
  The pathological case has a **panic mode**: an SCC of more than 3000 nodes
  that is denser than 1.4 edges per node would converge one node at a time,
  so back edges are stripped (in edge-sum order) until density falls to 1.4 —
  the order gets coarser but stays valid, and the sort terminates.
- *Slicing* — `kuna_regiongraph.rs (kuna_subgraph_between_nodes)` copies the
  graph, cuts edges into the source, keeps only paths that can still reach the
  frontier (one reverse BFS replaces angr's per-successor `has_path` probes —
  equivalent because the working copy is not mutated), and prunes dangling
  interior nodes to a fixpoint.

**Dominators, incrementally.** Immediate dominators are Cooper–Harvey–Kennedy
over the deterministic reverse postorder
(`kuna_regiongraph.rs (kuna_immediate_dominators)`). Because the acyclic phase
collapses a region after nearly every test, recomputing dominators each time
would be quadratic in practice; instead
`kuna_regiongraph.rs (KunaIncrementalDominators)` (port of angr
`utils/doms.py`) maintains the (post-)dominator tree and lazily-built
dominance frontiers under exactly one mutation shape — "a node set was
replaced by one new node" (`KunaIncrementalDominators::graph_updated`):
dominatees of replaced nodes are re-pointed at the wrapper, the wrapper
inherits the replaced head's frontier, and frontier sets are patched by
substitution. A from-scratch cross-check exists
(`KunaIncrementalDominators::verify`) and any inconsistency (a broken dom
chain, a dominatee missing from the inverted tree) is an error, not a silent
wrong answer.

## 7.2 The region identifier (angr)

**The artifact.** `decompiler/crates/kuna-decomp/src/p7_regions/kuna_regionid.rs
(KunaRegionIdentifier)` collapses the working graph into a nested
`kuna_regionid.rs (KunaGraphRegion)` tree. A region records its head, its
internal subgraph, its external successor frontier (plus the
subgraph-with-successors view), the cyclic flag, and — for loops — the *full*
graph of every edge among loop nodes including entries and exits. Two inputs
exist: `kuna_regionid.rs (KunaRegionIdentifier::build_from_block_graph)` reads
the decompiled `bblocks` read-only (one leaf node per basic block keyed on its
start address, one edge per CFG out-edge, entry = block 0's start, and a
per-block precomputation of "ends with BRANCHIND/CBRANCH" since the CFG cannot
change mid-analysis), and a synthetic address-only builder mirroring angr's
`RegionIdentifier(None, graph=g)` test mode.

**Preprocessing.** The simple-case rule: keep one weakly connected component
(the one containing the entry address, else the largest), then build the
**supergraph** (`kuna_regionid.rs (KunaRegionIdentifier::make_supergraph)`) by
repeatedly merging every single-out→single-in edge into a `Multi` chain — with
two refusals: never merge *into* the entry node, and never absorb a successor
that itself ends in BRANCHIND or CBRANCH — never form a chain whose *new* tail
would end in one (that branching is structure the later phases need to see). Then the
quasi-topological node order is computed once; collapsed wrappers inherit an
order (a loop wrapper takes its head's, an acyclic wrapper the minimum over
its members) so every later sort stays total without re-sorting the world.

**The cyclic phase** (`kuna_regionid.rs (KunaRegionIdentifier::make_regions)`,
loop half). Loop headers are the back-edge targets in node order; they are
processed in reverse, restarting the header scan after every successful
collapse. For one header, `kuna_regionid.rs
(KunaRegionIdentifier::make_cyclic_region)`:

1. *Initial body*: slice from the head to its latching nodes (with angr's
   self-loop hack — an infinite `x -> x` slices to nothing, so the body is
   restored explicitly), then the **switch-case expansion**: a body node with
   more than two non-self successors is probably a switch head, so successors
   whose predecessors all already sit in the slice are pulled in, to a
   fixpoint.
2. *Nested-header bail*: if the initial body contains another still-unstructured
   loop header, the attempt returns nothing and this header is removed from
   the live list — the inner loop must collapse first; the outer header is
   re-found on a later sweep.
3. *Loop refinement* (`kuna_regionid.rs (KunaRegionIdentifier::refine_loop)`),
   which only runs when there is more than one exit (option
   `refine_loops_with_single_successor`, angr default false, would extend it
   to single-exit loops): stage 1 absorbs exit nodes with in-degree 1 and
   out-degree ≤ 1 into the body (a straight-line tail belongs to the loop);
   stage 2 absorbs dominated exits whose predecessors are all inside — with a
   deliberate staleness quirk kept from angr (each round's veto set of
   "would-be new exits" is computed *before* any candidate is absorbed) and a
   progress guard (if every candidate is itself among the would-be new exits,
   the veto set is cleared rather than spinning); stage 3, gated by
   `largest_successor_tree_outside_loop` (angr default true), fires when
   refinement swallowed *every* exit: the largest successor tree that is
   reached from exactly one initial exit and touched by no other is pulled
   back out and its root becomes the loop's exit — but only when that maximum
   is unique and every node of the tree has a single owner; otherwise the
   loop keeps zero exits.
4. *Exit split*: with several refined exits, the **normal** exit is the first
   in DFS postorder from the head and the rest become abnormal exits. angr
   uses non-deterministic `networkx.dfs_postorder_nodes` here; kuna
   substitutes the deterministic variant — a documented strict improvement,
   not a silent divergence.
5. *Collapse*: `kuna_regionid.rs
   (KunaRegionIdentifier::abstract_cyclic_region)` replaces the body with a
   region wrapper, recording the successor frontier and the full loop graph;
   any in-edge not from a known entry, or out-edge not to a known exit, is an
   inconsistency error.

Two angr steps are deliberately **omitted** because they rewrite statements
and this port is analysis-only: `_ensure_jump_at_loop_exit_ends` (appends AIL
jumps at loop exits) and `_refine_loop_successors_to_guarded_successors` (the
`force_loop_single_exit` path, which edits conditional jumps to force one
exit). They are not lost: chapter 08's structurer re-realizes that refinement
on its own graph by virtualizing secondary exits to gotos, reading this
phase's loop identification through `kuna_regionid.rs
(KunaRegionIdentifier::cyclic_loops)` — per cyclic region, the head, body, and
exit-frontier *block start addresses* (option `regionlooprefine`, default on,
DIV-14).

**The acyclic phase** (`kuna_regionid.rs
(KunaRegionIdentifier::make_acyclic_region)`), run over each collapsed loop's
body and then the residual top-level graph. The simple-case rule: walking the
deterministic postorder, a candidate region is the span from a node to its
immediate post-dominator, accepted when the dominance-frontier predicate
`kuna_regionid.rs (KunaRegionIdentifier::check_region)` proves it single-entry
and single-exit (no frontier edge enters below the start, none leaves except
through the end); an accepted span is sliced out
(`KunaRegionIdentifier::compute_region` — single-node spans are not regions)
and collapsed to a wrapper, and both the dominator and post-dominator trees
are *patched*, not recomputed (§7.1). On rejection the search climbs the
post-dominator chain — but only while the start still dominates the candidate
end — and every failed `(start, end)` pair is memoized in a failed-attempts
set so no pair is ever re-tested across iterations. The machinery for
malformed sinks: when the graph has several end nodes (or one that is not a
predecessor of the head while the head has in-edges), a `Dummy` end node is
fabricated so a single post-dominator root exists; a lone isolated leaf is
wrapped in a trivial region because the root of the hierarchy must be a
region. The third angr option, `complete_successors` (default false), only
widens how a child region's successor view is backpatched from its parent.
None of the three angr options is a kuna `option` — per the `phases.toml`
`loop-refinement` subphase row they are LATENT, settable on the object but not
exposed; the §7.4 surfaces are their observable half.

**Guard caps and failure modes.** Every fixpoint loop of the identifier — the
switch-case expansion, refinement stages 1 and 2, the cyclic outer/inner
sweeps, and the per-subgraph acyclic collapse loop — is bounded by
`kuna_regionid.rs (kuna_guard_cap)`: `2*n² + 64` iterations, computed in
64-bit so the cap is inert for any real graph (a 32-bit `2*n*n` would overflow
at n ≥ 32768 and fire instantly). The caps exist for one reason: to turn a
mis-ported convergence guard into an *error* instead of a hang. The identifier
never panics on malformed input — empty graph, missing start node, unreachable
exit, inconsistent region edges, and cap overruns are all typed errors — and
its consumers degrade rather than die: the console commands print the error,
and the chapter-08 structurer treats a failed identification as non-fatal (it
structures the `sblocks` topology without region guidance, and the
loop-refinement data is simply absent).

## 7.3 kuna passes at the region tier: stackguard (angr)

The folder's one scheduled, output-changing pass:
`decompiler/crates/kuna-decomp/src/p7_regions/kuna_stackguard.rs
(ActionStripStackGuard)`, the port of angr's `StackCanarySimplifier`
(`option stackguard`, [docs/options.md](../options.md); registry row
P7/`edge-virtualization`, `source_decompiler = "angr"`). Scheduled in
fullloop's tail (00-overview §0.6), in the `returnsplit` group immediately
before `ActionReturnSplit`
(`decompiler/crates/kuna-decomp/src/infra/universalaction.rs
(universal_sched)`).

**What it matches.** Detection is purely structural — kuna's loader does not
reliably resolve a PLT stub to `__stack_chk_fail`, so the callee *name* is
never consulted. The canary check is pinned by its compare: a basic block with
exactly two out-edges whose tail CBRANCH tests an `INT_EQUAL`/`INT_NOTEQUAL`
where **both** operands derive from a LOAD of `<base> + 0x28` — the glibc
x86-64 TLS canary slot (`fs:0x28`): the saved canary versus a fresh reload.
The derivation walk (`kuna_stackguard.rs (derives_from_canary_load)`) peels
value-preserving ops (COPY/CAST/zext/sext/INDIRECT/zero-offset SUBPIECE) and
follows every MULTIEQUAL input, bounded by depth 32 and a visited set; the
pointer probe (`kuna_stackguard.rs (ptr_is_canary_slot)`) runs an 8-step walk
in which peels and the `INT_ADD`-of-`0x28` test share the budget, so at most 7
peeled COPY/CAST layers can precede a match. The
corrupted-canary successor is chosen from the compare sense (NOTEQUAL ⇒ the
true edge fails) corrected for the CBRANCH's boolean-flip flag, and one safety
gate remains: that successor must actually contain a CALL/CALLIND/CALLOTHER —
the no-return handler — or the match is abandoned.

**What it REMOVES.** The pass severs the corrupted-canary edge with the stock
primitive `Funcdata::remove_branch` (CBRANCH → fall-through, MULTIEQUALs
patched) and collects the now-orphaned handler block with
`remove_unreachable_blocks` — the CBRANCH and the handler block (the
`__stack_chk_fail` call) are deleted outright, while the now-dead compare and
canary reload are collected by the following dead-code pass; one canary per
apply (the
fullloop re-invokes it and it self-gates once the compare is gone). The
entry-side canary **init store** (`slot = *(fs:0x28)`) goes with the check —
angr's simplifier pops exactly that statement (`statements.pop(stmt_idx)`),
and kuna realizes the pop as a liveness release: the init writes an addrtied
stack varnode that `directwrite` marking makes address-forced, which is what
kept dead-code from ever collecting it. Before the strip the pass re-walks
the compare's own derivation chains (`kuna_stackguard.rs
(collect_canary_slots)`, the same peel set and bounds as the detector) and
records the storage of every addrtied slot version proven to derive from the
canary LOAD — the saved-canary stack slot itself; after the strip it clears
`addrforce` on every version at that storage and excises the slot range from
the local scope (`kuna_stackguard.rs (release_canary_slots)`, using
`ScopeLocal::markNotMapped` — the `checkUnaliasedReturn` idiom — so the freed
bytes cannot be absorbed into an adjacent open local's extent). Nothing is
deleted by the pass itself: the following dead-code pass collects the store,
the `fs:0x28` LOAD and the TLS-base residue through its ordinary consume
fixpoint — so an init store still feeding a live reader (a second,
not-yet-stripped check in a multi-check function, or a partial match the pass
declined) is provably never removed, and the `fs_offset` input declaration
disappears only when truly unreferenced. The
second-order win is why it sits where it does: the canary check is the single
shared return point that forces every deep `return` through a `goto`; once
stripped, the tail is a bare `return v` that the immediately-following
`ActionReturnSplit` duplicates into each predecessor, deleting the goto and
its label.

**Why tier=transform, destructive, default-on.** It deletes real instructions
the binary executes (`REMOVES CODE` in its catalog row, `destructive = true`)
— exactly the "near-always better on the right shape, wrong on the wrong one"
contract of the toggleable-transform tier (flip guidance: the catalog row).
Default **on** since DIV-14 (`docs/history.md`; live flag
`strip_stack_guard` set in
`decompiler/crates/kuna-decomp/src/infra/architecture.rs
(reset_defaults_internal)` and carried to the per-function ArchContext); the
one affected datatest file (`tests/datatests/partialsplit.xml`, whose Partial
splitting #1/#2 would change) opts out with a single `option stackguard off`
line at the top of its script, keeping the 675-assertion corpus
byte-identical with no baseline re-pin. The failure mode when detection is
wrong is asymmetric by design: a missed canary just leaves the check in the
output (the upstream rendering), while a false match would delete live code —
hence the both-operands-derive rule and the handler-call gate.

## 7.4 Observability (kuna)

Three console commands expose the region tree
(`decompiler/crates/kuna-console/src/kuna_console.rs (IfcKunaRegionTree,
IfcKunaRegionBlocks, IfcKunaRegionWalk)`): `region tree` renders the nested
hierarchy (the angr `GraphRegion.dbg_print` analog), `region blocks` the flat
per-region block-address lists (`regions_by_block_addrs`), and `region walk`
the recursive region-order visit of every leaf block. Each invocation builds a
*fresh* identifier over the currently decompiled function's `bblocks`
(`kuna_console.rs (build_region_identifier)`), so the dumps can never disagree
with the current CFG, and writes to the bulk-output stream — assertable from
datatests and capturable via `openfile write`. The dump grammar is a stable
contract, pinned by `tests/stages/kuna-regions.xml`: 2-space indent per
nesting level; `region head=0x<hex> nodes=<N>[ cyclic]` per region; `block
0x<hex>` per leaf (a merged `Multi` chain prints one `block` line per member);
one bracketed `[0x<hex>, ...]` line per flat region; `walk 0x<hex>` per
visited leaf; hex lowercase and unpadded. The rendering itself lives with the
identifier (`kuna_regionid.rs (KunaRegionIdentifier::render_tree)`; the walk
drives the `kuna_regionid.rs (KunaRegionVisitor)` callback trait, which
brackets nested regions and fires once per leaf). In `phases.toml` the three
surfaces are registered at P7/`loop-refinement` — the observable half of that
subphase's LATENT region-shaping assertions. The CLI mirror is
`kuna decompile <bin> <fn> --regions`, which captures `region blocks` +
`region tree` through a second `openfile write` so the region text never
mixes with the C.

**Handoff to chapter 08.** The region tree's consumer is the region
structurer: when `regionstructure` is on (default since DIV-12),
`decompiler/crates/kuna-decomp/src/p8_structure/blockaction.rs
(ActionBlockStructure)` routes S8 through
`decompiler/crates/kuna-decomp/src/p8_structure/region_structurer.rs
(run_region_structurer)`, which builds and computes a fresh
`KunaRegionIdentifier` over the real CFG per structuring attempt and consumes
ONE thing from this phase: when `regionlooprefine` is on, the `cyclic_loops()`
projection — the identifier's refined loop bodies and exit frontiers keyed by
block start address — as the read side of the omitted
`force_loop_single_exit` refinement. The identifier run is otherwise a
non-fatal live-CFG proof whose compute result is discarded; the
virtualize-to-goto tie-break is the structurer's own address-keyed order over
`sblocks` (`region_structurer.rs (RegionStructurer::order_virtualizable_edges)`). Nothing is cached in `Funcdata`; region
identification is recomputed by every consumer. How the structurer matches
schemas over this tree, virtualizes edges, and falls back to
`CollapseStructure` is chapter [08 — Structuring](08-structuring.md).
