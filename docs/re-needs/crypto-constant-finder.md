---
need_id: crypto-constant-finder
title: No way to find well-known crypto constants and the functions that use them
track: tooling
status: open
severity: major
probe_id: p-0758638fe229
acceptance_id: a-d0505d884f2a
hypothesis_status: upheld
credibility: 0.9
instances: 1
challenges: []
rounds: []
first_seen_round: null
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/crypto.rs, decompiler/crates/kuna-cli/src/cryptosig.rs, decompiler/crates/kuna-cli/src/main.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Locating the cipher or hash a crackme uses (the AES S-box, SHA-256's round constants, a CRC-32
table, the TEA delta, a custom Base64 alphabet) and the function that uses it is the first triage
hop on most crypto challenges, and kuna has no command for it. An agent falls back to hexdumping
`.rdata` with `kuna read` and grepping for `63 7c 77 7b`, then runs `kuna xrefs --to` on each
hit by hand, which is what findcrypt does in one step.

> **No crypto-constant inventory** (major, Flare-On 12 #7 `hopeanddreams.exe`)
> The AES S-box at 0x14046b8a0 (used by sub_140050560), the inverse S-box at 0x14046a540 and
> SHA-256's K at 0x14046a660 / H0 at 0x14046a640 were found only by hexdump-and-grep.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "crypto",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 2
    },
    "stderr_matches": [
      "unknown subcommand \"crypto\""
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/cryptoconst_x86_64",
    "binary_sha256": "623bacb1357d32db674e99d403bcfd42cd8f6c804b3596222ed5e040b09b7520",
    "binary_size": 16120,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/cryptoconst_x86_64"
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
    "crypto",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "\"address_hex\": \"0x402180\",\\s*\"algorithm\": \"AES\",\\s*\"name\": \"S-box\"",
      "\"name\": \"S-box\"[\\s\\S]*?\"name\": \"sub_bytes\"",
      "\"address_hex\": \"0x402080\",\\s*\"algorithm\": \"SHA-256\",\\s*\"name\": \"round constants K\"[\\s\\S]*?\"name\": \"sha256_round\"",
      "\"algorithm\": \"CRC-32\",\\s*\"name\": \"polynomial 0xEDB88320 \\(reflected\\)\"[\\s\\S]*?\"name\": \"crc32_update\"",
      "\"algorithm\": \"TEA[^\"]*\"[\\s\\S]*?\"name\": \"tea_encrypt\"",
      "\"name\": \"alphabet \\(custom\\)\"[\\s\\S]*?\"name\": \"b64_char\""
    ],
    "stderr_absent": [
      "unknown subcommand"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/cryptoconst_x86_64",
    "binary_sha256": "623bacb1357d32db674e99d403bcfd42cd8f6c804b3596222ed5e040b09b7520",
    "binary_size": 16120,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/cryptoconst_x86_64"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.**

- `kind: absence`: there is no subcommand. The scan is a byte-pattern search over the mapped
  sections; the "who uses it" half is the reference index `kuna strings` already attributes with.

## Refutation

_not refuted (absence)_

## Reference

- findcrypt / IDA `findcrypt-yara`: signature scan of the loaded image, one row per constant with
  its address and label.

## Instances

- Flare-On 12 #7 `hopeanddreams.exe` (local validation only; not vendored).
