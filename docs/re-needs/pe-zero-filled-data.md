---
need_id: pe-zero-filled-data
title: PE zero-filled data is reported as outside every loaded segment
track: loader
status: open
severity: minor
probe_id: p-c496f8860fe8
acceptance_id: a-7020a60fa5e5
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [640a526833c5d447bc761899]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/disassemble.rs, decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Read the initial global state at 0x140008740.

> **PE zero-filled data is reported as outside every loaded segment** (minor, `640a526833c5d447bc761899`)
> Exits 1 claiming no loaded segment covers the address and suggests unpacking. The .data section starts at RVA 0x8000 with VirtualSize 0x7a8 and raw size 0x200; all 64 requested bytes lie inside its zero-filled virtual tail.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "read",
    "{{BIN}}",
    "0x140008740",
    "--addr",
    "--bytes",
    "64",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "no loaded segment covers it"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_bsstail_x86_64.exe",
    "binary_sha256": "f52dc6962c6fadfbe5a287286933de4ef22bb34d7eb9750a161bb2bceefcb101",
    "binary_size": 1536,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_bsstail_x86_64.exe"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "read",
    "{{BIN}}",
    "0x140002740",
    "--addr",
    "--bytes",
    "64",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "\"hex\": \"0{128}\""
    ],
    "stderr_absent": [
      "no loaded segment covers it"
    ]
  },
  "notes": "Vendored twin of the reported image (kuna-analysis/tests/fixtures/pe_bsstail_x86_64.py): .data with VirtualSize 0x7a8 over SizeOfRawData 0x200, at RVA 0x2000. 0x140002740 sits 0x540 into the zero-filled tail, as 0x140008740 does in the original, which stays the witness in Reproduction. In-repo, so the probe runs where there is no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The read path may use file-backed length instead of PE virtual size.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `640a526833c5d447bc761899` (round 8, tester t-r8-640a5268)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 TRIAGE (captain): RETRACKED quality -> loader, RETOUCHED off the source. Nothing about this is emitted C: the failing surface is kuna read, and the 'no loaded segment covers the address' string lives in kuna-cli/src/disassemble.rs (the shared image-read seam), with the PE section mapping behind it in kuna-analysis. The gap is a section mapped at raw size instead of VirtualSize, so the zero-filled virtual tail of .data (RVA 0x8000, VirtualSize 0x7a8, raw 0x200) is unreadable. No option, no phases.toml row, no counters -- so it holds no lease and is dispatchable alongside a quality builder.
