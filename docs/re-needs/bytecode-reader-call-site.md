---
need_id: bytecode-reader-call-site
title: Bytecode-reader call site omits all four arguments with recovery options on
track: quality
status: open
severity: major
probe_id: p-2b88bd39be37
acceptance_id: a-f585903e8921
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [673da52e9b533b4c22bd2eeb]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: argument-recovery-knobs-still
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover context, buffer, length, and flags passed at 0x9d9d8.

> **Bytecode-reader call site omits all four arguments with recovery options on** (major, `673da52e9b533b4c22bd2eeb`)
> Both recovery options leave a zero-argument call. Kuna disassembly shows incoming RDI/RSI/RDX and ECX=1; decompiling the callee recovers four parameters.

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
    "sub_9d9c0",
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
      "(?m)^\\s+sub_[0-9a-f]+\\(\\);"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.x86_64.elf",
    "binary_sha256": "19351f6b9a6b8af7399470a3ff22ddf1b2b8f44a85f1e0497cbb6e4ed20fbb8a",
    "binary_size": 977264,
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
    "sub_9d9c0",
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
      "(?m)^\\s+sub_[0-9a-f]+\\(\\);"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.x86_64.elf",
    "binary_sha256": "19351f6b9a6b8af7399470a3ff22ddf1b2b8f44a85f1e0497cbb6e4ed20fbb8a",
    "binary_size": 977264,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `673da52e9b533b4c22bd2eeb` (round 8, tester t-r8-673da52e)

## Decision log

- filed by cluster.py from 1 observation(s)
- captain T_DEDUP (r8): SPLIT out of the same 4-member bucket, and RETITLED by the captain. The tester's title was "Argument-recovery options still omit all four bytecode-reader arguments", whose 4-word slug is `argument-recovery-options-still` — an EXISTING need, closed in round 4 by PR #455 (option `calleearitylive`). Filing under that id would have (a) flipped a closed need to `regressed` and sent it to the front of select()'s queue, (b) kept #455's graphy acceptance probe, and (c) silently DISCARDED this sighting's own probe and acceptance, because build_needs bumps instances/challenges only and never rewrites sections. I replayed #455's acceptance on this build first: `verify --acceptance-suite --need argument-recovery-options-still` at sha bfacf0c6 = pass 1 / fail 0 / regressed []. So #455's capability is intact and this is a case it never covered, NOT a regression of it. The observation's own `regression_of: argument-recovery-knobs-still` is the tester's advisory pointer and was left as filed.
- round 8 TRIAGE (captain): SIBLING NOTICE, not a merge. bytecode-reader-return-discarded is the same call in the same function of the same binary (Layers, sub_9d9c0 calling sub_875e0): this need is the four arguments vanishing at the call site, that one is the RAX/RDX return being dropped on the floor. Both are what an undeclared internal callee's prototype failing to reach its call site looks like from the two sides, and the tester reached the second one only after asserting the parameters by hand for this one. Their acceptances do not conflict (one wants arguments printed, the other wants the return consumed), so a builder that finds one mechanism may honestly close both in one PR -- check the sibling's acceptance before opening a second. Kept separate because either could turn out to be its own defect.
