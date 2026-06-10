# Stage implementation manifest

How kuna's code was modified to match the stage model (`docs/stages.md`,
`docs/stage-model.md`), stage by stage. This is the *session changelog against the
model* — the model itself is normative and lives in those documents; per-file placement
is `docs/stage-mapping.md` §0. Updated per work session; honest entries ("nothing changed")
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
  `docs/stage-mapping.md` §0 — they are the plane's artifact. The six pipeline variants
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
- **Issues fixed**: GH-8817 (`option v850indirectbranch` — V850 `jmp [reg]` CALLIND
  reclassified to BRANCHIND at flow time so switch recovery runs; the
  flow-classification sub-stage starving the switch-model sub-stage, both named S2
  catalog rows).
- **LATENT**: reachability pruning at user level; per-table switch-model *choice*
  (override exists; model selection among Basic/Basic2 does not).
- **Assessment**: richest pre-existing assertion surface; kuna's work here was naming
  and routing, plus making its feedback edge visible — and GH-8817 proved the catalog's
  adjacent-sub-stage structure describes real bugs.

## S3 — Definition Web (Band B)

- **Registry**: `analysis` (the ~110-rule pool), `subvar`, `conditionalexe`,
  `nodejoin`, `canonicalcompare` (kuna) tagged S3; `heritage` file is S3's engine.
