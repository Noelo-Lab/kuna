---
need_id: bytecode-append-call-loses
title: Bytecode append call loses two live register arguments
track: quality
status: open
severity: major
probe_id: p-c3f3fe8f19b5
acceptance_id: a-827297af3fce
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [69b7492049fa49a2a260218f]
rounds: [12]
first_seen_round: 12
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

Recover the vector end and byte pointer passed in RDX and R8 at 0x1400046cc.

> **Bytecode append call loses two live register arguments** (major, `69b7492049fa49a2a260218f`)
> With calleearity and varargstackargs enabled, the final append call has only one argument. Kuna disassembly shows RDX and R8 prepared before the call. Explicit prototypes restore all three arguments. The catalog's sibling-call reconciliation has no witness here; this does not establish a regression in that capability.

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
    "sub_140004540",
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
      "\n  sub_[0-9a-f]+\\(a[0-9]+\\);\n\\}"
    ]
  },
  "target": {
    "binary_rel": "bin/vm.exe",
    "binary_sha256": "7a7009becef2678b45b7bc660ae134f7742e2632cb46e4e024281dcf0428b957",
    "binary_size": 44032,
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
    "sub_140004540",
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
      "\n  sub_[0-9a-f]+\\(a[0-9]+\\);\n\\}"
    ]
  },
  "target": {
    "binary_rel": "bin/vm.exe",
    "binary_sha256": "7a7009becef2678b45b7bc660ae134f7742e2632cb46e4e024281dcf0428b957",
    "binary_size": 44032,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x140004540 --binary target/vm.exe --raw` — No reference result: decompile reported no running server; subsequent load failed before registration. Kuna disassembly established the three register arguments.

## Instances

- `69b7492049fa49a2a260218f` (round 12, tester t-r12-69b74920)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `argument-recovery-drops-both` — same family (callee-arity under-applied at a call site; explicit prototype restores). Kept separate: Jaccard 0.138, distinct ABIs (MS x64 here, SysV there), distinct acceptances.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, round 12). SYMPTOM UPHELD AND THE OUTPUT IS GENUINELY WRONG; nothing the need filed is false, but its only diagnostic element (the two arity options) is DEAD, and the mechanism is caller-side, not callee-side.

WHAT WAS VERIFIED. (1) Probe reproduces: `kuna decompile vm.exe sub_140004540` ends in `sub_140007be0(a0);`. (2) The loss is real wrong output, not a judgement call: kuna ITSELF recovers the callee as `char *sub_140007be0(long long *a0,long long a1,char *a2)`, and its prologue reads both dropped registers (`MOV R12,R8` @0x140007bfd, `MOV RBP,RDX` @0x140007c03). Two live, meaningful arguments are dropped at the call site.

FOUR DEAD LINES, so no builder repeats them. (a) calleearity and varargstackargs are DEFAULT-ON, so 'with both enabled' is vacuous; both OFF emits byte-identical output to the default. Neither option is on this path in either direction. (b) Callee arity does NOT fail -- it succeeds, 3 params, see above. (c) It is not a single-function-mode blind spot: whole-binary `decompile-all --json` over all 172 functions still emits `sub_140007be0(a0)`. (d) The trial machinery is alive in this very binary -- 223 call sites recover 2+ arguments (vs 196 one-arg, 31 zero-arg) in the same MS x64 image. Control for the silent-unknown-option trap: an unknown --option now errors, so --option does reach this command.

THE WORKAROUND REPRODUCES: `--assert 'prototype 0x140007be0 char *append_slow(long long *v,long long n,char *p)'` emits `sub_140007be0(a0,(long long)n,p)` -- three arguments.

REDIRECT (offered as a redirect, NOT a proven line) -- AND THIS NEED IS A CLEANER WITNESS THAN ITS SIBLING. Same gate the argument-recovery-drops-both refutation named: Funcdata::only_op_use / ancestor_op_use, substrate/funcdata_varnode.rs:2236 and :2405, whose own doc says 'a branch/load/store/non-matching-call/persistent-output use means the Varnode is not exclusively a parameter'. Here the disqualifier is visible in three instructions: RDX and R8 are set up BEFORE the branch (`MOV RDX,[RBX+8]` @0x1400046af, `LEA R8,[RSI+RDI]` @0x1400046b3), and on the fall-through (fast) path RDX takes a STORE (`MOV [RDX],AL` @0x1400046c1) and R8 a LOAD (`MOVZX EAX,[R8]` @0x1400046bd). The call at 0x1400046cc is on the OTHER (JZ) path. Inlined-fast-path-plus-slow-path-call is the shape: the same registers feed a load/store on the sibling block and a call on this one, and the load/store descendants disqualify them. Prefer this witness over the sibling's when building -- it is 6 instructions and one branch.

ACCEPTANCE IS SOUND, unlike the sibling's: stdout_absent on `\n  sub_[0-9a-f]+\(a[0-9]+\);\n\}` only demands the one-argument tail-call form disappear, so a fix recovering both arguments passes and a fix inventing extra ones is not rewarded. No trap here.

FOR T_TRIAGE: this pairs with argument-recovery-drops-both under one mechanism in two ABIs (MS x64 here, SysV there). Giving the pair a shared `cluster:` so ONE builder takes both is the cheap lever; the ablations transfer wholesale.
