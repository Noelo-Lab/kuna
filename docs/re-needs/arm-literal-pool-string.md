---
need_id: arm-literal-pool-string
title: ARM literal-pool string use has no owning function
track: tooling
status: open
severity: major
probe_id: p-d995eb7a24ba
acceptance_id: a-763430881ec8
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [5ab77f5733c5d40ad448c380]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/xrefs.rs, decompiler/crates/kuna-analysis/src/analyzers/strings]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find the function using the kernel-version error string.

> **ARM literal-pool string use has no owning function** (major, `5ab77f5733c5d40ad448c380`)
> Reports xrefs_count 0 and functions []. Kuna disassembly shows __libc_start_main loading r0 from 0x86e4 at 0x862c before calling __libc_fatal. Kuna read confirms that pool word points to the string at 0x661bc.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "strings",
    "{{BIN}}",
    "--filter",
    "FATAL: kernel too old",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "json": [
      {
        "path": "strings[0].functions",
        "op": "len_eq",
        "value": 0
      },
      {
        "path": "strings[0].xrefs_count",
        "op": "eq",
        "value": 0
      }
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
    "strings",
    "{{BIN}}",
    "--filter",
    "FATAL: kernel too old",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "json": [
      {
        "path": "strings[0].functions",
        "op": "len_gt",
        "value": 0
      }
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

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5733c5d40ad448c380` (round 3, tester t-r3-5ab77f57)

## Decision log
- filed by cluster.py from 1 observation(s)
captain T_DEDUP r3: cleared regression_of: no-strings-inventory (the tester's own field), which was applying REGRESSION_FLOOR and putting this 1-instance need at score 1110 -- ahead of a 5-tester need at 179. Two reasons. (a) Wrong target: no-strings-inventory is "kuna cannot list or search strings", which plainly still works; the need this resembles is strings-json-fails-report ("strings JSON fails to report the owning function for a directly referenced string"). (b) Not a regression: I re-ran both acceptance probes on cf5234ac -- strings-json-fails-report PASSES (kuna strings --json still attributes the owning function, functions[0] = sub_8048ace), and no-strings-inventory has no runnable acceptance at all (passed: null). The ARM literal-pool case is a NEW narrower gap in xref-to-function attribution when the reference goes through a literal pool, not a shipped capability that broke. Ranked on merit it scores 13.86.
captain T_TRIAGE r3: track CORRECTED quality -> tooling, touches CORRECTED kuna-decomp -> kuna-analysis. cluster.py inferred quality from kind=wrong-output, but the probe drives `kuna strings --json` and emits no C at all. Measured on cf5234ac: the string at 0x661bc reports xrefs_count 0 / functions [], so the gap is that the reference scan does not follow an ARM literal-pool load (LDR Rn,[PC,#imm]). No emitted C changes, so no option and no stages case; exact precedent is the closed xrefs-unify-pe-import (tooling, listing/xrefs.rs).
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
builder b-r3-arm-literal-pool r3: T_TRIAGE's reading is upheld exactly -- the walk files the `read` of the pool word at 0x86e4 and nothing joins it to 0x661bc. Closed by `poolref` (no option; track tooling, the change is confined to the on-demand xref index): a pointer-sized, pointer-aligned read of an allocated NON-writable location files a second edge from the same instruction to the address that word holds. Attributing it to the instruction rather than to the pool word is what gives it an owning function. Three refusals are pinned by the vendored fixture -- a narrow read, a writable .data slot, and a word holding 42. Measured over 15 images: 0 attributions lost, 0 edges added on every x86-64/PE image, and 2,239/763 new string-to-function attributions on u-boot / the witness, of which an independent capstone+symtab oracle corroborates 2,234/761 and every one of the 14 remaining outliers is a load the ORACLE missed. Residual, not claimed: a pool word reached through a register, and 195 u-boot strings a pool load reaches in code no descent reaches. Full record: docs/features/arm-literal-pool-string/record.json.
