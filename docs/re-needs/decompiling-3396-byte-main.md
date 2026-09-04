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
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
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

## Reference

_none recorded_

## Instances

- `69a3822f7b3cc38c80464da4` (round 2, tester t-r2-69a3822f)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): touches [] -> kuna-decomp; track perf and scope small CONFIRMED. An empty touches list declares no edit surface at all, which leaves the sibling-contract rule nothing to enforce. 68s on a 3396-byte function is a profiling task with a targeted fix, and perf-track needs hold no counter leases.
