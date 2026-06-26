# Region-based (Phoenix/SAILR) structurer for kuna — staged roadmap

**Status:** PROPOSAL (draft PR) — awaiting go/no-go before implementation.

This is the foundation behind the "structuring / goto / switch / loops" gains. Multiple
feature attempts (e.g. the head-in-body natural-loop PR #44) returned NEGATIVE with the same
verdict: *"needs region/SAILR structurer."* kuna's structuring is Ghidra's goto-emitting
`CollapseStructure`; angr's SAILR/Phoenix structurer produces far fewer gotos and recovers
loops/switches. This roadmap gives kuna that structurer, incrementally and parity-safely.

## Feasibility & seam

**Feasible, with an unusually clean seam**, because kuna already has both hard halves:
1. The **region tree** is already ported (`s7_regions/kuna_regionid.rs`, the angr
   `RegionIdentifier` port) — analysis-only today, not wired into output.
2. The **renderer's input vocabulary** (`substrate/block.rs` `BlockKind::{List,Condition,If,
   WhileDo,DoWhile,Switch,InfLoop,Goto,Copy}`) maps almost 1:1 onto angr's structurer nodes.

**Seam:** under a new master `--option regionstructure` (default **OFF**), replace the
`CollapseStructure::collapse_all()` call in `ActionBlockStructure::apply`
(`s8_structure/blockaction.rs:~3372`) with a region structurer that walks the region tree,
matches Phoenix schemas, and emits the same `sblocks` `BlockKind` tree the renderer
(`s9_emit/printc.rs`) already consumes. The p-code/varnode data flow is never touched.
Default-OFF ⇒ all 675 datatests render byte-identical. New module:
`s8_structure/region_structurer.rs`, porting angr
`analyses/decompiler/structuring/{phoenix,sailr,recursive_structurer,structurer_base}.py`.

## Increments (each: option-gated, default-OFF, both-directions stage test, 3 gates green)

| # | What | Ports (angr) | Risk |
|---|---|---|---|
| **0** | Close the W7 adapter: `build_from_block_graph` + `ends_with_branchind_or_cbranch` over real `bblocks` ops, so `ri.compute()` runs on real CFGs. No output change. | finishes the existing RI port | low |
| **1** | **PoC/skeleton (LARGE):** region post-order walk + acyclic **sequence** schema + `_virtualize_edge` goto fallback + SAILR H1/H3 ordering; wire into `ActionBlockStructure::apply` + register the option. Proves the whole seam end-to-end. | `recursive_structurer`, `phoenix._match_acyclic_sequence`, `_virtualize_edge`, `sailr._order_virtualizable_edges` | medium |
| **2** | Acyclic **if/else (ITE)** + structural edge-condition probe (from CBRANCH true/false + `boolean_flip`; no claripy). | `phoenix._match_acyclic_ite`, structural `have_opposite_edge_conditions` | med-high |
| **3** | **Loop recovery (payoff):** cyclic **while/do-while/natural-loop** → While/DoWhile/InfLoop + break/continue as kuna goto-flags. The exact #44 shape. | `phoenix._analyze_cyclic` + 4 cyclic matchers, break/continue rewriters | high |
| **4** | **Switch-case** recovery → BlockSwitch, fed by `switch_case_edges`/`find_jump_table_index` already in blockaction.rs. Independent of 2/3 after Inc 1. | `phoenix._match_acyclic_switch_cases*` | med-high |
| **5** | **Short-circuit** folding (`if(a)if(b)`→`if(a&&b)`) + SAILR **H2** post-dom heuristic (with angr's size caps) + recursive re-structure. Closes the quality gap. | `phoenix._match_acyclic_short_circuit_conditions*`, `sailr` H2 | medium |
| **6** | Flip default-ON where strictly better → **DIV-6**, re-pin datatests, regenerate baseline. The only intentional default change. | — | careful |

## Parity & performance
- **Parity:** default-OFF through Inc 0–5 ⇒ mechanically byte-identical (`make test` PARITY OK).
  Honest-partial-safe: an un-matched region falls back to a virtualized goto / the existing
  `CollapseStructure` path — never aborts.
- **Perf (critical):** cap the H2 post-dominator heuristic (port angr's `postdom_max_edges=10`/
  `postdom_max_graph_size=50`); reuse `KunaIncrementalDominators` rather than rebuilding per
  schema; watch `failed_region_attempts` growth. Zero perf cost on the default pipeline until
  Inc 6 (option OFF). Benchmark the largest datatest functions (cf. the prior `ActionPool`
  O(N²) regression).

## Effort & sequencing
~6–7 PRs. Sequential spine **Inc 0 → 1 → 2 → 3**; **Inc 4 and Inc 5 parallelizable** after
Inc 1; **Inc 6** last. First dispatch: **Inc 0** (small), then **Inc 1** PoC — target a function
shaped `if(c){A}else{B} → join → tail` where Ghidra gotos the join→tail edge; the sequence
schema folds it, yielding one fewer `goto` verifiable via
`kuna decompile ./elf fn --option regionstructure on|off`.

## Critical files
`s7_regions/kuna_regionid.rs`, `s8_structure/blockaction.rs`, `substrate/block.rs`,
`s9_emit/printc.rs`, `p0_knowledge/options.rs` (+ `stages.toml`, `infra/universalaction.rs`),
and the new `s8_structure/region_structurer.rs`.

## Inc 1 — implementation status & findings (option `regionstructure`, default OFF)

**Landed (parity-safe, 3 gates green):** the seam works end to end. Under `option
regionstructure on`, `ActionBlockStructure::apply` runs
`KunaRegionIdentifier::build_from_block_graph(fd).compute()` over the real CFG (the W7
adapter, proven on live functions) and then `region_structurer::run_region_structurer`
INSTEAD of `CollapseStructure::collapse_all`. The structurer (`s8_structure/region_structurer.rs`)
implements: the post-order driver, the **acyclic sequence** schema
(`_match_acyclic_sequence`, collapsing single-pred/single-succ chains into `BlockList`s), a
**structural ITE** schema (`_match_acyclic_ite` realized over kuna's `BlockIf` builders — no
claripy, since the edge condition is the block's terminal CBRANCH), and the
**virtualize-to-goto fallback** with the SAILR **H1** (sibling-count) / **H3** (return-edge)
edge ordering (`_virtualize_edge` + `_order_virtualizable_edges`). It is **honest-partial-safe**:
an un-structurable region (false-clause if-then needing `negateCondition`, a crossing, a loop,
a switch) falls back to `CollapseStructure` — never a panic. Default-OFF ⇒ 675/675 datatests
byte-identical; decompile speed ON vs OFF is identical (no measurable overhead).

**Empirical finding (the "one fewer goto" target is NOT acyclic):** the roadmap's Inc 1
target — *"a function shaped `if(c){A}else{B} → join → tail` where Ghidra gotos the join→tail
edge"* — does **not** reproduce. Across 25+ hand-crafted acyclic shapes (gcc -O0/-O1/-O2 and
clang: clean and multi-block if/else, if/else-if cascades, shared multi-predecessor tails,
guard cascades with side effects, nested diamonds, crossings, ladders) **Ghidra's
`CollapseStructure` already emits ZERO gotos** — it has sequence, if-then, if-else,
short-circuit `&&`/`||` folding, and a good `select_goto`. The only acyclic gotos Ghidra
emits are on true **crossing edges** (a non-series-parallel DAG, e.g. `A→{B,C}, B→{C,D},
C→D`), and on those its single goto is already minimal — the region structurer **matches** it
(folding the crossing requires the ITE/condition recovery of Inc 2, not the sequence schema).
A flat sequence-only structurer therefore has **no acyclic case where it beats Ghidra's goto
count**; it can only match it (which it does) or fall back. The goto-reducing wins all live in
the **cyclic** increments — Ghidra emits the multi-exit loop-cleanup gotos (the PR #44 /
`1after909` family, 9 `goto cleanup;`) that SAILR recovers as `break;`/`continue;` (Inc 3),
plus switches (Inc 4) and short-circuit/post-dom refinement (Inc 5).

**Consequence for the roadmap:** Inc 1's proof testcase (`tests/stages/regionstructure-seq.xml`)
asserts the structurer **actively + correctly structures** the `if(c){A}else{B}→join→tail`
shape (the success path, not the fall-back) and is **byte-identical to Ghidra** (the honest
parity result), rather than a goto delta. The "one fewer goto" verifiable delta should be the
success metric for **Inc 3** (loops), not Inc 1 — Inc 1 delivers the live, parity-safe
sequence/ITE/virtualize foundation the cyclic schemas build on.

## Inc 3 — implementation status & findings (cyclic/loop recovery, option `regionstructure`)

**Landed (parity-safe, 3 gates green):** the region structurer now recovers the three loop
classes — **do-while** (`BlockDoWhile`), **while/for** (`BlockWhileDo`), and **infinite-loop +
CBRANCH break** (`BlockInfLoop`) — porting angr `phoenix._analyze_cyclic` +
`_match_cyclic_{while,dowhile,natural_loop}` structurally over kuna's loop builders
(`new_block_{do_while,while_do,inf_loop}`). The schemas lean on the **back-edges + dominators
already marked on the seeded `sblocks` `BlockCopy`s** (`structure_loops`/`calc_forward_dominator`
run on `bblocks` in `structure_reset`, copied verbatim by `build_copy_from` — so the structurer
gets loop-edge marks and `immed_dom`/`index` for free), plus a port of
`CollapseStructure::negate_condition_rec` so a loop-condition orientation flip (the `i==0`
do-while/while-do arm) records a deferred **pending-flip** that `run_region_structurer` returns
and `ActionBlockStructure` realizes via `block_basic_negate_lastop` — identical to the
`CollapseStructure` `take_pending_flips` path. The `BlockWhileDo` fold carries Ghidra's
`ruleBlockWhileDo` **interior-goto-target guard**: a loop whose head is the target of a
virtualized continue/goto is NOT foldable as a top-tested `while` (that would render a `label:`
*inside* the `while (...)` condition — malformed C); such a loop folds as `BlockInfLoop` or falls
back. Honest-partial-safe throughout: a loop class that doesn't match falls back to
`CollapseStructure` (never a panic, never malformed). Default-OFF ⇒ 675/675 datatests
byte-identical; ON-vs-OFF decompile speed is within noise on loop-heavy functions.

A **multi-latch / multi-exit refinement** (`refine_loop_edges`, the kuna analog of
`_refine_cyclic_core` — virtualize the secondary back-edges/exits/mid-body entries so a
not-yet-clean loop collapses) is implemented but **deliberately NOT run in Inc 3**: a 262-binary
corpus sweep showed it produces **no goto reduction** over Ghidra's `select_goto`/`TraceDAG` *and*
its edge choice can render a worse (extra-goto) or — for a continue-goto into the head condition —
malformed loop. Inc 3 therefore folds only the **directly-foldable** loops (the clean reducible
shapes, which need no edge virtualization at all — verified: do-while/while/for/inf-loop and
nested loops fold with zero refinement rounds) and leaves everything else to the
virtualize-fallback → `CollapseStructure`, which **guarantees ON-output is never worse than OFF**.
The refinement machinery is retained (behind `#[allow(dead_code)]`) for an increment that pairs it
with a post-dominator exit analysis (Inc 5).

