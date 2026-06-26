# [PROPOSAL] CrossJumpReverter: revert ISC condensing of a non-return cross-jump tail

**Opportunity:** `test_who_condensing_opt_reversion :: scan_entries`
**Binary:** `who.o` (x86_64), function `scan_entries` (entry `0x401250`)
**Proposed option:** `crossjumpreverter` (default-OFF, opt-in while developing)
**angr reference:** SAILR `CrossJumpReverter` (+ `ReturnDuplicatorLow`),
`angr/analyses/decompiler/optimization_passes/cross_jump_reverter.py`,
Basque et al., USENIX Security 2024 ("SAILR").

## The problem

kuna emits **one** `goto` in `scan_entries`; angr's "full" preset emits **zero**. The goto
is a residual of compiler **Irreducible Statement Condensing (ISC)**: two identical
`v1 = *a1;` tails (the `need_deadprocs` case's and the cascade's skip/else path's) were
condensed by the compiler into **one shared basic block** reached by a `goto`:

```c
else if (need_deadprocs && v1 == 8) { print_deadprocs(a1); goto label_4012b5; }
...
else { label_4012b5: v1 = *a1; }      // shared, condensed tail; falls through to:
if (v1 == 2) { ... }
```

The SAILR de-optimization reverts the condensing by **duplicating the shared tail back**
into the goto source, so both paths fall straight through and the goto/label vanish.

This is the **CrossJumpReverter** case — a duplication of a **NON-return, fall-through**
cross-jump tail. It is distinct from the already-shipped `gotoreduce` option, whose
`return_tail_chain()` only fires when the tail ends in `return`. Measured: no existing
kuna option (`gotoreduce`, `regionstructure`, `loopbreak_recovery`, `foldcallret`, in any
combination) removes this goto — it stays at 1. See `analysis.md` for the table and the
full side-by-side (`angr-vs-kuna.txt`).

## Why this is LARGE (the proposal gate)

A scope-decider subagent ruled this **large** (verdict + justification recorded verbatim in
`record.json`). It trips Hard Rule 7 on three counts:

1. **New pass *type* / structured-tree-surgery infrastructure.** `gotoreduce`'s
   `kuna_inline_return_tail` appends duplicated `BlockCopy` leaves to an `if` true-clause
   for a tail that ends in `return` — a *closed* path, no fall-through to reconcile, no
   edge rewiring. A cross-jump tail **falls through** to a shared successor (`if (v1 == 2)`),
   so the reverter needs a *new* surgery (`kuna_inline_crossjump_tail`) that duplicates a
   non-return block and reconciles the fall-through — machinery `gotoreduce` deliberately
   lacks.
2. **A convergence/post-dominance precondition.** Duplicating is only correct when the
   goto-source if-block's own structured fall-through already converges on the *same*
   successor as the target's fall-through. Establishing that is a real dominator/convergence
   analysis over the **structured tree**, not the bounded single-successor *bblock* walk
   `return_tail_chain` performs. **A wrong precondition silently emits incorrect C** — a
   correctness defect far worse than the single harmless goto it removes.
3. **A likely multi-pass / edge-ordering dependency.** The angr test docstring states
   `scan_entries` reaches zero gotos only via **two** passes (`ReturnDuplicatorLow` +
   `CrossJumpReverter`) *and* a **specific edge-virtualization ordering** (post-order gives
   two gotos; virtualizing `0x401361 -> 0x4012b5` gives one, which the reverter then
   removes). kuna's verbatim `CollapseStructure`/`TraceDAG` structurer does not virtualize
   edges the way angr's region structurer does, so the goto *shape* kuna hands a reverter is
   not the shape angr's reverter consumes — there is no guarantee a single bounded Action
   lands on this exact goto.

This is the same SAILR tail-duplication family already parked as a proposal for the
return-tail gotoreduce gap (`docs/baseline`/PROGRESS history; sibling in-flight worker
branches `feat/angr-*ret-dup*`, `feat/angr-incorrect-duplication-chcon-*`). The
CrossJumpReverter (non-return) variant is the missing third leg.

## Proposed implementation plan (for human go/no-go)

1. **New module `kuna_crossjumpreverter.rs`** (S8, an `ActionCrossJumpReverter` modeled on
   `kuna_gotoreduce.rs`), gated by a new `crossjumpreverter` option
   (`revert_cross_jumps` flag on the seam `Architecture`, default-off; gated early-return so
   default output is byte-identical).
2. **Detection.** Over the structured (sblocks) tree, find every `if (cond) goto T` where
   `T` is a small (≤ N ops, no `CALL`/`STORE`) **non-return** block that falls through to a
   successor `S`.
3. **Convergence precondition.** Prove the goto-source if-block's structured fall-through is
   exactly `S` (a post-dominator/convergence check on the structured tree). Decline
   otherwise — this is the correctness-critical step.
4. **Surgery `kuna_inline_crossjump_tail`.** Duplicate `T`'s ops (print-tree `BlockCopy`
   leaves — no p-code cloned, SSA untouched) into the `if` true-clause in place of the
   `goto`; clear `T`'s label when no unstructured edge still targets it.
5. **Edge-virtualization-ordering investigation.** Determine whether kuna's structurer
   presents the right single-goto shape; if not, scope the additional ordering work (this is
   the part that may force a *second* gated pass and is the chief risk to a clean default-ON
   ablation).
6. **Validation.** New `tests/stages/ghangr-who-condensing-opt-reversion-72e518.xml`
   (two-pass: off ⇒ the goto, default/on ⇒ zero gotos), full `kuna test --all` ablation,
   speed measurement on `scan_entries`, and a `docs/divergences.md` DIV entry **only if** the
   ablation is clean and the option ships default-ON.

## Speed / risk assessment

- **Risk: HIGH for correctness** (precondition #3). Must be conservative — decline on any
  doubt; bound tail size; never duplicate side-effecting ops.
- **Risk: MEDIUM for completeness** — the structurer-shape/edge-ordering dependency (point 3
  above) may mean one Action does not reach zero gotos on this function, requiring follow-up.
- **Speed:** a bounded, post-structuring print-tree pass should be cheap (comparable to
  `gotoreduce`); to be measured during implementation. Ships **default-OFF opt-in** until
  the ablation is proven clean and within the speed budget.

## Proposed option name

`crossjumpreverter` (`change_kind = structure-recovery`,
`inspiration = "test_who_condensing_opt_reversion; CrossJumpReverter/ReturnDuplicatorLow; scan_entries"`).

---
On approval, re-dispatch an implementation worker on this branch
(`feat/angr-who-condensing-opt-reversion-72e518`).
