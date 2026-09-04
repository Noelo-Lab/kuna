---
need_id: simd-constant-string-initializer
title: SIMD constant-string initializer explodes into hundreds of scalar pseudo-operations
track: quality
status: open
severity: major
probe_id: p-7127e6d559a3
acceptance_id: a-f6dbde227633
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [653d88600f4238b24302b0ec]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Readable pseudocode for the initializer so I could recover the generated prompt, result messages, and serial without manually simulating SSE byte lanes.

> **SIMD constant-string initializer explodes into hundreds of scalar pseudo-operations** (major, `653d88600f4238b24302b0ec`)
> The 752-byte initializer expanded into a very large body with pshufb, dozens of SUB161 lane temporaries, and repeated byte stores, obscuring simple generation of consecutive character ranges.

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
    "sub_100003790"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "pshufb\\(",
      "SUB161\\("
    ],
    "stdout_bytes": {
      "gt": 12000
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
    "decompile",
    "{{BIN}}",
    "sub_100003790"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "SUB161\\("
    ],
    "stdout_bytes": {
      "lt": 12000
    }
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Vector lane extraction and constant propagation do not recognize the byte-sequence construction idiom.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `653d88600f4238b24302b0ec` (round 2, tester t-r2-653d8860)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): scope small -> LARGE. Folding pshufb/SUB161 lane temporaries back into a recognisable constant-string initializer is a new SIMD constant-folding capability, not a patch. Companion of strings-inventory-omits-statically: same challenge (653d8860), same 752-byte initializer, one seen from the decompiler surface and one from the strings surface. Kept separate because they are separately probeable and a fix to either does not imply the other, but a builder on one should read the other.
