---
need_id: keyboard-callback-uses-undefined
title: Keyboard callback uses undefined locals as incoming event arguments
track: quality
status: open
severity: major
probe_id: p-2fa68bfb8456
acceptance_id: a-d6b2084f35d5
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [6989ca5da15272fa37a80c43]
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

A decompilation whose signature includes the Wayland callback event arguments used by the first condition.

> **Keyboard callback uses undefined locals as incoming event arguments** (major, `6989ca5da15272fa37a80c43`)
> The function signature contains only a0, while the first condition reads register-associated locals, so the event-type and key parameters appear uninitialized. The disassembly shows those values arrive in R9D and R8D.

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
    "0x6500",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "void sub_6500\\(unsigned long \\*a0\\)",
      "if \\(v[0-9]+ != 1 \\|\\| !a0\\[0xd\\]\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/lugosiii",
    "binary_sha256": "8ea3c454dfcea5d5f970d695608568ed81b688dd367faa2f3fcbeb4bc700aa6e",
    "binary_size": 29336,
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
    "0x6500",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "sub_6500\\(",
      "sub_6500\\([^)]*,[^)]*\\)"
    ],
    "stdout_absent": [
      "^\\s+\\S[^;]*; // r8d$",
      "^\\s+\\S[^;]*; // r9d$"
    ]
  },
  "target": {
    "binary_rel": "bin/lugosiii",
    "binary_sha256": "8ea3c454dfcea5d5f970d695608568ed81b688dd367faa2f3fcbeb4bc700aa6e",
    "binary_size": 29336,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The undiscovered callback has no prototype evidence, and parameter recovery fails to promote live-in R8D/R9D values to arguments.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6989ca5da15272fa37a80c43` (round 2, tester t-r2-6989ca5d)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): SPLIT out of the 4-observation `wrong-output|decompile|stdout_absent` group; sibling of [direct-address-keyboard-handler] (shared root claim: 0x6500 undiscovered). Deliberately NOT merged into [argument-recovery-knobs-still]: this is callee-side INPUT recovery (live-in R8D/R9D never promoted to parameters), not caller-side argument recovery.
- round 2 T_TRIAGE (captain): scope small -> LARGE. The values arrive in R9D/R8D of a callback only ever reached through a registration site, so recovering them means inferring a prototype from that registration -- a real feature. Note the cheaper path this need is evidence FOR: if the CLI could carry a prototype override (need no-cli-rename-or-prototype-override) an agent could state the signature itself and would not need the inference at all. That makes this a demand witness for the no-cli-* family, which is exactly the independent tester evidence those captain-seeded needs were missing.
