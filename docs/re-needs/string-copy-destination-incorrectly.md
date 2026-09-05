---
need_id: string-copy-destination-incorrectly
title: String copy destination incorrectly becomes a null pointer
track: quality
status: open
severity: major
probe_id: p-18068442ae53
acceptance_id: a-f88ffa89ede1
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [640a526833c5d447bc761899]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Determine where the short-string copy writes.

> **String copy destination incorrectly becomes a null pointer** (major, `640a526833c5d447bc761899`)
> Emitted memmove(0,...,0x10). Disassembly at 0x14000229e shows RCX still holds the incoming destination pointer. Earlier instructions store zero through RCX, not into RCX. Reliable mode and argument-recovery options retain the error.

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
    "sub_140002240"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "memmove\\(\\s*(?:0|NULL)\\s*,"
    ]
  },
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
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
    "sub_140002240"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "memmove\\("
    ],
    "stdout_absent": [
      "memmove\\(\\s*(?:0|NULL)\\s*,"
    ]
  },
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- A stored memory value may have been substituted for its destination address.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `640a526833c5d447bc761899` (round 3, tester t-r3-640a5268)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_DEDUP r3: SPLIT out of the same 5-observation cluster. Not an argument-count or ABI gap: RCX still holds the destination at the call and a value stored THROUGH RCX is emitted in its place, which is a dataflow substitution.
captain T_TRIAGE r3: track quality and touches kuna-decomp CONFIRMED: the probe is a plain `kuna decompile` of one function and the complaint is the emitted C. Touches left at the crate because the phase is not yet identified -- a null destination in a copy is plausibly p3 dataflow or p5/p6 type-and-variable work, and guessing narrower would mislead the builder. Hypothesis is inconclusive and unrefuted (single instance, credibility 0.7): reproduce before designing.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
