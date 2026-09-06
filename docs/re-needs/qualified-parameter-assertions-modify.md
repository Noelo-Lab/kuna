---
need_id: qualified-parameter-assertions-modify
title: Qualified parameter assertions modify the caller instead of the named callee
track: tooling
status: open
severity: major
probe_id: p-c99ad8423821
acceptance_id: a-64ae0f00be99
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [60be2ad433c5d410b8842c95]
rounds: [4]
first_seen_round: 4
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/assertdecl.rs, decompiler/crates/kuna-cli/src/decompile.rs]
scope: medium
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Override the callee's ECX/EDX parameter storage.

> **Qualified parameter assertions modify the caller instead of the named callee** (major, `60be2ad433c5d410b8842c95`)
> Assertions qualified with sub_401c50 rename and retype sub_402020 inputs to maze/moves instead. The intended callee still has an empty argument list. No rejection warning appears.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/Sabloom Text 6.exe",
    "binary_sha256": "5a03a2b553065aedf58f3512e1a701d2ab5e8e9365ea713bfed98f19836747e3",
    "binary_size": 242176,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_402020",
    "--assert",
    "param sub_401c50::0 ECX char *maze",
    "--assert",
    "param sub_401c50::1 EDX char *moves"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\A[^\\n]*\\([^\\n]*\\bmaze\\b[^\\n]*\\bmoves\\b"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/Sabloom Text 6.exe",
    "binary_sha256": "5a03a2b553065aedf58f3512e1a701d2ab5e8e9365ea713bfed98f19836747e3",
    "binary_size": 242176,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_402020",
    "--assert",
    "param sub_401c50::0 ECX char *maze",
    "--assert",
    "param sub_401c50::1 EDX char *moves"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\A[^\\n]*\\([^\\n]*\\bmaze\\b",
      "\\bsub_[0-9a-f]+\\(\\)"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The parameter directive may discard its function qualifier.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `60be2ad433c5d410b8842c95` (round 4, tester t-r4-60be2ad4)

## Decision log

- filed by cluster.py from 1 observation(s)
- split out of the round-4 `16-byte-vm-state` mega-bucket by the captain at T_DEDUP: cluster.py's key is `kind|subcommand|clause-shape`, so nine unrelated wrong-output `decompile` defects collapsed into one need whose probe covered only the first. Each carries its own probe and acceptance from its own observation.
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, literally -- the qualifier is dropped by a `..` in one destructuring pattern -- and the defect is SIX directives wide, not one. Reproduced on sha 6188e204: the two sub_401c50-qualified asserts rename the DECOMPILED function to `sub_402020(char *maze,char *moves)`, exit 0, no warning. CONTROL: replace the qualifier with a function that does not exist (`param nosuchfunc_zzz::0 ECX char *maze`) and the output is byte-identical to both the real-qualifier run AND the unqualified `param 0 ECX ...` run, so the qualifier is not resolved wrongly, it is never looked up. THE CODE SAYS THE SAME: kuna-cli/src/assertdecl.rs parses it correctly (split_qualifier at :239) and then lowers with `Body::Param { index, storage, decl, .. }` -> "map param {index} {storage} {decl}" -- the `..` eats `func`. Same discard for Return, Comment, Flow, Name, Type; `split_qualifier`/`Body::` appear NOWHERE outside assertdecl.rs, so nothing in kuna-cli consumes a parsed qualifier. docs/cli.md:164-171 documents `[<func>::]` on all six. THE CORRECT APPLIER ALREADY EXISTS AND THE CLI BYPASSES IT: kuna-console/src/assertions.rs has `binds_to()` ("a qualified directive binds only to the function it names") and even the rejection the tester expected ("no decompiled function matched this directive"). `kuna decompile` parses into that crate's Directive type but applies through assertdecl's text lowering into a decomp_dbg script, and the console commands themselves (`map param <i> <storage> <decl>`) have no function operand -- so the fix is either a qualified console form or a `load function <func>` / apply / re-load dance, and it must not disturb the existing Image/Program/Function/Symbol slot ordering or the second pass. BEST LEAD, MEASURED: the documented cross-function `prototype` directive WORKS on this exact binary -- `--assert "prototype sub_401c50 int sub_401c50(char *maze,char *moves)"` renders the call site as `sub_401c50((char *)v20,moves)` and leaves the caller signature alone, i.e. it produces exactly the state the acceptance describes. apply_prototype writes the callee's FunctionSymbol + pending-prototype store (assertions.rs:389-400), which is where a qualified `param`/`return` should land too; a transient Funcdata lock will not survive re-loading the caller. ACCEPTANCE IS SOUND, NOT GAMEABLE: its cmd is the qualified-param one, so the prototype workaround cannot flip it; and the baseline really does print `v15 = sub_401c50();`, so a reject-with-a-warning fix keeps that empty-arg call and FAILS pattern 2. Residual: unrelated call-argument recovery on this binary could also flip it. SCOPE: filed `small` -- the parse half is free but the application half is a new plumbing path; call it medium and expect the builder to touch all six directives or to say why param/return only. TOUCHES IS WRONG: filed kuna-decomp; it is kuna-cli (assertdecl.rs, decompile.rs) + possibly kuna-console.
