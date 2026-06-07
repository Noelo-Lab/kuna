# The kuna Stage Model

**Decompiler stages, sub-stages, and feedback edges — derived from Ghidra, angr, and Reko.**

This document replaces the speculative 19-stage list previously in `STAGES.md`. It was
derived by studying the three decompilers side by side:

| Decompiler | Source studied | Role in this study |
|---|---|---|
| **Ghidra** (C++ decompiler) | `decompiler/cpp/` in this repo (byte-identical vendored copy of upstream `Ghidra/Features/Decompiler/src/decompile/cpp`) | The codebase kuna extracts; the model's primary target |
| **angr** | `angr/analyses/decompiler/` (+ `variable_recovery/`, `typehoon/`, `calling_convention.py`) | Highest output quality; richest feedback machinery (SAILR) |
| **Reko** | `src/Decompiler/` (+ `src/Core/`) | Simple baseline; clearest interprocedural (SCC) model |

All `file:line` anchors below refer to: Ghidra anchors → `decompiler/cpp/` in this repo;
angr anchors → `angr/angr/analyses/decompiler/`; Reko anchors → `reko/src/`.
Raw exploration evidence (structured reports with ~168 pipeline steps, ~80 feedback loops,
~98 decision points, ~150 intervention surfaces) is archived under `docs/evidence/`.

---

## 1. Why the 19-stage compiler model fails

The old list (`Loader → Decode → P-code Lift → Disassembly → IR Containers → Simplify →
Dataflow → SSA → Calls → Stack → Memory → Scalar Types → Aggregate Types → Variables →
Range → Interproc → Structuring → C Lowering → C Render`) treats a decompiler as a
feed-forward compiler. The evidence breaks that model in five ways:

1. **Phantom stages.** Four of the 19 stages (`Dataflow`, `Memory`, `Aggregate Types`,
   `Interproc`) map to *no module in any of the three decompilers* (the old
   19-stage mapping, now `docs/stage-mapping.md`, already had to admit this). They are not stages; they are aspects of
   other stages' fixed points.
2. **Over-split artifacts.** `Loader/Decode/Lift/Disassembly/IR Containers` are five names
   for at most two artifacts; `Simplify/Dataflow/SSA/Range` are four names for one op-graph
   being mutated in place; `C Lowering/C Render` is one inheritance chain (`PrintLanguage` →
   `PrintC`), not two passes.
3. **No feedback.** Ghidra's pipeline is *nested fixed-point loops inside a restart group*
   (`universal` ⊃ `fullloop` ⊃ `mainloop` ⊃ `stackstall` ⊃ rule pools,
   coreaction.cc:5609-5896). The single most important control-flow recovery mechanism —
   jump-table recovery — runs a **recursive reduced decompilation**
   (funcdata_block.cc:509-538). A linear list cannot express either.
4. **No place for the user/agent.** All three decompilers are operated through durable
   assertion stores (Ghidra Symbol DB + Override; angr KnowledgeBase; Reko `Program.User.*`)
   whose edits re-run the pipeline. The old model has no home for this, yet it is the
   dominant loop in practice.
5. **One tool's schedule baked in.** Any strict order among the middle analyses is a
   fiction: Ghidra co-iterates SSA, prototypes, types, and stack layout in *one* loop
   (`mainloop`, coreaction.cc:5636-5824), while angr/Reko run types as a separate terminal
   solver. A taxonomy must describe the dependency structure without canonizing either
   schedule.

---

## 2. Definitions

- **Stage** — a coarse processing step that owns one **artifact** (a materialized data
  structure with its own invalidation story). Stages trigger in order on the *first* pass;
  after that, order is governed by feedback edges.
