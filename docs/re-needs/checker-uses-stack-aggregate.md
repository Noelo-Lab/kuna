---
need_id: checker-uses-stack-aggregate
title: Checker uses a stack aggregate without declaring it
track: quality
status: closed
severity: major
probe_id: p-0aa9ec21399b
acceptance_id: a-2c0a327051af
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [6a09581717539b5175d122b9]
rounds: [9]
first_seen_round: 9
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/573
closed_in_round: 12
closing_pr: 573
reject_reason: null
---

## Symptom

A declaration for the stack aggregate used in emitted C.

> **Checker uses a stack aggregate without declaring it** (major, `6a09581717539b5175d122b9`)
> Assigns ._0_4_ and ._4_4_ fields of an undeclared local, then uses its address as the VM stack. The probe captures the variable rather than pinning its v-number. All four report probes were replayed successfully; all four acceptances failed.

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
    "stack_aggregate",
    "--option",
    "funcboundflow",
    "off"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\A(?=[\\s\\S]*\\b(v[0-9]+)\\._0_4_\\s*=)(?![\\s\\S]*\\b(?:long|int|char|short|undefined[0-9]+)\\s+\\1\\s*(?:\\[|;))"
    ]
  },
  "target": {
    "binary_rel": "checker_stack_aggregate_x86_64",
    "binary_sha256": "92968f535eaaa3dbf064699e5fcbe1e65ddd6456aa7c46495b26e1d63b9a9160",
    "binary_size": 10808,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/checker_stack_aggregate_x86_64"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "checker_stack_aggregate_x86_64",
    "binary_sha256": "92968f535eaaa3dbf064699e5fcbe1e65ddd6456aa7c46495b26e1d63b9a9160",
    "binary_size": 10808,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/checker_stack_aggregate_x86_64"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "stack_aggregate",
    "--option",
    "funcboundflow",
    "off"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\A(?=[\\s\\S]*\\b(v[0-9]+)\\._0_4_\\s*=)(?![\\s\\S]*\\b(?:long|int|char|short|undefined[0-9]+)\\s+\\1\\s*(?:\\[|;))"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Overlapping stack storage may reference an aggregate whose declaration was suppressed.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6a09581717539b5175d122b9` (round 9, tester t-r9-6a095817)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD ON THE SUPPRESSION HALF, CORRECTED ON THE 'AGGREGATE' AND 'OVERLAPPING' HALVES (captain in-tick, release binary 8df2a52e, arena .kuna-repipe/arena/9/6a09581717539b5175d122b9/target/phantomgate.exe).

THE DEFECT IS EXACTLY ONE VARIABLE, AND IT IS PRINTER-SIDE, NOT MODEL-SIDE. Sweeping the emitted C for declared-vs-used v-names: 69 declared, 70 used, and the difference is the singleton {v12}. Meanwhile 'kuna decompile ... --json' .functions[0].variables lists 71 entries INCLUDING v12 -- {name: v12, type: unsigned long long, kind: stack, stack_offset: -1256, size: 8, 26 line_numbers}. So the model has the variable with a type, a storage slot and every use site; only the declaration LINE is missing. Any fix belongs in the P9 declaration loop, not in stack/symbol recovery. The emitted C is uncompilable as a result: v12._0_4_ = a1 & 0xff ^ v10._0_4_; at line 94 with no declaration anywhere.

CORRECTION 1 -- IT IS NOT AN AGGREGATE. v12 is an 8-byte scalar (unsigned long long) in the model. What makes it LOOK aggregate in the text is that the code writes it at partial widths (._0_4_, ._4_4_) and then uses it as the VM stack base, (&v12)[v62], 29 times.
CORRECTION 2 -- NO STORAGE OVERLAP IS INVOLVED. The neighbour v53 sits at -1264 size 8 and therefore ENDS exactly where v12 begins at -1256. Adjacent, not overlapping.
CORRECTION 3 -- NEITHER PARTIAL-FIELD ACCESS NOR ADDRESS-TAKING IS SUFFICIENT, per an in-function control that costs nothing to re-run. Same function, same run: v10 is partial-accessed (._0_16_, ._0_4_, ._8_8_) AND address-taken 3x and IS declared (char v10[16]); v67 is partial-accessed (._1_3_) and IS declared (unsigned int v67); v11 is address-taken 42x and IS declared. Only v12 is both an 8-byte SCALAR with partial-width writes AND used as an array base. That conjunction is the discriminator a builder should start from.

ABLATIONS, ALL INERT: dedupvardecls off, arraycoverwidth off, and the default path (funcboundflow ON, without the probe's option) all reproduce identically -- 489/517 lines, same singleton {v12}. So this is NOT the dedupvardecls collapse and NOT probe-specific; the probe's 'funcboundflow off' is incidental and the defect is present in stock output.

WHERE TO LOOK, AND THE ONE THING THE BUILDER MUST DISCRIMINATE FIRST. printc.rs:2694-2760 holds two 'STUB A' skip arms in the declaration walk -- is_proto_partial_piece and is_scalar_partial_piece -- each of which does 'continue' on the belief that a WHOLE-cover sibling high of the same name emits the single declaration (the port of C++ emitScopeVarDecls' getFirstWholeMap()). v12's shape (strict partial covers of a scalar mapped Symbol) is precisely what is_scalar_partial_piece targets, so the leading mechanism is: the partial is skipped and the whole-cover sibling never emits. I could NOT confirm that arm actually fires without instrumenting the build, so treat it as a lead, not a finding. The competing mechanism, which the filed 'aggregate' wording may be gesturing at and which I could not eliminate: the same JSON lists v52 as unsigned long long[20] -- a 160-byte array, declared in the C, with ZERO line_numbers and ZERO addresses. A 20-entry u64 array is exactly the shape of the VM stack that (&v12)[i] walks. If v12's Symbol is a piece of THAT array, the aggregate is being declared at the wrong slot (-224) under the wrong name while its only real user goes undeclared. The two mechanisms have different fixes; decide between them with one instrumented run before writing any code.

ACCEPTANCE IS SOUND, WHICH IS RARE THIS ROUND -- DO NOT 'SIMPLIFY' IT. The clause is a single anchored regex with a backreference: it requires a 'vN._0_4_ =' to EXIST and then requires that no 'type vN;' declaration for that same N exists. Because it is conditioned on the use, it cannot be passed by deleting the code that uses v12 -- the beria/entry-point absence-only trap does not apply here. A builder must not replace it with a plain stdout_absent of '._0_4_'.

REGRESSION RISK FOR THE FIX: the skip arms exist to stop a wall of duplicate declarations for one tied scalar Symbol (LOSS-245, and the composite register-return piece case). Loosening either arm re-opens that, so the builder should measure the 675-assertion datatest corpus early and prefer a fallback ('emit the declaration if no sibling actually emitted one') over widening the predicate.

SIBLING: declared-checker-extent-suppress is from the SAME challenge and the same function family but is a different gap (funcboundflow interior cuts vs a missing declaration line) -- do not fold them; they can be built independently.

FILING ARTIFACT worth fixing at T_TRIAGE: this need doc's '## Symptom' section is duplicated verbatim (heading and body both appear twice). declared-checker-extent-suppress has the same duplication, so it is a cluster.py filing bug on this round's docs, not a one-off.

### 2026-09-08T06:05Z — captain, round 9 B_DRAIN: acceptance target bound

The acceptance probe carried no `target`, so the suite could not resolve `{{BIN}}` and
reported `ProbeError: {{BIN}} used but the context supplies no bin` — indeterminate rather
than failing, and therefore unclosable by B_DONE however good a fix was. Bound to the
challenge's own image (challenge `6a095817`, one image, no ambiguity):

    bin/phantomgate.exe (230400 B, sha256 72c8bbb3…)

Re-run through `verify --acceptance-suite --need checker-uses-stack-aggregate` on the current build it now RUNS
and FAILS, on the clause that carries the filed symptom:

    stdout_absent[0] → actual ""

Nothing about the probe's command or expectations was changed — only the target it
resolves against.
- round 10 CAPTAIN (18:45Z tick, B_DRAIN off-critical-path repair): REPRODUCTION ARM TARGET BOUND. This need's `## Acceptance` arm was already bound but its `## Reproduction` arm carried no `target` block, so the arm that is supposed to assert the bug EXISTS TODAY could not run (`ProbeError: {{BIN}} used but the context supplies no bin`). That never blocked B_DONE -- closure is gated on the ACCEPTANCE arm alone, measured -- but it blinded the refuter and the builder's reproduction step. Repaired by COPYING this need's own acceptance target verbatim, which is a copy and not a guess: the need has exactly one `challenges:` entry, so both arms necessarily address the same binary, and verify.resolve_binary treats a binary_sha256 mismatch as a hard stop. Measured after the write: the reproduction arm runs (unrunnable=false) and PASSES -- a passing reproduction arm means the defect still reproduces on main at b23e01ec, so this doubles as a re-confirmation that the symptom is live. Probe ids did NOT move (probe_id_of is keyed on cmd+expect only, needs.py:269; all 32 arms across the 16 repaired needs re-hashed byte-identically). PROMOTION CAVEAT unchanged: binary_source is `dataset` and `verify --promote` refuses that verbatim -- vendor an in-repo fixture in the SAME PR or B_DONE closes the need but cannot promote its probe.
- round 12 BUILDER: instrumented the P9 declaration walk and found that the full-width same-name sibling was a constant-only address reference, not storage. `high_name_has_scalar_whole_sibling` now requires a non-constant first member before suppressing partial declarations. Retargeted both probes to the vendored DWARF minimizer; the anchored backreference reproduces on the clean binary and clears on the repair. An exact before/after sweep covered all 274 XML corpus files plus all 1,100 decompilable functions in `phantomgate.exe`: core corpus output was byte-identical, the stage corpus changed only the minimizer, and the challenge changed only `sub_140001740` and `sub_1400271c0`; every hunk adds the one declaration for an already-used local and changes no body statement.
