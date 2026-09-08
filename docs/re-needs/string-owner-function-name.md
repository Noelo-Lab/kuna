---
need_id: string-owner-function-name
title: String-owner function name cannot be used by decompile
track: tooling
status: open
severity: major
probe_id: p-5fd8cbc040d6
acceptance_id: a-37f20d36850b
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [69761b7a39e9c4d85c2f9fc1]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile.rs, decompiler/crates/kuna-cli/src/strings.rs]
scope: small
regression_of: analysis-generated-function-name
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Navigate to the function name returned by strings.

> **String-owner function name cannot be used by decompile** (major, `69761b7a39e9c4d85c2f9fc1`)
> strings reports owner sub_100a3be for 'No error information'. Named decompilation exits 1 with 'no function'; decompilation at 0x100a3be with --addr succeeds.

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
    "sub_100a3be"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "error: no function"
    ]
  },
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
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
    "sub_100a3be"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "No error information"
    ],
    "stderr_absent": [
      "error: no function"
    ]
  },
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- String ownership and named decompilation use different function inventories.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69761b7a39e9c4d85c2f9fc1` (round 8, tester t-r8-69761b7a)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 TRIAGE (captain): RETRACKED quality -> tooling. Two CLI surfaces disagree about one name -- strings reports owner sub_100a3be, and decompile <bin> sub_100a3be answers 'no function' while the same address with --addr decompiles fine. The emitted C is not in question, so no option is involved; the resolution path is kuna-cli/src/decompile.rs (is_unknown_function / the resolve_entry miss at :455-479) against the synthesized name strings.rs prints. Same family as the entry-name-rank trap: a synthesized sub_<hex> is not resolvable by the name lookup that prints it.
- round 8 CAPTAIN B_VERIFY: acceptance probe TARGET-BOUND to bin/graphy-release.zip.__x/graphy (sha 1fb1f75b6a3939..., 40472 bytes, binary_source dataset, challenge 69761b7a39e9c4d85c2f9fc1) in BOTH the Reproduction and the Acceptance block, and re-measured at sha bfacf0c6 as runnable-and-still-FAILING (kuna strings reports text 'No error information' at 0x1001270 owned by sub_100a3be, and kuna decompile <bin> sub_100a3be exits 1 with error: no function). Before this the probe read unrunnable ({{BIN}} used but the context supplies no bin), which means B_DONE could never have closed the need no matter what the builder shipped. Do NOT re-cut the acceptance onto an easier fixture; when you promote it into tests/cli/ it must be retargeted onto a vendored fixture that fails the same way pre-fix, because CI has no dataset.
- round 8 BUILDER (b-r8-string-owner-fun): hypothesis PARTLY upheld -- the two surfaces really do use different inventories (`kuna strings` attributes a literal's owner from the reference walk's own flow attribution, which reached 0x100a3be, a CALL target discovery had folded into sub_100a2b6's 0x100a2b6-0x100a400 extent, so `kuna functions` never lists it) -- but the fix is not in kuna-cli and not in the inventories. `sub_<hex>` is minted by `Architecture::name_function` and carries no information beyond the address, so `ConsoleProgram::resolve_entry`/`find_entry_by_name` now retry a by-name MISS as the address the name spells, accepting a candidate only when `name_function` renders it back as the requested name and only when that address holds mapped bytes. The ACCEPTANCE IS UNCHANGED and still measures graphy (PASS at this branch); the promoted `tests/cli/string-owner-function-name.json` is retargeted onto the vendored tailcallframe_x86_64, where `decompile <bin> sub_11b0` prints `sub_1170(a0)` and `decompile <bin> sub_1170` answered `no function` pre-fix -- the same defect, on a binary CI has.
