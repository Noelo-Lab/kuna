---
need_id: short-utf-16-window
title: Short UTF-16 window class becomes a narrow string despite explicit type assertions
track: quality
status: closed
severity: major
probe_id: p-47b3ce664b23
acceptance_id: a-abec8c6e76cf
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [65acadf3eef082e477ff5ede]
rounds: [10, 12]
first_seen_round: 10
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-console, decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 593
closed_in_round: 12
closing_pr: "593"
reject_reason: null
---

## Symptom

Preserve UTF-16 `ID` at `0x1400074b0` when decompiling the `FindWindowW` calls,
including after declaring the data type and API prototype.

> **Short UTF-16 window class becomes a narrow string despite explicit type assertions**
> (`65acadf3eef082e477ff5ede`)
> The declaration was reported as applied, but emitted `v128 = "I";` followed by
> `FindWindowW((wchar_t *)v128,NULL);`.

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
    "0x140002530",
    "--assert",
    "data 0x1400074b0 wchar_t window_class[3]",
    "--assert",
    "prototype FindWindowW void *FindWindowW(wchar_t *class_name,wchar_t *window_name)",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "= \"I\";"
    ]
  },
  "target": {
    "binary_rel": "bin/ObfuscationFiesta.exe",
    "binary_sha256": "b852fd5bfa65f7d71b903fb1852ae8ad03f2611637a42d501a897424647deec9",
    "binary_size": 32768,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x401000",
    "--mode",
    "aggressive",
    "--assert",
    "data 0x402000 wchar_t window_class[3]",
    "--assert",
    "prototype find_window void *find_window(wchar_t *class_name,wchar_t *window_name)",
    "--assert-strict"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/short_utf16_window_pe_x86_64.exe",
    "binary_sha256": "4d2b992f86052a3f28b7a1d3d4aa52c2ad4076136d4251a0aece1a2954e5bacc",
    "binary_size": 5038,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/short_utf16_window_pe_x86_64.exe",
    "selector": "0x401000",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "L\"ID\"",
      "L\"OLLYDBG\"",
      "\"ASCII\""
    ],
    "stdout_absent": [
      "sub_401040\\(\"I\",0\\)",
      "\\(wchar_t \\*\\)"
    ]
  },
  "notes": "In-repo PE twin of ObfuscationFiesta.exe 0x140002530: operand_refs first maps the low byte of UTF-16 ID as char[2], then the strict data assertion declares wchar_t[3] at the same address. The neighboring UTF-16 OLLYDBG and ASCII literals are controls. Before the fix the first call renders narrow \"I\"; the dataset witness also casts that local to wchar_t * for FindWindowW."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- Automatic wide-string discovery requires five characters; explicit data
  typing may not reach constant-string rendering.

## Refutation

The five-character premise is refuted: that default belongs to the standalone
`strings` command, while the same function already renders adjacent wide
literals. The operative assertion diagnosis is upheld. `operand_refs` maps the
first UTF-16 code unit as a smaller `char[2]` object before assertions run. The
later typelocked `wchar_t[3]` was added as a second overlapping symbol, so normal
smallest-container lookup continued to select the inferred narrow object.

## Reference

IDA reference was unavailable during filing because no decompiler server was
running. The image bytes independently establish `49 00 44 00 00 00` as UTF-16
`ID`.

## Instances

- `65acadf3eef082e477ff5ede` (round 10, tester `t-r10-65acadf3`)

## Decision log

- filed by `cluster.py` from one observation
- round 10 REFUTER: upheld the assertion-precedence defect and refuted the
  proposed strings-minimum cause; bound both original probe arms to the dataset
  witness and required an in-repo fixture before promotion
- round 12 BUILDER closure (PR #593): traced the collision to two exact-address global
  mappings. Caller data now replaces a non-function mapping at that address,
  preserving the assertion's name, extent, and typelock on both the text-console
  and in-process paths. No string heuristic or `printc` rule changed. Added a
  deterministic PE fixture, CLI acceptance, database precedence test,
  assertion-plane integration test, and stage regression with adjacent UTF-16
  and ASCII controls. Dataset witness now emits `v128 = L"ID";` and
  `FindWindowW(v128,NULL);`.
