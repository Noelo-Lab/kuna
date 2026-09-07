---
need_id: strings-rejects-mach-o
title: strings rejects a Mach-O that decompile-all and read accept
track: tooling
status: closed
severity: major
probe_id: p-c7723f06652b
acceptance_id: a-4380c8226500
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [5ab77f5633c5d40ad448c29b]
rounds: [7]
first_seen_round: 7
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/strings.rs, decompiler/crates/kuna-analysis/src/loader/macho_fat.rs]
scope: small
regression_of: no-strings-inventory
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Enumerate selectors and success/error strings with addresses and owning functions.

> **strings rejects a Mach-O that decompile-all and read accept** (major, `5ab77f5633c5d40ad448c29b`)
> strings --json exits 1 with Unsupported file format and no JSON. Explicit --slice x86_64 also fails. kuna read exposes NUL-terminated selectors and Code Valid/Code Invalid in the same image.

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
    "strings",
    "{{BIN}}",
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

- The strings analyzer loader supports fewer formats than the decompiler loader.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5633c5d40ad448c29b` (round 7, tester t-r7-5ab77f56)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 7 REFUTER: hypothesis **overturned** (was inconclusive). REFUTED IN-TICK (captain, r7 T_REFUTE, sha dfc7acc5). Symptom stands exactly as filed; the CAUSE as stated is wrong and would send a builder to the wrong file. THE TWO PATHS USE THE SAME LOADER. Filed cause: 'the strings analyzer loader supports fewer formats than the decompiler loader.' They are the same loader -- kuna_analysis::loadimage_object::parse_object. What differs is a missing PRE-STEP. The target is a Mach-O UNIVERSAL (fat) binary ('Mach-O universal binary with 2 architectures'), and object::File::parse has no fat arm, so it returns literally 'Unsupported file format'; the canonical peel lives in kuna-analysis/src/loader/macho_fat.rs (select_fat_slice, default preference x86-64 -> arm64 -> first) and is applied at ONE point, the engine dispatch bootstrap_from_object (kuna-console/src/engine.rs:3193-3195), which substitutes the slice bytes before the loader runs. kuna-cli/src/strings.rs:90-93 reads the raw file with elf_shdr::read_image and calls parse_object on those bytes DIRECTLY, never touching macho_fat -- so it dies on the fat header while 'kuna functions' on the same file returns 84 functions, and the tester's 'read'/'decompile-all' worked for the same reason. AND --slice IS INERT ON THIS PATH, WHICH EXPLAINS THE TESTER'S SECOND FAILURE. StringsArgs.slice is parsed (strings.rs:55, :420) and threaded only into the SECOND stage -- the load_program call at :165-181 that attaches reference edges -- never into the :92 parse_object. So '--slice x86_64' fails with the identical message; it is not that the override was rejected, it is that nothing reads it before the parse. Fix shape is small and local: peel in query() before parse_object, honouring args.slice via SlicePref, i.e. the same two lines engine.rs already runs. Scope small is right; touches decompiler/crates/kuna-cli is right. WORTH CHECKING FOR SIBLINGS IN THE SAME PR: any other subcommand that calls parse_object on raw bytes instead of going through the engine dispatch has the same hole (grep parse_object across kuna-cli). ACCEPTANCE TRAP. Acceptance is only exit 0 + stdout_is_json, which an empty inventory satisfies -- a fix that peels but scans nothing, or that returns {"strings":[]}, closes the need with the capability still absent. B_DONE should require that the output actually contains the strings the tester saw in the same image ('Code Valid' / 'Code Invalid' and the Objective-C selectors) and that --slice x86_64 and --slice arm64 select DIFFERENT inventories, which is the only check that proves the override is now live rather than merely non-fatal.
- round 7 REFUTER: hypothesis **overturned**. CAPTAIN r7 B_PLAN DISPATCH INSTRUCTION -- YOU OWN TWO NEEDS, NOT ONE. This builder holds the cluster lease for BOTH strings-rejects-mach-o AND adding-summary-makes-working ('Adding --summary makes the working Mach-O function inventory fail'), because the r7 refuters proved they are ONE gap: kuna-cli calls kuna_analysis::loadimage_object::parse_object on RAW file bytes at EIGHT sites (decompile_graph.rs:179, strings.rs:92, xrefs.rs:207, decompile_all.rs:320/608/1098/1476/1609) while the Mach-O fat-header peel (loader/macho_fat.rs select_fat_slice) is applied at exactly ONE point in the tree, kuna-console/src/engine.rs:3193-3195 bootstrap_from_object. Fix it once, at or below the shared parse_object entry, and honour the existing --slice override there (StringsArgs.slice is parsed today but threaded only into the second-stage load_program, so --slice is INERT on these paths). Read docs/re-needs/adding-summary-makes-working.md for its own repro and acceptance; BOTH acceptances must pass and both need docs must be updated in your PR. TWO ACCEPTANCE TRAPS -- both probes assert only 'exit 0 + stdout_is_json', which an EMPTY result satisfies, so the captain will NOT close either need on exit-0 alone: (1) 'kuna functions <bin> --summary --json' must report the same total, 84, that the plain inventory reports, and 'kuna functions <bin> --reachable-from entry --json' must work on the same image (that is the real trigger -- decompile_all.rs:245 builds the call graph when reachable_from.is_some() || summary, so --summary is NOT the discriminator); (2) 'kuna strings <bin> --json' must actually contain the strings the tester saw ('Code Valid' / 'Code Invalid' and the Objective-C selectors), and --slice x86_64 vs --slice arm64 must select DIFFERENT inventories -- that difference is the only proof the override became live rather than merely non-fatal. Third confirmed site to fix and check: 'kuna xrefs <bin> --to 0x100001d2b --json'. Both acceptances are now TARGET-BOUND to the dataset copy of the witness (challenges/5ab77f5633c5d40ad448c29b/bin/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe, sha 5eacec69bcf1..., a Mach-O universal image with 2 architectures) and both were re-measured runnable-and-still-FAILING at sha 0bd41f10, so they are honest closing criteria -- do not re-cut them onto an easier fixture without keeping these clauses.
- round 7 BUILDER b-r7-strings-rejects-: CLOSED. Symptom and refuted cause both stand. `object::File::parse` has no fat arm, so a universal Mach-O must be peeled to one slice before parsing; the peel (`loader/macho_fat.rs`) was applied at exactly one point, the engine dispatch, while `kuna-cli` parsed the raw file bytes. The peel is now a shared function (`macho_fat::peel_fat_image`) applied at the canonical image read (`elf_shdr::read_image`), where the ELF section-table and PE data-directory repairs already live, and the dispatch calls the same policy. `--slice`/`--target` resolve through one function (`macho_fat::slice_pref`) and are threaded explicitly rather than through the `KUNA_MACHO_SLICE` env bridge, which is a RAII guard scoped to `load_program` and already restored by the time these surfaces re-parse. ACCEPTANCE TRAPS ANSWERED: strings on the witness returns 131 rows including `Code Valid`, `Code Invalid` and the Objective-C selectors; `--slice x86_64` (131 rows @0x1000029xx) and `--slice i386` (129 rows @0x36xx) select different inventories, so the override is live -- note the witness is x86_64+i386, NOT arm64. `kuna xrefs --to 0x100001d2b --json` answers too. NOTE ON THE `--reachable-from entry` CLAUSE: `entry` is a name lookup (`resolve_function_spec`), not a magic token, and this image has no symbol named `entry`; post-fix it answers `no function named "entry"` instead of `Unsupported file format`, and `--reachable-from 0x100001c54` returns the 4 functions the summary's `reachable_from_entry` reports. Promoted to tests/cli/strings-rejects-mach-o.json, retargeted onto the vendored 2-slice `macho_fat` fixture (CI has no dataset) -- the pre-fix binary fails on that fixture identically -- with clauses an empty inventory cannot satisfy.
