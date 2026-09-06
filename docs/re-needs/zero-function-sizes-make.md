---
need_id: zero-function-sizes-make
title: Zero function sizes make size-based triage discard the entire binary
track: tooling
status: closed
severity: major
probe_id: p-9b9b0d274cdf
acceptance_id: a-fe6b8034e76c
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [605443e333c5d42c3d016f59]
rounds: [4]
first_seen_round: 4
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-console/src/funcextent.rs, decompiler/crates/kuna-console/src/engine.rs]
scope: small
regression_of: whole-binary-json-untriageable
pr: null
closed_in_round: 4
closing_pr: null
reject_reason: null
---

## Symptom

Find the large VM interpreter using size-based inventory filtering.

> **Zero function sizes make size-based triage discard the entire binary** (major, `605443e333c5d42c3d016f59`)
> All 12 discovered functions had size 0. --min-size 1 returned count 0, total 12, error null. The summary reported code_bytes 0 despite successful decompilation of the large interpreter.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1",
    "binary_sha256": "95c300aedc728b643bf97c39b5e8db88e9ddc40bf4cf337cd6c777929684a5f9",
    "binary_size": 28682,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--min-size",
    "1",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "total",
        "op": "gt",
        "value": 0
      },
      {
        "path": "count",
        "op": "eq",
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
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--min-size",
    "1",
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/noshdr_x86_64",
    "binary_sha256": "f960c89e9a1308857a344bec32dd3a9b89535c235a428d5cde56ad0657921d65",
    "binary_size": 304,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/noshdr_x86_64",
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "gt",
        "value": 0
      }
    ]
  },
  "notes": "Vendored twin of the reported image: a 304-byte ELF64 PIE with e_shoff/e_shnum/e_shstrndx all zero and three PT_LOADs, one executable over two functions (noshdr_x86_64.py). Both entries report size 0 before the fix, 16 and 6 after. The dataset original stays the witness in Reproduction; this target is in-repo so the probe promotes into tests/cli/ and runs where there is no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Segment-only discovery may not propagate recovered extents into inventory metadata.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `605443e333c5d42c3d016f59` (round 4, tester t-r4-605443e3)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK BY THE CAPTAIN, on the filed witness 605443e3, sha 6188e204, arena .kuna-repipe/arena/4/605443e333c5d42c3d016f59/target/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1.

SYMPTOM REPRODUCED AND ISOLATED BY CONTROL. `kuna functions <bin> --json` on the witness returns 12 functions, ALL with size 0. The control matters more than the repro: the same binary from a different round-4 arena, graphy (sectioned static ELF), returns 55 functions with ZERO of them size 0 (sub_1002c90 size 8704). So `size` is not structurally dead -- it works, and it dies on this binary specifically. `file` reports the witness as a PIE with 'no section header'.

THE HYPOTHESIS IS UPHELD AND THE MECHANISM IS EXACT, NOT INFERRED. 'Segment-only discovery may not propagate recovered extents into inventory metadata' is right. kuna-console/src/funcextent.rs computes every inventory extent as `clip(entry, next_entry, code_spans(sections))`, and `clip` opens with: if no CODE span contains the entry, `return 0`. `code_spans()` is built ONLY from the loader section table (`ConsoleProgram::sections`, filtered on `section_flags::CODE`). A sectionless ELF has an empty section table, so `code_spans` is empty, so EVERY entry takes the early return. 12 of 12 is not a coincidence, it is the only reachable outcome. The 'code_bytes 0 in the summary' half of the report is the same root -- the summary sums these same extents.

WHY IT IS A REAL DEFECT AND NOT THE DOCUMENTED BEHAVIOUR. funcextent.rs deliberately specifies 0 as 'an entry outside every CODE section is an import pointer slot or an undefined external -- an address, not a body'. That semantic is sound when a section table EXISTS. With no section table the same branch silently changes meaning from 'this entry is not a body' to 'we cannot see any sections', and every real function inherits the sentinel reserved for non-bodies. Downstream, `--min-size 1` is documented triage and it discards all 12 -- count 0, total 12, error null -- which is the tester's actual loss: the filter reports emptiness with no error to distinguish it from a binary that really has nothing.

THE FIX DIRECTION IS SOUND, AND THIS FILE ALREADY CONTAINS THE PRECEDENT. Falling back to the executable PT_LOAD segment spans when the section table is empty preserves the field's stated contract exactly -- funcextent.rs defines the number as an UPPER BOUND clipped to the next entry, never as an exact body, so a coarser container is the same kind of answer, only looser at the last entry of a segment. And engine.rs::function_entries_executable already does precisely this shape of thing eleven lines away ('if sections.is_empty() { return self.function_entries_canonical() }', commented 'Loaders without section metadata retain the complete inventory'), so a sectionless fallback is the idiomatic move here, not a new concept.

