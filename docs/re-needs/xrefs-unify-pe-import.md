---
need_id: xrefs-unify-pe-import
title: xrefs does not unify a PE import thunk with its IAT symbol
track: tooling
status: closed
severity: major
probe_id: p-dc85ba90889e
acceptance_id: a-27cf701da662
hypothesis_status: upheld
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
closed_in_round: 2
closing_pr: "375"
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

**Upheld, and incomplete.** Reproduced and measured by the builder: `kuna functions
--filter VirtualProtect` does expose a veneer at `0x140019980` and an IAT slot at
`0x1400f4690` under one name, `pe_iat` does register the import name on both, and the
references really were attached only to the slot. The record's `inconclusive` hedge was
right about one thing though -- the hypothesis describes the *second* of two defects and
misses the first, which is not PE-specific at all:

1. `data_refs` skipped `in0` for every flow opcode, `BRANCHIND`/`CALLIND` included. An
   indirect flow op's `in0` is not a static target -- `classify` files no edge for it --
   so the skip lost the reference outright. SLEIGH lifts `JMP rm64` as `goto [rm64]`,
   one `BRANCHIND` whose `in0` is the slot, so *every import veneer in every program*
   referenced nothing. Independent witness on the same binary: `xrefs --to 0x1400f46c0`
   (the `wcrtomb` IAT slot) answered 0 although the veneer at `0x140019960` jumps
   through it.
2. The failure is symmetric, not veneer-only as filed. On this crackme every call site
   reads the slot so the *veneer* reports 0; on the vendored `pe_imports.exe` `puts` is
   called through its veneer so the *slot* reports 0. The fix unifies both directions
   rather than folding the veneer into the slot.

## Reference

_none recorded_

## Instances

- `694fd2f60c16072f40f5a4b3` (round 2, tester t-r2-694fd2f6)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): SPLIT from `strings-json-fails-report` -- different subcommand, different gap; they collided only because cluster.py's `_subcommand()` does not know the round-1 subcommands and hashes them all to '?'.
- round 2 T_TRIAGE (captain): track quality -> TOOLING, touches kuna-decomp -> kuna-analysis/src/listing/xrefs.rs. `kuna functions` surfaces a PE import thunk and its IAT symbol under ONE name and `kuna xrefs` then answers 0 for the thunk address -- a wrong answer to a question the tool invited, i.e. a strict bug fix, which by the repo's own rule needs no flag and therefore no counter leases. scope stays small. This is now one of the few needs co-schedulable alongside an option-adding builder.
- closed: acceptance a-27cf701da662 now PASSES at 93fa2e7e0482
