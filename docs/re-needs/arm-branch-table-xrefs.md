---
need_id: arm-branch-table-xrefs
title: ARM switch cases behind an A32 jump table have no references
track: tooling
status: open
severity: major
probe_id: p-db34e7133577
acceptance_id: a-f297e3e8ecb3
hypothesis_status: upheld
credibility: 0.9
instances: 1
challenges: []
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/xrefs.rs, decompiler/crates/kuna-analysis/src/listing/kuna_a32switch.rs, decompiler/crates/kuna-analysis/src/listing/classify.rs, decompiler/crates/kuna-analysis/src/listing/kuna_thumbswitch.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find who registers the callback `on_read`.

> **ARM switch cases behind an A32 jump table have no references** (major, synthetic `branchtable_arm_le32`)
> `kuna xrefs --to on_read` reports 0 references, although a case of `add_dispatch` and a case of `ldr_dispatch` each load it from its GOT slot and pass it to `set_timeout`. The walk follows only `b default` after `addls pc,pc,rN,lsl #2` or `ldrls pc,[pc,rN,lsl #2]`, and files a jump from the dispatch to that `b`.

Seen first on production ARM executables and a shared library (GCC, `-fPIC`), where the switch cases held most of the references kuna missed; `branchtable_arm_le32` reproduces both table forms from a small C source.

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
    "on_read",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 0
      }
    ]
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
    "on_read",
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
        "op": "eq",
        "value": 2
      },
      {
        "path": "xrefs[0].from_address_hex",
        "op": "eq",
        "value": "0x101ec"
      },
      {
        "path": "xrefs[0].from_function.name",
        "op": "eq",
        "value": "add_dispatch"
      },
      {
        "path": "xrefs[1].from_address_hex",
        "op": "eq",
        "value": "0x10338"
      },
      {
        "path": "xrefs[1].from_function.name",
        "op": "eq",
        "value": "ldr_dispatch"
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/branchtable_arm_le32",
    "binary_sha256": "93b3aa37fc16c266959a8e8d4f47a117e96fcdae82aeff124b185f7f84bb5126",
    "binary_size": 72920,
    "binary_source": "in-repo"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- The reference walk reads a computed jump's table only through a table base the dispatch names or a register chain, and an A32 table's base is the dispatch's own PC, so neither answers. The Listing walk already follows both forms under `armframes` (`kuna_a32switch`).

## Refutation

_not yet refuted_

## Reference

- Ghidra 12.1.4 recovers both through `ArmAnalyzer`'s switch recovery and adds a computed-jump reference from the dispatch to each case.

## Instances

- `branchtable_arm_le32` (in-repo, synthetic)

## Decision log

- filed by hand from vendor binaries and their synthetic reduction; prior art: `arm-pic-got-xrefs`
- the reference walk reads both forms under `armframes` alone (no `listing` or `funcstart_patterns` prerequisite, unlike frame recovery); the predicated skip is the dispatch's fall-through only where A32 tables are read, so `armframes off` and other architectures keep the skip row; a dispatch reached before its guard is read again once the guard is decoded (`early_dispatch` in the fixture)
