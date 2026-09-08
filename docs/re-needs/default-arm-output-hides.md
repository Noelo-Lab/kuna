---
need_id: default-arm-output-hides
title: Default ARM output hides the success literal behind a global
track: tooling
status: open
severity: minor
probe_id: p-c5f9d392d81a
acceptance_id: a-3d4934fe287a
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5733c5d40ad448c380]
rounds: [9]
first_seen_round: 9
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

Read the success value without separately inspecting the literal pool.

> **Default ARM output hides the success literal behind a global** (minor, `5ab77f5733c5d40ad448c380`)
> main assigns dat_8458. Disassembly identifies .word 0x00000539. Both readonly on and an explicit readonly assertion expose return 0x539. This is a default/discoverability limitation, not a missing override.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "main"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "= dat_[0-9a-f]+;"
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
    "binary_rel": "bin/1337_ARM.zip.__x/1337ARM.bin",
    "binary_sha256": "c8dcf51596afaee2c31b3f87fb9df9e84257c1b3881e90e24a1015fd88e2dd80",
    "binary_size": 571848,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "main"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "= dat_[0-9a-f]+;"
    ],
    "stdout_matches": [
      "0x539|1337"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Readonly folding is disabled in the default mode.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/1337_ARM.zip.__x/1337ARM.bin` — Server exited with status 1 before registering; no reference pseudocode available.

## Instances

- `5ab77f5733c5d40ad448c380` (round 9, tester t-r9-5ab77f57)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, but the filed framing ('a default limitation') points at the WRONG fix and the right one is narrower. THE SYMPTOM AND THE OVERRIDE BOTH STAND: default emits 'v3 = dat_8458;'; '--option readonly on' emits 'return 0x539;'. THE RANGE IS ALREADY CORRECT -- nothing is missing on the painting side. get_readonly (kuna-analysis/src/loadimage_object.rs:1082) paints every non-SHF_WRITE section, and 0x8458 sits in .text (PROGBITS AX, 0x8150 + 0x5d3a8) of a statically linked non-PIE image, so it is genuinely immutable. 'readonly' gates the CONSUMER (folding a read-only LOAD), not the painter. THE REUSABLE FINDING: KUNA'S OWN DISASSEMBLER ALREADY IDENTIFIES THIS EXACT WORD AND THE DECOMPILER NEVER HEARS ABOUT IT. 'kuna disassemble <bin> main' prints 'note: one word in this range is read as a constant by the range's own instructions (a literal pool), so it is listed as data rather than decoded' and ends main's listing with '0x8440 ldr r3,[0x8458]' ... '0x8458 39050000 .word 0x00000539'. 0x8458 is INSIDE main's own extent (symtab main = 0x8290 + 460 = 0x845c). So the fact the fix needs is computed on the disassemble side of the CLI and does not reach folding -- this need does NOT require flipping the program-wide readonly default. WHY THAT DISTINCTION IS LOAD-BEARING: readonly on folds every load from every non-writable section image-wide. That is sound on this static non-PIE ELF, but this campaign's corpus is largely packers/protectors whose section flags lie about run-time writability, so 'flip readonly default-on' is a much larger claim than the evidence here supports; 'fold a literal-pool word the ARM pool analysis already found inside the function's own extent' is not. BLAST RADIUS MEASURED, ALL UPSIDE: readonly-on main is 30 lines vs 32, loses NOTHING (full body present, the stack temp collapses, 'return 0x539' renders inline, structure cleaner). No code deleted -- unlike the beria/entry-point absence traps. ACCEPTANCE: the absence clause '= dat_[0-9a-f]+;' is paired with a positive '0x539|1337' and default stdout has ZERO matches for it, so it cannot false-pass on a banner. It CANNOT, however, distinguish the narrow fix from the risky program-wide default flip -- both pass. Tell the builder the narrow route explicitly at dispatch.

### 2026-09-08T05:51Z — captain, round 9 B_PLAN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate, not
failing, and therefore unclosable by B_DONE however good the fix. Bound to the challenge's
own image:

    bin/1337_ARM.zip.__x/1337ARM.bin (571848 B, ELF 32-bit ARM, statically linked, non-PIE)

Re-run on freshly built main (3177f045) it now RUNS and FAILS, on the right clause:
`= dat_8458;` in main, exactly the filed symptom

Nothing about the probe's command or expectations was changed — only the target it
resolves against. Do not re-derive this.
