---
need_id: prototype-parser-rejects-previously
title: Prototype parser rejects a previously declared struct tag as return type
track: tooling
status: open
severity: minor
probe_id: p-0fb156776be6
acceptance_id: a-907674c07f28
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [673da52e9b533b4c22bd2eeb]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-console/src/grammar, decompiler/crates/kuna-cli/src/assertdecl.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Use struct JSValue as a prototype return type.

> **Prototype parser rejects a previously declared struct tag as return type** (minor, `673da52e9b533b4c22bd2eeb`)
> Accepted the struct declaration but rejected the prototype with Bad C syntax. A typedef alias for the same structure works.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/crackme.x86_64.elf",
    "binary_sha256": "19351f6b9a6b8af7399470a3ff22ddf1b2b8f44a85f1e0497cbb6e4ed20fbb8a",
    "binary_size": 977264,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_9d9c0",
    "--assert",
    "typedef struct JSValue { unsigned long payload; long tag; };",
    "--assert",
    "prototype sub_875e0 struct JSValue sub_875e0(void *ctx, char *buf, unsigned long len, unsigned int flags)",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "prototype .*struct JSValue.*rejected: Bad C syntax"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "main",
    "--assert",
    "typedef struct JSValue { unsigned long payload; long tag; };",
    "--assert",
    "prototype authenticate struct JSValue authenticate(char *user, char *pass)",
    "--assert-strict"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "main",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "authenticate\\(.*\\)\\.payload"
    ],
    "stderr_absent": [
      "rejected: Bad C syntax"
    ]
  },
  "notes": "Desired: a struct tag declared in an earlier --assert is usable as a prototype return type. Dataset witness: crackmes.one 673da52e9b533b4c22bd2eeb (sub_875e0 returning struct JSValue). Once the tag is interned the lexer hands it back as a type name, so `struct JSValue` matched no production and the prototype was dropped with Bad C syntax."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `673da52e9b533b4c22bd2eeb` (round 8, tester t-r8-673da52e)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 TRIAGE (captain): RETOUCHED to kuna-console/src/grammar/ (+ kuna-cli/src/assertdecl.rs for the directive form) -- see the sibling prototype-parser-rejects-valid. Same parser, different rule: a struct tag accepted as a declaration is refused as a return type while a typedef alias for the same structure works. Track stays tooling. Worth telling the builder that the two are one grammar and can honestly ship together.
