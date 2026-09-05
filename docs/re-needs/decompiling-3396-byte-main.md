---
need_id: decompiling-3396-byte-main
title: Decompiling the 3396-byte main function takes about 68 seconds
track: perf
status: open
severity: major
probe_id: p-a9ebf39dace0
acceptance_id: a-53d616afcb6a
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [69a3822f7b3cc38c80464da4]
rounds: [2]
first_seen_round: 2
attempts: 4
covered_by_option: null
touches: [decompiler/crates/kuna-decomp, decompiler/crates/kuna-console, decompiler/crates/kuna-decomp/phases.toml, decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs, docs/options.md, docs/history.md, tests/stages]
scope: small
regression_of: null
pr: "396"
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Interactive decompilation of the program's main function, sub_140023350, fast enough to guide iterative reverse engineering

> **Decompiling the 3396-byte main function takes about 68 seconds** (major, `69a3822f7b3cc38c80464da4`)
> The command produced no output for roughly 68 seconds, then emitted about 30 KB of highly noisy pseudocode. The noisy-output aspect is the already-filed large-function-malformed-output issue; this observation is specifically about latency.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_140023350"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "wall_ms": {
      "stat": "median",
      "gt": 30000
    }
  },
  "target": {
    "binary_rel": "bin/nikos_crack_me.exe",
    "binary_sha256": "bcfacd743bc607beff4a5b9616d3f9d0335551c07233e07a0a01bf14f0c6e869",
    "binary_size": 235725,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_140023350"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "wall_ms": {
      "stat": "median",
      "lt": 10000
    }
  },
  "target": {
    "binary_rel": "bin/nikos_crack_me.exe",
    "binary_sha256": "bcfacd743bc607beff4a5b9616d3f9d0335551c07233e07a0a01bf14f0c6e869",
    "binary_size": 235725,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The function's extensive opaque arithmetic and indirect calls cause one or more analysis passes to scale poorly.

## Refutation

**Hypothesis partly refuted (round 2 builder, PR for `feat/re-decompiling-3396-byte-main`).**
The symptom stands — 71.5 s median reproduced for `kuna decompile <bin> sub_140023350`.
The diagnosis does not: it is not an analysis pass and not "opaque arithmetic and
indirect calls". gdb-sampled profiling put 53% of wall time in a single leaf frame,
`FlowInfo::xref_control_flow`, i.e. the **lifter**, before any analysis runs. The dead
p-code list is a doubly-linked list, but `dead_next`, `dead_tail`,
`delete_remaining_ops` and `Funcdata::op_target` re-derived position by scanning it,
making op generation O(N²) in op count. This function is unusual only in op count
(48,169 raw ops, 37,710 INDIRECTs across 365 call sites), which is exactly what the
quadratic squares — so the same defect is present in every function and merely invisible
on small ones.

Fixed to O(1) (output byte-identical): **71.46 s → 19.42 s median, −72.8%**. The probe
arm (`> 30 s`) no longer holds. **The acceptance (`< 10 s`) is still not met** and the
need should stay open: after the fix the profile is flat (p6 merge 27.6%, p3 heritage
26.3%, rule pool 19.8%, jump-table sub-decompilation 17.4%, p9 emit 13.7%), so the
remaining 48% is a campaign, not a fix. The two ranked leads —
`kuna decompile` following flow twice (~18%), and the per-table jump-table partial clone
(~20%, implemented then reverted because `option unrolledguard` depends on it) — are
recorded with evidence in `docs/features/decompiling-3396-byte-main/record.json`.

**Attempt 2 (round 2 wave 8, branch `feat/re-decompiling-3396-w8`).** The symptom
still stands and the acceptance is still unmet: **14,437 ms** against the
`< 10,000 ms` bar. What attempt 2 removed is the *second* flow follow — `kuna
decompile` drives the console with `load function` then `decompile`, and each of
those followed the same function's flow from scratch, jump-table
sub-decompilation included. Interleaved 7-pair A/B: **19,703 ms -> 15,317 ms
median, -22.3%** (min -24.7%, paired mean -20.6% +/- 7.8, ~7 sigma), output
byte-identical over 218 whole-surface decompiles across 20 binaries (203 of which
took the new fast path).

Two things attempt 3 should inherit rather than rediscover:

- **The residue is confirmed flat, on a fresh profile.** `stage_jump_table` 29.2%
  (the only block over 20%, and semantically load-bearing — `unrolledguard`
  depends on each table's clone re-cloning its siblings), p6 merge 18.2%, rule
  pool 10.4%, heritage 10.1%, symbol-container lookups ~11%, infertypes 6.0%,
  dead code 5.8%. One attempt-1 lead does NOT survive re-measurement: `bb_ops` is
  ~2.2% self spread over five callers, not ~10%.
- **Measuring this need is harder than fixing it.** `kuna decompile` (without
  `--json`) forks `decomp_dbg` and pipes both its streams, so a stderr marker is
  invisible and gdb does not follow the child — profile the in-process `--json`
  path and instrument to a file. And this box runs sibling builders at load
  average 8-24, where the same binary measures 14.6 s and 18.9 s hours apart; only
  an interleaved paired A/B means anything.

**Attempt 3 (round 2 wave 12, branch `feat/re-decompiling-3396-byte-main`).** The symptom still
stands and the acceptance is still unmet: **12,437 ms** against the `< 10,000 ms` bar. What
attempt 3 removed is not one mechanism but four, three of which are the same shape: kuna
**re-derives, per query, a fact upstream reads off a cached pointer**. `Merge::mergeTestAdjacent`
reads `high->getSymbol()` and its isolated bit; kuna's merged tree does not paint SymbolEntries
onto Varnodes before the merge group, so both reads re-ran a `findContainer` containment query
**per member Varnode, per candidate pair** — **26,243,952 queries** in one decompile of this
function. Interleaved 4-pair A/B: **14,565 ms -> 12,184 ms median, -16.3%** (and -16.2% re-measured on the rebased tree) (every pair a win,
paired mean -15.8% +/- 1.5), output byte-identical over 440 whole-surface decompiles across 55
binary-arms and 8 arch/format combinations.

Three things attempt 4 should inherit rather than rediscover:

- **Switch instruments when the sampling profile goes flat.** Attempt 2's 1,053-sample
  profile could not tell that its "symbol-container lookups ~11%" and its "p6 merge
  18.2%" were the *same* cost, and no sampler can report "26 M calls". A name-keyed
  `Instant` timer wrapped around `self.apply()` in `Action::perform` gives an exact
  per-Action profile of the whole pipeline in one run, and 16 indexed guard slots
  resolve any leaf from there. Also: gdb-as-parent sampling **no longer works** —
  `continue` from a stop-event handler needs `gdb.post_event`, and gdb then segfaults
  ("This is a bug, please report it"); `-nx` is required regardless or every sample is
  the box's GEF banner.
- **`Heritage::guard_calls` is NOT a prototype-query hot spot** — that was attempt 3's
  planned next fix and the measurement killed it. Of its 971 ms loop body, `has_effect`
  is ~30 ms, `characterize_as_output` 19.7 ms and `characterize_as_input_param` 9.8 ms.
  The cost is INDIRECT op and Varnode **construction**. Do not build a FuncProto memo.
- **The residue after attempt 3, measured on this branch:** `stage_jump_table` 31% (the
  per-table re-clone is load-bearing for `option unrolledguard` and sharing it needs its
  own option — the `phases.toml` lease this round did not have), heritage 22.9%,
  `ActionDeadCode` 14.9%, `oppool1` 14.3%, merge now 7.6%. Nothing else is over 7%.

**Attempt 4 (round 2 wave 14, branch `feat/re-decompiling-3396-byte-main`).** The symptom
still stands and the acceptance is still unmet: **11,225 ms** median against the
`< 10,000 ms` bar. Attempt 4's contribution is a *diagnosis*, and the fix is the smaller
half of it. A per-Action exclusive timer, 32 indexed guard slots and — decisively — a
per-Action **counter** of varnode/op creations say that one decompile of this function
performs **1,677,343 varnode creations, 1,523,008 destructions, 1,110,157 xref/make_free
re-keyings and 1,106,775 op creations/destructions**: about **9.5 M ordered-container
mutations, 3.6 s of an 11.8 s run**. The residue is not a pass that scales badly. The
function is IR-volume-bound and every unit of that volume pays two `BTreeMap` insertions
and later two removals. Four byte-identical cuts follow from that (tree keys stop
carrying `Rc`s and become `Copy`; `xref` takes one descent instead of two;
`LocationMap::add` reaches its candidate in one descent and returns the size its caller
was re-looking up — 469.6 ms over 1,427,964 calls; `rename_recurse` stops snapshotting
whole successor blocks). Interleaved 8-pair A/B: **12,055 ms -> 11,225 ms median, -6.9%**
(paired mean -7.3% +/- 5.0, every pair a win, 4.2 sigma), output byte-identical over
whole-surface `decompile-all` across the fixture corpus and the round's probe binaries,
0 diffs.

Attempt 5 should inherit three things and re-derive none of them:

- **The bar is reachable, once, by one identified change: share the jump-table
  partial.** kuna runs the partial sub-decompilation once *per table*; C++ runs it once
  per *function* (`stageJumpTable` guards the clone and the reduced pipeline behind
  `if (!partial.isJumptableRecoveryOn())`). Here that is two partials — **3,077 ms of
  action time plus 464 ms of cloning** — so sharing removes ~1.7 s, **~15%**, which with
  attempt 4's -7% lands the witness near **9.5 s, under the bar**. It changes which
  tables recover (`unrolledguard` fires on this very function) and therefore needs a
  `phases.toml` option row. **Dispatch attempt 5 WITH the phases.toml / catalog / DIV /
  stages-corpus leases** — those were held by a live sibling all of wave 14, which is the
  only reason this is still open. This is a scoped feature PR, not another profiling run.
- **Three leads are refuted by direct measurement.** `new_varnode`'s fresh
  `Rc<Datatype>` per varnode: 111 ms over 1,304,246 calls (and memoizing it aliases
  pointers that `Rc::ptr_eq` type comparisons can see). `setVarnodeProperties`' two scope
  containment queries: 86.5 ms over 539,462 calls. `guard_calls` outside INDIRECT
  construction: 97 ms over 546,144 iterations — all 913 ms of it is the 192,528 INDIRECT
  constructions, i.e. tree mutations again.
- **The measurement harness.** `perf` is blocked on this box
  (`perf_event_paranoid = 4`) and there is no valgrind, so instrumentation is the only
  option; the per-Action timer separates the three decompiles the command runs (main +
  two jump-table partials) for free, and the creation counter is what makes "flat"
  legible. Guard overhead is real and must be subtracted — 5.4 M guard pairs moved an
  11.8 s run to 13.3 s.

## Reference

_none recorded_

## Instances

- `69a3822f7b3cc38c80464da4` (round 2, tester t-r2-69a3822f)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): touches [] -> kuna-decomp; track perf and scope small CONFIRMED. An empty touches list declares no edit surface at all, which leaves the sibling-contract rule nothing to enforce. 68s on a 3396-byte function is a profiling task with a targeted fix, and perf-track needs hold no counter leases.
- round 2 BUILDER (b-r2-decompiling-3396) + captain B_DONE: **PR #380 merged and the need STAYS OPEN.** The fix is real and large -- `stop re-deriving dead-list position by scanning`, 71.46 s -> 19.42 s (-72.8%) on this witness with output byte-identical over 509 functions, verified green on merged main 6ce857c4 (four gates + catalog + 15/15 clitests). But acceptance `a-53d616afcb6a` asks for a median under 10,000 ms and the probe measured **18,762 ms** at 6ce857c4, so the `wall_ms` clause FAILS while `exit_code` passes. attempts -> 1, `pr: 380`, `closing_pr` stays null: only the acceptance probe may close a need.
- round 2 captain: **the probe is NOT relaxed, and that is the decision, not an oversight.** Re-cutting the bar to match what shipped would redefine `closed` as "a builder tried" instead of "the agent's task now works"; 10 s is the tester's interactive threshold, which is the thing the need is about. This is also not contention -- the builder's own interleaved A/B measured 18.44/23.35/21.79/19.42/18.84 s and its PR body says the bar is unmet in as many words. Do not re-run the probe hoping for a better number.
- round 2 captain, **brief for the next attempt (attempt 2)**: #380 names where the residue sits -- rule pool 19.8%, jump-table sub-decompilation 17.4%, p9 dead-code/emit 13.7%. There is no single quadratic left; closing this needs roughly another -50% spread across three phases, which is a materially harder job than attempt 1. So this need is dispatchable but should rank BEHIND needs no builder has attempted yet, and whoever takes it must be told up front that one attempt may not reach the bar. Hand them #380's profile as the starting point rather than letting them re-profile from scratch.
- round 2 B_PLAN (captain), **frontmatter correction: this need has had TWO attempts, not one.**
  `attempts: 1` / `pr: 380` were stale — attempt 2 shipped as **#385** (`e94a1a68`, follow the
  function's flow once, not twice) and its result is already written up in the prose above
  (14,437 ms against the `< 10,000 ms` bar), but the merge never wrote the fields because a
  `worker.sh` re-entry clobbered that builder's phase and no `apply-acceptance` ran for it
  (nothing closed). Set to `attempts: 2`, `pr: 385`; `closing_pr` stays null. **A dispatch from
  here is attempt 3.**
- round 2 B_PLAN (captain), **dispatch prep.** The stale local branch `feat/re-decompiling-3396-byte-main`
  (`679f4a70`, the captain-preserved salvage tree that became #385) was renamed to
  `wip/re-decompiling-3396-salvage-a2` and its worktree removed, so the name `spawn_builder` derives
  is free and attempt 3 gets a real attached branch off main rather than `worker.sh`'s silent
  detached fallback. Nothing was deleted; #385's content is in main via the squash.
- round 2 B_PLAN (captain), **brief for attempt 3 — inherit, do not re-profile.** Two attempts have
  already taken 71.46 s -> 19.42 s (#380) -> 14.44 s (#385), so the cheap quadratics are gone and the
  remaining ~31% is spread across the residue #380/#385 measured: rule pool, jump-table
  sub-decompilation, p9 dead-code/emit; `bb_ops` did NOT survive re-measurement and is not a lead.
  Read `docs/features/decompiling-3396-byte-main/record.json` first — it carries both attempts'
  profiles. Two harness facts that cost attempt 2 the most to learn and that attempt 3 must not
  rediscover: `kuna decompile` FORKS `decomp_dbg` and eats its stderr (profile `--json` and
  instrument to a file), and only an **interleaved paired A/B** survives this box's load — a
  sequential timeit here has reported +42% on byte-identical output. **This need may legitimately
  not close on this attempt**; a merged, measured, output-identical cut that still misses 10 s is a
  success by every rule this loop has, and the bar is not moved to meet it.
- round 2 B_PLAN (captain): `touches` widened to include `decompiler/crates/kuna-console` — #385
  landed mostly there (`ifacedecomp.rs`, `decompile_step.rs`) and the declared surface said only
  `kuna-decomp`. Adds no lease (perf holds none); it makes the sibling contract honest. Live sibling
  `b-r2-c-string-objects` is editing `kuna-decomp/src/p5_types/`, `p0_knowledge/`,
  `infra/architecture.rs` and `substrate/context.rs` — none of attempt 3's leads, but expect to
  rebase after it merges.
- round 2 B_DONE (captain): **attempt 3 (#393, `67182c7e`) MERGED and the need STAYS OPEN.**
  `pr` 385 -> 393; `attempts` stays 3 (the builder set it); `closing_pr` stays null. The captain's
  own quiet replay at reps=7 on merged main 8ff1b4ec: median **11,769 ms** (11,477-11,888) against
  the `< 10,000 ms` bar -- better than the 12,437 ms the PR reported, still short. Three merged,
  measured, output-identical attempts have now taken this witness 71.5 -> 19.4 -> 14.4 -> 11.8 s.
  **The bar is still not moved.** A dispatch from here is attempt 4, and per the builder prompt's
  two-failed-attempts rule the next tick should weigh a proposal-shaped dispatch rather than a
  fourth open-ended profiling run.
  Attempt 4 inherits, and must NOT re-buy: `stage_jump_table` 31% (2 tables; the per-table re-clone
  is load-bearing for `option unrolledguard`), heritage ~23%, oppool1 ~14%, ActionDeadCode ~14%,
  merge ~6% (was 1,331 ms before #393, now 3 ms). **One lead is REFUTED in-PR:**
  `Heritage::guard_calls` is not a prototype-query hot spot -- the three model queries measure
  30 / 19.7 / 9.8 ms inside a 971 ms loop, so the cost is INDIRECT op and Varnode construction
  (IR growth), not a memoizable lookup.
- round 2 B_DONE (captain): **attempt 4 (#396, `105beeec`) MERGED and the need STAYS OPEN.**
  `pr` 393 -> 396; `closing_pr` stays null; `attempts` stays 4. Post-merge witness on the fully
  rebuilt main 8634dbc9, from the acceptance suite itself (a-53d616afcb6a, 1 rep):
  **10,816 ms** against the `< 10,000 ms` bar. Four merged, measured, output-identical attempts
  have taken this witness 71.5 -> 19.4 -> 14.4 -> 11.8 -> 10.8 s. The bar is still not moved.
  **The pre/post delta this wave's harness printed (-0.04 s) is VOID -- do not cite it.** It copied
  the pre-merge `kuna` to /tmp and timed it against the rebuilt one, but `kuna decompile` forks
  `decomp_dbg`, and `kuna-cli/src/paths.rs` resolves that sibling by `exe_dir()` then falls back to
  `repo_root()/decompiler/target/release`; off /tmp the parent walk fails, `repo_root()` falls back
  to the CWD, and the "pre" binary executed the FRESHLY REBUILT engine (proven: the /tmp copy's
  `catalog --json` gained the new option across `make binaries`). Any future A/B of a decompile must
  pin the engine too -- copy `decomp_dbg` aside and point at it with `KUNA_DECOMP_DBG`.
  Attempt 5, if it is ever dispatched, is a **campaign, not a profiling run**: the post-#393 profile
  is flat (`stage_jump_table` ~31%, heritage ~23%, oppool1 ~14%, ActionDeadCode ~14%) and #396 took
  its 0.9 s out of Rc traffic on 9.5 M tree mutations, i.e. the volume, not a pass. Dispatch it
  `[PROPOSAL]`-shaped or park it; do not spend a fifth open-ended builder here.
- round 2 B_PLAN wave 17 (captain): **attempt 5 IS dispatched, and it is dispatched ALONE with the
  option leases — the one thing every previous attempt was denied.** `touches` widened from
  [kuna-decomp, kuna-console] to also declare `phases.toml`, `p0_knowledge/options.rs`,
  `docs/options.md`, `docs/history.md` and `tests/stages`, so `resources_for` now hands this need
  file:phases.toml + counter:catalog + counter:div + counter:stages-corpus + file:docs/options.md.
  That is deliberate and it costs a wave: those five leases are also the whole `quality`
  TRACK_RESOURCES set, so this dispatch excludes every quality need while it lives, and the
  higher-ranked `mach-o-entry-function` (13.86, small, attempts 0, acceptance genuinely failing on
  `name.*main`) is next in line rather than concurrent. The override of select's ranking (this need
  scores 4.62, last) is on one fact: it is the ONLY need in the backlog with a specified, measured,
  single-change path to its own acceptance, and it has been starved of the lease that path needs
  twice.
- round 2 B_PLAN wave 17 (captain), **the brief — this is a SCOPED FEATURE PR, not a fifth
  profiling run. Do not re-profile; the profile is in this file and in
  `docs/features/decompiling-3396-byte-main/record.json`.**
  1. Implement exactly one mechanism: **share the jump-table partial sub-decompilation across
     tables** (C++ `stageJumpTable` runs the clone + reduced pipeline once per function behind
     `if (!partial.isJumptableRecoveryOn())`; kuna runs it once per table). Measured here:
     2 tables, 3,077 ms of action time + 464 ms of cloning, so ~1.7 s / ~15% off an 11.2 s witness.
  2. It changes which tables recover, so it needs a `phases.toml` row + `options.rs` registration
     + the catalog counts + a DIV row + a `tests/stages` case. **You hold all of those leases.**
  3. A fact worth having before you design the gate: `unrolledguard` — the option whose
     per-table re-clone this would remove — is `default = off, destructive = true` in
     `phases.toml`. If the re-clone is load-bearing only when `unrolledguard` is on, the shared
     partial can be the DEFAULT and per-table cloning kept for the `unrolledguard on` path; verify
     that, do not assume it.
  4. Default-ON needs the usual evidence: 0/675 datatest assertions moved. If any move, ship the
     option default-OFF and say so — a measured, output-identical, merged cut that misses the
     10 s bar is still a success by this loop's rules, and **the bar is not moved to meet it**
     (four captains have now declined to re-cut it).
  5. Measurement, inherited: `kuna decompile` FORKS `decomp_dbg`, so an A/B that copies only
     `kuna` aside silently times the freshly built engine — pin the engine with
     `KUNA_DECOMP_DBG`. Interleaved paired A/B only; a sequential timeit on this box has
     reported +42% on byte-identical output. `perf` is blocked (`perf_event_paranoid = 4`);
     instrument, do not sample.

