# who-condensing-opt-reversion — analysis

**angr testcase:** `test_who_condensing_opt_reversion :: scan_entries`
**Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/who.o` (x86_64)
**Function:** `scan_entries` (entry `0x401250`)
**angr version:** 9.2.213

## What angr does better

The angr test asserts `goto count == 0` for `scan_entries`. With its "full" preset
angr reaches **zero gotos**; **kuna emits exactly one** (`goto label_4012b5`). The angr
test docstring is explicit about the mechanism:

> This testcase verifies that all the Irreducible Statement Condensing (ISC) optimizations
> are reverted by the **ReturnDuplicatorLow** and the **CrossJumpReverter** optimization
> passes. … there is some special ordering to edge virtualization … The default edge
> virtualization order (post-ordering) will lead to two gotos. virtualizing
> `0x401361 -> 0x4012b5` will lead to only one goto … Either way, these gotos can be
> eliminated by the **CrossJumpReverter** duplicating the statement at the end of the goto,
> after ReturnDuplicatorLow has fixed up the return statements.

These are the SAILR (Basque et al., USENIX Security 2024) de-optimization passes that
*revert* compiler "Irreducible Statement Condensing".

## The exact construct (the one structural difference)

Inside the case-cascade of the `do { … } while(true)` loop, kuna renders:

```c
else if (need_deadprocs && v1 == 8) {
    print_deadprocs(a1);
    goto label_4012b5;            // <-- the residual goto
}
...
else {
label_4012b5:
    v1 = *a1;                     // <-- the shared, condensed tail
}
if (v1 == 2) { v5 = ...; }
a1 = &a1[0xc0];
...
```

`label_4012b5` is a **single block `v1 = *a1;`** that **falls through** to `if (v1 == 2)`
and the loop continuation. The compiler *condensed* two identical `v1 = *a1;` tails — the
`need_deadprocs` case's and the skip/else path's — into **one shared block reached by a
`goto`**. Reverting the condensing = duplicate `v1 = *a1;` back into the `need_deadprocs`
branch so both paths fall straight through to `if (v1 == 2)`:

```c
else if (need_deadprocs && v1 == 8) {
    print_deadprocs(a1);
    v1 = *a1;                     // duplicated; no goto
}
```

This is the **CrossJumpReverter** case: a duplication of a **NON-return, fall-through
cross-jump tail** (contrast gotoreduce, which only duplicates tails that end in `return`).

## Owning stage

S8 structuring / goto-reduction (the SAILR de-optimization family), running after
`ActionFinalStructure` on the structured (sblocks) tree — the same neighbourhood as the
existing `kuna_gotoreduce.rs` (`docs/stage-mapping.md` → S8). The reversion is a
print-tree / structured-block surgery, downstream of `CollapseStructure`/`TraceDAG`.

## Why no existing option closes it (measured)

`scan_entries` decompiled with each of `gotoreduce`, `regionstructure`,
`loopbreak_recovery`, `foldcallret` (alone and combined) **still emits 1 goto**:

| options | gotos |
|---|---|
| (default) | 1 |
| `gotoreduce on` | 1 |
| `regionstructure on` | 1 |
| `loopbreak_recovery on` | 1 |
| all four on | 1 |

`gotoreduce` declines because its `return_tail_chain()` requires the tail to end in
`return`; this tail (`v1 = *a1`) falls through to a successor. So the gap is **not covered**
by an existing option.

## Hypothesis for the kuna change (and why it is LARGE)

A SAILR **CrossJumpReverter** pass that duplicates a small NON-return, fall-through
cross-jump tail into its `goto` source. Unlike `gotoreduce`'s return-tail duplication
(`kuna_inline_return_tail`, a closed path), the fall-through case must additionally prove a
**convergence/post-dominance precondition** — that the `goto`-source if-block's own
structured fall-through reaches the *same* successor as the target's fall-through — and
needs **new structured-tree surgery** (`kuna_inline_crossjump_tail`) that `gotoreduce` does
not provide. The angr test further names a *second* pass (`ReturnDuplicatorLow`) and a
*specific edge-virtualization ordering* as prerequisites for this function reaching zero.

A scope-decider subagent (recorded verbatim in `record.json`) ruled this **large**: it
trips Hard Rule 7 on (1) new structured-tree-surgery *infrastructure* (a new pass type, not
modelable as one Action like `kuna_loweredswitch.rs`), (2) a likely multi-pass/edge-ordering
dependency, and (3) the convergence-precondition machinery `gotoreduce` intentionally lacks.
A wrong precondition check would silently emit **incorrect C** (worse than the one harmless
goto). Therefore this opportunity goes through a draft `[PROPOSAL]` PR for human go/no-go
rather than being implemented in a single worker session — see `proposal.md`.
