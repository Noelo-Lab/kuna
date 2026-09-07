---
need_id: disassembling-40-instructions-takes
title: Disassembling 40 instructions takes about 21 seconds
track: perf
status: open
severity: major
probe_id: p-677eed496307
acceptance_id: a-c4f6f7e7667f
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [69d7f15a8afd9d6c48b48871]
rounds: [5]
first_seen_round: 5
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/disassemble.rs, decompiler/crates/kuna-analysis]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Inspect 160 bytes of the prologue interactively.

> **Disassembling 40 instructions takes about 21 seconds** (major, `69d7f15a8afd9d6c48b48871`)
> Identical requests returned 40 instructions in 20.9154 and 20.7541 seconds, recorded in notes/toolcalls.jsonl.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "disassemble",
    "{{BIN}}",
    "0x140001000",
    "--addr",
    "--count",
    "40",
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
        "value": 40
      }
    ],
    "wall_ms": {
      "stat": "median",
      "gt": 10000
    }
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
    "disassemble",
    "{{BIN}}",
    "0x140001000",
    "--addr",
    "--count",
    "40",
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
        "value": 40
      }
    ],
    "wall_ms": {
      "stat": "median",
      "lt": 10000
    }
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "36bfae11c18fb5fa214110d7f17cdc92026bce53e28e9a1965b4193e59c1a6a1",
    "binary_size": 9384960,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Address-based disassembly pays for whole-image analysis.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69d7f15a8afd9d6c48b48871` (round 5, tester t-r5-69d7f15a)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 5 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD in substance, with the mechanism corrected in two places. Measured on the arena binary (.kuna-repipe/arena/5/69d7f15a8afd9d6c48b48871/target/crackme_shroud.exe, PE32+ 9,384,960 bytes whose .text alone is 9,325,806 bytes = 99.4% of the image). Every number below is a RATIO comparison, so the two concurrent builder cargo builds on this box cannot change any conclusion. (1) THE 21s IS NOT DISASSEMBLY, IT IS FIXED SETUP: --count 1 = 20.44s, --count 40 = 20.56s, --count 400 = 20.16s. Four hundred instructions cost the same as one. The title's 'disassembling 40 instructions' is a misattribution; nothing in it is per-instruction. (2) THE SEAM: kuna-cli/src/disassemble.rs:293 calls load_program(&load, DriverDefaults::Inventory) -- the same in-process load decompile-all uses. kuna functions on the same binary costs 20.69s and kuna strings costs 31.39s, i.e. every Inventory surface pays the identical fixed toll. (3) THE LEADING CANDIDATE IS DEAD -- IT IS NOT THE DIV-15 LISTING. --option listing off gives BYTE-IDENTICAL json (diff = 0 lines) AND NO SPEEDUP (20.47s vs 21.23s). --mode fast does not help either (20.79s). Do not let a builder start from 'turn the Listing off'; the doc comment in decompile_all.rs already says the Listing is entry-neutral on x86-64 and this measures it. (4) WHAT IT ACTUALLY IS: the whole-.text discovery walk inside bootstrap_from_object. kuna xrefs, which takes DriverDefaults::Query and explicitly builds NO Listing, still costs 12.65s on this binary -- so >60% of the toll is upstream of the Listing. It also is not raw file size: /bin/bash (1,396,520 B ELF) costs 0.33s while illusion.exe (1,292,288 B PE) costs 5.76s, a 17x difference at equal size. It tracks EXECUTABLE bytes: 32 KB PE 0.16s, 1.29 MB PE 5.76s, 9.38 MB PE 20.26s. This binary is an obfuscation crackme that is 99.4% .text, which is why it is the one that surfaced this. (5) THE WRONG-OUTPUT TRAP, AND IT IS THE REASON TO READ THIS BEFORE BRIEFING ANYONE: the obvious fix -- give disassemble a lighter DriverDefaults, or skip the load entirely for a bare --addr target -- would pass the <10s acceptance and break real behaviour. Swapping to Query is not even a win (12.65s still fails the 10s bound). Dropping the discovery bundle loses the inventory that resolve_region / name_at (disassemble.rs:722) / function_extent_at depend on, and on non-x86-64 that bundle IS the discovery: decompile_all.rs's own comment records the exact regression (kuna disassemble <name> answering 'no function matches' for a name kuna functions prints -- RE-need analysis-generated-function-name). THE FIX SHAPE MUST BE LAZINESS OR A CHEAPER WALK, NOT A VARIANT SWAP: decode only the requested window for a bare-address or explicit-range target and pay for the inventory only when the target is a name, or make the .text discovery walk itself cheaper. (6) SCOPE IS WRONG. This is filed scope:small; a lazy-load seam through load_program that keeps every name-resolution consumer honest is not small.
round 5 TRIAGE (captain): SCOPE small -> large, TOUCHES [] -> kuna-cli/src/disassemble.rs + kuna-analysis. The refuter's point (5) is the reason: every cheap fix -- a lighter DriverDefaults, skipping the load for a bare --addr target -- PASSES the <10s acceptance and breaks name resolution, and decompile_all.rs already records that exact regression. A lazy-load seam through load_program that keeps resolve_region / name_at / function_extent_at honest is a design decision, so this goes to a [PROPOSAL] draft PR before anyone writes code. TOUCHES was empty, which meant the need carried NO path lease at all. Track perf is right. NOTE FOR B_VERIFY: the acceptance is a wall_ms median bound and is the one probe in this round that needs a quiet box -- see [[kuna-repipe-phantom-perf-regression]].
