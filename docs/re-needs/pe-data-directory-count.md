---
need_id: pe-data-directory-count
title: PE data-directory count rejection blocks all code inspection
track: tooling
status: open
severity: blocker
probe_id: p-498998c72431
acceptance_id: a-554672734890
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5c33c5d40ad448c67e]
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

Load the Invius-packed PE and inspect its entry stub, using a tolerant loader or analyst override if necessary.

> **PE data-directory count rejection blocks all code inspection** (blocker, `5ab77f5c33c5d40ad448c67e`)
> functions, decompile-all, and direct-address disassemble exit 1 with Invalid PE number of RVA and sizes. The catalog/manual exposed no loader override. unpack rejects this non-UPX image as documented.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/bustme.zip.__x/bustme1.exe",
    "binary_sha256": "301a5910aabeff950449d98b4378dbfa31e7d2ba6f0336e3132e97512170ba00",
    "binary_size": 77824,
    "binary_source": "dataset"
  },
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
      "Invalid PE number of RVA and sizes"
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/pe_datadircount_i386.exe",
    "binary_sha256": "a7bd3c04b1159dfc6d31a8426d26d1c6649b23b080f0c3845770313f25a51e9f",
    "binary_size": 1024,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/pe_datadircount_i386.exe",
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
  "notes": "Vendored twin of the reported image: a 1024-byte PE32 carrying the reported NumberOfRvaAndSizes verbatim (1531532893) in a 224-byte optional header that holds the 16 real directories, over two functions. The dataset original stays the witness in Reproduction; this target is in-repo so the probe promotes into tests/cli/ and runs where there is no dataset."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The loader may reject a deliberately unusual data-directory count before mapping inspectable code; unverified.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/bustme.zip.__x/bustme1.exe --backend ida --project-dir notes/ida --timeout 60 --json` — Reference failed too: exit 1, server exited before registering. Log tail shows headless IDA startup without a parser diagnosis. This does not establish IDA superiority or a shared failure cause.

## Instances

- `5ab77f5c33c5d40ad448c67e` (round 4, tester t-r4-5ab77f5c)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 4 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, mechanism exact, and the acceptance is provably reachable by changing ONE 4-byte field. Witness .kuna-repipe/arena/4/5ab77f5c33c5d40ad448c67e/target/bustme.zip.__x/bustme1.exe: NumberOfRvaAndSizes reads 1531532893 (Invius trashed it; the section names are trashed too). The one-bit control: copy the file, poke that field (file offset 0xf4) to 16, change nothing else. Before: 'kuna functions' exits 1 with 'Invalid PE number of RVA and sizes'. After: rc 0, count 54, with GetProcAddress/LoadLibraryA resolved. So the acceptance probe closes on the clamp alone -- nothing downstream of the header is actually broken. WHERE IT REJECTS, AND WHY THE OBVIOUS FIX IS NOT AVAILABLE: the error is NOT kuna's. It is object-0.39.1 read/pe/data_directory.rs:30, reached eagerly from read/pe/file.rs:652 inside Pe::parse, so EVERY PE surface dies at File::parse and no 'just don't ask for the data directories' path exists. object bounds optional_data by size_of_optional_header, so read_slice_at(0, number) fails for any number > (size_of_optional_header-96)/8. THE FIX HAS A TEMPLATE ALREADY IN THE TREE, and a builder who misses it will rewrite it: kuna-analysis/src/loader/elf_shdr.rs is the exact same shape one format over -- 'object validates a header field eagerly, so repair the field in a COPY of the bytes and hand that copy downstream'. Its read_image()/tolerate_unusable_section_table() are already wired into strings.rs, xrefs.rs, decompile_all.rs:319+1071, decompile_graph.rs and console/engine.rs:2055, so a PE arm added there rides every call site for free. THE GUARD: clamp to min(number, (size_of_optional_header - 96 for PE32 / 112 for PE32+)/8), not to a hard 16 -- that is what Windows does, and here it recovers the 16 genuine directory slots that are physically present (optsize 224 = 96 + 16*8), so imports are read from the real table and not fabricated. Do not clamp when the count already fits.
- round 4 BUILDER: acceptance re-pointed from the dataset image onto a vendored 1024-byte PE32 twin (`decompiler/crates/kuna-analysis/tests/fixtures/pe_datadircount_i386.exe`, same trashed NumberOfRvaAndSizes 1531532893 in the same 224-byte optional header) so the probe could promote into `tests/cli/` -- CI has no dataset. That re-hashes the block, so the frontmatter `acceptance_id` is relabelled a-e4f84764b32f -> a-554672734890. Both the twin and the dataset original pass: `kuna functions bustme1.exe --json` goes from exit 1 to count 54, and `kuna decompile --addr 0x40908e` emits the entry stub.
