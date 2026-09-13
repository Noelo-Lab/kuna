---
need_id: objective-c-method-addresses
title: Objective-C method addresses lose their low bit on x86-64
track: quality
status: closed
severity: major
probe_id: p-e83475af90ef
acceptance_id: a-76373b946940
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5633c5d40ad448c29b]
rounds: [7]
first_seen_round: 7
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/analyzers/objc/methods.rs]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/571
closed_in_round: 12
closing_pr: "571"
reject_reason: null
---

## Symptom

Resolve checkCode: to its actual implementation and preserve adjacent method bodies.

> **Objective-C method addresses lose their low bit on x86-64** (major, `5ab77f5633c5d40ad448c29b`)
> Reports checkCode: at 0x100001d2a with size 1; named decompilation yields only return. The method record at 0x1000046c0 contains IMP 0x100001d2b, where disassembly shows PUSH RBP. False entries also truncate preceding methods. With objc off the genuine entry has size 290.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe",
    "binary_sha256": "5eacec69bcf1be53f115fbc0ed93e032a888000b5aa9ac77e4da44325683ef41",
    "binary_size": 54224,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--json",
    "--filter",
    "checkCode"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].address",
        "op": "eq",
        "value": 4294974762
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
  "target": {
    "binary_rel": "bin/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe",
    "binary_sha256": "5eacec69bcf1be53f115fbc0ed93e032a888000b5aa9ac77e4da44325683ef41",
    "binary_size": 54224,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "-[CrackMeManager checkCode:]",
    "--option",
    "objc",
    "on",
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
        "path": "functions[0].address",
        "op": "eq",
        "value": 4294974763
      },
      {
        "path": "functions[0].size",
        "op": "eq",
        "value": 290
      },
      {
        "path": "functions[0].error",
        "op": "eq",
        "value": null
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "NSRunAlertPanel"
      }
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Objective-C metadata handling appears to clear the ARM Thumb tag bit even for x86-64 IMP addresses.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe --backend ida --project-dir notes/ida --json` — Load exited 1 before server registration after 1.3671 seconds. No reference decompilation was obtained; IDA correctness remains unknown.

## Instances

- `5ab77f5633c5d40ad448c29b` (round 7, tester t-r7-5ab77f56)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 7 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, r7 T_REFUTE, sha dfc7acc5). UPHELD, and it is pinned to two lines. Reproduced verbatim: 'kuna functions <CrackMe> --json --filter checkCode' returns 0x100001d2a '-[CrackMeManager checkCode:]' size=1, one byte BELOW the IMP the tester read out of the method record (0x100001d2b, where the disassembly shows PUSH RBP). THE MASK IS UNCONDITIONAL. kuna-analysis/src/analyzers/objc/methods.rs:152-153 is 'fn strip_thumb(imp: u64) -> u64 { imp & !1u64 }', called with no arch guard from :106 (the relative-method-list arm) and :137 (the absolute arm). Its own doc comment states the assumption the tester disproved: 'On AArch64 / x86 this is a no-op (those IMPs are even-aligned)... The x86-64 MVP never sets it'. This image's x86-64 IMP is odd-aligned, so the normalization silently walks the entry back one byte, which is where both filed symptoms come from -- a 1-byte false function at ...2a, and the truncation of whatever real method preceded it. Fix shape is small and local: gate strip_thumb on 32-bit ARM (the loaded architecture), leaving AArch64/x86 untouched; Ghidra's ObjcMethod applies it for the same ARM reason, not universally. Scope small is right. WATCH THE ARM FIXTURE: kuna-analysis/tests/fixtures/macho_objc_arm64 and kuna-console/tests/verify_objc.rs exercise this path, and arm64 IMPs are even, so neither will catch a gate that is wrong in the other direction -- if the gate keys off 'is Mach-O ARM' rather than 'is 32-bit ARM' the tests stay green while 32-bit ARM regresses. ACCEPTANCE IS TIGHT ON THE ADDRESS AND BLIND TO THE REST. It asserts functions[0].address == 4294974763 (0x100001d2b) and nothing else, so a fix that lands the entry correctly but still sizes it 1 passes. B_DONE should also require size == 290 (the tester measured that with objc off, so it is the known-good value on this binary) and that 'kuna decompile <bin> "-[CrackMeManager checkCode:]"' emits a body rather than a bare return -- recovering the address without the body leaves the tester exactly as blocked as before.
- round 12 reconciliation: this stale filing is the exact challenge duplicate of `x86-64-objective-c`, fixed by merged PR #571 (`e824dc1a`). On current main `fe3d668e`, the #571 analyzer blobs remain byte-identical (`objc/methods.rs` `d1e1475095cfa0ac05f066f5dc649e3d01d14639`; `objc/mod.rs` `cfff14b8586b43cd05f13598ebcf3c44ce6a020a`). The acceptance was strengthened from address-only `a-75f212a1cedb` to `a-76373b946940`: it now enables `objc`, selects the recovered name, pins exactly one function at `0x100001d2b`, requires the known-good 290-byte extent and `error: null`, and requires the real `NSRunAlertPanel` body. The existing #571 regression matrix keeps the architecture boundaries explicit: the vendored absolute-form x86-64 odd IMP survives; the ordinary x86-64 fixture exercises the small/relative form; only 32-bit ARM clears the Thumb bit while x86-64/arm64 do not; the arm64 chained-fixup slice still resolves; and the two-state console gate proves `objc` changes the name while preserving the body. This record remains open until the captain authorizes closure against exact runtime evidence.
- round 12 Sol-high owner verification on exact base `fe3d668e`: strengthened acceptance `a-76373b946940` passed 3/3 on the original fat Mach-O dataset witness. Every run returned exactly one `-[CrackMeManager checkCode:]` at `0x100001d2b`, size 290, `error: null`, a 24-line body containing `NSRunAlertPanel`, and 14 provenance mappings. The `objc off` address-selected control also passed 3/3: the name alone fell back to `sub_100001d2b`, while the address, size, null error, body line count, alert call, and all 14 line mappings were preserved. The strengthened vendored CLI regression passed 3/3 at the absolute-form odd x86-64 IMP (size 15, null error, `return arg1 * 3 + 7;`); 15 focused Objective-C analysis tests covered the absolute/relative, x86-64/ARM32/arm64, classic/chained-fixup boundaries; and all 3 console Objective-C tests passed with explicit on/off body assertions on the relative-form x86-64 and arm64 fixtures. No engine logic, option, or DIV was added; the stale record remains open for captain-authorized closure.
- closed in round 12 by captain authorization after reconciliation against current main `b1f0b4c1`. This duplicate credits the actual implementation, PR #571, rather than the administrative reconciliation PR; canonical stale-reconciliation metadata therefore keeps `rounds: [7]`, `attempts: 0`, and `covered_by_option: null`, with `pr: https://github.com/Noelo-Lab/kuna/pull/571`, `closing_pr: "571"`, and `closed_in_round: 12`.
