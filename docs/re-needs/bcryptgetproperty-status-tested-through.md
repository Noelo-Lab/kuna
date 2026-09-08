---
need_id: bcryptgetproperty-status-tested-through
title: BCryptGetProperty status is tested through an unassigned local
track: quality
status: open
severity: major
probe_id: p-9def21a83111
acceptance_id: a-7c5a8a2de1dd
hypothesis_status: upheld
credibility: 0.85
instances: 2
challenges: [6a3d73bfa4b247348ae8069e]
rounds: [7]
first_seen_round: 7
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 503
closed_in_round: null
closing_pr: 503
reject_reason: null
---

## Symptom

Follow the success/failure branch after the property query.

> **BCryptGetProperty status is tested through an unassigned local** (major, `6a3d73bfa4b247348ae8069e`)
> Emitted a standalone BCryptGetProperty call followed by a test of an unassigned eax local. Assembly at 0x140004766 calls the API and immediately tests EAX. An explicit int-returning prototype restores the assignment.

> **BCryptDecrypt ciphertext pointer becomes literal zero** (major, `6a3d73bfa4b247348ae8069e`)
> Emitted literal 0 as BCryptDecrypt's second argument, including with calleearity and varargstackargs enabled. An explicit API prototype restores the buffer variable. Assembly passes the buffer through RDX at the call at 0x140004b3c.

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
    "sub_140003f80"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^\\s*BCryptGetProperty\\("
    ]
  },
  "target": {
    "binary_rel": "bin/selamv2.exe",
    "binary_sha256": "1143e14d87e208c44bee97b0d5b1b2a816a4c5118e90670c170fa5f5a45eff11",
    "binary_size": 248320,
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
    "__printf_fp_l"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "= hack_digit\\("
    ],
    "stdout_absent": [
      "(?m)^\\s*hack_digit\\("
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/mcount_x86_64",
    "binary_sha256": "913d35f45475cc129d2fd6a94e35e62bd1b6be720a7207b74852705850611ee7",
    "binary_size": 896032,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/mcount_x86_64",
    "selector": "__printf_fp_l",
    "selector_kind": "name"
  },
  "notes": "Retargeted from the dataset witness (selamv2.exe sub_140003f80, BCryptGetProperty) to an in-repo fixture with the same defect: __printf_fp_l calls hack_digit three times and loses the result of exactly one of them. CI has no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Default imported signature or return recovery appears incomplete.
