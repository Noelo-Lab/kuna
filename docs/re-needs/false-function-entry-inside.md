---
need_id: false-function-entry-inside
title: False function entry inside an instruction splits the checker inventory
track: quality
status: open
severity: major
probe_id: p-a508b5cf3d7c
acceptance_id: a-889458c51ba2
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [6a0b84982b3df128c1df5c0d]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/analyzers/entry/patterns]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Trustworthy function boundaries and complete checker disassembly.

> **False function entry inside an instruction splits the checker inventory** (major, `6a0b84982b3df128c1df5c0d`)
> Discovered 0x14000310c inside the ten-byte MOV beginning at 0x14000310b. Default checker disassembly consequently stops at 0x140003115 with truncated=false. Decompiling the false entry interprets immediate bytes as instructions. Another internal entry truncates the SHA-256 helper into a tail call.

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
      "\"address\":\\s*5368721676\\b"
    ]
  },
  "target": {
    "binary_rel": "bin/frz_crackme_rage_v7.exe",
    "binary_sha256": "971dbc9fc68f8c2a3f516f49cc7c13534e6c57143d0160c648e0c1490662fbf2",
    "binary_size": 279552,
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
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\"address\":\\s*5368721676\\b"
    ]
  },
  "target": {
    "binary_rel": "bin/frz_crackme_rage_v7.exe",
    "binary_sha256": "971dbc9fc68f8c2a3f516f49cc7c13534e6c57143d0160c648e0c1490662fbf2",
    "binary_size": 279552,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Discovery may accept entries without reconciling established instruction ownership.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6a0b84982b3df128c1df5c0d` (round 3, tester t-r3-6a0b8498)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_TRIAGE r3: touches CORRECTED kuna-decomp -> the discovery tier. The probe is `kuna functions --json` and the complaint is a fabricated entry inside an instruction, which is what the byte-pattern entry matcher does; T_REFUTE named funcstart_patterns as the prime suspect after measuring exactly that failure on a second binary. Track stays quality for the same reason as default-decompilation-fails-despite (a default-ON discovery pattern moves inventories corpus-wide). NOT merged with that need: different format (PE vs i386 ELF) and the shared cause is suspected, not measured -- but a builder taking one MUST read the other, since both would edit analyzers/entry/patterns and neither holds a lease on it.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