- **Sub-stage** — a named decision point inside a stage where a different choice (made by
  the decompiler's heuristic, a human, or an LLM agent) changes the stage's artifact and
  everything downstream. Each sub-stage names the **assertion** that overrides it.
- **Feedback edge** — "stage X dirties the artifact of stage Y", typed by mechanism, with a
  **trigger signal**, a **rewind scope** (op / block / region / function / program), and
  **surviving state** (what persists across the rewind — this is what makes interventions
  durable).
- **Band** — a set of stages whose artifacts co-evolve to a *mutual* fixed point in at least
  one decompiler. Within a band, stage order is a per-decompiler *scheduling regime*, not
  part of the model.
- **Assertion** — a durable, typed fact written to the P0 store and consulted by a sub-stage
  on every (re-)run. Strength is **HARD** (blocks inference: e.g. typelock) or **HINT**
  (biases it: e.g. `must_struct`). Assertions are never imperative mid-pipeline edits.
- **LATENT** — a sub-stage whose decision exists in the code but has no override surface
  today. The LATENT set is kuna's product roadmap.

The test a stage must pass (and the phantom stages failed): *at least two of the three
decompilers materialize a distinguishable artifact for it, with its own invalidation story.*

---

## 3. The model at a glance

```
            P0  Knowledge & Configuration Plane  (orthogonal: read on every (re-)entry,
            ╔═══════════════════════════════════  written by analysis, humans, agents)
            ║         ▲ write-back (prototypes, names, notes, LLM suggestions)
            ▼         │
  S1 ──► S2 ──►┌──────┴───────── Band B ─────────────────┐──► S7 ──► S8 ──► S9
 Image   Flow  │  S3 Definition   S4 Call & Prototype    │  Region  AST &  Surface
 & Code  & Op- │     Web             Model               │  Hier-   Goto   Render
 Parti-  Graph │  S5 Value &      S6 Variable &          │  archy   Qual.  & Refine
 tion          │     Type Facts      Storage Model       │
               └── mutual fixed point (Ghidra mainloop) ──┘
      ▲            ▲                        │                  │
      │            └────────(c) restarts────┘                  │
      └──────────────(e) speculate/rollback, (c′) sub-query────┘
```

| | Stage | Artifact | One-line decision summary |
|---|---|---|---|
| P0 | Knowledge & Configuration Plane | assertion store + pipeline config | what is asserted, how strongly, and which pipeline variant runs |
| S1 | Image & Code Partition | bytes + segments + symbol/data map | what bytes exist and which are code/data/pointers |
| S2 | Flow & Op-Graph Recovery | lifted ops + CFG (`bblocks`) + jump tables (`jumpvec`) | where control flow goes |
| S3 | Definition Web *(Band B)* | SSA property over the op-graph + heritage state | which use reads which definition; when dead code may die |
| S4 | Call & Prototype Model *(Band B)* | `FuncCallSpecs`/prototypes/CC | what the interface contract of every call is |
| S5 | Value & Type Facts *(Band B)* | ranges, consume bits, type lattice | what values can be and what type they have |
| S6 | Variable & Storage Model *(Band B)* | `HighVariable`s / unified vars + frame layout | which SSA values are the same human variable |
| S7 | Region Hierarchy | region tree (`sblocks` ≠ `bblocks`) | what the nesting hierarchy of control flow is |
| S8 | Structured AST & Goto Quality | statement tree + goto set | which schema each region matches; which edges become gotos |
| S9 | Surface Rendering & Refinement | text + maps | names, casts, formats; external refinement |

The artifact split is physically real in Ghidra: `Funcdata` stores `jumpvec` (S2),
`bblocks` (S2), and `sblocks` (S7) as three separate members (funcdata.hh:90,94,95).

---

## 4. P0 — Knowledge & Configuration Plane (orthogonal)

**Artifact.** The durable store of assertions and pipeline configuration. It has *no
position in the first-pass order*: every stage reads it on (re-)entry, several stages write
inferred facts back into it, and it **survives every rewind** — that survivability is what
makes an intervention durable rather than a mid-pipeline edit.

| Decompiler | Store | Survives |
|---|---|---|
| Ghidra | Symbol DB (type/name locks database.hh:230-231, isolate database.hh:242, displayFormat) + Override table (force-goto, deadcode-delay, multistage-jump, proto/indirect/flow overrides; override.hh:50-92) + action-group config | `clearAnalysis()` and `ActionRestartGroup` restarts (action.cc:553-582) |
| angr | KnowledgeBase (`kb.variables[f].variables_with_manual_types`, `kb.functions[f].prototype` + source, labels) + `DecompilationCache` + options/presets | cache invalidation; manual types re-seed Typehoon as ground truth (clinic.py:2277-2303) |
| Reko | `Program.User.*` (Procedures, Globals, Calls, Heuristics, IndirectJumps, JumpTables, Patches, RegisterValues, Segments; Core/UserData.cs:39-177) + scripts | everything, including a full re-scan |

**Write-backs** (analysis → P0): Ghidra `ActionOutputPrototype`/`ActionInputPrototype`
commit recovered signatures; angr `CompleteCallingConventions` fills `kb.functions`
prototypes; Reko `CallRewriter.EnsureSignature` (Analysis/CallRewriter.cs:85-109). An LLM
refiner is the same kind of writer (angr `llm_refine`, decompiler.py:493-501).

**Sub-stages.**

| Sub-stage | Decision | Assertion (strength) | Exposure |
|---|---|---|---|
| Assertion admission policy | is an incoming fact ground truth or a bias; does it clear caches or only reflow | per-fact lock / cache policy (HARD vs HINT) | angr: `mark_manual=True`, `DecompilationOption.clears_cache` (fine-grained). Ghidra: locks are HARD, any edit → full re-run (coarse). Reko: all `User.*` admitted unconditionally. **LATENT everywhere: confidence-graded admission for agent-proposed facts** |
| Pipeline variant selection | which named pipeline/preset runs; restart budget | configuration assertion | Ghidra: root actions `decompile`/`jumptable`/`normalize`/`paramid`/`register`/`firstpass` via group filtering (coreaction.cc:5566-5605), `OptionSetAction`, per-rule toggles, `maxrestarts=1` (coreaction.cc:5621). angr: presets basic/fast/full/malware + `clinic_start/end/skip_stages` + `register_optimization_pass()`. Reko: `Needs*` gates + `User.Heuristics` |

A second, orthogonal axis lives here too: **injection slots** — arch-specific hooks that
add work *between* steps without being stages themselves (Ghidra `extra_pool_rules`
absorbed into the rule pool, coreaction.cc:5794-5796; angr `OptimizationPassStage` 0-11,
optimization_passes/optimization_pass.py:42-66; Reko `AnalysisStage` + `IAnalysisFactory`,
Core/Analysis/AnalysisContext.cs:121 — sparse-numbered explicitly so "custom analysis can
be injected at any of these stages"). All three tools independently evolved these; they are
the strongest evidence that *sub-stage = decision/injection point* is the natural grain.

---

## 5. S1 — Image & Code Partition

**Artifact.** Byte image + segment map + symbol/data map + the code-vs-data partition.

**Honesty note.** This stage passes the two-of-three test only barely: its invalidation
story is strong in Reko (DataScanner finds code pointers in data → enqueues procedures,
Scanning/Scanner.cs:1065-1118; shingle gap scanning), present in angr only as a late
write-back (final AST consts → `cfg.memory_data`, decompiler.py:463-465), and nearly absent
in Ghidra's C++ half (the partition arrives from the client; the decompiler only maps
globals late via `ActionMapGlobals`, coreaction.cc:5889, and promotes constants to pointers
via `ActionConstantPtr`). Treat S1 as thin by design — do not re-grow the old
Loader/Decode split here.

**Sub-stages.**

| Sub-stage | Decision | Assertion (strength) | Exposure |
|---|---|---|---|
| Environment binding | loader, architecture, platform/ABI, load address | environment assertion (HARD) | Reko: `User.Loader/Processor/Environment/LoadAddress`. Ghidra: `.ldefs`/`.cspec` selection at `SleighArchitecture` build. angr: project-level |
| Code/data partition & global map | which gaps are code; which data cells are code pointers; segment permissions | segment/global annotations (HARD) | Reko: `User.Segments`, `User.Globals`, `User.Heuristics('shingle')`. Ghidra: client-side; C++ side consumes Symbol DB. angr: loader symbols → `kb.variables['global']` |

---

## 6. S2 — Flow & Op-Graph Recovery

**Artifact.** The lifted op soup (Ghidra "dead list") + visited/context map + the **lazily
built** CFG (`bblocks`) + the work queues (`addrlist`/`tablelist`/`injectlist`; Reko
`procQueue` with priorities) + the **jump table** (`jumpvec`).

Block generation is lazy: Ghidra traces flow op-by-op (`FlowInfo::generateOps`,
flow.cc:383-580) and only then organizes blocks (`generateBlocks`, flow.cc:808-829). The
**jump table is a cross-stage artifact**: phase-1 target recovery happens here, phase-2
model/label matching (`matchModel`/`recoverLabels`) runs *after Band-B simplification*, and
a size mismatch fires a persisted-hint restart (jumptable.cc:2696-2721). Forcing the jump
table into a single stage repeats the old taxonomy's cross-cutting error — don't.

**Per-decompiler evidence.**

| | Implementation |
|---|---|
| Ghidra | `FlowInfo` (flow.cc): xrefControlFlow flow.cc:264-372, inline/inject flow.cc:1311-1339, call-spec setup flow.cc:680-723; `JumpModel` hierarchy (jumptable.hh:374-494); `stageJumpTable` reduced sub-decompilation (funcdata_block.cc:509-538) |
| angr | CFG is an *input*; AIL graph conversion + fixups (clinic.py `_decompilation_graph_recovery`/`_decompilation_fixups`, clinic.py:425-480); switch de-optimization passes at `AFTER_AIL_GRAPH_CREATION`; jump-table entry rewrites during region identification |
| Reko | Recursive-descent `Scanner` priority worklist (Scanning/Scanner.cs:1126-1166); `VectorBuilder`+`Backwalker` jump tables (Scanning/VectorBuilder.cs:98-135); `SplitBlock`/`PromoteBlockWorkItem` mid-scan graph surgery (Scanning/Scanner.cs:299-415) |

**Sub-stages.**

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Decode context resolution | which decoder context (THUMB, addr16/32…) applies at each address | context-set at address (HARD) | function | Ghidra: `ContextDatabase.setContextChangePoint/Region` (globalcontext.hh). Reko: `User.RegisterValues`/`Assume`. angr: from CFG |
| Decode error / limit policy | unimplemented/bad-data/out-of-bounds/too-many-instructions → truncate-with-HALT vs throw vs ignore | flow option flags | function | Ghidra: `flowoptions` flags (flow.hh:60-95) + `setMaximumInstructions` (flow.hh:145). Reko: `User.Patches` per address. angr: N/A (CFG/lifting are inputs to the decompiler) |
| Flow classification | call-vs-branch (PIC, thunk, tail call), noreturn kills fall-through | flow override (HARD) | function | Ghidra: `Override.insertFlowOverride/insertIndirectOverride` consulted at flow.cc:474,688,711. Reko: `Procedure.Characteristics(noreturn)`. angr: tail-call fixups, hardcoded |
| Inline / inject decision | clone callee body vs splice fixup p-code vs keep call | inline/inject directive | function | Ghidra: `FuncProto.isInline` + `PcodeInjectLibrary`. angr: `inline_functions`. Reko: **LATENT** (thunk heuristic hardcoded, Scanning/Scanner.cs:474-479) |
| **Switch model selection** | which model explains a `BRANCHIND` (JumpBasic/Basic2/Trivial/Assisted/Override), table bounds, case count | explicit jump table (HARD) | function (via c′/c) | Ghidra: `JumpBasicOverride` + `Override.insertMultistageJump`. Reko: `User.IndirectJumps`/`User.JumpTables` (richest). angr: tables resolved at CFG time; **LATENT** (no per-table assertion in the decompiler) |
| Reachability pruning | remove or keep unreachable blocks | unreachable policy | function | Ghidra: `removeUnreachableBlocks` flag-gated (flow.cc:827-828). Reko: `AggressiveBranchRemoval` heuristic. angr: `DeadblockRemover` preset-gated |

---

## 7. Band B — the mutual fixed-point band (S3–S6)

The four analysis stages below own four distinct artifacts — **but their first-pass order
is a per-decompiler scheduling regime, not part of the model**:

- **Ghidra (interleaved):** `ActionHeritage` (S3), `ActionActiveParam`/`ActionReturnRecovery`
  (S4), `ActionInferTypes` (S5), `ActionRestructureVarnode` (S6) are all members of the
  *same* repeating `mainloop` group (coreaction.cc:5636-5824). They run to *mutual*
  quiescence; no inter-stage boundary exists in the code.
- **angr (staged):** `ClinicStage` 2–15 orders them: callsites → SSA L0 → const-prop →
  callsites again → SSA L1 → simplify ×4 → variable recovery → Typehoon types
  (clinic.py:124-145, 684-699). Types are a terminal solver, cheap to re-run alone
  (`reflow_variable_types`).
- **Reko (staged, SCC-batched):** per call-graph SCC bottom-up: SSA + ~15 sub-transforms
  (Analysis/SccWorker.cs:160-277), then expression building, then types as a separate
  union-find phase (Typing/TypeAnalyzer.cs:77-112).

Inside the band, "later" stages routinely dirty "earlier" ones (see §11). Documenting the
band honestly — instead of inventing an order — is the single biggest correction over the
old taxonomy.

### 7.1 S3 — Definition Web

**Artifact.** The def-use linkage over the op-graph: phi placement (`MULTIEQUAL`),
INDIRECT effects, per-space heritage coverage (`LocationMap`), and the dead-definition
gate. **SSA is a reversible *property* of the op-graph, not a stage**: Ghidra heritages the
same Varnode web in place (heritage.cc:2663-2758); Reko converts back out of SSA when done
(`UnSsaTransform`, Analysis/DataFlowAnalysis.cs:243-325); angr makes the property explicit
and two-level (L0 registers, L1 stack).

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Heritage staging / space delay | per-space order and delay of SSA construction (stack delayed to collect varnodes first) | per-space delay | function | Ghidra: `HeritageInfo.delay` from arch spec — **LATENT at user level** (heritage.hh:124-126). angr: L0/L1 split fixed — **LATENT** |
| **Dead-definition gate** | when dead-code removal is allowed per space (`pass > deadcodedelay`); premature removal → restart | deadcode-delay (HARD, survives restart) | function (mechanism c) | Ghidra: `Override.insertDeadcodeDelay`; auto-bumped at heritage.cc:2571-2581, 2712-2730. angr: `remove_dead_memdefs` (coarse). Reko: **LATENT** |
| Phi / range granularity | split a memory range at gaps before placing phis (hardcoded `size > 4 && max < size`) | none today | function | **LATENT everywhere** (heritage.cc:2610) — matters for struct-heavy frames |
| Simplification quiescence | which algebraic/peephole rules fire; when the local fixed point is declared | per-rule enable/disable | op→function | Ghidra: `OptionToggleRule`, `Rule::setDisable`, arch `extra_pool_rules`. angr: `peephole_optimizations` list. Reko: **LATENT** (ExpressionSimplifier rules static) |
| Comparison canonicalization *(kuna-exposed, GH-558)* | canonicalize `V <= c` to `V < c+1` for analysis; fires from the rule pool AND the structuring-time branch-flips (3 sites via `replaceLessequal`) | `compareform canonical/original` (pipeline-config) | function | kuna: group `canonicalcompare` + provenance bit + `ActionPresentCompareForm` at the S8→S9 boundary (`docs/prototypes/gh558.md`). Upstream Ghidra/angr/Reko: **LATENT** |

*(The simplification fixed point operates on S2's op-graph artifact; it lives here because
its gate — what may be deleted — is the definition web's concern. `ActionDeadCode` itself
is a straddler: see §15.)*

### 7.2 S4 — Call & Prototype Model

**Artifact.** `FuncCallSpecs`/`FuncProto` (Ghidra), `function.prototype` + callsite
prototypes (angr), `ProcedureFlow` signatures (Reko): parameters, returns, calling
convention, preserved/trashed sets, extrapop.

**Interprocedural regimes diverge sharply** (and the model treats the regime as a
documented property, not a stage): Ghidra is per-function with restart-on-change and trial
recovery (`ParamActive`/`ParamTrial`, fspec.hh:285-337); Reko is bottom-up over call-graph
SCCs with callee summaries flowing to callers (Analysis/DataFlowAnalysis.cs:129-200);
angr recovers calling conventions on demand plus a whole-program `CompleteCallingConventions`
scheduler writing into the KB.

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Prototype source | locked/declared signature vs trial recovery | locked prototype (HARD) | function (Ghidra), S4+ (angr) | Ghidra: `FuncProto.setInputLock/setOutputLock` (getters: `isInputLocked/isOutputLocked`, fspec.hh:1397-1402; locked protos skip trials, coreaction.cc:1768). angr: `kb.functions[f].prototype` + `prototype_source=USER`. Reko: `User.Procedures[addr].Signature/CSignature` |
| Trial finalization budget | how many passes before param/return trials freeze (`numpasses >= maxpass`) | pass budget | function | Ghidra: `maxpass` per prototype model — **LATENT as a user assertion** (coreaction.cc:1774-1784). angr/Reko: hardcoded — **LATENT** |
| ABI trust | trust platform ABI for trashed/preserved vs infer from dataflow | ABI-trust flag | program | Reko: `User.Heuristics(CallsRespectABI)` (Analysis/SsaTransform.cs:98-100). Ghidra: prototype model choice (`OptionDefaultPrototype`). angr: **LATENT** |
| Extrapop / stack-delta | stack adjustment across calls when underdetermined (`StackSolver`, coreaction.cc:25-105) | extrapop override | function | Ghidra: `OptionExtraPop`. Reko: characteristics (partial). angr: from CC — **LATENT** |
| Interprocedural regime / batching | per-function-with-restart (Ghidra) vs bottom-up SCC summaries (Reko, hardcoded `SccFinder.Condense`, Analysis/DataFlowAnalysis.cs:189-230) vs on-demand + whole-program scheduler (angr) — decides what callee knowledge callers see and when | regime selection | program/SCC | **LATENT in all three** (each regime is hardcoded); see §14 for the regimes |

### 7.3 S5 — Value & Type Facts

**Artifact.** The fact fabric over the op-graph: value sets/ranges (`CircleRange`/
`ValueSetSolver`, rangeutil), consume bits, nonzero masks, and the type lattice state
(Ghidra `TypeFactory` + per-op `TypeOp.propagateType`; angr Typehoon constraints/solution;
Reko `TypeStore` equivalence classes).

**The biggest architectural divergence in the corpus lives here**: Ghidra runs a bounded
(7-pass) bidirectional lattice propagation *inside* the band loop (coreaction.cc:5521-5563)
with an explicit S5↔S6 feedback path (`propagateSpacebaseRef`, coreaction.cc:5412, invoked
from the inference pass at coreaction.cc:5557, mediated by `mainloop` repetition); angr and
Reko collect
constraints and solve once, terminally (Typehoon, clinic.py:2312-2361; union-find,
Typing/TypeAnalyzer.cs:77-112) — which is exactly what makes angr's cheap type-reflow
possible and Ghidra's impossible without a full re-run.

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Range / value-set facts | widening strategy and iteration cutoffs (`WidenerFull` 2/5, `WidenerNone` 3, rangeutil.hh:236-261); **feeds S2 jump-table bounds and determined branches** — a wrong range is a *flow* symptom, not a type symptom | range assertion | function | **LATENT everywhere** (only indirect: Ghidra `OptionJumpTableMax` caps case count) |
| Constant → pointer interpretation | is this constant an address? (space attribute > usage inference, `ActionConstantPtr::apply`, coreaction.cc:1168) | pointer/equate annotation | function | Ghidra: space attributes + Symbol equates. Reko: `User.Globals`. angr: partially via KB |
| Type lattice propagation / solve | per-opcode flow direction; solver model (in-loop lattice vs terminal solver); **type locks are HARD stops** (`propagateTypeEdge` returns false on `isTypeLock()`, coreaction.cc:5239) | **type lock (HARD)** / type hint (HINT) | Ghidra: function (full re-run); angr: S5→S9 only (`reflow_variable_types`, decompiler.py:699-830) | Ghidra: Symbol typelock. angr: `set_variable_type(mark_manual=True)`, `type_hints`. Reko: user sigs/globals seed collection (Typing/TypeCollector.cs:70-102) |
| Aggregate / union resolution | when to synthesize struct/array; which union field wins (`ScoreUnionFields` weights) | union facet (HARD) / must-struct (HINT) | function | Ghidra: `UnionFacetSymbol` via `DynamicHash` (consulted first). angr: `vars_must_struct`. Scoring weights **LATENT everywhere** |
| Solver budget | 7-pass cap (Ghidra, compile-time), constraint degradation threshold 150 (angr) | solver budget | function | angr: `type_constraint_set_degradation_threshold` (exposed). Ghidra: **LATENT** (coreaction.cc:5537-5543) |

### 7.4 S6 — Variable & Storage Model

**Artifact.** The mapping from SSA values + storage to human variables: `HighVariable`
(+ `Cover`, variable.hh), angr unified `SimVariable`s, Reko webs; plus the stack-frame
symbol layout (`ScopeLocal`/`varmap`) and alias sets.

Note the **object-grain invalidation** here: a merge sets `coverdirty/typedirty/symboldirty`
on the `HighVariable` and recomputation is lazy on next access (variable.hh:120-152) — a
different mechanism from any pass re-run (§11 g).

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Merge aggressiveness | forced (phi/address-tied) vs speculative (cover/type heuristics) vs adjacent merges; over-merging fuses distinct variables | **isolate (HARD)** | function | Ghidra: `Symbol::setIsolated` blocks speculative merge. angr: identity persists in KB via rename. Reko: **LATENT** |
| Stack-frame layout | overlapping `RangeHint` conflict policy: typelock > rangeType > type specificity (varmap.cc:126-157) | stack symbol + type lock (HARD) | function | Ghidra: locked stack symbols. angr: `static_buffers` + manual stack var types. Reko: **LATENT** for stack |
| Alias / dynamic facets | which stack offsets a pointer may alias (`AliasChecker` boundary policy: `localBoundary`/`localExtreme` from the prototype, varmap.cc `deriveBoundaries`); binding symbols to ephemeral temporaries by dataflow hash | dynamic-hash symbol | function | Ghidra: `DynamicHash` symbol attachment (dynamic.cc); boundary policy itself **LATENT**. angr/Reko: **LATENT** |
| Explicit vs implied | does a value get its own named token or inline into its parent expression (`ActionMarkExplicit/Implied`) | per-variable visibility | S9 only | **LATENT per-variable everywhere** (angr `expr_collapse_depth` is global). High-value readability knob for kuna |

---

## 8. S7 — Region Hierarchy

**Artifact.** The region tree — **distinct from the CFG**. Ghidra stores it separately
(`sblocks` vs `bblocks`, funcdata.hh:94-95) and rebuilds it twice (pre- and post-merge
`ActionBlockStructure`, coreaction.cc:5806,5884); angr's `RegionIdentifier` is re-run after
*any* graph edit by a `DURING_REGION_IDENTIFICATION` pass (decompiler.py:643-663); Reko
rebuilds dominators per structuring iteration (Structure/StructureAnalysis.cs:143-186).
This split is what makes the SAILR rollback edge (§11 e) expressible.

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Loop refinement / single-exit policy | how aggressively to absorb nodes into loops; force single-exit via guarded successors | region-shaping options | region | angr: `force_loop_single_exit`, `refine_loops_with_single_successor`, `complete_successors`. Ghidra/Reko: **LATENT** |
| **Edge virtualization (goto) selection** | which edge to cut when a region won't structure — SAILR ranks fewest-siblings/most-postdoms/goes-to-return (sailr.py:41-123); Ghidra ranks by likelihood (`TraceDAG::selectGoto`, blockaction.cc); Reko last-resort | **force-goto (HARD, per-edge)** | region→S7+ | Ghidra: `Override.insertForceGoto`. angr/Reko: **LATENT per-edge** — high-value kuna target |
| Re-identification batching | defer graph-topology updates so de-optimization passes can iterate (`delay_graph_updates`) | pass-set selection | region | angr: preset/pass list (decompiler.py:385-388). Others: implicit |

---

## 9. S8 — Structured AST & Goto Quality

**Artifact.** The schema-matched statement tree (SequenceNode / BlockIf/While/Switch /
AbsynStatement) *plus the goto set as a first-class quality signal* (angr `GotoManager`).

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Schema precedence / structurer choice | rule order when patterns overlap (Ghidra greedy: Goto,Cat,If,IfElse,While,DoWhile,InfLoop,Switch — blockaction.cc:1797-1840; angr: switch-before-ITE in Phoenix; algorithm choice Phoenix/SAILR/DREAM); includes break/continue/back-edge assignment when a loop matches (phoenix.py:283-323) | structurer selection | function | angr: `structurer_cls` + `improve_algorithm` (exposed). Ghidra/Reko: **LATENT** rule order |
| **Goto placement & quality acceptance** | accept or roll back a graph edit based on measured structure quality (goto count must not grow; for-loops must not degrade to while) | quality policy (per-pass) | region (speculative) | angr: `prevent_new_gotos`, `strictly_less_gotos`, `must_improve_rel_quality`, `recover_structure_fails`, `max_opt_iters` (optimization_pass.py:546-587, 662-724). Ghidra/Reko: **LATENT — no rollback at all** (greedy goto emission) |
| Readability rewrites | switch-vs-if-cascade, for-recognition, condition complexity (multi-stmt expressions) | structuring options | function | angr: `simplify_ifelse`, `MultiStmtExprMode`. Ghidra: `OptionAnalyzeForLoops`, `OptionJumpTableMax`. Reko: always-on (**LATENT**) |

---

## 10. S9 — Surface Rendering & Refinement

**Artifact.** Final text + position/instruction maps (Ghidra token markup; angr
`posmap`/`instr_map`; Reko output files). The only near-feed-forward stage — and therefore
the cheapest re-run scope (angr codegen-only options set `clears_cache=False`).

| Sub-stage | Decision | Assertion (strength) | Re-run scope | Exposure |
|---|---|---|---|---|
| Naming policy | auto vs symbol-derived vs semantic-pattern names; preservation of user names | **name lock (HARD, cosmetic)** | S9 only | Ghidra: namelock (database.hh:231) + `ActionNameVars`. angr: `var.renamed=True` + `semvar_naming`. Reko: `User.*` names |
| Cast policy | minimal-necessary vs always vs never (`CastStrategyC`, `ActionSetCasts` coreaction.cc:5892) | cast option | S9 only | Ghidra: `OptionNoCastPrinting` (global). angr: `show_casts`. **Per-expression LATENT everywhere** |
| Literal / format policy | integer base, NULL-vs-0, per-symbol display format | format assertion (per-symbol or global) | S9 only | Ghidra: `OptionIntegerFormat`/`OptionNullPrinting` + `Symbol.displayFormat`. angr: `const_formats` (per-location). Reko: **LATENT** |
| Pointer notation *(kuna-exposed, GH-558)* | render standalone `PTRADD` as `base + index` vs `&base[index]` | `arraynotation on/off` | S9 only | kuna: `OptionArrayNotation` → `PrintC::opPtradd` (`docs/prototypes/gh558.md`). Upstream Ghidra: **LATENT** (array form only inside load/store context) |
| External (LLM) refinement | proposed renames/retypes/summaries — **an assertion writer into P0 for the next run**, not a new analysis | any P0 assertion | per assertion | angr: `llm_refine` → notes → user applies (decompiler.py:493-501). The agent loop generalizes this |

---

## 11. Feedback edge model

The non-linearity core. An edge is *"stage X dirties the artifact of stage Y"*; every edge
carries a **trigger signal**, a **rewind scope**, and **surviving state**.

### Mechanism types

| | Mechanism | What it is | Canonical evidence |
|---|---|---|---|
| **a** | Local fixed-point | repeat within one stage until quiescent | Ghidra rule pools `rule_repeatapply` + opcode-change rewind (action.cc:822-888); angr `BlockSimplifier` ≤30 iters (block_simplifier.py:130-150); Reko VP+Coalescer loops (Analysis/DataFlowAnalysis.cs:275-286) |
| **b** | Staged re-entry | a later edit re-runs an earlier analysis *within the same run* | angr graph edit → `RegionIdentifier` re-run (decompiler.py:643-663); Reko `IndirectCallRewriter` → re-SSA (Analysis/SccWorker.cs:234-240); Ghidra spacebase type → `ActionRestructureVarnode` (`propagateSpacebaseRef`, coreaction.cc:5412,5557) |
| **c** | Global restart with persisted hints | clear-and-replay of the whole function; the *lesson* survives in P0 | `setRestartPending` + `ActionRestartGroup` (action.cc:553-582); hints: `insertMultistageJump` (jumptable.cc:2712-2717), `insertDeadcodeDelay` (heritage.cc:2571-2581). **Budgeted** (`maxrestarts=1`) and **gated off during c′** (`isJumptableRecoveryOn`) |
| **c′** | Reduced-pipeline sub-query | a *recursive sub-decompilation* with a different pipeline, on a clone, to answer one question | `stageJumpTable`: clone partial `Funcdata`, switch to the `jumptable` group set, `reset`+`perform`, restore (funcdata_block.cc:509-538). Weak analogs: Reko `Backwalker`; angr `LoweredSwitchSimplifier` |
| **d** | Knowledge-store re-decompilation | the outer human/agent loop: write assertion to P0, re-run — **parameterized by scope** | full: Ghidra `clearAnalysis` + re-decompile; angr `clears_cache=True`. Scoped: angr `reflow_variable_types` re-runs only S5→S9 (decompiler.py:699-830). *Same edit, different scope per tool* |
| **e** | Speculate-validate-rollback | edit, re-derive, measure quality, accept or roll back | angr `StructuringOptimizationPass`: edit graph → re-identify → re-structure → check gotos/loop quality → restore `_prev_graph` on regression (optimization_pass.py:546-587). Ghidra's pre/post-merge double structuring is the weak form; Reko's last-resort goto is the degenerate accept-all |
| **h** | Automatic fallback re-run | an error/failure triggers a whole-function re-run under a *degraded* pipeline variant; trigger is an exception, not an assertion; nothing survives but the variant switch | angr: failed `_decompile` → clear errors, switch to the `basic` preset, retry (decompiler.py:204-223). Related flavor: Reko `Needs*` gates, Ghidra bad-data truncation |
| **f** | *(annotation)* Worklist re-enqueue | address-grain late binding: work appended to a still-draining queue | Ghidra `tablelist`/`addrlist`/inline repopulation (flow.cc:781-805); Reko `procQueue` priorities, `SplitBlock`/`Promote` (Scanning/Scanner.cs:299-415) |
| **g** | *(annotation)* Lazy-dirty recompute | object-grain: dirty flags + demand recompute, no pass re-run | `HighVariable` cover/type/symbol dirty flags (variable.hh:120-152) |

(f) and (g) are annotations on edges rather than top-level types: they describe *grain*
(address / object) rather than new feedback semantics. Note also that mechanisms can be
**mutually disabling**: restarts (c) are suppressed during a jump-table sub-query (c′), and
deadcode-delay bumping is skipped under `isJumptableRecoveryOn` (heritage.cc:2714,2723).

### Edge table

| # | From → To | Mech | Trigger signal | Rewind scope | Survives |
|---|---|---|---|---|---|
| 1 | S2 → S2 | c′ | `BRANCHIND` with unrecovered targets | cloned function (discarded) | recovered table → `jumpvec` |
| 2 | S2 → S2 | c | jump-table size mismatch after simplification (`matchModel`) | function | `Override.insertMultistageJump` |
| 3 | Band B → S3/S2 | c | free varnode reappears at a heritaged address after dead-code removal | function | `Override.insertDeadcodeDelay(+1)` |
| 4 | S5 → S6 | b | spacebase-relative pointer type changes implied frame layout | function (capped iterations) | type facts |
| 5 | S4 → Band B | b/c | prototype changes (trial finalization, lock, callee commit) | function | committed prototype in P0 |
| 6 | S5 → S2 | b | constant-folded condition determines a branch → CFG edge removed (`ActionDeterminedBranch`) | block/function | simplified ops |
| 7 | S1/S2 → S2 | f | new code address from jump table / **data cell reclassified as code pointer** (Reko DataScanner, Scanning/Scanner.cs:1065-1118 — S1's one feedback edge) / inline clone | address | visited map, queues |
| 8 | S4 ↔ S4 | a | callee preserve/trash summary changes caller liveness within an SCC (Reko) | SCC | `ProgramDataFlow` summaries |
| 9 | S3/S5 → S3/S5 | a | any rule fires / value propagates (oppool1, BlockSimplifier, VP loops) | op→function | — |
| 10 | S8 → S7/S2 | e | de-optimization graph edit; goto/loop-quality regression measured post-structuring | region (speculative) | accepted graph or restored `_prev_graph` |
| 10′ | opt-pass → S7 | b | any `DURING_REGION_IDENTIFICATION` pass returns a modified graph → unconditional `RegionIdentifier` re-run (decompiler.py:643-663) — the always-on base that edge 10's quality gate wraps | region | re-identified region tree |
| 11 | S8 → S8 | a | schema match mutates the region graph mid-pass (`GraphChangedNotification`, phoenix.py:1266-1280; Reko region reduction do-while) | region | matched subtrees |
| 12 | S6 → S6 | g | merge changes a variable's cover/type/symbol | object | dirty flags |
| 13 | P0 → S5 | d (scoped) | manual type written (`mark_manual=True`) + reflow requested | S5→S9 only | KB ground truth |
| 14 | P0 → S1/S2 | d (full) | cache-clearing assertion: proto lock, override, force-goto, `clears_cache=True` option | function/program | the entire P0 store |
| 15 | S9 → P0 | d (writer) | LLM/human reads output, proposes assertions | n/a (write) | new assertions |
| 16 | error → S1+ | h | first decompile attempt errored | function | degraded preset selection (angr decompiler.py:204-223) |

---

## 12. Intervention model

```
assert(stage, anchor, assertion_type, value, strength)  →  P0 store
        │                                                    │
        └── anchor: address / varnode-hash / symbol /        └── scheduler computes the
            call-site / edge / option name                       minimal rewind set from the
                                                                 target artifact + transitive
                                                                 closure over §11 edges
```

- **Strength semantics.** HARD locks stop inference unconditionally (`propagateTypeEdge`
  returns false on typelock, coreaction.cc:5239; ground-truth TypeVariables are solver
  fixed points). HINTs bias (angr `type_hints`, `must_struct`, guessed prototypes). Names
  are cosmetic (S9-only) — the type/name asymmetry in Ghidra is intentional and worth
  preserving.
- **Scope semantics.** An assertion's *minimal invalidated artifact* determines the re-run:
  format → S9; force-goto → S7+; type lock → S5+ where types are a terminal solver (angr),
  whole function where they are in-loop (Ghidra today); proto lock → S4+/Band B; jump
  table / context → S2+. Ghidra currently rounds every scope up to "whole function"
  (`clearAnalysis`); making scope real is kuna's job (§16).
- **An LLM agent is just another assertion writer**, operating the loop deliberately:
  propose assertion → scoped re-run → read the changed artifact → refine. None of the three
  tools grade incoming assertions by confidence (HARD vs HINT admission is binary and
  caller-chosen) — a confidence-graded admission policy at P0 is the key missing surface
  for agent integration.

---

## 13. Symptom → sub-stage navigation (worked examples)

| Symptom in output | Stage / sub-stage | Assertion to apply | Mechanism that propagates it |
|---|---|---|---|
| Wrong function signature / spurious params | S4 prototype source; check trial budget | lock prototype (`setInputLock` / `User.Procedures.Signature` / `kb prototype=USER`) | edge 5 (b/c): re-runs Band B with the lock |
| Missed switch cases / `switch` became `goto` | S2 switch model; *also check S5 range facts* (bounds feed table size) | explicit jump table (`JumpBasicOverride`, `User.JumpTables`); widen range | edges 1-2 (c′/c): sub-query or multistage restart |
| Goto spaghetti | S7 edge virtualization + S8 quality policy | force-goto on the *right* edges; enable quality-gated de-optimization (SAILR-style) | edge 10 (e): speculative edit with rollback |
| Wrong type propagating everywhere | S5 type lattice | type lock at the source variable (HARD) | edge 13 (d-scoped) where supported; edge 14 (d-full) in Ghidra |
| Two distinct variables merged into one | S6 merge aggressiveness | `Symbol::setIsolated` on the victim | edge 14 (d): re-run; speculative merges skip it |
| Locals look like raw offsets / no struct | S5 aggregate + S6 frame layout | stack symbol with locked struct type; `must_struct` hint | edges 4-5 (b): restructure ↔ re-infer |
| Code missing entirely | S2 decode-error policy / S1 partition | patch bad instruction (Reko `User.Patches`); raise instruction limit; mark segment executable | edge 7 (f) + full re-run |
| Comparison shows `< c+1` instead of source's `<= c`; pointer arith instead of `[i]` (GH-558) | S3 comparison canonicalization + S9 presentation | `option compareform original`; `option arraynotation on` (kuna) | re-decompile; restore happens at the S8→S9 boundary (`docs/prototypes/gh558.md`) |

---

## 14. Per-decompiler scheduling regimes (appendix)

### Ghidra — one universal action, filtered and restarted
- **Universal action** (coreaction.cc:5609-5896): `ActionRestartGroup(universal, maxrestarts=1)`
  containing setup → `fullloop`(repeat) ⊃ `mainloop`(repeat) ⊃ `stackstall`(repeat) ⊃
  `oppool1` (131 rules, opcode-indexed worklist) … then cleanup pool → merge phalanx →
  casts/naming → final structure.
- **Action vs Rule**: an Action is a whole-function pass with `(status, count)` resumable
  state; a Rule is opcode-indexed and applied via worklist — `O(ops × avg-rules-per-op)`
  instead of `O(ops × rules)` (action.cc:822-888). The schedule unit is the
  `(Action, status, count)` triple, which is what makes interactive resume/breakpoints
  possible (`setBreakPoint`, action.cc:171-185).
- **Pipeline variants by group filtering** (coreaction.cc:5566-5605): `decompile` (33
  groups), `jumptable` (12 — `noproto` instead of `protorecovery`), `normalize`, `paramid`,
  `register`, `firstpass`. A variant is a *filter over the same universal action*
  (`ActionDatabase::deriveAction`/`setCurrent`), not a separate pipeline — which is why
  c′ sub-queries are cheap to express.
- **Restart discipline**: only the outermost `ActionRestartGroup` checks
  `restartPending`; inner loops converge locally. Cleanup is deferred *after* `fullloop`
  precisely so its rules cannot re-trigger type/proto changes — a termination-policy
  decision, not a content decision (it maps to the S3 simplification-quiescence /
  P0 pipeline-variant axes and is **LATENT**: there is no knob to move `actcleanup`).

### angr — explicit stage enum + injection slots + quality-gated passes
- `ClinicStage` 0-15 (clinic.py:124-145) with `start/end/skip` controls; dispatch map at
  clinic.py:684-699.
- `OptimizationPassStage` 0-11 (optimization_pass.py:42-66) is an orthogonal injection-slot
  axis; passes declare `STAGE`, `ARCHES`, `PLATFORMS`, `STRUCTURING` compatibility.
- Presets (basic/fast/full/malware) select pass sets; `register_optimization_pass()` is the
  third-party hook.
- SAILR's `StructuringOptimizationPass` carries a *per-pass rewind policy*
  (`prevent_new_gotos`, `must_improve_rel_quality`, `recover_structure_fails`,
  `max_opt_iters`) — risk tolerance is per-pass, not global.
- The cache (`DecompilationCache`) is the state boundary: codegen-only options re-render;
  `clears_cache=True` options re-run Clinic; `reflow_variable_types` re-solves types only.

### Reko — linear driver, SCC batches, ad-hoc loops
- Driver: `Scan → AnalyzeDataFlow → ReconstructTypes → StructureProgram → Write`
  (Decompiler.cs:132-154) with `Needs*` gates.
- Scanner is a priority worklist (entry=5, jump-target=6, global-data=7, vector=4,
  promote=3) — soft ordering that adapts to discovery.
- `DataFlowAnalysis` processes call-graph SCCs bottom-up; `SccWorker` runs ~15 sub-transforms
  with several ad-hoc loops (re-SSA on new frame vars; indirect-call resolution loop).
- Types are a fully decoupled phase (collect → unify → rewrite); no feedback into SSA.
- `AnalysisStage` (PreSsa=0, AfterRegisterSsa=1000, AfterStackSsa=2000, …) is sparse-numbered
  for injection (`IAnalysisFactory`).

---

## 15. Straddler registry

Placement is by *dominant owned artifact*; secondary effects are edges. These units span
artifacts and must not be forced into one box (the old taxonomy's recurring error):

| Unit | Home | Secondary effects |
|---|---|---|
| `ActionDeadCode` (coreaction.cc:4070-4208) | S5 (consume-bit fact propagation) | destroys ops (S2 artifact); gated by S3's deadcode delay (edge 3) |
| Jump table (`jumptable.cc`, `jumpvec`) | S2 (phase-1 target recovery) | phase-2 `matchModel`/`recoverLabels` runs post-Band-B; mismatch → edge 2 (c) |
| `ActionInferTypes` (coreaction.cc:5521-5563) | S5 | in-mainloop scheduling couples it to S4/S6 (edges 4-5) |
| `blockaction.cc` double structuring | S7 | first pass allows mods (S2 graph edits), second is analysis-only — the weak form of mechanism e; the in-`mainloop` `ActionBlockStructure` (coreaction.cc:5806) also re-fires Band-B actions (`ActionRedundBranch`) — a (b)-grain in-loop rebuild |
| `ActionConstantPtr` (coreaction.cc:1168) | S5 (constant→pointer fact) | feeds the S1 global/data map (promotes constants into address spaces) |
| `userop` (CALLOTHER definitions) | S2 (lift semantics) | volatile-memory modeling (S3/S5); display control (S9) |
| `ActionMarkExplicit/Implied` | S6 | consumed only at S9; no analysis effect |
| `heritage` LoadGuards (`analysisState` 0→1→2, heritage.hh:150, heritage.cc:753-799) | S3 | refined by S5 value-set analysis (two-phase: generate → analyze) |

---

## 16. kuna roadmap (ordered)

1. **Typed assertion API over the existing stores.** Ghidra already ships the richest HARD
   assertion surface (Override + Symbol DB + locks). Surface it as
   `assert(stage, anchor, type, value, strength)` mapped to the §5-§10 sub-stage catalogs.
   No new analysis code; battle-tested paths.
2. **Scoped re-run.** Replace blunt `clearAnalysis` with minimal-rewind computation: tag
   each assertion type with its earliest invalidated artifact and re-run only the closure
   (angr-reflow philosophy). This is the biggest interactivity win for human/agent loops —
   today every Ghidra edit costs a full function re-decompile.
3. **Named reduced pipelines as first-class objects.** `jumptable`/`normalize`/`paramid`/
   `register`/`firstpass` already exist as group filters; expose running them as
   sub-queries (mechanism c′ proves switch recovery *requires* this).
4. **Quality-gated structuring (port of mechanism e).** Ghidra greedily emits gotos with no
   rollback; SAILR's measure-and-rollback is the biggest output-quality gap. Port the
   accept-policy pattern (goto count + structure-quality counters) around
   `ActionBlockStructure`/`CollapseStructure`.
5. **Restart observability.** Report *why* a function restarted (deadcodedelay bump vs
   multistage jump vs proto change) — the single most diagnostic signal in the engine and
   currently invisible.
6. **The LATENT catalog as backlog.** Highest-value first: per-edge force-goto everywhere
   (S7), per-variable explicit/implied (S6), trial/solver budgets as assertions (S4/S5),
   phi/range granularity (S3), confidence-graded admission at P0.

---

## 17. Method & evidence

Derived 2026-06 from parallel structured exploration of the three codebases (10 readers),
three independent taxonomy designs (artifact-, decision-, and scheduling-centric lenses),
and two adversarial critique passes (evidence coverage + kuna usability), followed by
manual verification of every load-bearing claim in source. Raw reports: `docs/evidence/`.

Provenance note: edge 1 (mechanism c′, `stageJumpTable`) is corroborated by direct manual
source verification (funcdata_block.cc:509-538) rather than by an exploration report; the
nearest exploration-evidenced analog is Reko's `Backwalker` (docs/evidence/08.json). All
other mechanisms have multiple direct loop instances in the evidence files.

Key verified anchors: `universalAction`+`buildDefaultGroups` coreaction.cc:5566-5896;
`stageJumpTable` funcdata_block.cc:509-538; `jumpvec/bblocks/sblocks` funcdata.hh:90-95;
`ClinicStage` clinic.py:124-145; `OptimizationPassStage` optimization_pass.py:42-66;
Reko driver Decompiler.cs:132-154; `AnalysisStage` Core/Analysis/AnalysisContext.cs:121.
