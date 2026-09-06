---
need_id: argument-recovery-options-still
title: Argument-recovery options still omit graph-link endpoints
track: quality
status: closed
severity: major
probe_id: p-8208197170e8
acceptance_id: a-ee7c01822c2a
hypothesis_status: upheld
credibility: 1.0
instances: 3
challenges: [60be2ad433c5d410b8842c95, 69761b7a39e9c4d85c2f9fc1, 6989ed7dfb46458f1ef6cee4]
rounds: [4]
first_seen_round: 4
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p4_calls/funcdata_callsite.rs]
scope: medium
feature_option: calleearitylive
regression_of: argument-recovery-knobs-still
pr: pending
closed_in_round: 4
closing_pr: null
reject_reason: null
---

## Symptom

Recover all five arguments of the graph-linking helper.

> **Argument-recovery options still omit graph-link endpoints** (major, `69761b7a39e9c4d85c2f9fc1`)
> The helper decompiles with five parameters, but several checker calls retain only three arguments with both calleearity and varargstackargs enabled. An explicit prototype restores the missing arguments.

> **Success-output call loses its third argument despite recovery options** (major, `6989ed7dfb46458f1ef6cee4`)
> The call at 0x140004ab0 has two arguments although disassembly loads R8 from [RAX+0x10] before it. Sibling calls retain three arguments. Explicitly enabling calleearity and varargstackargs does not restore the length; declaring the prototype does.

> **Fastcall checker invocation loses both register arguments** (major, `60be2ad433c5d410b8842c95`)
> Emitted sub_401c50() despite ECX and EDX assignments immediately before the call. The callee separately decompiles with two inputs. Both recovery options leave the arguments absent; their documented cases do not cover this single fastcall site. A plain prototype supplies incorrect stack arguments, while __fastcall syntax is rejected.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_1002c90",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "stdout_matches": [
      "(?m)^\\s+sub_[0-9a-f]+\\(&v[0-9]+,[^,;\\n]+,[^,;\\n]+\\);"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_1002c90",
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
      "(?m)^\\s+sub_[0-9a-f]+\\(&v[0-9]+,[^,;\\n]+,[^,;\\n]+\\);"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Recovered callee arity is not reaching these caller sites.