THE ONE WAY THIS FIX GOES WRONG, WHICH THE BUILDER MUST GUARD. The fallback must trigger only when `code_spans` is EMPTY, never per-entry when an entry merely misses the spans it has. Widening it to 'entry in no CODE span -> use a segment' would hand a nonzero size to exactly the entries the 0 sentinel exists for -- PE import pointer slots and undefined externals -- and round 4 has a separate open need, `whole-binary-decompilation-treats`, about import pointer slots already being mistaken for function bodies. A per-entry fallback would make that need worse while closing this one.

TRIAGE FINDING, RELEVANT TO B_PLAN AND NOT VISIBLE FROM THE NEED DOCS. All THREE round-4 sectionless needs -- `zero-function-sizes-make`, `sectionless-elf-loses-string` (rank 2, same score), `sectionless-elf-import-relocations` -- carry the SAME single challenge, 605443e333c5d42c3d016f59, this binary. One tester, one sectionless PIE, three filed symptoms. They are the same CLASS (section-keyed machinery degrading with no section table) but plausibly different code paths: this one is console funcextent, the other two are analysis-tier string/reloc passes. Do NOT merge them on my word -- but do not dispatch them to three concurrent builders either, because they will contend and may each half-fix the loader. Confirm the shared root first, then prefer one builder taking the sectionless loader gap whole.

LINEAGE: filed `regression_of: whole-binary-json-untriageable` (closed round 1, PR #367). That is the right ancestor for the FEATURE (`--min-size` triage) but this is not a regression of it -- #367 shipped the filter, and the filter works; what is missing is the extent it filters on, on a binary class round 1 never saw.

- round 4 CAPTAIN (observe tick, 06:0xZ): independently re-derived the mechanism and STRENGTHENED THE CONTROL. The refutation above compared two *different* binaries (witness vs graphy), which leaves "these binaries differ in other ways" open. The same-binary control closes it: `cp /bin/true /tmp/true_noshdr` then zero `e_shoff`/`e_shnum`/`e_shstrndx` in the ELF64 header (offsets 0x28/0x3C/0x3E) and nothing else. `kuna functions /bin/true --json` -> count 54, ZERO of 54 size 0 (`_DT_INIT` 27, `sub_1020` 576). `kuna functions /tmp/true_noshdr --json` -> count 11, ELEVEN of 11 size 0. Identical bytes in every mapped segment; the only delta is the section table. That is the mechanism at `funcextent.rs::clip` (early `return 0` when no CODE span contains the entry, `code_spans()` built only from `ConsoleProgram::sections`) proven by ablation rather than by reading.

- SECOND FACT THE CONTROL EXPOSED, not previously recorded anywhere and NOT part of this need: stripping the section table also drops DISCOVERY from 54 functions to 11 on the same bytes. The extent bug is what this need is about, but a sectionless ELF is additionally losing ~80% of its function inventory. That is a separate defect with a separate locus, and it makes the sectionless family a four-symptom class, not three. Worth a filed need in round 5 if a tester hits it; do not widen this need's scope to cover it.

- TRACK CORRECTED quality -> tooling, and TOUCHES corrected off the broad `decompiler/crates/kuna-decomp` root to `kuna-console/src/funcextent.rs` + `kuna-console/src/engine.rs`. Rationale: the probe is `kuna functions --json` and emits no C at all (the precedent that closed `xrefs-unify-pe-import`), and the measured locus is the console inventory tier, not the decompiler. TWO DISPATCH CONSEQUENCES. (1) This need is now OUT of the quality-lease monopoly, so it can run beside a quality builder. (2) It is NOT part of the five-need `kuna-analysis/src/loader` block either -- the narrow fix touches neither the loader nor kuna-decomp, so it contends with nothing currently open. That makes it the cheapest dispatchable need in the round. THE CAVEAT THAT DECIDES THIS: the fix direction must be the funcextent fallback (use executable PT_LOAD segment spans when `code_spans` is empty), NOT teaching the loader to synthesize sections from segments. The loader route would take the loader lease, collide with the five-need block, and change `function_entries_executable`'s documented sectionless behaviour ("Loaders without section metadata retain the complete inventory") as a side effect. Bind that in the contract, or the builder may pick the wide route and re-enter the loader contention this correction just removed.
