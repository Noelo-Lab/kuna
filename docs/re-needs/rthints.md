---
need_id: rthints
title: Orientation does not say when an image is a bundle, managed assembly or packed stub
track: tooling
status: closed
severity: major
probe_id: p-43cf2fa671d8
acceptance_id: a-ed9f64ae2b32
hypothesis_status: upheld
credibility: 0.9
instances: 3
challenges: [fo12-chat_client, fo12-DrillBabyDrill, fo12-UnholyDragon]
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: 734
closed_in_round: null
closing_pr: "734"
reject_reason: null
---

## Symptom

`kuna functions --summary --json` is the first call on an unknown binary, and it says nothing
when native decompilation is the wrong tool. A PyInstaller bundle (Flare-On 12 `chat_client`,
`DrillBabyDrill.exe`) orients as 182 / 958 ordinary C functions of the bootloader; the Python
program, and the version needed to decompile its bytecode, sit in the CArchive cookie at the end of
the file. The same holds for a managed .NET assembly (only the `_CorExeMain` stub is native), a UPX
stub, an AutoIt script, a Nuitka build and a VB6/twinBASIC program: an agent spends its budget
decompiling the wrapper before it realises which tool it needed.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64",
    "binary_sha256": "aecc729c1f63c2f66fc8a2c3b9563e4db09aff86f4ba803d14bb5cc1f060ce92",
    "binary_size": 5128,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64"
  },
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
    "stdout_is_json": true,
    "stdout_absent": [
      "\"runtime\"",
      "pyinstxtractor"
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64",
    "binary_sha256": "aecc729c1f63c2f66fc8a2c3b9563e4db09aff86f4ba803d14bb5cc1f060ce92",
    "binary_size": 5128,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64"
  },
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
    "stdout_is_json": true,
    "stdout_matches": [
      "\"runtime\": \\[",
      "\"id\": \"pyinstaller\"",
      "\"version\": \"3\\.12\"",
      "PyInstaller bundle \\(Python 3\\.12\\): extract with pyinstxtractor-ng"
    ],
    "stderr_absent": [
      "note:"
    ]
  },
  "notes": "The fixture is arraycoverwidth_x86_64 with the 88-byte PyInstaller CArchive cookie appended (generator: kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64.py). Dataset witnesses: Flare-On 12 chat_client (PyInstaller 3.12 ELF) and DrillBabyDrill.exe (PyInstaller 3.13 PE); both report the bundle once this passes."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- No subcommand inspects the wrappers; the only packer check is the UPX `no functions discovered`
  diagnostic, which fires after a failed run and names nothing else.

## Refutation

_upheld: the summary document has no field for it, and no option in `kuna catalog --json` adds one._

## Reference

- `pyinstxtractor-ng`, `ilspycmd`, `autoit-ripper`, `nuitka-extractor`: the tools the hints name.

## Instances

- Flare-On 12 `6/chat_client` (PyInstaller, Python 3.12, ELF)
- Flare-On 12 `1/DrillBabyDrill.exe` (PyInstaller, Python 3.13, PE)
- Flare-On 12 `4/UnholyDragon-150.exe` (twinBASIC; its DOS magic is damaged, so it does not load at all)

## Decision log

- filed by hand with a vendored in-repo target so the probe runs without the dataset.
