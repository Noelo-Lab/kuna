---
need_id: explicit-function-boundary-aborts
title: Explicit function boundary aborts on a branch to its exclusive end
track: quality
status: open
severity: blocker
probe_id: p-75d948cd4495
acceptance_id: a-2eea743fefa7
hypothesis_status: upheld
credibility: 1.0
instances: 2
challenges: [69d6affb110488a3205426e2, 6a0b84982b3df128c1df5c0d]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p2_lift]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Analyze the 205-byte entry prefix with the documented warning for cut control-flow edges.

> **Explicit function boundary aborts on a branch to its exclusive end** (blocker, `69d6affb110488a3205426e2`)
> Decompilation exits 1 with code:null and Could not find op at target address. The missing target is the declared exclusive end, 0x1400010cd. Kuna disassembly confirms a conditional branch there from 0x140001054.

> **Clipping cold error paths aborts instead of emitting a boundary warning** (minor, `6a0b84982b3df128c1df5c0d`)
> The supplied end cuts cold error paths after the normal return. Kuna exits 1 with Could not find op at target address instead of producing warning-bearing C.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x140001000",
    "--addr",
    "--define-function",
    "0x140001000-0x1400010cd=entry_prefix",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "json": [
      {
        "path": "functions[0].error",
        "op": "matches",
        "value": "Could not find op at target address"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "72336301c26c106024d5ade1470fd10580bf444b53107b14908dfb12e50f0fe6",
    "binary_size": 7131136,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x140001000",
    "--addr",
    "--define-function",
    "0x140001000-0x1400010cd=entry_prefix",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "\\S"
      },
      {
        "path": "functions[0].error",
        "op": "eq",
        "value": null
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "72336301c26c106024d5ade1470fd10580bf444b53107b14908dfb12e50f0fe6",
    "binary_size": 7131136,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- A clipped conditional target is still resolved as an internal operation.
- A later pass may reference an operation removed by the explicit boundary.

## Refutation

**UPHELD, and narrowed by measurement** (captain, round 3, cf5234ac).

Filed hypotheses were "a clipped conditional target is still resolved as an internal
operation" (upheld) and "a later pass may reference an operation removed by the explicit
boundary" (overturned: the op is never created, not created-then-removed -- the target is
outside the declared extent, so it is never decoded, and the branch-target lookup aborts the
whole function).

Measured on `crackme_shroud.exe`, entry 0x140001000:

| `--define-function` end | result |
|---|---|
| `0x1400010cd` (the filed one) | exit 1, "Could not find op at target address: (ram,0x0001400010cd)" |
| `0x1400010c0` (target 13 bytes past END) | same error, same address |
| `0x140001060` (target far past END) | same error, same address |
| `0x1400010ce` (that target now inside) | same error at the NEXT unresolved target, 0x1406a7f3d |

So this is not about a target landing exactly ON the exclusive end: ANY branch target with no
decoded op aborts, and the failures chain. It is also mode-independent -- `--mode reliable`
and `--mode fast` both fail identically -- which separates it cleanly from
[default-decompilation-fails-despite], where reliable and fast both SUCCEED.

WHAT A BUILDER MUST NOT DO. The obvious fix -- "if the branch target has no op, drop the edge
and warn" -- is right here and WRONG for the sibling need, whose missing target
(0x8048541) is a real NOP inside its declared extent that a false discovered entry removed. A
blanket clip at the lookup site would silently truncate a function that decompiles correctly
today under `--mode reliable`. The clip must be conditioned on the target being outside the
declared extent. Note that reliable mode already prints the wanted shape of answer on the
sibling case -- `// warn: Function flows out of bounds` -- so the warning vocabulary exists.

## Reference

_none recorded_

## Instances

- `69d6affb110488a3205426e2` (round 3, tester t-r3-69d6affb)
- `6a0b84982b3df128c1df5c0d` (round 3, tester t-r3-6a0b8498)

## Decision log

- filed by cluster.py from 2 observation(s)
captain T_DEDUP r3: MERGED with obs22 (clipping-cold-error-paths, frz_crackme_rage_v7.exe sha256). Same mechanism on two challenges and two testers: under --define-function START-END, control reaching or passing END makes kuna hard-error `Could not find op at target address` and exit 1 instead of clipping with a warning. Verified still failing on cf5234ac (0x1400010cd == the exclusive end; 0x1400060c5 is 5 bytes past it).
captain T_DEDUP r3: deliberately NOT merged with default-decompilation-fails-despite. That one prints the same message but for 0x08048541, an address INSIDE its declared extent -- same symptom, different trigger.
captain T_REFUTE r3: hypothesis upheld -- see ## Refutation (measured on cf5234ac with the release binary).
captain T_TRIAGE r3: track quality and scope small CONFIRMED; touches narrowed to p2_lift (flow/function-boundary). Upheld and narrowed at T_REFUTE: any branch target with no decoded op aborts, measured at four different --define-function ends and identical under --mode reliable and --mode fast. BUILDER: the obvious repair -- drop the unresolvable edge and warn -- is correct HERE and WRONG for default-decompilation-fails-despite, whose missing target is a real in-extent NOP deleted by a phantom entry; do not write one fix for both.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
builder b-r3-explicit-functio r3: FIXED, hypothesis 1 upheld and the captain's narrowing kept. Two changes in `p2_lift/flow.rs`, both inert without a declared extent (`FlowInfo::set_range` is the only narrowing of the flow range, and only a declared extent calls it). (1) The `missing` artificial halt `fillin_branch_stubs` already plants for an out-of-extent address is now registered in `visited` as the instruction there, so `collect_edges` resolves the cut edge to it and the body is clipped under the `Function flows out of bounds` warnings instead of the function dying. Restricted to the addresses `handle_out_of_bounds` recorded, exactly as T_TRIAGE demanded -- `default-decompilation-fails-despite`'s in-extent target keeps upstream's throw and its acceptance still FAILS. (2) Found by the collateral sweep, not by this need: `FlowInfo::fallthru` reported out of bounds when the next address EQUALS `eaddr`, which is the last IN-BODY byte, so a CORRECT declared extent never decoded the function's closing instruction -- `aif_gap_x86_64 sub_1129` came out as an empty `void sub_1129(void)` under a bogus warning instead of `int sub_1129(int a0) { return (a0 + 10) * 2; }`. Measured over 112 in-repo fixtures with every function declared at its derived extent (2,765 functions): bodies differing from the undeclared decompile 403 -> 138, hard errors 142 -> 0; with nothing declared, 112/112 byte-identical. DIV-121, no option.
