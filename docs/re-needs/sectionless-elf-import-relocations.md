---
need_id: sectionless-elf-import-relocations
title: Sectionless ELF import relocations do not name PLT calls
track: loader
status: open
severity: major
probe_id: p-83c6426f6804
acceptance_id: a-4c6a2962c043
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [605443e333c5d42c3d016f59]
rounds: [4]
first_seen_round: 4
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

Identify puts, read, write, and memcpy from dynamic relocations.

> **Sectionless ELF import relocations do not name PLT calls** (major, `605443e333c5d42c3d016f59`)
> Emitted the puts thunk at 0x10c0 as an unnamed indirect call through dat_7f98. The checker likewise called anonymous thunks. readelf identified puts at GOT 0x7f98 and named all eight PLT relocations.

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
    "decompile",
    "{{BIN}}",
    "0x10c0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\(\\*dat_[0-9a-f]+\\)\\(\\)"
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
    "binary_rel": "bin/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1",
    "binary_sha256": "95c300aedc728b643bf97c39b5e8db88e9ddc40bf4cf337cd6c777929684a5f9",
    "binary_size": 28682,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x10c0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\bputs\\b"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The program-header fallback may load executable bytes without applying PT_DYNAMIC symbol and relocation markup.

## Refutation

_not yet refuted_

## Reference

- `readelf -h -d -r --use-dynamic target/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1` — Despite section-table warnings, readelf identified puts@GLIBC_2.2.5 at 0x7f98, write at 0x7fa0, read at 0x7fb8, and memcpy at 0x7fc8.

## Instances

- `605443e333c5d42c3d016f59` (round 4, tester t-r4-605443e3)

## Decision log

- filed by cluster.py from 1 observation(s)
- split out of the round-4 `sectionless-elf-import-relocations` group by the captain at T_DEDUP: the two observations share a probe signature but not a defect (PLT symbol naming vs. deleted character stores).
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the hypothesis is right in its strong form: there is NO PT_DYNAMIC reader anywhere in kuna-analysis, so a sectionless image gets its bytes and none of its import markup. Evidence on sha c5f4073a. WITNESS: KataVM_L1 has e_shnum=0/e_shoff=0 (readelf reports garbage sh_link/sh_entsize for every section), kuna finds 12 functions and decodes their calls, and 0x10c0 emits (*dat_7f98)() -- the probe reproduces verbatim. ONE-BIT CONTROL: gcc -O0 a main() calling memcpy+puts; copy it and zero ONLY e_shoff/e_shnum/e_shstrndx (0x28/0x3c/0x3e). Sectioned: 'memcpy(v1,"hi",3); puts(v1);'. Same bytes sectionless (decompiled at main's address, since the name is gone too): 'sub_1090(v1,0x2004,3); sub_1070(v1);'. Nothing else differs. THE MECHANISM, all three routes section-gated: (1) loader/elf_plt.rs locates every .plt*/.got/.text by file.sections()/section_by_name (87, 130, 449, 709, 716, 1019) and reads the dynamic table at 985 as file.section_by_name('.dynamic'); (2) its symbol source is file.dynamic_relocations() at 199/229, and object-0.39.1 read/elf/relocation.rs builds that iterator from a SectionTable scan for SHT_REL/SHT_RELA (lines 25-61, 153-167) -- it is section-driven, NOT segment-driven, so it yields nothing here; (3) grep for PT_DYNAMIC across kuna-analysis/src returns NOTHING outside upx/elf.rs (which walks phdrs for PT_LOAD only) and kuna_dynrelocs.rs::relro_range (raw phdr walk, PT_GNU_RELRO only). Also note loader/elf_shdr.rs deliberately CLEARS an out-of-range section table, so this witness lands in exactly the same state as a truly sectionless file. SCOPE CORRECTION FOR TRIAGE: this is not 'small' and not a kuna-decomp change. The object crate cannot supply the data, so a fix is a new PT_DYNAMIC walker in kuna-analysis/src/loader (DT_SYMTAB/DT_STRTAB/DT_SYMENT/DT_JMPREL/DT_PLTREL/DT_PLTRELSZ/DT_PLTGOT, both RELA and REL), feeding elf_plt's existing naming. touches = decompiler/crates/kuna-analysis. WHAT WOULD MAKE A FIX EMIT WRONG OUTPUT: elf_plt maps a stub to its JUMP_SLOT by decoding the .plt range it found BY SECTION NAME; a segment-based fix must derive that range from DT_PLTGOT + the reloc slots it actually sees, never by guessing an executable-segment sub-range, or stubs get named off-by-one-entry. SIBLING: same class as sectionless-elf-loses-string (that one is the listing-tier mapped_ranges blind spot); one image, two independent section-table dependencies. A builder that fixes only mapped_ranges will not move this probe.

### 2026-09-08T07:00Z — captain, round 9 B_DRAIN: track quality → loader

Applying this need's own round-4 refuter, which ended `touches = decompiler/crates/
kuna-analysis` and `this is not 'small' and not a kuna-decomp change`. It upheld the
hypothesis in its strong form: there is no PT_DYNAMIC reader anywhere in `kuna-analysis`, and
all three naming routes are section-gated, so a fix is a new PT_DYNAMIC walker in
`kuna-analysis/src/loader` feeding `elf_plt`'s existing naming. That writes no `settableTable`
row, so the five option leases `TRACK_RESOURCES` gives `quality` were blocking it for nothing.
Rationale and the measurement behind treating a lease-free track as safe (39 non-quality
closes, zero option-plane edits) are on `bulk-decompilation-decodes-pe`; the same
**stop-and-report-`blocked`-if-it-needs-an-option** requirement applies here.

`scope` left at `small` deliberately, against that refuter's "this is not 'small'". `small`
means *dispatch directly*, not *this is trivial*, and the builder prompt already routes a
builder to a `[PROPOSAL]` draft on its own when the mechanism proves large. Filing it `large`
up front would spend a builder on a design for work the refuter has already scoped
concretely (DT_SYMTAB/DT_STRTAB/DT_SYMENT/DT_JMPREL/DT_PLTREL/DT_PLTRELSZ/DT_PLTGOT, RELA and
REL). Carry the refuter's wrong-output warning into the contract instead: the `.plt` range
must be derived from DT_PLTGOT and the relocation slots actually seen, **never** guessed as an
executable-segment sub-range, or stubs get named off by one entry.

Not a duplicate of `sectionless-elf-loses-string` (closed, #451): one image, two independent
section-table dependencies, and that fix was the listing-tier `mapped_ranges` blind spot. It
did not move this probe.
