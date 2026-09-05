---
need_id: void-callee-spurious-arg
title: a void callee is called with an argument
track: quality
status: open
severity: major
probe_id: p-cb96bbe2ca7e
acceptance_id: a-293516d9c2f1
hypothesis_status: inconclusive
credibility: 0.8
instances: 1
challenges: [68149b8a8f555589f353117c]
rounds: [1, 2]
first_seen_round: 1
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

A call consistent with kuna's own recovered declaration `int _secret_function(void)`.

> **Void secret function is called with a spurious argument** (major, `68149b8a8f555589f353117c`)
> The same output declares `_secret_function(void)` but calls `_secret_function(v2)`, producing internally inconsistent and invalid prototype usage.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "_main"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "_secret_function\\(v[0-9]+\\);"
    ]
  },
  "target": {
    "binary_rel": "bin/practice_2_rengr",
    "binary_sha256": "687075c7e8234dc95de79950b01fe040f003d56a056e1f5559ff411046fef9fe",
    "binary_size": 33776,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "p-cb96bbe2ca7e"
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
    "decompile",
    "{{BIN}}",
    "_main"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "_secret_function\\(\\);"
    ]
  },
  "target": {
    "binary_rel": "bin/practice_2_rengr",
    "binary_sha256": "687075c7e8234dc95de79950b01fe040f003d56a056e1f5559ff411046fef9fe",
    "binary_size": 33776,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "a-293516d9c2f1"
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The live return register from scanf was incorrectly treated as an argument to the subsequent no-argument call.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `68149b8a8f555589f353117c` (round 1, tester t-r1-68149b8a)

## Decision log

- round 1: gated `admitted`, NOT built (round 1 closed 12 of 23). Carried into round 2.
- round 2 wave 20 (captain): **RESTORED FROM `.kuna-repipe/rounds/1/gate.json` + the arena
  report** and RE-GATED against merged main 509bb7a3. This record carried only a `## Symptom`
  stub, no probe and no acceptance, so it was undispatchable by `select` and unclosable by
  B_DONE; the two arms below existed all along in round-1 state and were never transcribed.
  Two-arm verdict today: `admitted` (probe PASSES, acceptance FAILS), so the gap is live and
  closable-by-probe. Target rebound from the round-1 arena path to the dataset challenge
  (`binary_source: dataset`, sha256 pinned), so the probe no longer depends on an arena that
  a later round may rebuild.
- round 2 wave 20 (captain): probe REWRITTEN, symptom UNCHANGED: as filed the probe pinned `_secret_function(v2);` and today's
  build emits `_secret_function(v3);` -- SSA numbering drift, not a fix. Re-gated with
  `_secret_function\(v[0-9]+\);` -> admitted. probe_id p-60d3b7f1c20b -> p-cb96bbe2ca7e (derived
  from cmd+expect); the acceptance is byte-identical, so a-293516d9c2f1 is unchanged.
- round 2 wave 20 (captain): `touches` LEFT as filed (`[decompiler/crates/kuna-decomp]`). I have measured evidence of the
  symptom but none of the owning module, and a confidently-wrong `touches` would misdirect a
  builder and mis-scope its leases. The builder should set it from its own root-cause work.
