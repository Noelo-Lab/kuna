---
need_id: strings-inventory-omits-statically
title: Strings inventory omits statically constructed strings needed by the checker
track: quality
status: open
severity: major
probe_id: p-7c997ef1f023
acceptance_id: a-926bce988084
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [653d88600f4238b24302b0ec]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/analyzers/strings]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

An inventory that recovers strings deterministically built by the initializer, including the serial horsey and prompt/result messages.

> **Strings inventory omits statically constructed strings needed by the checker** (major, `653d88600f4238b24302b0ec`)
> Even with minimum length 2 and all encodings, strings reported literals and instruction-byte false positives but not horsey or the generated messages.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "strings",
    "{{BIN}}",
    "--json",
    "--min-length",
    "2",
    "--encoding",
    "all"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_absent": [
      "horsey",
      "Correct Passphrase",
      "Enter the passphrase"
    ]
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
    "strings",
    "{{BIN}}",
    "--json",
    "--min-length",
    "2",
    "--encoding",
    "all"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "horsey"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The strings command scans mapped bytes only and does not incorporate simple static initializer emulation.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `653d88600f4238b24302b0ec` (round 2, tester t-r2-653d8860)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): third instance of the 'kuna will not tell me about data' family (see [cli-mode-read-raw]). Same challenge as [simd-constant-string-initializer] and plausibly the same root -- a string the program builds with SIMD stores is invisible to the string scanner AND explodes in the emitted C -- but the fixes sit in different tiers, so a builder closing one should re-check the other rather than assume it followed.
- round 2 T_TRIAGE (captain): track tooling -> QUALITY, scope small -> LARGE, touches kuna-cli -> kuna-analysis/src/analyzers/strings. The strings the tester wanted are never in the image as bytes: they are BUILT by the SIMD initializer at run time, so no scanner setting (min length 2, all encodings) can ever find them -- this is not a strings surface gap. The fix is synthesising strings from constant stores, a new analyzer capability that ships behind an option, hence `quality` and its counter leases. See simd-constant-string-initializer, its decompiler-side twin.
