---
need_id: declaring-double-score-return
title: Declaring a double score return disconnects the log result from its accumulator
track: quality
status: closed
severity: major
probe_id: p-ce0e609fb71c
acceptance_id: a-cc4fdb4b53b3
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [69a54bd70f5b9757a6a5f72f]
rounds: [5, 12]
first_seen_round: 5
attempts: 1
covered_by_option: calleeretpreserves
touches: [decompiler/crates/kuna-decomp/src/p4_calls]
scope: small
regression_of: null
pr: 599
closed_in_round: 12
closing_pr: "599"
reject_reason: null
---

## Symptom

Recover the score with correct double prototypes while preserving the accumulated log result.

> **Declaring a double score return disconnects the log result from its accumulator** (major, `69a54bd70f5b9757a6a5f72f`)
> With only the address-bound log prototype, kuna retains log in the accumulator but declares the score void. Adding the correct double score prototype makes log a discarded expression and leaves uninitialized XMM fragments in its consumers. Typing the cookie helper and enabling calloverlap full do not repair this. Disassembly shows CALL at 0x140003452 immediately followed by ADDSD XMM6,XMM0. The solve remains partial because the executable requires model.nli, which is absent from the supplied target.

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
    "0x140003150",
    "--addr",
    "--assert",
    "prototype 0x14000904f double log(double x)",
    "--assert",
    "prototype 0x140008140 void cookie(unsigned long long c)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^\\s*log\\s*\\("
    ]
  },
  "target": {
    "binary_rel": "bin/Release.zip.__x/PolyMLP.exe",
    "binary_sha256": "01652e8b03e2bec127f5726320db4c9925d495d75583a8bb4534ac640f6ec230",
    "binary_size": 54272,
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
    "0x140003150",
    "--addr",
    "--assert",
    "prototype 0x14000904f double log(double x)",
    "--assert",
    "prototype 0x140008140 void cookie(unsigned long long c)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "(?m)^\\s*log\\s*\\("
    ],
    "stdout_matches": [
      "(?m)^\\s*\\w+\\s*=\\s*[^;\\n]*\\blog\\s*\\(",
      "\\+\\s*log\\s*\\(|\\blog\\s*\\([^;\\n]*\\)\\s*\\+"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_double_score_return_x86_64.exe",
    "binary_sha256": "3cf5307ef9c572400d67db3556f5f4bfc5e9483d2ae1b787f4f34ed494eaed19",
    "binary_size": 3584,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_double_score_return_x86_64.exe"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Locking the enclosing floating return may change XMM heritage partitioning and break nested call-output recovery.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/Release.zip.__x/PolyMLP.exe` — Server exited with status 1 before registering. No reference pseudocode was obtained.

## Instances

- `69a54bd70f5b9757a6a5f72f` (round 5, tester t-r5-69a54bd7)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **overturned** (was inconclusive). REFUTED IN-TICK (captain, round 5) on .kuna-repipe/arena/5/69a54bd70f5b9757a6a5f72f/target/Release.zip.__x/PolyMLP.exe with the release kuna at 38c461d1. NINE runs, one variable changed at a time.

VERDICT: OVERTURNED. The hypothesis blames the enclosing floating return ("locking the enclosing floating return may change XMM heritage partitioning and break nested call-output recovery"). I KILLED IT BY EXPERIMENT, and the real trigger is something nobody suspected: A PROTOTYPE ASSERTION ON A THIRD, UNRELATED CALLEE.

THE MINIMAL BROKEN PAIR — two commands, one assertion apart, and this is the whole need:
  GOOD  kuna decompile <bin> 0x140003150 --addr \
          --assert "prototype 0x14000904f double log(double x)"
        ->  v3 = v5 + log(v3 + v6);            <- accumulator INTACT, correct
  BAD   ... the same, plus one assertion on an unrelated function:
          --assert "prototype 0x140008140 void cookie(unsigned long long c)"
        ->  log(*(double *)((long long)v36 + (long long)v6 * 8) + v4);   <- RESULT DISCARDED
0x140008140 is called at 0x14000351f, LATER in the same function; the log call is at 0x140003452, and the disassembly there is exactly what the tester reported: `CALL 0x14000904f` / `ADDSD XMM6,XMM0`. So one prototype assertion on a callee that runs AFTER the accumulation destroys the accumulation that happened BEFORE it.

WHAT IS INNOCENT — each proven by a run, do not re-test these:
  * THE ENCLOSING `score` RETURN LOCK. `--assert "prototype 0x140003150 double score(void *model,void *password)"` added to the GOOD command keeps the accumulator intact (`v3 = v5 + log(v3 + v6);`). THE HYPOTHESIS IS DEAD. It only looked guilty because the tester never varied it alone.
  * `--option calloverlap full`. Added to the GOOD command: intact.
  * `--option calleepreserves off` (the killed-set narrowing pass from #434). Added to the GOOD command: still intact — so that pass is not what preserves XMM6 here either.
  * THE SHAPE OF THE cookie PROTOTYPE. `unsigned long long cookie(unsigned long long c)` breaks it. `void cookie(void)` breaks it. It is not void-ness, not the return type and not the parameter list: ANY prototype assertion on 0x140008140 breaks it.

WHERE TO LOOK. XMM6 is NON-VOLATILE on Windows x64 and it is the accumulator (`ADDSD XMM6,XMM0`). The unlocked call at 0x14000351f evidently leaves XMM6 alone; the locked one does not. The question to answer FIRST is whether locking a prototype replaces that call spec's effect / killedbycall set with the prototype model's default and thereby drops a preserved-register answer kuna had derived — i.e. does an asserted prototype arrive carrying an EMPTY effect list that then reads as "kills everything the model kills"? Same family as #434 (`calleepreserves`: the ABI's killed set is an answer about the CONVENTION, not about this callee), but reached through the assertion plane rather than through the cspec, and #434's own prose says "a proto carrying its own effect override is left alone" — check what an asserted proto carries.

WHAT THIS DOES TO THE NEED, AND IT MATTERS FOR DISPATCH:
  * IT IS MIS-TRACKED. Filed `quality`, but the defect is an ASSERTION causing wrong output elsewhere in the function — the same plane b-r5-rejected-flow-ov is live in. It is not the same cluster and not the same file, but a builder must not be sent at this while that one is mid-flight without reading its branch first. Re-track at T_TRIAGE.
  * THE FILED PROBE IS NOISE. It carries three assertions and `--option calloverlap full`, of which exactly one assertion matters. Rewrite the reproduction probe to the two-command minimal pair above before dispatch; the acceptance should demand the accumulated form (`= ... + log(`) rather than merely the absence of a bare `log(` statement, or a builder can pass it by suppressing the statement entirely.
  * THE tester's "the executable requires model.nli, which is absent" caveat is IRRELEVANT to this probe — it is a static decompile, it runs, exit 0, and the arena binary is present (build.json primary target/Release.zip.__x/PolyMLP.exe, 54272 bytes). Nothing here is blocked on the missing data file.

ONE MORE THING, FREE AND UNFILED: with NO assertions at all the same call prints `log(SUB84(*(double *)(...) + v6,0))` — the double argument truncated to 4 bytes by a SUB84 that has no business being there. Different symptom, probably the same locked-vs-unlocked seam; worth one command to re-check after this is fixed.
round 5 TRIAGE (captain): PROBE AND ACCEPTANCE REWRITTEN to the refuter's minimal pair, TOUCHES -> p4_calls, TRACK STAYS quality. The filed probe carried three assertions and `--option calloverlap full`, of which exactly one assertion matters; both the `score` return lock and calloverlap were proven innocent by their own runs. The probe is now the two-assert BAD command and nothing else. THE ACCEPTANCE NOW DEMANDS THE ACCUMULATED FORM, not merely the absence of a bare statement: stdout_absent ^\s*log\s*\( plus stdout_matches for log's result being assigned AND participating in an addition, so a builder cannot pass by suppressing the call. I RAN BOTH SIDES ON THE ARENA BINARY AT 38c461d1 BEFORE WRITING THIS: the acceptance FAILS on today's output (bare `log(...)` statement, result assigned = false) and PASSES on the one-assert output (`v3 = v5 + log(v3 + v6);`), so it is a real, currently-failing acceptance. The regexes are rename-tolerant -- they never name v3/v5/v6. ON TRACK: the refuter called this 'mis-tracked', and I am deliberately NOT flipping it to tooling. The TRIGGER is the assertion plane but the DEFECT and its fix are in kuna-decomp (what effect/killed set an asserted prototype arrives carrying -- the same family as #434 `calleepreserves`), so it will need the quality counter leases if it ships an option; tooling would hand it the wrong lease set. The assertion-plane overlap with b-r5-rejected-flow-ov is a DISPATCH-ORDERING constraint, not a track: do not dispatch this until that branch is read.
round 5 TRIAGE (captain): probe/acceptance ids recomputed after the body rewrite -- probe p-b79ccff56b39 -> p-ce0e609fb71c, acceptance a-2a4aa2a07be7 -> a-cc4fdb4b53b3. The id is a content hash and is the opportunity selector, so a rewritten body with a stale id would have carried the old selector into apply-acceptance.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
- Builder implementation on `342eaa03`: the exact MSVC cookie proof now preserves only the caller's logical ABI output slice across a locked-void checker, while direct output writes, output-space STOREs (including joined pieces), incomplete positive evidence, and non-output XMM flanks remain conservative. All eligible sites are collected before one restart, including more than eight sites.
- The authoritative two-assert acceptance `a-cc4fdb4b53b3` passes unchanged on the SHA-bound PolyMLP.exe target; `calleeretpreserves off` restores the bare `log(...)` statement and fails both positive clauses. Its `cmd` and `expect` are unchanged here. Only the acceptance target was rebound to an in-repo PE twin that mirrors the loop-fed XMM6 accumulation, post-log volatile calls, final divide, XMM0 copy, and cookie check, so the same acceptance identity is promoted into `tests/cli/`.
- The need remains `open` with `pr: null` and `attempts: 0` until integration; no pre-PR closure metadata is claimed.
- closed: acceptance a-cc4fdb4b53b3 now PASSES at c4bac7552434
