---
need_id: string-ownership-misses-literal
title: String ownership misses a literal PUSH in the window handler
track: tooling
status: open
severity: major
probe_id: p-71e859940699
acceptance_id: a-e1c6f00964d8
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [60be2ad433c5d410b8842c95]
rounds: [4]
first_seen_round: 4
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/xrefs.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Locate the handler referencing Product Already Registered.

> **String ownership misses a literal PUSH in the window handler** (major, `60be2ad433c5d410b8842c95`)
> Finds the string but reports zero xrefs and no owning functions. Direct xrefs also returns zero. Disassembly shows PUSH 0x403288 at 0x4016ef, and decompile-all includes the literal in the handler.

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
    "strings",
    "{{BIN}}",
    "--filter",
    "Product Already Registered",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "strings[0].functions",
        "op": "len_eq",
        "value": 0
      }
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386",
    "binary_sha256": "7c67abaf4be653478fff9c6344c1fe58fe68675d022bab1e110985f67044a8cc",
    "binary_size": 9092,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "strings",
    "{{BIN}}",
    "--filter",
    "switch case alpha reached",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 1
      },
      {
        "path": "strings[0].functions",
        "op": "len_gt",
        "value": 0
      }
    ]
  },
  "notes": "Vendored twin of the reported image: a non-PIE ELF32 dispatching JMP dword ptr [EAX*0x4 + 0x101000] over a four-entry .long table, each case body pushing a literal -- the shape of the reported PUSH 0x403288. The dataset original stays the witness in Reproduction and passes the same assertions; this target is in-repo so the probe promotes into tests/cli/ and runs where there is no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The reference walk may miss this switch branch.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `60be2ad433c5d410b8842c95` (round 4, tester t-r4-60be2ad4)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and broader than filed. The listing-tier xref walk is a recursive descent that only follows classify()'s direct flows plus fall-through (listing/xrefs.rs:626-639), so a BRANCHIND contributes no successor and the walk dies at every jump table. Control on sha c5f4073a: xrefs --from 0x4011a0 files 61 refs whose from-addresses run 0x4011ac..0x40143b and then resume at 0x401732 -- the entire case-body region 0x40143c..0x401731 is unwalked, and 0x40143b is itself the row 'JMP dword ptr [EAX*0x4 + 0x4017c4]', the table base, filed as a data ref. Both pushes in that case body are lost, not just the filtered one: xrefs --to 0x403278 is also empty. The free in-function control is the decompiler on the SAME function: kuna decompile 0x4011a0 --addr emits v20 = the literal at line 305, i.e. p2's switch recovery reaches the case body that the analysis-tier walk cannot. So the defect is not in strings.rs (it just consumes XrefIndex, and xrefs --to 0x403288 is 0 too) and not in the string scan (the literal is found, section .rdata). SCOPE CORRECTION: touches is kuna-analysis (listing/xrefs.rs), not kuna-decomp, and the fix is jump-table target enumeration for the descent -- every switch case body in every binary is invisible to xrefs/strings today, so the blast radius of a fix is wider than one need.
- round 4 BUILDER: acceptance retargeted from the dataset witness to the vendored twin `decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386` so it promotes into `tests/cli/` (CI has no dataset). The witness passes the same assertions on the fixed build (`strings[0].functions` = `sub_4011a0`, `xrefs --to 0x403288` = 1). `cmd` changed with the target, so `acceptance_id` was relabelled a-240de031bdd8 -> a-e1c6f00964d8.
