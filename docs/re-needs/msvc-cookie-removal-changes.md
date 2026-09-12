---
need_id: msvc-cookie-removal-changes
title: MSVC cookie removal changes failure return from 1 to 0
track: quality
status: closed
severity: major
probe_id: p-edd7d1f331d9
acceptance_id: a-7c038829dc65
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [68bffbe9224c0ec5dcedbe7a]
rounds: [6, 12]
first_seen_round: 6
attempts: 1
covered_by_option: msvcstackguard
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 587
closed_in_round: 12
closing_pr: "587"
reject_reason: null
---

## Symptom

Recover the wrapper's success and failure return values.

> **MSVC cookie removal changes failure return from 1 to 0** (major, `68bffbe9224c0ec5dcedbe7a`)
> Default output returns the cookie helper result. An explicit void prototype leaves an undefined return local. With msvcstackguard on, both paths return 0. Disassembly shows MOV EAX,0x1 on failure and XOR EAX,EAX on success; the cookie helper preserves RAX on its returning path.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/runtime.exe",
    "binary_sha256": "4283a5283c9d5375535da28dff9a98b78c1d660b1b44dcd53dd2c10a2eb85c75",
    "binary_size": 52736,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x140001010",
    "--option",
    "msvcstackguard",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "CrackMe failed[^;\n]*;\\s*return 0;"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/runtime.exe",
    "binary_sha256": "4283a5283c9d5375535da28dff9a98b78c1d660b1b44dcd53dd2c10a2eb85c75",
    "binary_size": 52736,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x140001010",
    "--option",
    "msvcstackguard",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "CrackMe failed[^;\n]*;\\s*return 1;"
    ],
    "stdout_absent": [
      "CrackMe failed[^;\n]*;\\s*return 0;"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Cookie-call removal may discard the reaching return definition.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/runtime.exe` — Backend exited with status 1 before registering. No reference pseudocode obtained.

## Instances

- `68bffbe9224c0ec5dcedbe7a` (round 6, tester t-r6-68bffbe9)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 6 REFUTER: hypothesis **overturned** (was inconclusive). REFUTED IN-TICK by the captain on the need's own arena binary (.kuna-repipe/arena/6/68bffbe9224c0ec5dcedbe7a/target/runtime.exe, sub_140001010), release kuna at 9c4c4508. SYMPTOM STANDS: 'kuna decompile runtime.exe 0x140001010 --option msvcstackguard on' emits 'return 0;' at BOTH exits, and the failure exit is a false statement -- the disassembly sets MOV EAX,0x1 at 0x1400010fb (failure) and XOR EAX,EAX at 0x1400011cd (success), and both fall into the SHARED epilogue at 0x140001100 whose only content is the cookie reload, XOR RCX,RSP, CALL 0x140001430 and RET at 0x140001124. HYPOTHESIS OVERTURNED ('Cookie-call removal may discard the reaching return definition'): the reaching definition is ALREADY gone with the pass OFF, so removal cannot be what discards it. Control 1 -- default, no msvcstackguard: both exits emit 'return sub_140001430(v5);' and the constant 1 appears NOWHERE in the function. Control 2 -- declare the checker void and keep the pass off ('--assert-strict --assert "prototype sub_140001430 void sub_140001430(unsigned long long v)"'): the call becomes a statement and both exits become 'return v3;' with v3 an UNDEFINED local -- still one merged value, still not phi(1,0). Control 3 -- that same void prototype WITH msvcstackguard on: back to 'return 0;' twice. Control 4 -- 'calleepreserves on' + 'msvcstackguard on': byte-identical, still 'return 0;' twice, matching the sibling refutation of [[main-returns-invented-cookie]] and that option's own module header (it re-promotes the range to killedbycall when it is the call's return storage). REAL MECHANISM, and it is the SAME GAP as main-returns-invented-cookie wearing different words: RAX is in <killedbycall> for the x86-64-win prototype model, guardCalls plants an INDIRECT at the /GS checker call, and that breaks the def-use chain from BOTH pre-call defs to the RET. What differs is only the substitute the three modes print for a value that was lost identically in all three: the call's output (default), an undefined local (void proto), the constant 0 (msvcstackguard). msvcstackguard is a P7 pass, so by the time it destroys the CALL both defs have long been dead-coded for want of a reader -- deleting the call cannot resurrect them, which is why the pass makes the output WORSE (an opaque call result is at least visibly opaque; 'return 0' on a path that returns 1 reads as recovered truth). A builder must NOT go looking inside kuna_msvcstackguard.rs for something it deletes; it deletes only the CALL, its FuncCallSpecs and the cancelled XOR chain, exactly as its header says. ACCEPTANCE IS WEAK -- T_TRIAGE MUST HARDEN IT: a-b3a6a8f8cd4f has only a stdout_absent clause ('CrackMe failed...;\\s*return 0;') and NO positive clause, so it flips green if the failure exit prints ANY other value, including the undefined 'return v3;' that control 2 already produces today. It must demand 'return 1;' on the failure exit (compare main-returns-invented-cookie's acceptance, which pairs absence with a required 'return 0;'). DISPATCH: hold this need and main-returns-invented-cookie on ONE builder via a shared cluster lease -- they are one mechanism -- but they are NOT the same binary: this one is challenge 68bffbe9 (runtime.exe), main-returns-invented-cookie lists 69b74920/69d6e9ba, so a fix must be measured against both. HARNESS TRAP CONFIRMED WHILE REFUTING: '--assert' with a malformed directive is SILENT and the run still succeeds -- 'prototype 0x140001430 void(uint64)' (address instead of name, no C decl) changed nothing and exited 0. Always pass --assert-strict when an assertion is the control, or the control proves nothing.
- round 6 T_TRIAGE: ACCEPTANCE REWRITTEN (a-b3a6a8f8cd4f -> a-7c038829dc65), and it was verified two-sided before the swap: it FAILS on the need's own arena binary at 9c4c4508 (`probe check --bin` rc=1) and PASSES when simulated against plausible correct renderings. It ACCEPTED A NON-FIX: absence-only on `CrackMe failed...; return 0;` goes green if the return is merely deleted or the string moves. Now REQUIRES `return 1;` at that site, which is the real value -- `CALL 0x1400011e0` (the failure printf) @0x1400010f6 is immediately followed by `MOV EAX,0x1` @0x1400010fb, disassembled to confirm.
- implementation attempt 1: the late exact `/GS` recognizer records only its
  direct, unread-output, one-cancel call site and requests a pipeline restart.
  On replay, P3 preserves only ABI output storage at that seeded
  `KILLEDBYCALL`; an explicit effect override remains authoritative. P7 then
  removes the checker with its existing exact algebra. No generic tail-call,
  no-return, flow, or prototype policy is changed. Need remains OPEN and
  `pr: null` until review/merge assigns the durable closing reference.
- closure gate for PR #587: acceptance a-7c038829dc65 PASSES at
  7af54b0b14e1. On the pinned dataset artifact, `msvcstackguard on` returns
  success 0 / failure 1 with both the default and `calleetpreserves off`;
  `msvcstackguard off` retains the raw checker-call return under both callee
  settings. Focused units pass 18/18, the focused stage pair passes 12/12,
  full stage parity passes 794/794, and the final serial CLI suite passes
  138/138. The record is closed for this measured candidate; closure becomes
  durable only when PR #587 merges.

## Implementation verification

The implementation is measured against the dataset artifact itself, not a
vendored or reconstructed sample:

- path: `challenges/68bffbe9224c0ec5dcedbe7a/bin/runtime.exe`
- SHA-256: `4283a5283c9d5375535da28dff9a98b78c1d660b1b44dcd53dd2c10a2eb85c75`
- size: 52,736 bytes
- function: `0x140001010`

The acceptance probe above is the primary two-sided check: the failure message
must be followed by `return 1;` and must not be followed by `return 0;`.
Three controls close the adjacent escape hatches:

1. Add `--option calleeretpreserves off` while keeping `msvcstackguard on`:
   failure still returns 1 and success still returns 0. This proves the repair is
   owned by the destructive `/GS` option rather than the generic callee-body
   heuristic.
2. Set `msvcstackguard off`: both exits retain the raw
   `return sub_140001430(...)` behavior. This proves the opt-out path is
   unchanged.
3. The stage fixture `tests/stages/kuna-msvcstackguard-return.xml` crosses both
   options over a two-arm/shared-epilogue caller. Its checker failure tail enters
   a helper containing an ordinary returning CALL before INT 0x29, so the
   generic body probe is deliberately incomplete. Unit negatives additionally
   reject missing/mismatched algebra, duplicate cancels, indirect calls, and
   live checker outputs; `tests/stages/kuna-calleeretpreserves.xml` retains the
   `endbr64; ret` negative and direct/tailcallee `RAX`-writer controls.
