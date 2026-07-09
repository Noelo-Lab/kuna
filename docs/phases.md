# Decompiler Phases

> **The phase model at a glance.** The normative, live description of what each phase
> actually computes — algorithms, thresholds, failure modes, provenance — is the chaptered
> spec under **`docs/spec/`** (start at `docs/spec/00-overview.md`). This page is the
> one-screen map; the original 2026-06 derivation study (Ghidra/angr/Reko) is archived at
> `docs/history/stage-model.md`.

The kuna phase model: **ordered phases + explicit typed feedback edges**, with one
orthogonal plane and one fixed-point band. This is the model kuna is organized around —
the runtime registry (queryable via the `phase list`/`phase map`/`phase catalog` console
commands; `stage ...` still works as a deprecated alias), the per-file source mapping
(`docs/history/stage-mapping.md`), the option catalog (`docs/options.md`), and the issue
testcases (`tests/stages/`) all speak it.

A decompiler is not a feed-forward pipeline: phases fire in order on the *first* pass, but
information learned later routinely rewinds or modifies earlier phases (the feedback-edge
model; `docs/spec/00-overview.md` §0.7 has the live edge inventory).

## Phases

| | Phase | Artifact owned |
|---|---|---|
| P0 | Knowledge & Configuration Plane *(orthogonal — read by every phase on (re-)entry; survives every rewind)* | assertion store (Symbol DB + Override in Ghidra) + pipeline configuration |
| P1 | Image & Code Partition | bytes + segments + symbol/data map + code-vs-data partition |
| P2 | Flow & Op-Graph Recovery | lifted ops + CFG (`bblocks`) + jump tables (`jumpvec`) + work queues |
| P3 | Definition Web *(Band B)* | SSA-as-property over the op-graph; heritage state; dead-definition gate |
| P4 | Call & Prototype Model *(Band B)* | call specs / prototypes / calling conventions |
| P5 | Value & Type Facts *(Band B)* | ranges, consume bits, type lattice |
| P6 | Variable & Storage Model *(Band B)* | HighVariables, stack-frame layout, alias sets |
| P7 | Region Hierarchy | region tree (`sblocks` — physically distinct from the CFG) |
| P8 | Structured AST & Goto Quality | statement tree + goto set (the quality signal) |
| P9 | Surface Rendering & Refinement | text + position maps |

**Band B (P3–P6)** is a *mutual fixed-point band*: in Ghidra these four artifacts co-evolve
inside one repeating `mainloop` (no inter-phase boundary exists in the code); angr and Reko
run them as ordered passes with types as a terminal solver. Within-band order is a
per-decompiler scheduling regime, not part of the model.

## Source layout (the phases on disk)

The decompiler source is **physically organized by phase**: every module file under
`decompiler/crates/kuna-decomp/src/` lives in a phase-named folder (the folder name carries the phase code `pN` so it greps against this doc and the registry, plus a plain word so the tree reads to a newcomer). Module *names* stay flat (`kuna_decomp::flow`) via re-exports in
`lib.rs`, so the layout is documentation, not an API change. The per-file assignment is
`docs/history/stage-mapping.md`; the live group→phase registry is `phases.toml`.

| Phase | Folder | Reads as |
|---|---|---|
| — (substrate) | `substrate/` | shared IR & containers used by every phase (`varnode`/`op`/`block`/`funcdata*`, `dtype`, rewrite helpers) |
| P0 | `p0_knowledge/` | knowledge & configuration plane (symbol DB, options, overrides, the phase registry) |
| P1 | `p1_partition/` | image & code partition (architecture/loader binding) |
| P2 | `p2_lift/` | flow & op-graph recovery (lift, CFG, jump tables, p-code injection) |
| P3 | `p3_dataflow/` | definition web (SSA/heritage + the simplification rule pools) |
| P4 | `p4_calls/` | call & prototype model |
| P5 | `p5_types/` | value & type facts (type system + inference) |
| P6 | `p6_variables/` | variable & storage model (HighVariables, merge, stack layout) |
| P7 | `p7_regions/` | region hierarchy (the angr RegionIdentifier port) |
| P8 | `p8_structure/` | structured AST & goto quality (the structuring engine, `blockaction`) |
| P9 | `p9_emit/` | surface rendering & refinement (PrintC, casts, strings, naming) |
| Infra | `infra/` | orchestration & framework (the schedule, the Action/Rule engine) |

The **program-prep analyses** that sit *outside* this phase model (the loader/analyzer tier
Ghidra runs as "Run Analysis": PLT/GOT markup, strings, DWARF, demangling, function-start
discovery, …) live in their own crate, `kuna-analysis` (P0/P1), organized as `loader/`
(image-format markup) + `analyzers/` (one module per pass). See `docs/missing-analyses.md`.

## Sub-phases

A **sub-phase** is a named decision point inside a phase where a different choice — by
heuristic, human, or LLM agent — changes the phase's artifact and everything downstream.
Every sub-phase names the **assertion** that overrides it, with strength **HARD** (blocks
inference, e.g. typelock) or **HINT** (biases it), a **re-run scope**, and a **LATENT**
flag when the decision is hardcoded with no override today (the LATENT set is the kuna
roadmap). Full catalogs per phase: `docs/history/stage-model.md` §4–§10; the machine-readable,
flippable subset is `docs/options.md` (`kuna catalog --json`).

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
never imperative mid-pipeline edits: `assert(phase, anchor, type, value, strength)`. An
LLM agent is just another assertion writer driving the feedback edges deliberately.
Symptom→sub-phase navigation table: `docs/history/stage-model.md` §13 (caveat from practice: a
symptom's phase is not always its decision's phase — wrong-looking constants in the
rendered C are usually destroyed in Band B, not mis-rendered at P9).

In kuna this is concrete: `kassert <phase> <substage> <args> [hard|hint]` at the console
(reports the computed minimal rewind scope), `--option NAME VALUE` / `--kassert "<args>"`
on `kuna decompile`, and discovery via `kuna catalog`.

## Implementation state

The model is physical in kuna:

- **Registry & console**: every action/rule group, console surface, and settable
  assertion is phase-addressed in `kuna_phases.rs`; `phase list/map/status/catalog`,
  `kassert`, `pipeline`, `quality`, `restarts` operate it. The P7 region tree is
  directly observable via `region tree/blocks/walk` (the angr RegionIdentifier
  port — `docs/history/regions.md`).
- **Issues fixed through the model** (each pinned by a `tests/stages/` testcase
  asserting both directions of the decision): GH-558, 2786, 8471, 6930, 6990, 1282,
  7190, 8817, 8913, 9230, 1537. Per-phase changelog: `docs/history/stage-implementation.md`.
- **Defaults**: eight sub-phase fixes are kuna defaults (DIV-2); the destructive ones
  stay opt-in per the ablation evidence. Record: `docs/divergences.md`.
- **Empirical critique** of the model against those fixes (phase-fit, ablations,
  navigation score, per-phase verdicts): `docs/history/stage-critique.md`.
