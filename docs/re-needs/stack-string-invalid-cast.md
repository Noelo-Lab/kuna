---
need_id: stack-string-invalid-cast
title: stack-string initialisation is emitted as an invalid array cast
track: quality
status: open
severity: minor
probe_id: p-1d6eb3f72061
acceptance_id: a-d7fec4000cb6
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

Readable, valid C initializing the local message buffer, ideally from the recognized string "Hello, Reverse Engineer!".

> **Stack string initialization is emitted as invalid array casts and scalar assignments** (major, `68149b8a8f555589f353117c`)
> It emitted expressions including `v1[0] = (char[8])s_100003f1d._0_8_;` and `v5[0] = (char[7])s_100003f1d._9_7_;`; array casts are not valid C and assigning them to individual chars misrepresents the multi-byte copies.

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
      "v1\\[0\\]\\ =\\ \\(char\\[8\\]\\)s_100003f1d\\._0_8_;"
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
  "probe_id": "p-1d6eb3f72061"
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
      "Hello,\\ Reverse\\ Engineer!"
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
  "probe_id": "a-d7fec4000cb6"
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Piecewise constant-to-stack stores were typed as array-valued casts without being reconstructed as a single string-backed buffer initialization.

## Refutation

_not yet refuted_

## Reference

`{"tool": "strings", "command": "strings ./target/practice_2_rengr", "evidence": "It directly reports `Hello, Reverse Engineer!`, the buffer content obscured by kuna's emitted assignments."}`

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
- round 2 wave 20 (captain): `touches` LEFT as filed (`[decompiler/crates/kuna-decomp]`). I have measured evidence of the
  symptom but none of the owning module, and a confidently-wrong `touches` would misdirect a
  builder and mis-scope its leases. The builder should set it from its own root-cause work.
