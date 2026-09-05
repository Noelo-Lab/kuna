---
need_id: mach-o-entry-function
title: Mach-O entry function is not identified as main
track: quality
status: open
severity: major
probe_id: p-d21d1d48685f
acceptance_id: a-9e76eaa91fcb
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [653d88600f4238b24302b0ec]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/loader]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

The LC_MAIN entry routine identified as main, with its conventional argc/argv prototype.

> **Mach-O entry function is not identified as main** (major, `653d88600f4238b24302b0ec`)
> functions listed the program routine only as sub_100003a80; no function named main was present, and decompilation rendered it as unsigned long sub_100003a80(void).

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_absent": [
      "name.*main"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme0x04",
    "binary_sha256": "9f17db47a3486acfa57590c2a0e551f25b1e25e8dfdf4a584f1f5c673b4fe26e",
    "binary_size": 50176,
    "binary_source": "dataset"
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
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "name.*main"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme0x04",
    "binary_sha256": "9f17db47a3486acfa57590c2a0e551f25b1e25e8dfdf4a584f1f5c673b4fe26e",
    "binary_size": 50176,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- entrymainproto handles CRT call-site recovery but the Mach-O LC_MAIN loader path does not label/type the entry routine.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `653d88600f4238b24302b0ec` (round 2, tester t-r2-653d8860)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): track tooling -> QUALITY, touches kuna-cli -> kuna-analysis/src/loader. LC_MAIN is a load-command fact, so this is loader/analysis-tier and not a CLI change; and naming an entry `main` with an argc/argv prototype is a naming POLICY that changes emitted output, so it ships behind an option and must hold the counter leases. scope stays small -- the entry offset is directly available in the load command.
- round 2 B_PLAN wave 17 (captain): **NEXT IN LINE, not dispatched this wave.** Top of the
  13.86 tie among needs with a real acceptance probe (its `name.*main` clause measured
  `<no match>` on merged main 8634dbc9, so it is closable by probe), small scope, attempts 0.
  It is held back only because wave 17 gave the quality/counter lease set to the perf need
  `decompiling-3396-byte-main` for its one specified fix; dispatch this the moment that
  builder drains.

