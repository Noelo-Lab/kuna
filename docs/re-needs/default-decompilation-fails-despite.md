---
need_id: default-decompilation-fails-despite
title: Default decompilation fails despite an explicit function extent
track: quality
status: open
severity: major
probe_id: p-dfb1e0ead0ec
acceptance_id: a-61b6770a99df
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [5ab77f5833c5d40ad448c399]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/analyzers/entry/patterns, decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover the complete keyfile reader.

> **Default decompilation fails despite an explicit function extent** (major, `5ab77f5833c5d40ad448c399`)
> Exited 1 with Could not find op at target address. The target is a NOP before the read loop. Explicit boundaries did not repair it; reliable mode worked.

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
    "0x80484f2",
    "--addr",
    "--define-function",
    "0x80484f2-0x80485ba=read_key"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "Could not find op at target address"
    ]
  },
  "target": {
    "binary_rel": "bin/collide.tgz.__x/collide.tar.__x/collide/collide",
    "binary_sha256": "2141200d97193c42c25144374eeeced095d570e6f5e88b30ff9e6d4fa4594c97",
    "binary_size": 9400,
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
    "0x80484f2",
    "--addr",
    "--define-function",
    "0x80484f2-0x80485ba=read_key"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "Could not find op at target address"
    ],
    "stdout_matches": [
      "read\\("
    ]
  },
  "target": {
    "binary_rel": "bin/collide.tgz.__x/collide.tar.__x/collide/collide",
    "binary_sha256": "2141200d97193c42c25144374eeeced095d570e6f5e88b30ff9e6d4fa4594c97",
    "binary_size": 9400,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- An aggressive-mode flow transformation may mishandle the branch targeting a NOP.

## Refutation

**OVERTURNED -- and the trigger is named** (captain, round 3, cf5234ac).

Filed hypothesis: "an aggressive-mode flow transformation may mishandle the branch targeting a
NOP". It is aggressive-mode-only, but it is not a flow transformation: it is the analysis-tier
function-discovery option **`funcstart_patterns`**, which aggressive turns on.

Bisected over aggressive's 31 overrides on `collide`, `read_key`
(`--define-function 0x80484f2-0x80485ba`):

```
--mode aggressive                                  -> exit 1, Could not find op at (ram,0x08048541)
--mode aggressive, all 31 overrides off            -> full C
--mode aggressive, only funcstart_patterns on      -> exit 1, same address
--mode reliable / --mode fast                      -> full C, "// warn: Function flows out of bounds"
```

The mechanism is visible in the inventory: under aggressive `kuna functions` reports
`sub_8048542`, a byte-pattern "function" planted INSIDE read_key's body, one byte past the
`NOP` at 0x8048541 that `JNZ 0x8048541` (at 0x804853d) targets. The enclosing body is cut at
that phantom entry, the NOP's op stops existing, and the branch-target lookup aborts the
function.

The existing containment guard does not reach it: `--option fdeinterior on` changes nothing
because this i386 image has NO `.eh_frame` section at all, so the FDE-body test has no
authority to apply. `overlapbranch on` does not help either. The fix belongs in discovery --
reject a pattern-discovered entry that falls inside a body the walk already decoded, the
eh_frame-less analogue of `fdeinterior` -- not at the branch-target lookup, and not in the
structuring passes the hypothesis pointed at. Same option is the prime suspect for
[false-function-entry-inside].

## Reference

_none recorded_

## Instances

- `5ab77f5833c5d40ad448c399` (round 3, tester t-r3-5ab77f58)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_DEDUP r3: kept separate from explicit-function-boundary-aborts despite the identical error text. Its failing address 0x08048541 lies INSIDE the declared extent 0x80484f2-0x80485ba, so it is not the flow-past-the-end case. Re-verified failing on cf5234ac.
captain T_REFUTE r3: hypothesis overturned -- see ## Refutation (measured on cf5234ac with the release binary).
captain T_TRIAGE r3: touches CORRECTED per the T_REFUTE measurement: the trigger is the discovery tier, not structuring. `--mode aggressive --option funcstart_patterns off` gives full C, and funcstart_patterns alone reproduces the abort by inventing sub_8048542 one byte inside read_key. Track stays quality because narrowing a default-ON discovery pattern moves emitted C and function inventories corpus-wide -- that is a DIV-shaped default change (cf. ppc64-localentry-splits-function), not CLI plumbing. See also false-function-entry-inside, the same suspected mechanism on a PE.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
