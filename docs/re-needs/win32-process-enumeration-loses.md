---
need_id: win32-process-enumeration-loses
title: Win32 process enumeration loses required call arguments
track: quality
status: open
severity: major
probe_id: p-92a90707ef5a
acceptance_id: a-e1701a898c8f
hypothesis_status: overturned
credibility: 1.0
instances: 2
challenges: [640a526833c5d447bc761899, 68d9ee36224c0ec5dcedc3fc]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p4_calls]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Follow the snapshot handle and entry buffer through the anti-debug loop.

> **Win32 process enumeration loses required call arguments** (major, `640a526833c5d447bc761899`)
> Emitted CreateToolhelp32Snapshot(2), CloseHandle(), and Process32NextW(). Both argument-recovery options retained the omissions. Kuna disassembly shows the missing arguments explicitly prepared in RCX/RDX. Main additionally invents an argument to GetCurrentProcess.

> **VirtualAlloc loses its fourth register argument** (major, `68d9ee36224c0ec5dcedc3fc`)
> Emitted VirtualAlloc(0,0x36,0x3000), omitting 0x40 passed in R9D. Enabling calleearity and varargstackargs did not restore it. A prototype assertion did.

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
    "sub_1400015c0",
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
      "Process32NextW\\(\\s*\\)",
      "CloseHandle\\(\\s*\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
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
    "sub_1400015c0",
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
      "Process32NextW\\([^),]+,[^)]+\\)",
      "CloseHandle\\([^)]+\\)"
    ],
    "stdout_absent": [
      "Process32NextW\\(\\s*\\)",
      "CloseHandle\\(\\s*\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Missing imported Win32 prototypes leave recovery dependent on local argument scoring.
- Missing WinAPI prototype knowledge combined with register argument recovery.

## Refutation

**OVERTURNED** -- symptom stands, filed mechanism does not (captain, round 3, cf5234ac).

Filed hypothesis: "missing imported Win32 prototypes leave recovery dependent on local
argument scoring". Measured on `KeyCheker.exe`, `sub_1400015c0`, one run, no assertions:

```
v1 = Process32FirstW(CreateToolhelp32Snapshot(2),v3);   <- 2 args recovered
    CloseHandle();                                      <- 0 args
v1 = Process32NextW();                                  <- 0 args
```

Three imported kernel32 calls, one run, no prototype knowledge for any of them, and one of
them recovers both arguments. Import prototypes are therefore NOT the gate. The disassembly
says what is: the argument register is defined in a PREDECESSOR block.

```
0x140001600  CALL [0x140005018]   ; Process32FirstW -- RCX set 6 bytes earlier, same block  -> recovered
0x14000161e  MOV  RCX,RDI         ; hoisted above the JNZ
0x140001624  JNZ  0x140001656
0x14000162b  CALL [0x140005010]   ; Process32NextW  -- RCX from the predecessor block       -> dropped
0x14000165a  CALL [0x140005020]   ; CloseHandle     -- RCX from the same predecessor block   -> dropped
```

Note the second arg of Process32NextW (`LEA RDX,[RSP+0x20]` at 0x140001626) is set in the
call's OWN block and is dropped too, so once the first slot's trial dies the whole list dies.

The values are present in the dataflow the whole time: with
`--assert 'prototype CloseHandle int4 CloseHandle(void *h);'` and the matching
Process32NextW assertion, the same run emits `CloseHandle(h)` and `Process32NextW(h,v3)`
with h the CreateToolhelp32Snapshot result. So this is input-trial scoring at a call whose
argument register is defined in a dominating predecessor, in kuna-decomp p4_calls -- not
missing knowledge, and NOT a Win32 prototype database. A builder who ships kernel32
prototypes would make this need's acceptance pass while leaving the general defect (which is
not Windows-specific) untouched.

ACCEPTANCE STRENGTHENED for exactly that reason: it previously asserted only the ABSENCE of
`Process32NextW()`/`CloseHandle()`, which a prototype table -- or a fabricated argument --
satisfies. It now also requires two non-empty arguments at Process32NextW and one at
CloseHandle.

## Reference

_none recorded_

## Instances

- `640a526833c5d447bc761899` (round 3, tester t-r3-640a5268)
- `68d9ee36224c0ec5dcedc3fc` (round 3, tester t-r3-68d9ee36)

## Decision log

- filed by cluster.py from 2 observation(s)
captain T_DEDUP r3: SPLIT out of the same 5-observation cluster (obs10 + obs13). Both name missing imported Win32 prototypes, and obs13 records that an explicit prototype assertion DID restore the argument -- so the gap is import prototype knowledge, not the argument-recovery machinery. Related but NOT identical to the closed need argument-recovery-knobs-still; check that need's acceptance at the next acceptance-suite run.
captain T_REFUTE r3: hypothesis overturned -- see ## Refutation (measured on cf5234ac with the release binary).
captain T_TRIAGE r3: touches CORRECTED per the T_REFUTE overturn: this is p4_calls input-trial scoring, NOT missing Win32 knowledge. Refuted inside one run -- Process32FirstW recovers both arguments with no prototype while CloseHandle and Process32NextW recover none, and the difference is that Process32FirstW's RCX arrives from a MOV in a PREDECESSOR block. BUILDER: a kernel32 prototype table would satisfy the old acceptance while leaving the defect intact, which is why the acceptance was strengthened (a-e1701a898c8f) to require two non-empty arguments at Process32NextW and one at CloseHandle.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
