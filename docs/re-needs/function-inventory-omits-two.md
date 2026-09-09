---
need_id: function-inventory-omits-two
title: Function inventory omits two address-taken VM handlers
track: loader
status: open
severity: major
probe_id: p-7893d3fda03c
acceptance_id: a-e8bc8dd2c6a2
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [69b7492049fa49a2a260218f]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Include halt and mixing handlers in the function inventory.

> **Function inventory omits two address-taken VM handlers** (major, `69b7492049fa49a2a260218f`)
> The inventory omits 0x140003c60 and 0x1400041b0 despite their pointers occupying handler-table entries 27 and 28. Both decompile successfully by address. Explicit boundary definitions add both and raise the inventory total from 228 to 230.

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
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_absent": [
      "\"address_hex\":\\s*\"0x140003c60\"",
      "\"address_hex\":\\s*\"0x1400041b0\""
    ]
  },
  "target": {
    "binary_rel": "bin/vm.exe",
    "binary_sha256": "7a7009becef2678b45b7bc660ae134f7742e2632cb46e4e024281dcf0428b957",
    "binary_size": 44032,
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
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stdout_matches": [
      "\"address_hex\":\\s*\"0x140003c60\"",
      "\"address_hex\":\\s*\"0x1400041b0\""
    ]
  },
  "target": {
    "binary_rel": "bin/vm.exe",
    "binary_sha256": "7a7009becef2678b45b7bc660ae134f7742e2632cb46e4e024281dcf0428b957",
    "binary_size": 44032,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- PE discovery may miss leaf handlers without unwind entries.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69b7492049fa49a2a260218f` (round 12, tester t-r12-69b74920)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `missing-capability|functions|exit_code,stdout_is_json,stdout_matches` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED on the cause; the symptom is exact, including the entry indices. Confirmed absent on aa18e56d: kuna functions vm.exe --json lists 228 functions and neither 0x140003c60 nor 0x1400041b0 is among them, while kuna decompile vm.exe 0x140003c60 --addr emits a real one-line body (void sub_140003c60(long long a0){*(char*)(a0+0x48)=0;}), so both are code. THE FILED CAUSE IS NOT THE DISCRIMINATOR. I parsed the PE exception directory (dd[3] rva 0xc000, 1836 bytes = 153 RUNTIME_FUNCTION entries): neither missing handler has a .pdata entry, but NEITHER DOES sub_140003c50, WHICH IS IN THE INVENTORY. Lacking an unwind entry is therefore not sufficient to be missed, and a builder sent to the .pdata oracle finds nothing to fix there -- these bytes are not in .pdata at all. THE ACTUAL DISCRIMINATOR IS CALL vs DATA REFERENCE, and one xrefs pair settles it: 0x140003c50 has count 4 including kind=call (CALL at 0x140007fd7), while 0x140003c60 and 0x1400041b0 each have count 2 and BOTH are kind=data, the identical instruction LEA RCX,[<target>], both inside sub_140002210. So the inventory is (analyzer oracles) union (direct call targets found by recursive descent), and an address-taken-only function is seeded by nothing. That matches the pass's own header: kuna-analysis/src/analyzers/entry/mod.rs runs entry + .pdata + TLS for a PE (mod.rs:415) and documents the missing Listing/call sweep as a LOSS. There is no data-pointer-table oracle in any format. THE TESTER'S TABLE IS REAL AND I MEASURED IT. Scanning .rdata for a contiguous run of in-.text qwords gives a 30-entry table at VA 0x1400093e0..0x1400094c8 (file 0x83e0..0x84c8), and 0x140003c60 / 0x1400041b0 are entries 27 and 28 EXACTLY as filed. That table is the fix oracle -- not a heuristic sweep. FOUR ENTRIES ARE MISSING, NOT TWO: index 20 = 0x140003c20 and index 24 = 0x140003f40 are also absent from the inventory, and I checked both -- each has exactly two xrefs, both kind=data, both LEA RCX. So a table-seeding fix takes 228 to 232, not to 230; do not treat a count of 230 as the success criterion. 26 of the 30 entries are already known (via calls), which is why the table looks half-covered rather than wholly missing. ACCEPTANCE IS SOUND: two positive address_hex clauses that cannot be satisfied by losing anything, and both targets are genuine code. ONE HAZARD, THE SAME ONE MEASURED ON THE LOADER PAIR: a run-on function currently swallows each target -- sub_140003c50 has size 32 (0x140003c50..0x140003c70, contains 0x140003c60) and sub_140004050 has size 384 (0x140004050..0x1400041d0, contains 0x1400041b0). Seeding gives the bounder something to stop at, so those two sizes WILL move; check them rather than assuming the change is purely additive. NOT THE SAME NEED AS THE LOADER PAIR: arm-inventory-omits-41 and elf-discovery-omits-explicit need a BL/call-target sweep and a second _start idiom; this one needs a code-pointer-table oracle, which neither of those fixes would provide. Same pass, different oracle -- do not fold them.
- round 12 CAPTAIN (B_FANOUT, pre-dispatch brief): PROMOTION CAVEAT -- this need's probe target is `binary_source: dataset`, and `verify --promote` refuses anything that is not `in-repo` (verify.py:750, "CI has no dataset"); `--force` does not override it. B_DONE can therefore CLOSE this need on the measured acceptance flip but cannot promote its probe into `tests/cli/`, so the fix ships with no regression guard unless the builder vendors an in-repo fixture (or reuses a vendored twin) in the SAME PR and points a promotable copy of the probe at it. Same refusal already blocked virtualalloc-protection-argument-disappears and arm-inventory-invents-function.
- round 12 BUILDER (b-r12-function-invento): refuter's overturn UPHELD and its measurement reproduced — the discriminator is call-vs-data reference, and FOUR entries are missing (0x140003c20, 0x140003f40 as well), so the witness goes 228 -> 232. Closed with a new PE entry oracle over the **base-relocation directory** rather than the refuter's contiguous-run table scan: a relocated word is the image's own statement that the word holds an address, which needs no run-length/stride/alignment rule and yields exactly the four on this image (39 exec-landing targets, 35 already known). Two false-positive classes found by a 152-binary PE sweep, both settled by `.pdata`: a non-PIC jump table's labels (dropped when strictly inside a `RUNTIME_FUNCTION`), and a packed image's data merged into `.text` (jormungandr.exe's UTF-16 locale table, 666 pointers, all below the first record — so a candidate must also lie in the span its own section's `.pdata` records vouch for). That guard is also the precondition, so a PE32 with no exception directory abstains. No option: a relocation is not a judgement call. Sizes moved exactly as the refuter warned and it is a correction (sub_140003c50 32 -> 16); 31 entries disappear corpus-wide and every one is superseded by a new start 4-0x400 bytes earlier. Probe promoted as a CI twin on a hand-assembled `pe_reloctable_x86_64.exe` — `--promote` refuses the dataset target, and the need keeps the dataset witness.
