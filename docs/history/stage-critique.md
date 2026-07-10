> **ARCHIVED (2026-07).** Empirical critique of the stage model (ablations, navigation scoring); retained as evidence. Retained for provenance; paths and terminology inside are frozen pre-rename (stage = phase, S-codes = P-codes).

# Critiquing the kuna stage model — empirical evidence

Does the stage model (`docs/stages.md`, `docs/stage-model.md`) actually help fix real
decompiler issues, or is it a taxonomy that looks tidy and does nothing? This document
answers that from session evidence, not from first principles. The data come from the
2026-06 session that physicalized the model (registry, `kassert`, observability) and
fixed a batch of PHADE-dataset issues through it.

The honest bottom line up front: the model earned its keep as a **navigation and
routing** tool (it reliably tells you *where* a fix lives, and that location is a real
program point), but it did **not** reduce the difficulty of the hard fixes — a missed
simplification is still a missed simplification whether or not you have a name for the
sub-stage it belongs to. Its biggest concrete win is the **gated-exposure pattern**: by
giving every decision a default-off P0 assertion, fixes ship without risking upstream
parity, which is what made a batch of cross-architecture fixes safe to land in one
session.

---

## 1. Per-issue stage-fit

For each fixed issue: the stage predicted by the §13 symptom→sub-stage navigation table,
the stage(s) actually touched, how many distinct code sites the decision lived at (the
GH-558 "smear" metric — GH-558 itself was a 3-site decision), whether the fit was clean,
and the exposure mechanism.

| Issue | Symptom | Predicted (§13) | Actual stage / sub-stage | Sites | Clean? | Exposure |
|---|---|---|---|---|---|---|
| GH-2786 | `--x` invalid C | S9 literal/format | S9 surface (`parentheses`) | 1 | ✓ clean | default-flip |
| GH-8471 | Thumb funcptr → hex | S9 (surfaces there) | **S5** const-pointer (`RulePtrsubUndo`) | 1 | ✗ *predicted S9, lived in S5* | option |
| GH-6930 | funcptr → image-base literal | S9 (surfaces there) | **S5** const-pointer (`isPointer`) | 1 | ✗ *predicted S9, lived in S5* | option |
| GH-6990 | `CONCAT44` 64-bit void return | S4 prototype source | S4 trial finalization (`buildReturnOutput`) | 1 | ✓ clean | option |
| GH-1282 | 8051 `(b<<7)>>7` noise | S3 simplification | S3 simplification-quiescence (`RuleLeftRight` gate) | 1 | ✓ clean | new-rule option |
| GH-8913 | 6502 carry chain | S5 value facts | S5/S3 (PIECE matcher in pool) | 1 site, **2 producer forms** (raw `INT_CARRY` + `RuleCarryElim`-folded) | ✓ (smear-lite: one decision, two upstream forms) | new-rule option |
| GH-7190 | V850 flag compare | S3 simplification | S3 simplification-quiescence | 1 | ✓ clean | new-rule option |
| GH-8748 | else{if} not else-if | S8 readability | S8 schema (shared-tail blocks `t_if` collapse) | 1 | ✓ located — **fix declined** (see §7.4) | (partial) |
| GH-9230 | unrolled memset | S5 constsequence | S5 const-sequence (`constseq` family) | 1 site, 2 blockers | ✓ clean | new-rule option |
| GH-8817 | V850 jmp[reg] switch | S2 switch model | **S2 flow-classification** (one sub-stage earlier: the CALLIND starves switch-model) | 1 | ✓ (routed within S2) | option |
| GH-1537 | SIMD memset on global | S5 constsequence | S5 const-sequence — **fixed by GH-9230's existing option, zero new code** | 0 | ✓ routing | routing |

