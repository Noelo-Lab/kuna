---
need_id: default-memmove-call-writes
title: Default memmove call writes to zero instead of the destination parameter
track: quality
status: rejected
severity: major
probe_id: p-18068442ae53
acceptance_id: a-f88ffa89ede1
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [640a526833c5d447bc761899]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: "duplicate of string-copy-destination-incorrectly: the same KeyCheker.exe challenge, sub_140002240 command, target, probe, and acceptance were already filed in round 3. PR #575 closes the shared resolver-address defect; retain the earlier filing as the canonical closure and preserve this later filing only in the rejected ledger."
---

## Symptom

Follow the short-string copy without inventing a null write.

> **Default memmove call writes to zero instead of the destination parameter** (major, `640a526833c5d447bc761899`)
> Emits memmove(0,v7,0x10). Disassembly preserves incoming RCX through the call. An explicit thunk prototype alone restores the destination parameter.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
    "binary_source": "dataset"
  },
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
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
    "binary_source": "dataset"
  },
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
  }
}
```

## Hypothesis

**Overturned.** The signature was correct but parked by name on a different
symbol than the thunk entry queried at the call site.

## Refutation

`KeyCheker.exe` exposes both a six-byte `memmove` import thunk at
`0x140003dd9` and its IAT slot at `0x1400051b8`. A name-keyed prototype binds
one symbol while the caller targets the other. Declaring the same prototype by
name leaves `memmove(0,v7,0x10)` unchanged; declaring it at `0x140003dd9`
produces `memmove(a0,src,0x10)`.

## Reference

_none recorded_

## Instances

- `640a526833c5d447bc761899` (round 8, tester t-r8-640a5268)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 8 REFUTER: the correct built-in `memmove` signature existed, but name-keyed parking selected the IAT slot while the caller targeted the thunk entry. The merged VirtualAlloc observation had a different cause and was split into `virtualalloc-protection-argument-disappears`.
- rejected: duplicate of `string-copy-destination-incorrectly`. Both records
  originally used the same challenge, binary, selector, command, probe ID, and
  acceptance ID. The round-3 record is the canonical survivor (with its
  acceptance now strengthened), and PR #575 closes their shared
  resolver-address defect.