- An incomplete imported signature appears to interact with argument recovery.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/selamv2.exe` — Server exited with status 1 before registering. No reference pseudocode obtained.
- `ida-decompile load ./target/selamv2.exe` — Server exited with status 1 before registering. No reference pseudocode obtained.

## Instances

- `6a3d73bfa4b247348ae8069e` (round 7, tester t-r7-6a3d73bf)
- `6a3d73bfa4b247348ae8069e` (round 7, tester t-r7-6a3d73bf)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 7 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, r7 T_REFUTE, sha dfc7acc5, arena binary selamv2.exe / sub_140003f80). Hypothesis UPHELD and now measured: with no declared signature for the import, kuna's default recovery fails in BOTH directions, and one declared prototype fixes each half. THE DEDUP RESIDUE IS RESOLVED -- KEEP THE TWO OBSERVATIONS MERGED. They are one root cause, proved by A/B: '--assert prototype 0x1400282b8 int BCryptGetProperty(void*,unsigned short*,unsigned char*,unsigned int,unsigned int*,unsigned int)' turns the bare 'BCryptGetProperty(...);' statement into 'v15 = BCryptGetProperty(...)' AND the branch into 'if ((v15 < 0) || ...)'; '--assert prototype 0x1400282b0 int BCryptDecrypt(...10 args...)' turns 'BCryptDecrypt(v51,0,...)' into 'BCryptDecrypt(v50,v30,...)' -- the literal 0 becomes the buffer. Same lever, same binary, two visible symptoms. THE VOID-RETURN CONTROL IS THE SHARPEST RESULT. Declaring the args but 'void' return leaves the arguments correct and STILL emits 'if ((v16 < 0) || ...)' reading an undefined local: kuna prints a read of a variable it never assigned. So the undefined-local read is caused by the missing RETURN specifically, not by arity, and emitting an undefined read at all (rather than dropping or annotating it) is arguably the deeper defect a fix should address. THE DEFECT IS PER-CALL-SITE, NOT A BLANKET 'IMPORTS RETURN VOID' -- this is what a builder must explain. In the SAME default run, other bcrypt imports DO recover a return: 'v15 = BCryptSetProperty(...)', 'v15 = BCryptGenerateSymmetricKey(...)', 'if (0 <= (int)BCryptOpenAlgorithmProvider(...))'. Only GetProperty loses it. The call sites are indirect through the IAT ('CALL qword ptr [0x1400282b8]' at 0x140004766, resolved to the name), and GetProperty's EAX is consumed by 'TEST EAX,EAX; JS' -- flags only, no store -- whereas the recovered ones flow into a value. So look at return-trial scoring for a flags-only consumer of RAX at an unlocked callee, not at the import loader. Default arity is also over-guessed there (6 args, the last being garbage 'v70 & 0xffffffff00000000'), and RDX is not written at that call site at all. RELATED, MEASURED LAST TICK: the same 'no Win32 signature source exists in the tree' gap is clause 2 of resource-loader-c-retains; a signature database would close both but is a PROPOSAL-sized feature. This need is the cheaper half -- fix the trial, not the database. Prototype assertions ARE read by IAT-slot address here, so the parked-by-name hazard is not in play. ACCEPTANCE TRAP -- IT COVERS ONLY ONE OF THE TWO OBSERVATIONS. Acceptance = stdout contains 'BCryptGetProperty' and does NOT match '(?m)^\\s*BCryptGetProperty\\('; it says nothing about BCryptDecrypt's literal-zero argument, so a return-only fix closes the need with half the filed evidence still broken. B_DONE must additionally run: kuna decompile <bin> sub_140003f80 | grep BCryptDecrypt, and require the second argument is NOT '0'. Also note a fix that assigns the return to a dead/unused variable flips the acceptance regex without making the branch read the call's value -- require 'v?? < 0' to test the SAME variable the call assigns.
- round 9 BUILDER: root cause is NOT the import loader and not the return-trial scoring. `Funcdata::opInsertAfter` (funcdata_op.cc:373) carried a `STUB(W3-varnode)` where upstream REDIRECTS an insertion after an INDIRECT marker to the CALL that marker speaks for. `RulePullsubIndirect` pulls a SUBPIECE through the guard INDIRECT of the frame slot at -0x2d0 and inserts it after that INDIRECT, so it landed INSIDE the call's guard run; `FuncCallSpecs::collectOutputTrialVarnodes` (fspec.cc:5543) walks back from the CALL and stops at the first non-INDIRECT, so it stopped at the SUBPIECE 18 ops short of the EAX INDIRECT creation, every output trial was marked no-use, and the CALL got no output. Closed by option `indirectanchor` (default on), which completes the redirect. Measured off/on: `BCryptGetProperty(v48,v27,&v50,4,v49,v70 & 0xffffffff00000000);` + `if ((v16 < 0) || ...)` becomes `v15 = BCryptGetProperty(v46,v26,&v48,4,v47,v68);` + `if ((v15 < 0) || ...)` -- the branch tests the SAME variable the call assigns, as the refuter required.
- round 9 BUILDER: acceptance RETARGETED from the dataset witness to an in-repo fixture so it can run in CI (`decompiler/crates/kuna-analysis/tests/fixtures/mcount_x86_64`, `__printf_fp_l`, which calls `hack_digit` three times and loses exactly one result). Same assertion shape, same defect. `acceptance_id` relabelled `a-b30578566846` -> `a-7c5a8a2de1dd`. The dataset witness still passes the original acceptance; both were measured on the same build.
- round 9 BUILDER: the SECOND filed observation is NOT closed and is a DIFFERENT defect. `BCryptDecrypt`'s second argument is still literal `0` after the fix (`v15 = BCryptDecrypt(v49,0,...)`), and no existing option moves it (`calloverlap in|full`, `calleedeadarg`, `calleearity`, `peimportcall`, `framelayout`, `calleepreserves`, `spillargtrial`, `retinputhalf` all measured, all unchanged). That half is INPUT-trial recovery for a register whose 4-byte sub-range (`EDX`, read by `SUB R8D,EDX` at 0x140004b10) is heritaged separately from the 8-byte `RDX` the caller wrote at 0x140004a7e; the return-trial walk is not on that path. It wants its own need.