**Finding (so far): the "two-artifact" issues are the model's sharpest insight.** GH-8471
and GH-6930 both *appear* at S9 (a wrong-looking literal in the rendered C) but the
decision that produced them is at S5 (constant→pointer inference in Band B). The naive
"fix it where you see it" instinct — which the original triage followed, pointing at
`printc.cc` — was wrong for both; the symbolic information is already destroyed before
S9 runs. The stage model's insistence that **a symptom's stage ≠ its decision's stage**,
mediated by feedback edges, is exactly what redirected both fixes to the real site. This
is the single most useful thing the model did this session.

The flip side: the §13 navigation table, as written, routes by *symptom* and so points
at S9 for these — it needs a "but check the upstream Band-B decision" caveat for
constant/pointer rendering. Recorded as a model gap below.

---

## 2. Ablation experiments

Protocol (from `docs/prototypes/gh558.md`): force a sub-stage's exposed option to its
non-default value across the 204 unit + 675 datatest upstream assertions, count how many
change, and classify each change as text-only (presentation) vs analysis-perturbing.
This quantifies how independent the decision is and whether it is purely cosmetic.

| Option | non-default value | changed upstream assertions | classification |
|---|---|---|---|
| `compareform` | original | 12 / 675 (gh558 baseline) | text-only |
| (remove `canonicalcompare`) | — | 9 / 675 (gh558 baseline) | analysis |
| `arraynotation` | on | **10 / 675** (Heap/Stack string ×6, Pointer Compare, Pointer-to-array ×2, Relative pointers) | text-only (pointer-arith renderings) |
| `inferfuncentry` | on | **0 / 675** | invisible on corpus (no upstream test has a function at a single-bit address — guard is narrowly scoped) |
| `thumbfuncptr` | on | **0 / 675** | invisible on corpus (no mode-bit funcptrs in upstream tests) |
| `returnpair` | single | **3 / 675** (Mixed float/int #1/#2/#6) | **analysis** — genuine multi-register returns get truncated |
| `booleanmask` | on | **0 / 675** | corpus-invisible (no non-byte-aligned boolean shift idiom in upstream tests) |
| `addcarrychain` | on | **0 / 675** | corpus-invisible (no 8-bit carry-chain adds in upstream tests) |
| `ovlesssimplify` | on | **0 / 675** | corpus-invisible (no V850 OV-flag idiom in upstream tests) |
| `v850indirectbranch` | on | **pathological — run diverges** | the "V850 jmp shape" (CALLIND of a named register) matches *every* register-indirect call on other arches; the global flip sends each into jump-table recovery sub-queries. Correct as a per-program V850 assertion; meaningless (and unbounded) as a global default — measured the hard way |
| `memsetrecover` | on | **1 / 675** (Stack string #6) | analysis — a constant fill the corpus expects in string form becomes a memset |

Measured 2026-06-07 (flip default → rebuild → full suite → revert). The pattern the
model predicts holds, with one twist worth keeping:
- The S9 options perturb **text-only** sets (`arraynotation` 10, `compareform` 12) —
  presentation decisions whose flip is always safe in meaning, never in diff.
- Two Band-B options are **corpus-invisible** (0/675): their guards are scoped to
  constructs (single-bit function entries, Thumb LSB pointers) that the upstream corpus
  simply lacks. Narrow scoping is what makes them safe even as defaults — but they stay
  opt-in per the divergence policy.
- `returnpair single` is the instructive one: **3 upstream tests legitimately need the
  o0:o1 join** (real multi-register float/int returns). The "right" answer is genuinely
  context-dependent — pair for real wide returns, single for SPARC void functions. That
  is the model's definition of a sub-stage decision (different choices correct in
  different contexts ⇒ expose as assertion, never flip globally), demonstrated
  empirically: the global flip breaks exactly the tests where the other context applies.
- `v850indirectbranch` is the cautionary one: as a *global* flip it diverges (every
  x86 register-indirect call gets reclassified and sent into jump-table recovery). The
  flag's correctness depends on the program's architecture — i.e. it is a P0
  per-program assertion by *nature*, not just by policy. The ablation protocol itself
  surfaced this; a default-flip review process that only counted changed assertions
  would never have terminated.

---

## 3. Coverage gaps — which stages attracted nothing

Predicted before the session (so this is a prediction the evidence can falsify, not a
post-hoc rationalization):
- **S1 (Image & Code Partition): nothing, by design.** The C++ decompiler receives the
  partition from its client; PHADE issues that are truly S1 (loader, code/data split)
  cannot reproduce in the harness at all. Confirmed: zero S1 issues were viable.
- **S7 (Region Hierarchy): nothing landed**, though `force-goto` (the per-edge S7
  assertion) is now routed through `kassert`. GH-8748 is S8 (schema/readability), not
  S7. The dataset is representation-heavy and light on "won't structure" cases.
- **The Band B middle (S3/S4/S5/S6) attracted the most**, which matches the model's
  claim that this is where the substantive decisions — and thus the bugs — concentrate.

Interpretation: the gaps are dataset bias (PHADE is dominated by rendering/representation
complaints and arch-specific lifting), not model overfit. The model predicted S1 would be
thin and it was; it did not invent stages that the issues then failed to populate (every
S1–S9 box has a real artifact and a real assertion surface, even where no issue landed).

---

## 4. Band B — did the "no internal boundary" property matter?

The model's most contested claim is that S3–S6 have no fixed first-pass order (they
co-iterate to a mutual fixed point in Ghidra's `mainloop`). Evidence this session:
- GH-8471 and GH-6990 both required placing the fix at a point where Band-B analysis had
  already partly run (type recovery started; return trials marked active) but not
  finished — neither could be expressed as "run S5, then do X." GH-8471's guard sits
  inside `RulePtrsubUndo`, which fires repeatedly as the rule pool iterates; GH-6990's
  sits in `buildReturnOutput`, which runs after return-trial quiescence. Neither has a
  clean "stage N" home — they live *inside the band's fixed point*, exactly as the model
  says. A linear-stage model would have mis-placed both.

---

## 5. Registry usefulness for an LLM operator

Score: for each issue, did the §13 navigation + the `stage map` registry route an agent
to the correct fix site without a wrong turn?

- GH-2786, GH-6990, GH-1282, GH-7190, GH-9230, GH-1537, GH-8748: §13/catalog routed
  correctly to the owning sub-stage — 7 clean routes.
- GH-8817: routed to "switch model"; the actual decision was flow-classification, the
  *adjacent sub-stage in the same stage* — the catalog's S2 row structure caught it
  (the mis-classification *starves* switch-model; both are named S2 decisions).
  Half-credit.
- GH-8471, GH-6930: §13 routed to S9 (the symptom stage); the *correct* site was S5.
  The registry named the right surfaces (const-pointer is in the catalog as S5) but the
  symptom-first table pointed one stage too late.

**Score: 7 clean + 1 within-stage + 2 one-stage-late, of 10.** The registry's
group→stage and surface→stage maps (`stage map`) were accurate in all cases; every
mis-route came from the symptom-first §13 table, and both failures share one cause (the
two-artifact pattern). Actionable fix recorded in §7.1.

---

## 6. Verdict per stage

Final (10 issues fixed: GH-2786, 8471, 6930, 6990, 1282, 7190, 8817, 8913, 9230, 1537;
plus the V850.cspec unblock and the GH-8748 negative result):

| Stage | Verdict | Basis |
|---|---|---|
| P0 | USEFUL | the gated-assertion pattern made every fix parity-safe; `kassert`/`restarts`/`pipeline` give the plane a real surface; ten new assertions registered |
| S1 | THIN (as designed) | no viable issues; partition is client-side — predicted and confirmed |
| S2 | USEFUL | GH-8817 (flow-classification starving switch-model — the catalog's two adjacent S2 rows described the bug exactly); restart edge observable (switchmulti) |
| S3 | USEFUL | GH-1282 + GH-7190 landed at simplification-quiescence; it is the natural home for new-rule fixes |
| S4 | USEFUL | GH-6990 fit cleanly at trial finalization |
| S5 | USEFUL (load-bearing) | five fixes (8471, 6930, 8913, 9230, 1537); const-pointer caught two fixes the symptom-stage would have misplaced; const-sequence generalized across issues (1537 via 9230's option, zero code) |
| S6 | UNEXERCISED | routed (isolate/aliasblock/kassert) but no issue landed — dataset bias, not model failure |
| S7 | UNEXERCISED (and informative) | force-goto routed; GH-8748's blocker (un-clonable INDIRECT tails) is really an S7 region-surgery limitation surfacing under S8 — see §7.4 |
| S8 | PARTIAL | GH-8748 correctly located but the fix needs SSA out-block phi patching beyond `nodeSplit`'s envelope; the `quality` metric landed as the measurement half |
| S9 | USEFUL | GH-2786 default-flip; cheapest re-run scope confirmed (ablation: S9 flips are text-only) |

---

## 7. Model gaps found this session (fed back into the model)

1. **§13 symptom→sub-stage is symptom-first and mis-routes two-artifact decisions.** A
   wrong literal/pointer in the C output (S9 symptom) is often an S5 const-pointer
   decision destroyed before rendering. The navigation table should carry an explicit
   "if the symbolic information is *gone* rather than *mis-rendered*, the decision is the
   upstream Band-B producer, not S9." (GH-8471, GH-6930.)
2. **No sub-stage for return-value register joining.** GH-6990 mapped to S4 "trial
   finalization" but the specific decision (join o0:o1 into a wide return) deserves its
   own catalog row. Added implicitly via the `returnpair` exposure.
3. **The gated-exposure pattern should be promoted from a GH-558 footnote to a
   first-class model element.** Every fix this session used it; it is the mechanism that
   makes the model *safe to apply incrementally* (default-off ⇒ PARITY OK ⇒ no
   upstream-sync risk). This is arguably the model's most practically important property
   and it was under-documented. The strongest evidence is GH-1537: an exposed sub-stage
   assertion (`memsetrecover`, built for GH-9230's stack case) fixed a *second issue* —
   global destination, RIP-relative SIMD stores — with zero additional code. Exposed
   decisions generalize; point fixes do not.
4. **Negative result (GH-8748): tail duplication is blocked by un-clonable INDIRECTs.**
   The shared continuation block that defeats else-if collapsing carries 8 live-out
   store-clobber INDIRECT phis; `Funcdata::nodeSplit`/`CloneBlockOps` throws
   "Can't clone INDIRECTs", and duplicating past it requires synthesizing out-block
   MULTIEQUALs — SSA surgery that miscompiles silently if subtly wrong. The honest scope
   of any future quality-gated structuring (roadmap #4 / §8b) must include this: the
   speculative-edit half needs an *INDIRECT-aware* region duplication primitive that
   upstream does not have. The measurement half (`quality`) and the located decision
   point (S8 schema collapse defeated by a `t_ls` else-child) are both real and landed.
5. **Engineering hazard found: option registration silently collides without
   ElementIds.** `OptionDatabase::registerOption` keys on `ElementId::find(name)`, which
   returns `ELEM_UNKNOWN` for unregistered names — two kuna options without ElementIds
   overwrite each other and the console dispatches the wrong `apply()`. Every kuna
   option now allocates a 4000+ ElementId (4000–4009 used); this is a convention the
   model's "exposure = option" pattern must carry (recorded in CLAUDE.md conventions
   via UPSTREAM.md).

---

## 8. Deferred-design notes (the observable-half roadmap items)

Two roadmap items (`docs/stage-model.md` §16) were shipped only in their *observable*
half this session; their full designs are recorded here for a follow-up.

### 8a. Scoped re-run (roadmap #2)

`kassert list` already computes and reports each assertion's minimal rewind stage (§12).
The missing half is *acting* on it: today every assertion triggers Ghidra's
whole-function `clearAnalysis`. Design for the enacting half:
- Tag each `Override`/lock type with its earliest invalidated artifact (the `rewind`
  field already in the registry's `KunaSubStage`).
- Replace `clearAnalysis(fd)` at the assertion's re-decompile with a scoped reset that
  re-runs only the action sub-range from the rewind stage forward, reusing the
  `ActionGroupList` filter machinery (the named reduced pipelines prove the engine can
  run an action subset). The hard part is restoring the pre-stage `Funcdata` snapshot;
  the cheap first version is S9-only re-render (angr's `clears_cache=False` analog),
  which needs no snapshot because S9 is feed-forward.
- Risk: Band-B assertions (typelock, proto) genuinely need a whole-function rewind in
  Ghidra because types are in-loop, not a terminal solver — scoped re-run is only sound
  for S7+ (force-goto), S9 (format/name), and is the reason angr (terminal types) can do
  it cheaply and Ghidra cannot. The honest scope of this item is "S7–S9 assertions get
  cheap re-run; Band-B assertions stay whole-function until/unless types are lifted out
  of the loop."

### 8b. Quality-gated structuring with rollback (roadmap #4)

The `quality` command already measures the goto/unstructured count over `sblocks`. The
missing half is the SAILR accept-policy: speculatively apply a structuring
de-optimization, re-measure, and roll back if the goto count grew or a for-loop
degraded. Design:
- Snapshot the region graph (`sblocks`) before a candidate edit.
- Apply the edit (e.g. a tail-duplication like GH-8748's `splittail`, or a guarded
  goto→structured rewrite), re-run `ActionBlockStructure`, re-measure via the existing
  `quality` walker.
- Accept iff `gotos_after <= gotos_before` (and no for→while regression); else restore
  the snapshot. This is angr's `StructuringOptimizationPass` policy ported around
  Ghidra's `CollapseStructure`/`ActionBlockStructure`.
- The `quality` metric shipped this session is precisely the measurement primitive this
  policy needs; the rollback harness is the remaining work.

---

## 2026-06-08 — all-remaining-issues wave (per-issue fit + DIV-3 ablation)

- **Triage of 46 issues** (`docs/issue-coverage.md`): 20 predicted spec-fix, 12
  stage-exposure, 13 not-viable, 1 already-correct. Live reproduction corrected several
  (766/5666/7332 spec→not-viable once the fix proved to need a whole new context-register /
  address model / Java-analyzer naming; 3723 already-correct under the right language).
- **Stage-fit**: the new fixes landed where the §13 symptom→stage navigation predicts,
  with two instructive multi-stage cases — `sparcstructret` (S2 flow + S4 prototype) and
  `stackprobeloop` (S2 op-graph + S6 stack frame) — reinforcing that Band-B/flow decisions
  often surface as S9 text. `flagcompare` reused the exact S3 simplification-rule shape of
  the prior `booleanmask`/`ovlesssimplify` fixes (one option, two De-Morgan-dual idioms).
- **DIV-3 ablation** (all six non-destructive options forced on, full 204+675 suite):
  **0 of 675** upstream assertions changed — their fix patterns (flag-compare idioms,
  stack-probe loops, dense-hash collisions, strided indices, loop-block const-COPY,
  overlapping input varnodes) do not occur in the upstream corpus, so the flip is
  corpus-invisible. Flipped default-on.
- **Deliberately opt-in** (ablation-clean on the corpus but context-destructive by
  construction): `switchmodbound` (may over-bound an unrelated indirect jump),
  `stackalias` (relaxes a dead-store race), `sparcstructret` (reclassifies a real trap on
  non-SPARC). These match the `returnpair`/`v850indirectbranch` precedent: correct
  per-program, unsafe as a global default.
- **Negative results** (honest non-fits): 6674 (V850 free-register switch dispatch — a
  recovery path `switchmodbound` does not reach) and 6858 (stripped-PIE main — distinct
  from 8017's probe loop; the committed `stackprobeloop` option does not change its output).
