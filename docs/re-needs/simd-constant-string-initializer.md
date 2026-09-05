---
need_id: simd-constant-string-initializer
title: SIMD constant-string initializer explodes into hundreds of scalar pseudo-operations
track: quality
status: open
severity: major
probe_id: p-7127e6d559a3
acceptance_id: a-9667a93853f5
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [653d88600f4238b24302b0ec]
rounds: [2]
first_seen_round: 2
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: large
regression_of: null
pr: "406"
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Readable pseudocode for the initializer so I could recover the generated prompt, result messages, and serial without manually simulating SSE byte lanes.

> **SIMD constant-string initializer explodes into hundreds of scalar pseudo-operations** (major, `653d88600f4238b24302b0ec`)
> The 752-byte initializer expanded into a very large body with pshufb, dozens of SUB161 lane temporaries, and repeated byte stores, obscuring simple generation of consecutive character ranges.

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
    "sub_100003790"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "pshufb\\(",
      "SUB161\\("
    ],
    "stdout_bytes": {
      "gt": 12000
    }
  },
  "target": {
    "binary_rel": "bin/crackme0x04",
    "binary_sha256": "9f17db47a3486acfa57590c2a0e551f25b1e25e8dfdf4a584f1f5c673b4fe26e",
    "binary_size": 50176,
    "binary_source": "dataset"
  }
}
```

## Acceptance

Both positive clauses are MEASURED on a freshly built main (2e56daca), not guessed: the emission
opens with `void sub_100003790(void)` and the destination buffer `0x100008050` appears three times
in the store loop. They exist so this acceptance cannot be satisfied by emitting NOTHING -- with
only `exit_code == 0`, absent-`SUB161(` and an upper byte bound, a build that emitted a stub body
or a short diagnostic for this function would have "passed" and closed a live need.

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_100003790"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "void sub_100003790\\(void\\)",
      "100008050"
    ],
    "stdout_absent": [
      "SUB161\\("
    ],
    "stdout_bytes": {
      "lt": 12000,
      "gt": 600
    }
  },
  "target": {
    "binary_rel": "bin/crackme0x04",
    "binary_sha256": "9f17db47a3486acfa57590c2a0e551f25b1e25e8dfdf4a584f1f5c673b4fe26e",
    "binary_size": 50176,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Vector lane extraction and constant propagation do not recognize the byte-sequence construction idiom.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `653d88600f4238b24302b0ec` (round 2, tester t-r2-653d8860)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): scope small -> LARGE. Folding pshufb/SUB161 lane temporaries back into a recognisable constant-string initializer is a new SIMD constant-folding capability, not a patch. Companion of strings-inventory-omits-statically: same challenge (653d8860), same 752-byte initializer, one seen from the decompiler surface and one from the strings surface. Kept separate because they are separately probeable and a fix to either does not imply the other, but a builder on one should read the other.

- round 2 B_DRAIN (captain): acceptance a-f6dbde227633 -> a-9667a93853f5. The old form was closable by emitting NOTHING (exit 0 + absent `SUB161(` + a 12,000-byte CEILING), so a build that emitted a stub body for sub_100003790 would have closed this need. Added two measured positive clauses (`void sub_100003790(void)` present, buffer `100008050` present) and a 600-byte floor. Re-run at 2e56daca: still FAILS, on the two clauses it always failed on (32x `SUB161(`, 15,532 bytes). The bar was raised, never relaxed.

- round 2 B_PLAN (captain): **SELECTED FOR DISPATCH over the two needs `select` ranked ahead of
  it.** All three open major needs score 13.862944 exactly, so the ranked order is Python's
  stable sort over the load order (alphabetical), not evidence — the pick among them is a
  captain call. This one wins on three measured grounds: (a) its acceptance is the only one of
  the three whose clauses were measured on a real build and cannot be satisfied cosmetically —
  replayed at 8799f22e it still fails on exactly `stdout_absent[0] SUB161(` (present) and
  `stdout_bytes lt 12000` (15,532), while both positive anchors match; (b) the mechanism is
  nameable and option-shaped (fold a constant SIMD lane construction back into an initializer)
  rather than open-ended; (c) it is the upstream half of [[strings-inventory-omits-statically]]
  — same binary, same 752-byte initializer — so a fold here makes that need cheap, while the
  reverse is not true. Dispatch order between the pair is therefore simd first, strings second.

- round 2 B_DRAIN (captain): **ATTEMPT 1 ENDED ON THE $30 BUDGET CAP, AND THE WORK IS COMPLETE AND
  PUSHED — do not re-file, do not redesign.** Builder `b-r2-simd-constant-st` ran 63 min / 202
  turns / $30.20 and died at the merge step: `.kuna-repipe/logs/b-r2-simd-constant-st.result.json`
  says `subtype: error_max_budget_usd`, `terminal_reason: budget_exhausted`, so `status: failed`
  here means "ran out of money", not "failed to build the thing" ([[kuna-repipe-budget-cap-salvage]]).
  What exists: **PR #406**, branch `feat/re-simd-constant-string-initializer` @ `f22d7325`, one
  commit, 31 files, local tip == pushed head, worktree
  `.kuna-repipe/worktrees/b-r2-simd-constant-st` CLEAN, and the commit's parent is `78025620` ==
  `origin/main` exactly, so **no rebase is owed and no count/baseline re-derivation is owed**.
- round 2 B_DRAIN (captain): **hypothesis inconclusive -> upheld, with a correction the next tick
  must carry.** The filed cause (constant-mask `pshufb` lanes never fold, so 32 `SUB161` temporaries
  survive) is real and is shipped as option `simdlane` (P3, DIV-115) — but the builder measured it
  as *less than half* the symptom: folding the lanes alone makes the function 278 bytes LONGER
  (15,532 -> 15,810). 45.8% of the bytes were one 72-store epilogue emitted three times, fixed
  separately as option `retsplitglobal` (P8, DIV-116), an output-side bound on
  `ActionReturnSplit::isSplittable`, which declines to split past 8 global stores. Acceptance
  a-9667a93853f5 passes on the branch (11,264 bytes, zero `SUB161(`, both positive anchors matched);
  with both options off the output is byte-identical to main.
- round 2 B_DRAIN (captain): **what is left is a merge button, priced at a merge and nothing more.**
  PR #406 is OPEN / MERGEABLE / `full-ci` labelled; `parity gates` SUCCESS, `cargo workspace suite`
  IN_PROGRESS as of 15:20Z (mergeStateStatus UNSTABLE is that one running check, not a conflict).
  A salvage dispatch must be MERGE-BUTTON-ONLY: `RESUME_BRANCH=feat/re-simd-constant-string-initializer`,
  no redesign, no force-push, wait for the workspace suite, then merge under the `merge` lease.
  Two mechanical traps first — `git worktree remove .kuna-repipe/worktrees/b-r2-simd-constant-st`
  before spawning (the wid is derived from the need id and `git worktree add` fails on an occupied
  path, falling back to a SILENT detached worktree, [[kuna-repipe-detached-branch-trap]]), and
  never `git branch -D` this branch.
