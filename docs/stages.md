# Decompiler Stages

The kuna stage model: **ordered stages + explicit typed feedback edges**, with one
orthogonal plane and one fixed-point band. This is the model kuna is organized around —
the runtime registry (`decompiler/cpp/kuna_stages.cc`, queryable via the `stage
list`/`stage map`/`stage catalog` console commands), the per-file source mapping
(`docs/history/stage-mapping.md`), the assertion catalog (`docs/assertions.md`), and the issue
testcases (`tests/stages/`) all speak it.

Full normative model with evidence, sub-stage catalogs, and code anchors:
**`docs/history/stage-model.md`** (derived 2026-06 from a side-by-side study of Ghidra, angr,
and Reko; the rationale for rejecting a linear compiler-style pipeline is its §1).

A decompiler is not a feed-forward pipeline: stages fire in order on the *first* pass,
but information learned later routinely rewinds or modifies earlier stages.

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

## Source layout (the stages on disk)

The decompiler source is **physically organized by stage**: every module file under
`decompiler/crates/kuna-decomp/src/` lives in a stage-named folder (the folder name keeps the
canonical `Sx` code so it greps against this doc and the registry, and adds a plain word so the
tree reads to a newcomer). Module *names* stay flat (`kuna_decomp::flow`) via re-exports in
`lib.rs`, so the layout is documentation, not an API change. The per-file assignment is
`docs/history/stage-mapping.md`; the live group→stage registry is `stages.toml`.

| Stage | Folder | Reads as |
|---|---|---|
| — (substrate) | `substrate/` | shared IR & containers used by every stage (`varnode`/`op`/`block`/`funcdata*`, `dtype`, rewrite helpers) |
| P0 | `p0_knowledge/` | knowledge & configuration plane (symbol DB, options, overrides, the stage registry) |
| S1 | `s1_partition/` | image & code partition (architecture/loader binding) |
| S2 | `s2_lift/` | flow & op-graph recovery (lift, CFG, jump tables, p-code injection) |
| S3 | `s3_dataflow/` | definition web (SSA/heritage + the simplification rule pools) |
| S4 | `s4_calls/` | call & prototype model |
| S5 | `s5_types/` | value & type facts (type system + inference) |
| S6 | `s6_variables/` | variable & storage model (HighVariables, merge, stack layout) |
| S7 | `s7_regions/` | region hierarchy (the angr RegionIdentifier port) |
| S8 | `s8_structure/` | structured AST & goto quality (the structuring engine, `blockaction`) |
| S9 | `s9_emit/` | surface rendering & refinement (PrintC, casts, strings, naming) |
| Infra | `infra/` | orchestration & framework (the schedule, the Action/Rule engine) |

The **program-prep analyses** that sit *outside* this stage model (the loader/analyzer tier
Ghidra runs as "Run Analysis": PLT/GOT markup, strings, DWARF, demangling, function-start
discovery, …) live in their own crate, `kuna-analysis` (P0/S1), organized by the same scheme
(`s1_loader/`, `s1_strings/`, …). See `docs/missing-analyses.md`.

## Sub-stages

A **sub-stage** is a named decision point inside a stage where a different choice — by
heuristic, human, or LLM agent — changes the stage's artifact and everything downstream.
Every sub-stage names the **assertion** that overrides it, with strength **HARD** (blocks
inference, e.g. typelock) or **HINT** (biases it), a **re-run scope**, and a **LATENT**
flag when the decision is hardcoded with no override today (the LATENT set is the kuna
roadmap). Full catalogs per stage: `docs/history/stage-model.md` §4–§10; the machine-readable,
flippable subset is `docs/assertions.md` (`kuna catalog --json`).

## Feedback mechanisms

Eight mechanism types (+2 grain annotations), each edge carrying trigger, rewind scope, and
surviving state:

- **(a)** local fixed-point (rule pools, simplifier loops)
- **(b)** staged re-entry (later edit re-runs an earlier analysis in the same run)
- **(c)** global restart with persisted hints (`setRestartPending` + Override; budgeted, gated)
  — *observable in kuna via the `restarts` console command*
- **(c′)** reduced-pipeline sub-query (`stageJumpTable`: a recursive sub-decompilation with
  the `jumptable` group set on a cloned function) — *drivable in kuna via the `pipeline`
  console command*
- **(d)** knowledge-store re-decompilation (the outer human/agent loop; parameterized by
  scope — full clear-and-replay vs scoped reflow)
- **(e)** speculate-validate-rollback (SAILR: edit → re-structure → measure gotos → accept
  or roll back; absent in Ghidra/Reko — kuna's `quality` command is the measurement half)
- **(h)** automatic fallback re-run (error → retry the whole function under a degraded
  pipeline variant; angr's preset fallback)
- **(f)** worklist re-enqueue (address-grain) and **(g)** lazy-dirty recompute
  (object-grain) as edge annotations

Edge table with anchors: `docs/history/stage-model.md` §11.

## Interventions

All interventions are durable typed assertions written to P0 and consulted on (re-)run —
never imperative mid-pipeline edits: `assert(stage, anchor, type, value, strength)`. An
LLM agent is just another assertion writer driving the feedback edges deliberately.
Symptom→sub-stage navigation table: `docs/history/stage-model.md` §13 (caveat from practice: a
symptom's stage is not always its decision's stage — wrong-looking constants in the
rendered C are usually destroyed in Band B, not mis-rendered at S9).

In kuna this is concrete: `kassert <stage> <substage> <args> [hard|hint]` at the console
(reports the computed minimal rewind scope), `--option NAME VALUE` / `--kassert "<args>"`
on `kuna decompile`, and discovery via `kuna catalog`.

## Implementation state

The model is physical in kuna:

- **Registry & console**: every action/rule group, console surface, and settable
  assertion is stage-addressed in `kuna_stages.cc`; `stage list/map/status/catalog`,
  `kassert`, `pipeline`, `quality`, `restarts` operate it. The S7 region tree is
  directly observable via `region tree/blocks/walk` (the angr RegionIdentifier
  port — `docs/regions.md`).
- **Issues fixed through the model** (each pinned by a `tests/stages/` testcase
  asserting both directions of the decision): GH-558, 2786, 8471, 6930, 6990, 1282,
  7190, 8817, 8913, 9230, 1537. Per-stage changelog: `docs/history/stage-implementation.md`.
- **Defaults**: eight sub-stage fixes are kuna defaults (DIV-2); the destructive ones
  stay opt-in per the ablation evidence. Record: `docs/divergences.md`.
- **Empirical critique** of the model against those fixes (stage-fit, ablations,
  navigation score, per-stage verdicts): `docs/history/stage-critique.md`.