- **Exposures**: `kassert S3 dead-definition-gate <space> <delay>` (the deadcodedelay
  override — edge 3's surviving hint); `kassert S3 simplification-quiescence <rule>
  <on|off>` (per-rule toggle); `kassert S3 comparison-canonicalization
  canonical|original` (GH-558).
- **Restart observability (new)**: deadcode-delay bumps (and suppressed bumps) are
  recorded with their space.
- **Issues fixed**: GH-558 (prior session); GH-1282 (`option booleanmask` —
  RuleBoolSignShift folds `(b<<k) s>>k` boolean masks past RuleLeftRight's byte-aligned
  gate); GH-7190 (`option ovlesssimplify` — V850 S/OV-flag compare idiom rewritten to
  the direct signed comparison).
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
- **Issues fixed**: GH-6990 (`option returnpair single` — buildReturnOutput keeps only
  the first return register, collapsing SPARC's passive o0:o1 over-claim).
- **LATENT**: trial finalization budget (`maxpass`); ABI trust as a per-program
  assertion; interprocedural regime.
- **Assessment**: the trial-finalization sub-stage fit GH-6990 cleanly; the ablation
  showed the flip breaks 3 upstream tests that genuinely need the pair — the textbook
  case for an assertion (context-dependent right answer) over a default change.

## S5 — Value & Type Facts (Band B)

- **Registry**: `typerecovery`, `splitcopy`, `splitpointer`, `constsequence`,
  `bitfields`, `doubleload`, `doubleprecis`, `floatprecision`, `deadcode` (straddler)
  tagged S5; `type`/`typeop`/`unionresolve`/`rangeutil` files.
- **Exposures**: `kassert S5 type-propagation <symbol> <type>` (retype + typelock —
  the HARD stop in `propagateTypeEdge`); `map unionfacet` and `option
  inferconstptr`/`splitdatatype` routed in the registry.
- **Issues fixed**: GH-8471 (`option thumbfuncptr`), GH-6930 (`option
  inferfuncentry`) — both const-pointer decisions whose symptoms surface at S9;
  GH-8913 (`option addcarrychain` — 6502 carry-chain wide-add recovery); GH-9230 +
  GH-1537 (`option memsetrecover` — constant-fill runs become builtin_memset; 1537 was
  fixed by 9230's option with zero new code).
- **LATENT**: range/value-set assertions; solver budget (7-pass compile-time cap);
  union scoring weights.
- **Assessment**: the load-bearing stage — five of the session's ten fixes. The
  const-pointer sub-stage redirected two fixes away from their S9 symptom site, and
  const-sequence generalized across issues. This is where the model earned its keep.

## S6 — Variable & Storage Model (Band B)

- **Registry**: `localrecovery`, `stackptrflow`, `stackvars`, `merge`, `dynamic`
  tagged S6; `varmap`/`variable`/`merge`/`cover`/`dynamic` files.
- **Exposures**: `kassert S6 merge-aggressiveness <symbol>` (isolate — blocks
  speculative merge); `map hash`/`option aliasblock` routed in the registry.
- **Issues fixed**: — (no viable S6 issue in the reproduced PHADE set).
- **LATENT**: per-variable explicit/implied (high-value readability knob); alias
  boundary policy.
- **Assessment**: fully routed (isolate via kassert, aliasblock, map hash) but
  unexercised by the issue program — dataset bias (representation-heavy), not a model
  gap; the surfaces are tested by the kuna-assert testcase.

## S7 — Region Hierarchy

- **Registry**: `blockrecovery` tagged S7 (straddling S8); `blockaction` file.
- **Exposures**: `kassert S7 edge-virtualization <branch> <dest>` (per-edge
  force-goto — upstream's only per-edge structuring assertion, now stage-addressed);
  `region tree` / `region blocks` / `region walk` — the **first standalone S7
  artifact**: an analysis-only nested region tree from the angr RegionIdentifier
  port (`kuna_regionid.cc`, provenance `region_identifier.py`/`graph_region.py`;
  `docs/regions.md`). The `loop-refinement` shaping *options* stay LATENT — these
  commands are the observable half.
- **Issues fixed**: — (GH-8748's blocker turned out to be an S7-grade limitation: see
  S8 below and the critique §7.4).
- **LATENT**: loop refinement / single-exit policy; re-identification batching; an
  INDIRECT-aware region-duplication primitive (the missing piece under GH-8748).
- **Assessment**: routed (per-edge force-goto via kassert) but unexercised; the
  GH-8748 negative result sharpened what S7 surgery upstream actually supports.

## S8 — Structured AST & Goto Quality

- **Registry**: `normalizebranches` tagged S8; schema/goto decisions live in
  `blockaction` (S7 straddler).
- **Exposures (new)**: the `quality` command — goto nodes (printed/fall-through),
  multi-goto edges, if-goto edges, unstructured total. This is the *measurement half*
  of quality-gated structuring (roadmap #4); rollback remains future work (design:
  `docs/stage-critique.md`).
- **Exposures (routed)**: `kassert S8 readability-rewrites on|off`
  (analyzeforloops); `option jumptablemax`.
- **Issues fixed**: GH-8748 partial — the decision (shared continuation block defeats
  the else-if `t_if` collapse) was located exactly, but the fix needs SSA out-block phi
  patching beyond `nodeSplit`'s envelope and was declined rather than shipped
  unverified (critique §7.4).
- **LATENT**: schema precedence; the quality *acceptance* policy itself; the
  speculative-edit half of quality gating (blocked on the S7 duplication primitive).
- **Assessment**: the measurement half (`quality`) landed and is regression-tested;
  the structuring-edit half is honestly out of reach without new SSA machinery.

## S9 — Surface Rendering & Refinement

- **Registry**: `casts`, `presentcompare` (kuna) tagged S9; all print/emit files.
- **Exposures**: `kassert S9 naming-policy <old> <new>` (rename + namelock);
  `kassert S9 pointer-notation on|off` (GH-558); cast/format options routed in the
  registry.
- **Issues fixed**: GH-558 arraynotation (prior session); GH-2786 (default-flip DIV-1
  — adjacent sign tokens parenthesized; 0/675 upstream churn).
- **LATENT**: per-expression cast policy; per-variable explicit/implied consumption.
- **Assessment**: the cheapest re-run scope, confirmed by ablation (S9 flips are
  text-only) — but NOT the densest decision site: four issues that *look* S9 (8471,
  6930, 9230, 1537) had their decisions in Band B. S9 is where bugs are seen, S5 is
  where they live.

---

## Band B as a whole

No internal boundaries were added between S3–S6 — deliberately. The four artifacts
remain co-resident in `mainloop` (the per-decompiler scheduling regime is not part of
the model). What changed: each artifact's groups are named and stage-addressed in the
registry, each stage's assertions are routable through `kassert`, and the band's two
restart edges (3 and 5) are observable. Evidence on whether the band held up under
issue fixes: `docs/stage-critique.md`.

---

## 2026-06-08 session addendum — all-remaining-issues wave

This session extended the stage model to the remaining 46 reproduced PHADE issues
(`docs/issue-coverage.md` is the full ledger). New stage-exposure sub-stage fixes
(option-gated, ElementIds 4010–4018):

- **S2** — `switchmodbound` (GH-9191, modulo/and-mask LOAD-table index bound in
  `JumpBasic::kunaTryModuloBoundTable`); `stackprobeloop` (GH-8017, normalize the
  stack pointer across a gcc stack-probe loop); `sparcstructret` (GH-6882, post-call
  `unimp` fall-through reclassification in `flow.cc`).
- **S3** — `flagcompare` (GH-1276/8777, fold `(b<<k) s< 0` and the N==V SBORROW
  signed-compare idioms — two rules under one option, cousins of `booleanmask`/
  `ovlesssimplify`); `arraystride` (GH-8724, strided-induction index recovery);
  `condexeplace` (GH-9203, keep ActionConditionalConst const-COPY out of loop blocks).
- **S6** — `dynamichashmax` (GH-8467, DynamicHash collision budget 8→16);
  `stackalias` (GH-8500, preserve a store-through-a-stack-pointer-alias);
  `inputvarnodeadjust` (GH-9218, absorb overlapping input varnodes instead of aborting).

Plus 17 SLEIGH spec-fixes (no option; decode/lift corrections across HCS12, 8051, x86,
ARM, avr8, V850, 8085, RISC-V, PowerPC). DIV-3 flipped the six corpus-clean
non-destructive options default-on (ablation: 0/675). The three destructive options
(`switchmodbound`, `stackalias`, `sparcstructret`) remain opt-in.
