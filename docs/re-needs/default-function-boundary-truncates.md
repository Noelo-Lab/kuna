---
need_id: default-function-boundary-truncates
title: Default function boundary truncates the shared syscall return
track: quality
status: closed
severity: major
probe_id: p-3f3a4aa63b91
acceptance_id: a-9d9a83349539
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5c33c5d40ad448c615]
rounds: [10, 12]
first_seen_round: 10
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: "589"
closed_in_round: 12
closing_pr: "589"
reject_reason: null
---

## Symptom

Recover the fork wrapper return value and conditional errno handling.

> **Default function boundary truncates the shared syscall return** (major, `5ab77f5c33c5d40ad448c615`)
> Emits a void function with an empty success conditional and unconditional errno assignment. Disassembly shows a shared RET; disabling funcboundflow restores conditional returns.

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
    "sub_80488e0",
    "--option",
    "linuxsyscall",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "if\\s*\\([^\\n]*\\)\\s*\\{\\s*\\}",
      "funcboundflow:"
    ]
  },
  "target": {
    "binary_rel": "bin/trace-p.zip.__x/crackme",
    "binary_sha256": "7b8a32c3cc0e9816bfe03d2e7c4aa5a596a21e43a6e35dc43cb17fa5867138ec",
    "binary_size": 5208,
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
    "shared_wrapper"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "funcboundflow:",
      "return\\s+0x21;"
    ],
    "stdout_matches": [
      "return\\s+7;",
      "return\\s+0xffffffff;"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/function_boundary_return_x86_64",
    "binary_sha256": "6aac76982527b9e550ef1a70bbe0ab8dd7d59fc67c42cf21a33991e03e40ff01",
    "binary_size": 5080,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/function_boundary_return_x86_64",
    "selector": "shared_wrapper",
    "selector_kind": "name"
  },
  "notes": "Synthetic x86-64 twin: shared_wrapper falls through to callable_ret, a genuine STT_FUNC RET which ret_caller also calls. Before, funcboundflow replaced the RET with a no-return halt and lost both returns. The absent 0x21 clause proves flow stops at the admitted RET. The i386 dataset image remains the exact witness."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- A separately callable RET is treated as a hard boundary within a shared epilogue.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5c33c5d40ad448c615` (round 10, tester t-r10-5ab77f5c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER: hypothesis **upheld** (was inconclusive). REFUTER RAN 2026-09-08 10:20Z (captain tick, pre-#511 release build; #511 is inert here — this ELF takes the section-driven path). VERDICT UPHELD, and unusually precisely: the filed cause 'a separately callable RET is treated as a hard boundary within a shared epilogue' is literally the mechanism, address for address.

THE SHAPE. sub_80488e0 is a 7-byte dietlibc-style syscall stub: 'MOV AL,0x2; JMP 0x80488a7'. 0x80488a7 is the SHARED trampoline inside sub_80488a5, which does CALL [0x804a384], 'CMP EAX,-0x7c; JC 0x80488d7' (success), the errno tail (NEG/MOV EBX/CALL 0x8048960/MOV [EAX],EBX), 'OR EAX,0xffffffff' at 0x80488d4, and both paths converge on the epilogue 'POP EBX; POP ESI; POP EDI' at 0x80488d7..0x80488d9 terminated by RET at 0x80488da. *** 0x80488da IS A DISCOVERED FUNCTION ENTRY (sub_80488da), so funcboundflow cuts the epilogue's own RET off. *** That is the whole defect: the success branch loses its body and the function loses its return.

CONTROL: --option funcboundflow off => correct output, 'unsigned int sub_80488e0(void)' with 'if (v1 <= 0xffffff83) return v1;' and 'return 0xffffffff;'. Default => 'void', an EMPTY 'if (v1 <= 0xffffff83) { }', and the funcboundflow warn.

*** THE WRONG-PASS TRAP IS CLOSED: DO NOT 'FIX' DISCOVERY. *** The obvious counter-hypothesis is that 0x80488da is a spurious entry invented mid-epilogue. It is not. 'kuna xrefs BIN --to 0x80488da' returns count 1: a real 'CALL 0x80488da' at 0x804893e. sub_80488da is a genuinely, separately callable no-op RET. Suppressing that entry would delete a real function and break that call site. The entry is right; funcboundflow's use of it as a hard cut is wrong.

THE SAFE FIX, and why the obvious one is a mass regression. Simply relaxing 'fall-through reached the next function entry' is exactly what funcboundflow exists to prevent — every fall-through function would swallow its neighbour, corpus-wide. The narrow rule that is safe here: *** if the instruction AT the next-function entry is itself a flow terminator (RET), admitting it cannot swallow the neighbour, because flow ends on that one instruction. *** Including 0x80488da's RET gives the shared epilogue its return and stops. Keep funcboundflow default-ON; this is a strict wrong-output fix, no new flag.

*** IT DOES NOT SHARE A ROOT WITH success-report-callback-fails — I TESTED THE LAST TICK'S GUESS AND IT IS WRONG. DO NOT HOLD THEM ON ONE BUILDER. *** The 10:05Z note proposed a shared cluster: lease because both are funcboundflow on this same binary. The declared-extent control separates them. For success-report-callback-fails, declaring funcboundflow's own cut EXITS 0 with full C (opposite outcome, same address). Here, 'kuna decompile BIN 0x80488e0 --addr --define-function 0x80488e0-0x8048900' gives a THIRD, different wrong output: 'void sub_80488e0(void) // warn: Function flows out of bounds' with an empty body and 'Function flow out of bounds: r0x080488e2 flows to r0x080488a7' — the declared path REFUSES the outbound JMP, while funcboundflow FOLLOWS it and then truncates at the entry. Different mechanism, different fix, different code path. They share only the file (flow.rs); B_PLAN's disjoint-lease rule already keeps them apart since both carry touches=[decompiler/crates/kuna-decomp].
- round 10 CAPTAIN (18:35Z tick, B_DRAIN off-critical-path repair): PROBE TARGET BOUND -- this need was STRUCTURALLY UNCLOSABLE as filed. Both arms carried no `target` block, so `verify --acceptance-suite` returned `ProbeError: {{BIN}} used but the context supplies no bin` with unrunnable=true / transition=indeterminate: a builder could have done the work and still not closed it. Bound both arms to the single ELF/PE candidate under ds/challenges/<hexid>/ (sha256 + size captured from the dataset copy, binary_source=dataset). Verified post-fix: unrunnable=false, passed=false, transition=unchanged -- i.e. the acceptance now FAILS honestly and can be seen to flip. NOTE probe ids did NOT move: probe_id_of() is keyed on cmd+expect only (needs.py:269), so binding a target is id-stable and breaks no historical reference. PROMOTION CAVEAT: binary_source is `dataset` and `verify --promote` refuses that verbatim (CI has no dataset, --force does not help) -- vendor an in-repo fixture in the SAME PR or B_DONE cannot promote this probe into tests/cli/.
- builder attempt 1 (2026-09-12): commit `b6f20b414a3f85abe5946e2fb89b4c6460713117` vendors the byte-reproducible x86-64 shared-RET fixture and promotes its robust acceptance as `a-9d9a83349539`. The original dataset reproduction remains `p-3f3a4aa63b91`; only the acceptance moved because CI cannot replay dataset binaries. Status remains open with `pr: null` pending semantic review, merge, and acceptance application on the landed commit.
- semantic review (2026-09-12): SAFE on `b6f20b414a3f85abe5946e2fb89b4c6460713117`, with no findings. The reviewer independently validated the synthetic controls and the conservative return predicate. The need remains open pending merge and acceptance application.
- closed: acceptance a-9d9a83349539 now PASSES at 3900ab8b0deb
- round 12 landing acceptance (2026-09-12): canonical acceptance `a-9d9a83349539` passed at the exact pre-closure head `3900ab8b0deb0b7444b0ce7b620302c5e1820f7d`. The dataset witness remained sha256 `7b8a32...` / 5208 bytes and recovered both the unsigned `return v1` and `return 0xffffffff` with exactly one xref at `0x804893e`; the synthetic fixture remained sha256 `6aac769...` / 5080 bytes and rebuilt byte-identically. Validation passed 675 default parity tests, 794 stage parity tests, 140 CLI tests, 5 focused function-boundary tests, and 4 release CLI integration tests; both exact-head semantic reviewers reported SAFE. This closed metadata is carried on PR #589 and becomes durable only when that PR merges; the PR remains unmerged here.
- round 12 merge guard review (2026-09-12): mergecheck against the exact `origin/main` base returned exactly two shape-C line-removed rejects, solely the intentional replacement of the authoritative `phases.toml` funcboundflow summary and its matching generated `docs/options.md` mirror. The catalog, phase fixture, options freshness, and counters all rederived successfully, with no row, counter, or baseline duplicated or lost; both exact-head reviewers inspected and accepted these two findings. This closure remains conditional on PR #589 merging and is not durable while that PR remains unmerged.
