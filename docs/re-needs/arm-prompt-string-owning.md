---
need_id: arm-prompt-string-owning
title: ARM prompt string has no owning function
track: quality
status: open
severity: major
probe_id: p-bff94f2df22d
acceptance_id: a-7837f5c28063
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5d33c5d40ad448c69c]
rounds: [6]
first_seen_round: 6
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis]
scope: small
regression_of: arm-literal-pool-string
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Navigate from the input prompt to main.

> **ARM prompt string has no owning function** (major, `5ab77f5d33c5d40ad448c69c`)
> The prompt at 0x9124 has xrefs_count=0 and functions=[]. xrefs also returns zero. Disassembly shows ldr r0,[0x9024] at 0x8d20, loading the prompt address immediately before printf. The strings interface offers no manual function definition to repair ownership.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "strings",
    "{{BIN}}",
    "--json",
    "--filter",
    "Enter your key"
  ],
  "target": {
    "binary_rel": "bin/arm_kgme1.tar.gz.__x/arm_kgme1.tar.__x/arm_kgme1",
    "binary_sha256": "49c725b2580c62d802009a94f9db70c298367909ed578ce793fca2838807002e",
    "binary_size": 6484,
    "binary_source": "dataset"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "strings[0].functions",
        "op": "len_eq",
        "value": 0
      }
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
    "strings",
    "{{BIN}}",
    "--json",
    "--filter",
    "Enter your key"
  ],
  "target": {
    "binary_rel": "bin/arm_kgme1.tar.gz.__x/arm_kgme1.tar.__x/arm_kgme1",
    "binary_sha256": "49c725b2580c62d802009a94f9db70c298367909ed578ce793fca2838807002e",
    "binary_size": 6484,
    "binary_source": "dataset"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "strings[0].functions",
        "op": "len_gt",
        "value": 0
      },
      {
        "path": "strings[0].xrefs_count",
        "op": "gt",
        "value": 0
      }
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- May follow from incomplete function discovery.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5d33c5d40ad448c69c` (round 6, tester t-r6-5ab77f5d)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 6 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK BY THE CAPTAIN (round 6), all claims MEASURED on the need's own arena binary arm_kgme1 (stripped ARM32 ELF), kuna @ 9c4c4508.

VERDICT: UPHELD. 'Incomplete function discovery' IS the cause. But three refinements below change which file the builder opens, and one of them says the need's own front matter is wrong.

1. THE SURFACE CHECK LIES -- DO NOT CONCLUDE 'DISCOVERY IS FINE' FROM 'functions' COVERING THE LOAD.
'kuna functions' reports 13 functions incl. _INIT_0 @0x83e8 size 3368, i.e. a declared extent 0x83e8..0x9110 that NOMINALLY CONTAINS the load site 0x8d20. A builder who checks 'is 0x8d20 covered?' gets YES and will wrongly clear discovery. That extent is GAP-FILL, not a CFG extent. Measured: 'xrefs --from 0x83e8' returns only 4 edges, ALL inside 0x83e8..0x840c -- the walk stops at the unconditional 'b 0x8350' at 0x8404, 36 bytes into a 3368-byte declared extent. Everything past 0x8404, including 0x8d20, is never walked.

2. THE MISSING FUNCTION IS main @0x8cf8, AND ITS PROLOGUE IS TEXTBOOK.
0x8cf4 'ldmia sp,{r11,sp,pc}' (previous epilogue), then 0x8cf8 'mov r12,sp' / 0x8cfc 'stmdb sp!,{r11,r12,lr,pc}' / 0x8d00 'sub r11,r12,#0x4' / 0x8d04 'sub sp,sp,#0xa8'. Discovery never files it. 0x8d20 'ldr r0,[0x9024]' then 0x8d24 'bl 0x835c' is exactly the printf call the tester described. The fix shape is an ARM prologue-signature scan over undiscovered gaps; it is NOT an xref change.

3. THE 'regression_of: arm-literal-pool-string' LINK IS FALSE, AND IT IS WHY THIS RANKS #1 OF 97 (1110.90).
PR #431 (poolref) is INTACT -- proved by control, not assumed. Re-ran #431's OWN round-3 case (.kuna-repipe/arena/3/5ab77f5733c5d40ad448c380, 1337ARM.bin): 'FATAL: kernel too old' still resolves xrefs=1 functions=[__libc_start_main]. So poolref did not regress; it is simply NEVER REACHED here. Its input edge is absent: 'xrefs --to 0x9024' (the pool word itself) = 0, so the base read edge 0x8d20->0x9024 was never filed, so poolref has nothing to extend. kuna_poolref.rs is also unconditionally wired into xrefs.rs (no option gate) -- I checked. DO NOT OPEN kuna_poolref.rs. cluster.py linked these on near-identical TITLES ('ARM literal-pool string use has no owning function'); the mechanisms are different layers. The REGRESSION_FLOOR that puts this ahead of a 3-instance blocker therefore rests on a false premise -- a T_TRIAGE call, flagged not acted on.

4. ACCEPTANCE TRAP -- BOTH THE RIGHT AND THE WRONG FIX FLIP THE PROBE.
Acceptance asserts only strings[0].functions len_gt 0. Discovering main @0x8cf8 flips it (attribution -> main). But so does the CHEAP WRONG FIX of linearly walking the gap-fill extent, which flips it while attributing the prompt string to the bogus mega-function _INIT_0. The probe cannot tell these apart; prefer the discovery fix and consider asserting the owning function is not _INIT_0.

Pool word confirmed by hand: 0x9024 holds bytes 24 91 00 00 = 0x9124 = the string, and kuna currently renders that data word as 'andeq r9,r0,r4, lsr #0x2'.
- round 6 T_TRIAGE: ACCEPTANCE REWRITTEN (a-bb4686077f58 -> a-7837f5c28063), and it was verified two-sided before the swap: it FAILS on the need's own arena binary at 9c4c4508 (`probe check --bin` rc=1) and PASSES when simulated against plausible correct renderings. It `strings[0].functions len_gt 0` alone is satisfiable by attributing the string to the bogus gap-fill `_INIT_0` extent that already nominally covers the load site (see the refuter note). Now also requires `strings[0].xrefs_count > 0` (today it is 0), which cannot be produced without actually disassembling the literal-pool load at 0x8d20 -- i.e. without the discovery fix.
- round 6 T_TRIAGE: TRIAGE: touches kuna-decomp -> kuna-analysis: the refuter placed the gap in function discovery (the real missing function is main @0x8cf8; _INIT_0's 3368-byte extent is gap-fill whose walk stops at the `b 0x8350` at 0x8404).
