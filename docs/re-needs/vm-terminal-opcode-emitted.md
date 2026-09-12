---
need_id: vm-terminal-opcode-emitted
title: VM terminal opcode is emitted as an empty case label
track: quality
status: closed
severity: major
probe_id: p-b0dfb36b575d
acceptance_id: a-c051e63bb8de
hypothesis_status: overturned
credibility: 0.85
instances: 1
challenges: [6a17264017539b5175d123a9]
rounds: [6, 12]
first_seen_round: 6
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 598
closed_in_round: 12
closing_pr: "598"
reject_reason: null
---

## Symptom

Recover all VM opcode labels, especially the success-return opcode.

> **VM terminal opcode is emitted as an empty case label** (major, `6a17264017539b5175d123a9`)
> Emits case : with exit 0 and no warning. The empty label remains with switchselector on, argument recovery enabled, and loweredswitch off. Reading the two-level table at 0x140003d64 proves opcode 0xff maps to 0x140003c40, the terminal Boolean-return handler.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/crackme_very_hard.exe",
    "binary_sha256": "9d250e1c9e0fab29b860494cefc0093cff989d01c801f79bf571512692c1ae42",
    "binary_size": 31744,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x1400033c0"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\bcase\\s*:"
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
    "binary_rel": "bin/crackme_very_hard.exe",
    "binary_sha256": "9d250e1c9e0fab29b860494cefc0093cff989d01c801f79bf571512692c1ae42",
    "binary_size": 31744,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x1400033c0"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "case\\s+(?:0xff|255)\\s*:"
    ],
    "stdout_absent": [
      "\\bcase\\s*:"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Possibly loses the highest selector value when translating the byte-indexed, two-level jump table.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load ./target/crackme_very_hard.exe` — Server exited with status 1 before registering. No reference pseudocode was obtained; this does not establish whether IDA handles the function.

## Instances

- `6a17264017539b5175d123a9` (round 6, tester t-r6-6a172640)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 6 REFUTER: hypothesis **overturned** (was inconclusive). OVERTURNED on the CAUSE; the SYMPTOM stands and the tester's table reading is exactly right. Measured on .kuna-repipe/arena/6/6a17264017539b5175d123a9/target/crackme_very_hard.exe with the release kuna at 9c4c4508. (1) I DECODED THE TABLE BY HAND AND THE TESTER IS CORRECT. Dispatch at 0x1400035de: 'LEA EAX,[RDX-0x11]; CMP EAX,0xee; JA 0x140003c70; CDQE; MOVZX EAX,byte ptr [R12+RAX+0x3d94]; MOV ECX,dword ptr [R12+RAX*4+0x3d64]; ADD RCX,R12; JMP RCX' (R12 = imagebase). Byte index table 0x140003d94 (0xef entries), dword RVA table 0x140003d64. 12 distinct targets: 10 one-opcode handlers (0x11 0x12 0x21 0x22 0x23 0x24 0x31 0x32 0x33 0x41), the catch-all 0x140003c70 taking 228 opcodes, and **opcode 0xff (index 0xee) -> 0x140003c40, a target distinct from the default**. kuna emits those same 10 labels plus 'default:' plus one 'case :' with an EMPTY label list -- so exactly one label is missing and it is 0xff, the highest selector. (2) WHY THE FILED CAUSE IS WRONG. 'Loses the highest selector when translating the two-level table' predicts the destination was never enumerated -- but it WAS: the 'case :' block exists in the switch as a node separate from 'default:', so the guard bound (0xee INCLUSIVE) and the table walk both reached index 0xee. There is no off-by-one to find in the jumptable recovery. The loss is at label ATTACHMENT/emission, downstream of the table. A builder sent into the jump-table code on this diagnosis will find the bound already correct and will burn the dispatch. (3) THE BIGGER FINDING -- THE EMPTY CASE'S BODY IS NOT THE 0xff HANDLER, IT IS THE EPILOGUE. 0x140003c40 computes the function's boolean result: 'MOV ECX,EDI; SUB RAX,RDX (program length); CMP RCX,RAX; JC 0x140003c61; MOV AL,0xff; TEST AL,AL; SETNZ AL; JMP 0x140003c72' / else 'MOVZX EAX,[RSP+0x40]; XOR AL,[RCX+RDX]; SETNZ AL; JMP 0x140003c72'. kuna's 'case :' body is only 'sub_140004930(v35 ^ v21); return;' -- BYTE-IDENTICAL to the 'default:' body (default is 0x140003c70 = 'XOR AL,AL' then the same epilogue at 0x140003c72). The whole handler is dead-coded. (4) TWO CONTROLS, NEITHER RESURRECTS IT. kuna declares 'void sub_1400033c0(...)' although every caller does 'MOVZX R14D,AL' / 'TEST AL,AL'. Forcing the real return with --assert-strict 'prototype sub_1400033c0 unsigned char sub_1400033c0(unsigned long long *a0, unsigned int a1, char a2)' (rc=0, signature really changes) turns both bodies into 'return sub_140004930(v20);' -- i.e. it returns the void __security_check_cookie call -- but the length compare and the XOR/SETNZ are STILL absent. Adding '--option msvcstackguard on' on top changes nothing at all. So the void prototype is not the whole story either; do not assume fixing the return type fixes the body. (5) NOTE THE OVERLAP BUT DO NOT MERGE THE NEEDS: 'return sub_140004930(v20);' is the same cookie-call-eats-the-return-value shape as msvc-cookie-removal-changes / main-returns-invented-cookie, on a third binary. *** WARNING FOR T_TRIAGE -- THE ACCEPTANCE PROBE (a-4d58a42b70bc) IS CLOSED BY THE WORST FIX AVAILABLE. *** It is absence-only on '\\bcase\\s*:'. The cheapest way to make that regex stop matching is to DELETE the label-less block (fold it into default) -- which destroys the 0xff destination entirely and is strictly worse than today's output. The probe must require the label positively: stdout_matches 'case 0xff:' (or 'case 0x?ff\\s*:'). Third probe this round that is wrong in the closing direction; see the guard-dispatch and code-byte-loads notes.
- round 6 T_TRIAGE: ACCEPTANCE REWRITTEN (a-4d58a42b70bc -> a-c051e63bb8de), and it was verified two-sided before the swap: it FAILS on the need's own arena binary at 9c4c4508 (`probe check --bin` rc=1) and PASSES when simulated against plausible correct renderings. It was absence-only on `\bcase\s*:`, whose cheapest satisfier is deleting the label-less block into `default:` -- strictly worse than today. Now also REQUIRES `case 0xff:` (kuna renders case labels in hex, verified: `case 0x11:` .. `case 0x41:`).
- implementation: `PrintC::emit_switch_case` only pushed the numeric token when the case block supplied an op to anchor markup. The no-op path now renders the same width-aware, signed integer token directly, without inventing an `opref`. The crackme witness emits `case 0xff:` and retains the separate arm body; Rust remains `255 =>`. Printer tests cover no-op unsigned and signed labels, the ordinary op-backed path, `default:`, packed markup, and absent fabricated provenance.
- closed: acceptance a-c051e63bb8de now PASSES at 65e2cf60fb32
