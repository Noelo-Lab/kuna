---
need_id: arm-plt-stub-partial-xrefs
title: ARM PLT stubs file their partial addresses as data references
track: tooling
status: open
severity: minor
probe_id: p-0d31764822bb
acceptance_id: a-3d610a512268
hypothesis_status: upheld
credibility: 0.9
instances: 1
challenges: []
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/xrefs.rs, decompiler/crates/kuna-analysis/src/listing/kuna_pltstub.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find what references 0x12000, an address inside `table`.

> **ARM PLT stubs file their partial addresses as data references** (minor, synthetic `xreffp_arm_le32`)
> `kuna xrefs --to 0x12000` reports 7 data references, one from each PLT stub's `add r12,r12,#0x12000`, and `kuna xrefs --from printf` reports `adr r12,0x10318` and that add instead of the GOT slot `0x2300c` the stub jumps through. Neither value is an address the stub uses.

Seen first on production ARM executables and a shared library (GNU ld, `-fPIC`), where these rows outnumbered every other false positive; `xreffp_arm_le32` reproduces the stub from a small C source.

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
    "0x12000",
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
        "value": 7
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
    "--from",
    "printf",
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
        "path": "xrefs[0].to_address_hex",
        "op": "eq",
        "value": "0x2300c"
      },
      {
        "path": "xrefs[0].kind",
        "op": "eq",
        "value": "read"
      },
      {
        "path": "xrefs[0].from_address_hex",
        "op": "eq",
        "value": "0x10318"
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/xreffp_arm_le32",
    "binary_sha256": "33c2504a9e043837b83c7668ab36b4e602bff2bb198728af6980a96839ff8e92",
    "binary_size": 77140,
    "binary_source": "in-repo"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- The constant scan reads each instruction on its own, so the PC value of `adr r12` and the rotated immediate of `add r12,r12,#N` are filed whenever they land in a mapped section. Evaluating the stub's three instructions together gives the slot.

## Refutation

_not yet refuted_

## Reference

- Ghidra 12.1.4 files no row from the stub's adds; IDA files the slot read from the `ldr pc`.

## Instances

- `xreffp_arm_le32` (in-repo, synthetic)

## Decision log

- filed by hand from vendor binaries and their synthetic reduction; prior art: `arm-pic-got-xrefs`
- the walk recognises `reg = constant`, one or two `reg += constant`, then a `BRANCHIND` through a load of `reg + constant`; the chain files no constant data row and the load files the slot as a read; the slot's word (the PLT resolver) is not followed
- the matcher follows the chain as the walk decodes it (rows of an open chain held, released when it breaks or the body drains) instead of decoding ahead, so it uses the walk's mode selection and commits no context out of order
- a chain starts on any constant register write (not only one whose value is mapped); the slot is the word the `BRANCHIND` consumes and must be mapped; the matcher runs on ARM only
- a chain's add must be computed from its register (`mov ip,#K` breaks it); a reloaded register names no slot; chains run only inside the linker stub sections (`.plt`, `.iplt`), and an image without section headers follows none
- gated behind `armpltslot` (default on): the change renumbers anonymous structs in some functions of large ARM images, so it ships as an option; off gives the earlier rows and `decompile-all` output byte for byte
