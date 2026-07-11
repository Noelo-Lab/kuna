# decbench O0: the GED metric's blind spot, and the real angr/ida gap

A fair O0 re-measurement (2026-07-11) of the current engine against all six decbench decompilers, plus a
direct study of what the GED metric actually rewards. The headline, in three parts (each measured):
**(1)** kuna already **beats Ghidra** on O0 and is **≈ tied with angr in aggregate GED** (Σ 40,468 vs
40,901) — the per-function "loses to angr 624×" count is misleading; **(2)** the GED metric is **blind to
syntactic form** (ternaries/gotos/branchflip are all node-count-neutral); **(3)** the one *identified*
real lever is a narrow structural case — **reused-stack-local conditional-value diamonds**
(`if(c) v=a; else v=b;` with `v` a shared stack slot → STORE arms) that kuna can't fold and angr collapses
to source-perfect — worth **32 functions / 721 GED (12% of the angr loss)**, a `[PROPOSAL]`-class fix, not
the pure-arm ternary line (which is correctly GED-neutral). Sections below give the evidence and the exact
block-level diff.

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
| angr   | 426 | 406 | **623** | see the aggregate caveat below |
| phoenix| 497 | 388 | 570 | |

**Per-function win/loss overstates the angr gap — in *aggregate GED* kuna ≈ angr.** On the
2,014 O0-coreutils functions scored by both (a later re-count), the head-to-head is kuna-better
453 / tie 937 / kuna-worse 624, yet **Σ kuna GED = 40,468 vs Σ angr GED = 40,901** — kuna is
*marginally ahead in total edit distance*. kuna's losses are many-but-small; its wins are
fewer-but-large (≈9.5 GED/loss vs ≈14 GED/win). So "kuna loses to angr" is a **count** artifact;
by the metric's own summed magnitude the two are a wash, and kuna clearly beats ghidra
(522 better / 321 worse) while still losing to ida (424 / 681).

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

### The gap IS identified now (direct CFG-node diff) — reused-stack-local conditional-value diamonds

A **direct Joern block-level diff** of `output_one_dumb_line` (kuna 48 nodes vs angr 22, dumping each
block's statements) finally pins the cause, and it revises the paragraph above:

- **The source CFG is 22 nodes / 31 edges — and angr reproduces it EXACTLY (GED = 0.0, perfect).**
  kuna's 48-node form is **GED = 98.0** from source. So this is *not* a form-blind wash; it is a real,
  large, measured loss on this function.
- kuna emits **six repeated 3-block diamonds** `if (c) v1 = <global>; else v1 = 0;` where a single local
  `v1` is **reused across all six**. angr promotes `v1` to an SSA value and folds/inlines each into a
  single block (`v1 = (!g?0:g_838)` / inlined into the use).
- **Why `iteexpr` does NOT fold these** (verified: iteexpr-ON leaves the function at 48 nodes, unchanged):
  the diamonds render as clean single-statement arms, but because `v1` is a **reused stack local**, each
  arm assignment is a **rejected op** — a `STORE` to `v1`'s slot (and the shared merge brings
  `MULTIEQUAL`/`INDIRECT` bookkeeping), which `single_assign_arm` correctly refuses (a `STORE` can't be a
  ternary branch expression). `iteexpr` only ever reaches *pure-value*-arm computed ternaries, which is
  exactly why it (and the whole computed-arm line) measured GED-neutral — that ablation was **right**, it
  just wasn't touching *this* population.

So the earlier "gap UNIDENTIFIED / diamonds are not it" wording is now superseded: the gap **is**
conditional-value diamonds, but specifically the **reused-stack-local (STORE-arm)** subclass — a
*structural* case that neither `iteregion` nor `iteexpr` can reach today.

**Scale (O0 coreutils, the angr-near-perfect / kuna-badly-off bucket, `angr GED ≤ 1 ∧ kuna GED ≥ 10`):**
**32 functions, 721 GED = 12% of the total kuna-vs-angr loss.** But the bucket is **heterogeneous**, and
the STORE-arm diamond is only *one* cause in it — dedup by distinct source function (many are the same
function compiled into sibling coreutils binaries):

| distinct fn | GED×count | share | over-structuring cause (inspected) |
|---|---|---|---|
| `filename_unescape` | 28 × 9 = **252** | 35% | nested **if/else-if comparison tree** on char values + `branchflip` — *not* value diamonds |
| `bsd_split_3` | 17 × 9 = **153** | 21% | **loop + guard structuring** (`do-while`/`for`/guard-`if`) — *not* value diamonds |
| `output_one_dumb_line` | 98 × 1 = **98** | 14% | **the reused-stack-local STORE-arm diamond** (this section) |
| `sort_files` | 13 × 3 = 39 | 5% | (unverified) |
| ~10 others | 10–45 × 1 | ~25% | mixed |

So the STORE-arm fold below addresses **only `output_one_dumb_line` (98 GED ≈ 1.7% of the total angr
loss)** — a **single-function outlier**, not the 32-function lever. The bucket's real bulk (56%,
`filename_unescape` + `bsd_split_3`) is **different, deeper structuring gaps** (comparison-tree flattening,
loop/guard structuring) that need their own per-function root-cause. **Verdict: do NOT build the STORE-arm
fold as a feature** — the ROI (one function) does not justify a default-changing IR/printer pass with
regression surface. The heterogeneous bulk is the honest next target, one distinct function at a time.

**The STORE-arm fold, for the record (a 1-function `[PROPOSAL]`, not recommended standalone):**
- *Partial:* extend the ITE matcher to accept **two STOREs to the same slot** → `*s = c ? A : B`
  (48 → ~30 nodes). Contained but a real IR/printer change; must prove storage-equivalence.
- *Full (GED 0):* stack-local → register/SSA promotion + inline into the single use (48 → 22) — Ghidra-style
  stack-var promotion that isn't firing here; deep dataflow. Only worth it if the promotion *also* helps the
  comparison-tree/loop-guard bulk, which is unverified.

Secondary syntactic transforms (ternaries on *pure* arms, gotos, branchflip, empty-switch) remain
GED-neutral — the metric moves only on the **block count**.

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
