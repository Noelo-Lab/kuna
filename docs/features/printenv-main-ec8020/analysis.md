# printenv-main-ec8020 — analysis

## Opportunity
- angr testcase: `test_decompiling_printenv_main` (`angr/tests/analyses/decompiler/test_decompiler.py:2347`)
- Binary: `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/printenv.o`
- Function: `main` (x86_64), angr addr `0x4007a0`.
- angr assertion: **`"goto " not in d.codegen.text`** — the function must structure with **zero gotos**.

## What angr does better (the construct)
The angr test docstring states the mechanism precisely:

> when a subgraph inside a loop cannot be structured, instead of entering last-resort
> refinement, we should return the subgraph and let structuring resume with the knowledge
> of the loop. otherwise, in this function, we will see a goto while in reality we do not
> need any gotos.

The fix is in angr's **Phoenix structurer** (`angr/analyses/decompiler/structuring/phoenix.py:221-226`):

```python
if not progressed:
    if self._region.cyclic_ancestor and not self._region.cyclic:
        # we prefer directly returning this subgraph in case it can be further
        # restructured within a loop region
        l.debug("No progress is made on this acyclic graph with a cyclic ancestor. Give up.")
        break
    l.debug("No progress is made. Enter last resort refinement.")
    removed_edge = self._last_resort_refinement(...)   # <- this is what injects gotos
```

i.e. an **acyclic** subgraph that has a **cyclic ancestor** but made no structuring
progress is *returned to its enclosing loop region* rather than forced through
`_last_resort_refinement()` (which deletes a CFG edge and materializes it as a `goto`).
The outer loop region then restructures it with full loop context, producing
`continue`/structured exits instead of gotos.

## Owning stage
- **S7 structuring** (`docs/stage-mapping.md`: Stage 16 → S7/S8). kuna's structurer is
  Ghidra's `blockaction` — `ActionBlockStructure` / `CollapseStructure` / `TraceDAG`
  (`blockaction.cc:2170`), implemented in `decompiler/crates/kuna-decomp/src/substrate/block.rs`
  + the S7 region code. Goto selection (S8) lives in the same collapse engine.

## kuna's current output (the bug)
`printenv.o` is **ET_REL** (`readelf -h` → `Type: REL`). kuna's loader maps PT_LOAD
segments only, so it cannot load the object at all (`Unable to load 512 bytes at
r0x004007a0`) — the [ET_REL loader gap], tracked separately as proposal PR #37. To
characterise the *decompiler* gap I linked the `.o` into a standalone non-PIE ELF
(stub every UND symbol, `-no-pie -nostdlib -static`) and ran
`kuna decompile ./synth main`. Result: **7 gotos** (see `angr-vs-kuna.txt`):

```
if (v4 != v2) goto label_4011ef;
goto label_4011ef;
else if (v4 == '\0') goto label_4011ef;
goto label_4012d0;
if (*(char **)&v10[0x10] == (char *)0x0) goto label_4012e9;
goto label_401393;
if ((v6 != ...) && (v5 = strncmp(...), v5 != 0)) goto label_401430;
```

The `goto label_4011ef` cluster is exactly the angr-described case: jumps to the tail of
an inner `for` loop body (a `continue`-equivalent), inside an outer `do/while`. kuna's
`blockaction` collapse cannot structure that inner subgraph and falls back to unstructured
goto edges.

## Hypothesis / why this is NOT a single-pass option
1. **Loader-blocked.** Full-pipeline reproduction is impossible until the ET_REL loader
   gap (proposal #37) lands; the gap is reproducible only via the link-to-ELF bypass.
2. **Algorithm mismatch.** angr's fix is a change to its *Phoenix iterative region
   collapse* recursion (cyclic-ancestor awareness). kuna uses Ghidra's
   `blockaction`/`TraceDAG` — a structurally different algorithm with no `cyclic_ancestor`
   / `_last_resort_refinement` concept. Reducing these gotos requires modifying kuna's
   **core S7 structuring engine** (`CollapseStructure`/`TraceDAG` goto-edge selection),
   not an option-gated `Action`/`Rule` with a gated early-return.

Per Hard rule 7 this is a **LARGE** feature (touches S7 structuring/region code beyond a
single gated early-return; cannot be modelled as one Action/Rule like `kuna_loweredswitch.rs`).
The formal scope/proposal-vs-negative decision is recorded in `record.json` `decisions`.
