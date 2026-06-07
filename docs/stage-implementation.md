# Stage implementation manifest

How kuna's code was modified to match the stage model (`STAGES.md`,
`docs/stage-model.md`), stage by stage. This is the *session changelog against the
model* — the model itself is normative and lives in those documents; per-file placement
is `STAGE_MAPPING.md` §0. Updated per work session; honest entries ("nothing changed")
are deliberate.

**Session 2026-06-06 — stage-model physicalization.** Infrastructure: the stage
registry (`kuna_stages.{hh,cc}`: group→stage map, 40-entry sub-stage catalog, surface
routing), the self-registering console capability (`kuna_console.{hh,cc}`: `stage
list/map/status`, `pipeline`, `quality`, `restarts` — zero upstream anchor edits), the
typed assertion API (`kuna_assert.{hh,cc}`: `kassert` + reported minimal rewind
scopes), and restart observability (`kuna_restartlog.{hh,cc}` + five one-line trigger
anchors). Plus the per-stage exposures and issue fixes below.

Template per stage:
- **Registry**: how the stage is represented in `kuna_stages.cc`.
- **Exposures**: sub-stage surfaces available (new this session vs routed pre-existing).
- **Issues fixed**: PHADE issues addressed through this stage this session.
- **LATENT**: decisions still without an override surface (the backlog).
- **Assessment**: honest state of the stage.

---

## P0 — Knowledge & Configuration Plane

- **Registry**: `database`/`override`/`options` promoted from INFRA to P0 in
  `STAGE_MAPPING.md` §0 — they are the plane's artifact. The six pipeline variants
  (`buildDefaultGroups`) are P0 data, listed by `pipeline list`.
