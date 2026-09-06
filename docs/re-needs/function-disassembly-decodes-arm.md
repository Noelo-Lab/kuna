---
need_id: function-disassembly-decodes-arm
title: Function disassembly decodes ARM literal-pool data as an instruction
track: tooling
status: closed
severity: minor
probe_id: p-e6b8fbb0a325
acceptance_id: a-19c4b0c635a6
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5733c5d40ad448c380]
rounds: [3]
first_seen_round: 3
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/litpool.rs, decompiler/crates/kuna-cli/src/disassemble.rs, decompiler/crates/kuna-console/src/engine.rs]
scope: small
regression_of: null
pr: null
closed_in_round: 4
closing_pr: null
reject_reason: null
---

## Symptom

Distinguish main's trailing success constant from executable instructions.

> **Function disassembly decodes ARM literal-pool data as an instruction** (minor, `5ab77f5733c5d40ad448c380`)
> Lists the pool word 39050000 at 0x8458 as andeq after the return. The load at 0x8440 reads this word; asserting readonly 0x8458+4 makes decompilation return 0x539.

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
    "main",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "stdout_matches": [
      "\"bytes\": \"39050000\",\\s*\"mnemonic\": \"andeq\""
    ]
  },
  "target": {
    "binary_rel": "bin/1337_ARM.zip.__x/1337ARM.bin",
    "binary_sha256": "c8dcf51596afaee2c31b3f87fb9df9e84257c1b3881e90e24a1015fd88e2dd80",
    "binary_size": 571848,
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
    "main",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "stdout_absent": [
      "\"bytes\": \"39050000\",\\s*\"mnemonic\": \"andeq\""
    ]
  },
  "target": {
    "binary_rel": "bin/1337_ARM.zip.__x/1337ARM.bin",
    "binary_sha256": "c8dcf51596afaee2c31b3f87fb9df9e84257c1b3881e90e24a1015fd88e2dd80",
    "binary_size": 571848,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

The symptom stands exactly as filed. The *fix* the symptom line proposes does
not: asserting `readonly 0x8458+4` is a decompile-plane knob and never reaches
the listing, which does its own straight-line walk of the extent. The listing
needed evidence of its own -- and it already has it, in the `ldr r3,[0x8458]`
five rows above.

## Reference

_none recorded_

## Instances

- `5ab77f5733c5d40ad448c380` (round 3, tester t-r3-5ab77f57)

## Decision log

builder b-r4-function-disasse r4: CLOSED. `kuna disassemble` now folds a word its
own listed instructions read at a fixed address, and none of them branch to, into
a `.word 0x...` data row (`decompiler/crates/kuna-cli/src/litpool.rs`, fed by
`ConsoleProgram::add_fixed_refs_at`). Evidence stays inside the listed range, so
listing the word alone still decodes it raw. Acceptance PASSES; promoted to
`tests/cli/function-disassembly-decodes-arm.json`, re-pointed at the in-repo
`cortexm_poolentry_le32` (CI has no dataset).

- filed by cluster.py from 1 observation(s)
captain T_TRIAGE r3: track CORRECTED quality -> tooling, touches CORRECTED kuna-decomp -> the disassembly path. Inferred quality from kind=wrong-output, but the probe is `kuna disassemble main --json` and no C is produced; the gap is that the listing walks an ARM literal pool as code. Same data-vs-code question as arm-literal-pool-string on the same binary -- a builder taking either should read both, though the fixes are in different files.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
