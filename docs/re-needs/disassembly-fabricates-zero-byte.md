---
need_id: disassembly-fabricates-zero-byte
title: Disassembly fabricates zero-byte instructions beyond mapped memory
track: tooling
status: open
severity: major
probe_id: p-26e8859f6f77
acceptance_id: a-bc4ff057bebd
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ee1f28c33c5d449d91ae7c0]
rounds: [10]
first_seen_round: 10
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: no-disassembly-command
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Disassembly limited to mapped bytes.

> **Disassembly fabricates zero-byte instructions beyond mapped memory** (major, `5ee1f28c33c5d449d91ae7c0`)
> Returns instructions through 0x80d191b, including 0000 ADD instructions beyond exclusive segment end 0x80d1904, with truncated:false and no notes. Direct disassembly at 0x80d190b correctly rejects the address. The read command also supplies zeros across this boundary.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "disassemble",
    "{{BIN}}",
    "0x80d18b0",
    "--count",
    "30",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "end",
        "op": "gt",
        "value": 135076100
      },
      {
        "path": "truncated",
        "op": "eq",
        "value": false
      }
    ]
  },
  "target": {
    "binary_rel": "bin/keygenme.7z.__x/keygenme",
    "binary_sha256": "506fc3c8a43d480bdb877c8dd192e3a6c062013c5dd88a119e6c7cedf5ced890",
    "binary_size": 6502308,
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
    "disassemble",
    "{{BIN}}",
    "0x80d18b0",
    "--count",
    "30",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "end",
        "op": "le",
        "value": 135076100
      }
    ]
  },
  "target": {
    "binary_rel": "bin/keygenme.7z.__x/keygenme",
    "binary_sha256": "506fc3c8a43d480bdb877c8dd192e3a6c062013c5dd88a119e6c7cedf5ced890",
    "binary_size": 6502308,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Reads starting in mapped memory appear to zero-fill an unmapped suffix.

## Refutation

_not yet refuted_

## Reference

- `readelf -lW target/keygenme.7z.__x/keygenme` — Executable LOAD starts at 0x08048000 with FileSiz=MemSiz=0x89904, ending at 0x080d1904. Next LOAD starts at 0x080d2f50.

## Instances

- `5ee1f28c33c5d449d91ae7c0` (round 10, tester t-r10-5ee1f28c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER: hypothesis **upheld** (was inconclusive). REFUTER RAN (captain, tick 10:05Z). Filed cause -- 'reads starting in mapped memory appear to zero-fill an unmapped suffix' -- is UPHELD, and the refuter pinned the boundary to the byte. readelf -lW on .kuna-repipe/arena/10/5ee1f28c33c5d449d91ae7c0/target/keygenme.7z.__x/keygenme: LOAD1 = 0x08048000 + 0x89904, exclusive end 0x080d1904; LOAD2 starts at 0x080d2f50. So 0x080d1904..0x080d2f50 is a genuine unmapped gap. Three commands disagree about it. (1) kuna disassemble 0x80d18b0 --count 30 --json returns end=0x80d191b, truncated=false, notes=[], with eight fabricated '0000 ADD byte ptr [EAX],AL' past the end. (2) kuna read 0x80d18f8 --count 24 prints 384 bytes straight through the gap as zeros and its own header claims the range 0x80d18f8..0x80d1a78. (3) kuna disassemble 0x80d190b and kuna read 0x80d1908 BOTH correctly error 'no bytes mapped ... no loaded segment covers it'. PRECISE CAUSE, sharper than the filed wording: this is not a zero-fill POLICY, it is a MISSING END-CLIP. The segment-containment guard is applied to the START address only; the length is never clipped to the containing segment, and the uncovered suffix reads back as zeros. The correct behaviour already exists in the same binary -- it is the direct-query path. A fix that clips the window to the containing segment and sets truncated/notes is low risk because it makes the windowed path agree with the direct path rather than inventing a new policy. *** SIBLING: this is the SAME ROOT as default-entry-decompilation-aborts, whose failing fetch at r0x080d1955 lies inside this exact gap. Hold the two on ONE builder via a shared cluster lease -- see that need's decision log. ***
- round 10 T_TRIAGE (captain, 10:39Z tick): RETRACKED quality -> tooling, touches kuna-decomp -> kuna-cli. The refuter's fix is a length clip on the windowed read, and that read lives in decompiler/crates/kuna-cli/src/disassemble.rs -- the correct direct-query path that already errors is the same file (disassemble.rs:337, 'no loaded segment covers it'). No engine phase, no option, no catalog counter is involved, so the quality lease set was five leases this builder does not need and would have blocked every other quality need for nothing. *** DISPATCH CONSTRAINT THIS CREATES, AND IT IS NOT ENFORCED BY THE LEASE ALGEBRA: tooling contends for nothing (select.py TRACK_RESOURCES['tooling'] == []), so this need is now freely co-dispatchable with its shared-root sibling default-entry-decompilation-aborts (quality). The refuter proved they are ONE root -- the permissive read is what carries flow into the gap. Dispatch THIS one FIRST and hold the sibling until it merges; a builder who takes the sibling alone will teach the lifter to survive a failed fetch while it still lifts fabricated zero instructions. ***
