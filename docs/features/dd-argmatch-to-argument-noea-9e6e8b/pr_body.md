## dd-argmatch-to-argument-noea-9e6e8b: SAILR goto-reduction by return-tail duplication (`option gotoreduce`)

Closes the one structural gap on angr testcase
`test_decompiling_dd_argmatch_to_argument_noeagerreturns::argmatch_to_argument`
(coreutils `dd`, x86-64 @ `0x40a640`). See
[`docs/features/dd-argmatch-to-argument-noea-9e6e8b/analysis.md`](docs/features/dd-argmatch-to-argument-noea-9e6e8b/analysis.md)
for the full side-by-side.

### Why angr was better

The only difference was a single residual `goto label_40a6a3;` + `label_40a6a3:`. A shared
return tail (`v2 = 0;` falling through to the single `return v2;`) has **two** predecessors —
the entry `if (v2 == 0)` true-arm and an in-loop `if (v2 == 0)` edge — so Ghidra's
`CollapseStructure` (the verbatim port of `CollapseStructure`/`TraceDAG`/`BlockTrace`) must
express the second edge as an unstructured `goto`. It has no schema that duplicates a tail to
avoid a goto. angr (SAILR/Phoenix `ReturnDuplicator`) emits **0** gotos by duplicating the
constant return tail into the in-loop edge.

### Mechanism

New S8 pass `kuna_gotoreduce.rs` (`ActionGotoReduce`), registered after `ActionFinalStructure`.
It finds every `BlockIf` that renders as `if (cond) goto T` whose target `T` is a small
single-successor basic-block chain ending in `return` (bounded: ≤3 blocks / ≤8 ops, no
`CALL`/`STORE`), and rewrites it to `if (cond) { <tail> }` by minting fresh `BlockCopy` leaves
over the **same** underlying basic blocks (a print-tree duplication — the C printer re-emits
their ops; `CPUI_RETURN` carries the `returns` flag, not `branch`, so it survives the if-body
`NO_BRANCH`). **No p-code is cloned and SSA/def-use is untouched.** When the converted goto was
the only unstructured edge into `T`, `T`'s label is cleared. The only genuine graph surgery is
one `(kuna)` method `BlockGraph::kuna_inline_return_tail` in `block.rs`; everything else is the
new module plus the standard option anchors.

### The option

`option gotoreduce on|off` (architecture/seam flag `reduce_return_gotos`, ElementId 4100),
**default-OFF opt-in** — flip per decompilation:

```
kuna decompile ./dd argmatch_to_argument --option gotoreduce on
```

### Ablation / parity / speed

- **Ablation: clean — 0/675** upstream datatest assertions change with the feature default-ON.
- **Speed:** within the +5% budget on the target (`off≈148.7ms`, `on≈105.1ms`, Δ≈-29% — the
  extra post-structuring scan is negligible; the spread is measurement noise).
- **Default decision:** kept **default-OFF opt-in** despite the clean ablation. This is the
  first S8 structured-tree mutation on the verbatim-ported collapse engine and is rated
  high-risk in the approved proposal, so it ships opt-in pending broader validation (no DIV
  entry; default output stays byte-identical). A reviewer can flip it default-ON later given
  the clean ablation.
- **Gates:** `kuna catalog --check` OK · datatest parity **PARITY OK** (675/675) · stage suite
  **PARITY OK** (162/162, incl. the new `tests/stages/ghangr-dd-argmatch-to-argument-noea-9e6e8b.xml`).

This is the same SAILR goto-reduction family as the parked `morton`/`newburry`/`tr-build`
proposals; `dd::argmatch_to_argument` is the minimal `return <const>` variant of that transform.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
