---
need_id: pe-header-entry-mapped
title: PE header entry cannot be mapped even with an explicit function definition
track: tooling
status: open
severity: blocker
probe_id: p-e69e2a8ab589
acceptance_id: a-28cf2741c644
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f6333c5d40ad448ca40]
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

Read and decompile the declared entry at 0x400154, or explicitly map its file-backed header bytes.

> **PE header entry cannot be mapped even with an explicit function definition** (blocker, `5ab77f6333c5d40ad448ca40`)
> Read/disassemble report no loaded segment; decompile rejects the explicit entry as unmapped. Disabling unmappedentry has no effect. unpack rejects this non-UPX image.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/BUG.cRYPTO.kEYGENME.zip.__x/BUG.exe",
    "binary_sha256": "54d08ffbaba9daebe6f337c37472cfacfcc9b1c2d303c16c8ec65e73ab90f5bf",
    "binary_size": 99469,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x400154",
    "--addr",
    "--define-function",
    "0x400154=entry"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "address 0x400154 is not mapped"
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_headerentry_i386.exe",
    "binary_sha256": "ca5f1405e7c60baccf2f00e0848febf941243e35f070b1a8adfd64ce3df82705",
    "binary_size": 1024,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_headerentry_i386.exe"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x400154",
    "--addr",
    "--define-function",
    "0x400154=entry"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\{"
    ],
    "stderr_absent": [
      "address 0x400154 is not mapped"
    ]
  },
  "notes": "Vendored twin of the reported image (kuna-analysis/tests/fixtures/pe_headerentry_i386.py): e_lfanew 0x0c and a 0xe0-byte optional header over two sections, so the section table runs 0x104..0x154 and AddressOfEntryPoint 0x154 lands one byte past it, in the header page. The dataset original stays the witness in Reproduction; this target is in-repo so the probe runs where there is no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- PE header bytes are omitted from the loaded memory map; no mapping override was found in the CLI or catalog.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/BUG.cRYPTO.kEYGENME.zip.__x/BUG.exe` — Server c5d5f50694 exited with status 1 before registering. Reference comparison is inconclusive: startup failed before entry analysis.

## Instances

- `5ab77f6333c5d40ad448ca40` (round 4, tester t-r4-5ab77f63)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD verbatim: the PE header page is absent from kuna's memory map and the declared entry lives inside it. Witness .kuna-repipe/arena/4/5ab77f6333c5d40ad448ca40/target/BUG.cRYPTO.kEYGENME.zip.__x/BUG.exe is a hand-built tiny PE -- e_lfanew 0xc, 2 sections, SizeOfHeaders 0x200, first section at RVA 0x1000, AddressOfEntryPoint 0x154. The section table sits at file 0x104 and 2*40 bytes end at exactly 0x154, i.e. the entry code starts the byte after the section table, in the header region Windows maps at ImageBase but kuna does not. THE ASYMMETRY IS THE PROOF: 'disassemble --addr 0x437000' (section 2, RVA 0x37000) works, while 0x401000 (section 1, raw size 0) and 0x400154 (headers) both give 'no loaded segment covers it'. The loader is fine; only the header page is missing. THE CONTROL: in a copy, repoint section 1's header to vs=0x200 va=0 rawsz=0x200 rawoff=0 so kuna's OWN section machinery maps file 0..0x200 at 0x400000. 0x400154 then decodes to real code -- XCHG [0x44f250],ESP / POPAD / XCHG EAX,ESP / PUSH EBP / MOVSB / MOV DH,0x80 / CALL [EBX] / JNC 0x40015d, the aPLib-style bit-reader stub -- and the acceptance command returns rc 0 with C for entry(). So mapping the header page alone closes it. THE DESIGN QUESTION IS ALREADY ANSWERED, do not re-litigate it: I ran the control twice, once with section 1's original 0xe00000e0 (executable) characteristics and once forced to 0x40000040 (read, no execute). BOTH give rc 0 and the same C, so a READ-ONLY header mapping is sufficient, and read-only is the right choice -- it matches what Windows actually does (PAGE_READONLY), and it keeps executable-region scanning away from MZ/PE header bytes so 'functions' cannot invent entries in the header of every PE in the corpus. kuna already distinguishes: read-only, 'disassemble --addr 0x400154' correctly answers 'is in a non-executable data section ... pass --as code' instead of pretending. THE GUARD: map exactly [ImageBase, ImageBase + SizeOfHeaders), clamped to the file length AND to the first section's RVA, so a malformed SizeOfHeaders cannot shadow section 1. LIMIT OF THIS CONTROL, stated honestly: it REPLACED section 1's mapping rather than adding a region, which is why 'functions' drops from 2 to 0 on my controls -- that is an artifact of clobbering RVA 0x1000-0x37000, not evidence about the real fix, and it does not measure whether adding a header region creates discovery noise on well-formed PEs. Read-only makes that question structurally moot, but a builder should still count functions across a few normal PEs before and after.
- round 4 BUILDER: closed by mapping the PE header page (loader/pe_headers.rs, ObjectFormat::header_region). The refuter's diagnosis and its read-only/clamp design were both confirmed and implemented as written. The Acceptance block above was retargeted from the dataset witness to a vendored 1024-byte PE32 twin (kuna-analysis/tests/fixtures/pe_headerentry_i386.exe) so the probe could promote into tests/cli/, where there is no dataset; it was run against the dataset original FIRST and passes there too (exit 1 -> exit 0 with C for entry(), and `disassemble --addr 0x400154 --as code` decodes the aPLib bit-reader stub). acceptance_id is unchanged at a-28cf2741c644 because it derives from cmd+expect, both of which are identical.
