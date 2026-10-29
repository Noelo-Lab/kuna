---
need_id: batch-decompilation-succeeds-bodies
title: Batch decompilation succeeds with no bodies and no diagnostic
track: tooling
status: rejected
severity: major
probe_id: p-55b9adf72947
acceptance_id: a-0b045e12fce4
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [5ab77f5433c5d40ad448c1c1]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: duplicate of batch-silently-omits-explicitly: one root cause (the section-flag-only image_has_executable_content test in kuna-cli/src/decompile_all.rs:1628-1653), one command, one binary, one builder. Its acceptance demanded `decompile-all <BIN> --json` exit NONZERO, which the correct fix -- count > 0 on a binary kuna demonstrably decompiles -- can never satisfy, so held open it was an unclosable need that also invited a builder to ship an error path over working analysis. The mechanism and the residual diagnostic ask are recorded in the survivor's decision log.
---

## Symptom

An actionable diagnostic when no function bodies can be analyzed.

> **Batch decompilation succeeds with no bodies and no diagnostic** (major, `5ab77f5433c5d40ad448c1c1`)
> Exited 0 with count 0, error null, and functions []. Direct entry-point decompilation works. The image is NEOpack-packed; kuna unpack rejects it because only UPX is supported.

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
    "--json"
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
      },
      {
        "path": "error",
        "op": "eq",
        "value": null
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
    "decompile-all",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "ne": 0
    }
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Import slots may prevent the empty-discovery diagnostic despite yielding no decompilable bodies.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5433c5d40ad448c1c1` (round 8, tester t-r8-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 REFUTER: hypothesis **overturned** (was inconclusive). captain refutation (in-tick, r8, HEAD bfacf0c6, same arena binary as `batch-silently-omits-explicitly`: CryptoME.exe, 5ab77f54). THE SYMPTOM STANDS BUT THE FRAMING IS WRONG, AND THIS NEEDS ACCEPTANCE IS A TRAP. (1) The filed reading is "kuna cannot analyze a NEOpack-packed image and should say so". Measured: kuna CAN analyze it -- `kuna decompile <bin> entry --define-function 0x4f7001-0x4f700c=entry` emits `void entry(void) { sub_4f7990(); }` with no unpacking at all. The image is not opaque; only the batch path refuses it. (2) MECHANISM, and it is shared: every section in this PE is flagged 0xC0000040 (INITIALIZED_DATA|READ|WRITE) -- no IMAGE_SCN_MEM_EXECUTE, no IMAGE_SCN_CNT_CODE, on .text as well as .NEOpack. `image_has_executable_content` (decompiler/crates/kuna-cli/src/decompile_all.rs:1628-1653, section flags first) therefore answers false, and decompile_all.rs:1599 takes the deliberate "an image with no executable content has no functions" arm -- exit 0, count 0, error null. That arm is correct code reached on a false premise: the premise is a section-flag-only executability test that any packer defeats by marking its image data. So the missing thing is not a diagnostic, it is the executability test. (3) THE TRAP: this needs Acceptance demands `decompile-all <BIN> --json` exit NONZERO. The right fix makes that command exit ZERO with count > 0 (the entry is decompilable, proven in 1), so a builder who fixes the real gap FAILS this acceptance, and a builder who satisfies this acceptance ships an error path over a binary kuna can actually decompile. Do not dispatch this as written. (4) RECOMMENDATION FOR T_TRIAGE: fold this into `batch-silently-omits-explicitly` -- one root cause, one builder, one fix -- and keep a diagnostic only as the residual: when the target list is genuinely empty AND the image has a plausible entry point, say so on stderr and exit nonzero, which is still worth having for a truly opaque image. If it stays separate, its acceptance must be rewritten to "exit nonzero with a diagnostic OR count > 0" before any builder takes it.
- rejected: duplicate of batch-silently-omits-explicitly: one root cause (the section-flag-only image_has_executable_content test in kuna-cli/src/decompile_all.rs:1628-1653), one command, one binary, one builder. Its acceptance demanded `decompile-all <BIN> --json` exit NONZERO, which the correct fix -- count > 0 on a binary kuna demonstrably decompiles -- can never satisfy, so held open it was an unclosable need that also invited a builder to ship an error path over working analysis. The mechanism and the residual diagnostic ask are recorded in the survivor's decision log.
