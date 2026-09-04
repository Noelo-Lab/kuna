---
need_id: direct-address-keyboard-handler
title: Direct-address keyboard handler decompilation annexes the unrelated renderer
track: quality
status: open
severity: major
probe_id: p-36f0974fc119
acceptance_id: a-a9a71e29cc50
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [6989ca5da15272fa37a80c43]
rounds: [2]
first_seen_round: 2
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

A bounded decompilation of the keyboard callback at 0x6500.

> **Direct-address keyboard handler decompilation annexes the unrelated renderer** (major, `6989ca5da15272fa37a80c43`)
> The output begins with keyboard handling, but continues through the renderer logic from 0x4610, including the UI title and drawing calls. The disassembly shows a tail jump to 0x4610 at 0x66e2; it should remain a call/tail-call boundary rather than duplicating thousands of lines into the callback.

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
    "0x6500",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "LUGOSI.*S II",
      "sub_5e30\\("
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
    "decompile",
    "{{BIN}}",
    "0x6500",
    "--addr"
  ],
  "expect": {
    "stdout_absent": [
      "LUGOSI.*S II",
      "sub_5e30\\("
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Because 0x6500 is absent from the discovered function map, direct-address recovery follows its tail jump into the already separate renderer instead of treating the jump as an interprocedural edge.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6989ca5da15272fa37a80c43` (round 2, tester t-r2-6989ca5d)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): SPLIT out of the 4-observation `wrong-output|decompile|stdout_absent` group. Its sibling is [keyboard-callback-uses-undefined]; both testers trace their symptom to the same root (0x6500 is absent from the discovered function map), but the remedies differ -- a tail-jump-as-interprocedural-edge boundary fix here, live-in register promotion there -- so they carry separate acceptance probes. This is also the first INDEPENDENT tester demand for the captain-seeded `no-cli-function-boundary-override`; note the tester asks kuna to get the boundary right by itself, not for a CLI override.
- round 2 T_TRIAGE (captain): track/touches/scope CONFIRMED (quality / kuna-decomp / small). Measured this tick so the builder does not have to: `funcboundflow` does NOT cover this. Decompiling 0x6500 of lugosiii with and without `--option funcboundflow on` gives byte-identical 1555-line output still containing the renderer's LUGOSI title twice, i.e. the tail jump at 0x66e2 still annexes 0x4610. (funcboundflow is default-ON per DIV-67, so `on` is a no-op; the point is that the shipped fix for function-merge does not reach a tail jump into a known entry.) Kept small: treating a jump to a discovered function entry as a tail call is a bounded, gateable decision.
