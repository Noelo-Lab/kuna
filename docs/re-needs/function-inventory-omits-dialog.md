---
need_id: function-inventory-omits-dialog
title: Function inventory omits the dialog callback that xrefs recognizes
track: quality
status: open
severity: major
probe_id: p-5e71731ba9ec
acceptance_id: a-5a48b22e4076
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5d33c5d40ad448c6f6]
rounds: [10]
first_seen_round: 10
attempts: 0
covered_by_option: fast_funcdisc
touches: [decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Include the callbacks at 0x401410 and 0x401000 in the application inventory and whole-binary decompilation.

> **Function inventory omits the dialog callback that xrefs recognizes** (major, `5ab77f5d33c5d40ad448c6f6`)
> functions returns 150 entries without 0x401410; filtering 401410 returns count 0. Yet xrefs identifies its to_function and the PUSH at 0x4013ed. Explicit address decompilation recovers the dialog handler successfully. The define-function interface works, so this is an automatic inventory inconsistency.

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
    "stdout_absent": [
      "\"address_hex\":\\s*\"0x401410\""
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/stdcallpop_pe_i386.exe",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/stdcallpop_pe_i386.exe",
    "binary_sha256": "78d28ec6d13b9c1634101e86b2042ed61560db31ba63705adfb2b75f5757b7b4",
    "binary_size": 40960,
    "binary_source": "in-repo"
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
    "stdout_matches": [
      "\"address_hex\":\\s*\"0x401000\"[\\s\\S]{0,240}\"size\":\\s*[1-9][0-9]*",
      "\"address_hex\":\\s*\"0x4013e0\"[\\s\\S]{0,240}\"size\":\\s*4[1-8]",
      "\"address_hex\":\\s*\"0x401410\"[\\s\\S]{0,240}\"size\":\\s*[1-9][0-9]*"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/stdcallpop_pe_i386.exe",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/stdcallpop_pe_i386.exe",
    "binary_sha256": "78d28ec6d13b9c1634101e86b2042ed61560db31ba63705adfb2b75f5757b7b4",
    "binary_size": 40960,
    "binary_source": "in-repo"
  }
}
```

## Hypothesis

Executable code-pointer operands that are passed as stack arguments should be promoted only after target-validity checks, then re-seeded so newly reached callback bodies can expose nested callbacks.

## Refutation

The earlier extent diagnosis was overturned. The Listing's flow walk leaves 0x401410 undefined after the parent returns; the later inventory extent is only a downstream size estimate. The xref walk already proves the address operand at 0x4013ed, but the discovery walk did not consume executable data references.

## Reference

Ghidra 11.4 also omits both callbacks on this stripped witness. Recovering the stack-passed callback chain is therefore a concrete Kuna completeness advantage rather than a parity repair.

## Instances

- `5ab77f5d33c5d40ad448c6f6` (round 10, tester t-r10-5ab77f5d)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 10 REFUTER (2026-09-08): upheld the inventory inconsistency and initially attributed it to the parent function's reported 144-byte extent swallowing 0x401410. The refuter also identified 0x401000 as a second, cascading omission and warned that accepting 0x401410 alone would stop one link short.
- round 10 T_TRIAGE: acceptance widened to require both cascading callback entries.
- round 10 B_DRAIN: both probe arms were bound to the dataset witness; CI promotion remained impossible while the target was dataset-only.
- implementation triage: instruction-level Listing inspection overturned the proposed extent mechanism. The Listing leaves 0x401410 undefined after the parent's return; the 144-byte extent is assigned later and is not what suppresses discovery. The existing xref walk observes the pointer, but the discovery walk had no guarded consumer for executable scalar arguments. The vendored `stdcallpop_pe_i386.exe` is byte-identical to the witness (same SHA-256 and size), so both arms now use the in-repo fixture. Acceptance also pins the parent extent to 41..48 bytes and requires positive callback extents; CLI coverage requires unfiltered `decompile-all --json` to emit both callback bodies and streamed project selection to resolve both callbacks from the shared inventory.
- adversarial implementation review: narrowed evidence to the value stored by a real x86 `PUSH` (including COPY/sign-extension provenance), explicitly excluding absolute LOAD/STORE addresses; bounded and deduplicated the side model to 4,096 targets with a deterministic address-space sample; capped cascading discovery at four generations and 1,024 roots; and made strict validation reject an instruction whose bytes cross into already-known code. Live SLEIGH tests pin `push imm` positive and `push [absolute]` negative behavior.
