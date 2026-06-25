# [PROPOSAL] loopsubgraphreturn — return unstructurable loop subgraphs instead of goto-refining them

**Status:** draft proposal, awaiting human go/no-go. Do **not** implement until approved.
**Opportunity:** angr `test_decompiling_printenv_main` :: `main` (`printenv.o`, x86_64).
**Proposed option:** `loopsubgraphreturn` (S7 structuring, default-off opt-in).
**Scope (decider):** LARGE — touches S7 structuring/region code beyond a single gated early-return.

## The problem
angr's test asserts `main` structures with **zero gotos**. kuna emits **7 gotos**
(reproduced via the link-to-ELF bypass — `printenv.o` is ET_REL, see *Dependency* below).
The salient cluster:

```c
while (v2 = *v8, v2 != '\0') {
  ...
  if (v4 != v2) goto label_4011ef;     // continue-equivalent to inner for-loop tail
  ...
  goto label_4011ef;
  else if (v4 == '\0') goto label_4011ef;
  ...
}
label_4011ef:
```

These are `continue`-equivalent jumps to the tail of an inner `for` loop nested in an
outer `do/while`. kuna's structurer cannot collapse the inner subgraph and falls back to
unstructured goto edges.

## The angr reference fix
angr Phoenix structurer, `angr/analyses/decompiler/structuring/phoenix.py:221-226`:

```python
if not progressed:
    if self._region.cyclic_ancestor and not self._region.cyclic:
        # we prefer directly returning this subgraph in case it can be further
        # restructured within a loop region
        break                                     # <- return the subgraph
    # otherwise:
    removed_edge = self._last_resort_refinement(...)   # <- deletes a CFG edge => goto
```

Mechanism: an **acyclic** subgraph with a **cyclic ancestor** that made no structuring
progress is *returned to its enclosing loop region* rather than forced through
`_last_resort_refinement()` (which deletes an edge and materializes it as a `goto`). The
outer loop region then restructures it with full loop context (producing
`continue`/structured exits).

## Why this is LARGE (not one Action/Rule)
kuna's structurer is **Ghidra's `blockaction`** — `ActionBlockStructure` /
`CollapseStructure` / `TraceDAG` (`blockaction.cc:2170`, Rust in
`decompiler/crates/kuna-decomp/src/substrate/block.rs` + S7 region code). It is a
*structurally different* algorithm from angr's Phoenix/SAILR:
- no `cyclic_ancestor` / `_last_resort_refinement` concept;
- goto-edge selection happens inside the TraceDAG collapse engine, not a discrete pass.

Replicating "return the loop subgraph instead of goto-refining it" therefore requires
modifying kuna's **core S7 collapse engine** (TraceDAG/CollapseStructure goto-edge
selection): detect a sub-DAG with a cyclic ancestor that cannot be collapsed, defer its
goto materialization, and let the enclosing loop collapse run first. This cannot be a
gated early-return on a single new `Action`. It is multi-anchor S7 surgery → Hard rule 7
LARGE.

## Proposed implementation plan (for the approving human)
1. **Loader prerequisite.** Land proposal #37 (`elf-reloc-objects`) first, so the full
   pipeline can load `printenv.o` and reproduce end-to-end. Until then this proposal is
   validatable only via the link-to-ELF bypass.
2. **Identify the deferral point** in kuna's TraceDAG/CollapseStructure where an
   uncollapsible acyclic sub-DAG currently gets a forced goto edge (the analog of angr's
   `not progressed → _last_resort_refinement`).
3. **Add a cyclic-ancestor check**: if the stuck sub-DAG sits inside a not-yet-collapsed
   loop region, defer (return) it rather than emitting the goto, and re-run the loop
   collapse so loop context is available.
4. **Gate behind `loopsubgraphreturn`** (new architecture flag, default-off), with an
   early-return to byte-identical default output. New element id `4020+` (next free after
   4019).
5. **Tests:** a `tests/stages/ghangr-loopsubgraphreturn.xml` two-pass datatest (bytechunk
   of the inner-loop subgraph) asserting the goto cluster off-default and its
   absence/`continue` on-default; ablation over the 675-assertion corpus; speed measurement.

## Speed / risk assessment
- **Risk: high.** Changes to the core collapse engine can perturb structuring for *many*
  functions; the ablation risk over 675 assertions is the main reason for opt-in default-off
  and for a human go/no-go before spending an implementation worker.
- **Speed:** an extra deferral + re-collapse pass per stuck loop subgraph is bounded by the
  region count; expected within the +5% budget but must be measured. Stays opt-in regardless
  until measured.

## Dependency
- **Blocked-on #37** for full-pipeline reproduction (`printenv.o` is ET_REL; kuna's
  PT_LOAD-only loader returns `Unable to load 512 bytes at r0x004007a0`). The decompiler
  gap itself is real and reproduced via link-to-ELF (`docs/features/printenv-main-ec8020/angr-vs-kuna.txt`).

## Distinctness
- Distinct from **#37** (`elf-reloc-objects`, the loader gap — a prerequisite, not this fix).
- Distinct from **#39** (`condfold` — SAILR condition-folding / crossing-edge goto
  reduction). This is a *different* SAILR/Phoenix mechanism: loop-subgraph return vs
  last-resort edge deletion.
