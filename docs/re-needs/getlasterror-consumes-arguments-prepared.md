---
need_id: getlasterror-consumes-arguments-prepared
title: GetLastError consumes arguments prepared for FormatMessageA
track: quality
status: open
severity: major
probe_id: p-17d6a2545f66
acceptance_id: a-40f8d23cc7c1
hypothesis_status: inconclusive
credibility: 0.85
instances: 2
challenges: [5ab77f5d33c5d40ad448c6f6]
rounds: [10]
first_seen_round: 10
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

Recover the error-reporting function with the actual Win32 call arguments.

> **GetLastError consumes arguments prepared for FormatMessageA** (major, `5ab77f5d33c5d40ad448c6f6`)
> Emits FormatMessageA(0x1300,0,GetLastError(0,&v1,0,0)). The four pushes preceding GetLastError belong to the subsequent seven-argument FormatMessageA. Enabling calleearity and varargstackargs leaves this unchanged. Kuna disassembly confirms the pushes and call order.

> **Accepted stdcall prototypes still lose pre-call stack argument values** (major, `5ab77f5d33c5d40ad448c6f6`)
> Both assertions are accepted, but FormatMessageA(0x1300,NULL,GetLastError(),a0,v1,a1,a2) uses incoming arguments for pushed zeros and an uninitialized pointer for the local address. MessageBoxA and LocalFree also receive a0 instead of the local output pointer. Reliable mode and both argument-recovery options do not repair this.

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
    "0x401000",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "FormatMessageA\\((?:[^,()]*(?:\\([^()]*\\))?[^,()]*,){6}"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/calleeprotostack_pe_i386.exe",
    "binary_sha256": "a1734e48030ee5035a5f6d57cf044e8e894422bec392246d865c72ea60889d2e",
    "binary_size": 8704,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/calleeprotostack_pe_i386.exe",
    "selector": "0x401000",
    "selector_kind": "addr"
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
    "0x401000",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "GetLastError\\([^)]*[^\\s()]\\)",
      "FormatMessageA\\((?:[^()]|\\([^()]*\\))*\\ba[0-9]\\b"
    ],
    "stdout_matches": [
      "GetLastError\\(\\)",
      "FormatMessageA\\((?:[^,()]*(?:\\([^()]*\\))?[^,()]*,){6}",
      "FormatMessageA\\((?:[^()]|\\([^()]*\\))*&"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/calleeprotostack_pe_i386.exe",
    "binary_sha256": "a1734e48030ee5035a5f6d57cf044e8e894422bec392246d865c72ea60889d2e",
    "binary_size": 8704,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/calleeprotostack_pe_i386.exe",
    "selector": "0x401000",
    "selector_kind": "addr"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/Imagination.zip.__x/Imagination.exe` — Reference unavailable: server exited with status 1 before registering. No IDA pseudocode was obtained.
- `ida-decompile load target/Imagination.zip.__x/Imagination.exe` — Reference unavailable: server exited with status 1 before registering. Kuna's instruction listing explicitly shows the pushed zeros and local address.

## Instances

- `5ab77f5d33c5d40ad448c6f6` (round 10, tester t-r10-5ab77f5d)
- `5ab77f5d33c5d40ad448c6f6` (round 10, tester t-r10-5ab77f5d)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 10 REFUTER: hypothesis **inconclusive**. REFUTER RAN (captain, tick 10:05Z); verdict is inconclusive ONLY because the need was filed with '_none offered_' -- there was no hypothesis to uphold or overturn. The refuter did decide the cause; it is recorded here. GROUND TRUTH, from kuna disassemble .kuna-repipe/arena/10/5ab77f5d33c5d40ad448c6f6/target/Imagination.zip.__x/Imagination.exe 0x401820: PUSH ECX reserves the LPSTR out-slot; PUSH 0 / LEA EAX,[ESP+4] / PUSH 0 / PUSH EAX / PUSH 0 are FormatMessageA args 7,6,5,4; CALL [0x405058] is GetLastError with ZERO args; PUSH EAX / PUSH 0 / PUSH 0x1300 are args 3,2,1; CALL [0x405034] is FormatMessageA. Correct output is FormatMessageA(0x1300,NULL,GetLastError(),0,&buf,0,NULL). CAUSE OF LAYER 1 (the acceptance clause): the two Win32 IAT imports carry no prototype, so arity is inferred from surviving call-site state and the four pushes staged for FormatMessageA are vacuumed into the nearest preceding call -- same family root as sha-finalization-receives-two. *** LAYER 2 IS THE PART THAT MATTERS AND THE ACCEPTANCE DOES NOT COVER IT. *** Asserting both prototypes DOES flip this need's acceptance -- output becomes FormatMessageA(0x1300,NULL,GetLastError(),a0,v1,a1,a2), GetLastError() argumentless, FormatMessageA present -- yet FOUR of the seven arguments are still WRONG: the three PUSH 0 slots become invented incoming parameters a0/a1/a2 (sub_401820 gains three parameters it does not have) and the &buf argument becomes an uninitialized local v1. Worse, the assertion REGRESSES the aliasing: default output is at least self-consistent (v1 = a0; FormatMessageA(...,&v1,...); MessageBoxA(0,v1,...)), while the asserted run reads the same slot as v1 in one call and a0 in the next. SO: a builder who ships only an import type-library CLOSES THIS NEED WITH THE WRONG-OUTPUT DEFECT STILL PRESENT. Recommend at T_TRIAGE either splitting layer 2 into its own need or tightening this acceptance with stdout_absent on the invented-parameter form; do not let it close on the GetLastError clause alone.
- round 10 T_TRIAGE (captain, 10:39Z tick): ACCEPTANCE WIDENED, a-355aba3b8d3f -> a-20de0ce0d19c. The old positive clause was the bare literal 'FormatMessageA\\(', which the unpatched tree ALREADY matches -- so the whole acceptance rested on the single negative clause, and a fix that gave GetLastError arity 0 while leaving FormatMessageA with three arguments scored green. That is only half the filed evidence: the second tester quote is that FormatMessageA's own arguments are wrong. THE GROUND TRUTH, read off the listing at 0x401820..0x40183a: PUSH 0 / LEA EAX,[ESP+4] / PUSH 0 / PUSH EAX / PUSH 0 / CALL GetLastError / PUSH EAX / PUSH 0 / PUSH 0x1300 / CALL FormatMessageA -- i.e. FormatMessageA(0x1300, 0, GetLastError(), 0, &v1, 0, 0), seven arguments, and GetLastError takes none. Today kuna emits 'FormatMessageA(0x1300,0,GetLastError(0,&v1,0,0));'. The two new clauses are 'GetLastError\\(\\)' (arity 0, positively) and a top-level six-comma count on FormatMessageA that tolerates one nesting level. DELIBERATELY SPELLING-INDEPENDENT: it matches whether the zeros print as 0 or NULL and with or without spaces after the commas, because a type-inference improvement changing 0 to NULL must not false-regress this need. Verified by construction against five renderings -- the current wrong line does NOT match, the corrected line matches in three spellings, and the half-fixed 'FormatMessageA(0x1300,0,GetLastError());' does NOT match, which is the case the old acceptance could not see.
- round 10 REFUTER: hypothesis **inconclusive**. CAPTAIN B_PLAN BRIEF (18:2xZ tick), all of it MEASURED on merged main b23e01ec -- read this before re-deriving anything. (1) THE FILED SYMPTOM IS STALE. win32sigs (PR #520, DIV-141) landed AFTER this need was filed and already fixed layer 1. Main now emits: FormatMessageA(0x1300,0,GetLastError()); -- GetLastError() is argumentless and the argument-eating form is GONE. The live defect is different: the four pushes staged for FormatMessageA across the intervening CALL are now DROPPED, so the call gets 3 of its 7 arguments. Do NOT re-ship an import type-library as the fix. (2) BOTH PROBE ARMS WERE UNRUNNABLE AND ARE NOW REPAIRED. They carried no target block at all, so verify --acceptance-suite returned 'ProbeError: {{BIN}} used but the context supplies no bin' and the need could never have closed. Bound to challenge 5ab77f5d33c5d40ad448c6f6, binary_rel bin/Imagination.zip.__x/Imagination.exe, sha256 78d28ec6d13b9c16..., 40960 bytes, source dataset (byte-identical to the arena copy, checked). The Reproduction arm's old clause described the pre-#520 symptom and FAILED on main -- retargeted to the live defect (FormatMessageA does not have 7 args), so it passes today. ids moved: p-eb78ee4e719b -> p-17d6a2545f66, a-20de0ce0d19c -> a-40f8d23cc7c1. (3) ABLATIONS ALREADY RUN -- DO NOT REPEAT THEM. --option stackarggap off, --option calleearity on, --option varargstackargs on: all three leave the output BYTE-IDENTICAL. stackarggap (DIV-140, the sibling fix from #518) is NOT what truncates this call, and no existing flag reaches this. (4) THE TRAP THAT WOULD HAVE LET YOU CLOSE THIS NEED ON WRONG OUTPUT, AND WHY THE ACCEPTANCE IS NOW TIGHTER. FormatMessageA is NOT in kuna_win32sigs.rs (MessageBoxA is), so adding a table row is the obvious first move -- but it is NOT sufficient. Measured with --assert 'prototype FormatMessageA int FormatMessageA(int,void*,int,int,char*,int,void*)': output becomes 'void sub_401820(char *a0,int a1,void *a2)' with FormatMessageA(0x1300,NULL,GetLastError(),(int)a0,v1,a1,a2) -- seven arguments, so the OLD comma-count acceptance went GREEN, while FOUR of the seven values are wrong and the function grew three parameters it does not have. I added two clauses, each verified by construction to reject that exact string and to accept the correct call in three spellings (&v1 / spaced &buf / &local_8): stdout_absent on a bare a[0-9] inside FormatMessageA's argument list, and stdout_matches requiring an address-of among its arguments. Getting arity right is not the bar; the VALUES are. (5) GROUND TRUTH, from the listing at 0x401820: PUSH ECX reserves the LPSTR out-slot; PUSH 0 / LEA EAX,[ESP+4] / PUSH 0 / PUSH EAX / PUSH 0 are args 7,6,5,4; CALL [0x405058] is GetLastError (zero args); PUSH EAX / PUSH 0 / PUSH 0x1300 are args 3,2,1; CALL [0x405034] is FormatMessageA. Correct output: FormatMessageA(0x1300,NULL,GetLastError(),0,&buf,0,NULL), with the buffer a LOCAL whose address is taken -- not an incoming parameter. (6) PROMOTION WILL BE REFUSED UNLESS YOU VENDOR A FIXTURE. binary_source is 'dataset' and verify --promote rejects that verbatim ('CI has no dataset, so vendor the binary into the repo first'); --force does not help. Ship an in-repo fixture (binary_source in-repo + in_repo_path) in the SAME PR or B_DONE cannot promote this probe into tests/cli/ and the need ends the round half-closed, as windows-api-calls-acquire just did.
