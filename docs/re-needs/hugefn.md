---
need_id: hugefn
title: Giant functions and jump tables give no warning before decompiling, and the jump-table cap is not catalogued
track: tooling
status: open
severity: major
probe_id: p-3fe5ee026f79
acceptance_id: a-f06d350ede15
hypothesis_status: upheld
credibility: 1.0
instances: 1
challenges: []
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile_all.rs, decompiler/crates/kuna-analysis/src/listing/xrefs.rs, decompiler/crates/kuna-analysis/src/listing/kuna_switchtable.rs, decompiler/crates/kuna-decomp/phases.toml]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Know, before decompiling, which functions will blow the `maxinstruction` budget or
dispatch through a switch longer than the jump-table cap, and raise the cap from a
catalogued option.

> **Giant state machine gives no warning** (major, Flare-On 12 challenge 5 `ntfsm.exe`)
> `main` at 0x14000c0b0 is a 16.6 MB extent of about 2.5M instructions that dispatches
> through a 90,781-entry image-base-relative table at 0x14000ca5a (`cmp idx,0x1629c`).
> `kuna functions --summary --json` lists it only as the largest function; nothing says
> the switch is over the 1024-entry cap or that the body is over `maxinstruction`, and
> `kuna catalog` has no jump-table option (the analysis tier's own table read was a
> hard-coded 1024 that no option reached).

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
    "--json",
    "--option",
    "jumptablemax",
    "2",
    "--option",
    "maxinstruction",
    "10"
  ],
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "summary.largest[0].address_hex",
        "op": "eq",
        "value": "0x140001040"
      },
      {
        "path": "summary.limits",
        "op": "absent"
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_switchdelta_x86_64.exe",
    "binary_sha256": "013837321f893184671d054265b24d0c8a7055dff3db5c5481be863a4c351177",
    "binary_size": 3072,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_switchdelta_x86_64.exe"
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
    "--json",
    "--option",
    "jumptablemax",
    "2",
    "--option",
    "maxinstruction",
    "10"
  ],
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "summary.limits.maxinstruction",
        "op": "eq",
        "value": 10
      },
      {
        "path": "summary.limits.jumptablemax",
        "op": "eq",
        "value": 2
      },
      {
        "path": "summary.limits.over[0].address_hex",
        "op": "eq",
        "value": "0x140001040"
      },
      {
        "path": "summary.limits.over[0].over_maxinstruction",
        "op": "eq",
        "value": true
      },
      {
        "path": "summary.limits.over[0].switches_over_jumptablemax[0].address_hex",
        "op": "eq",
        "value": "0x14000105d"
      },
      {
        "path": "summary.limits.over[0].switches_over_jumptablemax[0].cases",
        "op": "eq",
        "value": 4
      },
      {
        "path": "summary.limits.over[0].switches_over_jumptablemax[0].read",
        "op": "eq",
        "value": 2
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_switchdelta_x86_64.exe",
    "binary_sha256": "013837321f893184671d054265b24d0c8a7055dff3db5c5481be863a4c351177",
    "binary_size": 3072,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_switchdelta_x86_64.exe"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- The reference walk already decodes every body and reads every switch's range check,
  so the budgets can be measured there and reported by the inventory surfaces.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- Flare-On 12 challenge 5 `ntfsm.exe`, `main` at 0x14000c0b0 (manual validation only; not vendored)

## Decision log

- filed by the builder with a vendored fixture: `pe_switchdelta_x86_64.exe` has the same MSVC delta-table dispatch behind a four-case range check, so `--option jumptablemax 2` puts it over the cap and `--option maxinstruction 10` puts its 16-instruction body over the budget.
