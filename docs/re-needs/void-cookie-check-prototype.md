---
need_id: void-cookie-check-prototype
title: Void cookie-check prototype leaves main returning an undefined EAX local
track: quality
status: closed
severity: major
probe_id: p-ec5e7370c859
acceptance_id: a-f15210e51172
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [68b94ba38fac2855fe6fbada]
rounds: [11, 12]
first_seen_round: 11
attempts: 1
covered_by_option: calleeretpreserves
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 607
closed_in_round: 12
closing_pr: "607"
reject_reason: null
---

## Symptom

Preserve main's explicit exit codes across its RAX-preserving cookie helper, ideally with an interface to declare callee-preserved registers.

> **Void cookie-check prototype leaves main returning an undefined EAX local** (major, `68b94ba38fac2855fe6fbada`)
> Default output returns the cookie helper's result despite separately decompiling that helper as void. A void prototype override instead leaves an unassigned EAX local returned on normal exits. Argument-recovery options did not repair the default symptom. Disassembly shows EAX set to 0 at 0x140001b8f or 1 at 0x140001bed before the shared cookie call.

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
    "sub_1400012f0",
    "--assert",
    "prototype sub_140001c10 void security_check_cookie(unsigned long long cookie)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "unsigned int (v[0-9]+); // eax(?:(?!\\b\\1\\s*=)[\\s\\S])*return \\1;"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme1.exe",
    "binary_sha256": "16512154b0c419b2e4b064dbabd4932251ec5e0e11e14302d86821dca1ade1f0",
    "binary_size": 16384,
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
    "sub_1400012f0",
    "--assert",
    "prototype sub_140001c10 void security_check_cookie(unsigned long long cookie)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "unsigned int (v[0-9]+); // eax(?:(?!\\b\\1\\s*=)[\\s\\S])*return \\1;"
    ],
    "stdout_matches": [
      "return\\s+0\\s*;",
      "return\\s+1\\s*;",
      "(?m)^\\s*sub_140001c10\\(",
      "(?s)sub_140001c10\\([^\\n]*\\);[^\\n]*\\n\\s*return\\s+1\\s*;.*sub_140001c10\\([^\\n]*\\);[^\\n]*\\n\\s*return\\s+1\\s*;.*sub_140001c10\\([^\\n]*\\);[^\\n]*\\n\\s*return\\s+0\\s*;"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_void_cookie_multi_exit_x86_64.exe",
    "binary_sha256": "16512154b0c419b2e4b064dbabd4932251ec5e0e11e14302d86821dca1ade1f0",
    "binary_size": 16384,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_void_cookie_multi_exit_x86_64.exe",
    "selector": "sub_1400012f0",
    "selector_kind": "name"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The call model treats RAX as clobbered despite the helper preserving it on its returning path.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/crackme1.exe` — Server acec83e148 exited with status 1 before registering. No reference pseudocode was obtained.

## Instances

- `68b94ba38fac2855fe6fbada` (round 11, tester t-r11-68b94ba3)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **inconclusive**. REFUTER (captain, in-tick, round 11). VERDICT: inconclusive -- a refuter DID run and settled one question decisively, but did not reach the mechanism. Recording the scope honestly rather than leaving the filed default.

1. WHAT WAS SETTLED: THIS IS NOT A REGRESSION, AND IT MUST NOT BE FILED AS ONE. There are three cookie needs in the backlog and it is easy to conflate them. main-returns-invented-cookie (rounds [6], hypothesis overturned) was CLOSED in round 8 by PR #494 against the in-repo fixture decompiler/crates/kuna-analysis/tests/fixtures/pe_cookiecheck_x86_64.exe. I replayed that fixture on current main (d6105e3d): kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/pe_cookiecheck_x86_64.exe 0x140001000 --addr emits sub_140001080(v2 ^ (unsigned long long)v1); return 0; -- the literal return 0 the acceptance demands, with no return sub_XXXX(...) shape anywhere. #494 HOLDS. So this round-11 need is a case #494 never covered, on a different binary (68b94ba3, Patch protect v2, PE x86-64 16 KB), not a regression of it. Do not set regression_of and do not let it outrank the queue on that basis. Recall that regression_of is set from TITLES elsewhere in this pipeline, so a title-similar close is exactly the trap here.

2. WHY THE #494 ACCEPTANCE DID NOT CATCH THIS SHAPE, which is the useful part for whoever builds it. That acceptance is stdout_absent of return\s+sub_[0-9a-fA-F]+\([^;]*\);\s*\} plus stdout_matches of return\s+(?:0|0x0)\s*;. It forbids returning the CALL and requires a literal zero to appear somewhere in the output. This round's symptom is main returning an UNDEFINED EAX LOCAL -- a return vN; with vN never assigned. That shape trips neither clause: it is not a return sub_...() , and the positive clause is satisfied by any function in the output that returns 0. If you build a fix here, do not copy that acceptance pattern; require the returned expression itself to be the preserved zero, not merely that some line in the file says return 0.

3. THE THIRD MEMBER OF THE FAMILY, for context: msvc-cookie-removal-changes (rounds [6], status open, hypothesis_status overturned) is still open and unbuilt. Read its Decision log before starting -- it already carries an overturned diagnosis on adjacent machinery and will save re-deriving it.

4. WHAT I DID NOT DO, stated plainly so nobody reads more into this verdict than it carries. I did not reproduce the round-11 probe on the 68b94ba3 arena binary and did not test the need's own claim that an explicit void prototype assertion is ignored. The hypothesis is therefore UNJUDGED, not upheld and not overturned. Round 6's work on this family is the strongest lead: per that round's refutation, the return register is never narrowed and guardCalls re-promotes it to killedbycall, so the callee-preserves option everyone reaches for cannot apply and the fix has to keep the caller's pre-call RAX alive instead. Check whether that carve-out explains the undefined-EAX shape before spending a builder.

- round 12 CLOSURE (PR #607): acceptance `a-f15210e51172` closes on the byte-faithful 16,384-byte fixture (`sha256:16512154b0c419b2e4b064dbabd4932251ec5e0e11e14302d86821dca1ade1f0`). The root cause was the exact `/GS` cookie walk collapsing an inner all-backedge `MULTIEQUAL` to `No`; propagating `Backedge` lets the outer fixed point reach its concrete stack-cookie seed while seedless, unknown, and conflicting-offset shapes still refuse. The existing default-on `calleeretpreserves` path preserves the caller's exact EAX literals, and final-C presentation folds only an adjacent literal assignment plus the complete standalone checker statement whose markup identifies that exact P7-marked call occurrence plus return of the same local. The checker remains present; same-name unmarked calls and all ambiguous shapes decline. The default acceptance emits the three exact `return 1;`, `return 1;`, and `return 0;` tails, while the option-off control restores the undefined-EAX local. Rebased on `fe3d668e7b2b1d2cbb213eef709924f784d5cbc3`; gates are 675/675 core, 813/813 stages, 153/153 CLI, 6696 passed / 0 failed / 38 ignored across the full workspace and doctests, lenient and strict spec checks, catalog check, and acceptance 1/1. Canonical interleaved median-of-five timing is 536.39 ms on versus 556.65 ms off (-3.64%).
