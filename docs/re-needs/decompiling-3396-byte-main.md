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
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The function's extensive opaque arithmetic and indirect calls cause one or more analysis passes to scale poorly.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69a3822f7b3cc38c80464da4` (round 2, tester t-r2-69a3822f)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): touches [] -> kuna-decomp; track perf and scope small CONFIRMED. An empty touches list declares no edit surface at all, which leaves the sibling-contract rule nothing to enforce. 68s on a 3396-byte function is a profiling task with a targeted fix, and perf-track needs hold no counter leases.
