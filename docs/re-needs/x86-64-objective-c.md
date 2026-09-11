---
need_id: x86-64-objective-c
title: x86-64 Objective-C recovery rounds odd IMP addresses down
track: quality
status: closed
severity: major
probe_id: p-dfea0a57f6e4
acceptance_id: a-78b86f730467
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f5633c5d40ad448c29b]
rounds: [11]
first_seen_round: 11
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/analyzers/objc/methods.rs]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/571
closed_in_round: 12
closing_pr: 571
reject_reason: null
---

## Symptom

Decompile checkCode: at the exact IMP stored in Objective-C metadata.

> **x86-64 Objective-C recovery rounds odd IMP addresses down** (major, `5ab77f5633c5d40ad448c29b`)
> Resolved checkCode: to 0x100001d2a, size 1, emitting only return with error:null. kuna read at 0x1000046c0 shows its IMP is 0x100001d2b. Disassembly shows RET at the reported entry and PUSH RBP at the actual entry. Other odd-address methods suffer the same corruption, and false boundaries truncate preceding functions.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "-[CrackMeManager checkCode:]",
    "--json"
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
    "decompile",
    "{{BIN}}",
    "-[Greeter greet:]",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].address",
        "op": "eq",
        "value": 4294968897
      }
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/macho_objc_odd_imp",
    "binary_sha256": "254ed9afc05e4c4913b325058e436fc4ecbfa15ca950777a83ff064cc87753c2",
    "binary_size": 16728,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/macho_objc_odd_imp",
    "selector": "-[Greeter greet:]",
    "selector_kind": "name"
  },
  "notes": "Vendored x86-64 twin of the reported Objective-C metadata defect. Its absolute method_t stores the valid odd IMP 0x100000641; the preceding even byte is padding. The dataset witness remains unchanged in Reproduction."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Possibly ARM/Thumb low-bit normalization applied to x86-64 IMPs; implementation was not inspected.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/CrackMe.zip.__x/CrackMe.app/Contents/MacOS/CrackMe` — Reference unavailable: server exited with status 1 before registering. No IDA pseudocode was obtained.

## Instances

- `5ab77f5633c5d40ad448c29b` (round 11, tester t-r11-5ab77f56)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the mechanism is a single named line. Symptom reproduced at the default: decompile BIN '-[CrackMeManager checkCode:]' --json returns address 0x100001d2a, size 1, body "return;". Byte-level proof from the fat image's x86_64 slice (cpu 0x01000007 at file 0x1000): the method_t at 0x1000046c0 reads name=0x1000033d5 ("checkCode:"), types=0x1000035b4, imp=0x100001d2b -- the IMP is ODD in the metadata. Bytes at 0x100001d2a are c3 55 48 89 e5: a RET ending the previous function, then PUSH RBP / MOV RBP,RSP at 0x100001d2b. So kuna is reading the IMP correctly and then corrupting it.

  THE LINE: kuna-analysis/src/analyzers/objc/methods.rs:152 strip_thumb(imp) = imp & !1u64, applied UNCONDITIONALLY at both method_t decoders (line 106 large/absolute, line 137 small/relative) with no architecture test. Its own doc comment states the premise that this binary falsifies: "On AArch64 / x86 this is a no-op (those IMPs are even-aligned)". Here it is not.

  THE FIX PRODUCES CORRECT OUTPUT, MEASURED. decompile BIN 0x100001d2b (the unstripped IMP) yields sub_100001d2b, size 289, error null, with a full well-formed body -- not the 1-byte stub. The engine has no problem with an odd function entry, so un-stripping heals both halves the tester reported: the truncated boundary AND the name binding (objc/mod.rs:187 addr: m.imp and :194 FidMatch{addr: m.imp} are both fed from the same corrupted value, which is why the real function comes out as sub_100001d2b with the ObjC name stranded at 0x100001d2a).

  *** WRONG-FIX TRAP: NOTHING IN THE TREE GUARDS strip_thumb, SO DELETING IT GOES GREEN. *** The function exists for a real reason -- on 32-bit ARM the IMP's LSB flags Thumb and is not part of the address -- but both objc fixtures have EVEN IMPs (mod.rs:311 x86-64 pins 0x100000640, mod.rs:378 arm64 pins 0x1000005f0) and there is no ARM32 fixture anywhere. A builder who simply removes the call passes every existing test while silently breaking every 32-bit ARM Mach-O. The fix must be ARCHITECTURE-GATED (strip only on 32-bit ARM), and the PR must carry regression evidence in BOTH directions: an odd-IMP non-ARM case that must survive, and an ARM32/Thumb odd-IMP case that must still be stripped.

  Acceptance checked and sound: it is a JSON equality on functions[0].address == 4294974763 (0x100001d2b), which cannot be satisfied spuriously and is exactly what the correct fix produces. No hole, unlike live-simd-string-copies and explicit-stdcall-prototypes-still this round.
- round 11 TRIAGE (captain): touches kuna-decomp -> kuna-analysis/src/analyzers/objc/methods.rs. The refuter pinned the defect to one line there (strip_thumb applied unconditionally at both method_t decoders, no architecture test); kuna-decomp is not involved. The wrong-fix trap stands and belongs in the builder brief: nothing in the tree guards strip_thumb, so DELETING it goes fully green -- both objc fixtures have even IMPs and there is no ARM32 fixture -- so the fix must be architecture-gated and show both directions.
- implementation retargeted Acceptance to the in-repo `macho_objc_odd_imp` twin so it can be promoted into `tests/cli/`; Reproduction remains the original dataset binary and exact `0x100001d2a` bad-address clause.
- implementation measured both targets after the fix: the dataset witness resolves `-[CrackMeManager checkCode:]` at `0x100001d2b` with a 290-byte body, and the vendored acceptance resolves `-[Greeter greet:]` at `0x100000641` with a 15-byte body returning `arg1 * 3 + 7`. `scripts.repipe.verify --need x86-64-objective-c --json` reports PASS and the acceptance is promoted verbatim to `tests/cli/x86-64-objective-c.json`.
- closed: acceptance a-78b86f730467 now PASSES at 093debb64679
