# Decompiler Stages

The normative stage model for kuna, derived from a side-by-side study of Ghidra (the
decompiler kuna extracts), angr, and Reko. Full model with evidence, sub-stage catalogs,
and code anchors: **`docs/stage-model.md`**.

This replaces the earlier speculative 19-stage compiler-like list. A decompiler is not a
feed-forward pipeline: stages fire in order on the *first* pass, but information learned
later routinely rewinds or modifies earlier stages. The model is therefore **ordered
stages + explicit typed feedback edges**, with one orthogonal plane and one fixed-point
band.

## Stages

| | Stage | Artifact owned |
|---|---|---|
| P0 | Knowledge & Configuration Plane *(orthogonal — read by every stage on (re-)entry; survives every rewind)* | assertion store (Symbol DB + Override in Ghidra) + pipeline configuration |
| S1 | Image & Code Partition | bytes + segments + symbol/data map + code-vs-data partition |
| S2 | Flow & Op-Graph Recovery | lifted ops + CFG (`bblocks`) + jump tables (`jumpvec`) + work queues |
| S3 | Definition Web *(Band B)* | SSA-as-property over the op-graph; heritage state; dead-definition gate |
| S4 | Call & Prototype Model *(Band B)* | call specs / prototypes / calling conventions |
| S5 | Value & Type Facts *(Band B)* | ranges, consume bits, type lattice |
| S6 | Variable & Storage Model *(Band B)* | HighVariables, stack-frame layout, alias sets |
| S7 | Region Hierarchy | region tree (`sblocks` — physically distinct from the CFG) |
| S8 | Structured AST & Goto Quality | statement tree + goto set (the quality signal) |
| S9 | Surface Rendering & Refinement | text + position maps |

**Band B (S3–S6)** is a *mutual fixed-point band*: in Ghidra these four artifacts co-evolve
inside one repeating `mainloop` (no inter-stage boundary exists in the code); angr and Reko
run them as ordered passes with types as a terminal solver. Within-band order is a
per-decompiler scheduling regime, not part of the model.

## Sub-stages

A **sub-stage** is a named decision point inside a stage where a different choice — by
heuristic, human, or LLM agent — changes the stage's artifact and everything downstream.
Every sub-stage names the **assertion** that overrides it, with strength **HARD** (blocks
inference, e.g. typelock) or **HINT** (biases it), a **re-run scope**, and a **LATENT**
flag when the decision is hardcoded with no override today (the LATENT set is the kuna
roadmap). Full catalogs per stage: `docs/stage-model.md` §4–§10.

## Feedback mechanisms

Eight mechanism types (+2 grain annotations), each edge carrying trigger, rewind scope, and
surviving state:

- **(a)** local fixed-point (rule pools, simplifier loops)
- **(b)** staged re-entry (later edit re-runs an earlier analysis in the same run)
- **(c)** global restart with persisted hints (`setRestartPending` + Override; budgeted, gated)
- **(c′)** reduced-pipeline sub-query (`stageJumpTable`: a recursive sub-decompilation with
  the `jumptable` group set on a cloned function)
- **(d)** knowledge-store re-decompilation (the outer human/agent loop; parameterized by
  scope — full clear-and-replay vs scoped reflow)
- **(e)** speculate-validate-rollback (SAILR: edit → re-structure → measure gotos → accept
  or roll back; absent in Ghidra/Reko)
- **(h)** automatic fallback re-run (error → retry the whole function under a degraded
  pipeline variant; angr's preset fallback)
- **(f)** worklist re-enqueue (address-grain) and **(g)** lazy-dirty recompute
  (object-grain) as edge annotations

Edge table with anchors: `docs/stage-model.md` §11.

## Interventions

All interventions are durable typed assertions written to P0 and consulted on (re-)run —
never imperative mid-pipeline edits: `assert(stage, anchor, type, value, strength)`. An
LLM agent is just another assertion writer driving the feedback edges deliberately.
Symptom→sub-stage navigation table: `docs/stage-model.md` §13.

---

*Note: `STAGE_MAPPING.md` still maps source files to the **old** 19-stage numbering; the
remap to this model is a pending follow-up. The old list is preserved there for reference.*
