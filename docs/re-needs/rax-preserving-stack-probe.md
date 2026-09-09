---
need_id: rax-preserving-stack-probe
title: RAX-preserving stack probe leaves an undefined stack base
track: quality
status: open
severity: blocker
probe_id: p-5ac15314ce59
acceptance_id: a-ccd85db0e5f2
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [69d7f15a8afd9d6c48b48871]
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

Recover the constant 0x5e028 stack allocation, or assert the helper's register-preservation effects.

> **RAX-preserving stack probe leaves an undefined stack base** (blocker, `69d7f15a8afd9d6c48b48871`)
> The caller loads EAX=0x5e028 and calls a helper whose disassembly never writes RAX. Kuna loses the constant and indexes Stack objects by an undefined local. Naming the helper __chkstk did not help. An accepted void prototype with calleepreserves explicitly enabled also failed. Separately encountered already-filed strings-inventory-omits-statically for constructed access messages.

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
    "0x140001000",
    "--addr",
    "--define-function",
    "0x140001000-0x140001043",
    "--assert",
    "prototype 0x1408d0000 void probe(void)",
    "--option",
    "calleepreserves",
    "on",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "Stack[0-9a-fA-F]+\\)\\[v[0-9]+\\]"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "36bfae11c18fb5fa214110d7f17cdc92026bce53e28e9a1965b4193e59c1a6a1",
    "binary_size": 9384960,
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
    "0x140001000",
    "--addr",
    "--define-function",
    "0x140001000-0x140001043",
    "--assert",
    "prototype 0x1408d0000 void probe(void)",
    "--option",
    "calleepreserves",
    "on",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].error",
        "op": "eq",
        "value": null
      },
      {
        "path": "functions[0].line_mappings",
        "op": "len_gt",
        "value": 7
      }
    ],
    "stdout_absent": [
      "Stack[0-9a-fA-F]+\\)\\[v[0-9]+\\]"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "36bfae11c18fb5fa214110d7f17cdc92026bce53e28e9a1965b4193e59c1a6a1",
    "binary_size": 9384960,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Callee preservation or stack-effect recovery loses the incoming RAX across this Windows stack-probing helper.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/crackme_shroud.exe` — Server exited with status 1 before registering. The preceding decompile request reported no running server. Reference capability remains unknown.

## Instances

- `69d7f15a8afd9d6c48b48871` (round 5, tester t-r5-69d7f15a)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, round 5). VERDICT: UPHELD, and the mechanism is now pinned to one line of an ALREADY-SHIPPED pass -- the builder should not re-diagnose.

EVIDENCE 1 -- the premise is true. `kuna disassemble crackme_shroud.exe --addr 0x1408d0000 --count 14` is textbook MSVC __chkstk: SUB RSP,0x10 / MOV [RSP],R10 / MOV [RSP+8],R11 / XOR R11,R11 / LEA R10,[RSP+0x18] / SUB R10,RAX / CMOVC R10,R11 / MOV R11,qword ptr GS:[0x10] / CMP / JNC ... It READS RAX and never writes it, so RAX genuinely survives the call. The caller is the canonical large-frame prologue: MOV EAX,0x5e028 ; CALL 0x1408d0000 ; SUB RSP,RAX.

EVIDENCE 2 -- the emitted body, reproduced without --json:
  void sub_140001000(unsigned int a0,unsigned long long a1)
  { int8 v2; // rax
    sub_1408d0000();
    v1 = -v2;
    *(unsigned long long *)&(&Stack000000000005dfd0)[v1] = v7; ...
v2 is declared with rax storage and is NEVER ASSIGNED. The constant 0x5e028 does not cross the call, `SUB RSP,RAX` becomes `v1 = -v2`, and every local is then indexed off an undefined frame base. That is the filed symptom exactly.

EVIDENCE 3 -- WHY calleepreserves DID NOT SAVE IT, AND WHY THE TESTER COULD NOT TELL. phases.toml:2486 has calleepreserves default = "on", so the probe`s `--option calleepreserves on` was a NO-OP -- it was already on and already failing. (Third vacuous-option report this round; assume "I tried option X" means nothing until the default is checked.) The pass DID run and DID decline, and its own summary says when it declines: the callee-body write walk "declares itself incomplete -- proving nothing -- at a nested CALL/CALLIND/CALLOTHER, an unresolved BRANCHIND, an undecodable instruction, or its instruction budget." The helper hits one of those at 0x1408d001c: `MOV R11,qword ptr GS:[0x10]` is a segment-override access, which x86-64 SLEIGH lowers to a CALLOTHER (segment userop). One pseudo-op eight instructions in kills the whole proof, killedbycall stands for RAX, an INDIRECT is planted at the call, and the incoming constant dies.

WOULD A FIX ON THIS DIAGNOSIS PRODUCE WRONG OUTPUT? No, if it stays one-sided the way the existing pass already is. The narrow fix is to stop treating EVERY CALLOTHER as fatal to the walk: a CALLOTHER whose output varnode is not the register range being asked about (and which writes no other register) proves nothing about RAX and should not abort the summary. That keeps every existing guard -- register ranges only, downgrade-only never promote, PLT stubs still BRANCHIND, proto-with-effect-override still untouched -- so it can only ever KEEP a value the disassembly already shows crossing the call. Demand that the builder keep the walk conservative for a CALLOTHER with an unknown/whole-register output; the unsound version is one that assumes any userop is harmless.

WHY THIS IS WORTH A BUILDER: this is the standard __chkstk prologue emitted by MSVC for every frame over a page, so the defect fires on essentially every large-frame Windows binary, and it destroys the ENTIRE frame (all locals become Stack...[undefined]) rather than one expression. Precedent + template: repipe-get-pc-helper-loses / commit c92dddbb, same pass, same seam.

TRAPS FOR THE BUILDER: (1) the acceptance also requires functions[0].error == null and a non-empty body, not just the absence of the Stack...[v] pattern. (2) the caller writes EAX (32-bit) and the callee reads RAX -- the zero-extend must survive whatever narrowing is added. (3) the probe`s `--assert "prototype 0x1408d0000 void probe(void)"` DOES bind (the call renders as `sub_1408d0000();` with no args); the prototype is not the problem, the killed set is.
round 5 TRIAGE (captain): CONFIRMED AS FILED -- quality / kuna-decomp / small / blocker, hypothesis upheld. This is the best-specified need in the round and the first quality need to dispatch once the option-shipping counter leases free.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
- round 12 captain: ACCEPTANCE TIGHTENED before dispatch (a-3e9d8a0a72f3 -> a-ccd85db0e5f2). The old contract's only positive clauses were `functions[0].error == null` and `functions[0].code matches ".+"`, so an empty `void sub_140001000(void){}` body satisfied it while still losing the 0x5e028 stack allocation the tester asked for. Replaced the vacuous code clause with `functions[0].line_mappings len_gt 7`, measured on today's build (7ccd3175): the current body carries 11 mapping entries over 8 distinct addresses (0x140001011..0x140001053), one per register-save store, and any correct fix keeps those 8 stores while folding their index to a constant. A degraded/empty body collapses the mapping list and now FAILS. The defect clause is unchanged and still the only failing one today (8 `Stack...[v1]` hits).
