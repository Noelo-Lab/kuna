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