**break/continue:** angr emits `BreakNode`/`ContinueNode` directly in the structurer. kuna instead
determines break/continue in a **separate, opt-in** pass (`kuna_loopbreak_recovery::kuna_scope_break`,
gated by `option loopbreak_recovery`, default-OFF) that runs in `ActionFinalStructure` *after*
structuring and lowers loop-exit `goto <successor>` edges to `break;` once the loop scope is
finalized. The region structurer therefore marks every virtualized loop edge as a **plain goto**
(`set_goto_branch`) — setting `f_break_goto`/`f_continue_goto` before that pass mis-renders the
loop — and the gated scope-break pass converts them, exactly as it does for the CollapseStructure
path. This keeps the loop recovery parity-safe and renderer-correct.

**Empirical finding (the goto-reduction is IRREDUCIBLE-only, as Inc 1 predicted):** across 40+
hand-crafted loop shapes (gcc/clang -O0/-O1/-O2: clean do-while/while/for/inf-loop, multi-continue,
multi-break, two-latch with side effects, nested loops, head-in-body two-entry, and goto-spaghetti
loops) **Ghidra's `CollapseStructure` already emits the minimal goto count** — ZERO for every
*reducible* loop (it has comma-folding into the condition, a complete while/do-while/inf-loop rule
set, and a good `select_goto`), and exactly ONE for a genuinely *irreducible* loop (a 2-entry
natural loop *needs* a goto to enter, and Ghidra's single goto is already minimal). The region
structurer **matches** this: it folds the reducible loops byte-identically (`tests/stages/`
`regionstructure-loop.xml` asserts the three-class success path + ZERO gotos, the honest parity
result, mirroring Inc 1's acyclic finding) and falls back on the irreducible / nested-interlock
ones (e.g. fmt's `get_space`, where an inlined-`getc` interlock yields 2 gotos that both Ghidra and
the region structurer leave). The roadmap's multi-goto target — the **PR #44 head-in-body**
function — lives in a **Windows PE** binary the kuna loader does not yet accept (`kuna decompile`
rejects PE32+; that loader is a separate in-flight effort), so that specific over-virtualization
could not be reproduced in-tree. No constructible-in-C or available-x86-64-ELF loop in the corpus
exhibits a Ghidra over-virtualization the flat schemas can beat.

**Consequence for the roadmap:** Inc 3 delivers **live, correct, parity-safe loop recovery**
(all three classes folded via the cyclic schemas, proven on the success path) — the structural
foundation the goto-reduction needs — but the **measurable goto delta** on a loop awaits either
(a) the PE loader, to reach the PR #44 / `1after909`-family irreducible regions, or (b) Inc 5's
post-dom (H2) refinement, which restructures the irreducible interlocks (`get_space`) Ghidra and
the flat cyclic schemas both leave as gotos. The cyclic schemas + the break/continue plumbing are
in place for both.

## Inc 5 — implementation status & findings (short-circuit / condition folding, option `regionstructure`)

**Landed (parity-safe, 3 gates green):** the region structurer now folds **cascading
short-circuit conditions** into a single compound `&&`/`||` condition, matching Ghidra
byte-for-byte. `region_structurer::match_acyclic_short_circuit_conditions` (→ `try_block_or`)
is a faithful port of Ghidra `CollapseStructure::ruleBlockOr` (`blockaction.cc:1321`) — the
kuna analog of angr `phoenix._match_acyclic_short_circuit_conditions` (types a–d). When a 2-out
condition `bl` has a single-in, non-complex, 2-out successor `orblock` that reconverges through
a shared clause, the pair folds into a [`BlockCondition`](../decompiler/crates/kuna-decomp/src/substrate/block.rs)
via `new_block_condition`, which picks `CPUI_INT_AND`/`CPUI_INT_OR` from the edge orientation
*exactly* as Ghidra does (so the rendered `&&`/`||` is byte-identical). The `isComplex` gate
reuses `BlockBasic::isComplex` (precomputed `complex_blocks`, the same set `CollapseStructure`
consumes), and the condition orientation uses the existing `negate_condition_rec` (recording the
deferred data-flow flips into `pending_flips`). It runs as an **up-front fixpoint** before the
sequence/ITE/loop cascade, mirroring Ghidra's `collapseAll` order (`collapseConditions` — which
is *only* `ruleBlockOr` — runs before `collapseInternal`).

**This closes the default-on blocker.** The single datatest where the region structurer (`on`)
diverged from Ghidra was `elseif.xml` *Else-if #14*: the `if (b==0xc9) { if (a<=299) { … } }`
cascade rendered as a nested `if` under `regionstructure on` instead of Ghidra's folded
`else if ((b == 0xc9) && (a <= 299))`. Inc 5 makes the region structurer produce the **identical**
folded `&&` form. With the option temporarily flipped default-ON, **the whole 675-assertion
datatest suite is byte-identical to default-OFF (0/675 divergences, PARITY OK)** — so the future
Inc 6 default-on flip is clean. `tests/stages/regionstructure-shortcircuit.xml` asserts the
`&&`-folded form across BOTH passes (off=CollapseStructure, on=region structurer) on the upstream
`testElseIf` binary; the full per-function C output is byte-identical between the two passes.

**Honest-partial-safe:** a 2-out node whose successor is not a single-in, non-complex, 2-out
condition reconverging through the shared clause is left untouched and flows to the
sequence/ITE/virtualize schemas (never a panic, never a whole-function abort). Default-OFF ⇒
675/675 byte-identical; ON-vs-OFF decompile speed is unchanged (the fold only runs under the
option, and the up-front fixpoint is bounded by the same `round_cap` as the main loop).

**Not in this increment:** the SAILR **H2** post-dominator virtualization heuristic (the
`postdom_max_edges`/`postdom_max_graph_size`-capped refinement that would restructure the
irreducible `get_space`-style interlocks Inc 3 leaves as gotos) is still future work — Inc 5 here
delivers the **short-circuit `&&`/`||` folding** half of the roadmap's Inc 5, which is the piece
that unblocks the clean default-on flip; the H2 post-dom refinement remains for a later pass.
