---
need_id: runtime-decrypted-code-opaque
title: code decrypted at runtime stays an opaque indirect call
track: quality
status: open
severity: minor
probe_id: p-f83385153d24
acceptance_id: a-da7ddcf87198
hypothesis_status: inconclusive
credibility: 0.8
instances: 1
challenges: [61ffb07c33c5d46c8bcbfc1d]
rounds: [1, 2]
first_seen_round: 1
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

Follow the constant-seeded LCG/XOR loop, treat the resulting 197 stack bytes as x86-64 code, and decompile the indirect call through &v4.

> **Runtime-decrypted stack code remains an opaque indirect call** (major, `61ffb07c33c5d46c8bcbfc1d`)
> sub_119a clearly initializes and decrypts the stack blob, but emits v34 = (void *)&v4 followed by v32 = (*v34)(a1[1],v3), with no way to materialize or define the decrypted bytes as code.

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
    "sub_119a"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "v[0-9]+ = \\(\\*v[0-9]+\\)\\(a1\\[1\\],v[0-9]+\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/hidden_password",
    "binary_sha256": "d944813fd8e623a05072c2c866394f467b5ec1ff2352f037fe36466209fee838",
    "binary_size": 14440,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "p-f83385153d24"
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
    "sub_119a"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "hello_world_42"
    ]
  },
  "target": {
    "binary_rel": "bin/hidden_password",
    "binary_sha256": "d944813fd8e623a05072c2c866394f467b5ec1ff2352f037fe36466209fee838",
    "binary_size": 14440,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "a-da7ddcf87198"
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The decompiler lacks a constant-execution/dataflow bridge for writable stack buffers that are later called, and an analyst interface to supply decoded buffers as code.

## Refutation

_not yet refuted_

## Reference

`{"tool": "objdump", "command": "objdump -D -b binary -m i386:x86-64 /tmp/decrypted.bin", "evidence": "Decoded a complete 197-byte function that checks length 14 and compares password XOR key1 against key2."}`

## Instances

- `61ffb07c33c5d46c8bcbfc1d` (round 1, tester t-r1-61ffb07c)

## Decision log

- round 1: gated `admitted`, NOT built (round 1 closed 12 of 23). Carried into round 2.
- round 2 wave 20 (captain): **RESTORED FROM `.kuna-repipe/rounds/1/gate.json` + the arena
  report** and RE-GATED against merged main 509bb7a3. This record carried only a `## Symptom`
  stub, no probe and no acceptance, so it was undispatchable by `select` and unclosable by
  B_DONE; the two arms below existed all along in round-1 state and were never transcribed.
  Two-arm verdict today: `admitted` (probe PASSES, acceptance FAILS), so the gap is live and
  closable-by-probe. Target rebound from the round-1 arena path to the dataset challenge
  (`binary_source: dataset`, sha256 pinned), so the probe no longer depends on an arena that
  a later round may rebuild.
- round 2 wave 20 (captain): probe REWRITTEN, symptom UNCHANGED: as filed the probe pinned `v32 = (*v34)(a1[1],v3)` and today's
  build emits `v31 = (*v33)(a1[1],v2)` -- SSA numbering drift, not a fix. Re-gated with
  `v[0-9]+ = \(\*v[0-9]+\)\(a1\[1\],v[0-9]+\)` -> admitted. probe_id p-9908ee386268 ->
  p-f83385153d24; acceptance a-da7ddcf87198 unchanged.
- round 2 wave 20 (captain): scope small -> LARGE: the acceptance asks for the decrypted literal `hello_world_42` to appear in
  the decompilation of `sub_119a`, which needs the runtime decryption to be evaluated, not just
  the indirect call to be typed. RECORDED RISK: a builder may reasonably argue this bar is not
  reachable by a decompiler and propose a weaker one; that argument belongs in a proposal PR with
  evidence, not in a silent probe rewrite.
- round 2 wave 20 (captain): `touches` LEFT as filed (`[decompiler/crates/kuna-decomp]`). I have measured evidence of the
  symptom but none of the owning module, and a confidently-wrong `touches` would misdirect a
  builder and mis-scope its leases. The builder should set it from its own root-cause work.
