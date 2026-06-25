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
