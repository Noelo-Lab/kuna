---
need_id: live-loop-counter-stores
title: Live loop-counter stores disappear from the checker
track: quality
status: open
severity: blocker
probe_id: p-c3c218c4054a
acceptance_id: a-621e3664421d
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [65acadf3eef082e477ff5ede]
rounds: [5]
first_seen_round: 5
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

Recover terminating checker loops and the final serial computation.

> **Live loop-counter stores disappear from the checker** (blocker, `65acadf3eef082e477ff5ede`)
> A for loop initializes and tests one local but assigns its increment to another. Other loops also lose increments. Disassembly confirms actual counter stores, including 0x1400035ff. Stores at 0x140003a11 and 0x140003abb also disappear, leaving the final comparison value unassigned. Reliable mode with stackalias on did not resolve this. Also hit already-filed stack-arrays-acquire-incorrect; not filed again.

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
    "sub_140002530"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "for \\((v[0-9]+) = 0; \\1 <= 4; (?!\\1\\b)v[0-9]+ = \\1 \\+ 1\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/ObfuscationFiesta.exe",
    "binary_sha256": "b852fd5bfa65f7d71b903fb1852ae8ad03f2611637a42d501a897424647deec9",
    "binary_size": 32768,
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
    "sub_140002530"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "for \\((v[0-9]+) = 0; \\1 <= 4; (?!\\1\\b)v[0-9]+ = \\1 \\+ 1\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/ObfuscationFiesta.exe",
    "binary_sha256": "b852fd5bfa65f7d71b903fb1852ae8ad03f2611637a42d501a897424647deec9",
    "binary_size": 32768,
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

- `ida-decompile decompile 0x140002530 --binary ./target/ObfuscationFiesta.exe --output notes/ida-main.c` — No pseudocode obtained: decompile required a running server, then ida-decompile load failed with status 1 before registering. This is an infrastructure failure; IDA's decompilation quality remains unknown.

## Instances

- `65acadf3eef082e477ff5ede` (round 5, tester t-r5-65acadf3)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **inconclusive**. REFUTED IN-TICK (captain, round 5). VERDICT: INCONCLUSIVE -- a refuter ran, the SYMPTOM IS CONFIRMED REAL AND SEVERE, and the single most plausible prior cause was RULED OUT by A/B. No hypothesis was on file to uphold, and I could not close the last step from the C text alone.

THE SYMPTOM IS REAL AND IT IS AN INFINITE LOOP, NOT A COSMETIC RENAME. Reproduced verbatim:
  kuna decompile ObfuscationFiesta.exe sub_140002530
  -> for (v55 = 0; v55 <= 4; v7 = v55 + 1) { v46 = (v46 ^ v55 * 0x17) + 0x11; }
The induction variable v55 is never updated; the increment lands on v7. As written the loop does not terminate, so this is invalid C, not a naming nit.

THE BINARY SAYS THE COUNTER IS A STACK SLOT AND THE STORE IS REALLY THERE. `kuna disassemble --addr 0x1400035f0 --count 10`:
  0x1400035f4  JMP 0x140003606                     <- test-first entry
  0x1400035f6  MOV EAX,dword ptr [RSP + 0x84]
  0x1400035fd  INC EAX
  0x1400035ff  MOV dword ptr [RSP + 0x84],EAX      <- the store the output loses
  0x140003606  CMP dword ptr [RSP + 0x84],0x5
  0x14000360e  JGE 0x140003629
One slot, [RSP+0x84], is loaded, incremented, stored back, and re-read by the test. kuna renders the LOAD side correctly (v55 <= 4) and misplaces the STORE side (v7 = ...). So the load and the store of one stack slot end up on two different variables.

RULED OUT -- DO NOT SPEND THE BUILDER HERE: DIV-105 / tiedstorekeep (the shipped "marker copy-propagation deletes frame stores" pass) IS NOT THE CAUSE. A/B on the arena binary:
  --option tiedstorekeep off -> for (v57 = 0; v57 <= 4; v7 = v57 + 1)
  --option tiedstorekeep on  -> for (v55 = 0; v55 <= 4; v7 = v55 + 1)
Byte-identical defect both ways; only the variable numbering shifts. That was the leading candidate given the shape and it is dead. (The tester`s own negative -- reliable mode with stackalias on -- also stands.)

THE EVIDENCE THAT MAKES ME DOUBT THE FILED TITLE ("stores DISAPPEAR"). An assignment IS emitted; it is attributed to the wrong variable. And v7 is not an innocent register temp: in the declaration block every register-resident local carries its storage (`unsigned int v10; // eax`, `unsigned long long v11; // rax`), while v7 is declared bare `int v7;` -- no storage comment -- and is reused across the whole function for unrelated values: v7 = sub_140004c70(), v7 = GetTickCount(), v7 = (int)v50, v7 = v52 + 1, v7 = v53 + 1, v7 = v55 + 1, v7 = v57 + 1. That is the signature of ONE over-merged HighVariable collecting the def side of at least four different loop counters plus two call returns. FIVE loops in this one function show the same shape, which is why the tester saw "other loops also lose increments" (0x140003a11, 0x140003abb) -- it is one defect, not three.

WHY INCONCLUSIVE RATHER THAN A VERDICT. Two readings produce identical C and I could not separate them without a p-code dump: (a) the store to [RSP+0x84] is deleted and `v7 = v55 + 1` is the surviving register def of INC EAX, or (b) the store survives but its output varnode is merged into the wrong HighVariable. Reading (b) is better supported -- v7 has no single storage, which is what an over-merge looks like and not what a plain EAX temp looks like -- but I am not asserting it. THE BUILDER SHOULD START BY DECIDING (a) vs (b), because the fix lives in a different phase for each: (a) is dead-store/copy-prop in P3, (b) is merge/naming in P6. Do that before writing any option.

CHEAP STARTING POINT: the loop at 0x1400035f4 is small, isolated, and test-first; `--define-function` around 0x1400035f0-0x140003629 should reproduce the misattribution in a few lines instead of a 600-line function.
round 5 TRIAGE (captain): CONFIRMED AS FILED -- quality / kuna-decomp / small / blocker. This is the round's ONE genuine refuter-backed `inconclusive`: a refuter ran and could not decide, which is not the same as unexamined. Dispatch is allowed, but the builder owes a diagnosis in design before it writes code.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
