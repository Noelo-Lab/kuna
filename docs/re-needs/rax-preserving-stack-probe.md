---
need_id: rax-preserving-stack-probe
title: RAX-preserving stack probe leaves an undefined stack base
track: quality
status: open
severity: blocker
probe_id: p-5ac15314ce59
acceptance_id: a-5557685cdbec
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [69d7f15a8afd9d6c48b48871]
rounds: [5]
first_seen_round: 5
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 542
closed_in_round: null
closing_pr: 542
reject_reason: null
---

## CAPTAIN BRIEF — READ THIS BEFORE ANYTHING ELSE

**Your implementation is already complete, pushed, and green. This dispatch is MERGE-ONLY.
Do not redesign, do not re-implement, do not open a new branch or a new PR.**

You are resuming your own earlier attempt on `feat/re-rax-preserving-stack-probe`
(head `20a55f1d`). PR **#542** is OPEN / MERGEABLE / CLEAN with every CI check green.
The previous attempt ran all four gates locally and they passed: `make test` 675/675,
`make test-stages` 730/730, `make rust-test`, `make check-spec`, plus `make test-cli`
108/108 and `kuna catalog --check`. (The workspace suite shows SKIPPED in CI because this
is a branch PR, which is why those local runs are the evidence.)

**Why you stopped last time, and why that reason is gone.** You held the PR because the
acceptance clause `functions[0].line_mappings len_gt 7` was unsatisfiable by any correct
fix. You were right, and the clause was the captain's error. It has been REPAIRED: the
positive clause is now `functions[0].code matches "sub_1408d0000\\("`. The acceptance
JSON in the `## Acceptance` section of THIS file is the current, authoritative one
(`a-5557685cdbec`) — the copy committed on your branch is the OLD, dead one, so trust this
file and not the branch's.

The captain reproduced your finding independently on unpatched main `1c66b462`, using
`--assert bytes` overlays over the 13 bytes at `0x14000100c`, with three controls:
unmodified gave 11 line_mappings and 8 `Stack...[v1]` hits; a simulated ideal fix
(`CALL 0x1408d0000` + `SUB RSP,0x5e028` + NOP) gave 2 line_mappings, 0 Stack hits and the
body `{ sub_1408d0000(); return; }`; removing the call entirely gave 1. Your reported count
was reached without your patch, so the corroboration is not circular. The mechanism is that
the clip's exit `JMP 0x140001053` lies outside the `--define-function` range, so a correct
fix legitimately collapses the body.

**What to do, in order.**

1. Confirm the branch is intact and the PR is still green (`gh pr view 542`,
   `gh pr checks 542`).
2. Rebase onto current `origin/main` **only if** GitHub reports the PR is no longer
   mergeable. It was CLEAN at dispatch time. If you do rebase, re-check the counter
   surfaces you touched — `catalog_bytecompat.rs`, `kuna_phases/tests.rs`, `xml.rs`,
   `docs/options.md` — because those are exactly the files that merge cleanly to a wrong
   number.
3. Run the acceptance probe in the `## Acceptance` section below against your branch build
   and confirm it now PASSES. If it does not, say so plainly in `result.json` and stop —
   do not weaken the clause.
4. Run the gates on the branch, then **merge PR #542** through the normal
   `open_pr.sh --merge` path under the `merge` lease.
5. Promote the acceptance probe into `tests/cli/` if you have not already, and close the
   need record (`status: closed`, `closing_pr: 542`).

DIV-149 was still unclaimed at dispatch time; `#539` took 148. Your option is
`calleescratchbody`, default ON. Note `calleepreserves` already exists on main — the probe's
`--option calleepreserves on` is valid there, and `calleescratchbody` is the one your branch
adds.

This should be a cheap tick. If you find yourself designing, you have misread this brief.

