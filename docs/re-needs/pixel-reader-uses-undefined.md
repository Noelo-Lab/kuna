---
need_id: pixel-reader-uses-undefined
title: Pixel reader uses undefined stack objects after an accepted API prototype
track: quality
status: open
severity: blocker
probe_id: p-058e4fa81777
acceptance_id: a-4e3b8ec59e3f
hypothesis_status: upheld
credibility: 1.0
instances: 3
challenges: [5ab77f5d33c5d40ad448c6f6, 68bffbe9224c0ec5dcedbe7a, 6a17264017539b5175d123a9]
rounds: [6]
first_seen_round: 6
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

Recover the ReadFile buffer and bytes-read variable in the bitmap decoder at 0x401290.

> **Pixel reader uses undefined stack objects after an accepted API prototype** (blocker, `5ab77f5d33c5d40ad448c6f6`)
> The LocalAlloc prototype is applied, but ReadFile writes to undeclared Stackffffffe0 and Stackffffffe4 objects. The character sum uses a separate variable assigned zero. Explicit ReadFile and SetFilePointer prototypes did not repair this. Kuna disassembly shows the loop reading the same ESP-relative slots passed to ReadFile. Probe passed and acceptance failed in local verification.

> **Windows DWORD argument includes undefined upper stack bytes** (major, `68bffbe9224c0ec5dcedbe7a`)
> Protection flags become CONCAT44((int)(vN >> 0x20),0x40), although the instruction writes DWORD 0x40. CreateProcessW similarly consumes undefined upper bytes for BOOL. Explicit standard-width prototypes remove these artifacts.

