---
need_id: adding-summary-makes-working
title: Adding --summary makes the working Mach-O function inventory fail
track: tooling
status: closed
severity: minor
probe_id: p-6b42363e7bb9
acceptance_id: a-ffe743c852b5
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5633c5d40ad448c29b]
rounds: [7]
first_seen_round: 7
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile_graph.rs, decompiler/crates/kuna-cli/src/decompile_all.rs, decompiler/crates/kuna-cli/src/xrefs.rs]
scope: small
regression_of: whole-binary-json-untriageable
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

A concise entry, reachability, and size summary of the VM methods.

> **Adding --summary makes the working Mach-O function inventory fail** (minor, `5ab77f5633c5d40ad448c29b`)
> Plain functions --json returns 84 entries. Adding --summary exits 1 with Unsupported file format and no JSON, including when filtered to checkCode.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--summary",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "Unsupported file format"
    ]
  },
  "target": {
    "binary_rel": "bin/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe",
    "binary_sha256": "5eacec69bcf1be53f115fbc0ed93e032a888000b5aa9ac77e4da44325683ef41",
    "binary_size": 54224,
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
    "functions",
    "{{BIN}}",
    "--summary",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true
  },
  "target": {
    "binary_rel": "bin/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe",
    "binary_sha256": "5eacec69bcf1be53f115fbc0ed93e032a888000b5aa9ac77e4da44325683ef41",
    "binary_size": 54224,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Summary invokes a second analysis loader that rejects the image after inventory loading succeeds.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5633c5d40ad448c29b` (round 7, tester t-r7-5ab77f56)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 7 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD IN SUBSTANCE, WRONG IN SCOPE -- and it is the SAME GAP as strings-rejects-mach-o. Reproduced on the arena binary (arena 5ab77f5633c5d40ad448c29b, target/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe, a Mach-O UNIVERSAL/fat image): 'kuna functions <bin> --json' returns 84 entries; adding --summary exits 1 with 'could not parse <bin>: Unsupported file format'. The filed hypothesis ('summary invokes a second analysis loader that rejects the image after inventory loading succeeds') is CORRECT that a second parse is what dies, so the verdict is upheld -- but the '--summary' framing is wrong and a builder who takes it literally will gate the fix on the wrong flag. THE A/B THAT SETTLES IT: 'kuna functions <bin> --reachable-from entry --json' fails IDENTICALLY with NO --summary. decompile_all.rs:245 builds the call graph when '--reachable-from is_some() || summary', so the trigger is CallGraph::build, not the summary document. THE MECHANISM IS THE ONE ALREADY PINNED BY strings-rejects-mach-o: decompile_graph.rs:179 calls kuna_analysis::loadimage_object::parse_object(&*bytes) on the RAW file bytes and never peels the fat header. The peel lives in loader/macho_fat.rs and is applied at exactly ONE point in the tree, engine.rs:3193-3195 bootstrap_from_object -- which is why the inventory path works and every raw-parse_object path does not. --slice x86_64 is INERT here too (same as on the strings path). THIS IS A FAMILY, NOT A FLAG: grep finds EIGHT raw parse_object call sites in kuna-cli (decompile_graph.rs:179, strings.rs:92, xrefs.rs:207, decompile_all.rs:320/608/1098/1476/1609), and I confirmed a third one live -- 'kuna xrefs <bin> --to 0x100001d2b --json' fails with the same message. ACTION FOR T_TRIAGE: this need and strings-rejects-mach-o must go to ONE builder (add the sibling cluster lease), because one fix -- peel the fat header once, at or below the shared parse_object entry -- flips both acceptances, and split across two builders they collide on the same files. ACCEPTANCE TRAP: this need's acceptance is only 'exit 0 + stdout_is_json', which a summary over an EMPTY inventory passes. Require the summary's total to equal the 84 the plain inventory reports, and require --reachable-from to work on the same binary, or a fix that merely swallows the parse error closes the need with the capability still missing.
- round 7 REFUTER: hypothesis **upheld**. CAPTAIN r7 B_PLAN: THIS NEED IS NOT DISPATCHED ON ITS OWN. It is owned by the builder holding strings-rejects-mach-o, which took this need's cluster lease too (same one-line gap: the Mach-O fat-header peel exists only at engine.rs:3193-3195 while kuna-cli calls parse_object on raw bytes at eight sites). All dispatch instructions and both acceptance traps live in that need's decision log; the closing PR must flip BOTH acceptances. Acceptance target bound to challenges/5ab77f5633c5d40ad448c29b/bin/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe (sha 5eacec69bcf1...), re-measured runnable and still FAILING at sha 0bd41f10. The 84-function total from the plain inventory is the number the --summary total must match.
- round 7 BUILDER b-r7-strings-rejects-: CLOSED by the same one-line gap as strings-rejects-mach-o (that need's decision log carries the mechanism). `--summary`/`--reachable-from` build a call graph, which re-parses the image outside the engine load; that raw parse now goes through `elf_shdr::read_image`, which peels the Mach-O fat header. On the witness the summary's size buckets total 10+34+19+17+4 = 84, matching the plain inventory's 84. `--reachable-from 0x100001c54` returns 4 functions, matching `reachable_from_entry: 4` (`--reachable-from entry` is not a magic token -- see the sibling need). SECOND DEFECT FOUND IN THE SWEEP AND FIXED HERE: the summary took its entry from a raw `std::fs::read`, so an image that only parses after the loader's header repairs (`corruptshdr_i386`, `pe_datadircount_i386.exe`) reported `entry: null` / `reachable_from_entry: null` beside a populated inventory. Promoted to tests/cli/adding-summary-makes-working.json on the vendored `macho_fat` fixture, with count/total pinned to the plain inventory's 6 so a summary over an EMPTY inventory cannot pass.
