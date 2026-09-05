---
need_id: get-pc-helper-loses
title: Get-PC helper loses a live stat-buffer argument
track: quality
status: open
severity: major
probe_id: p-8980f5aa3c7b
acceptance_id: a-1d7268631488
hypothesis_status: upheld
credibility: 1.0
instances: 2
challenges: [5ab77f5833c5d40ad448c399, 68d9ee36224c0ec5dcedc3fc]
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

Preserve the incoming buffer argument across a helper that only writes EBX.

> **Get-PC helper loses a live stat-buffer argument** (major, `5ab77f5833c5d40ad448c399`)
> The __lxstat call uses an undefined EDX local. Disassembly shows EDX loaded from the buffer parameter before the helper and preserved through it. Explicit prototype and both argument-recovery options did not repair the output.

> **Loader return is incorrectly sourced from the security-cookie check** (major, `68d9ee36224c0ec5dcedc3fc`)
> Default output loses the return. Asserting a pointer return instead returns an invented security_cookie_check result. Disassembly shows RAX receives RBX before the helper, whose returning path preserves RAX. Also encountered already-filed runtime-decrypted-code-opaque; not refiled.

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
    "0x8049f20",
    "--addr",
    "--assert",
    "prototype sub_8049f20 int4 sub_8049f20(char *path,void *buf);",
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
      "__lxstat\\(3,path,v[0-9]+\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/collide.tgz.__x/collide.tar.__x/collide/collide",
    "binary_sha256": "2141200d97193c42c25144374eeeced095d570e6f5e88b30ff9e6d4fa4594c97",
    "binary_size": 9400,
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
    "0x8049f20",
    "--addr",
    "--assert",
    "prototype sub_8049f20 int4 sub_8049f20(char *path,void *buf);",
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
    "stdout_absent": [
      "__lxstat\\(3,path,v[0-9]+\\)"
    ],
    "stdout_matches": [
      "__lxstat\\(3,path,[^;]*buf"
    ]
  },
  "target": {
    "binary_rel": "bin/collide.tgz.__x/collide.tar.__x/collide/collide",
    "binary_sha256": "2141200d97193c42c25144374eeeced095d570e6f5e88b30ff9e6d4fa4594c97",
    "binary_size": 9400,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Ordinary ABI call-clobber assumptions obscure the helper's actual register effects.
- An ordinary caller-clobbering ABI model is applied to a register-preserving helper.

## Refutation

**UPHELD on the i386 arm by direct measurement**; the Win64 arm is argued, not measured
(captain, round 3, cf5234ac).

`collide`, `sub_8049f20`, with the filed prototype assertion:

```
0x8049f29  MOV EDX,[EBP + 0xc]     ; the `buf` parameter
0x8049f2c  CALL 0x8049f55          ; the get-PC thunk: MOV EBX,[ESP]; RET  -- writes EBX only
0x8049f3e  MOV [ESP + 0x8],EDX     ; third argument of __lxstat
0x8049f42  MOV EDX,[EBP + 0x8]     ; the `path` parameter
```

kuna emits `__lxstat(3,path,v1)` with `unsigned int v1; // edx`. The controlled comparison is
inside the one function: `path` is loaded from the frame AFTER the call and is recovered;
`buf` is loaded from the frame BEFORE it and is lost. The only difference between the two is
that EDX crosses the CALL, and EDX is in the i386 cdecl killedbycall set -- while the callee
provably writes EBX and nothing else. So an ABI clobber model applied to a register-preserving
helper is the mechanism, as filed.

Two things a builder must carry. (1) No option covers this: nothing in the 149-row catalog
narrows a call's killed set from the callee's actual writes, and there is no get-PC-thunk
recognizer (the image is stripped, so `__x86.get_pc_thunk.bx` is not a name here). (2) The
general form -- "trust the decoded callee's register writes over the cspec" -- is only sound
for a fully decoded, non-recursive, call-free callee; anything looser produces wrong output on
an unresolved or indirect callee.

ACCEPTANCE STRENGTHENED: the filed acceptance only required `__lxstat(3,path,v<N>)` to
disappear, which a fabricated or constant third argument also satisfies. It now requires `buf`
to appear as that argument (tolerant of a cast).

## Reference

_none recorded_

## Instances

- `5ab77f5833c5d40ad448c399` (round 3, tester t-r3-5ab77f58)
- `68d9ee36224c0ec5dcedc3fc` (round 3, tester t-r3-68d9ee36)

## Decision log

- filed by cluster.py from 2 observation(s)
captain T_DEDUP r3: SPLIT out of a 5-observation cluster. Kept with obs14 (Win64 security-cookie helper) because both hypotheses are the same mechanism -- a caller-clobbering ABI model applied to a helper that demonstrably preserves the register (EDX across the i386 get-PC thunk; RAX across the cookie check). Two testers, two challenges, two architectures.
captain T_REFUTE r3: hypothesis upheld -- see ## Refutation (measured on cf5234ac with the release binary).
captain T_TRIAGE r3: track quality CONFIRMED and touches narrowed to p4_calls; scope small. Upheld by measurement at T_REFUTE. This one DOES need an option: 'trust the decoded callee's register writes over the cspec killedbycall set' is a judgement call and is only sound for a fully decoded, non-recursive, call-free callee, so it ships gated per docs/agents.md.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
