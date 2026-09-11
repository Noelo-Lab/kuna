---
need_id: arm-neon-zero-initialization
title: ARM NEON zero initialization becomes near-null memory reads
track: quality
status: open
severity: major
probe_id: p-7d8042bf74cc
acceptance_id: a-663f9d6c4a03
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [68d40081224c0ec5dcedc2d2]
rounds: [11]
first_seen_round: 11
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

Faithful initialization of the structure passed to __libc_init.

> **ARM NEON zero initialization becomes near-null memory reads** (major, `68d40081224c0ec5dcedc2d2`)
> vmov.i32 q8,#0 followed by vst1.64 stores a zero vector to the stack. Decompiled C instead reads words from (0 + 4), (0 + 8), and (0 + 0xc). Enabling dynrelocs does not change this.

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
    "_start_main"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\*\\([^\\n]*\\*\\)\\(0 \\+ (?:4|8|0xc)\\)"
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
    "_start_main"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\*\\([^\\n]*\\*\\)\\(0 \\+ (?:4|8|0xc)\\)"
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

- Vector-store expansion may treat a constant vector as a source pointer.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/trap` — Server exited with status 1 before registering. No reference pseudocode was obtained.

## Instances

- `68d40081224c0ec5dcedc2d2` (round 11, tester t-r11-68d40081)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED. Symptom stands and is stronger than filed (no options needed: bare `kuna decompile trap _start_main` emits all three near-null reads; the tester's "enabling dynrelocs does not change this" is a red herring, nothing needs enabling). The CAUSE is misnamed and the store named in the hypothesis is provably innocent.

WHAT THE HYPOTHESIS BLAMES: "vector-store expansion may treat a constant vector as a source pointer" -- i.e. the vst1.64. MEASURED: the vst1 semantics are correct. ARMneon.sinc:6026-6035 buildVst1DdList is `* mult_addr = Dreg; mult_addr = mult_addr + 8; build buildVst1DdList;` -- two clean 8-byte stores of d16 then d17, and the writeback form (6056) adds `RnAligned45 = RnAligned45 + (8 * vst1DdList)`. There is no pointer-typed read anywhere in the store path. A builder who goes looking in the vst1 lowering will find nothing wrong.

WHERE THE POINTER IS ACTUALLY BORN: the vmov, not the vst1. `vmov.i32 q8,#0` (f2c00050 at 0x4e0) matches ARMneon.sinc:549-553, the all-zero-immediate simdExpImm_16 case, whose body is `tmp:16 = 0; export *[const]:16 tmp;`. That is a DYNAMIC const-space export: the exported operand's address is a 16-byte temporary, so `Qd = simdExpImm_16` lowers to a LOAD whose pointer is that temporary, not to a COPY of a constant. The "0" being dereferenced in the emitted C IS that temporary. So the chain is vmov -> unfolded const-space LOAD -> the 16-byte value the (correct) vst1 stores.

THE SYMPTOM'S SHAPE PREDICTS THIS, WHICH IS THE STRONGEST EVIDENCE HERE. Output is `v1 = 0` at stack+0 but `*(unsigned int *)(0 + 4)`, `(0 + 8)`, `(0 + 0xc)` for the other three words. Note the address expressions are unfolded `0 + k` ADDs of two constants -- ordinary constant folding would have printed `4`. The piece at offset +0 needs no ADD and therefore collapsed to the constant 0 correctly; the three that needed `const0 + k` did not. A defect in the STORE would corrupt all four words uniformly. A defect in the SOURCE's const-space fold corrupts exactly the three that require an offset ADD -- which is what we see, 3 for 3, with the +0 case as the built-in control.

THE WRONG-FIX TRAP, AND IT IS A REAL ONE. The obvious fix is "fold a const-space LOAD at a constant address back into that constant" -- that is RuleLoadVarnode (p3_dataflow/ruleaction_4.rs:383 apply_op, via check_spacebase at :323, group "stackvars"). It cannot be applied naively at size 16: a varnode offset is a 64-bit uintb, so a 16-byte constant is not representable in general. It happens to work for this case only because the value is zero. A builder who "fixes" this by widening the fold will produce a silently TRUNCATED constant for any non-zero 16-byte SIMD immediate -- vmov.i32 q8,#1 would become 0x...0001 in the low 8 bytes and garbage or zero above. Every other simdExpImm_16 arm in ARMneon.sinc (556, 574, 596, 631, 661, 677 ...) builds a genuinely non-zero 16-byte val and exports it the same way, so those arms are exactly the ones such a fix would corrupt, and NONE of them appears in this binary -- the acceptance probe would go green while the fix is wrong. The sound shapes are (a) split the 16-byte const-space load into representable pieces before folding, or (b) handle it in the lifter so the 16-byte immediate never becomes a dynamic export at all. Whichever is chosen, the regression evidence must include a NON-ZERO 16-byte NEON immediate, because this need's own binary cannot detect the truncation.

METHOD (cheap, reusable): bare-default run first (killed the "dynrelocs" framing in one call); objdump the reported range (0x4d4-0x4fc showed the store is a plain 2x8-byte vst1 with writeback); then read the SLEIGH for both the store AND the source -- the source is where it was. Do not stop at the instruction the tester named.

- round 12 BUILDER (resumed): shipped as option `constspaceload`, default on (DIV-158). The refuter's diagnosis is confirmed verbatim by the raw p-code (`u0x00024200:16 = #0x0:16` then `u0x00024300:16 = *(const,u0x00024200:16)`), and the vst1 is innocent as it said.

  **The refuter's wrong-fix trap is real, and the corpus sweep caught a second one it did not predict.** The first cut of the rule applied the address-is-value identity at every width: `COPY p` when N == S, `SUBPIECE(p,0)` when N < S, `INT_ZEXT p` when N > S. It passed the acceptance probe, the datatests (0/675) and the stage test, and its non-zero regression evidence (`vmov.i32 q8,#1` -> four 1s) was green. The whole-corpus `decompile-all` ablation is what refuted it: the resize arms changed 5 binaries beyond the witness, all statically linked AVX-512 glibc, by re-rendering `k` mask registers at their true width. Most of that was a faithful improvement -- but in the `__strlen_evex`-shaped `sub_476bc0` of two of them it was WRONG. At `label_476da0`, which is reached both by fall-through and by a `goto`, the live mask ended up in `v1`/`v4` on one path and `v9` on the other while the scan loop read `v3` -- a stale mask from the *previous* compare, which is zero on that path. The option-off arm used a single variable `v2` on all three paths and was correct. So the resizing lost a reaching definition at a shared label.

  Shipped narrowed to the equal-width identity only (`out:N = LOAD(const, p:N)` -> `COPY p`). Principled as well as convenient: a dynamic `export *[const]:N tmp` gives the exported operand and `tmp` the same size, so N != S is a different shape and resizing it invents a truncation rather than applying an identity. Re-measured after the narrowing, all 5 of those binaries are byte-identical, and the shipped rule changes exactly 1 of 122 compared binaries in the 137-target sweep: the witness.

  Method note for whoever reads this next: the acceptance probe, the datatests and the stage test were ALL green on the wrong version. Only the whole-corpus before/after with per-hunk classification found it, and only because a `return`-count drop of 1 in two binaries was chased instead of waved through as a rendering difference.
