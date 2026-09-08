---
need_id: utf-8-prompt-loses
title: UTF-8 prompt loses its prefix, start address, and owning function
track: tooling
status: open
severity: minor
probe_id: p-34a6b92abff4
acceptance_id: a-2b59e0e61f43
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [6736b3a09b533b4c22bd2b9f]
rounds: [10]
first_seen_round: 10
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis, decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Inventory the complete UTF-8 prompt referenced at 0x2000 and identify its owner.

> **UTF-8 prompt loses its prefix, start address, and owning function** (minor, `6736b3a09b533b4c22bd2b9f`)
> --encoding all returns only an ASCII suffix starting at 0x200c, with zero xrefs and no functions. kuna read shows valid UTF-8 beginning at 0x2000; disassembly shows the entry routine referencing that address.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "strings",
    "{{BIN}}",
    "--encoding",
    "all",
    "--json",
    "--filter",
    "magical"
  ],
  "target": {
    "binary_rel": "bin/no-standards",
    "binary_sha256": "5cb1538c58846236e962497d1ccab88e02cc958a031d2c96c150e11c0d51ef27",
    "binary_size": 21840,
    "binary_source": "dataset"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "strings[0].address",
        "op": "eq",
        "value": 8204
      },
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
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "strings",
    "{{BIN}}",
    "--encoding",
    "all",
    "--json",
    "--filter",
    "magical"
  ],
  "target": {
    "binary_rel": "bin/no-standards",
    "binary_sha256": "5cb1538c58846236e962497d1ccab88e02cc958a031d2c96c150e11c0d51ef27",
    "binary_size": 21840,
    "binary_source": "dataset"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "strings[*].address",
        "op": "eq",
        "value": 8192
      },
      {
        "path": "strings[0].functions",
        "op": "len_gt",
        "value": 0
      }
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The matcher supports ASCII and limited UTF-16 but lacks UTF-8 decoding.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `6736b3a09b533b4c22bd2b9f` (round 10, tester t-r10-6736b3a0)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER: hypothesis **upheld** (was inconclusive). REFUTER RAN (captain, tick 10:05Z). Filed cause -- 'the matcher supports ASCII and limited UTF-16 but lacks UTF-8 decoding' -- is UPHELD, and confirmed straight from the CLI surface: kuna strings --help offers [--encoding ascii|utf16|all], so 'all' means ascii PLUS utf16 and there is no UTF-8 decoder anywhere in the option space. EVIDENCE on .kuna-repipe/arena/10/6736b3a09b533b4c22bd2b9f/target/no-standards. kuna read 0x2000 shows the literal is a kaomoji whose prefix is all valid UTF-8: ef bc bf (U+FF3F FULLWIDTH LOW LINE), cf 86 (U+03C6 GREEK SMALL PHI), 28 20, c2 b0 (DEGREE SIGN), 2d, c2 b0, then ')/ so what was the magical keycombination? '. kuna strings --encoding all --json --filter magical returns exactly one hit, address 0x200c, encoding 'ascii' -- i.e. the scanner discards everything up to and including the LAST multi-byte sequence and starts the run at the first byte after it. *** THE HALF OF THE SYMPTOM THAT LOOKS LIKE A SECOND DEFECT IS NOT ONE -- IT IS A CONSEQUENCE, AND ONE FIX CLOSES BOTH. *** The need also reports 'zero xrefs and no functions', which reads as a separate xref/ownership gap. It is not: kuna xrefs --to 0x2000 --json returns count 1 (the reference the entry routine really makes), while kuna xrefs --to 0x200c --json returns count 0. The reference machinery already finds the string's TRUE start; the strings record simply reports the wrong address, so nothing can attach to it. Decode UTF-8 so the run starts at 0x2000 and xrefs_count/functions populate from the existing machinery with no separate work. BUILDER NOTE: this is an ADDITIVE decoder plus an encoding value, not a change to the ascii path -- the ascii scanner's behaviour on pure-ASCII input must not move, or the parity corpora will notice. Whether 'all' should start including utf8 is a default-visible change and is the one judgement call here.
- round 10 T_TRIAGE (captain, 10:39Z tick): touches corrected to name kuna-analysis first; track LEFT at tooling, deliberately. The string scan is kuna-analysis/src/analyzers/strings/kuna_stringinv.rs (and kuna_widestrings.rs for the encoding arm); kuna-cli/src/strings.rs is only the surface, so a builder pointed at kuna-cli alone would be reading the wrong file. Track stays tooling because no option, no phases.toml row and no catalog counter is involved, and neither of these two crate paths maps to a lease in select.py PATH_RESOURCES -- so this correction changes where the builder looks, not what it holds.
