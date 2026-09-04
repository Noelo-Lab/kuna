---
need_id: cold-load-xref-lookup
title: Cold-load xref lookup takes about four seconds on a 466 KB ELF
track: perf
status: open
severity: minor
probe_id: p-6d1e0b3b78b9
acceptance_id: a-6763a70e69bc
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [5bd1d1bb33c5d4110a29b31e]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Iteratively query references while triaging the checker.

> **Cold-load xref lookup takes about four seconds on a 466 KB ELF** (minor, `5bd1d1bb33c5d4110a29b31e`)
> The actual `kuna xrefs ./target/Obfuscation1 --to 0x80ba3d2 --json` invocation took 4.1303 seconds according to notes/toolcalls.jsonl. Every independent query reloads analysis state.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x80ba3d2",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "wall_ms": {
      "stat": "median",
      "gt": 2000
    }
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x80ba3d2",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "wall_ms": {
      "stat": "median",
      "lt": 1000
    }
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Each CLI invocation performs a full cold load and analysis rather than reusing a project or resident analysis session.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/Obfuscation1 --backend ida --json` — The reference interface is designed around a persistent server/database, although its server could not start in this arena because it attempted to create a config lock on a read-only path.

## Instances

- `5bd1d1bb33c5d4110a29b31e` (round 2, tester t-r2-5bd1d1bb)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): touches [] -> kuna-cli; track perf, scope small, severity minor CONFIRMED. The friction is per-query cold load (4.1s x N queries), so the cheapest real fix is a CLI batch mode (several --to in one invocation) rather than making the load faster -- which is why the edit surface is kuna-cli though the track is perf. Neither track holds counter leases, so the label costs nothing either way.
