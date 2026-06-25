# [PROPOSAL] angr-dd-argmatch-to-argument: SAILR goto-reduction by return-tail duplication

**Status:** draft proposal — needs human go/no-go before an implementation worker is spent.
**Scope:** large (per Hard rule 7 and the recorded decider verdict).
**This is the same family as, and should be folded into, the existing `gotoreduce` proposal**
(`morton-my-message-callback-bfd2fa`). dd/argmatch is the simpler *return-constant* variant of
the same transform — file it as an additional motivating testcase, not a competing point fix.

## The problem

On `argmatch_to_argument` in `dd` (x86_64 @ `0x40a640`) the **only** structural difference
between angr and kuna is a single residual `goto` + `label:`:

| metric | angr | kuna |
|---|---|---|
| gotos | **0** | **1** |
| labels | **0** | **1** |
| loops / switches | 1 / 0 | 1 / 0 |

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

The shared block `label_40a6a3: v2 = 0;` (falling through to the single `return v2;`) has two
predecessors — the initial `if (v2 == 0)` true-arm and the in-loop `if (v2 == 0) goto` edge — so
the structurer can only express the second edge as an unstructured `goto`.

angr (zero gotos) **duplicates the shared return tail** (`v2 = 0; return v2;` → `return 0;`)
into each path, turning the cross-edge into a structured early return. See `analysis.md` and
`angr-vs-kuna.txt` for the full side-by-side.

## The angr reference pass / class

angr's **goto-reduction via tail duplication** — the Phoenix/SAILR family in
`angr.analyses.decompiler` (`ReturnDuplicator` / `DuplicationOptimizer` / `GotoSimplifier`).
The structurer is allowed to duplicate a small, side-effect-bounded tail (here just the
constant `return 0`) to eliminate an unstructured edge; its quality metric trades a couple of
duplicated statements for zero gotos. This case is the cheapest variant: a **return-constant**
tail.

## Why this is large (owning stage: S8 structuring)

The goto is produced by `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`, the
**verbatim port of Ghidra's `CollapseStructure` / `TraceDAG` / `BlockTrace`** (~3635 lines).
Its schema precedence order and DAG bad-edge scoring *choose* which edge becomes a goto.
Ghidra has **no schema that duplicates a tail to avoid a goto** — that transform does not
exist in the collapse algorithm.

Matching angr therefore requires a **new structuring transform**, not a gated `Action` that
manufactures an artifact for the existing structurer (the way `kuna_loweredswitch.rs`
manufactures an S2 BRANCHIND/JumpTable). Concretely it must:

1. identify unstructured ("goto") edges after `CollapseStructure` whose **target** is a small
   single-exit block ending in `return` (statement-count ≤ threshold, no side effects unsafe to
   duplicate — here a bare `v2 = 0; return v2;`);
2. **duplicate** that target's statements into the goto source (a `BlockGraph` mutation — clone
   the `BlockBasic`, its p-code ops, and rewire edges), then drop the unstructured edge;
3. re-collapse locally so the duplicated tail folds into the arm as a structured early return;
4. preserve def/use and ordering — the duplicated assignment/return are real p-code ops over a
   live `v2` Varnode, so this is **not** a print-time textual rewrite.

That is a **new pass type touching S8 structuring/region code beyond a single gated
early-return** — three independent triggers of Hard rule 7 (new pass type; S8 mutation; new
module + block/p-code-clone anchors). The decider verdict (recorded in `record.json`) is
`scope: large`, and a post-emit textual peephole is explicitly ruled out (it would have to
reconstruct structured control flow from emitted C and re-prove def/use equivalence for the
live `v2` Varnode).

## Proposed implementation plan (multi-step — for the approved worker)

Implement **once** as the shared `gotoreduce` pass (the morton proposal); make it cover the
return-constant variant this testcase exercises.

- **New module** `kuna_gotoreduce.rs` (S8) — a post-`CollapseStructure` `Action`, option-gated,
  default-OFF.
- **Anchor edits (minimal, `// (kuna)`):** register the action in `universalaction.rs` / the
  relevant `coreaction*` registration; option flag on the architecture struct (reset-default
  off); `options.rs`; `stages.toml` `settableTable` row.
- **Core mechanics:** a `BlockGraph` helper to clone a `BlockCopy`/`BlockBasic` tail and rewire
  the floating edge (the genuinely new, risky piece — it mutates the structured block tree the
  verbatim engine produced).
- **Bounds / fail-safe:** cap duplicated statement count (e.g. ≤ 4 ops, single-exit, ends in
  `return`), bail on any block whose duplication would change semantics, mirroring
  loweredswitch's fail-safe cap. The return-constant case is the safe lower bound.
- **Tests:** `tests/stages/ghangr-dd-argmatch-to-argument-noea-9e6e8b.xml` two-pass (off = goto
  present, on = goto gone) as the minimal return-constant fixture; plus the morton epilogue
  fixture for the `free(p); return;` variant.

## Speed / risk assessment

- **Risk: high.** Mutating the structured block tree after a verbatim-ported engine; incorrect
  duplication can change semantics or desync def/use. The bounds above contain it but need
  adversarial testing across the 675-assertion datatest corpus.
- **Ablation risk:** likely non-zero — tail duplication changes rendering on functions that
  currently use a clean shared-tail goto, so expect **default-OFF opt-in** even if correct.
- **Speed:** an extra post-structuring scan + occasional re-collapse; expected small but must be
  measured against the +5% budget on the target.
- **Same family** as the irreducible-loop SAILR gap (proposal PR #46) and the morton/newburry/
  tr-build proposals — build **one coherent goto-reduction layer**, not point fixes.

## Proposed option name

`gotoreduce` (reuse the morton-proposed option). `change_kind = structure-recovery`,
`source_decompiler = angr`,
`inspiration = test_decompiling_dd_argmatch_to_argument_noeagerreturns; SAILR/Phoenix goto-reduction (return-tail duplication / ReturnDuplicator); argmatch_to_argument`.
