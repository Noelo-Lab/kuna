---
need_id: sectionless-elf-loses-string
title: Sectionless ELF loses string owners and RIP-relative data xrefs
track: tooling
status: closed
severity: major
probe_id: p-dde3912121c9
acceptance_id: a-d4ec9195af39
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [605443e333c5d42c3d016f59]
rounds: [4]
first_seen_round: 4
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/loader, decompiler/crates/kuna-analysis/src/analyzers/strings]
scope: small
regression_of: no-strings-inventory
pr: "451"
closed_in_round: 4
closing_pr: "451"
reject_reason: null
---

## Symptom

Navigate from the Correct string to main.

> **Sectionless ELF loses string owners and RIP-relative data xrefs** (major, `605443e333c5d42c3d016f59`)
> Returned the string at 0x6b22 with xrefs_count 0 and functions []. xrefs --to 0x6b22 also returned zero. Disassembly shows LEA RDI,[0x6b22] at 0x1181, and decompilation passes that address to the output function.

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
    "strings",
    "{{BIN}}",
    "--filter",
    "Correct",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 1
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
  "cmd": [
    "{{KUNA}}",
    "strings",
    "{{BIN}}",
    "--filter",
    "Correct",
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/sectionless_x86_64",
    "binary_sha256": "0b3c1f3491912be34b1af5421578de88e46cf8cdcdd49979e43241049ab5f0e9",
    "binary_size": 8206,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/sectionless_x86_64",
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
  "notes": "Vendored twin of the reported image: an ELF64 PIE with e_shoff=0 and two PT_LOADs, one PF_X holding a function that forms LEA RDI,[0x2000] and one PF_R holding the string there. The dataset original (KataVM_L1, 0x1181 LEA RDI,[0x6b22]) stays the witness in Reproduction and passes the same assertions; this target is in-repo so the probe promotes into tests/cli/ and runs where there is no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Segment-only analysis may recover call edges while omitting RIP-relative data references.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `605443e333c5d42c3d016f59` (round 4, tester t-r4-605443e3)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, mechanism located exactly, and the hypothesis bullet is right verbatim rather than approximately: call edges ARE recovered while RIP-relative data references are ALL dropped. Evidence on the witness (KataVM_L1, sectionless PIE, sha 6188e204). `kuna disassemble --addr 0x1170` shows LEA RDI,[0x6b22] at 0x1181 and MOV RDI,[0x8010] at 0x1195; `kuna xrefs --from 0x1170` returns 4 rows and every one is kind=call (0x1178->0x12d0, 0x1188->0x10c0, 0x11b4->0x10c0, 0x119c->0x1130) -- not one data row, so the walk decoded the exact instructions and threw their data operands away. CONTROL, and it is a one-bit control: gcc -O0 a 3-line puts("[+] Correct!") program, copy it, remove ONLY the section header table. Sectioned: strings --filter Correct reports xrefs_count 1, functions [main]. Same bytes sectionless: xrefs_count 0, functions []. Nothing else differs. THE LINE: listing/xrefs.rs:466-467. `sections_are_runtime = !exec.is_empty() && seeds.iter().any(|s| in_range(&exec,s))`, then `mapped = if sections_are_runtime { mapped_ranges(file) } else { Vec::new() }`. mapped_ranges() iterates file.sections(), so with no section table exec is empty, sections_are_runtime is false, and mapped is EMPTY. Every data-ref candidate then dies at line 1092 (`!in_range(mapped, value) -> continue`) and at 1060/867. Control flow never consults mapped, which is why calls survive -- the asymmetry the tester saw is structural, not partial. WHAT A BUILDER MUST NOT DO: the guard is not a bug. Its comment says why it exists -- an ET_REL's section addresses are pre-link and describe a different address space, so classifying data refs against them would make every one wrong. It conflates two conditions: (a) sections exist but are not the runtime layout (ET_REL) -- keep declining; (b) there is NO section table -- exec.is_empty(), and here the PT_LOAD table IS the runtime layout and is sitting right there. Only arm (b) may take a segment fallback. engine.rs::function_entries_executable already has this exact sections.is_empty() shape. RISK a builder should price: PT_LOAD spans are coarser than the section list (inter-section padding becomes 'mapped'), so expect a few more Data rows on sectionless images; looks_like_address() still bounds it. FREE RIDERS: kuna_poolref.rs:110 and kuna_picbase.rs:757 read the same `mapped`, so the fallback un-blinds literal-pool and PIC-base refs on sectionless images too. RELATION TO THE SIBLING NEED: this is NOT the same bug as zero-function-sizes-make. That one is console funcextent.rs clipping against loader section spans; this one is the analysis-tier listing walk's mapped_ranges. Two paths, one class. strings.rs:232 attributes an owner via index.function_containing(), which comes from the xref walk's own decode and NOT from function extents, so the zero sizes do not block this probe -- fixing mapped alone should flip the acceptance from len_eq 0 to len_gt 0.
- round 4 BUILDER (b-r4-sectionless-elf-): CLOSED. mapped_ranges() in listing/xrefs.rs now falls back to the PT_LOAD segments when file.sections() is empty, mirroring entry::executable_sections. One correction to the refutation, which does not change the fix: exec is NOT empty on a section-less image (executable_sections already falls back to segments), so sections_are_runtime is TRUE there; mapped is empty because mapped_ranges itself iterates file.sections(). Gating on sections_are_runtime would therefore have been the wrong lever. Coarseness risk priced and measured at zero: on a control pair differing only in the presence of the section header table, the section-less image now answers 13 data references and the sectioned one answers 13 -- identical rows. Acceptance retargeted to a vendored twin (sectionless_x86_64, e_shoff=0, LEA RDI,[0x2000]) after passing on the dataset witness, and promoted to tests/cli/sectionless-elf-loses-string.json.
