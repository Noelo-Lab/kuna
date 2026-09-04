---
need_id: c-string-objects-become
title: C++ string objects become five-level pointer types in the checker
track: quality
status: open
severity: major
probe_id: p-a597c89a3648
acceptance_id: a-4ac515df8701
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [6547b4d50f4238b24302b588]
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

Readable recovery of the std::string/ostringstream values used to construct and compare the key.

> **C++ string objects become five-level pointer types in the checker** (major, `6547b4d50f4238b24302b588`)
> The checker is usable only after manually tracking fields through declarations such as unsigned long long ***** and char *****. These false types dominate the 9.9 KB output and make it needlessly difficult to distinguish the input string from the expected string.

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
    "0x1400011c0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "unsigned long long \\*\\*\\*\\*\\*",
      "char \\*\\*\\*\\*\\*"
    ]
  },
  "target": {
    "binary_rel": "bin/trappy attack.exe",
    "binary_sha256": "4872962a99da4def9048556345bdd1699a41b77dc0a148c8c28412b8adb3a06e",
    "binary_size": 26112,
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
    "decompile",
    "{{BIN}}",
    "0x1400011c0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "unsigned long long \\*\\*\\*\\*\\*",
      "char \\*\\*\\*\\*\\*",
      "\\*\\*\\*"
    ],
    "stdout_matches": [
      "\"\\+184V38cC\\.TRAPPY-ATTACK0\"",
      "\"Correct code, ur guess was: 33791\"",
      "\"M28j2\"",
      "\"24452\""
    ]
  },
  "target": {
    "binary_rel": "bin/trappy attack.exe",
    "binary_sha256": "4872962a99da4def9048556345bdd1699a41b77dc0a148c8c28412b8adb3a06e",
    "binary_size": 26112,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- MSVC STL object layouts and constructor prototypes are not propagated into stack locals.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6547b4d50f4238b24302b588` (round 2, tester t-r2-6547b4d5)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): scope small -> LARGE. Recovering std::string/ostringstream through five-level pointer types is C++ type recovery, a known multi-part area of this codebase (TypePointerRel / FuncProto-this / struct-descent are ported but unfed), not a one-session patch.
