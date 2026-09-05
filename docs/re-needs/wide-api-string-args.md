---
need_id: wide-api-string-args
title: wide (UTF-16) Windows API string arguments collapse to one character
track: quality
status: open
severity: major
probe_id: p-3c2902ec2d39
acceptance_id: a-038711a56c33
hypothesis_status: inconclusive
credibility: 0.8
instances: 1
challenges: [65acadf3eef082e477ff5ede]
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

The complete DLL and window-name wide strings used by LoadLibraryW and FindWindowW.

> **Wide Windows API string arguments collapse to one-character strings** (major, `65acadf3eef082e477ff5ede`)
> Rendered calls such as `LoadLibraryW("n")` and `FindWindowW("O",0)`, losing the remainder of the UTF-16 strings and making anti-debugging checks opaque.

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
    "sub_140002530"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "LoadLibraryW\\(\"n\"\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/ObfuscationFiesta.exe",
    "binary_sha256": "b852fd5bfa65f7d71b903fb1852ae8ad03f2611637a42d501a897424647deec9",
    "binary_size": 32768,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "p-3c2902ec2d39"
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
    "sub_140002530"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "LoadLibraryW\\(L\"ntdll\\.dll\"\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/ObfuscationFiesta.exe",
    "binary_sha256": "b852fd5bfa65f7d71b903fb1852ae8ad03f2611637a42d501a897424647deec9",
    "binary_size": 32768,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "a-038711a56c33"
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- String markup or emission treats UTF-16LE data as an 8-bit NUL-terminated string.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `65acadf3eef082e477ff5ede` (round 1, tester t-r1-65acadf3)

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
- round 2 wave 20 (captain): `touches` LEFT as filed (`[decompiler/crates/kuna-decomp]`). I have measured evidence of the
  symptom but none of the owning module, and a confidently-wrong `touches` would misdirect a
  builder and mis-scope its leases. The builder should set it from its own root-cause work.
