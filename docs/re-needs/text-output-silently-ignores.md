---
need_id: text-output-silently-ignores
title: Text output silently ignores a prototype override that JSON output applies
track: tooling
status: open
severity: major
probe_id: p-8cb750f382af
acceptance_id: a-5a9ec955c9a0
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [6a0b84982b3df128c1df5c0d]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/assertdecl.rs, decompiler/crates/kuna-cli/src/decompile.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Assign the hash helper a pointer return and named pointer parameters.

> **Text output silently ignores a prototype override that JSON output applies** (major, `6a0b84982b3df128c1df5c0d`)
> When the declaration uses a new function name, text output retains the original void return and integer parameters, exiting 0 even with --assert-strict. Adding --json applies the requested types and reports the assertion applied.

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
    "sub_1400055e0",
    "--assert",
    "prototype sub_1400055e0 void * sha256(void *out, void *input)",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "^void\\s+[^\\s(]+\\("
    ],
    "stderr_absent": [
      "rejected"
    ]
  },
  "target": {
    "binary_rel": "bin/frz_crackme_rage_v7.exe",
    "binary_sha256": "971dbc9fc68f8c2a3f516f49cc7c13534e6c57143d0160c648e0c1490662fbf2",
    "binary_size": 279552,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-5a9ec955c9a0",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "authenticate",
    "--assert",
    "prototype authenticate void *hashit(void *out,void *input)",
    "--assert-strict"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "authenticate",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "void \\*\\s*authenticate\\(void \\*out,void \\*input\\)"
    ],
    "stdout_absent": [
      "^unsigned long\\s+authenticate\\("
    ],
    "stderr_absent": [
      "rejected"
    ]
  },
  "notes": "Desired: a prototype assertion whose declaration renames the function still binds to <func>, on the TEXT surface. `--assert 'prototype authenticate void *hashit(...)'` must retype authenticate; before the fix the console script lowered it to `parse line extern`, which binds by the DECLARED name, so text kept `unsigned long authenticate(char *a0,char *a1)` and exited 0 while --json applied it."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Text and JSON paths may bind the declaration to different function names.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6a0b84982b3df128c1df5c0d` (round 3, tester t-r3-6a0b8498)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_TRIAGE r3: track tooling CONFIRMED and touches narrowed. The defect is a divergence between two CLI output paths -- the same --assert/--assert-strict is applied under --json and dropped in text -- so the engine is producing the right thing and the printer path is losing it. No option, no stages case; a cargo test in kuna-cli plus the promoted probe.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- round 3 REFUTER: hypothesis **upheld** (was inconclusive). Refuted by measurement (captain, B_DRAIN observe tick, release kuna @20:01 on cf5234ac+, probebin/971dbc9fc68f8c2a/frz_crackme_rage_v7.exe). SYMPTOM STANDS AND THE HYPOTHESIS IS RIGHT, BUT THE SEAM IS NAMED WRONG -- it is not two paths disagreeing about a name, it is ONE path throwing the name away. Measured 4 ways: (a) text + decl name 'sha256' -> 'void sub_1400055e0(unsigned int *a0,unsigned long long *a1)', override dropped, exit 0, nothing on stderr even with --assert-strict; (b) --json, same directive -> 'void * sub_1400055e0(void *out,void *input)', applied; (c) TEXT WITH THE SAME NAME IN THE DECL ('prototype sub_1400055e0 void * sub_1400055e0(void *out, void *input)') APPLIES CORRECTLY -- so the discriminator is the decl name, not the output format; (d) --json with <func> naming another or a bogus function ('prototype main ...', 'prototype no_such_fn ...') does NOT touch the selected function, so JSON honours <func>. MECHANISM: assertdecl.rs:343 lowers Body::Prototype { func: _, decl } -- the <func> target is DISCARDED -- to 'parse line extern <decl>;', which binds the prototype to a symbol named by the DECLARATION. Same name => it lands on the selected function; a renaming decl => it lands on a fresh unrelated symbol and the selected function is untouched. The JSON surface is a SECOND IMPLEMENTATION, not a second dialect as decompile.rs:14 claims: --json routes through run_json -> in-process decompile_all (decompile.rs:1152/1241), while text drives decomp_dbg as a subprocess over the console script. FIX DIRECTION: make the text lowering honour <func> (bind the parsed prototype to the target function, renaming it when the decl name differs). Making JSON match text would be the wrong direction. SECOND DEFECT FOUND, DO NOT TREAT --assert-strict AS A GUARD HERE: the outcome is recovered from the console transcript (assertion_outcomes), so a directive that changed nothing still reports status 'applied' -- including 'prototype no_such_fn ...' under --json. The acceptance probe's stderr_absent 'rejected' clause therefore proves nothing on its own; the load-bearing clauses are stdout_absent/stdout_matches.
captain B_DRAIN r3: touches CORRECTED after the refutation -- the seam is assertdecl.rs:343 (the prototype lowering that discards <func>), not output.rs, which the refuter measured is not involved. assertdecl.rs is also being edited RIGHT NOW by b-r3-prototype-assert (feat/re-prototype-assertions-reject-ordinary, --assert scalar types); this need must not be dispatched until that lands, and select.py's file-lease disjointness now sees the overlap.
