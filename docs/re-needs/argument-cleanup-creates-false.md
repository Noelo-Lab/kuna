---
need_id: argument-cleanup-creates-false
title: Argument cleanup creates a false internal tail call
track: quality
status: closed
severity: major
probe_id: p-2bc2afad11b1
acceptance_id: a-d6135fbc9bd1
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [5ab77f5f33c5d40ad448c820]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: tailcallsaved
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/560
closed_in_round: 12
closing_pr: "560"
reject_reason: null
---

## Symptom

Preserve the shared character-classification continuation.

> **Argument cleanup creates a false internal tail call** (major, `5ab77f5f33c5d40ad448c820`)
> Classified the jump at 0x4010f2 as a tail call after ADD ESP,8 removes two call arguments. Its destination, 0x401102, is an internal TEST EAX,EAX reached by both paths; saved registers remain on the stack.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/sddecoder_v1.1.zip.__x/decoder.exe",
    "binary_sha256": "8a0d47662bd432bf5e6c4e6d0665303b9a2e6a925befeaf9e6bebc38979c1fea",
    "binary_size": 172032,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x4010b0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "tailcallframe: recovered tail call"
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
    "binary_rel": "bin/sddecoder_v1.1.zip.__x/decoder.exe",
    "binary_sha256": "8a0d47662bd432bf5e6c4e6d0665303b9a2e6a925befeaf9e6bebc38979c1fea",
    "binary_size": 172032,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x4010b0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "tailcallframe: recovered tail call"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Frame matching may confuse argument cleanup with restoration of saved registers.

## Refutation

**Hypothesis CONFIRMED** (`frame matching may confuse argument cleanup with restoration of
saved registers`). `tailcallframe` (DIV-109) fires when the straight-line run ending at a
direct `jmp` raises the stack pointer by exactly what the run leaving the entry address
lowered it by. Both of its scans stop at the first control-flow op, so on this witness the
backward one measures `0x4010ef add esp,8` (+8) and stops at `0x4010ea call 0x40167d`, never
seeing the `push 0x103` / `push esi` that the cleanup undoes. The entry run is
`push ebx; push esi` (-8). Two unrelated stack motions of 8 bytes each satisfy the
exact-cancellation test; `ebx` and `esi` are still on the stack and the stack pointer is
still 8 bytes below its entry value when the `jmp` executes.

Not covered by an existing option: the only option in this family is `tailcallframe` itself,
and turning it off closes the witness by giving up the capability it exists for (the round-2
keyboard-callback witness regresses to inlining its callee).

## Reference

_none recorded_

## Instances

- `5ab77f5f33c5d40ad448c820` (round 8, tester t-r8-5ab77f5f)

## Decision log

- filed by cluster.py from 1 observation(s)
- closed by `tailcallsaved` (new option, default on, DIV-157): a stack-pointer run that loads
  nothing back through the stack pointer is argument cleanup, not a frame teardown. Each scan
  now reports the bytes it moved *through* the stack pointer beside its delta, and the branch
  is a tail call only when `restored >= saved`.
- captain T_DEDUP (r8): SPLIT out of a 4-member deterministic bucket. cluster.py's key is `kind|subcommand|acceptance-clause-shape`, so four unrelated `wrong-output`+`decompile`+`exit_code,stdout_absent` defects collapsed into one need and only the witness's probe would have survived. This one is the tail-call/frame-matching defect and is unrelated to the other three.
- closed: acceptance a-d6135fbc9bd1 now PASSES at d23e5a3e69c3.
- round 12 reconciliation: PR #560 (squash `d23e5a3e69c335e9c9a818f5f33c79de61d189ed`) implemented default-on `tailcallsaved` for this need, but the authoritative index and front matter retained their pre-closure `open` snapshot.
- current controls at `bf22331158be444ebfbabf937ce8141826490fa9`: the exact acceptance passes with `tailcallsaved on`; adding `--option tailcallsaved off` restores the false `tailcallframe: recovered tail call` witness. This closure distinguishes this saved-register/argument-cleanup case and does not claim to resolve broader tail-call ambiguity.
- index reconciliation constraint: the authoritative index contains 197 active records and 1 rejected record. This closure preserves every non-target row and changes only the four reconciled rows plus `by_status`; no sparse reindex was run.