One housekeeping note: this file is TRACKED on your branch and the captain has overwritten
your worktree's copy with the corrected one, so `git status` will show it modified. That is
deliberate — the branch's committed copy carries the dead acceptance clause. **Commit this
file to the branch** as part of the merge; it leaves the worktree clean and lands the
corrected acceptance record alongside the fix.


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
        "path": "functions[0].code",
        "op": "matches",
        "value": "sub_1408d0000\\("
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
- round 5 REFUTER: hypothesis **overturned** (was upheld). BUILDER MEASUREMENT (worker b-r12-rax-preserving-s), not a refuter run -- recorded here because this field is the only machine-readable home for it. The round-5 refutation named the right pass and the WRONG mechanism: it read 'MOV R11,qword ptr GS:[0x10]' as a CALLOTHER that truncated calleepreserves' callee-body walk. On x86-64 SLEIGH lowers a segment override to GS_OFFSET + disp (ia.sinc:1117,1143) with no userop, so the walk COMPLETES at 19 instructions and proves_untouched(RAX) is already true. The real blocker is the positive-evidence gate body_departs_from_convention, which demands the callee write a register the model marks <unaffected>; __chkstk writes only RSP/R10/R11 and x86-64-win.cspec names none of them in either list, so a well-behaved scratch-only helper never trips it. Symptom stands exactly as filed.
- round 12 CAPTAIN (B_MERGE bookkeeping): BUILDER HELD THIS DELIBERATELY -- NOT A CRASH, AND NOT UNTOUCHED WORK. b-r12-rax-preserving-s ran to completion ($20.58, 164 turns, result.json full, subtype=success) and ended `failed` on purpose. PR #542 is OPEN, MERGEABLE/CLEAN, all CI green (workspace suite SKIPPED per the branch-PR rule; the builder reports make test 675/675, test-stages 730/730, rust-test, check-spec, test-cli 108/108, catalog --check all green locally), branch feat/re-rax-preserving-stack-probe at 20a55f1d, based on 7ccd3175. attempts bumped 0 -> 1 and pr set to 542 so select.py does not re-dispatch this as virgin work and burn another ~$20 repeating a refuted sub-hypothesis. THE HOLD IS ABOUT ONE ACCEPTANCE CLAUSE -- ONE THE CAPTAIN ADDED LAST TICK. Acceptance a-ccd85db0e5f2 is 3 of 4: the defect clause (stdout_absent 'Stack<hex>)[v]'), error == null and exit 0 all PASS; `functions[0].line_mappings len_gt 7` fails at 2. The builder's control for that clause: the probe's own `--define-function 0x140001000-0x140001043` clips the function's flow, so once the frame index folds to a constant the 8 register-save stores are unread locals and dead-store elimination removes them -- and replacing the ENTIRE calling sequence with a literal `sub rsp,0x5e028` and no call at all (via `--assert 'bytes 0x14000100c 4881ec...'`) yields the same empty body. If that control holds, no fix to THIS defect can satisfy the clause and the clause is the captain's error, not the builder's shortfall. IT IS UNVERIFIED BY THE CAPTAIN -- a later tick must reproduce it before relaxing anything ([[do not relax an acceptance to close a need]] is the standing rule; relaxing it because a builder asked is exactly what that rule forbids). RESUME PATH IF THE CONTROL HOLDS: the work is already pushed, so this is a cheap merge-only re-dispatch (IMPL_PROPOSAL not needed; RESUME_BRANCH=feat/re-rax-preserving-stack-probe), NOT a fresh $20 builder. Repair the acceptance clause first, then re-dispatch. SEPARATE NEED, DO NOT BOLT IT ON: keeping a clipped function's frame live-out is an independent P3 dead-code change -- seeding every address-tied stack Varnode as consumed at the top of ActionDeadCode did NOT restore them, so it sits upstream of that seam. The permanent regression guard shipped in the PR as tests/stages/kuna-calleescratchbody.xml (two-pass, __chkstk verbatim, with an `endbr64; ret` negative control); `verify --promote` still refuses the dataset probe target.
- round 12 CAPTAIN (09:3xZ tick, B_DONE->B_IDLE, off-critical-path): **THE BUILDER'S CONTROL HOLDS. `line_mappings len_gt 7` WAS UNSATISFIABLE BY CONSTRUCTION AND IS NOW REPAIRED** (a-ccd85db0e5f2 -> a-5557685cdbec). I reproduced it myself rather than taking the builder's word, per the standing rule that a clause is not relaxed because a builder asked. Three controls, ALL on unpatched main 1c66b462, all with the probe's own argv and its own clip `--define-function 0x140001000-0x140001043`, differing only in a `--assert bytes` overlay at 0x14000100c (the 13 bytes holding `MOV EAX,0x5e028 / CALL 0x1408d0000 / SUB RSP,RAX`):
  (a) UNMODIFIED -- line_mappings 11, 8 `Stack...[v1]` hits, helper call present. The broken body.
  (b) IDEAL FIX SIMULATED, overlay `e8efef8c004881ec28e0050090` = `CALL 0x1408d0000` (rel32 recomputed for the new site) + `SUB RSP,0x5e028` + one NOP, 13 bytes exactly -- i.e. the call is STILL THERE and the allocation is a constant, which is precisely what a correct `calleepreserves` fix produces. Result: **line_mappings 2**, 0 Stack hits, call present, body `{ sub_1408d0000(); return; }`. That is byte-for-byte the count the builder reported from their ACTUAL fix, reached WITHOUT their fix -- so their measurement is corroborated independently and there is no circularity in trusting it.
  (c) CALL REMOVED ENTIRELY, overlay `4881ec28e00500` + 6 NOPs -- line_mappings **1**, empty body. Stronger than the builder's report of 2.
  MECHANISM: the clip's exit is `JMP 0x140001053`, outside the defined range, so the 8 register-save stores are never read. While the frame base is the undefined `v1` their addresses are indeterminate and dead-store elimination cannot touch them; the moment a fix folds the index to a constant they become provably-dead stores at known stack offsets and collapse. The clause therefore MEASURES THE DEFECT: only the broken output is large enough to pass it, and every fix fails it. Widening the range to the full extent 0x140001000-0x140001063 does NOT help (identical output, 11/8 -- the out-of-bounds warning is unchanged), and dropping `--define-function` altogether explodes the function past the 100000-instruction budget (5837 mappings, 4596 Stack hits), so the clip is not removable either.
  THE REPAIR, AND WHY IT IS NOT A RELAXATION. The vacuous-positive hole the clause was added to plug is still plugged, by a clause calibrated on MAIN rather than on the fix: `functions[0].code matches "sub_1408d0000\\("` -- the helper call must still be emitted. Measured just now on unpatched main it is TRUE (control a), and on the ideal shape it is TRUE (control b), while an empty or lost body fails it (control c has no call at all). It is a degradation guard, not a discriminator; the DEFECT clause `stdout_absent Stack<hex>)[v]` remains the sole discriminator and it still FAILS today. Verified after the edit with `verify --acceptance-suite --need rax-preserving-stack-probe`: unrunnable=false, passed=false, and clause-by-clause exit_code ok / **stdout_absent ok=FALSE** / error==null ok / code-matches ok -- one failing clause, and it is the bug. By control (b) all four pass on a correct fix, so the acceptance is now satisfiable AND still honestly failing. The reproduction arm's probe_id did not move (cmd+expect unchanged there).
  NEXT: PR #542 is OPEN / MERGEABLE / CLEAN at 20a55f1d, green, and its code is unchanged by any of this. The re-dispatch is MERGE-ONLY (RESUME_BRANCH=feat/re-rax-preserving-stack-probe, no IMPL_PROPOSAL, not a fresh ~$20 builder).
