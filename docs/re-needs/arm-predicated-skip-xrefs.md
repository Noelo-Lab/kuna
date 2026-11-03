---
need_id: arm-predicated-skip-xrefs
title: Every predicated ARM instruction files a jump to the next instruction
track: tooling
status: open
severity: minor
probe_id: p-6ea0c401dba0
acceptance_id: a-7e864af9d414
hypothesis_status: upheld
credibility: 0.9
instances: 1
challenges: []
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/classify.rs, decompiler/crates/kuna-analysis/src/listing/xrefs.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find what branches to the `ldreq` in `pick`.

> **Every predicated ARM instruction files a jump to the next instruction** (minor, synthetic `xreffp_arm_le32`)
> `kuna xrefs --to 0x10568` reports a jump from `0x10564 ldrne r0,[r0,#0x4]`, the instruction before it. No instruction branches there: `ldrne` is not a branch. Every predicated A32 instruction (`movle`, `ldrne`, `popeq {..,pc}`) and every instruction in a Thumb `it` block files such a row.

Seen first on production ARM executables and a shared library, where these rows were the second-largest false positive; `xreffp_arm_le32` reproduces them from a small C source.

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
    "0x10568",
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
        "value": 1
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
    "0x10568",
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
        "value": 0
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

- SLEIGH lifts a predicated instruction to a `CBRANCH` to the next instruction ahead of its own ops, and `classify` reports that skip as one of the instruction's flows, which the reference walk files as a `Jump`. Ghidra keeps a `goto inst_next` out of an instruction's flows.

## Refutation

_not yet refuted_

## Reference

- Ghidra 12.1.4 and IDA file no row for a predicated instruction's skip; both keep a conditional branch to the next instruction.

## Instances

- `xreffp_arm_le32` (in-repo, synthetic)

## Decision log

- filed by hand from vendor binaries and their synthetic reduction; prior art: `arm-branch-table-xrefs`, which stopped the row for a predicated table dispatch under `armframes` only
- `Classified::condition_skip` reads a `CBRANCH` to the fall-through that other ops follow from the instruction's ops (not stored in `Classified` or the decode memo); the skip stays a flow for every consumer and only the reference walk files no row for it, under every option; a `bne` to the next instruction is a lone `CBRANCH` and keeps its row
- a skip that runs into another known function's entry keeps its row: the call graph reads it as a way into that function, and dropping it changed `decompile-all` output (anonymous struct numbering in 62 musl `libc.so` functions)
- the rule is ARM-only: the same `goto inst_next` skip of x86 `cmovcc`/`rep`, AArch64 `ccmp`/`fcsel`, PPC `beqlr`, MIPS `bc1t` and the SPARC `call` keeps its row; cleaning those up is a separate change
- the cross-entry rule is decided after the walk over every entry it found (the first version checked only the seeds, so a skip into an entry found from a call lost its row on a stripped image); `xreffp_strip_arm_le32` covers it
- only the first flow equal to the skip is the skip: a predicated branch to the next instruction (`it eq; beq.w`, flows `[next, next]`) keeps the branch's row
- a skip into an entry is kept only from an instruction with no fall-through (`bxne lr`, `ldrne pc`, `addls pc`): a predicated `ldrne`/`moveq`/`andeq` right before an entry reaches it by its fall-through too, so its skip is no way in of its own and files nothing; this renumbers anonymous structs in 62 musl `libc.so` functions, like the PLT rule in `libstdc++.so`
- gated behind `armcondskip` (default on): the change renumbers anonymous structs in some functions of large ARM images, so it ships as an option; off gives the earlier rows and `decompile-all` output byte for byte
