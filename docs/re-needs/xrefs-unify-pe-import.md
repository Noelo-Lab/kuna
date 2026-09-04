---
need_id: xrefs-unify-pe-import
title: xrefs does not unify a PE import thunk with its IAT symbol
track: tooling
status: open
severity: major
probe_id: p-dc85ba90889e
acceptance_id: a-27cf701da662
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [694fd2f60c16072f40f5a4b3]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/xrefs.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find callers/references of the VirtualProtect function surfaced by `kuna functions` at 0x140019980

> **xrefs does not unify a PE import thunk with its IAT symbol** (major, `694fd2f60c16072f40f5a4b3`)
> `kuna functions --filter VirtualProtect` exposed both a thunk at 0x140019980 and an IAT symbol at 0x1400f4690 under the same name. xrefs to the thunk returned count 0, although decompilation calls VirtualProtect; only manually targeting the second alias address returned the two references.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x140019980",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "target.name",
        "op": "eq",
        "value": "VirtualProtect"
      },
      {
        "path": "count",
        "op": "eq",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.rar.__x/system.exe",
    "binary_sha256": "a06370d535da9a8aaee5a66c0a35c9b39487cb538b64248736d741e030c468ed",
    "binary_size": 986624,
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
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x140019980",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "gt",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.rar.__x/system.exe",
    "binary_sha256": "a06370d535da9a8aaee5a66c0a35c9b39487cb538b64248736d741e030c468ed",
    "binary_size": 986624,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The PE loader creates separate thunk and IAT function records with the same import name, while xrefs are attached only to the IAT record and are not folded across import aliases.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `694fd2f60c16072f40f5a4b3` (round 2, tester t-r2-694fd2f6)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): SPLIT from `strings-json-fails-report` -- different subcommand, different gap; they collided only because cluster.py's `_subcommand()` does not know the round-1 subcommands and hashes them all to '?'.
- round 2 T_TRIAGE (captain): track quality -> TOOLING, touches kuna-decomp -> kuna-analysis/src/listing/xrefs.rs. `kuna functions` surfaces a PE import thunk and its IAT symbol under ONE name and `kuna xrefs` then answers 0 for the thunk address -- a wrong answer to a question the tool invited, i.e. a strict bug fix, which by the repo's own rule needs no flag and therefore no counter leases. scope stays small. This is now one of the few needs co-schedulable alongside an option-adding builder.
