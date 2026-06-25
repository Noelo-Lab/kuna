# [PROPOSAL] angr-morton-my-message-callback: SAILR-style goto-reduction by epilogue tail-duplication

**Status:** draft proposal — needs human go/no-go before an implementation worker is spent.
**Scope:** large (per Hard rule 7 and the recorded decider verdict).

## The problem

On `my_message_callback` in `morton` (x86_64, @ `0x403de7`) the **only** structural
difference between angr and kuna is a single residual `goto` + `label:`:

| metric | angr | kuna |
|---|---|---|
| gotos | **0** | **1** |
| labels | **0** | **1** |
| ifs | 9 | 9 |
| loops / switches | 0 | 0 |

kuna:

```c
        else {
          v1 = strcmp(v3,"UV");
          if (v1 != 0) {
            v5 = strlen("Invalid Command");
            mosquitto_publish(a0,0,"app/error",v5 & 0xffffffff,"Invalid Command",1);
            goto label_41d6;
          }
          ...
        }
      ...
label_41d6:
  free(v6);
  return;
```

angr (zero gotos) — it **duplicates** the shared `free(ptr); return;` epilogue into each
arm that reaches it, so the cross-edge becomes a structured early return:

```c
        else
        {
            v5 = mosquitto_publish(a0, 0, "app/error", strlen(v9) & 4294967295, v9, 1, 0);
            free(ptr);
            return;
        }
        puts(ptr);
        ...
```

See `analysis.md` and `angr-vs-kuna.txt` for the full side-by-side.

## The angr reference pass / class

This is angr's **goto-reduction via tail duplication** (the Phoenix/SAILR family in
`angr.analyses.decompiler` — the region-simplification + `DuplicationOptimizer` /
`GotoSimplifier` style transforms). angr's structurer is *allowed* to duplicate small,
side-effect-bounded tails (a `return`, a `free(x); return`) to eliminate an unstructured
edge; its quality metric explicitly trades a few duplicated statements for zero gotos.

## Why this is large (owning stage: S8 structuring)

The goto is produced by `decompiler/crates/kuna-decomp/src/s8_structure/blockaction.rs`,
the **verbatim port of Ghidra's `CollapseStructure` / `TraceDAG` / `BlockTrace`**. Its
header states the schema-precedence order and the DAG bad-edge scoring are
"output-determining ... transcribed verbatim" — they *choose* which edge becomes a goto.
Ghidra has **no schema that duplicates a tail to avoid a goto**; that transform simply
does not exist in the collapse algorithm.

Matching angr therefore requires a **new structuring transform**, not a gated `Action`
that manufactures an artifact for the existing structurer (the way `kuna_loweredswitch.rs`
manufactures an S2 BRANCHIND/JumpTable). Concretely it must:

1. identify unstructured ("goto") edges after `CollapseStructure` whose **target** is a
   small epilogue block (statement-count ≤ threshold, no incoming side effects that would
   change semantics if duplicated, e.g. a `free(p); return;`);
2. **duplicate** that target's statements into the goto source (a CFG/`BlockGraph` mutation
   — clone the `BlockBasic`, its ops, and rewire edges), then drop the unstructured edge;
3. re-run / locally re-collapse the structuring so the duplicated tail folds into the arm
   as a structured early return;
4. preserve def/use and ordering (the duplicated `free`/`return` are real p-code ops, so
   this is *not* a pure print-time rewrite).

That is a **new pass type touching S8 structuring/region code beyond a single gated
early-return** — two independent triggers of Hard rule 7. The decider verdict (recorded in
`record.json`) is `scope: large`.

## Proposed implementation plan (multi-step — for the approved worker)

- **New module** `kuna_gotoreduce.rs` (S8) — a post-`CollapseStructure` `Action`,
  option-gated, default-OFF.
- **Anchor edits (minimal, `// (kuna)`):** register the action in `universalaction.rs` /
  the relevant `coreaction*` registration; option flag on the architecture struct
  (reset-default off); `options.rs`; `stages.toml` `settableTable` row.
- **Core mechanics:** add a `BlockGraph` helper to clone a `BlockCopy`/`BlockBasic` tail
  and rewire the floating edge (this is the genuinely new, risky piece — it mutates the
  structured block tree the verbatim engine produced).
- **Bounds / fail-safe:** cap duplicated statement count (e.g. ≤ 4 ops, single-exit,
  ends in `return`), bail on any block with multiple non-goto predecessors that would
  change semantics, mirroring loweredswitch's fail-safe cap.
- **Tests:** `tests/stages/ghangr-morton-my-message-callback-bfd2fa.xml` two-pass
  (off=goto present, on=goto gone); plus a synthetic minimal CFG fixture.

## Speed / risk assessment

- **Risk: high.** This mutates the structured block tree after a verbatim-ported engine.
  Incorrect duplication can change semantics (duplicating a block with side effects reached
  by more than the goto source) or desync def/use. The bounds above contain it but need
  careful adversarial testing across the datatest corpus.
- **Ablation risk:** likely non-zero — tail-duplication changes rendering on many
  functions that currently use a clean shared-epilogue goto, so this is expected to ship
  **default-OFF opt-in** even if correct.
- **Speed:** an extra post-structuring scan + occasional re-collapse; expected small but
  must be measured against the +5% budget on the target.
- **Same family** as the irreducible-loop SAILR gap (proposal PR #46) — worth coordinating
  so kuna grows one coherent goto-reduction layer rather than point fixes.

## Proposed option name

`gotoreduce` (`change_kind = structure-recovery`, `source_decompiler = angr`,
`inspiration = test_decompiling_morton_my_message_callback; SAILR/Phoenix goto-reduction (tail duplication); my_message_callback`).

## Recommendation

**Go** if the team wants kuna to start matching angr/SAILR on goto count (high-value,
broadly applicable, but real structuring risk) — staff it as a deliberate S8 effort with a
full adversarial test pass. **No-go / defer** if structuring-quality parity with angr is
out of scope right now; the gap is cosmetic (one goto, semantics already correct) and the
implementation cost/risk is disproportionate to this single function.
