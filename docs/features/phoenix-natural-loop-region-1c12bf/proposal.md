# [PROPOSAL] phoenix-natural-loop-region: recover head-in-body natural loops (angr Phoenix)

**Status:** proposal / draft — no engine code changed. Human go/no-go requested on
which implementation path to fund.

## The problem

angr's decompiler structures `sub_442300`
(`test_decompiling_phoenix_natural_loop_region_head_in_body`, 0x442300, a Go Windows
PE) as a single clean `while (true) { … continue; … }` with exactly one `goto`.
kuna renders the *same* function with **31** `goto`/`label_` tokens: the natural-loop
head at `0x4425a0` becomes `label_4425a0:` with `goto label_4425a0;` back-edges, and
the loop body is scattered across ~10 more labels. See
`docs/features/phoenix-natural-loop-region-1c12bf/analysis.md`,
`angr-vs-kuna.txt`, `kuna-full.txt`.

(kuna currently loads this binary only under `--experimental-formats`; the default
loader rejects the PE. A stage testcase would therefore need a synthesised bytechunk
of just the loop CFG, not the whole Go function.)

## angr reference

- Pass/class: `RegionIdentifier` (Phoenix structuring),
  `angr/analyses/decompiler/region_identifier.py` — natural-loop region construction
  where the loop **head** node is also reachable inside the loop **body**.
- angr 9.2.213.

## Why it is large (not a single Action/Rule)

kuna's printed C comes from the upstream Ghidra **collapse-based** structurer
(`s8_structure/blockaction.rs`: `CollapseStructure` / `LoopBody` / `TraceDAG`), not
from a region identifier. Unlike `kuna_loweredswitch.rs` — which manufactures an S2
`BRANCHIND`+`JumpTable` artifact upstream and lets the existing structurer+printer
emit the `switch` — there is **no single artifact** to inject here; the loop-recovery
logic lives *inside* the structurer. The decider subagent returned `scope: "large"`.

## Two candidate implementation paths (pick one to fund)

### Path A — extend the collapse structurer's loop detection (in-place)
Teach `blockaction.rs` `LoopBody`/`CollapseStructure` to recognise the head-in-body
natural loop and prefer a loop-header collapse over emitting a back-edge goto,
gated by a new `option naturalloophead on|off` (default off).

- **Pros:** stays in the engine kuna already uses to print C; no emit-path rewrite;
  the option can be ablated against the 675-datatest corpus like every other feature.
- **Cons:** touches S7/S8 structuring **core** (loop body/loop-edge selection). High
  regression risk across the datatest corpus; requires careful guards so it only fires
  on the head-in-body shape. Multi-step: (1) detect the shape on the collapsed graph,
  (2) reorder/relabel the loop-edge so the header collapses, (3) verify goto/label
  count drops with no datatest churn.

### Path B — wire the ported `RegionIdentifier` into emit
Promote `s7_regions/kuna_regionid.rs` (today analysis-only) into a real structuring
pass that produces the region tree the printer consumes, gated by the new option.

- **Pros:** reuses the already-ported, already-faithful angr algorithm — the
  head-in-body case is *already handled* in that port; closest to "do what angr does."
- **Cons:** a **new pass type** + new emit infrastructure (region-tree → C printer
  adapter, `getStart()/lastOp()` block-graph adapter that is currently SEAM'd off).
  The largest of the two; effectively a second structurer.

## Speed / risk assessment

- **Speed:** the target function is large; region identification is `~O(n²)` (the port
  carries `2n²+64` guard caps). Either path must measure decompile wall-time off vs on
  (Hard rule 6) and stay within the +5% budget, else ship default-OFF opt-in.
- **Risk:** Path A risks datatest-corpus churn (it edits the structurer everyone uses);
  Path B risks correctness drift between two structurers but isolates blast radius
  behind the option (default-off, only the gated path uses the region tree).
- **Recommendation:** Path A as a *narrow, heavily-guarded* gated structurer tweak is
  the smaller bet if the head-in-body shape can be detected cheaply; Path B is the
  faithful-to-angr bet if a second structurer is acceptable. Human decision requested.

## Proposed option

`option naturalloophead on|off` (default off), `change_kind = structure-recovery`,
`source_decompiler = angr`,
`inspiration = "test_decompiling_phoenix_natural_loop_region_head_in_body; RegionIdentifier (Phoenix); 0x442300"`.

## What this PR contains

Analysis + reproduction artifacts + this proposal only. No `kuna_*.rs`, no option
registration, no stage test, no baseline change.
