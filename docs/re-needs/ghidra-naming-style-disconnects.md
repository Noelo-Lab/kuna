---
need_id: ghidra-naming-style-disconnects
title: Ghidra naming style disconnects the VM body from its parameters
track: quality
status: closed
severity: major
probe_id: p-1aad63758f5d
acceptance_id: a-76fef1f16a2f
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [69761b7a39e9c4d85c2f9fc1]
rounds: [12]
first_seen_round: 12
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/580
closed_in_round: 12
closing_pr: 580
reject_reason: null
---

## Symptom

Use alternative naming to identify the problematic temporary without changing semantics.

> **Ghidra naming style disconnects the VM body from its parameters** (major, `69761b7a39e9c4d85c2f9fc1`)
> The signature uses param_1, param_2, param_3, but the body uses newly declared, uninitialized a0, a1, a2. The first bounds check reads a1 and instruction reads use a0; no assignments connect them to formal parameters.

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
    "sub_1005350",
    "--option",
    "namestyle",
    "ghidra"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\bparam_[0-9]+",
      "(?m)^  (?:unsigned )?(?:long|int) (a[0-9]+);(?![\\s\\S]*\\b\\1\\s*=)"
    ]
  },
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
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
    "sample",
    "--option",
    "namestyle",
    "ghidra"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "int sample\\(int param_1\\)",
      "return scale \\* param_1 \\+ bias \\+ dat_50000000 \\* 2;"
    ],
    "stdout_absent": [
      "int a0;",
      "\\* a0"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64",
    "binary_sha256": "1c9ad3b67437c7e85ac53d0755609c5b5e015225796a98c0287de5f2cb3d382e",
    "binary_size": 15800,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/assertranges_x86_64",
    "selector": "sample",
    "selector_kind": "name"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Parameter declarations and body references appear to use different naming policies.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69761b7a39e9c4d85c2f9fc1` (round 12, tester t-r12-69761b7a)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_absent` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). round 12 REFUTER (captain, in-tick): UPHELD on aa18e56d, and the hypothesis is right almost word for word -- this IS two naming policies disagreeing. It is also FAR bigger than the one function the tester filed. THE CONTROL SETTLES IT: the same function under the DEFAULT namestyle is completely correct. kuna decompile graphy sub_1005350 --option calleearity on --option varargstackargs on emits sub_1005350(long a0, unsigned int a1, unsigned long *a2) and the body reads a0/a1/a2 with NO local declarations for them; only adding --option namestyle ghidra turns the signature into (param_1, param_2, param_3) while the body keeps a0/a1/a2 and gains three orphan declarations long a0; unsigned int a1; unsigned long *a2;. So the dataflow, the parameter recovery and the prototype are all fine and none of them is the defect; the rename is. THE MECHANISM IS TWO LINES AND THEY AGREE WITH EACH OTHER -- THE THIRD PRODUCER IS THE ODD ONE OUT. printc.rs:2518-2526 prints an unlocked parameter's default name from arch.name_style_angr: angr style gives kuna_arg_name(i) i.e. a<i>, anything else gives format param_{i+1}. printc.rs:2588-2608 builds the param_names suppression set -- the set that stops a formal parameter from also being declared as a local -- with byte-identical logic. Under ghidra style both produce param_1..param_N. But the body's input HighVariables are still named a0/a1/a2 by the p6 variable-naming pass, which does not consult the name style at all. The suppression set therefore never matches the body names, every parameter falls through as an undeclared body variable, and the declaration pass emits it as a fresh uninitialized local. The fix belongs in the p6 input-HighVariable naming (make it honour the style, as Ghidra does -- param_1 in both places), NOT in printc, whose two sites are already consistent. SCALE, MEASURED, AND IT SHOULD PROBABLY RAISE THE SEVERITY: this is not one function. kuna decompile-all graphy --option namestyle ghidra over the whole image yields 103 never-assigned aN local declarations across 39 of 55 functions, every one of them under a param_N signature -- func_0x01002c90 (a0,a1), func_0x01004e90 (a0,a1), func_0x01004ef0 (a0,a1), func_0x01005250 (a0..a4), and 35 more. In other words --option namestyle ghidra emits semantically wrong C for essentially every function that takes an argument: the body reads uninitialized locals and the recovered parameters are never used. Anyone who selects the Ghidra convention to compare against Ghidra gets output that cannot be trusted anywhere. THREE THINGS FOR WHOEVER BUILDS IT. (1) DROP THE TWO EXTRA OPTIONS FROM THE REPRO. calleearity on and varargstackargs on are in both probes and are IRRELEVANT -- I ran sub_1005350 under ghidra style without them and the output is identical, same three orphan declarations. Leaving them in points the builder at call/argument recovery, which is the wrong subsystem and is exactly where this round's sibling needs live. (2) THERE IS A SECOND, SEPARATE DEFECT IN THE SAME PASS AND THE ACCEPTANCE DOES NOT COVER IT: the ghidra rename is incomplete for locals too. In sub_1005350 under ghidra style the array is still unsigned long v10 [64] and the pointer is still long *v12 -- default-style v names that Ghidra would spell auVar10 and plVar12 -- while unsigned long *Var1 has lost its type prefix entirely (Ghidra: puVar1). Only the plain uVarN scalars were converted. Fixing the parameters will not touch any of that, so decide deliberately whether it is in scope rather than discovering it in review. (3) THE ACCEPTANCE REGEX IS SOUND BUT SINGLE-FUNCTION. Its backreference-inside-lookahead form (?![\s\S]*\b\1\s*=) does evaluate correctly -- the T_GATE replay passed the probe arm three times out of three on this exact pattern, so the runner's engine supports it and it is not a Rust-regex hazard. But it proves only sub_1005350. Since 39 of 55 functions fail, add a whole-binary clause over decompile-all --option namestyle ghidra asserting zero such declarations; a fix that special-cases one function would otherwise pass. TRACK AND SCOPE CONFIRMED as filed: quality, kuna-decomp, small. NOT A DUPLICATE of changing-namestyle-invalidates-discovered (closed via #530): that one is decomp_dbg name RESOLUTION -- a generated name failing to select its function across styles -- and this is name PRODUCTION inside one function's body. Different subsystem, different symptom, no shared code path; the T_DEDUP decision not to fold them was right.
- round 12 BUILDER: removed the two unrelated options from the dataset reproduction and hardened acceptance onto the vendored `assertranges_x86_64` witness. It requires the exact `param_1` signature and complete return expression while rejecting both the orphan `int a0;` declaration and `* a0` use.
- closed: acceptance a-76fef1f16a2f now PASSES at 13064810013f; PR #580.
