---
need_id: accepted-sqrt-prototype-still
title: Accepted sqrt prototype still leaves floating arguments absent
track: quality
status: open
severity: major
probe_id: p-fd6d3fefafe2
acceptance_id: a-bb9896e9cfbb
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [640a526833c5d447bc761899]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp/src/p4_calls]
scope: small
regression_of: null
pr: null
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
      "sqrt\\("
    ],
    "stdout_absent": [
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
