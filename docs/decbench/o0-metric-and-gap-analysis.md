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

## The gap is conditional-value materialization — but the fix is NARROW (Joern node-count study)

Studying the top kuna-loses-to-angr/ida cases (e.g. coreutils `ptx::output_one_dumb_line`: angr **22** CFG
nodes vs kuna **48**), the gap is **how a conditional value is emitted** — kuna materializes an if/else
diamond assigning a temp (`if(c)v1=A;else v1=B; f(v1)`), angr inlines/folds it (`f(c?A:B)`). But a careful
Joern node-count study shows the GED-movable part of this is **narrow**:

| form | Joern CFG nodes |
|---|---|
| if/else materialized (`if(c)v=A;else v=B; f(v)`) | **4** |
| **simple-arm** ternary `v = c ? a : b` (statement) OR `f(c?a:b)` (inline) | **1** |
| LOAD-arm ternary `f(c ? *p : 0)` (inline) | 3 |
| INT-arm ternary `f(c ? b+5 : b-3)` (inline OR statement) | **4** |

**Joern only collapses a *simple-arm* ternary (variables/constants) to 1 node.** Computed-arm ternaries
(`*p`, `b+5`) stay 3-4 nodes whether written inline or as a statement — Joern re-expands their sub-expression
operators into control-flow. So:

- The **only GED-positive ternary case is a simple-arm diamond (4→1)** — and kuna's shipped `iteregion`
  (print-only statement form) **already captures it** when it matches.
- **Inlining computed-arm ternaries is NOT the lever** — it saves 0-1 nodes/diamond. `iteexpr` (PR #161,
  computed arms) and a would-be inline pass are both essentially **GED-neutral** (they are readability wins).
**EMPIRICAL CONFIRMATION (a valid ablation):** re-decompiling coreutils O0 with `iteexpr` ON (forced
correctly — `DECBENCH_DECOMPILERS=kuna` + `KUNA_SPECS`, verified the `.c` gained ternaries) and re-scoring
gives **0 GED change across all 1477 functions** (0 improved / 0 regressed / ΣΔ = 0). So **the entire ternary
line — `iteexpr`, computed-arm recovery, and any would-be inline pass — is a GED dead end.** Converting
kuna's if/else diamonds to ternaries does not move the metric, which means **the diamonds are NOT the
angr/ida gap** (contrary to an earlier draft of this doc that framed conditional-value inlining as "the
lever" — that framing is **refuted**). `iteexpr` (PR #161) is correctly a readability-only, default-off
feature.

**So the true cause of the O0 node-count gap vs ida/angr (e.g. `output_one_dumb_line` 48 vs 22) is still
UNIDENTIFIED and is NOT ternaries.** It is some other basic-block-count difference (candidates to
investigate next, all block-count movers per the metric's nature: empty/forwarding-block retention,
sequential-block non-merging, condition-tree shape, or a Joern parsing sensitivity to kuna's `undefinedN`
types / casts that the minimal-snippet tests don't capture). This needs a direct CFG-node diff (dump both
Joern CFGs and see which blocks differ), not another syntactic pass. Every syntactic transform tried so far
— ternaries, gotos, branchflip, empty-switch — is GED-neutral.

**Process note (this cost real time):** several ablations were **false-nulls** — `run_benchmark` silently
resumed/skipped the re-decompile (see gotchas below), so the `.c` never changed and the GED "0" was
meaningless. The result above is the first ablation where the output `.c` was verified to actually change.
Always confirm that before trusting a "no change" ablation.

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
