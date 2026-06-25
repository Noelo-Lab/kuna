# Analysis — angr `dd::argmatch_to_argument` vs kuna (SAILR goto-reduction)

**Testcase:** `test_decompiling_dd_argmatch_to_argument_noeagerreturns::argmatch_to_argument`
**Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd` (x86_64, `argmatch_to_argument` @ `0x40a640`)
**angr:** 9.2.213

## The gap

The **only** structural difference between angr and kuna is a single residual `goto` +
`label:`. Both produce one loop and the same conditionals; kuna's metrics differ only in
gotos/labels:

| metric | angr | kuna |
|---|---|---|
| gotos | **0** | **1** |
| labels | **0** | **1** |
| loops | 1 | 1 |
| switches | 0 | 0 |

kuna:

```c
int8 argmatch_to_argument(unsigned long a0,int8 *a1,int8 a2,int8 a3)
{
  int4 v1; // eax
  int8 v2;

  v2 = *a1;
  if (v2 == 0) {
label_40a6a3:
    v2 = 0;
  }
  else {
    v1 = bcmp(a0,a2,a3);
    while (v1 != 0) {
      a2 = a2 + a3;
      a1 = &a1[1];
      v2 = *a1;
      if (v2 == 0) goto label_40a6a3;
      v1 = bcmp(a0,a2,a3);
    }
  }
  return v2;
}
```

The shared block `label_40a6a3: v2 = 0;` (which then falls through to the single
`return v2;`) has **two** predecessors — the initial `if (v2 == 0)` true-arm and the
in-loop `if (v2 == 0) goto` edge — so the structurer can only express the second edge as
an unstructured `goto`.

angr (zero gotos) **duplicates the shared return tail** (`v2 = 0; return v2;` →
`return 0;`) into each path, turning the cross-edge into a structured early return:

```c
unsigned long long argmatch_to_argument(...)
{
    if (!*(a1))
        return 0;                 // <- duplicated return-0 tail (path 1)
    if (bcmp(a0, a2, a3))
    {
        ...
        while (true)
        {
            ...
            if (!v3[0])
                break;            // <- loop exits to the shared return-0 (path 2)
            ...
        }
    }
    else
    {
        return *(a1);
    }
    return 0;                      // <- duplicated return-0 tail (path 2)
}
```

See `angr-vs-kuna.txt` for the full side-by-side.

## The angr reference pass / class

This is angr's **goto-reduction via tail duplication** — the Phoenix/SAILR family in
`angr.analyses.decompiler` (region-simplification + `DuplicationOptimizer` / `ReturnDuplicator`
/ `GotoSimplifier`). angr's structurer is allowed to duplicate a small, side-effect-bounded
tail (here just `return 0`) to eliminate an unstructured edge; its quality metric trades a
couple of duplicated statements for zero gotos. This particular case is the **return-constant
duplication** variant (the cheapest possible tail: a single constant return).

## Owning stage — why this is large

The goto is produced by `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`, the
**verbatim port of Ghidra's `CollapseStructure` / `TraceDAG` / `BlockTrace`** — the schema
precedence order and the DAG bad-edge scoring *choose* which edge becomes a goto. Ghidra has
no schema that duplicates a tail to avoid a goto; that transform does not exist in the collapse
algorithm.

Matching angr requires a **new structuring transform** (clone the shared return tail's
`BlockBasic`/ops, rewire edges, re-collapse so each arm folds the duplicated tail into a
structured early return, preserving def/use for the real `v2` Varnode) — a new pass type
touching S8 structuring/region code beyond a single gated early-return. That is two
independent triggers of Hard rule 7 → **large, proposal-required**.

## Relationship to existing work (do not duplicate)

This is the **same SAILR goto-reduction family** already proposed and parked:

- `[PROPOSAL] morton-my-message-callback-bfd2fa: SAILR-style goto-reduction (tail duplication)`
  — proposes the new S8 `kuna_gotoreduce.rs` pass. Morton is the `free(p); return;` epilogue
  variant; **dd/argmatch is the simpler `return <const>;` variant** of the *same* transform.
- `[PROPOSAL] newburry-main-57ccb1: SUID-guard goto-elimination is the SAILR family (large)`
- `[PROPOSAL] tr-build-spec-list-ea32be: SAILR condition folding / goto reduction (S7, large)`
- `dir-gcc-o0-main-e5544a` / `phoenix-natural-loop-region-1c12bf` — NEGATIVE results: the goto
  gap needs SAILR-grade region restructuring, not a peephole.

`dd::argmatch_to_argument` should be folded in as an **additional motivating testcase** for the
single coherent `gotoreduce` pass rather than spawning a competing point fix. No existing
*shipped* option covers it (`kuna catalog --json` has no goto-reduction / tail-duplication
option), so this is not "covered" under Hard rule 5 — it is covered by an existing *proposal*.

## Hypothesis for the kuna change (for the approved implementation worker)

Implement the proposed S8 `kuna_gotoreduce.rs` (default-OFF, option `gotoreduce`) and make it
handle the **return-constant tail** case demonstrated here: detect an unstructured edge whose
target is a small single-exit block ending in `return` (≤ N ops, no side effects unsafe to
duplicate), clone it into the goto source, rewire, and re-collapse. dd/argmatch is the minimal
fixture for that variant.
