---
need_id: string-inventory-include-non
title: String inventory cannot include non-NUL-terminated printable runs
track: tooling
status: open
severity: major
probe_id: p-5634c1d030db
acceptance_id: a-7d115bcb7ba0
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [673da52e9b533b4c22bd2eeb]
rounds: [8]
first_seen_round: 8
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

Inventory embedded bytecode names, with a scan-policy option if necessary.

> **String inventory cannot include non-NUL-terminated printable runs** (major, `673da52e9b533b4c22bd2eeb`)
> Returned count 0 even with encoding all and minimum length 4. Kuna read shows out.js at 0xb2024 and numerous identifiers separated by control bytes. The catalog exposes only NUL-terminated detection.

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
    "--section",
    ".rodata",
    "--filter",
    "_0x|out.js",
    "--encoding",
    "all",
    "--min-length",
    "4",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.x86_64.elf",
    "binary_sha256": "19351f6b9a6b8af7399470a3ff22ddf1b2b8f44a85f1e0497cbb6e4ed20fbb8a",
    "binary_size": 977264,
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
    "strings",
    "{{BIN}}",
    "--section",
    ".rodata",
    "--filter",
    "_0x|out.js",
    "--encoding",
    "all",
    "--min-length",
    "4",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "count",
        "op": "gt",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.x86_64.elf",
    "binary_sha256": "19351f6b9a6b8af7399470a3ff22ddf1b2b8f44a85f1e0497cbb6e4ed20fbb8a",
    "binary_size": 977264,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

- `strings -a target/crackme.x86_64.elf` — Contains out.js, _0x8ec6b3, _0x3f5c60 and hundreds of other identifiers; saved in notes/reference-strings.txt.

## Instances

- `673da52e9b533b4c22bd2eeb` (round 8, tester t-r8-673da52e)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 CAPTAIN B_VERIFY: acceptance probe TARGET-BOUND to bin/crackme.x86_64.elf (sha 19351f6b9a6b8a..., 977264 bytes, binary_source dataset, challenge 673da52e9b533b4c22bd2eeb) in BOTH the Reproduction and the Acceptance block, and re-measured at sha bfacf0c6 as runnable-and-still-FAILING (strings --section .rodata --filter _0x|out.js --encoding all --min-length 4 --json exits 0 with count 0). Before this the probe read unrunnable ({{BIN}} used but the context supplies no bin), which means B_DONE could never have closed the need no matter what the builder shipped. Do NOT re-cut the acceptance onto an easier fixture; when you promote it into tests/cli/ it must be retargeted onto a vendored fixture that fails the same way pre-fix, because CI has no dataset.