- **Exposures (new)**: `kassert <stage> <substage> <args> [hard|hint]` — the uniform
  typed assertion writer of stage-model §12, dispatching to Override/Symbol-DB/
  FuncProto/options; `kassert list` reports each assertion's strength and computed
  minimal rewind scope (observable half of roadmap #2 — scope is *reported*, Ghidra
  still re-runs whole-function). `pipeline <variant>` runs a named reduced pipeline as
  a sub-query with guaranteed root-action restore (mechanism c′ at the console —
  roadmap #3). `restarts` exposes mechanism (c) trigger reasons (roadmap #5).
- **Issues fixed**: — (P0 is the intervention plane; issues land in S2–S9).
- **LATENT**: confidence-graded admission for agent-proposed facts (HARD/HINT is
  caller-chosen, not graded); kassert records strength but nothing consumes HINT
  differently yet.
- **Assessment**: the plane now has a *name and a uniform surface* in the console;
  durability semantics (survive restarts) were already real in upstream's stores.

## S1 — Image & Code Partition

- **Registry**: loaders (`loadimage*`, `*_arch`) tagged S1; `fixateglobals` group
  (ActionMapGlobals — the C++ side's late write-back into the global map) tagged S1.
- **Exposures (routed)**: `option setlanguage` (environment binding), `global
  add/remove`, `readonly`, `option readonly` (code/data partition annotations).
- **Issues fixed**: — .
- **LATENT**: everything interesting — the partition arrives from the client
  (stage-model §5 honesty note).
- **Assessment**: thin by design; deliberately not re-grown into Loader/Decode.

## S2 — Flow & Op-Graph Recovery

- **Registry**: the largest file set (decode engine + IR containers + flow + jump
  tables); groups `deadcontrolflow`, `switchnorm`, `unreachable`, `segment` tagged S2.
- **Exposures (routed through kassert)**: `kassert S2 switch-model <addr>`
  (multistage hint), `kassert S2 flow-classification <addr> <type>` (flow override);
  decode-error policy and context options routed in the registry (`stage map`).
- **Restart observability (new)**: the multistage jump-table restart (edge 2) is now
  recorded and reported — the previously-silent switchmulti double-decompile is the
  testcase (`tests/stages/kuna-restarts.xml`).
- **Issues fixed**: TBD-PHASE2.
- **LATENT**: reachability pruning at user level; per-table switch-model *choice*
  (override exists; model selection among Basic/Basic2 does not).
- **Assessment**: richest pre-existing assertion surface; kuna's work here was naming
  and routing, plus making its feedback edge visible.

## S3 — Definition Web (Band B)

- **Registry**: `analysis` (the ~110-rule pool), `subvar`, `conditionalexe`,
  `nodejoin`, `canonicalcompare` (kuna) tagged S3; `heritage` file is S3's engine.
- **Exposures**: `kassert S3 dead-definition-gate <space> <delay>` (the deadcodedelay
  override — edge 3's surviving hint); `kassert S3 simplification-quiescence <rule>
  <on|off>` (per-rule toggle); `kassert S3 comparison-canonicalization
  canonical|original` (GH-558).
- **Restart observability (new)**: deadcode-delay bumps (and suppressed bumps) are
  recorded with their space.
- **Issues fixed**: GH-558 (prior session); TBD-PHASE2.
- **LATENT**: heritage staging per space; phi/range granularity (heritage.cc:2610).
- **Assessment**: the GH-558 home stage; the simplification-quiescence sub-stage is
  where new-rule-option fixes land.

## S4 — Call & Prototype Model (Band B)

- **Registry**: `protorecovery[_a,_b]`, `noproto`, `deindirect`, `fixateproto`,
  `returnsplit`, `siganalysis` tagged S4; `fspec`/`modelrules` files.
- **Exposures**: `kassert S4 prototype-source lock|unlock` (FuncProto locks);
  `kassert S4 extrapop <value>`; `override prototype` and `parse line` routed in the
  registry.
- **Restart observability (new)**: late-prototype restarts (`deindirect`/`forceSet`,
  edge 5) recorded with the call site.
- **Issues fixed**: TBD-PHASE2.
- **LATENT**: trial finalization budget (`maxpass`); ABI trust as a per-program
  assertion; interprocedural regime.
- **Assessment**: TBD-PHASE2.

## S5 — Value & Type Facts (Band B)

- **Registry**: `typerecovery`, `splitcopy`, `splitpointer`, `constsequence`,
  `bitfields`, `doubleload`, `doubleprecis`, `floatprecision`, `deadcode` (straddler)
  tagged S5; `type`/`typeop`/`unionresolve`/`rangeutil` files.
- **Exposures**: `kassert S5 type-propagation <symbol> <type>` (retype + typelock —
  the HARD stop in `propagateTypeEdge`); `map unionfacet` and `option
  inferconstptr`/`splitdatatype` routed in the registry.
- **Issues fixed**: TBD-PHASE2.
- **LATENT**: range/value-set assertions; solver budget (7-pass compile-time cap);
  union scoring weights.
- **Assessment**: TBD-PHASE2.

## S6 — Variable & Storage Model (Band B)

- **Registry**: `localrecovery`, `stackptrflow`, `stackvars`, `merge`, `dynamic`
  tagged S6; `varmap`/`variable`/`merge`/`cover`/`dynamic` files.
- **Exposures**: `kassert S6 merge-aggressiveness <symbol>` (isolate — blocks
  speculative merge); `map hash`/`option aliasblock` routed in the registry.
- **Issues fixed**: TBD-PHASE2.
- **LATENT**: per-variable explicit/implied (high-value readability knob); alias
  boundary policy.
- **Assessment**: TBD-PHASE2.

## S7 — Region Hierarchy

- **Registry**: `blockrecovery` tagged S7 (straddling S8); `blockaction` file.
- **Exposures**: `kassert S7 edge-virtualization <branch> <dest>` (per-edge
  force-goto — upstream's only per-edge structuring assertion, now stage-addressed).
- **Issues fixed**: TBD-PHASE2.
- **LATENT**: loop refinement / single-exit policy; re-identification batching.
- **Assessment**: TBD-PHASE2.

## S8 — Structured AST & Goto Quality

- **Registry**: `normalizebranches` tagged S8; schema/goto decisions live in
  `blockaction` (S7 straddler).
- **Exposures (new)**: the `quality` command — goto nodes (printed/fall-through),
  multi-goto edges, if-goto edges, unstructured total. This is the *measurement half*
  of quality-gated structuring (roadmap #4); rollback remains future work (design:
  `docs/stage-critique.md`).
- **Exposures (routed)**: `kassert S8 readability-rewrites on|off`
  (analyzeforloops); `option jumptablemax`.
- **Issues fixed**: TBD-PHASE2.
- **LATENT**: schema precedence; the quality *acceptance* policy itself.
- **Assessment**: TBD-PHASE2.

## S9 — Surface Rendering & Refinement

- **Registry**: `casts`, `presentcompare` (kuna) tagged S9; all print/emit files.
- **Exposures**: `kassert S9 naming-policy <old> <new>` (rename + namelock);
  `kassert S9 pointer-notation on|off` (GH-558); cast/format options routed in the
  registry.
- **Issues fixed**: GH-558 arraynotation (prior session); TBD-PHASE2.
- **LATENT**: per-expression cast policy; per-variable explicit/implied consumption.
- **Assessment**: the cheapest re-run scope and the densest issue cluster — TBD-PHASE2.

---

## Band B as a whole

No internal boundaries were added between S3–S6 — deliberately. The four artifacts
remain co-resident in `mainloop` (the per-decompiler scheduling regime is not part of
the model). What changed: each artifact's groups are named and stage-addressed in the
registry, each stage's assertions are routable through `kassert`, and the band's two
restart edges (3 and 5) are observable. Evidence on whether the band held up under
issue fixes: `docs/stage-critique.md`.
