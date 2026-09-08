---
need_id: batch-silently-omits-explicitly
title: Batch silently omits an explicitly defined function in a data section
track: tooling
status: open
severity: blocker
probe_id: p-f8d3f0389e36
acceptance_id: a-aa61cda6c47a
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5433c5d40ad448c1c1]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile_all.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Use --define-function to authorize analysis of the known entry despite section classification.

> **Batch silently omits an explicitly defined function in a data section** (blocker, `5ab77f5433c5d40ad448c1c1`)
> The declaration adds an 11-byte entry to kuna functions, but decompile-all with the same declaration still returns count 0 and no error.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--json",
    "--define-function",
    "0x4f7001-0x4f700c=entry"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
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
    "decompile-all",
    "{{BIN}}",
    "--json",
    "--define-function",
    "0x4f7001-0x4f700c=entry"
  ],
  "expect": {
    "json": [
      {
        "path": "count",
        "op": "gt",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Batch filtering may prioritize section flags over explicit function declarations.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/CryptoME.zip.__x/CryptoME.exe` — Server exited with status 1 before registering. Reference comparison is inconclusive; this does not establish an IDA analysis limitation.

## Instances

- `5ab77f5433c5d40ad448c1c1` (round 8, tester t-r8-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 REFUTER: hypothesis **upheld** (was inconclusive). captain refutation (in-tick, r8, HEAD bfacf0c6, arena binary .kuna-repipe/arena/8/5ab77f5433c5d40ad448c1c1/target/CryptoME.zip.__x/CryptoME.exe). HYPOTHESIS UPHELD, WITH THE FILE AND LINE, AND IT IS THE SAME ROOT CAUSE AS `batch-decompilation-succeeds-bodies`. (1) Reproduced: `kuna functions <bin> --define-function 0x4f7001-0x4f700c=entry --json` reports count 7 including `entry` at 0x4f7001 size 11, and `kuna decompile <bin> entry --define-function 0x4f7001-0x4f700c=entry` DECOMPILES it -- `void entry(void) { sub_4f7990(); }`. The same declaration through `kuna decompile-all <bin> --json --define-function ...` gives count 0, functions [], error null. Nothing in the decompiler refuses this data-section address; only the batch path drops it, and it drops ALL SEVEN entries, not just the declared one. (2) MECHANISM: decompiler/crates/kuna-cli/src/decompile_all.rs:1265-1269 -- "No filter at all => every executable function, exactly once" -- builds the target list as `prog.function_entries_executable()`. Section executability, not the declaration, decides the target list, exactly as the tester guessed. The `--define-function` declaration reaches the program (it is in `kuna functions` output) and is then filtered out downstream. (3) WHY THE FILTER EATS EVERYTHING HERE: this PE has SIX sections and not one of them is executable -- .text/.rdata/.data/.rsrc/.NEOpack/.NEOdata all carry characteristics 0xC0000040 (INITIALIZED_DATA|READ|WRITE), no IMAGE_SCN_MEM_EXECUTE (0x20000000), no IMAGE_SCN_CNT_CODE (0x20). NEOpack marks the whole image as data. So `image_has_executable_content` (same file, 1599/1628-1653, section flags first) also answers false and the run takes what the code calls the legitimate empty case. (4) FIX SHAPE: union the explicitly declared functions into the batch target list regardless of section flags (the single-function path already proves the decompiler is happy with them), and treat a declared function -- or the PE entry point -- landing in a non-executable section as evidence the image HAS executable content. (5) DISPATCH WARNING, READ BEFORE BUILDING: `batch-decompilation-succeeds-bodies` (same challenge, same command) asserts the OPPOSITE outcome -- its acceptance is `decompile-all <BIN> --json` with `exit_code ne 0`. If this need is fixed first, that command starts exiting 0 with count > 0 and that acceptance can never pass. The two are one gap wearing two acceptances; they must be dispatched to ONE builder or the second is unclosable.
- round 8 TRIAGE (captain): the sibling batch-decompilation-succeeds-bodies is FOLDED INTO THIS NEED (rejected as a duplicate) and its measured mechanism belongs here. Every section in this PE is flagged 0xC0000040 (INITIALIZED_DATA|READ|WRITE) -- no IMAGE_SCN_MEM_EXECUTE and no IMAGE_SCN_CNT_CODE, on .text as well as .NEOpack -- so image_has_executable_content (kuna-cli/src/decompile_all.rs:1628-1653, section flags first) answers false and :1599 takes the deliberate 'an image with no executable content has no functions' arm: exit 0, count 0, error null. That arm is correct code on a false premise; the missing thing is the executability test, which any packer defeats by marking its image data. kuna CAN decompile this binary -- decompile <bin> entry --define-function 0x4f7001-0x4f700c=entry emits void entry(void) { sub_4f7990(); } with no unpacking -- so THIS acceptance (count > 0) is the closable one. The sibling's acceptance demanded the same command exit NONZERO and could never co-exist with it; do not resurrect it. RESIDUAL, worth having but not a blocker: when the target list is genuinely empty AND the image has a plausible entry point, say so on stderr rather than reporting success.
- round 8 CAPTAIN B_VERIFY: acceptance probe TARGET-BOUND to bin/CryptoME.zip.__x/CryptoME.exe (sha bcfe3d24b0b745..., 139776 bytes, binary_source dataset, challenge 5ab77f5433c5d40ad448c1c1) in BOTH the Reproduction and the Acceptance block, and re-measured at sha bfacf0c6 as runnable-and-still-FAILING (decompile-all --json --define-function 0x4f7001-0x4f700c=entry exits 0 with count 0). Before this the probe read unrunnable ({{BIN}} used but the context supplies no bin), which means B_DONE could never have closed the need no matter what the builder shipped. Do NOT re-cut the acceptance onto an easier fixture; when you promote it into tests/cli/ it must be retargeted onto a vendored fixture that fails the same way pre-fix, because CI has no dataset.
