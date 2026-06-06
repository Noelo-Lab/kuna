# Stage-model evidence

Raw structured outputs from the 2026-06 study that produced `docs/stage-model.md` and the
rewritten `STAGES.md`. Anchors inside these files refer to the source trees as of the study
date (Ghidra anchors → `decompiler/cpp/` in this repo; angr → `angr/angr/analyses/decompiler/`;
Reko → `reko/src/`); the curated, verified subset lives in `stage-model.md`.

## Exploration reports (10 parallel readers, structured schema)

| File | Decompiler | Focus |
|---|---|---|
| `00.json` | Ghidra | Action/Rule engine, `universalAction` pass order, restart mechanism, group system |
| `01.json` | Ghidra | Frontend: flow following, jump-table recovery lifecycle, overrides, injection |
| `02.json` | Ghidra | Middle-end: heritage/SSA, deadcode delay, prototype trials, varmap, value sets |
| `03.json` | Ghidra | Types, variable merging, structuring, emission, user-intervention model |
| `04.json` | angr | Decompiler/Clinic pipeline (`ClinicStage`), caching, variable/type integration |
| `05.json` | angr | Optimization-pass system (`OptimizationPassStage`), presets, SAILR quality gates |
| `06.json` | angr | Structuring subsystem: RegionIdentifier, Phoenix/SAILR, goto feedback |
| `07.json` | angr | Options surface, KnowledgeBase intervention model, Typehoon, reflow |
| `08.json` | Reko | Driver phases, scanner worklist, `Program.User.*` surfaces |
| `09.json` | Reko | DataFlowAnalysis/SCC interprocedural model, typing, structuring |

Each report contains: `pipeline` (ordered steps with anchors), `feedback_loops`,
`decision_points`, `intervention_surfaces`, `insights`.

## Taxonomy designs and critiques

| File | Role |
|---|---|
| `A-artifact.json` | Proposal A — artifact-centric lens (SSA-as-property, orthogonal P0 plane, region/CFG split) |
| `B-decision.json` | Proposal B — decision-centric lens (assertion catalog, HARD/HINT, LATENT flags) |
| `C-scheduling.json` | Proposal C — scheduling/feedback lens (mechanism typology, reduced-pipeline sub-query, rewind scopes) |
| `critique-coverage.json` | Adversarial critic: evidence coverage, forced placements, synthesis recommendation |
| `critique-usability.json` | Adversarial critic: kuna usability, straddler test cases, symptom navigation |

The final model in `stage-model.md` is the critics' converged synthesis: A's artifact
spine + B's sub-stage/assertion catalog + C's feedback-edge and scheduling model.