> **Accepted VM and cookie prototypes leave an undefined Boolean return** (major, `6a17264017539b5175d123a9`)
> Default VM output is void. An explicit unsigned-char VM prototype returns the cookie-check call result instead. Adding a void cookie prototype yields an unassigned unsigned-char local returned by both exits. Explicit calleepreserves on and msvcstackguard on do not repair it. Disassembly sets AL before the cookie call; the checker's normal return path does not write AL.

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
    "sub_401290",
    "--assert",
    "prototype LocalAlloc void *LocalAlloc(unsigned int flags,unsigned int size)",
    "--json"
  ],
  "target": {
    "binary_rel": "bin/Imagination.zip.__x/Imagination.exe",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/stdcallpop_pe_i386.exe",
    "binary_sha256": "78d28ec6d13b9c1634101e86b2042ed61560db31ba63705adfb2b75f5757b7b4",
    "binary_size": 40960,
    "binary_source": "in-repo"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "ReadFile\\([^;]*&Stack[fF0-9a-f]+"
    ],
    "json": [
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
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
    "decompile",
    "{{BIN}}",
    "sub_401290",
    "--assert",
    "prototype LocalAlloc void *LocalAlloc(unsigned int flags,unsigned int size)",
    "--json"
  ],
  "target": {
    "binary_rel": "bin/Imagination.zip.__x/Imagination.exe",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/stdcallpop_pe_i386.exe",
    "binary_sha256": "78d28ec6d13b9c1634101e86b2042ed61560db31ba63705adfb2b75f5757b7b4",
    "binary_size": 40960,
    "binary_source": "in-repo"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "ReadFile\\("
    ],
    "stdout_absent": [
      "&Stack[fF0-9a-f]+"
    ],
    "json": [
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
      }
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Possibly incorrect x86 Windows callee stack-cleanup tracking.
- Recovered API argument widths default to machine-word width.
- The preservation summary may reject the checker because its failure branch reaches a complex no-return routine. Prototype overrides cannot express the returning-path register effect.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x401290 --binary ./target/Imagination.zip.__x/Imagination.exe --raw` — Reference unavailable: no running server. Subsequent ida-decompile load exited 1 because the server exited before registering. No reference pseudocode obtained; superiority remains unverified.
- `ida-decompile load ./target/crackme_very_hard.exe` — Reference backend failed to start. No comparison pseudocode was available.

## Instances

- `5ab77f5d33c5d40ad448c6f6` (round 6, tester t-r6-5ab77f5d)
- `68bffbe9224c0ec5dcedbe7a` (round 6, tester t-r6-68bffbe9)
- `6a17264017539b5175d123a9` (round 6, tester t-r6-6a172640)

## Decision log

- filed by cluster.py from 3 observation(s)
- round 6 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK BY THE CAPTAIN (round 6) on the WITNESS ONLY (5ab77f5d33c5d40ad448c6f6 / Imagination.exe, PE32 x86), kuna @ 9c4c4508. SCOPE FIRST: this need carries 3 hypotheses that belong to its 3 different instances. I tested the witness, so I judge H1 ('possibly incorrect x86 Windows callee stack-cleanup tracking'). H2 (arg widths, 68bffbe9...) and H3 (preservation summary, 6a172640...) were NOT tested here -- do not read this verdict as covering them.

VERDICT ON H1: UPHELD, and the truth is worse than filed -- THE EMITTED C IS SEMANTICALLY WRONG, NOT MERELY UNDECLARED.

GROUND TRUTH from the disassembly, frame traced by hand (E = entry ESP; prologue is SUB ESP,8 + 4 pushes so ESP = E-24):
  0x4012a9  MOV EBP,[ESP+0x1c]  = E+4   <- hFile is an INCOMING PARAMETER
  0x4012c4  LEA EAX,[ESP+0x14]  = E-4   <- slot_A = lpNumberOfBytesRead
  0x4012cb  LEA ECX,[ESP+0x18]  = E-8   <- slot_B = lpBuffer  (ESP is 8 lower here: PUSH 0, PUSH EAX intervene)
  0x4012e3  MOV EAX,[ESP+0x14]  = E-4   <- reads slot_A (the !bytesRead test)
  0x4012eb  MOV EAX,[ESP+0x10]  = E-8   <- reads slot_B == THE VERY BUFFER ReadFile FILLED
So lpBuffer and the checksum source ARE THE SAME SLOT (E-8). Every one of these offsets is relative to a MOVING ESP -- the lpBuffer LEA sits between two PUSHes -- which is exactly the tracking H1 names.

WHAT kuna PRODUCES INSTEAD:
  ReadFile(v5,&Stackffffffe0,4,&Stackffffffe4,0);
  wsprintfA(v1,0x406030,v1,(v3 >> 8 & 0xff) + (v3 >> 0x10 & 0xff) + (v3 & 0xff));   // v3, 'stack - 0x34', assigned v3 = 0 in the loop
The one slot E-8 has been SPLIT INTO TWO INDEPENDENT OBJECTS: the buffer (&Stackffffffe0) and the checksum source (v3). Because the second is never written by the ReadFile edge, kuna assigns it 0, so THE EMITTED PROGRAM CHECKSUMS A CONSTANT ZERO. An RE agent reading this gets a wrong algorithm, not just an ugly name. That is the blocker, and the need's title understates it.

SECOND, INDEPENDENT DEFECT ON THE SAME FUNCTION: PARAMETER RECOVERY FAILED. hFile is read from E+4 (an incoming argument) but kuna declares 'void *sub_401290(void)' and models it as v5, an uninitialized local at 'stack - 0x4', then emits 'v4 = v5;' and 'SetFilePointer(v5,...)' reading it before any write. The probe's LocalAlloc prototype assertion cannot reach this; no --assert on a CALLEE fixes the SUBJECT's own missing parameter.

THE FRAME IS NOT MISSING THE SYMBOLS -- THE REFERENCES DO NOT BIND TO THEM. 'variables' already contains local_20 (stack_offset -32 = 0xffffffe0) and local_1c (stack_offset -28 = 0xffffffe4), i.e. symbols at EXACTLY the offsets the synthetic Stackffffffe0/Stackffffffe4 names encode -- both with line_numbers [] and addresses [], so they are mapped and unreferenced while the printer emits a synthetic name. So this is a binding/identity failure, NOT a missing-frame-layout failure. Do not go build frame layout.

*** ACCEPTANCE TRAP -- THE PROBE CAN BE PASSED BY A COSMETIC RENAME THAT FIXES NOTHING. ***
Acceptance asserts ONLY stdout_absent 'ReadFile\([^;]*&Stack[fF0-9a-f]+'. Simply binding the two address-taken references to the ALREADY-EXISTING local_20/local_1c symbols renders '&local_20' and FLIPS THE ACCEPTANCE GREEN while leaving the split intact -- v3 would still be assigned 0 and the checksum would still be computed over zero. A builder can close this need without fixing the blocker. Before accepting a fix, require that the checksum operand and ReadFile's lpBuffer are THE SAME object (and ideally that sub_401290 gains its parameter); consider tightening the acceptance to assert the wsprintfA operand is the buffer, not a zero-assigned local.
- round 6 T_TRIAGE: ACCEPTANCE REWRITTEN (a-5979e658cd8b -> a-4e3b8ec59e3f), and it was verified two-sided before the swap: it FAILS on the need's own arena binary at 9c4c4508 (`probe check --bin` rc=1) and PASSES when simulated against plausible correct renderings. It two holes. (a) absence-only on `ReadFile\([^;]*&Stack...`, so re-rendering the first argument alone or renaming the pseudo-symbol turns it green -- the absence is now on `&Stack[fF0-9a-f]+` ANYWHERE in the function, which is never a correct rendering. (b) the probe carried `assertions[0].status == applied` and the acceptance DROPPED it, so a builder could win by making the LocalAlloc assertion stop applying; that clause is restored, and `ReadFile\(` must still be present.
