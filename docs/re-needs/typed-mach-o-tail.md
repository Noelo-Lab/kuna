---
need_id: typed-mach-o-tail
title: Typed Mach-O tail stub drops both forwarded arguments
track: quality
status: open
severity: major
probe_id: p-4d8cc41166db
acceptance_id: a-0f1c487a5eb9
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [64c8b272b25df8732eebc2a6]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Preserve arguments through an indirect Mach-O import tail jump, including when
the import pointer is explicitly typed.

> **Typed Mach-O tail stub drops both forwarded arguments** (major,
> `64c8b272b25df8732eebc2a6`)
> Emitted `(*strcmp_ptr)()` despite the two-argument signature and explicit
> pointer type. Disassembly shows one JMP through the import slot, preserving
> RDI and RSI.

The vendored `macho_imports` reduction has the same one-instruction x86-64
shape at `printf` (`0x1000005cc`):

```text
ff 25 2e 2a 00 00    jmp qword ptr [0x100003000]
```

Before the repair, even the stronger two-parameter declaration below rendered
`(*printf_ptr)()` while the surrounding function declared `fmt` and `value`.

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
    "0x100003ede",
    "--addr",
    "--assert",
    "data 0x100008030 int (*strcmp_ptr)(char *,char *)"
  ],
  "expect": {
    "exit_code": { "eq": 0 },
    "stdout_matches": [
      "\\(\\*strcmp_ptr\\)\\(\\s*\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/main",
    "binary_sha256": "979555f6b20fc5f024358ef26603f4c25e8db326941f86af30726eca9fea453e",
    "binary_size": 50080,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-0f1c487a5eb9",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "printf",
    "--assert",
    "prototype 0x1000005cc int printf(char *fmt,int value)",
    "--assert",
    "data 0x100003000 int (*printf_ptr)(char *,int)",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": { "eq": 0 },
    "stdout_matches": [
      "int printf\\(char \\*fmt,int value\\)",
      "\\(\\*printf_ptr\\)\\(fmt,value\\)"
    ],
    "stdout_absent": [
      "\\(\\*printf_ptr\\)\\(\\s*\\)"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/macho_imports",
    "binary_sha256": "0394e1be81c4ccdfb28a50cf5dba55a6d95a692cd5504f2305a8f139e8f86675",
    "binary_size": 16688,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/macho_imports",
    "selector": "printf",
    "selector_kind": "name"
  },
  "notes": "Positive match is required: merely losing the asserted slot name would also make the historical absent-only clause pass."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** The CALLIND call spec never
consulted the exact global slot supplying its target. Consequently both a
prototype-bearing function-pointer data assertion and the address-keyed import
prototype parked by the library analysis were available but inert.

## Refutation

The hypothesis was upheld on the original dataset witness and on the vendored
reduction. The latter lifts the instruction as a CALLIND whose target is the
unwritten RAM Varnode at `0x100003000`; no explicit LOAD survives in that SLEIGH
form.

## Reference

- IDA, Ghidra, and angr backends were attempted through the repository's
  decompiler workflow. Each server exited before registering, so no external
  reference pseudocode was available. The machine instruction and Kuna raw
  p-code above are the available ground truth.

## Instances

- `64c8b272b25df8732eebc2a6` (round 12, tester t-r12-64c8b272)

## Decision log

- Filed by `cluster.py` from one observation and split by the captain from the
  broader argument-loss family: in that family a call-site prototype restores
  arguments, while here an explicit function-pointer type did not change arity.
- Round 12 refutation upheld the symptom. The original dataset stub is one
  `JMP qword ptr [0x100008030]`; the emitted `strcmp` function declared two
  inputs and used neither.
- The historical acceptance had only an absent clause for `(*strcmp_ptr)()`.
  It could pass if the asserted name disappeared while the empty call remained,
  so the promoted acceptance pins both the declared parameters and the positive
  `(*printf_ptr)(fmt,value)` call.
