---
need_id: success-report-callback-fails
title: Success-report callback fails with an internal missing-op error
track: quality
status: closed
severity: major
probe_id: p-4bf0b7ef2c26
acceptance_id: a-13a141bf0db7
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [5ab77f5c33c5d40ad448c615]
rounds: [10, 12]
first_seen_round: 10
attempts: 1
covered_by_option: funcboundflow
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 605
closed_in_round: 12
closing_pr: "605"
reject_reason: null
---

## Symptom

Decompile the callback selecting the accepted or rejected message.

> **Success-report callback fails with an internal missing-op error** (major, `5ab77f5c33c5d40ad448c615`)
> Exits 1 with 'Could not find op at target address: (ram,0x080488de)' and emits no C. A branch assertion does not avoid the error.

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
    "0x8048820",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "Could not find op at target address"
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
    "0x8048820",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "Could not find op at target address"
    ],
    "stdout_bytes": {
      "gt": 0
    }
  },
  "target": {
    "binary_rel": "bin/trace-p.zip.__x/crackme",
    "binary_sha256": "7b8a32c3cc0e9816bfe03d2e7c4aa5a596a21e43a6e35dc43cb17fa5867138ec",
    "binary_size": 5208,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Overlapping junk instructions may interact incorrectly with function-boundary truncation.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5c33c5d40ad448c615` (round 10, tester t-r10-5ab77f5c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER: hypothesis **overturned** (was inconclusive). REFUTER RAN (captain, in-tick, 2026-09-08). OVERTURNED: the filed cause names OVERLAPPING JUNK INSTRUCTIONS interacting with boundary truncation. The overlap half is provably innocent and the truncation half is the whole defect. (1) Repro on .kuna-repipe/arena/10/5ab77f5c33c5d40ad448c615/target/trace-p.zip.__x/crackme: 'kuna decompile BIN 0x8048820 --addr' -> exit 1, 'Could not find op at target address: (ram,0x080488de)'. (2) --option overlapbranch off: STILL FAILS, identical error. --option funcboundflow off: EXIT 0, full C. So the culprit is funcboundflow, not overlap handling; a builder who hardens the overlap path fixes nothing. (3) Where 0x80488de comes from: 0x8048825 JNZ 0x804885d jumps over an embedded string at 0x8048827..0x804885c; the never-taken fall-through decodes that string as code and one junk instruction, 0x8048845 '657374' JNC 0x80488bc, branches into the MIDDLE of the real MOV EDI,[EDI+0x20] at 0x80488bb. Decoding at 0x80488bc yields '7f20' JG 0x80488de. The junk only SUPPLIES the edge -- it is not what mishandles it. (4) THE MECHANISM, pinned in code. 'kuna functions' reports sub_8048820 size 133 = extent 0x8048820..0x80488a5, and 0x80488a5 is the entry of sub_80488a5, so funcboundflow (flow.rs:1585 is_funcbound_fallthru -> plant artificial RETURN, flow.rs:1602 warning) cuts fall-through at 0x80488a5. 0x80488de is past the cut and reachable only by that branch, so it stays in 'unprocessed'. fillin_branch_stubs (flow.rs:1838) DOES build a halt stub for it, but registers it in 'visited' ONLY under 'if self.outofbounds.contains(&addr)' (flow.rs:1855). 'outofbounds' is populated exclusively by the declared-extent range check (flow.rs:910/928/1782); the funcboundflow truncation never inserts into it. So target() (flow.rs:788) finds no op and the whole function dies. (5) THE CONTROL THAT SETTLES IT, one command, no harness: declare the IDENTICAL boundary funcboundflow picks -- 'kuna decompile BIN 0x8048820 --addr --define-function 0x8048820-0x80488a5' -> EXIT 0 with '// warn: Function flows out of bounds'. Same cut address, opposite outcome: the declared path registers the stub, funcboundflow does not. (6) This is a KNOWN, ALREADY-CLOSED defect class on the other path -- kuna-console/tests/verify_funcbounds.rs:145 'a_declared_end_a_branch_targets_clips_instead_of_aborting' documents the same 'Could not find op at target address' abort for a declared end that cut a branch, and the flow.rs:1855 guard is that fix. funcboundflow was never extended to it. FOR THE BUILDER: this is a strict wrong-output/hard-error bug fix, no new flag (AGENTS.md); the likely shape is to record a funcboundflow truncation's cut addresses the same way the declared range does so fillin_branch_stubs registers the stub, keeping funcboundflow default-ON. Do NOT ship 'funcboundflow off'. Expect the clipped body to carry the out-of-bounds warning, as the --define-function control already shows.
- round 10 T_TRIAGE (captain, 10:39Z tick): RETRACKED tooling -> quality, touches kuna-cli -> kuna-decomp. The filed track came from the symptom being a CLI exit-1, but the refuter pinned the defect to funcboundflow in p2_lift/flow.rs (flow.rs:1585/1602): '--option funcboundflow off' exits 0 with full C, '--option overlapbranch off' still fails. Nothing in kuna-cli is involved. This also puts the need on the quality lease set, which is what serialises it against its two extent-vs-flow siblings (default-function-boundary-truncates, function-inventory-omits-dialog) -- they pull the same knob in OPPOSITE directions and must not be built concurrently.
- round 10 CAPTAIN (18:35Z tick, B_DRAIN off-critical-path repair): PROBE TARGET BOUND -- this need was STRUCTURALLY UNCLOSABLE as filed. Both arms carried no `target` block, so `verify --acceptance-suite` returned `ProbeError: {{BIN}} used but the context supplies no bin` with unrunnable=true / transition=indeterminate: a builder could have done the work and still not closed it. Bound both arms to the single ELF/PE candidate under ds/challenges/<hexid>/ (sha256 + size captured from the dataset copy, binary_source=dataset). Verified post-fix: unrunnable=false, passed=false, transition=unchanged -- i.e. the acceptance now FAILS honestly and can be seen to flip. NOTE probe ids did NOT move: probe_id_of() is keyed on cmd+expect only (needs.py:269), so binding a target is id-stable and breaks no historical reference. PROMOTION CAVEAT: binary_source is `dataset` and `verify --promote` refuses that verbatim (CI has no dataset, --force does not help) -- vendor an in-repo fixture in the SAME PR or B_DONE cannot promote this probe into tests/cli/.
- closed: acceptance a-13a141bf0db7 now PASSES at d2860b8574b2707ceaf4b6e7f7b8a9108e6c0d0c
