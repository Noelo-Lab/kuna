---
need_id: quadratic-transform-loses-four
title: Quadratic transform loses four accumulator initialization loads
track: quality
status: open
severity: blocker
probe_id: p-c7c053c8c9a0
acceptance_id: a-b49275132108
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [5ab77f5f33c5d40ad448c820]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover the initialized 128-bit accumulator at 0x401330.

> **Quadratic transform loses four accumulator initialization loads** (blocker, `5ab77f5f33c5d40ad448c820`)
> Emitted an uninitialized four-word array subsequently XORed through a pointer. Kuna disassembly shows four initial loads from 0x4060bc through 0x4060c8. Stack-preservation options, disabling loadguardrange, and explicit typed readonly data did not restore them.

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
    "0x401330",
    "--addr",
    "--assert",
    "data 0x4060bc unsigned int coefficients[33028]",
    "--assert",
    "readonly 0x4060bc+16"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "coefficients",
      "\\^"
    ],
    "stdout_absent": [
      "coefficients\\s*\\[\\s*[0-3]\\s*\\]|\\*\\s*coefficients|(?i:35f71e7f|3aa6a7f0|5c642a5|c30f7c)|(?:memcpy|memmove)\\("
    ]
  },
  "target": {
    "binary_rel": "bin/sddecoder_v1.1.zip.__x/decoder.exe",
    "binary_sha256": "8a0d47662bd432bf5e6c4e6d0665303b9a2e6a925befeaf9e6bebc38979c1fea",
    "binary_size": 172032,
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
    "decompile",
    "{{BIN}}",
    "0x401330",
    "--addr",
    "--assert",
    "data 0x4060bc unsigned int coefficients[33028]",
    "--assert",
    "readonly 0x4060bc+16"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "coefficients\\s*\\[\\s*[0-3]\\s*\\]|\\*\\s*coefficients|(?i:35f71e7f|3aa6a7f0|5c642a5|c30f7c)|(?:memcpy|memmove)\\("
    ]
  },
  "target": {
    "binary_rel": "bin/sddecoder_v1.1.zip.__x/decoder.exe",
    "binary_sha256": "8a0d47662bd432bf5e6c4e6d0665303b9a2e6a925befeaf9e6bebc38979c1fea",
    "binary_size": 172032,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Possibly stack-store liveness across indexed XOR operations; unconfirmed.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x401330 --binary target/sddecoder_v1.1.zip.__x/decoder.exe --raw` — Reported no running server. Subsequent load failed with server exited with status 1 before registering. Reference comparison is inconclusive.

## Instances

- `5ab77f5f33c5d40ad448c820` (round 8, tester t-r8-5ab77f5f)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED -- the symptom is real and reproduces at HEAD bfacf0c6, but the filed cause (store liveness across the indexed XOR) is the wrong lever, and building on it would emit WRONG C. THE FRAME MAP IS THE DEFECT, and kuna prints it itself: kuna decompile /tmp/decoder.exe 0x401330 --addr --json lists, in ONE function, v11 = unsigned int[4] at stack_offset -16 size 16 AND three separate undefined4 symbols local_c(-12), local_8(-8), local_4(-4) -- all three lie INSIDE v11's own extent [-16,0). The four initializing stores (MOV [ESP+0x10],EAX / [ESP+0x14],ECX / [ESP+0x20],EDX / [ESP+0x1c],EAX at 0x401344..0x40135d, reading 0x4060bc..0x4060c8) target exactly -16/-12/-8/-4, so three of them bind to the shadow scalars, which have line_numbers [] and addresses [] -- nothing reads them, so deadcode drops them -- while every READ reaches the array through the LEA pointer (LEA EAX,[ESP+0x20] at 0x401399 == -16). The array is right, the reads are right, and the stores land on symbols that alias it. This is P6 variable mapping (RangeHint join failing to fold the direct-store hints into the open range the indexed-pointer guard opened -- the same varmap.cc machinery loadguardrange documents), NOT liveness. WHY THE FILED HYPOTHESIS IS ACTIVELY DANGEROUS: the option that IS 'pin stack stores live across the deadcode race' already exists -- stackalias (P6/alias-facets, GH-8500, destructive opt-in) -- and I measured it: --option stackalias on changes NOTHING here. Worse, if such a brake DID fire it would print 'local_c = coefficients[1];' for bytes that are simultaneously v11[1] -- two names for one slot, which is wrong output, exactly the failure this refute exists to catch. RULED OUT BY MEASUREMENT, not argument: the tester's two asserts are irrelevant (a bare 'kuna decompile <bin> 0x401330 --addr' with no asserts emits the identical body); loadguardrange off, tiedstorekeep off and stackalias on all leave the four stores dropped; and the DIV-105 tiedstorekeep brake could never fire here anyway since its predicate requires the stored value to come from a CALL and these come from LOADs of a global. STACK-DELTA TRACKING IS INNOCENT AND I CHECKED IT: the interleaved PUSH EBX/ESI/EDI/EBP mean the four stores are written through three different ESP values, but kuna resolves them correctly -- MOV [ESP+0x10],EAX at 0x401369 (three pushes deep, EAX=0 from XOR EAX,EAX) comes out as v14 = 0 at offset -28, which is the arithmetically correct slot. Do not send a builder after extrapop/push deltas. Track quality and touches [kuna-decomp] are right; scope small is plausible if the fix is in the RangeHint join. Side observation for whoever takes it: v11 reports line_numbers [] / addresses [] even though it is printed, so the JSON variable-to-line mapping is thin for array locals -- a separate gap, not this one.
