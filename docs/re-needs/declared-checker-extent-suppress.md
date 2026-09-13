---
need_id: declared-checker-extent-suppress
title: Declared checker extent does not suppress interior function-boundary cuts
track: quality
status: closed
severity: major
probe_id: p-9d292ed0d0f9
acceptance_id: a-fed104002ff7
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [6a09581717539b5175d122b9]
rounds: [9]
first_seen_round: 9
attempts: 0
covered_by_option: pdatainterior
touches: [decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_pdatainterior.rs]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/557
closed_in_round: 12
closing_pr: "557"
reject_reason: null
---

## Symptom

Treat 0x140001740 through 0x140002300 as one checker, including its VM handlers.

> **Declared checker extent does not suppress interior function-boundary cuts** (major, `6a09581717539b5175d122b9`)
> Explicit --define-function still emits five funcboundflow truncation warnings, cutting VM operations. Disassembly at the inferred interior entry 0x140001de8 shows arithmetic followed by a jump back to the VM dispatcher.

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
    "sub_140001740",
    "--define-function",
    "0x140001740-0x140002300=check_serial"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "funcboundflow: fall-through reached the next function entry; truncating flow here"
    ]
  },
  "target": {
    "binary_rel": "bin/phantomgate.exe",
    "binary_sha256": "72c8bbb37270b42e75bf9a4b5828e6ec65df37ce92d5eb001b57b69dd7ba4676",
    "binary_size": 230400,
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
    "binary_rel": "bin/phantomgate.exe",
    "binary_sha256": "72c8bbb37270b42e75bf9a4b5828e6ec65df37ce92d5eb001b57b69dd7ba4676",
    "binary_size": 230400,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_140001740",
    "--define-function",
    "0x140001740-0x140002300=check_serial"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "funcboundflow: fall-through reached the next function entry; truncating flow here"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The declared enclosing extent does not supersede inferred interior entries for the boundary gate.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/phantomgate.exe` — Exit 1: Decompiler server exited with status 1 before registering. No reference decompilation was obtained.

## Instances

- `6a09581717539b5175d122b9` (round 9, tester t-r9-6a095817)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, AND THE DIRECTIVE IS NOT INERT -- IT REGISTERS AND ONLY THE EXTENT IS IGNORED (captain in-tick, release binary 8df2a52e, arena .kuna-repipe/arena/9/6a09581717539b5175d122b9/target/phantomgate.exe).

THE FILED CAUSE IS RIGHT: the declared [start,end) does not reach funcboundflow's "next function entry" test. Six discovered entries sit strictly inside the declared 0x140001740-0x140002300 extent (0x140001de8/88, 0x140001e40/344, 0x140001f98/72, 0x140001fe0/64, 0x140002020/648, 0x1400022a8/88) and the cut still fires five times.

THE FOUR-RUN CONTROL TABLE, ONE VARIABLE EACH:
(D) `decompile BIN sub_140001740`                                  -> 488 lines, 5 funcboundflow warns
(P) (D) + `--define-function 0x140001740-0x140002300=check_serial`  -> 488 lines, 5 warns, BYTE-IDENTICAL to (D) -- and the emitted name is still sub_140001740
(N) `decompile BIN check_serial --define-function <same>`           -> 488 lines, 5 warns, but the header IS `unsigned long long check_serial(...)`
(O) (D) + `--option funcboundflow off`                              -> 517 lines, 0 warns  <-- the shape a correct fix must produce

(N) is the load-bearing one: the directive DOES register -- the declared name resolves as a selector and prints -- so this is not a parse or registration failure. The extent simply never reaches the boundary gate. And (O) proves the engine can already produce the untruncated body; what is missing is scoping that behaviour to a declared extent instead of the whole run.

CORRECTION TO THE FILING (affects both probes): selecting by the OLD name in the same run leaves the function named sub_140001740 -- (P) vs (N) differ in the header alone. Both probes select `sub_140001740`, so they exercise the by-old-name path. The warning counts are identical either way so the probes still reproduce, but a builder reading the repro will otherwise conclude --define-function is wholly inert, and it is not. The two selectors answering with different names for one declared function is itself a UX defect worth a separate line at T_TRIAGE.

ACCEPTANCE IS ABSENCE-ONLY AND PASSABLE BY DELETING THE WARNING. The clause is stdout_absent on the funcboundflow message; a one-line change that stops PRINTING the message while still truncating passes it and delivers nothing. This is the same trap flagged on beria-call-fragments-produce and entry-point-ret-dispatch this round. TIGHTEN BEFORE DISPATCH to a positive witness taken from run (O): the fix must RECOVER the body, so require a token that exists only in the 517-line version -- the deeper frame slots (v61..v70 and their `// stack - 0x578` / `- 0x530` / `- 0x500` slot comments) are absent from all 488 truncated lines. Pair "warning absent" with "deep-slot witness present" and the probe cannot be passed by silencing.

WOULD A FIX ON THIS CAUSE PRODUCE WRONG OUTPUT? Not here, if it stays scoped. funcboundflow (DIV-67, #280) removes code on purpose and absorbing a neighbour is normally wrong -- but the whole point of --define-function's documented end bound ("the exclusive end bounds its flow so it stops swallowing its neighbours") is that the user has asserted the extent. Suppressing the cut ONLY for entries strictly interior to a declared extent, and only for the declared function, is the narrow form. A builder must not weaken funcboundflow globally: run (O) is what that looks like and it is already available as an option.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `6a095817`, one image, no ambiguity):

    bin/phantomgate.exe (230400 B, sha256 72c8bbb3…)

Re-run through `verify --acceptance-suite --need declared-checker-extent-suppress` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    stdout_absent[0] → actual "funcboundflow: fall-through reached the next function entry; truncating flow here"

The clause that fails is the presence of
`funcboundflow: fall-through reached the next function entry; truncating flow here` —
a heuristic overriding an extent the analyst declared with `--define-function`.

Nothing about the probe's command or expectations was changed — only the target it
resolves against.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
- closed: acceptance a-fed104002ff7 now PASSES at 02b8ddc2efaf
- REGRESSED: acceptance a-fed104002ff7 fails again at 3dc794bac7dc
- closed: acceptance a-fed104002ff7 now PASSES at 7e151cfaade7
- round 12 reconciliation: the current-main acceptance passed 3/3 at
  `ccc82a37812a1bbfd6c2eac43f1fa6597d631efa`. Every run exited zero with no
  `funcboundflow` truncation warning and retained the recovered 518-line body,
  including the deep frame slots at `-0x578`, `-0x530`, and `-0x500`. The
  one-option control, `--option pdatainterior off`, restored all five warnings
  and shortened the body to 488 lines. This credits the causal implementation,
  PR #557 (squash `02b8ddc2efafc43eef6ac4e72480346ea3b6f846`), not the later
  administrative PR #564. The implementation commit is an ancestor of the
  verified main, and the entire `kuna_pdatainterior.rs` file is still blamed to
  it. Canonical stale-reconciliation metadata therefore keeps `rounds: [9]`
  and `attempts: 0`, records `covered_by_option: pdatainterior`, and closes in
  round 12 with `pr`/`closing_pr` pointing to #557.
