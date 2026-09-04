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
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 380
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
