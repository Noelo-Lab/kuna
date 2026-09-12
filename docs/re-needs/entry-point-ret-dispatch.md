---
need_id: entry-point-ret-dispatch
title: Entry-point RET dispatch disappears into an empty return
track: quality
status: closed
severity: major
probe_id: p-9c278d6e9d91
acceptance_id: a-96dd05b48b4a
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5c33c5d40ad448c681]
rounds: [9, 12]
first_seen_round: 9
attempts: 1
covered_by_option: entryretdispatch
touches: [decompiler/crates/kuna-console, decompiler/crates/kuna-cli, decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: 595
closed_in_round: 12
closing_pr: "595"
reject_reason: null
---

## Symptom

See the transfer to LoadLibraryA, or a warning that stack-directed control flow was not recovered.

> **Entry-point RET dispatch disappears into an empty return** (major, `5ab77f5c33c5d40ad448c681`)
> Emitted only return; with exit 0. Kuna disassembly at 0x40175f shows PUSH 0x40151e; PUSH 0x401770; PUSH [0x4010fe]; RET. A flow-call override restores LoadLibraryA, confirming that the default output erased a transfer. These are decodable instructions, not encrypted code.

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
    "0x401757",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\{\\s*return;\\s*\\}"
    ]
  },
  "target": {
    "binary_rel": "bin/bm3.zip.__x/bm3.exe",
    "binary_sha256": "8116e2cb1bbe102beba05abe99c6e41f913e8fad7867c1640fcb002492b3fe79",
    "binary_size": 26624,
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
    "entry_dispatch"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\{\\s*return;\\s*\\}"
    ],
    "stdout_matches": [
      "\\(\\*dat_804a000\\)\\(\\);",
      "\\(\\*dat_804a004\\)\\(\\);",
      "\\(\\*dat_804a008\\)\\(\\);"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/entry_ret_dispatch_i386",
    "binary_sha256": "d56c357fef6d4fcd03aa96a20fd5adb4bf3635a803f0cfa91779f3428492f7b6",
    "binary_size": 9284,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/entry_ret_dispatch_i386",
    "selector": "entry_dispatch",
    "selector_kind": "name"
  },
  "notes": "Synthetic i386 twin of bm3's three-link RET dispatcher. E2e controls preserve ordinary/immediate/computed RETs and reject discarded or unrelated fall-through stores, a signed negative-displacement overwrite, a conditional bypass, and a partial-SP alias write. CLI tests pin option-off restoration and explicit RETURN precedence."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- RETURN handling appears to discard the computed destination and its feeding stores.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/bm3.zip.__x/bm3.exe` — Server aeddc29d01 exited with status 1 before registering. No reference pseudocode was obtained; IDA's ability to recover this dispatch remains unknown.

## Instances

- `5ab77f5c33c5d40ad448c681` (round 9, tester t-r9-5ab77f5c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD (captain in-tick, release binary 8df2a52e, arena .kuna-repipe/arena/9/5ab77f5c33c5d40ad448c681/target/bm3.zip.__x/bm3.exe).

FILED CAUSE ('RETURN handling appears to discard the computed destination and its feeding stores') is right, and the single-variable control proves the information is PRESENT and merely unused. Four runs, one variable each, same address:
 (D) default                                 -> void sub_401757(void) { return; }        [the bug]
 (A) --assert 'flow 0x40176f call'           -> LoadLibraryA(s_40151e);
 (B) --assert 'flow 0x40176f branch'         -> LoadLibraryA(s_40151e); // jump-as-call
 (C) tiedstorekeep off / calleepop off / constselectjump on / tailcalljump off -> all byte-identical to (D)
Nothing in the 164-row catalog moves this: there is no push-ret / stack-directed-transfer row at all, so 'no option covered it' is a fact about the catalog, not a tester ablation miss.

THE DECISIVE POINT: the RET's popped destination is dword ptr [0x4010fe], an IAT slot, and kuna RESOLVES IT TO LoadLibraryA the instant the RET is reinterpreted as a transfer (run A). Same binary, same dataflow, same pass order -- only the RETURN interpretation differs. So the destination is computable at the point where it is thrown away; it is not a missing-analysis problem, it is that nothing ever asks whether a RETURN's popped value is a resolvable constant rather than the incoming frame's return address. Feeding stores then die as ordinary dead stack writes, which is why the body empties instead of degrading.

Disassembly confirmed byte-for-byte (kuna disassemble 0x40175f): 68 1e 15 40 00 PUSH 0x40151e / 68 70 17 40 00 PUSH 0x401770 / ff 35 fe 10 40 00 PUSH [0x4010fe] / c3 RET. Stack at the RET is target=[0x4010fe], return-addr=0x401770, arg=0x40151e -- and run A picks the argument up correctly, which is further evidence the frame is understood. Note 0x401757 is the PE ENTRY POINT, and functions[0].size is 105 bytes: the whole entry stub renders as one empty return.

TWO THINGS THE BUILDER MUST NOT DO -- both pass the acceptance as written (stdout_absent '\{\s*return;\s*\}', absence-only, same trap as beria-call-fragments-produce):
 (1) 'Preserve the feeding stores' READ LITERALLY. Suppressing dead-store elimination on the pre-RETURN stack writes yields a non-empty body of raw *(esp-N) = 0x... garbage with no transfer recovered. Acceptance goes green, output gets worse.
 (2) Internally applying the equivalent of run A. That IS the intended shape, but on this binary it emits exactly one statement for a 105-byte function: the pushed continuation 0x401770 (XCHG EBX,EBX; PUSH 0x40152b; ...) and everything after it is dropped, including the later GetProcAddress fragment. Acceptance green, ~50 bytes of real body silently gone.
ACCEPTANCE REWRITE REQUIRED BEFORE DISPATCH: replace the absence clause with a positive witness -- stdout_matches 'LoadLibraryA' AND a continuation witness (the 0x401770 fragment, e.g. '0x40152b' or a second call), so neither wrong shape can pass.

SIBLING COUPLING -- THIS IS THE DISPATCH CONSTRAINT. flow-call-override-retain (20.79 blocker, same challenge 5ab77f5c, UPHELD last tick, currently in flight as builder b-r9-flow-call-overri) is precisely 'the override cannot retain the pushed continuation'. Run A above is that need's bug seen from this need's side: recovering the transfer here yields a truncated body until the continuation is retained. Do NOT dispatch entry-point-ret-dispatch as an independent builder. Either hold it until flow-call-override-retain merges and re-measure, or bind them onto one builder via the sibling cluster: lease.

FIX-DIRECTION RISK for the eventual builder: a blanket 'RET with a computed destination is a transfer' rule is the wrong generalization -- gate it on the popped value provably originating from an in-function store/load rather than the incoming return-address slot, or unbalanced-stack functions (cf. RAX-preserving stack probe, int29) start losing genuine returns.
- round 9 TRIAGE: acceptance rewritten AND the sibling-coupling block LIFTED -- re-measured this tick against a post-#504 binary on the r9 arena bm3.exe. (i) The default at 0x401757 is UNCHANGED by #504: still `void sub_401757(void) { return; }`, so the need is live and is the no-override half of the same idiom. (ii) Run A (`--assert 'flow 0x40176f call'`) is TRANSFORMED by #504: where it used to emit the single truncated line `LoadLibraryA(s_40151e);`, it now recovers the whole 105-byte entry stub -- LoadLibraryA, GetProcAddress, MessageBoxA, ExitProcess, GetModuleHandleA, VirtualProtect, CreateFileA, VirtualAlloc, SetFilePointer -- with the pushed continuation at 0x401770 retained (`v15 = s_40152b;`). The refuter's second forbidden shape ("internally applying run A drops ~50 bytes of real body") no longer exists, so this need may now be dispatched as an independent builder. (iii) Acceptance: the absence-only clause is kept but joined by stdout_matches `LoadLibraryA` and `GetProcAddress` -- both 0 hits in today's output, both present in run A, and the second one can only come from the retained continuation, so neither forbidden shape passes. touches gains kuna-console: #504's chain walk (kuna-console/src/kuna_retcallchain.rs) is the code this extends. The fix-direction risk stands: gate on the popped value originating from an in-function store, not on RET-with-a-computed-destination.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `5ab77f5c`, one image, no ambiguity):

    bin/bm3.zip.__x/bm3.exe (26624 B, sha256 8116e2cb…)

Re-run through `verify --acceptance-suite --need entry-point-ret-dispatch` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    stdout_matches[0] → actual "<no match>"; stdout_matches[1] → actual "<no match>"; stdout_absent[0] → actual "{\n  return;\n}"

This one is notable: it still fails AFTER #504's RET-call chain walk merged, so the
gap at 0x401757 on bm3.exe is live, not a stale filing.

Nothing about the probe's command or expectations was changed — only the target it
resolves against.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
- builder attempt 1 (2026-09-12): the shared RET-call-chain detector now exposes its complete entry walk to the shared decompile step. A default drive seeds CALL overrides only when raw-p-code provenance proves RETURN loaded a slot written by the current run and the adjacent continuation slot stores that RET's own fall-through, retaining the existing instruction/site caps and #504 continuation walk; explicit flow facts are inserted afterwards. The bm3 dataset witness recovers LoadLibraryA, GetProcAddress, and the remaining 105-byte entry body. Acceptance moved to the in-repo `entry_ret_dispatch_i386` twin (`a-96dd05b48b4a`), while the dataset reproduction remains `p-9c278d6e9d91`. Ordinary RET, RET-immediate, copied incoming return-address, slot-computed RET, unrelated constant-target RET, a discarded exact-fall-through push, and an exact-fall-through store to unrelated memory remain returns. Status stays open pending landing and acceptance application.
- attempt-1 review speed (exact `d7734811` release vs candidate, 25 interleaved post-warmup medians): unchanged `ordinary_ret` 428.254 -> 431.019 ms (+0.65%), unchanged `fauxware main` 140.028 -> 144.203 ms (+2.98%), and the three-link recovery 461.836 -> 446.849 ms (-3.25%). Both unchanged controls were byte-identical; all deltas are below the 5% quality budget.
- amended attempt 1 (DIV-168): automatic recognition is now gated by default-on `entryretdispatch`; option off restores the original first-RET termination without disabling explicit flow assertions. Review controls close signed affine displacement, conditional-bypass, and overlapping-register-alias gaps. An explicit `flow 0x804900b return` cleanly vetoes the derived CALL and retains caller precedence instead of asking the engine to apply a redundant RETURN override. The dedicated two-pass stage test, CLI controls, catalog entry, feature bundle, spec text, and regenerated stage baseline record the decision.
- closed: acceptance a-96dd05b48b4a now PASSES at 5d487ef214f5
- round 12 PR bookkeeping: canonical `apply-acceptance` closed this need with `closing_pr: 595`; `pr` is likewise 595. PR #595 remains open and unmerged pending explicit authorization.
