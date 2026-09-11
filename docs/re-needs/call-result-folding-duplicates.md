---
need_id: call-result-folding-duplicates
title: Call-result folding duplicates a random-number call inside the uniqueness loop
track: quality
status: closed
severity: major
probe_id: p-4d929322cb94
acceptance_id: a-ac834a820d26
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [661daf77cddae72ae250c836]
rounds: [10]
first_seen_round: 10
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: 12
closing_pr: null
reject_reason: null
---

## Symptom

Preserve one random-number draw per outer iteration, reusing its result for comparisons and the array store.

> **Call-result folding duplicates a random-number call inside the uniqueness loop** (major, `661daf77cddae72ae250c836`)
> Default C embeds the same call in the inner-loop comparison and subsequent array assignment. Assembly calls once at 0x1400029c0, repeatedly compares AX at 0x1400029d8, then stores AX at 0x1400029e8. Both initialization loops are affected. Argument-recovery options leave this unchanged; foldcallret off restores a temporary before each inner loop.

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
    "sub_140002930"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "if \\([^\\n]*== \\(unsigned short\\)([A-Za-z_][A-Za-z_0-9]*)\\(\\)\\)[\\s\\S]*?\\]\\s*=\\s*\\(unsigned short\\)\\1\\(\\);"
    ]
  },
  "target": {
    "binary_rel": "callretmulti_x86_64",
    "binary_sha256": "8c9139dd5b2083a6bc27bcffa08d3ad4da3d4688710292c5f9e4dba823707a26",
    "binary_size": 9040,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/callretmulti_x86_64"
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
    "sub_140002930"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "if \\([^\\n]*== \\(unsigned short\\)([A-Za-z_][A-Za-z_0-9]*)\\(\\)\\)[\\s\\S]*?\\]\\s*=\\s*\\(unsigned short\\)\\1\\(\\);"
    ]
  },
  "target": {
    "binary_rel": "callretmulti_x86_64",
    "binary_sha256": "8c9139dd5b2083a6bc27bcffa08d3ad4da3d4688710292c5f9e4dba823707a26",
    "binary_size": 9040,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/callretmulti_x86_64"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- foldcallret appears to propagate a call expression into multiple uses across a loop.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/vm_obfuscation.exe` — Reference unavailable: server exited status 1 before registering. Kuna disassembly and option-off output independently confirm the defect.

## Instances

- `661daf77cddae72ae250c836` (round 10, tester t-r10-661daf77)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER: hypothesis **upheld** (was inconclusive). REFUTER RAN (captain, tick 10:05Z). Filed cause -- 'foldcallret appears to propagate a call expression into multiple uses across a loop' -- is UPHELD, and it is worse than the wording suggests: this is a SEMANTICS CHANGE, not a presentation defect. GROUND TRUTH, kuna disassemble .kuna-repipe/arena/10/661daf77cddae72ae250c836/target/vm_obfuscation.exe 0x1400029b0 --count 16: ONE call at 0x1400029c0 (CALL 0x14000d018), the sign-fixup at 0x1400029c5..0x1400029d3, then an inner loop 0x1400029d8..0x1400029e6 (CMP word ptr [RSP+RCX*2+0x62],AX / INC RCX / CMP RCX,0x10 / JL back to 0x1400029d8) that re-reads AX WITHOUT re-calling. AX is loop-invariant; the machine draws once. DEFAULT OUTPUT emits it TWICE, once inside the loop: 'do { if (v8[v4] == (unsigned short)sub_14000d018()) goto label_1400029a1; v4 += 1; } while (v4 < 0x10); v8[v6] = (unsigned short)sub_14000d018();' -- so the C makes up to 17 RNG draws where the binary makes 1, compares against a different value each iteration, and stores a value that was never compared. Both initialization loops are affected. --option foldcallret off collapses both to a single 'v4 = sub_14000d018();' and the probe regex stops matching, which isolates the pass beyond doubt (/tmp/fold_def.c matches, /tmp/fold_off.c does not). *** THE DECISIVE POINT FOR THE BUILDER: kuna IS VIOLATING ITS OWN DOCUMENTED OPTION CONTRACT, SO THIS NEEDS NO NEW FLAG. *** docs/options.md:998 defines foldcallret as 'Fold an order-safe SINGLE-USE call return value into its use site' and its symptoms row says 'call result spilled to a temp USED EXACTLY ONCE'; it further promises 'Only folds when the single use is in the same block with no intervening call/load/store'. Here the value has TWO use sites in TWO different blocks, one of them a loop body. Per AGENTS.md a strict bug fix that only corrects wrong output ships without an option -- the guard to enforce is the one already written down: fold only at a single use site, and never across a back-edge. DO NOT ship 'foldcallret off' as the fix; the option is default-ON under DIV-14 and turning it off regresses the presentation win it was measured for. Guard the fold, keep the default.
- round 10 CAPTAIN (18:35Z tick, B_DRAIN off-critical-path repair): PROBE TARGET BOUND -- this need was STRUCTURALLY UNCLOSABLE as filed. Both arms carried no `target` block, so `verify --acceptance-suite` returned `ProbeError: {{BIN}} used but the context supplies no bin` with unrunnable=true / transition=indeterminate: a builder could have done the work and still not closed it. Bound both arms to the single ELF/PE candidate under ds/challenges/<hexid>/ (sha256 + size captured from the dataset copy, binary_source=dataset). Verified post-fix: unrunnable=false, passed=false, transition=unchanged -- i.e. the acceptance now FAILS honestly and can be seen to flip. NOTE probe ids did NOT move: probe_id_of() is keyed on cmd+expect only (needs.py:269), so binding a target is id-stable and breaks no historical reference. PROMOTION CAVEAT: binary_source is `dataset` and `verify --promote` refuses that verbatim (CI has no dataset, --force does not help) -- vendor an in-repo fixture in the SAME PR or B_DONE cannot promote this probe into tests/cli/.
- round 12 BUILDER: retargeted both probes without changing their command or expectation to the vendored `callretmulti_x86_64` minimizer. It preserves the witness's CALL / sign-fixup / low-word fan-out shape in 72 text bytes, reproduces the duplicate-call default on the parent tree, and is available to CLI CI.
