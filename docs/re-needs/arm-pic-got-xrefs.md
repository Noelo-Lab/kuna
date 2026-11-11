---
need_id: arm-pic-got-xrefs
title: ARM -fPIC callback read through the GOT has no reference
track: tooling
status: closed
severity: major
probe_id: p-fd504c2f4ae3
acceptance_id: a-336a275d03d0
hypothesis_status: upheld
credibility: 0.9
instances: 2
challenges: []
rounds: []
first_seen_round: null
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/kuna_picpool.rs, decompiler/crates/kuna-analysis/src/listing/xrefs.rs]
scope: small
regression_of: null
pr: 1264
closed_in_round: null
closing_pr: "1264"
reject_reason: null
---

## Symptom

Find who registers the timer callback `on_expire`.

> **ARM -fPIC callback read through the GOT has no reference** (major, synthetic `gotxref_arm_le32`)
> `kuna xrefs --to on_expire` reports 0 references, although `arm_timers` loads it from its GOT slot and passes it to `set_timeout`. The string literals of the same function also have no xref, and the raw pool words are filed as data references into `.text`.

Seen first on a production ARM executable (GCC, `-fPIC`, linked non-PIE at 0x10000); `gotxref_arm_le32` reproduces it from a small C source.

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
    "on_expire",
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
    "on_expire",
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
        "value": 1
      },
      {
        "path": "xrefs[0].from_address_hex",
        "op": "eq",
        "value": "0x10144"
      },
      {
        "path": "xrefs[0].kind",
        "op": "eq",
        "value": "data"
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/gotxref_arm_le32",
    "binary_sha256": "7fd54d44cf3d1e1464abd08a9d593f59d2ac94225a50dc2f2eff311b0aaecfb2",
    "binary_size": 71396,
    "binary_source": "in-repo"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- `picpool` declines a pool word that is itself a mapped address, so in an image mapped low a PC displacement that lands in a section by chance forms nothing, and `poolref` files it as a pointer.
- Nothing models the ARM GOT base (`ldr r6,[pc]; add r6,pc,r6`, kept in a callee-saved register across calls and joins), so `ldr r3,[r6,r3]` is never followed to the slot or to the address the linker wrote there.

## Refutation

_not yet refuted_

## Reference

- Ghidra 12.1.4 finds both through `SymbolicPropogator` constant propagation and its ELF loader's `.got` pointer markup.

## Instances

- `gotxref_arm_le32` (in-repo, synthetic)
- `gotxref_thumb_le32` (in-repo, synthetic): the same in Thumb-2, where the literal loads lift as copies out of the image rather than loads, plus a `tbb` switch whose cases read slots through the dispatch's base

## Decision log

- filed by hand from one vendor binary and its synthetic reduction; prior art: `arm-pic-literal-pool` (#474)
- closed: acceptance `a-336a275d03d0` passes with #1264 (promoted to `tests/cli/arm-pic-got-callback-xref.json`, with `arm-pic-mapped-displacement-string` and `arm-pic-displacement-not-a-pointer`); the reproduction still passes on the unpatched tree.
