---
need_id: unpack-upx-packed-executable
title: kuna cannot unpack a UPX-packed executable before analysis
track: tooling
status: open
severity: blocker
probe_id: p-160471d44d7e
acceptance_id: a-3d357962b3f7
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [64f1f7afd931496abf909525]
rounds: [20]
first_seen_round: 20
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

Unpack target/snake and analyze the actual ncurses application rather than its decompression stub

> **kuna cannot unpack a UPX-packed executable before analysis** (blocker, `64f1f7afd931496abf909525`)
> The binary contains the UPX signature. Function discovery returned zero functions (the already-filed zero-functions-exit-0 issue), direct entry-point decompilation produced only calls into the loader, and `kuna unpack` exited 2 with `unknown subcommand`.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "unpack",
    "{{BIN}}"
  ],
  "expect": {
    "exit_code": {
      "eq": 2
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
    "unpack",
    "{{BIN}}"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    }
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- kuna has no packer-detection or unpacking stage, so analysis never reaches the original program image.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `64f1f7afd931496abf909525` (round 20, tester t-r20-64f1f7af)

## Decision log

- filed by cluster.py from 1 observation(s)
