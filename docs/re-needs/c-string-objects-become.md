---
need_id: c-string-objects-become
title: C++ string objects become five-level pointer types in the checker
track: quality
status: closed
severity: major
probe_id: p-a597c89a3648
acceptance_id: a-c52c9a0120b6
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [6547b4d50f4238b24302b588]
rounds: [2]
first_seen_round: 2
attempts: 1
covered_by_option: ptrdepthcap
touches: [decompiler/crates/kuna-decomp]
scope: large
regression_of: null
pr: "392"
closed_in_round: 2
closing_pr: "392"
reject_reason: null
---

## Symptom

Readable recovery of the std::string/ostringstream values used to construct and compare the key.

> **C++ string objects become five-level pointer types in the checker** (major, `6547b4d50f4238b24302b588`)
> The checker is usable only after manually tracking fields through declarations such as unsigned long long ***** and char *****. These false types dominate the 9.9 KB output and make it needlessly difficult to distinguish the input string from the expected string.

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
    "0x1400011c0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "unsigned long long \\*\\*\\*\\*\\*",
      "char \\*\\*\\*\\*\\*"
    ]
  },
  "target": {
    "binary_rel": "bin/trappy attack.exe",
    "binary_sha256": "4872962a99da4def9048556345bdd1699a41b77dc0a148c8c28412b8adb3a06e",
    "binary_size": 26112,
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
    "0x1400011c0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "unsigned long long \\*\\*\\*\\*\\*",
      "char \\*\\*\\*\\*\\*",
      "\\*\\*\\*"
    ],
    "stdout_matches": [
      "\"\\+184V38cC\\.TRAPPY-ATTACK0\"",
      "\"Correct code, ur guess was: 33791\"",
      "\"M28j2\"",
      "\"24452\""
    ]
  },
  "target": {
    "binary_rel": "bin/trappy attack.exe",
    "binary_sha256": "4872962a99da4def9048556345bdd1699a41b77dc0a148c8c28412b8adb3a06e",
    "binary_size": 26112,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- MSVC STL object layouts and constructor prototypes are not propagated into stack locals.

## Refutation

**REFUTED (builder `b-r2-c-string-objects`, round 2).** Nothing about this defect is C++, MSVC,
STL, layout or constructor-prototype specific; it reproduces on a 0x131-byte hand-built x86-64 ELF
bytechunk with three unnamed extern stubs and no C++ at all (`tests/stages/kuna-ptrdepthcap.xml`).
The cause is a type-lattice escalation: a small-string-optimized object puts `PTRSUB(spacebase,-0xN)`
(typed pointer-to-the-mapped-local) and a LOAD from that same address (typed as the local) on the two
inputs of one MULTIEQUAL, i.e. `T = ptr(T)`, so `ActionInferTypes` adopts a type one pointer level
deeper per pass until its seven-pass settle ceiling. Making that ceiling settable and sweeping it
1..12 on the witness gives max pointer depth `2,2,3,4,5,5,5,5` -- one level per pass. Closed by
`ptrdepthcap` (DIV-108), which applies upstream's own `TypeFactory::getTypePointerNoDepth` rule at
the propagation funnel where it was missing.

## Reference

_none recorded_

## Instances

- `6547b4d50f4238b24302b588` (round 2, tester t-r2-6547b4d5)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 B_DONE (builder `b-r2-c-string-objects`): **the acceptance probe was STRENGTHENED, not bent**, before it was relied on. As filed, `a-4ac515df8701` asserted only `stdout_absent` of the two `*****` spellings with NO `exit_code` clause, so a crash, a timeout or a decompile that printed nothing would have passed it, and so would one that replaced the pointer chain with `undefined8` everywhere. Added (never relaxed): `exit_code: {eq: 0}`; a third `stdout_absent` pattern `\\*\\*\\*` so NO three-deep pointer survives anywhere, not just the two named spellings; and `stdout_matches` on all four string literals the tester had to hand-track (`"+184V38cC.TRAPPY-ATTACK0"`, `"Correct code, ur guess was: 33791"`, `"M28j2"`, `"24452"`), which is the positive evidence that the recovered type is actually better rather than merely shallower.
- round 2 B_DONE (builder): `verify --promote` refuses this acceptance because its target is `binary_source: dataset` and CI has no dataset (the same refusal PR #377 got). No in-repo fixture was fabricated with a different sha to get around it; the in-repo regression cover is the two-pass `tests/stages/kuna-ptrdepthcap.xml`, which reproduces the defect on a hand-built bytechunk.
- round 2 B_DONE (builder): the captain's `scope: large` triage followed from the refuted hypothesis. The fix is one module + one gated call site + one settable row, so the proposal fork was NOT taken.
- round 2 T_TRIAGE (captain): scope small -> LARGE. Recovering std::string/ostringstream through five-level pointer types is C++ type recovery, a known multi-part area of this codebase (TypePointerRel / FuncProto-this / struct-descent are ported but unfed), not a one-session patch.
- closed: acceptance a-c52c9a0120b6 now PASSES at c2e442188496 (the record's `## Acceptance` block is truth and hashes to a-c52c9a0120b6 after the builder strengthened it; the stale frontmatter label a-4ac515df8701 was corrected to match)
