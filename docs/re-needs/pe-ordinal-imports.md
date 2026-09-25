---
need_id: pe-ordinal-imports
title: PE imports by ordinal from OLEAUT32/WS2_32 render as <DLL>_Ordinal_<n>
track: quality
status: closed
severity: major
probe_id: p-379af488fc2f
acceptance_id: a-f99c3e5c2e57
hypothesis_status: upheld
credibility: 0.9
instances: 1
challenges: []
rounds: []
first_seen_round: null
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-analysis, decompiler/crates/kuna-decomp, decompiler/crates/kuna-cli, decompiler/crates/kuna-console]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Read a PE that imports the BSTR/VARIANT API or WinSock by ordinal and see API names.

> **PE imports by ordinal render as `<DLL>_Ordinal_<n>`** (major)
> Flare-On 12 challenge 4 (`UnholyDragon-150.exe`, first byte patched back to `M`) imports 45
> OLEAUT32 functions by ordinal only. Every call, `kuna functions` entry and `kuna xrefs` target
> reads `OLEAUT32_Ordinal_2`, `OLEAUT32_Ordinal_6`, `OLEAUT32_Ordinal_150`, ... and none of them
> gets a Win32 prototype, because the signature tables are keyed by name.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x401200",
    "--addr"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_ordinal_i386.exe",
    "binary_sha256": "467116a4503a003f6e11bc7b5abd0bf14c5a43103a767e379c1e82d55509aff2",
    "binary_size": 4608,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_ordinal_i386.exe",
    "selector": "0x401200",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "OLEAUT32_Ordinal_6\\(OLEAUT32_Ordinal_2\\(",
      "WS2_32_Ordinal_23\\(2,1,6\\)",
      "WS2_32_Ordinal_115\\("
    ]
  },
  "notes": "Every import in the fixture is by ordinal. Without a name table the covered system DLLs render as <DLL>_Ordinal_<n>."
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
    "0x401200",
    "--addr"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_ordinal_i386.exe",
    "binary_sha256": "467116a4503a003f6e11bc7b5abd0bf14c5a43103a767e379c1e82d55509aff2",
    "binary_size": 4608,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_ordinal_i386.exe",
    "selector": "0x401200",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "SysFreeString\\(SysAllocString\\(",
      "socket\\(2,1,6\\)",
      "WSAStartup\\(0x202,0\\)",
      "MYLIB_Ordinal_7\\(\\)",
      "OLEAUT32_Ordinal_9999\\(\\)"
    ],
    "stdout_absent": [
      "OLEAUT32_Ordinal_[26]\\b",
      "WS2_32_Ordinal_"
    ]
  },
  "notes": "OLEAUT32 #2/#6 and WS2_32 #23/#115 are named from the built-in export tables (SysAllocString, SysFreeString, socket, WSAStartup). An uncovered DLL (MYLIB #7) and an ordinal outside the OLEAUT32 table (#9999) keep the synthesized name."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- `loader/pe_iat.rs` synthesizes `<DLL>_Ordinal_<n>` for every ordinal-only import. OLEAUT32,
  WS2_32 (WinSock 1.1 range), WSOCK32 and MSVBVM60 have fixed, public ordinal tables, so a
  built-in table can name them before the name reaches the symbol table, xrefs and `win32sigs`.

## Refutation

_not yet refuted_

## Reference

- `objdump -p UnholyDragon-150.exe` lists the OLEAUT32 import block as `<none>` hint/name entries
  with ordinals 2, 4, 6, 9, 10, 11, ... 419.

## Instances

- Flare-On 12 challenge 4 (`UnholyDragon-150.exe`), manual validation only (not vendored).

## Decision log

- filed with a vendored in-repo fixture, `pe_ordinal_i386.exe`, so the probe runs in CI.
- closed: acceptance `a-f99c3e5c2e57` passes with option `peordinal` (default on); the reproduction
  still passes on the unpatched tree and under `--option peordinal off`. Manually validated on
  `UnholyDragon-150.exe`: all 45 OLEAUT32 ordinal imports resolve (`SysAllocStringByteLen`,
  `SysFreeString`, `VariantClear`, `SafeArrayAccessData`, ...), `kuna xrefs --to SysFreeString`
  lists 602 references.
