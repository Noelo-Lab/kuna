---
need_id: vm-temporary-declared-as
title: VM temporary is declared as an integer but dereferenced as a pointer
track: quality
status: closed
severity: major
probe_id: p-24314022cb06
acceptance_id: a-70bd6c755858
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [69761b7a39e9c4d85c2f9fc1]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: declhightype
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/563
closed_in_round: 12
closing_pr: "563"
reject_reason: null
---

## Symptom

Consistently typed VM pseudocode suitable for translating into an evaluator.

> **VM temporary is declared as an integer but dereferenced as a pointer** (major, `69761b7a39e9c4d85c2f9fc1`)
> Declares unsigned long v6, then emits v6 = &v10[(long)v6] and *v6 = *v6 << ... . The contradiction survives both argument-recovery options and an explicit VM prototype.

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
    "sub_1005350",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^  unsigned long (v[0-9]+);[\\s\\S]*?\\*\\1\\b"
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
    "sub_1005350",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "(?m)^  unsigned long (v[0-9]+);[\\s\\S]*?\\*\\1\\b"
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

- Distinct register values may share one printed identifier despite incompatible types.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x1005350 --binary target/graphy-release.zip.__x/graphy --raw` — No running server. Subsequent ida-decompile load exited 1 before registering, so no reference pseudocode was obtained.

## Instances

- `69761b7a39e9c4d85c2f9fc1` (round 12, tester t-r12-69761b7a)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `byte-comparisons-incorrectly-dereference` — near-sibling: inconsistent type on a merged register variable. Kept separate (Jaccard 0.048).
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). round 12 REFUTER (captain, tick 05:3xZ) -- VERDICT UPHELD, but the filed cause aims a builder at the WRONG SUBSYSTEM; read the redirect before dispatching.

WHAT IS MEASURED, on aa18e56d, one command (kuna decompile <graphy> sub_1005350 --option calleearity on --option varargstackargs on):
1. The symptom is real and far larger than filed. v6 is declared "unsigned long v6; // rax" once, and across the 423-line body it is assigned pointers (v6 = (unsigned long *)v10[v7 - 1]; v6 = *(unsigned long **)(a0 + 1 + v9); v6 = *(unsigned long **)(a2[3] + (long)v6 * 8)), dereferenced (v6 = (unsigned long)*v6), compared against a pointer (if (v6 != (unsigned long *)0xffffffff)), used as an integer index ((long)v6 * 8), stored as an integer (v10[v8] = (unsigned long)v6), and returned 20+ times from a function whose return type is "unsigned long *".
2. THE OUTPUT DOES NOT TYPE-CHECK. gcc -fsyntax-only -std=c99 on the emitted body (only addition: stddef.h for NULL) reports 65 -Wint-conversion diagnostics in this one function -- "returning long unsigned int from a function with return type long unsigned int * makes pointer from integer without a cast" and "assignment to long unsigned int from long unsigned int * makes integer from pointer without a cast". -Wint-conversion is an error by default in GCC 14+ and clang 16+, so this is invalid C on a current toolchain, not merely ugly C. Severity major is if anything low.
3. The hypothesis as written is literally TRUE. v6 is one identifier covering distinct rax values with incompatible types: at some program points it holds a node pointer that is returned and dereferenced, at others an opcode index multiplied by 8. Those are different values, one name, incompatible types. UPHELD.

THE REDIRECT, AND IT IS THE POINT OF THIS VERDICT. "Distinct register values share one identifier" reads as an instruction to stop merging them -- p6 variable merge surgery, high risk, and not necessary to fix the emitted C. There is a second, independently sufficient defect in p9 that the same output proves: THE CAST MACHINERY AND THE DECLARATION PRINTER DISAGREE ABOUT THIS ONE HighVariable. Every cast kuna itself inserted around v6 treats it as "unsigned long *" (it casts the 0xffffffff constant to a pointer to compare against v6, and casts the loaded values to pointer types on the way in); the declaration line calls it "unsigned long"; and the return statements carry no cast at all. One producer says pointer, the other says integer, in the same function, for the same high. Making the declaration agree with the casts the printer already emits makes the body compile without touching the merge at all. Structural context for whoever takes it: vn_high_type_def_facing (p6_variables/funcdata_facing.rs:139-152) resolves through high_get_type(vn) but FALLS BACK to the raw Varnode type when the high has none -- a divergence path of exactly this shape. I did not instrument to confirm the fallback fires here, so treat that pin as a lead, not a measurement.

ACCEPTANCE, CHECKED, AND IT IS SATISFIABLE BUT GAMEABLE. The regex is (?m)^  unsigned long (v[0-9]+);[\s\S]*?\*\1\b. I enumerated every integer declaration in the body (v11, v2, v3, v4, v5, v6, v7, v8, v9) and every *vN use: only v6 both matches the declaration form and is dereferenced. v1 IS dereferenced but is declared "unsigned long *v1;" so it cannot match. So the acceptance fires on v6 alone and a correct fix clears it -- it is not the loose kind. The gaming risk is the flip side: declaring "unsigned long *v6;" alone clears the regex while merely MOVING the 65 int-conversions to the integer assignments. Add a clause that fails on an uncast integer-to-pointer assignment or return, or better, gate on the emitted body compiling clean under -Werror=int-conversion. Do not let a builder close this by editing one declaration.

TWO MEASURED FACTS FOR THE SIBLINGS, both out of the --json arm of the same command, both free:
(a) local-type-assertion-target is CONFIRMED AT ITS ROOT and this is the harder half of its evidence. The JSON "variables" array for this function has 12 entries and contains ZERO register variables: 3 args plus 9 stack slots. v1 through v9 and v11 -- every register temporary the C body declares, INCLUDING v6 -- are absent from kuna's own variable inventory. The reason a local type assertion cannot target v6 is not that the assertion parser rejects the name; it is that kuna never exports a handle for it. That need is about the inventory, not the parser.
(b) A naming inconsistency in the DEFAULT namestyle, which no round-12 need currently covers: the same invocation prints the signature as sub_1005350(long a0, unsigned int a1, unsigned long *a2) in the C arm and names those same three parameters param_1, param_2, param_3 in the JSON arm. C output and JSON output disagree on parameter names in the default style, so any agent that reads names from --json and feeds them back to an assertion is wrong before it starts. This is NOT the already-refuted ghidra-naming-style-disconnects (that one needs --option namestyle ghidra; this reproduces with no option at all).

DEAD LINES CLOSED: both probe options are irrelevant to this defect. The filed reproduction pins calleearity and varargstackargs on; the round has repeatedly shown those two aim a builder at call/argument recovery, which is the wrong subsystem here. Drop them from probe and acceptance before dispatch, as was done for ghidra-naming-style-disconnects.
- closed: acceptance a-70bd6c755858 now PASSES at 29254c22e8fe
- REGRESSED: acceptance a-70bd6c755858 fails again at 3dc794bac7dc
- closed: acceptance a-70bd6c755858 now PASSES at 7e151cfaade7
- round 12 reconciliation: PR #563 (squash `29254c22e8fe5ce175f662f50587d85db3186d97`) implemented `declhightype` and explicitly validated this independently reported `graphy` function. PR #598 only changed terminal switch-label emission and is not the implementation for this need. The need record remained an untracked pipeline file and did not enter the #563 squash, so the authoritative index kept its pre-closure `open` snapshot even after the acceptance flipped.
- current controls at `342eaa03f0b71f7c312de004017eb8262fe0f63c` and its parent `4702e9dd582a41d8a89804b6a4467246ad63293a`: the exact dataset acceptance passes with default `declhightype on`, declaring `unsigned long *v6`; adding `--option declhightype off` restores the filed `unsigned long v6` declaration followed by dereferences. This only satisfies acceptance `a-70bd6c755858`: the default body still contains broader pointer/integer lifetime diagnostics, so this closure does not claim valid C or repair of the merged VM value's body-wide typing.
- index reconciliation constraint: at `b5c0b4f8b2159e871f18a99baae9a333954d9d12`, the authoritative committed index contains 197 active and 1 rejected records, but the clean commit exposes only 141 active Markdown sources and no rejected source directory to `needs reindex`. Restoring this record raises the source count to 142; a sparse-source regeneration would still destructively discard the other 55 active rows and the rejected ledger row, so this closure preserves the authoritative index byte-for-byte except for its `closed`/`open` totals and this need's object. Regenerate only after the missing sources have landed.
