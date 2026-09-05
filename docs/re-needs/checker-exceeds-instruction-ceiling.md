---
need_id: checker-exceeds-instruction-ceiling
title: Checker exceeds instruction ceiling with no discoverable override
track: tooling
status: closed
severity: blocker
probe_id: p-59baa2cff0a9
acceptance_id: a-d0071cb49b29
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [69d6affb110488a3205426e2]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src, decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs]
scope: small
regression_of: null
pr: 422
closed_in_round: 3
closing_pr: 422
reject_reason: null
---

## Symptom

Decompile the password checker or explicitly raise its instruction budget.

> **Checker exceeds instruction ceiling with no discoverable override** (blocker, `69d6affb110488a3205426e2`)
> The 6,981,712-byte checker returns code:null and Flow exceeded maximum allowable instructions. Enabling calleearity, varargstackargs and switchselector also fails. No instruction-budget override was found in the catalog or CLI documentation.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--functions",
    "sub_140001000",
    "--max-fn-seconds",
    "30",
    "--json"
  ],
  "expect": {
    "json": [
      {
        "path": "functions[0].error",
        "op": "matches",
        "value": "Flow exceeded maximum allowable instructions"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "72336301c26c106024d5ade1470fd10580bf444b53107b14908dfb12e50f0fe6",
    "binary_size": 7131136,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--functions",
    "sub_140001000",
    "--max-fn-seconds",
    "30",
    "--json"
  ],
  "expect": {
    "json": [
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "\\S"
      },
      {
        "path": "functions[0].error",
        "op": "eq",
        "value": null
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "72336301c26c106024d5ade1470fd10580bf444b53107b14908dfb12e50f0fe6",
    "binary_size": 7131136,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- A fixed flow instruction budget prevents analysis of this unusually large checker.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69d6affb110488a3205426e2` (round 3, tester t-r3-69d6affb)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_TRIAGE r3: track tooling CONFIRMED, touches CORRECTED to name the ceiling's real home. Verified on cf5234ac: none of the 149 catalog options is the instruction ceiling (no match for instr/max/ceil/budget), so `--option` genuinely cannot reach it; the value is Architecture::max_instructions, set at infra/decompile_drive.rs:580 from the console-only `maxinstruction`. The need is reachability, so this is CLI plumbing of an existing knob -- no new option, no stages case.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- closed: acceptance a-d0071cb49b29 now PASSES at 80e965ca649d
captain B_DONE r3 (re-applied): closed by #422 (80e965ca); the first application of this bookkeeping was lost when the main tree was checked out to branch docs/restore-round3-records and back, so it was redone from rounds/3/acceptance-suite.json (sha 80e965ca, the current HEAD).
