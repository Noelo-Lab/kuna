# [PROPOSAL] `dedup-ite-tail` — condition-aware if/else tail deduplication (S8 structuring)

**Opportunity:** `test_decompiling_true_1804_x86_64::usage`
**Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu1804` — `usage` (x86_64 @ `0x401610`)
**angr:** 9.2.213 · **Scope:** LARGE (Hard rule 7) · **Proposed option:** `dedup-ite-tail` (default-off) · **Proposed ElementId:** 4100

## The problem

On `usage`, angr's decompilation is materially cleaner than kuna's
(loc 112 vs 121, gotos 2 vs 3, labels 2 vs 3, ifs 6 vs 9). The full analysis is
in [`analysis.md`](./analysis.md); the side-by-side is in
[`angr-vs-kuna.txt`](./angr-vs-kuna.txt). Two structuring-quality differences:

1. **Duplicated if/else tail (dominant).** kuna's trailing
   `if (v28 == 0) { A } else { B }` duplicates a shared prefix and suffix across
   *both* arms — the `__printf_chk("...online help...")` call, the `setlocale(5,0)`
   call, the `strncmp(...,"en_",3)` check, and the `__printf_chk("...Full
   documentation...")` call all appear twice. angr emits each once and converges the
   two distinct continuations through shared labels (`LABEL_4018e8`/`LABEL_401822`).
2. **Loop shape.** kuna emits the inlined `rep cmps` loop with a `goto` out of the
   loop body; angr renders it as a `do/while` terminated by a `break`.

Both are decisions of the **structurer**, i.e. kuna's S7/S8 `blockaction`
CollapseStructure/TraceDAG region-collapse engine and its goto-vs-`break` schema
selection (`decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`).

## Why this is not a single gated Action (the scope call)

A decider subagent (verbatim verdict recorded in `record.json → decisions`)
returned **`scope: large`, `proposal_required: true`**. Reasoning:

- The duplication is produced by the structurer **cloning a shared continuation**
  instead of converging two predecessors onto one labelled block. There is no
  pre-structuring dataflow form that "shares" the tail — sharing it is a
  condition-aware ITE region-dedup capability that lives *inside*
  CollapseStructure/TraceDAG.
- Unlike `kuna_loweredswitch` (which fabricates an S2 `BRANCHIND`+`JumpTable` and
  lets the **unchanged** structurer render it), there is **no manufacturable upstream
  CFG artifact** that makes Ghidra's collapse engine share the continuation.
- Closing the gap therefore requires a new pass *type* touching S8
  `blockaction` region collapse — exactly Hard rule 7's "new infrastructure" and
  "touch S7/S8 structuring beyond a single gated early-return" triggers.

## angr reference

- angr's structurer condition-processing / ITE region handling — the
  `RegionSimplifier` / condition-deduplication path in
  `angr/analyses/decompiler/region_simplifiers/` and the structurer's
  `_make_ites` / condition-node merging that converges shared continuations onto
  labelled successors rather than cloning them. (kuna already has the analysis-only
  region tree from the RegionIdentifier port: `kuna_regionid` / `kuna_regiongraph`,
  S7 — a natural substrate.)

## Proposed implementation plan (multi-step — for human go/no-go)

1. **Detection (S7, analysis-only).** Using the existing `kuna_regionid` region
   tree, identify an `if/else` region whose two arms share a maximal common
   *prefix* and/or *suffix* of statement-equivalent blocks (same PcodeOp sequence,
   modulo the condition varnode). Bound it (small fixed cap on hoisted blocks) to
   keep it safe and fast.
2. **Transform (S8, gated).** Behind `option dedup-ite-tail` (default-off), rewrite
   the region so the shared prefix/suffix is emitted once and the divergent middles
   are reached via labelled blocks — i.e. teach the collapse engine (or a
   pre-collapse normalizer feeding it) to converge the predecessors. This is the
   genuinely new region-collapse logic.
3. **Loop schema (optional, separable).** A follow-up gated tweak to
   goto-vs-`break` selection for the `rep cmps` loop; can ship separately.
4. **Verify & gate.** New `tests/stages/ghangr-true-1804-x86-64-8a690c.xml`
   (off=current duplicated tail, on=deduplicated). `kuna test --all` ablation must
   stay PARITY OK with the option default-off; measure target decompile speed
   off-vs-on (structurer changes are the risk).

## Speed / risk assessment

- **Risk: HIGH.** Region-collapse changes are the most correctness-sensitive part of
  the pipeline — a wrong convergence emits invalid C. Must stay default-off until an
  ablation over the full datatest corpus is clean.
- **Speed:** detection is O(region · block-eq) over a bounded window; expected
  small, but must be measured per Hard rule 6 because it runs in the structurer.
- **Generality:** the shared-prefix/suffix-across-if-arms shape is common in
  gettext/`--help`-style code (coreutils family) and in canary/cleanup epilogues, so
  the pass should generalize beyond this single function — but that breadth is also
  why it must be gated and corpus-ablated before any default-on consideration.

## Recommendation

Approve as a **draft proposal**. On go, re-dispatch an implementation worker on this
branch to build the S7 detector + S8 gated region converter as `dedup-ite-tail`
(ElementId 4100, default-off), with the stage test and full ablation.
