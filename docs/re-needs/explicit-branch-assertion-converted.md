---
need_id: explicit-branch-assertion-converted
title: Explicit branch assertion is converted back into a tail call
track: quality
status: closed
severity: major
probe_id: p-bc1cfbde3d40
acceptance_id: a-17e6917f0e3f
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5f33c5d40ad448c834]
rounds: [9, 12]
first_seen_round: 9
attempts: 1
covered_by_option: tailcalljump
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/622
closed_in_round: 12
closing_pr: "622"
reject_reason: null
---

## Symptom

Treat flow 0x40d126 branch as an instruction to follow the fragment within this function.

> **Explicit branch assertion is converted back into a tail call** (major, `5ab77f5f33c5d40ad448c834`)
> The accepted assertion produces only a tail-call stub, hiding the entry body. Disabling tailcalljump and tailcallframe alongside the assertions exposes the body; this is an interaction between existing controls, not a missing flow interface.

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
    "0x40d120",
    "--addr",
    "--assert",
    "flow 0x40d126 branch"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "// tail-call"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme1.zip.__x/crkme.exe",
    "binary_sha256": "9031b8200481747e7ddb5cd5fc2a74030cd04052770bfa23d857a0662a58bc06",
    "binary_size": 266240,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/explicit_branch_assertion_pe_i386.exe",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/explicit_branch_assertion_pe_i386.exe",
    "binary_sha256": "9031b8200481747e7ddb5cd5fc2a74030cd04052770bfa23d857a0662a58bc06",
    "binary_size": 266240,
    "binary_source": "in-repo"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x40d120",
    "--addr",
    "--assert",
    "flow 0x40d126 branch"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "// tail-call"
    ],
    "stdout_matches": [
      "0x40d12b",
      "sub_40d21e\\(\\)"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Tail-call recovery appears to run after the explicit flow override and treat the discovered fragment entry as a callee.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5f33c5d40ad448c834` (round 9, tester t-r9-5ab77f5f)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the fix direction DELETES NOTHING -- measured. Binary .kuna-repipe/arena/9/5ab77f5f33c5d40ad448c834/target/crackme1.zip.__x/crkme.exe (PE32 i386). (1) The mechanism, exactly as filed. 0x40d126 is CALL 0x44ac2f; the callee at 0x44ac2f is LEA ESP,[ESP+4] (pops the return address -- it is not a call) / SUB ESP,0xcfc / JMP 0x40d12c, and 0x40d12c is one byte INSIDE the CMP at 0x40d12b, i.e. overlapping-instruction obfuscation that re-enters sub_40d120. The tester's 'flow 0x40d126 branch' is the semantically right directive. tailcalljump then sees a direct BRANCH to a KNOWN function entry (query_call(0x44ac2f).is_some() -- kuna functions has sub_44ac2f, the default output calls it by name) and recovers it as CALL+RETURN, so the function ends at 0x40d126 and the 254-byte body is replaced by 'void sub_40d120(void){ sub_44ac2f(); // tail-call }'. Hypothesis upheld verbatim: tail-call recovery runs after the override and treats the fragment entry as a callee. (2) ONE CONTROL, NOT TWO. The tester reported needing tailcalljump AND tailcallframe off. Measured one variable at a time: 'tailcallframe off' + assert is BYTE-IDENTICAL to the broken stub; 'tailcalljump off' + assert alone exposes the full body. tailcallframe is irrelevant here (its gate is a frame-teardown scan, and there is none at 0x40d126). Dispatch against tailcalljump only. (3) THE FIXED OUTPUT IS A STRICT IMPROVEMENT OVER DEFAULT, WHICH MAKES THIS THE SECOND r9 NEED WHOSE FIX COSTS NO BODY. default (no assert) = 76 lines; assert + tailcalljump off = 78 lines, same body, and the diff is entirely gains: 'int v19; // stack - 0xd68' vs default 'stack - 0x64' -- the fragment's SUB ESP,0xcfc is now in the frame, which the DEFAULT SILENTLY LOSES (0xd68-0x64 = 0xd04 = 0xcfc+8) -- plus 'v21 = 0x40d12b' materialising the CALL's dead pushed return address that the fragment's LEA then discards. Nothing is deleted, unlike the beria / entry-point-ret-dispatch overrides. The natural fix is precedence-only: an explicit user 'flow <addr> branch' is the user asserting 'this is intraprocedural', which is the exact negation of 'this is a tail call', so tailcalljump must not fire at an address carrying a user branch override. Risk to flag for the builder: the datatest corpus carries Ghidra <flow> markup, so a blanket 'any flow override suppresses tailcalljump' must be run past make test rather than assumed. (4) THE ACCEPTANCE IS AN ABSENCE TRAP AND MUST BE REWRITTEN AT T_TRIAGE. It is exit 0 + stdout_absent ['// tail-call'], and the DEFAULT output contains ZERO '// tail-call' -- so a build that simply IGNORES the flow override passes the acceptance with output byte-identical to today's default. Two positive witnesses separate the real fix from the inert one, each 0 hits in default and 1 in the fixed run: '0x40d12b' and '0xd68'. Require at least one of them alongside the absence clause. (5) TOOLING HALF IS REAL, BUT touches IS WRONG. --json reports the assertion as status 'applied', detail null, and function warnings: null; stderr is 0 bytes; and grepping stdout+stderr for tailcall/recovered finds nothing -- so the pass that undid the override logs NOTHING anywhere, despite docs/options.md:651 promising a 'tailcalljump: recovered tail call' warning at the branch site. The agent is told its directive was applied and is given no reason the body vanished. But the fix site is p2_lift/kuna_tailcalljump.rs (flow-override precedence + the missing warning), NOT decompiler/crates/kuna-cli as filed. Retarget touches to decompiler/crates/kuna-decomp at T_TRIAGE.
- round 9 TRIAGE: acceptance rewritten, touches retargeted, track corrected. RE-MEASURED this tick on a post-#504 binary, three runs on the r9 arena crkme.exe at 0x40d120: default (76 lines) has ZERO hits of `0x40d12b`, ZERO of `0xd68` and ZERO of `// tail-call`; assert alone is the 3-line tail-call stub; assert + `tailcalljump off` (78 lines) has exactly one hit of each witness. So the old absence-only clause passed on output byte-identical to default -- both witnesses are now required alongside it. touches kuna-cli -> kuna-decomp (the fix is flow-override precedence in p2_lift/kuna_tailcalljump.rs plus its missing warning, not a CLI change). track tooling -> quality: the precedence rule changes emitted C and the datatest corpus carries Ghidra <flow> markup, so it will ship behind an option and must hold the counter set.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `5ab77f5f`, one image, no ambiguity):

    bin/crackme1.zip.__x/crkme.exe (266240 B, sha256 9031b820…)

Re-run through `verify --acceptance-suite --need explicit-branch-assertion-converted` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    stdout_matches[0] → actual "<no match>"; stdout_matches[1] → actual "<no match>"; stdout_absent[0] → actual "// tail-call"

Nothing about the probe's command or expectations was changed — only the target it
resolves against.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.

### 2026-09-13 — round 12 closure

Vendored the challenge's exact 266,240-byte PE32/i386 witness and made an
explicit, successfully applied `flow 0x40d126 branch` take precedence over the
lower-priority `tailcalljump` inference at that same instruction. Provenance is
recorded during flow following, so a refused raw override fact, a BRANCH at
another address, or another override kind cannot suppress tail-call recovery.

The historical `0xd68` acceptance witness was presentation drift rather than a
semantic invariant. PR #528's call-trampoline following correctly incorporated
the return-address-discarding fragment into frame recovery and moved that local
to `0xd70`. The promoted acceptance therefore requires the stable positive
witnesses `0x40d12b` and downstream `sub_40d21e()`, plus absence of the broken
`// tail-call` stub.

Focused evidence is 11/11 tail-call classifier tests and 3/3 executable CLI
tests. A release A/B against parent `5fb5b031` was byte-identical for no
assertion, explicit `tailcalljump off`, and same-site `callreturn` controls; only
the target BRANCH assertion changed, from the tail-call stub to the full body.