- round 12 CAPTAIN (B_IDLE->B_PLAN tick, 09:4xZ): SELECTED FOR WAVE 4 AS A MERGE-ONLY RESUME, AND **THE RESUME RECIPE RECORDED ABOVE IS WRONG — `RESUME_BRANCH` ALONE IS INERT.** `tools/pipeline/worker.sh:50` honours it only under `[ "$IMPL_PROPOSAL" = 1 ] && [ -n "$RESUME_BRANCH" ]`; without the flag it falls to `BRANCH="${BRANCH_PREFIX}${SLUG}"` and then to the reuse arm, whose `git worktree list --porcelain | grep -qxF` is the SIGPIPE race that is dead at 88+ worktrees — this repo now has **107**, so that arm cannot be relied on, and both fallbacks (`worktree add -b`, then detached `worktree add`) fail on the already-existing path. A plain re-spawn therefore dies at setup. The working form is `IMPL_PROPOSAL=1 RESUME_BRANCH=feat/re-rax-preserving-stack-probe`, which takes worker.sh's arm 1 (`git worktree add "$WT" "$BRANCH"`) — but that also fails on an existing path, so `.kuna-repipe/worktrees/b-r12-rax-preserving-s` must be removed first. Verified safe to remove: the worktree is CLEAN (`git status --short` empty) at 20a55f1d, which is exactly `origin/feat/re-rax-preserving-stack-probe`, so nothing unpushed is in it.
- round 12 CAPTAIN (same tick): **THE BRANCH CARRIES A STALE COPY OF THIS NEED DOC AND IT WILL RE-BREAK THE BUILDER IF NOT OVERWRITTEN.** `docs/re-needs/rax-preserving-stack-probe.md` is UNTRACKED on main but TRACKED on the branch (PR #542 added it), and the branch copy is frozen at `acceptance_id: a-ccd85db0e5f2` — the unsatisfiable `line_mappings len_gt 7` contract. `verify` resolves the record relative to `KUNA_ROOT`, which worker.sh points at the worktree, so a resumed builder would read the OLD clause, re-derive that it cannot be met, and hold the PR a second time for ~$20. The fanout tick MUST `cp docs/re-needs/rax-preserving-stack-probe.md <WT>/docs/re-needs/` after the worktree is created and before the prompt renders (the `make binaries` window is minutes wide). Same rule for every fresh builder this wave: 92 of the 197 need docs are untracked on main, so a worktree branched from main has no need doc at all unless the captain copies it in.
- round 12 CAPTAIN (same tick): MERGE ORDER IS THE REASON THIS OUTRANKS A FRESH QUALITY BUILDER. #542 adds `calleescratchbody` (default **on**, DIV-149) and touches all four counter surfaces — phases.toml, options.rs, catalog_bytecompat.rs, kuna_phases/tests.rs, xml.rs, docs/options.md, docs/history.md, tests/stages/. Any other quality builder bumps the same counts, and an identical `N -> N+1` on two branches merges CLEANLY to the wrong number. Merging #542 first costs one cheap resume; dispatching `undecodable-encrypted-code-becomes` (blocker, 20.79) first makes #542 a conflicted rebase. Checked today: #540 and #541 touched NO counter file, `mergeStateStatus: CLEAN`, all checks green (workspace suite SKIPPED per the branch-PR rule), and DIV-149 is still free on main (#539 took 148). ALSO CHECKED, so the resume is not a gamble: the branch's own module doc renders the fixed body as `sub_1408d0000();`, which satisfies the REPAIRED positive clause `functions[0].code matches "sub_1408d0000\\("`, and the option is default-on so the probe's plain argv exercises the fix.
