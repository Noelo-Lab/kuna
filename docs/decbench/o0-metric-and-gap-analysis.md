# decbench O0: the GED metric's blind spot, and the real angr/ida gap

A fair O0 re-measurement (2026-07-11) of the current engine against all six decbench decompilers, plus a
direct study of what the GED metric actually rewards. The headline: **kuna already beats Ghidra on O0**, and
**the entire remaining gap vs ida/angr is one thing — conditional-value inlining — because the GED metric is
blind to almost everything else.**

## The metric's blind spot (the most important finding)

decbench GED is graph-edit-distance between the **Joern CFG** of the source and of the decompiled C. Joern's
CFG is a **basic-block** graph. That makes it **blind to syntactic form**:

| Transform | GED effect | Verified |
|---|---|---|
| `v = c ? A : B` vs `if(c)v=A;else v=B` | **none** | identical CFG (4 nodes / 4 edges) |
| `goto loop_head` vs `break`/`continue` | **none** | same back-edge |
| `branchflip` (negated-guard flip, arm swap) | **none** | node count unchanged (151→151) |
| per-function empty `switch()` (invalid C) | drops only that function | pyjoern is per-function resilient (recovered 1239 fns from ssh-keygen with 12 OR 2 empty switches) |

**Consequence:** every "readability" win — ternary recovery, goto reduction, branch flipping, and (per
function) invalid-C emission — is a **dead end for GED**. Only the **basic-block count / branch structure**
moves the metric. This is why the campaign's earlier structural wins (noreturn pruning, S6 variable merge)
moved kuna from last place to beating Ghidra, while syntactic passes did not.

## Fair O0 standing (coreutils, all 6 re-scored post-#6)

| kuna vs | kuna-wins | tie | kuna-loses | notes |
|---|---|---|---|---|
| ghidra | 431 | 781 | **260** | kuna wins ~1.7× more than it loses |
| ida    | 426 | 391 | **659** | the real target |
| angr   | 426 | 406 | **623** | the real target |
| phoenix| 497 | 388 | 570 | |

kuna **beats its ancestor Ghidra** on O0 (the primary goal is largely met). The remaining gap is to **ida
and angr**.

## The real gap: conditional-value inlining vs materialization

Studying the top kuna-loses-to-angr/ida cases (e.g. coreutils `ptx::output_one_dumb_line`: angr **22** CFG
nodes vs kuna **48**), the gap is almost entirely **how a conditional value is emitted**:

- **kuna materializes it** as an if/else diamond assigning a temp, then uses the temp:
  ```c
  if (c) v1 = A; else v1 = B;   // 4-node diamond in the CFG
  f(... v1 ...);
  ```
- **angr inlines it** into the use expression:
  ```c
  f(... c ? A : B ...);          // 1 node
  ```

Verified directly: `g(c ? a : b)` → **1 CFG node**; `if(c)v=a;else v=b; g(v)` → **4 nodes**. So inlining a
conditional value collapses a 4-node diamond to 1 node — a real GED reduction. angr's `ITERegionConverter`
**modifies the IR** to collapse the region into an expression; kuna's `iteregion` (and the new `iteexpr`,
PR #161) are **print-only** — they render `v1 = c ? A : B;` as a *statement* and never inline it, so the
diamond's 4 blocks survive in the printed CFG. For a **multi-use** conditional, angr duplicates the ternary
at each use where kuna materializes once.

**This is the one lever that moves O0 GED vs angr/ida.** It is a substantial printer/dataflow change
(the value's def is a downstream control-flow diamond, so the printer must render the ternary at the use
site and suppress the diamond; or the S8 pass must collapse the region in the IR like angr). It is
source-dependent (inlining diverges when the source materialized), so it belongs behind an option +
ablation, likely a `--mode aggressive` member rather than a default flip.

Secondary, lower-value: jump-table recovery failures (e.g. `get_funky_string`'s CET `notrack jmp` table) —
rare (~10 coreutils O0 fns) and **shared with Ghidra** (kuna inherits Ghidra's jumptable models), so kuna
does not lose to Ghidra there.

## Measurement gotchas (for reproducing this)

- **`DECBENCH_REDO_DECOMPILERS=kuna` is a no-op unless `DECBENCH_DECOMPILERS` also lists kuna** —
  `run_benchmark`'s `to_run` iterates `DECOMPILERS` (default `angr,ghidra`). Use
  `DECBENCH_DECOMPILERS=kuna DECBENCH_REDO_DECOMPILERS=kuna` to actually force a kuna re-decompile.
- run_benchmark **resumes** a project as "complete" from `checkpoints/<project>.pkl`; deleting the `.c`
  alone does not force a re-decompile, and the `.c` are not auto-regenerated.
- A scratchpad-copied `kuna` binary needs **`KUNA_SPECS`** exported to find its `.sla`, or it emits empty
  output under run_benchmark.
- Fair scoring requires purging **all** `reeval_ged/*<project>*.json` (not just kuna's), or the other
  decompilers keep stale pre-#6 scores (the "kuna-missing" artifact, mostly single-block/degenerate fns).
- **Always confirm the output `.c` actually changed before trusting an ablation** — several were false-nulls.
