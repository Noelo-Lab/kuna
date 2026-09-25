---
need_id: pemain
title: A stripped PE never names its user entry function main/WinMain
track: quality
status: open
severity: major
probe_id: null
acceptance_id: null
hypothesis_status: unrefuted
credibility: 0.9
instances: 1
challenges: []
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis, decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find where a stripped Windows program starts.

> **A stripped PE never names its user entry function** (major)
> kuna names `main` on ELF (`elfmain`) and Mach-O (`machomain`) but not on PE. On a stripped PE `kuna functions` lists the CRT startup as the entry and the function it eventually calls (MSVC `mainCRTStartup -> __scrt_common_main_seh -> invoke_main -> main`, MinGW `__tmainCRTStartup -> main`) as one more `sub_<addr>`. On Flare-On 12 `hopeanddreams.exe` that is `sub_14020ba00` among 1511 functions, reached by hand from entry `sub_14044ca84` through `sub_14044c908`. `functions --summary` reports only the entry and has no field for `main`.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--summary",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "summary.entry.address_hex",
        "op": "eq",
        "value": "0x1400014f0"
      },
      {
        "path": "summary.main",
        "op": "absent"
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_imports_stripped.exe",
    "binary_sha256": "981f4d8e9bba6b4bb039d91253f54e6f068d7513930e1f0ab5979b5bf7fa6cad",
    "binary_size": 39424,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_imports_stripped.exe"
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
    "functions",
    "{{BIN}}",
    "--summary",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "summary.entry.address_hex",
        "op": "eq",
        "value": "0x1400014f0"
      },
      {
        "path": "summary.main.name",
        "op": "eq",
        "value": "main"
      },
      {
        "path": "summary.main.address_hex",
        "op": "eq",
        "value": "0x140001592"
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_imports_stripped.exe",
    "binary_sha256": "981f4d8e9bba6b4bb039d91253f54e6f068d7513930e1f0ab5979b5bf7fa6cad",
    "binary_size": 39424,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_imports_stripped.exe"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- The in-image CRT startup calls the user entry from a site recognisable per toolchain (UCRT accessor imports, the WinMain command-line accessor plus `&__ImageBase`, MinGW's `__initenv` store, the static-CRT `invoke_main` shape); nothing reads it for a name.

## Refutation

_not yet refuted_

## Reference

IDA Pro and Ghidra label `main`/`WinMain` on all three Flare-On 12 PE samples (`hopeanddreams.exe` 0x14020ba00, `ntfsm.exe` 0x14000c0b0, `FlareAuthenticator.exe`).

## Instances

- filed directly (Flare-On 12 triage)

## Decision log

- filed with an in-repo stripped MinGW fixture (`pe_imports_stripped.exe`, whose unstripped twin `pe_imports.exe` names `main` at 0x140001592).
