---
need_id: pe-dos-magic
title: A PE whose DOS e_magic is not MZ is rejected before any code is mapped
track: tooling
status: open
severity: blocker
probe_id: p-edc6f397bde5
acceptance_id: a-53f1b10e39e2
hypothesis_status: upheld
credibility: 0.9
instances: 1
challenges: [fo12-4-unholydragon]
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/loader, decompiler/crates/kuna-console/src/engine.rs, decompiler/crates/kuna-cli/src/decompile_all.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Load a PE32 whose first byte was overwritten (e_magic 0x5a15 instead of MZ) and inspect its code.

> **A PE whose DOS e_magic is not MZ is rejected before any code is mapped** (blocker)
> `functions`, `decompile-all`, `decompile`, `disassemble`, `strings` and `xrefs` all exit 1 with
> `not in recognized object file format: Unknown file magic` (or `unrecognized input format`)
> although `e_lfanew` (offset 0x3c) still points at an intact `PE\0\0` signature, COFF header,
> optional header and section table. Seen on a twinBASIC i386 PE32 CTF image whose first
> byte is 0x15. There is no loader override short of `--raw-image`, which loses the
> sections, imports and entry point.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--summary",
    "--json"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_dosmagic_i386.exe",
    "binary_sha256": "e53a812817447fa428948cfd4a6f296747cfa42e2acba4e03f78e474941ac912",
    "binary_size": 1024,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_dosmagic_i386.exe",
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "Unknown file magic"
    ]
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
    "functions",
    "{{BIN}}",
    "--summary",
    "--json"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_dosmagic_i386.exe",
    "binary_sha256": "e53a812817447fa428948cfd4a6f296747cfa42e2acba4e03f78e474941ac912",
    "binary_size": 1024,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_dosmagic_i386.exe",
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "gt",
        "value": 0
      },
      {
        "path": "summary.entry.address_hex",
        "op": "eq",
        "value": "0x401000"
      },
      {
        "path": "warnings[0]",
        "op": "matches",
        "value": "e_magic is 0x5a15, not MZ.*e_lfanew 0x40 -> PE signature"
      }
    ],
    "stderr_matches": [
      "\\[kuna\\] PE DOS header e_magic is 0x5a15, not MZ"
    ],
    "stderr_absent": [
      "Unknown file magic"
    ]
  },
  "notes": "A 1024-byte PE32 (two functions, the pe_datadircount_i386 layout with a well-formed data-directory count) whose first byte is 0x15 instead of 'M', as in the reported image, while e_lfanew still points at an intact PE signature. In-repo so the probe promotes into tests/cli/."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- `object::FileKind::parse` sniffs the format from the first bytes only, so a DOS header
  whose `e_magic` is not `MZ` is never routed to the PE parser, even though every header the
  loader reads lives behind `e_lfanew`. Rewriting `e_magic` in a copy of the bytes (the file
  untouched), exactly as the `NumberOfRvaAndSizes` clamp rewrites its field, should be enough.

## Refutation

_not yet refuted_

## Reference

- `xxd -l 2 UnholyDragon-150.exe` -> `155a`; `xxd -s 0x3c -l 4` -> `e8000000`; `xxd -s 0xe8 -l 4` -> `50450000` (`PE\0\0`), machine 0x14c.

## Instances

- `fo12-4-unholydragon` (manual, UnholyDragon-150.exe)

## Decision log

- filed directly by the builder with an in-repo 1024-byte twin (`pe_dosmagic_i386.exe`) so the acceptance promotes into `tests/cli/`.
