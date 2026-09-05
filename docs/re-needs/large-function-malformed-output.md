---
need_id: large-function-malformed-output
title: a large checker decompiles into malformed and prohibitively noisy C
track: quality
status: open
severity: major
probe_id: p-8355fed97a86
acceptance_id: a-f6e8100daa0b
hypothesis_status: inconclusive
credibility: 0.8
instances: 1
challenges: [6a3822888a86e4c2c55254fe]
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

Readable pseudocode for the core checker at 0x140002d50, with coherent C++ string objects and crypto-call arguments.

> **Large checker decompiles into malformed and prohibitively noisy C** (blocker, `6a3822888a86e4c2c55254fe`)
> It emitted over 1,200 lines and 165 anonymous variables. The output included impossible conditions such as `v13 != v13 + 6`, raw synthetic stack-address writes, and badly reconstructed BCrypt calls.

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
    "0x140002d50",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "if\\ \\(v13\\ !=\\ \\(unsigned\\ int\\ \\*\\)\\(\\(long\\ long\\)v13\\ \\+\\ 6\\)\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/selam.exe",
    "binary_sha256": "01f288daf9ca08857b51c550a0e4b1d422724de883d984780fb8c493f2e2c859",
    "binary_size": 240128,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "p-8355fed97a86"
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
    "0x140002d50",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "std::string"
    ]
  },
  "target": {
    "binary_rel": "bin/selam.exe",
    "binary_sha256": "01f288daf9ca08857b51c550a0e4b1d422724de883d984780fb8c493f2e2c859",
    "binary_size": 240128,
    "binary_source": "dataset"
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "a-f6e8100daa0b"
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Stack-variable splitting and C++ small-string-object type recovery fail together in this optimized function.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6a3822888a86e4c2c55254fe` (round 1, tester t-r1-6a382288)

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
- round 2 wave 20 (captain): scope small -> LARGE: the acceptance asks for `std::string` in a 1,200-line optimized MSVC C++
  checker, i.e. C++ object/type recovery, not a local print fix. Expect a proposal fork.
- round 2 wave 20 (captain): `touches` LEFT as filed (`[decompiler/crates/kuna-decomp]`). I have measured evidence of the
  symptom but none of the owning module, and a confidently-wrong `touches` would misdirect a
  builder and mis-scope its leases. The builder should set it from its own root-cause work.
