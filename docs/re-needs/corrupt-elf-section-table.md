---
need_id: corrupt-elf-section-table
title: Corrupt ELF section table blocks analysis despite readable program headers
track: tooling
status: open
severity: blocker
probe_id: p-b0eecb741356
acceptance_id: a-e4f84764b32f
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [5ab77f6633c5d40ad448cc64]
rounds: [3]
first_seen_round: 3
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/loader]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Load through program headers, optionally overriding corrupt section metadata, and enumerate executable functions.

> **Corrupt ELF section table blocks analysis despite readable program headers** (blocker, `5ab77f6633c5d40ad448cc64`)
> functions, decompile-all, strings, and disassemble exited 1 with Invalid ELF section header offset/size/alignment. readelf recovered entry 0x80492d0 and both LOAD segments. Clearing section metadata in a copy allowed parsing but still yielded zero functions and zero code bytes.

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
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "Invalid ELF section header offset/size/alignment"
    ]
  },
  "target": {
    "binary_rel": "bin/0x1d01ebcc.tar.gz.__x/0x1d01ebcc.tar.__x/0x1d01ebcc",
    "binary_sha256": "821fab4ad881c3ad26f79cbd3c700b0dcc1faf6643449f896cc8f4b40890bd0e",
    "binary_size": 161156,
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
    "functions",
    "{{BIN}}",
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/corruptshdr_i386",
    "binary_sha256": "1c969d3da25e1b7a3426aa2fec04497d919c532e4ce97e17a624257cfc62eb27",
    "binary_size": 107,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/corruptshdr_i386",
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
    ],
    "stderr_absent": [
      "not in recognized object file format"
    ]
  },
  "notes": "Vendored twin of the reported image: a 107-byte ELF32 carrying its e_shoff/e_shnum/e_shstrndx verbatim (57005/57007/47806) over two functions, with one intact PF_X PT_LOAD. The dataset original stays the witness in Reproduction; this target is in-repo so the probe promotes into tests/cli/ and runs where there is no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The loader requires valid section metadata before exploiting program-header mappings; removing the table also leaves code classification empty.

## Refutation

_not yet refuted_

## Reference

- `readelf -h -l target/0x1d01ebcc.tar.gz.__x/0x1d01ebcc.tar.__x/0x1d01ebcc` — Warns about section headers but emits the entry point, LOAD segments, interpreter, and DYNAMIC segment; exits 0.

## Instances

- `5ab77f6633c5d40ad448cc64` (round 3, tester t-r3-5ab77f66)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_TRIAGE r3: track tooling CONFIRMED (kind=missing-capability), touches CORRECTED kuna-cli -> kuna-analysis/src/loader: `kuna functions --json` returning nothing is a loader-tier fallback (use the program headers when the section table is unusable), not a CLI defect. Deliberately NOT filed on the `loader` track -- builder_prompt.md has sections for tooling/quality/perf only, so a `loader` need would reach its builder with no protocol at all.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- round 3 REFUTER: hypothesis **overturned** (was inconclusive). Captain refuted BY MEASUREMENT in-tick on the main tree build (kuna 20:01, cf5234ac-era). The filed cause -- 'the loader requires valid section metadata before exploiting program-header mappings' -- is WRONG in its load-bearing half. Measured: (1) the stock probe fails as filed (exit 1, 'Invalid ELF section header offset/size/alignment'); the header is deliberately corrupt -- e_shoff=57005 (0xDEAD), e_shnum=57007, e_shstrndx=47806 -- while the 9 program headers are intact (entry 0x80492d0, LOAD 0x08048000+0x26c6c R E). (2) With e_shoff/e_shnum/e_shstrndx zeroed in a copy, 'kuna functions --json' exits 0 with count=0 -- the tester's second observation reproduces exactly. (3) But 'kuna decompile <copy> --addr 0x80492d0' DECOMPILES CLEANLY (emits sub_80492d0 calling sub_8048c60), so memory mapping already comes from PT_LOAD and needs no sections at all. The real shape is TWO independent gaps: (a) the loader hard-errors on an unusable section table instead of falling back to the program headers, and (b) function DISCOVERY returns nothing without sections even though EntryDiscoveryPass (passes.rs:90, always-on, sources ELF e_entry) should have the entry seed -- i.e. discovery, not mapping, is what yields zero. CONSEQUENCE FOR THE BUILDER: a fix that only tolerates the corrupt table will exit 0 with count=0 and the acceptance probe (count gt 0) STILL FAILS; it must also seed discovery from e_entry and the executable PT_LOAD ranges. Symptom and severity stand; credibility unchanged.
- round 3 BUILDER (b-r3-corrupt-elf-sect): hypothesis CONFIRMED as the refuter restated it -- two independent gaps, not one. (a) `object::File::parse` validates the section table eagerly, so the loader rejected an image whose program headers were intact; (b) `analyzers/entry::executable_sections` walks `file.sections()` alone, so with no section table the "is this plausible code?" oracle rejected the image's own `e_entry` and discovery returned nothing. Both closed: measured on the dataset original, `kuna functions --json` goes exit 1 / parse error -> exit 0 / count 24, `decompile-all` 24, `strings` 83 (scanned: segments), `disassemble 0x80492d0` 29 instructions. The acceptance target was re-pointed from the dataset binary to a vendored 107-byte twin carrying the same three corrupt header values, so the probe promotes into tests/cli/ and runs in CI, which has no dataset.
