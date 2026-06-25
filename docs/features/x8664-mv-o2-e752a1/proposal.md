# [PROPOSAL] angr-x8664-mv-o2-e752a1 — getopt loop + switch structuring in `mv -O2 main`

**Status:** design-only. This gap is **large** (Hard rule 7) and needs human go/no-go
before any implementation worker is spent. A decider subagent confirmed `scope: large`
(verdict recorded verbatim in `record.json`).

- Opportunity: `test_decompiling_x8664_mv_O2::main`
- Binary: `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/mv_-O2`, `main` @ `0x402b40`, x86_64
- Reference: angr 9.2.213. Full side-by-side: `angr-vs-kuna.txt`. Analysis: `analysis.md`.

## The problem

`mv`'s `main` is a getopt option-parsing loop:

```c
while (true) {
    v = getopt_long(a0, a1, "bfint:uvS:TZ", &long_options, NULL);
    if (v == -1) break;
    switch (v) { case 'b': ...; case 'f': ...; default: usage(1); }
}
```

angr renders exactly that. kuna recovers the **same jump-table switch** but wraps it in
goto-soup: **24 gotos / 14 labels** vs angr's **9 / 8**, and only **10 of 13** switch
cases. The four independent structural defects (evidence in `analysis.md`):

1. **Switch breaks emitted as gotos.** 9 case bodies end `goto label_2c88` (the
   post-switch latch block) instead of `break`.
2. **The getopt loop is not recovered as a loop.** kuna emits the prologue *after* the
   switch and back-jumps over it (`goto label_2c88`); angr recovers `while (true)` with
   the prologue ordered first.
3. **3 sparse switch cases missing.** The long-option return values 128 / `--version`
   (`0xFFFFFF7D`) / `--help` (`0xFFFFFF7E`) survive as an equality cascade folded inside
   kuna's `default:` arm instead of being jump-table cases.
4. **Tail goto-soup.** The post-getopt `target_directory_operand`/`do_move` region uses
   ~6 gotos where angr nests `if/else`.

## Why this is large (not one Action/Rule)

The clean angr shape is the **aggregate** of several distinct structuring transforms,
each touching Ghidra's **S7 region / S8 structuring** code — the boundary Hard rule 7
explicitly fences off — not a single Varnode-Rule or single Action (contrast
`kuna_loweredswitch.rs`, which manufactures one jump table from one Action):

- **switch-break recovery** (case-exit-to-follow → `break`) — S8 `BlockSwitch`/structurer;
- **loop head/latch recovery + block reordering** so the getopt loop becomes
  `while (true)` with the prologue first — S7/S8;
- **jump-table augmentation** to absorb the 3 sparse equality cases into the recovered
  switch — S2 jump-table + a cascade detector;
- **goto → if/else condensing** in the tail — S7/S8.

The decider also noted the sub-gaps are **entangled here**: the most natural standalone
small feature, switch-break recovery (#1), cannot be isolated in *this* function because
the post-switch convergence block (`label_2c88`) is *also* the loop latch — a `break`
there is only correct once the enclosing `while (true)` loop (#2) is recovered. So #1 and
#2 must land together, which already exceeds the one-Action budget.

This is the angr **SAILR** structuring contribution (loop refinement, switch-break
recovery, region condensing; Basque et al., USENIX Security 2024) applied to Ghidra's
structurer — a multi-pass S7/S8 effort, not a single gated rule.

## angr reference

The relevant angr passes are in `angr.analyses.decompiler.structuring` (the
SAILR/Phoenix structurer) plus `RegionIdentifier` and the optimization passes
(`LoweredSwitchSimplifier` for #3-style cascades). kuna already ports
`RegionIdentifier` (`s7_regions/kuna_regionid.rs`, `kuna_regiongraph.rs`) and the
lowered-switch detector (`s2_lift/kuna_loweredswitch.rs`) — the proposed work builds on
those seams.

## Proposed multi-step implementation plan (for human review)

Each step is its own option-gated PR; recommend tackling in this order, smallest first:

1. **`jumptable-augment` (S2)** — extend the lowered-switch cascade detector to detect an
   equality cascade in the `default`/guard region of an *already-recovered* `BRANCHIND`
   switch and append its (value → target) pairs to the existing `JumpTable`. Closes #3.
   Self-contained-ish (builds on `kuna_loweredswitch.rs`); medium risk (mutates a live
   jump table). Worth attempting standalone first.
2. **`getopt-loop-recover` (S7/S8)** — recognise the `do { v=call; if (v==-1) goto out; }`
   self-loop whose body is a switch and re-form it as `while(true){...; if(...) break;}`
   with prologue ordering. Closes #2; unblocks #1. Largest/riskiest step (touches loop
   structuring + block order).
3. **`switch-break-recover` (S8)** — once the loop is a loop, render case-exit-to-latch as
   `break`. Closes #1. Small, but depends on step 2.
4. **`tail-condense` (S7/S8)** — goto→if/else condensing for the tail. Closes #4. Likely
   the general SAILR condensing transform; broad blast radius — assess separately.

## Speed / risk assessment

- **Risk:** high. Steps 2-4 touch S7/S8 region/structuring — the most parity-sensitive
  stages (every datatest exercises the structurer). Each step must ship **default-OFF**
  until an ablation over all 675 datatests is clean, and each carries real regression risk
  to the `make test` / `make test-stages` gates.
- **Speed:** unmeasured (no implementation). Loop/region recovery adds a structuring pass;
  expect to measure per-step against the +5% target budget and likely keep at least the
  loop-recovery step opt-in if it regresses the large `main`.
- **Recommended scope per implementation worker:** **one** of the four steps, default-OFF,
  with its own stage test — not all four at once.

## Proposed option name(s)

`jumptable-augment`, `getopt-loop-recover`, `switch-break-recover`, `tail-condense`
(one per step; `change_kind = structure-recovery`, `source_decompiler = angr`).

## Recommendation

Approve **step 1 (`jumptable-augment`)** as the first standalone implementation worker —
it is the closest to a single Action (builds directly on `kuna_loweredswitch.rs`), has the
narrowest blast radius, and delivers a measurable win (3 recovered cases, fewer
default-arm gotos) without the entangled loop-recovery work. Defer steps 2-4 pending its
outcome.
