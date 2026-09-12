---
need_id: rdtsc-timestamp-incorporates-stale
title: RDTSC timestamp incorporates stale upper bits from the preceding computation
track: quality
status: open
severity: major
probe_id: p-cc77a0a0be92
acceptance_id: a-50993426267c
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [69a3822f7b3cc38c80464da4]
rounds: [11]
first_seen_round: 11
attempts: 0
covered_by_option: null
touches: [specs/Ghidra/Processors/x86/data/languages/ia.sinc]
scope: small
regression_of: null
pr: "574"
closed_in_round: null
closing_pr: "574"
reject_reason: null
---

## Symptom

Reliable timestamp-dependent state updates in helper 0x140011280 and main.

> **RDTSC timestamp incorporates stale upper bits from the preceding computation** (major, `69a3822f7b3cc38c80464da4`)
> The helper merges the second argument's upper bits into the timestamp using CONCAT44. Kuna disassembly at 0x1400112f4 shows RDTSC; SHL RDX,0x20; OR RAX,RDX, which must not retain pre-RDTSC RAX bits. Similar corruption recurs throughout main; helper 0x140011940 uses an uninitialized local for those bits.

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
    "0x140011280"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "CONCAT44\\(\\(int\\)\\(\\(unsigned long long\\)a[0-9]+ >> 0x20\\),\\(int\\)v[0-9]+\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/nikos_crack_me.exe",
    "binary_sha256": "bcfacd743bc607beff4a5b9616d3f9d0335551c07233e07a0a01bf14f0c6e869",
    "binary_size": 235725,
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
    "decompile",
    "{{BIN}}",
    "timestamp"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "return rdtsc\\(\\);"
    ],
    "stdout_absent": [
      "CONCAT44"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/rdtsc_zero_extend_x86_64",
    "binary_sha256": "f7f1219e46801562b2ec5871cbcb29c9ec64294b3d80849ea989e96c3776774d",
    "binary_size": 4664,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/rdtsc_zero_extend_x86_64",
    "selector": "timestamp",
    "selector_kind": "name"
  },
  "notes": "Promoted acceptance of rdtsc-timestamp-incorporates-stale, re-pointed from the dataset witness at a vendored x86-64 fixture. The fixture seeds nonzero upper halves in RAX/RDX before RDTSC and recombines EDX:EAX; the positive return assertion proves the result is exactly the timestamp, while the CONCAT44 absence rejects stale upper bits."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Possibly RDTSC lowering fails to zero-extend EAX into RAX; not independently diagnosed.

## Refutation

The hypothesis is upheld and extended to EDX. The x86 SLEIGH constructor writes EDX and
EAX without invoking the existing 64-bit destination builders, so the upper halves of
both RDX and RAX retain their incoming values. The adjacent RDMSR constructor already
performs the required writes and builds in the correct order.

## Reference

- Intel's instruction-set reference for RDTSC says that the high 32 bits of both RAX and
  RDX are cleared on processors supporting Intel 64.
- The pinned Ghidra source and current Ghidra master both omit the two destination builders;
  this correction is therefore recorded as DIV-164.

## Instances

- `69a3822f7b3cc38c80464da4` (round 11, tester t-r11-69a3822f)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **upheld**. `ia.sinc` writes EDX/EAX without the
  `check_EDX_dest` / `check_EAX_dest` builders used by RDMSR.
- builder: acceptance re-pointed to the vendored `rdtsc_zero_extend_x86_64` fixture and
  strengthened from merely matching `rdtsc` to positively requiring `return rdtsc();`.
