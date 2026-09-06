---
need_id: accepted-sqrt-prototype-still
title: Accepted sqrt prototype still leaves floating arguments absent
track: quality
status: open
severity: major
probe_id: p-fd6d3fefafe2
acceptance_id: a-94788ed2c81d
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [640a526833c5d447bc761899]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-console/src/assertions.rs, decompiler/crates/kuna-console/src/ifacedecomp.rs]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/470
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Force the known XMM0 double argument using an explicit prototype.

> **Accepted sqrt prototype still leaves floating arguments absent** (major, `640a526833c5d447bc761899`)
> Default output drops sqrt arguments and reads unassigned result locals. calloverlap full recovers results, but accepted thunk-address, import-address, and parameter assertions still leave sqrt() argumentless.

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
    "sub_140001890",
    "--option",
    "calloverlap",
    "full",
    "--assert",
    "prototype 0x140003ddf float8 sqrt(float8 x)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "rejected"
    ],
    "stdout_matches": [
      "sqrt\\(\\s*\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/KeyCheker.exe",
    "binary_sha256": "351e54ecaa80f0395111a90e332313c15bd1e19d1e12da87606a045efb5afecf",
    "binary_size": 25600,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "main",
    "--assert",
    "prototype 0x4006ed int4 accepted(int4 status)"
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
    "stdout_matches": [
      "accepted\\("
    ],
    "stdout_absent": [
      "accepted\\(\\s*\\)"
    ],
    "stderr_absent": [
      "rejected"
    ]
  },
  "notes": "Retargeted onto the in-repo fauxware fixture: same defect, no dataset. `accepted` at 0x4006ed renders `accepted()` and an address-form prototype assertion is reported `applied` while changing nothing; measured on c92dddbb. The dataset KeyCheker.exe sqrt() call stays the witness in Reproduction."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Whole-width XMM writes may defeat the locked scalar argument.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `640a526833c5d447bc761899` (round 3, tester t-r3-640a5268)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_TRIAGE r3: track quality CONFIRMED and touches narrowed to p4_calls: the prototype is accepted (the assert does not error), so the loss is in call input-trial/float-parameter recovery, not in the grammar -- unlike prototype-assertions-reject-ordinary, which is the grammar and is on the tooling track.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- round 3 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED by measurement on the 20:01 release binary (arena/3/640a526833c5d447bc761899/target/KeyCheker.exe). The filed cause is 'whole-width XMM writes may defeat the locked scalar argument'. Nothing about XMM width is ever reached: the prototype assertion is a SILENT NO-OP at this call site. Three runs, same 4 call sites each time, output identical in all three: (a) the filed 'float8 sqrt(float8 x)' -> 'v26 = (double)sqrt();'; (b) 'float8 sqrt(float8 x, float8 y)' -> still zero arguments, and a locked 2-input prototype cannot print zero args; (c) 'int8 zzzmarker(int8 x)' -> the name is still sqrt and the cast is still (double). kuna functions puts a real 33-byte function 'sqrt' at exactly 0x140003ddf, so the address is right and the assert exits 0 with an empty stderr -- accepted and then discarded. The (double) cast and the sqrt name come from kuna's own recovery for that address, which is what makes the assertion look applied. A builder who follows the hypothesis into p4_calls float-parameter recovery would be fixing a mechanism that never runs; the T_TRIAGE narrowing to p4_calls rested on 'the assert does not error' and is wrong for the same reason. The gap is upstream in the path that turns a --assert prototype into a locked FuncProto on the callee (kuna decides nothing from it here); the argumentless call is the downstream symptom. Not measured: which of parse / attach-to-address / lock-the-input drops it. Note the relation to in-flight PR #421 (prototype-assertions-reject-ordinary): that one is the GRAMMAR rejecting ordinary C signatures; this one is a signature the grammar accepts being thrown away, so #421 merging will not close it.

- round 4 BUILDER: the refuter is right and the cause is one line further up than even it said. `<func>` in a `prototype`/`param`/`return` directive was resolved by NAME ONLY (`Architecture::set_function_prototype_pieces` -> `queryFunction(name)`), and `0x140003ddf` is not a name, so the pieces were parked under a key nothing ever reads while the report still said `applied`. Fixed by resolving `<func>` as an entry address when it is not a name, and parking through the address-keyed `set_function_prototype_pieces_at` -- which is the key the READ side (`ArchContext::callee_proto_pieces`) already used. Nothing in p4_calls changed: the XMM0 input trial was never the problem, and `float8 sqrt(float8 x)` at 0x140003ddf now renders `sqrt(v35._0_8_)`.
- round 4 BUILDER: the by-NAME form is still wrong here and is left open deliberately. This PE has TWO FunctionSymbols called `sqrt` -- the import thunk at 0x140003ddf that every call goes to, and the IAT slot at 0x140005238 -- and the global by-name query answers with the slot, so `--assert 'prototype sqrt ...'` parks on a symbol no call site reads. Disambiguating a duplicated import name touches the library-prototype pass and both parity corpora; the address form is the sound way to say it and is what this need closes.
- round 4 BUILDER: acceptance retargeted onto the in-repo `fauxware` fixture (`accepted` at 0x4006ed, argumentless for the same reason) so the probe promotes into `tests/cli/` where CI has no dataset; `acceptance_id` re-derived a-bb9896e9cfbb -> a-94788ed2c81d. Measured on c92dddbb: `accepted();` with the directive reported `applied`.
