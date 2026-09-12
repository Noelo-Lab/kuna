---
need_id: whole-binary-decompilation-exits
title: Whole-binary decompilation exits zero when every function fails
track: tooling
status: closed
severity: major
probe_id: p-6b7d31fb2398
acceptance_id: a-ca0acbabc161
hypothesis_status: inconclusive
credibility: 0.7
instances: 1
challenges: [5ab77f5433c5d40ad448c1cd]
rounds: [6]
first_seen_round: 6
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli, decompiler/crates/kuna-console/src/project.rs, docs/cli.md, docs/spec/00-overview.md, docs/history.md]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/592
closed_in_round: 12
closing_pr: "592"
reject_reason: null
---

## Symptom

A successful process status when no function could be decompiled.

> **Whole-binary decompilation exits zero when every function fails** (major, `5ab77f5433c5d40ad448c1cd`)
> Exited 0 with top-level error null, while its only function had code null and error Unable to load 512 bytes at r0x00401d91. Single-function decompilation correctly exited 1.

## Reproduction

The original EndOfWorld report is retained in the decision log as the observation that exposed this contract. It is not the executable witness: after `decodehalt` landed, that packed function produces a truncation body instead of failing outright. The vendored `fauxware` fixture makes the same aggregate state deliberately and durably by selecting `main` with a fatal five-instruction ceiling.

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--functions",
    "main",
    "--option",
    "maxinstruction",
    "5",
    "--option",
    "errortoomanyinstructions",
    "on",
    "--json"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "main",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 1
      },
      {
        "path": "error",
        "op": "eq",
        "value": null
      },
      {
        "path": "functions[0].code",
        "op": "eq",
        "value": null
      },
      {
        "path": "functions[0].error",
        "op": "contains",
        "value": "Flow exceeded maximum allowable instructions"
      }
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-ca0acbabc161",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile-all",
    "{{BIN}}",
    "--functions",
    "main",
    "--option",
    "maxinstruction",
    "5",
    "--option",
    "errortoomanyinstructions",
    "on",
    "--json"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "main",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stdout_is_json": true,
    "stderr_matches": [
      "decompilation produced zero function bodies for 1 selected function",
      "see the per-function error record"
    ],
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 1
      },
      {
        "path": "error",
        "op": "contains",
        "value": "decompilation produced zero function bodies"
      },
      {
        "path": "functions[0].code",
        "op": "eq",
        "value": null
      },
      {
        "path": "functions[0].error",
        "op": "contains",
        "value": "Flow exceeded maximum allowable instructions"
      }
    ]
  },
  "notes": "Retargeted from the now-stale EndOfWorld witness to vendored fauxware. A fatal 5-instruction budget puts main in the same durable state: one selected function, zero bodies, one error record. The cargo gates separately pin text output, both project modes, completed streamed artifacts, and mixed batches remaining exit 0."
}
```

The closing cargo tests add the opposite control the one-binary probe cannot express: `main,__libc_csu_fini` under the same ceiling has one failed function and one body, so `decompile-all` and both project modes must remain exit 0. They also require text output and every ordinary/streamed project artifact to be complete before the all-failed run exits 1; a completed streamed export removes `.streaming` even though its process status is nonzero.

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5433c5d40ad448c1cd` (round 6, tester t-r6-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 6 REFUTER: hypothesis **inconclusive**. INCONCLUSIVE ONLY BECAUSE THE NEED WAS FILED '## Hypothesis: _none offered_' -- there was no diagnosis to uphold or overturn. A REFUTER RAN, THE SYMPTOM IS CONFIRMED, AND THE MECHANISM IS BELOW. DO NOT RE-SPEND A REFUTER ON IT. Measured on .kuna-repipe/arena/6/5ab77f5433c5d40ad448c1cd/target/EndOfWorld.zip.__x/EndOfWorld.exe (note the nested path -- the binary is NOT directly under target/) with the release kuna at 9c4c4508. (1) THE SYMPTOM STANDS VERBATIM: 'decompile-all <bin> --json' exits **0** with top-level "error": null while its one and only function carries "code": null and "error": "Unable to load 512 bytes at r0x00401d91". (2) THE INCONSISTENCY IS THREE-WAY, NOT TWO-WAY, AND THE NEED NAMES THE WRONG CONTROL. Four measured runs: 'decompile-all --json' -> rc=0, error null, per-function error set. 'decompile 0x4100a0 --json' -> rc=1 with a top-level "error" string (this is the row the need calls 'single-function decompilation correctly exited 1' -- true, but ONLY in JSON mode). 'decompile-all' TEXT -> rc=0, prints '// Function: sub_4100a0 @ 0x4100a0  (error: Unable to load 512 bytes at r0x00401d91)' -- wrong code but it does name the failure. **'decompile 0x4100a0' TEXT -> rc=0 with ZERO BYTES on stdout AND ZERO BYTES on stderr.** That last one is the worst of the four: a silent success-looking exit with no output and no message anywhere, and it is not mentioned in the need at all. (3) IT IS NOT A HARNESS ARTIFACT. Control: the same text-mode command on a healthy function (crackme_very_hard.exe 0x140002960) returns rc=0 with 2493 bytes of C, so the text path itself works; the empty output is specific to the failing function. Checked for the stale-decomp_dbg trap ([[kuna-stale-debug-decompdbg]]) -- a debug decomp_dbg does exist (04:21) alongside the release one (23:54) -- and the healthy-function control rules it out. (4) SO THE REAL GAP IS 'FAILURE IS ONLY REPORTED ON THE --json SINGLE-FUNCTION PATH', not 'decompile-all exits zero'. A builder should make all four paths agree, and the text single-function path is the one that needs a message, not just an exit code. *** WARNING FOR T_TRIAGE -- THE ACCEPTANCE PROBE (a-0b045e12fce4) CANNOT TELL A CORRECT FIX FROM A DANGEROUS ONE. *** Its only clause is exit_code ne 0 on 'decompile-all --json'. The need's own title says 'when EVERY function fails', but the probe is equally green if a builder makes decompile-all exit non-zero when ANY ONE function fails -- which would turn every large real binary (where a handful of functions always fail) into a red exit and break whole-binary workflows across the board. The probe needs a second, opposite case: a binary where some functions decompile and some fail must still exit 0. This is the fourth closing-direction probe hole found this round (see guard-dispatch-calls-hide, vm-terminal-opcode-emitted, code-byte-loads-print, msvc-cookie-removal-changes).
- round 6 T_TRIAGE: ACCEPTANCE REWRITTEN (a-0b045e12fce4 -> a-b1806ca4663e), and it was verified two-sided before the swap: it FAILS on the need's own arena binary at 9c4c4508 (`probe check --bin` rc=1) and PASSES when simulated against plausible correct renderings. It its only clause was `exit_code ne 0`, equally green if a builder red-exits when ANY ONE function fails -- which would break every large binary. Now also requires the TOP-LEVEL `error` field to be populated, so the fix must REPORT the failure rather than just colour the exit. *** THE OPPOSITE CASE IS STILL NOT EXPRESSIBLE HERE: one probe carries one {{BIN}}, and `cli-pair` has no separate implementation in probe.py (it is a kind name only). THE BUILDER MUST ADD IT BY HAND: a binary where some functions decompile and some fail MUST still exit 0, as a second case in the promoted tests/cli probe. Treat that as a contract term, not a nice-to-have.
- round 6 T_TRIAGE: TRIAGE: touches += docs/history.md. This is a driver-tier exit-code change, which per the CLI-tier recipe needs no option and no stage XML but DOES need a DIV row -- and that row is a counter two branches can merge cleanly to the wrong value, hence the lease.
- 2026-09-12 BUILDER: the EndOfWorld observation is historical, not the current witness. DIV-151 `decodehalt` makes that packed function report a truncation body, so it no longer reaches the all-failed aggregate state. Reproduction and acceptance were re-cut on origin/main 31b96e1 to vendored `fauxware main` with `maxinstruction 5` and `errortoomanyinstructions on`: one selected record, `code: null`, fatal flow-ceiling error, but exit 0 and top-level error null before this fix. The mixed `main,__libc_csu_fini` control has one body plus one failure and remains exit 0. Probe/acceptance IDs were re-derived; the original challenge remains the instance that exposed the interface contract.
- closed: acceptance a-ca0acbabc161 now PASSES at 22424a828c3a
- round 12 BUILDER: PR #592 carries the fix and the canonical acceptance closure above. `attempts` remains 0 because that field counts failed builder attempts; this first successful builder reached a green, shippable result without a failed attempt.
