---
need_id: both-argument-recovery-options
title: Both argument-recovery options still emit argumentless ptrace calls
track: quality
status: open
severity: major
probe_id: p-80a79049a86b
acceptance_id: a-804be55f4379
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [5ab77f5c33c5d40ad448c615]
rounds: [10]
first_seen_round: 10
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: argument-recovery-knobs-still
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover the four pushed ptrace arguments.

> **Both argument-recovery options still emit argumentless ptrace calls** (major, `5ab77f5c33c5d40ad448c615`)
> With both recovery options enabled, state-machine calls still print ptrace(). Adjacent emitted stores contain the request, PID, address and data.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/trace-p.zip.__x/crackme",
    "binary_sha256": "7b8a32c3cc0e9816bfe03d2e7c4aa5a596a21e43a6e35dc43cb17fa5867138ec",
    "binary_size": 5208,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_80483b0",
    "--define-function",
    "0x8048968=ptrace",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "ptrace\\(\\)"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/trace-p.zip.__x/crackme",
    "binary_sha256": "7b8a32c3cc0e9816bfe03d2e7c4aa5a596a21e43a6e35dc43cb17fa5867138ec",
    "binary_size": 5208,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_80483b0",
    "--define-function",
    "0x8048968=ptrace",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "ptrace\\(\\)"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Dynamic stack alignment and loop-carried stack pointers may defeat argument recovery.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/trace-p.zip.__x/crackme --backend ida` — Backend exited status 1 before registering. No reference pseudocode was available.

## Instances

- `5ab77f5c33c5d40ad448c615` (round 10, tester t-r10-5ab77f5c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER: hypothesis **overturned** (was inconclusive). REFUTER RAN (captain, tick 10:05Z). Filed cause -- 'dynamic stack alignment and loop-carried stack pointers may defeat argument recovery' -- is WRONG PASS. Caller-side stack-argument recovery is NOT defeated: adding only a signature repairs every call site through those very same loop-carried esp accesses. Experiment on .kuna-repipe/arena/10/5ab77f5c33c5d40ad448c615/target/trace-p.zip.__x/crackme: default (with calleearity+varargstackargs on) emits 'v16 += ptrace();' at 15 sites; adding --assert 'prototype ptrace long ptrace(int req,int pid,void *addr,void *data)' turns every one into 4-argument form reading exactly the pushed slots, e.g. ptrace(v18[-0xc],v18[-0xb],(void *)v18[-10],(void *)v18[-9]). The loop-carried esp variable is untouched by the fix. REAL CAUSE: callee ARITY inference. kuna functions --json shows 0x8048968 is NOT a body-less import -- it is a 552-byte statically-linked body -- so this is NOT the size-0-import case of sha-finalization-receives-two. glibc's ptrace is a VARARGS wrapper (long ptrace(enum __ptrace_request, ...) fetching the rest with va_arg), so no amount of body analysis can derive arity 4; it can only come from a libc signature/type-library. A builder must NOT rework stack-arg recovery or the esp/alignment model -- that pass is proven working here. BUILDER WARNING: the probe pins --define-function 0x8048968=ptrace, which names the callee but supplies no signature, so a name-to-signature libc table is what flips this acceptance.
