---
need_id: keyboard-callback-uses-undefined
title: Keyboard callback uses undefined locals as incoming event arguments
track: quality
status: closed
severity: major
probe_id: p-2fa68bfb8456
acceptance_id: a-d6b2084f35d5
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [6989ca5da15272fa37a80c43]
rounds: [2]
first_seen_round: 2
attempts: 1
covered_by_option: inputparamgap
touches: [decompiler/crates/kuna-decomp]
scope: large
regression_of: null
pr: "405"
closed_in_round: 2
closing_pr: "405"
reject_reason: null
---

## Symptom

A decompilation whose signature includes the Wayland callback event arguments used by the first condition.

> **Keyboard callback uses undefined locals as incoming event arguments** (major, `6989ca5da15272fa37a80c43`)
> The function signature contains only a0, while the first condition reads register-associated locals, so the event-type and key parameters appear uninitialized. The disassembly shows those values arrive in R9D and R8D.

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
    "0x6500",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "void sub_6500\\(unsigned long \\*a0\\)",
      "if \\(v[0-9]+ != 1 \\|\\| !a0\\[0xd\\]\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/lugosiii",
    "binary_sha256": "8ea3c454dfcea5d5f970d695608568ed81b688dd367faa2f3fcbeb4bc700aa6e",
    "binary_size": 29336,
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
    "0x6500",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "sub_6500\\(",
      "sub_6500\\([^)]*,[^)]*\\)"
    ],
    "stdout_absent": [
      "^\\s+\\S[^;]*; // r8d$",
      "^\\s+\\S[^;]*; // r9d$"
    ]
  },
  "target": {
    "binary_rel": "bin/lugosiii",
    "binary_sha256": "8ea3c454dfcea5d5f970d695608568ed81b688dd367faa2f3fcbeb4bc700aa6e",
    "binary_size": 29336,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The undiscovered callback has no prototype evidence, and parameter recovery fails to promote live-in R8D/R9D values to arguments.

## Refutation

**Overturned by the BUILDER, not by a refuter** — no refuter agent ever ran on this need
(T_REFUTE round 2 skipped it), so this verdict comes from #405's own measurement and is
recorded here because it is the durable part.

The filed hypothesis was "no prototype evidence, so live-in R8D/R9D are not promoted". That
is not the mechanism. `ActionInputPrototype` registers a trial for every input Varnode and
marks it ACTIVE when the body reads it, and `buildTrialMap` synthesizes filler trials for the
ignored argument registers — so the missing prototype is not what drops them. The
discriminating variable is the WIDTH OF THE HOLE: `ParamListStandard::forceInactiveChain`
(`fspec.cc:1519`) runs with `maxchain = 2`, and once a run of unused slots passes two it marks
every remaining trial inactive, already-active ones included. The witness's rsi/rdx/rcx hole
is exactly three registers, one past the limit; a two-register hole recovers on main today.
Shipped as option `inputparamgap` (DIV-114, default on), which exempts an ACTIVE REGISTER
trial from that veto during the function's own input recovery only.

## Reference

_none recorded_

## Instances

- `6989ca5da15272fa37a80c43` (round 2, tester t-r2-6989ca5d)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): SPLIT out of the 4-observation `wrong-output|decompile|stdout_absent` group; sibling of [direct-address-keyboard-handler] (shared root claim: 0x6500 undiscovered). Deliberately NOT merged into [argument-recovery-knobs-still]: this is callee-side INPUT recovery (live-in R8D/R9D never promoted to parameters), not caller-side argument recovery.
- round 2 T_TRIAGE (captain): scope small -> LARGE. The values arrive in R9D/R8D of a callback only ever reached through a registration site, so recovering them means inferring a prototype from that registration -- a real feature. Note the cheaper path this need is evidence FOR: if the CLI could carry a prototype override (need no-cli-rename-or-prototype-override) an agent could state the signature itself and would not need the inference at all. That makes this a demand witness for the no-cli-* family, which is exactly the independent tester evidence those captain-seeded needs were missing.

- round 2 B_PLAN wave 58 (captain): **DISPATCHED, attempt 1, with the briefing below.** Everything
  in it was MEASURED this tick at `57ebfd2b` on `.kuna-repipe/probebin/8ea3c454dfcea5d5/lugosiii`,
  not inherited from an earlier wave.
