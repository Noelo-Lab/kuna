---
need_id: arm-pic-literal-pool
title: ARM PIC literal-pool string has no owning function
track: tooling
status: open
severity: major
probe_id: p-43fc3a2b7429
acceptance_id: a-3158325b7849
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [68d40081224c0ec5dcedc2d2]
rounds: [5]
first_seen_round: 5
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/kuna_poolref.rs]
scope: small
regression_of: arm-literal-pool-string
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find the function printing the success string at 0x4ab.

> **ARM PIC literal-pool string has no owning function** (major, `68d40081224c0ec5dcedc2d2`)
> Reports xrefs_count 0 and functions []. main computes 0x4ab from the displacement at 0x6a0 plus PC 0x66c. Decompiling with readonly on resolves the success literal, but that option does not repair strings ownership.

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
    "--json",
    "--filter",
    "Benar!"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 1
      },
      {
        "path": "strings[0].functions",
        "op": "len_eq",
        "value": 0
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
    "strings",
    "{{BIN}}",
    "--json",
    "--filter",
    "Benar!"
  ],
  "expect": {
    "json": [
      {
        "path": "strings[0].functions",
        "op": "len_gt",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/trap",
    "binary_sha256": "afc01737ed5b76cf5fc243709bfc3cd1f12ca8467b6dd269d274a171f58a2fad",
    "binary_size": 4902,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Reference analysis may not follow signed pool displacements through ADD PC.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/trap` — Server exited with status 1 before registering; no reference decompilation obtained.

## Instances

- `68d40081224c0ec5dcedc2d2` (round 5, tester t-r5-68d40081)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the mechanism is now pinned to a named, already-shipped seam. Symptom reproduced exactly: kuna strings target/trap --json --filter 'Benar!' -> xrefs_count 0, functions []; kuna xrefs target/trap --to 0x4ab --json -> count 0. THE IDIOM, AND IT IS NOT ONE SITE. main uses ldr rX,[pool] ; add rX,pc,rX THREE times -- 0x5d4+0x5d8, 0x660+0x664, 0x670+0x674 -- so every string reference in the function is lost, not just the success literal. Arithmetic checks out: the word at 0x6a0 is 3f fe ff ff = 0xfffffe3f = -0x1c1; PC at the add (0x664) is 0x66c; 0x66c - 0x1c1 = 0x4ab. The tester's reading of the binary was right. WHY NO EXISTING PASS COVERS IT -- this is a THIRD form, and both neighbours exclude it by their own stated rule. kuna-analysis/src/listing/kuna_poolref.rs does one dereference and follows the pool word as an ABSOLUTE pointer: it admits a value only if it 'passes the same ScalarOperandAnalyzer.checkOperands filter the constant scan uses and lands in a mapped section'. 0xfffffe3f is a signed displacement and lands nowhere, so poolref correctly declines. Its module header even points at the sibling case -- 'This is kuna_picbase's defect one indirection over: there the address is a register plus a displacement, here it is a word in the image' -- and this binary is BOTH at once: the displacement is a word in the image AND has to be composed with the PC of the add. kuna_picbase.rs is scoped to i386 GOT-base recovery (call/pop idiom) by its own header and does not see ARM. So the gap is real, unclaimed, and exactly where the tester said it was. SHAPE OF THE FIX. Compose the two: when a poolref dereference yields a value that is NOT a mapped address, and the loaded register is consumed by an add-with-PC in the same function, resolve target = pc_of_add + sign_extend(word) and file the Data edge on the INSTRUCTION, not on the pool word -- poolref's existing attribution rule, which is what gives the reference an owning function and is exactly what this acceptance asserts (strings[0].functions len_gt 0). Stays inside poolref's soundness envelope: the word is still read-only image content, the add's PC is still a decode-time constant. THE OPTION-DID-NOT-HELP CLAIM IN THE FILING IS SOUND FOR ONCE. The tester noted 'readonly on resolves the success literal but does not repair strings ownership' -- that is consistent: readonly affects constant folding in the decompiler, not the analysis-tier xref walk, and the acceptance asserts the xref walk. NOT the same defect as arm-literal-pool-disassembly, contrary to the last tick's guess. That one is kuna-cli/src/disassemble.rs (byte-recovery alignment in the CLI listing); this one is kuna-analysis/src/listing (the xref walk). Different crates, different tiers, and neither fix moves the other's probe -- strings/xrefs never call kuna-cli's litpool. Refute-together was the right instinct and the wrong conclusion; do NOT merge these needs, and do not let one builder claim both as one job.
round 5 TRIAGE (captain): TRACK quality -> tooling, TOUCHES kuna-decomp -> kuna-analysis/src/listing/kuna_poolref.rs. The acceptance asserts the analysis-tier xref walk (strings[0].functions len_gt 0), which never enters kuna-decomp. Same counter-lease correction as arm-literal-pool-disassembly, and the two remain SEPARATE needs in separate crates -- do not let one builder claim both. SCOPE stays small: the fix composes poolref's existing attribution rule with an add-with-PC resolution.
