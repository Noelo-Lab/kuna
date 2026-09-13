---
need_id: string-copy-destination-incorrectly
title: String copy destination incorrectly becomes a null pointer
track: quality
status: closed
severity: major
probe_id: p-18068442ae53
acceptance_id: a-045c2e212a05
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [640a526833c5d447bc761899]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: libproto
touches: [decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/575
closed_in_round: 12
closing_pr: "575"
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
      "memmove\\(\\s*a0\\s*,"
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

**Overturned.** The null destination was not a dataflow substitution. The known
`memmove` prototype had been parked by name on the IAT slot while the call-site
read path queried the thunk entry address.

## Refutation

`KeyCheker.exe` exposes both the `memmove` IAT slot at `0x1400051b8` and its
six-byte import thunk at `0x140003dd9`. The caller targets the thunk. PR #575
parks built-in library prototypes on every concrete resolver import address, so
the call now retains the incoming destination parameter as `a0`.

## Reference

_none recorded_

## Instances

- `640a526833c5d447bc761899` (round 3, tester t-r3-640a5268)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_DEDUP r3: SPLIT out of the same 5-observation cluster. Not an argument-count or ABI gap: RCX still holds the destination at the call and a value stored THROUGH RCX is emitted in its place, which is a dataflow substitution.
captain T_TRIAGE r3: track quality and touches kuna-decomp CONFIRMED: the probe is a plain `kuna decompile` of one function and the complaint is the emitted C. Touches left at the crate because the phase is not yet identified -- a null destination in a copy is plausibly p3 dataflow or p5/p6 type-and-variable work, and guessing narrower would mislead the builder. Hypothesis is inconclusive and unrefuted (single instance, credibility 0.7): reproduce before designing.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- closed: acceptance a-045c2e212a05 now PASSES at
  `b1f0b4c1a7b4d1d36bf5def703bba27f4b538bf2`.
- round 12 reconciliation: `default-memmove-call-writes` is the same challenge,
  target, command, probe, and acceptance re-filed in round 8; it is preserved in
  the rejected ledger while this earlier round-3 record remains the canonical
  survivor. PR #575 (squash `bf8c0b95bfd2125ab6ef522866cfb466904f5545`)
  fixed the shared resolver-address defect. The acceptance is strengthened from
  any non-null `memmove` destination to the observed correct `a0` destination;
  `--option libproto off` remains the control for the built-in signature pass.
- current controls on exact `b1f0b4c1`: the strengthened dataset acceptance
  passed 3/3 as `memmove(a0,v8,0x10)` with no null destination. Disabling only
  `libcsigs` preserves `a0`; disabling `libproto` restores
  `memmove(0,v7,0x10)` in 3/3 runs, confirming that `libproto` is the covering
  option and that PR #575's base-table address binding is the shipped fix.
