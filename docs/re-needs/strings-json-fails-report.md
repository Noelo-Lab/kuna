---
need_id: strings-json-fails-report
title: strings JSON fails to report the owning function for a directly referenced prompt
track: quality
status: open
severity: major
probe_id: p-3820d5b66146
acceptance_id: a-f0c7bd5b4a7e
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f6333c5d40ad448ca52]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/listing/xrefs.rs]
scope: large
regression_of: no-strings-inventory
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

The owning function and reference count for the `Password: ` string, as promised by the new strings inventory.

> **strings JSON fails to report the owning function for a directly referenced prompt** (major, `5ab77f6333c5d40ad448ca52`)
> `kuna strings --json --filter Password` found the string at 0x8049127 but returned `xrefs_count: 0` and `functions: []`, despite code directly forming its address for output.

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
    "--json",
    "--filter",
    "Password"
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
        "path": "strings[0].xrefs_count",
        "op": "eq",
        "value": 0
      },
      {
        "path": "strings[0].functions",
        "op": "len_eq",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/pcme0.tar.gz.__x/pcme0.tar.__x/pcme0/pcme0",
    "binary_sha256": "1cfa9dc11e637657c2ae9b5d1f549ac59d0f8aef10053191bc083e4b6b16bd52",
    "binary_size": 6372,
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
    "--json",
    "--filter",
    "Password"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "strings[0].xrefs_count",
        "op": "gt",
        "value": 0
      },
      {
        "path": "strings[0].functions",
        "op": "len_gt",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/pcme0.tar.gz.__x/pcme0.tar.__x/pcme0/pcme0",
    "binary_sha256": "1cfa9dc11e637657c2ae9b5d1f549ac59d0f8aef10053191bc083e4b6b16bd52",
    "binary_size": 6372,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- PIC-relative address formation through the EBX thunk is not connected to string xrefs or function ownership.

## Refutation

**UPHELD in mechanism, but `scope: small` is wrong (captain, round 2 T_REFUTE).**

- The tester's EBX-thunk diagnosis is what the binary actually does. `kuna disassemble` at 0x8048823 shows `CALL 0x8048828; POP EBX; ADD EBX,0x1a18` and later `MOV EAX,dword ptr [EBX + 0xa8]` — GOT/base-relative address formation in a **non-PIE** ELF 32-bit executable, which is why nothing looks PIC at the ELF level.
- **No address-matching scheme can ever produce this xref**: the literal `0x8049127` does not occur anywhere in the image (all little-endian 4-byte occurrences searched: none). The address is computed at run time.
- The failure is not the `strings` surface and not this string. Code xrefs work on this binary (`xrefs --to 0x80485fc` returns 4 CALL sites with instruction text and owning functions), the strings→xrefs plumbing works generally (`/bin/ls`: 128 of 538 strings carry `xrefs_count > 0` and a `functions` entry), and on this binary **all 23** strings report `xrefs_count: 0`.
- **Wrong-output risk:** any heuristic that attributes a string to a nearby or enclosing function without folding the thunk base would produce WRONG ownership, and the loop has no gate that would see it. The honest fix is constant propagation of the thunk base (and base-relative loads) into the xref index — re-scope this need before dispatching it as a `small`.

## Reference

- `objdump -d -Mintel --start-address=0x80489e6 --stop-address=0x8048d12 {{BIN}}` — The routine forms addresses relative to its recovered EBX PIC base, including the prompt passed to write/printf.

## Instances

- `5ab77f6333c5d40ad448ca52` (round 2, tester t-r2-5ab77f63)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_DEDUP (captain): SPLIT from `xrefs-unify-pe-import`, which cluster.py merged with it because `_subcommand()` in cluster.py only knows the pre-round-1 subcommand set, so `strings`, `xrefs`, `disassemble` and `unpack` all hash to '?' and any two wrong-output complaints about the new CLI surface collide. Worth a builder-tier fix alongside the probe JSON-escape defect.
- round 2 T_REFUTE (captain): hypothesis **UPHELD** (EBX-thunk address formation confirmed in the disassembly), but `scope: small` is refuted -- the string's address never appears as a literal in the image, so no address-matching fix can work and any nearby-function heuristic would attribute ownership WRONGLY. Re-scope at T_TRIAGE.
- round 2 T_TRIAGE (captain): scope small -> LARGE and touches kuna-decomp -> kuna-analysis/src/listing/xrefs.rs. T_REFUTE proved no address-matching fix can work (the literal 0x8049127 is nowhere in the image; the address is computed at run time by the CALL/POP EBX/ADD EBX,0x1a18 thunk in a NON-PIE i386 ELF), so the honest fix is constant propagation of the thunk base into the xref index -- a real analysis-tier feature whose failure mode is attributing a string to the WRONG function, with no gate that would see it. `kuna strings` computes xrefs_count in kuna-cli/src/strings.rs:191 but the index it queries is built by kuna_analysis::listing::xrefs::build, which is where the propagation belongs. track stays `quality`: the fix ships behind an option, so it must hold the counter leases.
