# Region identification (S7) — the angr `RegionIdentifier` port

kuna ships a faithful, **analysis-only** C++ reimplementation of the angr
decompiler's region identification analysis
(`angr/analyses/decompiler/region_identifier.py` + `graph_region.py`).
It is the first standalone, inspectable artifact of stage **S7 (Region
Hierarchy)**: until now the region tree existed only implicitly inside
Ghidra's `ActionBlockStructure` collapse engine (`blockaction.cc`).

A *region* is a single-entry subgraph of control flow. The identifier
recursively finds the smallest possible regions in a function's basic-block
graph and produces a nested **region tree** whose nodes are either basic
blocks (or merged single-out→single-in chains, angr's `MultiNode`) or
sub-regions. Cyclic regions (loops) are found first via DFS back edges plus
loop refinement; acyclic regions via dominance-frontier checks while walking
up the post-dominator tree.

## Source map

| kuna file | contents | angr source |
|---|---|---|
| `decompiler/cpp/kuna_regiongraph.{hh,cc}` | `KunaRegionNode`, `KunaRegionGraph` (mutable digraph), DFS back edges, deterministic postorder, quasi-topological sort, `subgraph_between_nodes`, immediate dominators, `KunaIncrementalDominators` | `angr/utils/graph.py`, `angr/utils/doms.py` |
| `decompiler/cpp/kuna_regionid.{hh,cc}` | `KunaGraphRegion`, `KunaRegionVisitor`, `KunaRegionIdentifier` (supergraph, cyclic + acyclic phases, outputs), console commands | `angr/analyses/decompiler/region_identifier.py`, `graph_region.py` |

The identifier builds its own private graph from `Funcdata::getBasicBlocks()`
(read-only — **the decompiler's CFG and structuring are never touched**, so
default decompiler output is byte-identical and this feature is intentionally
*not* in `docs/divergences.md`). It can also be built from synthetic
addr-only nodes, mirroring angr's `RegionIdentifier(None, graph=g)` test mode
(`decompiler/unittests/testkunaregion.cc`).

## Console commands

All three compute on demand over the currently decompiled function (so they
can never disagree with the current CFG) and write to the bulk-output stream
(assertable from datatests, capturable via `openfile write`):

```
[decomp]> region tree          # nested tree (angr GraphRegion.dbg_print analog)
Region tree for test:
region head=0x3c0 nodes=3
  block 0x3c0
  region head=0x3c5 nodes=2
    block 0x3c5
    region head=0x3e0 nodes=2
      region head=0x3e0 nodes=1 cyclic
        block 0x3e0
      block 0x40a
  block 0x418

[decomp]> region blocks        # flat regions_by_block_addrs lists
Regions for test: 4
[0x3c0, 0x3c5, 0x418]
[0x3c5, 0x3e0]
[0x3e0, 0x40a]
[0x3e0]

[decomp]> region walk          # recursive region-order walk over leaf blocks
Region walk for test:
walk 0x3c0
walk 0x3c5
walk 0x3e0
walk 0x40a
walk 0x418
```

**Dump grammar (stable contract, pinned by `tests/stages/kuna-regions.xml`):**
2-space indent per nesting level; region lines are
`region head=0x<hex> nodes=<N>[ cyclic]`, leaf lines are `block 0x<hex>`
(merged chains print one `block` line per member); flat lists are one
bracketed `[0x<hex>, ...]` line per region; walk lines are `walk 0x<hex>`.
Hex is lowercase and unpadded.

`stage map region tree` routes the surface to S7 / sub-stage
`loop-refinement` (whose region-shaping *options* remain LATENT — these
commands are the observable half).

## Python / LLM access

```bash
python -m kuna.decompile ./a.out main --regions
```

prints the C, then a `// ==== kuna regions (S7) ====` separator, then the
`region blocks` + `region tree` dumps (captured through a second
`openfile write`, so the region text never mixes with the C). The library
call `kuna.decompile.decompile(..., regions=True)` returns
`(c_text, regions_text)`.

## C++ API (walking a region's blocks)

```cpp
#include "kuna_regionid.hh"

KunaRegionIdentifier ri;
ri.buildFromBlockGraph(fd->getBasicBlocks());   // never mutates the CFG
KunaGraphRegion *top = ri.compute();

ri.printTree(os);                                // nested dump
ri.getRegionsByBlockAddrs();                     // vector<vector<uintb>>

class MyVisitor : public KunaRegionVisitor {
  virtual void enterRegion(const KunaGraphRegion *r) { ... }   // brackets nesting
  virtual void exitRegion(const KunaGraphRegion *r) { ... }
  virtual void visitBlock(const FlowBlock *bl,uintb addr) { ... } // once per leaf
};
MyVisitor v;
ri.walkBlocks(v);                                // whole tree
KunaRegionIdentifier::walkRegionBlocks(loop,v);  // one region (e.g. just a loop body)
```

Per-region queries: `getHead()`, `getGraph()`, `getSuccessors()` /
`getGraphWithSuccessors()` (the region plus its exit frontier), `isCyclic()`,
`hasCyclicAncestor()`, and `getFullGraph()` (cyclic regions: all edges among
loop nodes including entries/exits).

## Intentional divergences from angr (analysis-only)

kuna never mutates p-code or statements, so the two statement-rewriting
pieces of angr's algorithm are omitted (each marked with a `(kuna)` comment
at its call site):

- `_ensure_jump_at_loop_exit_ends` — appends AIL `Jump` statements at loop
  exits for the later structurer. No-op here.
- `_refine_loop_successors_to_guarded_successors` — rewrites a multi-exit
  loop into a guarded `ConditionNode` tree by editing conditional jumps
  through the condition processor. Omitted, so angr's
  `force_loop_single_exit` option (default True there) is effectively
  **false** in kuna: a multi-successor cyclic region keeps all its
  successors.

Other angr options keep their defaults and are settable on the C++ object:
`largest_successor_tree_outside_loop` (true),
`refine_loops_with_single_successor` (false), `complete_successors` (false).
Also: determinism comes from a global `(address, creation-index)` node order
instead of Python dict insertion order (equal-address ties may produce a
different but still-valid tree); `(addr, idx)` block keys degrade to plain
addresses; angr's `fake_return`/`call` supergraph arms have no p-code
counterpart (calls do not terminate a `BlockBasic`); `test_reducibility` and
the structurer-phase `GraphRegion` editing API are not ported.

## Tests

- `decompiler/unittests/testkunaregion.cc` — `regionid_diamond0` /
  `regionid_diamond1` are direct ports of angr's `test_region_identifier_0/1`
  (`tests/analyses/decompiler/test_structurer.py`), asserting the final
  top-level region graph has exactly two nodes; `regionid_loop` adds cyclic
  coverage (run: `make test`, or
  `decomp_test_dbg unittests regionid_diamond0 regionid_diamond1 regionid_loop`).
- `tests/stages/kuna-regions.xml` — end-to-end on a real loop+diamond
  function (the gh8724 bytes): cyclic region found, tree rooted at entry,
  walker covers each block exactly once, `stage map` routing
  (run: `make test-stages`).
