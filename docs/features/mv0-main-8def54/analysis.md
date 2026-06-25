# Analysis — `test_decompiling_mv0_main::main` (coreutils `mv`, x86_64)

Binary: `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/mv_0`
Function: `main` @ `0x402c40`  (angr 9.2.213)

## The gap (what angr does better)

`main` is GNU coreutils `mv`'s option-parsing entry: a `getopt_long` loop with an
11-case `switch` on the returned option character.

| metric | angr (ref) | kuna | delta |
|---|---|---|---|
| loc | 284 | 243 | |
| **gotos** | **10** | **29** | kuna ~3× more |
| **labels** | **9** | **16** | kuna ~2× more |
| **loops** | **3** | **2** | kuna recovers one fewer |
| switches | 1 | 1 | |
| cases | 11 | 11 | |

Signals emitted by the compare tool:
- ref has fewer gotos (10 vs 29)
- ref has fewer labels (9 vs 16)
- ref recovered more loops (3 vs 2)
- kuna emitted a recovery-failure marker

### The one concrete structural difference

angr renders the getopt loop natively, with the `switch` **inside** the loop:

```c
while (true) {
    v17 = getopt_long(a0, a1, "bfint:uvS:TZ", &long_options.name, NULL);
    if (v17 == -1) break;
    if (v17 > 128) goto LABEL_4030c1;
    if (v17 > 82) { switch (v17) { case 83: ...; break; ... } }
    else if (v17 == -131) { ... }
    ...
}
```

kuna **fails to recover this loop as a structured `while`**. Instead it:
1. emits a bare label `label_2d00:` at the loop header (the `getopt_long` call),
2. **hoists the entire `switch` out of the loop** to the very bottom of the
   function — physically *after* the function's normal `return` /
   `__stack_chk_fail` tail (`label_2d30: switch(v6) { ... }`),
3. makes **every** switch case end in `goto label_2d00;` to jump back to the
   header.

That single failure is the root of all three deltas: the hoisted switch's 11
back-jumps are the extra gotos (29 vs 10) and labels (16 vs 9), and the loop
itself is never counted as a structured loop (2 vs 3).

## Root cause (the owning stage)

This is **S8 structuring** (`decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`),
Ghidra's `CollapseStructure` / `TraceDAG` / `LoopBody` fixed-point graph reduction
(`ruleBlockWhileDo`, `emitLikelyEdges`). The getopt loop has **one header but many
latches** — each `switch` case is its own back-edge to the `getopt_long` block.
Ghidra's `ruleBlockWhileDo` collapses a loop only when it presents a *single*
structured back-edge; with N latches the extra back-edges get marked unstructured
(`emitLikelyEdges`), the loop degrades to a `BlockGoto` form, and the switch can no
longer nest under a `BlockWhileDo`, so it is hoisted out and gotos are synthesized.

angr keeps the switch inside the loop because its Phoenix/Dream-style structurer
(`RegionIdentifier` + loop refinement: break/continue insertion, single-latch
normalization) routes the many back-edges through `continue`, presenting one
back-edge to the loop body.

## Hypothesis for the kuna change

Recover the multi-latch loop as a structured `while` by **normalizing the N
back-edges to a single latch** (a synthesized common latch / pre-header) *before*
`ruleBlockWhileDo` runs, and teaching the `BlockSwitch` nesting path to keep case
bodies whose tail is a loop back-edge nested under the enclosing `BlockWhileDo`
(rendered as `continue`) instead of hoisting them.

## Scope verdict

A decider subagent (Opus 4.8) was asked SMALL vs LARGE under Hard rule 7 and
returned **LARGE / high confidence** — the fix lives inside the S8 collapse
fixed point (`blockaction.rs`) and must change how latches are unified before loop
collapse plus the switch-nesting path; it is not modelable as one gated
Action/Rule and cannot be done as a single gated early-return. Full decision in
`record.json` → `decisions`. Per Hard rule 7 this worker **stops at design and
files a `[PROPOSAL]` draft PR** (`proposal.md`); a human approves before any
implementation worker is spent.