- Sibling arity reconciliation may rescue empty lists but leave partially recovered argument lists untouched.
- Callee calling-convention knowledge may not propagate into the caller.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load './target/Sabloom Text 6.exe'` — Exit 1: Failed to open database. No reference decompilation obtained.

## Instances

- `69761b7a39e9c4d85c2f9fc1` (round 4, tester t-r4-69761b7a)
- `6989ed7dfb46458f1ef6cee4` (round 4, tester t-r4-6989ed7d)
- `60be2ad433c5d410b8842c95` (round 4, tester t-r4-60be2ad4)

## Decision log

- filed by cluster.py from 3 observation(s)
- split out of the round-4 `16-byte-vm-state` mega-bucket by the captain at T_DEDUP: cluster.py's key is `kind|subcommand|clause-shape`, so nine unrelated wrong-output `decompile` defects collapsed into one need whose probe covered only the first. Each carries its own probe and acceptance from its own observation.
- lineage: `argument-recovery-knobs-still` (closed round 2 by PR #376) still PASSES its own acceptance on main, so this is not a regression of that probe -- it is the same class of gap on three new binaries, found by three testers with both `calleearity` and `varargstackargs` enabled.
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK BY THE CAPTAIN (no refuter role exists in the launcher), on the 69761b7a witness, sha 6188e204, arena .kuna-repipe/arena/4/69761b7a39e9c4d85c2f9fc1/target/graphy-release.zip.__x/graphy.

SYMPTOM REPRODUCED AND THE ARGUMENTS ARE GENUINELY LIVE. `kuna decompile ... sub_1002c90 --option calleearity on --option varargstackargs on` emits 15 calls to sub_1005250; 11 carry five arguments and FOUR carry three. The callee itself decompiles as sub_1005250(long*,uint,uchar,uint,uchar). objdump at the broken site 0x10033c7 and at a good site 0x10035ba is instruction-for-instruction the same shape -- lea -0x40(%rbp),%rdi / mov %r15d,%esi / movzbl ..,%edx / mov %r12d,%ecx / movzbl %al,%r8d, then CALL -- so ecx and r8d are written by dedicated movs immediately before the call at BOTH. The three-argument rendering is wrong output, not a defensible shorter list.

HYPOTHESIS BULLET 2 IS EXACTLY RIGHT AND IS THE ACTIONABLE CAUSE. 'Sibling arity reconciliation may rescue empty lists but leave partially recovered argument lists untouched' is not a guess -- it is the documented, deliberate limit of the mechanism. kuna_calleearity.rs: 'Only a call that recovered NOTHING. This is the limit the whole-corpus sweep bought.' These four sites recovered three arguments, so calleearity declines them by design. Bullets 1 and 3 ('callee arity/convention does not reach the caller') are true as DESCRIPTION but misleading as direction: kuna deliberately never promotes a caller trial from the callee's own recovered prototype, only from a sibling CALL SITE.

TWO CORRECTIONS THE BUILDER MUST HAVE, BOTH OF WHICH DEFEAT THE OBVIOUS FIX.

(1) DROPPING THE EMPTY-ONLY GATE IS ALREADY MEASURED TO FABRICATE ARGUMENTS. The same module records what happened when the rule read 'same callee, same arity' without that limit, over the whole corpus: Sleep(200) became Sleep(200,0) from a sibling that had over-recovered rdx, and sub_1b11c(5,0,"Zip: empty archive?") gained two arguments its format string has no conversions for. So the one-line relaxation is a known regression, not an untried idea. A fix needs a NEW discriminator that the 2026 sweep did not have -- the callee's own recovered input list is the obvious candidate (it would refuse the variadic Sleep case and admit this one), and that is a real feature, not a gate flip.

(2) THE WITNESS ORDERING IS ADVERSARIAL, SO THE GATE FLIP ALONE FIXES NOTHING HERE. The four broken sites are the FIRST four; the first five-argument site is the fifth. calleearity only reconciles against sites that finalize BEFORE the one it is rescuing, so at all four there is no witness yet. calleearityfwd is the deferred-retry direction that could see a later witness -- and it is itself gated to 'a call that recovered NOTHING at all'. Both gates therefore have to move together. A patch touching only kuna_calleearity.rs will leave this probe red.

SCOPE WARNING FOR TRIAGE: this need's three instances are NOT one shape. 69761b7a and 6989ed7d are partial lists (3-of-5, 2-of-3) and are the shape above. 60be2ad4 is an EMPTY list at a lone fastcall site -- calleearity would already admit an empty list, so what defeats that one is the absence of ANY sibling witness in the function, a different sub-cause that a sibling-based fix cannot close. Its acceptance clause is also the loose one ('no zero-arg sub_ call anywhere'). Expect this need to close on 2 of 3 instances; do not treat the third as a regression of the fix.

VERDICT upheld: the filed cause stands as the diagnosis, but the fix it most obviously implies is a measured regression, so scope 'small' is optimistic -- this is a discriminator, not a gate flip.

- BUILDER b-r4-argument-recover: closed by option `calleearitylive` (default-on, DIV-123).
  Hypothesis UPHELD; the captain's two corrections both held. The filed cause
  (partial lists are left alone by design) is right, and the obvious relaxation is
  a measured regression, so the fix is a NEW discriminator: the callee's own body,
  read with `calleedeadarg`'s existing bounded entry decode, must show that the
  registers the witness claims beyond this site's list are read before written AND
  that no OTHER argument register of the model is. The negative half is what does
  the work -- liveness alone re-admits the `linker64` variadic regression, because
  an AArch64 `va_start` register-save area reads x3..x7 before writing them.
  ONE CORRECTION TO THE DIAGNOSIS. Instrumented on the witness, the trial that
  decides is R8, not RCX: `ancestor_op_use` answers `only=false` at 0x10033c7 on a
  `CPUI_STORE` at 0x10036d5 -- a competing use of the same register value a
  thousand bytes away, reached through a MULTIEQUAL -- and `fillin_map`'s
  positional rule then drops RCX behind the hole R8 leaves. RCX is `only=false` at
  the GOOD site too, so it was never the discriminator. The class is the same
  (a competing use anywhere in the function sinks the last recovered trial); the
  opcode is STORE, not CBRANCH.
  Closes 2 of the 3 instances as the captain predicted: 69761b7a and 6989ed7d are
  the partial-list shape. 60be2ad4 (an EMPTY list at a lone fastcall site with no
  sibling witness at all) is a different sub-cause and is NOT closed by this.

