---
need_id: disassembling-non-executable-rdata
title: Disassembling a non-executable .rdata range silently presents data as code
track: tooling
status: open
severity: minor
probe_id: p-8c0894d9c37e
acceptance_id: a-c42440103c7d
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [69a3822f7b3cc38c80464da4]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

A raw byte or typed-data view of the encoded globals at 0x140032950 through 0x1400329ad

> **Disassembling a non-executable .rdata range silently presents data as code** (minor, `69a3822f7b3cc38c80464da4`)
> kuna exited successfully and decoded .rdata bytes as instructions such as SBB, POP, PUSH, JNC and JO, with no warning that the address range is non-executable. The JSON bytes were usable, but the instruction presentation is actively misleading.

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
    "0x140032950-0x1400329ad",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "instructions[0].mnemonic",
        "op": "eq",
        "value": "SBB"
      },
      {
        "path": "instructions[1].mnemonic",
        "op": "eq",
        "value": "POP"
      }
    ]
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
    "0x140032950-0x1400329ad",
    "--json"
  ],
  "expect": {
    "stderr_matches": [
      "non-executable|data section|not code"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The range form of disassemble decodes any mapped bytes without consulting executable section permissions.

## Refutation

_not yet refuted_

## Reference

- `objdump -s -j .rdata --start-address=0x140032940 --stop-address=0x1400329c0 ./target/nikos_crack_me.exe` — It displayed the bytes as data without inventing control-flow instructions.

## Instances

- `69a3822f7b3cc38c80464da4` (round 2, tester t-r2-69a3822f)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): second instance of the 'kuna will not tell me about data' family; see [cli-mode-read-raw] for why the two were not merged into one record.
- round 2 T_TRIAGE (captain): track tooling / touches kuna-cli / scope small / severity minor CONFIRMED. The defect is silence, not the decode: kuna exited 0 and presented .rdata bytes as SBB/POP/PUSH/JNC with nothing saying the range is non-executable. A warning (or a refusal behind a flag) is a bounded CLI change. See cli-mode-read-raw -- a data view would answer both.
