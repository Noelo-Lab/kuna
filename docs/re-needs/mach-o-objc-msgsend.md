---
need_id: mach-o-objc-msgsend
title: Mach-O objc_msgSend import remains an anonymous pointer call
track: quality
status: closed
severity: minor
probe_id: p-909191fe1944
acceptance_id: a-38de212485a2
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [5ab77f5633c5d40ad448c29b]
rounds: [11, 12]
first_seen_round: 11
attempts: 1
covered_by_option: peimportcall
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 606
closed_in_round: 12
closing_pr: "606"
reject_reason: null
---

## Symptom

Carry the loader-resolved import name into calls through its slot.

> **Mach-O objc_msgSend import remains an anonymous pointer call** (minor, `5ab77f5633c5d40ad448c29b`)
> functions identifies objc_msgSend at 0x100004038, but checker decompilation emits (*dat_100004038)(...). Enabling calleearity and varargstackargs preserves the symptom. The catalog documents peimportcall as PE-only. A data assertion successfully supplies a name, so the override interface exists.

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
    "0x100001d2b",
    "--option",
    "funcboundflow",
    "off",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "\\(\\*dat_100004038\\)\\("
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
    "0x100001d2b",
    "--option",
    "funcboundflow",
    "off",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "json": [
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "dat_100004038"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "objc_msgSend"
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

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Mach-O import slots may lack the external-reference linkage supplied for PE.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f5633c5d40ad448c29b` (round 11, tester t-r11-5ab77f56)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED. The symptom stands, the diagnosis does not. Filed hypothesis was "Mach-O import slots may lack the external-reference linkage supplied for PE". Measured: the linkage EXISTS. kuna functions BIN --json already names the slot -- 0x100004038 objc_msgSend (size 0) -- and names all seven __la_symbol_ptr slots too (0x100004040 NSApplicationMain, 0x100004048 NSRunAlertPanel, 0x100004050 exit, 0x100004058 memcpy, 0x100004060 objc_getProperty, 0x100004068 objc_msgSendSuper2, 0x100004070 objc_setProperty) plus 0x100004028 dyld_stub_binder. So a builder who goes looking for missing loader-tier import linkage will find it already there and close nothing.

WHAT IS ACTUALLY HAPPENING. Section walk of the x86_64 slice: 0x100004038 is the ENTIRE __got section (size 0x8, type 0x06 = S_NON_LAZY_SYMBOL_POINTERS) -- one non-lazy import pointer. Disassembly at the call site: 0x100001d4a ff15e8220000 CALL qword ptr [0x100004038]. That is an ff15 memory-indirect call through the import slot, and it renders as (*dat_100004038)(...) -- the name that the loader attached to that address is not consulted when the CALL target is a data-slot read.

THE DISCRIMINATOR, AND IT IS CLEAN: STUB-MEDIATED IMPORTS ALREADY WORK. In the same function, NSRunAlertPanel(0x100004f98,0x100004fb8,0,0,0) renders with its name, because that call goes through a __stubs entry (0x100002e9c..0x100002ec6), and those are real 6-byte FUNCTIONS carrying names. Only the GOT-direct import is anonymous. So the gap is not Mach-O import naming in general -- it is specifically a call whose target is a non-lazy import POINTER rather than a stub.

WHERE THE WORK IS. This is the identical rendering the PE-only option already owns: docs/options.md:215 lists the symptom "windows api calls render as (*dat_411324)() with no name" under peimportcall. The direction is to extend that existing mechanism (or add its Mach-O sibling) to non-lazy import pointers, keyed on a function symbol existing at the slot address -- NOT to add loader linkage that is already present. Implementation lives in kuna-decomp/src/p6_variables/kuna_calleepop.rs, infra/architecture.rs, substrate/context.rs.

SAFETY NOTE FOR THAT KEYING, ALREADY CHECKED. Not every ff15 through data is an import. The same function has 0x100001d5a CALL qword ptr [0x100004d60], and 0x100004d60 is inside __objc_msgrefs (0x100004d50 size 0x40) -- an objc_msgSend_fixup message-ref struct, not an import slot. kuna functions lists NO symbol there, so a fix keyed on "a named function symbol exists at the slot address" naturally skips it. Keying on "the call target is a data address" instead would name it wrongly.

ACCEPTANCE CHECKED AND SOUND -- unlike two other needs this round. Ran the exact acceptance command on today's tree: both clauses FAIL now (code contains dat_100004038; objc_msgSend absent), so both must genuinely flip. The contains "objc_msgSend" clause cannot be satisfied spuriously by the objc_msgSendSuper2 import -- verified that string is absent from this function's body. No hole.
