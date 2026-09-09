---
need_id: x86-64-syscall-wrapper
title: x86-64 syscall wrapper returns the incoming syscall number
track: quality
status: open
severity: blocker
probe_id: p-babb09aa99a6
acceptance_id: a-266d44a4acaf
hypothesis_status: upheld
credibility: 0.45
instances: 1
challenges: [652e6f896e7e520ff1c3db3c]
rounds: [7]
first_seen_round: 7
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

An instruction-level ABI override retaining the dynamic syscall number, argument registers, and returned RAX.

> **x86-64 syscall wrapper returns the incoming syscall number** (blocker, `?`)
> Even with an explicit prototype and argument-recovery options, emits syscall(); return number;. Disassembly shows RAX returned after SYSCALL. The linuxsyscall catalog explicitly limits support to i386, so this is not an int80 regression.

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
    "0x78",
    "--addr",
    "--option",
    "linuxsyscall",
    "on",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--assert",
    "prototype 0x78 long kernel_call(long number, long address, long length)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "syscall\\(\\s*\\)",
      "return\\s+number\\s*;"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.tar.gz.__x/crackme.tar.__x/crackme/crackme",
    "binary_sha256": "d7de651f041e8536fd37ec3461047a7a14a8c9d331c92ce19e505683ab0770b1",
    "binary_size": 220761,
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
    "0x78",
    "--addr",
    "--option",
    "linuxsyscall",
    "on",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--assert",
    "prototype 0x78 long kernel_call(long number, long address, long length)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "syscall\\(\\s*\\)",
      "return\\s+number\\s*;"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.tar.gz.__x/crackme.tar.__x/crackme/crackme",
    "binary_sha256": "d7de651f041e8536fd37ec3461047a7a14a8c9d331c92ce19e505683ab0770b1",
    "binary_size": 220761,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The x86-64 syscall userop likely lacks register input/output effects.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/crackme.tar.gz.__x/crackme.tar.__x/crackme/crackme` — Exited 1: server exited before registering. Reference pseudocode unavailable; superiority is unproven.

## Instances

- `?` (round ?, tester ?)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 7 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK 2026-09-07 10:2xZ by the captain (no refuter role exists; see the refute-tick memory). VERDICT: UPHELD, and the mechanism is now pinned to a single line of vendored SLEIGH -- but the FIX SHAPE THE HYPOTHESIS IMPLIES IS NOT linuxsyscall's, and a builder that copies linuxsyscall CANNOT flip this probe. Read this before designing.

REPRO on the need's own arena binary (.kuna-repipe/arena/7/652e6f896e7e520ff1c3db3c/target/crackme.tar.gz.__x/crackme.tar.__x/crackme/crackme, ELF64 PIE, static, NO SECTION HEADER):
  kuna decompile $B 0x78 --addr --option linuxsyscall on --option calleearity on --option varargstackargs on --assert 'prototype 0x78 long kernel_call(long number, long address, long length)'
  -> long sub_78(long number,long address,long length) { syscall(); return number; }
Reproduces exactly as filed. NOTE the --assert IS honored (the three parameters are named and typed as asserted), so the prototype/argument-assertion path is NOT at fault -- do not spend a builder there.

THE CAUSE, EXACTLY. specs/Ghidra/Processors/x86/data/languages/ia.sinc:4836-4838 lowers SYSCALL as:
    RCX = inst_next; R11 = rflags; syscall();
i.e. a **zero-input, zero-output CALLOTHER**. That single fact produces BOTH halves of the symptom, and the disassembly makes it mechanical:
  0x88 MOV RAX,[RBP-0x18]   ; number
  0x8c MOV RDX,[RBP-0x20]
  0x90 MOV RSI,[RBP-0x28]
  0x94 MOV RDI,RDX
  0x97 SYSCALL
  0x99 MOV [RBP-0x8],RAX
(a) NO INPUTS -> the RAX/RDI/RSI/RDX setup stores have no reader, the dead-code fixpoint collects them, and syscall() prints with empty parens. (b) NO OUTPUT -> nothing redefines RAX across 0x97, so the RAX that 0x99 stores is still the RAX that 0x88 loaded from 'number'. 'return number;' is not an invented value -- it is correct copy-propagation over an operation kuna has been told writes nothing. The tester's one-line hypothesis ('the x86-64 syscall userop likely lacks register input/output effects') is right on both counts.

WHY THE OBVIOUS PATCH IS THE WRONG ONE -- THE SYSCALL NUMBER HERE IS A RUNTIME PARAMETER. linuxsyscall (P2/flow-classification, default off) works by *resolving the number to a constant* via a backward block walk that accepts only a full-width EAX=<const>, looking the name+arity up in a vetted 332-entry table, and retargeting a CALLIND onto a synthesized sys_<name> spec. NONE of that transfers:
  - there is no CALLIND at all here. int 0x80 lowers to 'intloc = swi(0x80); call [intloc]'; SYSCALL lowers to a bare CALLOTHER with no indirect call to retarget.
  - the number is the incoming parameter 'number', not a constant, so linuxsyscall's first refusal clause fires by construction. Any name-lookup design refuses on this very binary.
  - linuxsyscall is explicitly gated to x86-32 ('any language that is not x86-32' is its third refusal), so it is inert here no matter what the tester passed -- passing '--option linuxsyscall on' in the probe changes nothing and is NOT evidence.
The fix has to be NUMBER-INDEPENDENT: give the syscall CALLOTHER the x86-64 Linux ABI effects (reads RAX,RDI,RSI,RDX,R10,R8,R9; writes RAX; RCX/R11 are already written by the SLEIGH constructor itself). That keeps the setup stores alive AND redefines RAX, which is what both acceptance clauses need.

THE PRECEDENT TO BUILD ON, not to re-invent: p2_lift/kuna_cortexmpriv.rs registers a synthesized <callotherfixup> payload for isCurrentModePrivileged through UserOpManage::manual_call_other_fixup, deliberately WITHOUT editing any vendored *.cspec so the spec tree stays byte-identical to upstream (docs/history.md sync procedure). Read its header -- it is the same problem class (a userop whose vendored model is wrong for the target we actually decompile) and it documents the wiring. Neighbours worth reading: p2_lift/userop.rs, p2_lift/pcodeinject.rs, p2_lift/inject_sleigh.rs.

ACCEPTANCE TRAP CHECK (a-266d44a4acaf), done against the fix shape above: the probe forbids /syscall\(\s*\)/ and /return\s+number\s*;/. Both clauses DO flip under the ABI-effects fix -- note the first regex requires EMPTY parens, so an emitted 'syscall(number,address,length)' passes it, and the second dies as soon as RAX is redefined. So the builder does NOT have to name the call. But a fix that only adds a WARNING, or that only names the op, flips neither. Also: this binary has no section header, so anything that reaches for the section table will not see this function at all (cf. the sectionless-ELF need family).

SCOPE unchanged (small) and severity stands: the fix is one userop effect model plus an option, and the emitted C is currently a FALSE STATEMENT about the program (it claims the return value is the syscall number), which is the worst class of wrong output.
