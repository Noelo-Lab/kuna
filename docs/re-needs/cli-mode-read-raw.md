---
need_id: cli-mode-read-raw
title: No CLI mode to read raw bytes from data addresses
track: tooling
status: closed
severity: major
probe_id: p-74901277a96c
acceptance_id: a-ecb7f35d39aa
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [653d88600f4238b24302b0ec]
rounds: [2]
first_seen_round: 2
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: 379
closed_in_round: 2
closing_pr: "379"
reject_reason: null
---

## Symptom

Inspect bytes at referenced data VMAs such as 0x100003f30 without leaving kuna.

> **No CLI mode to read raw bytes from data addresses** (major, `653d88600f4238b24302b0ec`)
> The new disassemble command returned raw instruction bytes, but there is no analogous data/hexdump mode; strings discarded non-printable constants.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "disassemble",
    "{{BIN}}",
    "sub_100003790",
    "--json",
    "--count",
    "40"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "0x100003f30"
    ],
    "stdout_absent": [
      "000102030405060708090a0b0c0d0e0f"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme0x04",
    "binary_sha256": "9f17db47a3486acfa57590c2a0e551f25b1e25e8dfdf4a584f1f5c673b4fe26e",
    "binary_size": 50176,
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
    "disassemble",
    "{{BIN}}",
    "0x100003f30-0x100003f40",
    "--addr",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "000102030405060708090a0b0c0d0e0f"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme0x04",
    "binary_sha256": "9f17db47a3486acfa57590c2a0e551f25b1e25e8dfdf4a584f1f5c673b4fe26e",
    "binary_size": 50176,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Raw-byte exposure was implemented only for decoded instructions, not as a general memory-range view.

## Refutation

_not yet refuted_

## Reference

- `xxd -g1 -s 0x3f30 -l 96 ./target/crackme0x04` — Displayed the 00..0f, repeated 0x10, and repeated 0x20 vectors needed to simplify the initializer.

## Instances

- `653d88600f4238b24302b0ec` (round 2, tester t-r2-653d8860)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): kept SEPARATE from [disassembling-non-executable-rdata] (69a3822f, bad-ux: `disassemble` silently presents .rdata as instructions) although they are one user-facing gap -- 'I asked kuna about data and it either had nothing or lied'. cluster.py cannot merge across `kind`, and the remedies differ (a byte-read surface vs a guard on an existing one). Together with [strings-inventory-omits-statically] these are the three-instance TESTER demand that the captain-seeded `no-cli-data-code-override` was missing.
- round 2 T_TRIAGE (captain): track tooling / touches kuna-cli / scope small CONFIRMED as filed, and kind is `absence` (there is no data-read mode), which is why T_REFUTE correctly skipped it. Companion of disassembling-non-executable-rdata: a hexdump/typed-data view is the same missing surface seen once as 'I cannot read bytes' and once as 'disassemble happily decoded .rdata as instructions'. Kept separate -- each is independently probeable and closing one does not close the other -- but whichever is dispatched first should read the other, because one subcommand plausibly answers both and the second would then close as already-supported.
- closed: acceptance a-ecb7f35d39aa now PASSES at 3ae69cbcec70