- round 2 B_PLAN wave 58 (captain): briefing.
  (a) THE DEFECT REPRODUCES EXACTLY AND IS SMALL NOW. `kuna decompile <bin> 0x6500 --addr` -> rc 0,
  427 lines, opening `void sub_6500(long a0)` with `int v7; // r8d` and `int v8; // r9d` declared
  and then READ before any definition (`if (v8 != 1 || !*(long *)(a0 + 0x68))`,
  `xkb_state_key_get_one_sym(..., v7 + 8)`). The acceptance fails on exactly three clauses:
  `stdout_matches[1]` (`sub_6500\([^)]*,[^)]*\)` = <no match>) and both `stdout_absent` clauses,
  which match those two declarations. `exit_code eq 0` and `stdout_matches[0]` ALREADY pass, so
  the probe cannot be closed by emitting nothing or by erroring out.
  (b) #395 IS DONE WITH THIS FUNCTION; DO NOT RE-LITIGATE THE BOUNDARY. `tailcallframe` (DIV-109,
  default on) shrank 0x6500 from 1,555 lines to the 427 above and closed the sibling need
  `direct-address-keyboard-handler`. The BODY is now right; only the INTERFACE is wrong. Nothing
  in this need asks for a boundary change.
  (c) NO SHIPPED OPTION COVERS THIS -- measured, not assumed. Signature and declaration block are
  unchanged under every flippable parameter-recovery option: `calloverlap full`,
  `spillargtrial spill`, `spillargtrial reload`, `evalcurrentproto off`, `calleedeadarg off`.
  The other param-adjacent options (`inputvarnodeadjust`, `callsitestackargs`, `varargstackargs`,
  `calleearity`, `calleearityfwd`) are call-site-scoped and this function HAS no call site in the
  binary: it is a Wayland `wl_keyboard_listener` entry reached only through a function-pointer
  table, so its prototype can come only from its own body.
  (d) GROUND TRUTH FOR "RIGHT", so you are not scored by the regex alone. The witness is the
  Wayland keyboard `key` callback:
  `void (*key)(void *data, struct wl_keyboard *, uint32_t serial, uint32_t time, uint32_t key, uint32_t state)`
  -- SysV rdi=data, rsi=wl_keyboard, rdx=serial, rcx=time, r8d=key, r9d=state. The body corroborates
  it: `v8 != 1` is the `state == WL_KEYBOARD_KEY_STATE_PRESSED` test and `v7 + 8` is the evdev->X
  keycode bias handed to `xkb_state_key_get_one_sym`. So `v7` is arg 5 and `v8` is arg 6, and
  rsi/rdx/rcx are dead. Recovery therefore has to survive a GAP; a fix that only extends a
  contiguous prefix never reaches r8/r9.
  (e) EVIDENCE THAT BEARS ON THE FILED HYPOTHESIS (which is advisory and unrefuted --
  `hypothesis_status` stays `inconclusive`, that field is a refuter's). #395 left an in-repo
  Wayland-shaped fixture, `decompiler/crates/kuna-analysis/tests/fixtures/tailcallframe_x86_64`,
  whose `on_key` at 0x11b0 is ALSO reachable only through a listener[] pointer table. kuna emits
  `void sub_11b0(int *a0,int a1)` there -- i.e. live-in registers from a pointer-table-only
  callback with no prototype evidence ARE promoted when they are CONTIGUOUS (rdi, rsi). The filed
  hypothesis ("no prototype evidence, so live-in R8D/R9D are not promoted") does not explain that
  contrast; the discriminating variable in the witness is the rsi/rdx/rcx gap. Confirm or overturn
  this yourself -- if you overturn it, say so explicitly in the PR, exactly as #395's builder did.
  (f) IN-REPO COVER. `binary_source: dataset` means `verify --promote` will REFUSE this acceptance
  (the #377/#392/#404 wall: CI has no dataset). #395 solved the identical problem by SYNTHESIZING
  the fixture in (e) and vendoring `tests/cli/direct-address-keyboard-handler.json` against it --
  that is the precedent to copy. The fixture above does NOT reproduce this symptom (its params are
  contiguous), so you need a new one with the gap shape, plus/or a `tests/stages/` two-pass XML.
  (g) BLAST RADIUS. Promoting live-in registers into prototypes changes signatures and call-site
  argument counts corpus-wide. `make test` (675/675) and `make test-stages` are the guard; a
  default-ON flip needs 0/675 changed assertions and a DIV row, otherwise ship it default-OFF.
  (h) `scope: large`. If the honest mechanism is a multi-step engine change, take the `[PROPOSAL]`
  fork rather than half-porting it -- the captain reviews parked proposals and re-dispatches an
  implementation worker on the same branch. Branch `feat/re-keyboard-callback-uses-undefined` does
  not exist and no worktree `b-r2-keyboard-callbac` exists, so worker.sh's silent detached-worktree
  fallback is NOT armed for this dispatch.
- closed: acceptance a-d6b2084f35d5 now PASSES at fba4ebd8d4b3

- round 2 wave 61 B_DONE (captain): **CLOSED on #405**, `closed_in_round` 2, `pr`/`closing_pr`
  405, `attempts` stays 1, `covered_by_option` inputparamgap. The close is mechanical, not a
  judgment: acceptance `a-d6b2084f35d5` FAILED at filing and PASSES at `fba4ebd8` on exactly the
  three clauses that failed — `stdout_matches[1]` (`sub_6500\([^)]*,[^)]*\)` now matches
  `sub_6500(long a0,unsigned long a1,unsigned long a2,unsigned long a3,i...`) and both
  `stdout_absent` clauses (`// r8d`, `// r9d` declarations gone). `exit_code eq 0` and a positive
  match are both in the probe, so this is NOT the emit-nothing loophole. Applied from
  `rounds/2/acceptance-w60.json` (35 needs, closed ['keyboard-callback-uses-undefined'],
  regressed []) after wave 60 ran all five gates green on a rebuilt main.
- round 2 wave 61 B_DONE (captain): `hypothesis_status` inconclusive -> **overturned**, see the
  Refutation section. That makes it 4-of-9 filed diagnoses overturned across the campaigns while
  the symptom stood in all 9 — the reason a need's hypothesis is advisory.
- round 2 wave 61 B_DONE (captain): NO `verify --promote` call. `tests/cli/keyboard-callback-uses-undefined.json`
  is already on main at `fba4ebd8` (the builder promoted it inside the squash) and `make test-cli`
  counted 28/28 in wave 60, up from 27. Re-promoting would have been a no-op at best.
