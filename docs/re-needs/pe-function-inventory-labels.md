---
need_id: pe-function-inventory-labels
title: PE function inventory labels executable routines as vftables
track: tooling
status: open
severity: major
probe_id: p-8e912566d7ea
acceptance_id: a-d6666365cbb6
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [6547b4d50f4238b24302b588]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/functions.rs, decompiler/crates/kuna-analysis/src/listing]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

A trustworthy function inventory for triaging the PE.

> **PE function inventory labels executable routines as vftables** (major, `6547b4d50f4238b24302b588`)
> The inventory includes executable ranges named std::bad_alloc::vftable_1 and numerous std::basic_stringbuf::vftable_N entries as functions, including ranges hundreds of bytes long. These names are data-symbol aliases and misleadingly classify routines as vtable objects.

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
    "stdout_matches": [
      "std::bad_alloc::vftable_1",
      "std::basic_stringbuf::vftable_[0-9]+"
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
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "stdout_absent": [
      "\"name\": \"[^\"]*::vftable_[0-9]+\""
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

- Public data aliases near code addresses are winning the canonical-name selection for discovered function entries.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6547b4d50f4238b24302b588` (round 2, tester t-r2-6547b4d5)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): track quality -> TOOLING, touches kuna-decomp -> the inventory surface. Naming an executable range after a data symbol (std::bad_alloc::vftable_1) is wrong output with no judgment call in it, so it is a bug fix and needs no option. scope small. If the builder finds the alias comes from symbol application in the analysis tier and correcting it changes function DISCOVERY rather than labelling, STOP -- that is a different, gated change.
