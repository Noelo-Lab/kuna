---
need_id: overlapping-anti-disassembly-sequence
title: Overlapping anti-disassembly sequence produces bogus out-of-image code and corrupted checker dataflow
track: quality
status: closed
severity: blocker
probe_id: p-410dd063e811
acceptance_id: a-52c2ad89b522
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [5ab77f6333c5d40ad448ca52]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: large
regression_of: null
pr: null
closed_in_round: 2
closing_pr: "383"
reject_reason: null
---

## Symptom

A decompilation of the recursive password checker at 0x804881c that follows the real instruction stream beginning at 0x8048838 and preserves its two ordinary arguments.

> **Overlapping anti-disassembly sequence produces bogus out-of-image code and corrupted checker dataflow** (blocker, `5ab77f6333c5d40ad448ca52`)
> Kuna treated the intentionally overlapping bytes at 0x8048837 as a CALL, emitted a call to out-of-image sub_b10cdc7, writes through uninitialized pointers, a bogus dat_d98ddd00 global, malformed split arguments, and an unreadable comparison. The related enclosing routine also hit the already-filed no-cli-function-boundary-override problem: `kuna decompile ... 0x80489e6 --addr` exited 1 at internal target 0x8048c49.

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
    "0x804881c",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "sub_[0-9a-f]{7}\\(\\)",
      "dat_[0-9a-f]{8} \\+="
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
    "decompile",
    "{{BIN}}",
    "0x804881c",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "sub_[0-9a-f]{7}\\(\\)",
      "dat_[0-9a-f]{8} \\+="
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

- The recursive traversal commits to the CALL instruction at 0x8048837 and does not reconsider the conditional branch target at 0x8048838 as an overlapping instruction boundary.

## Refutation

**UPHELD (captain, round 2 T_REFUTE)** — confirmed by direct disassembly, with the real risk sitting one step past the diagnosis.

- `kuna disassemble --addr 0x804881c` reports `sub_804881c` spanning 0x804881c..0x804883d and decodes `0x8048835 JNZ 0x8048838` immediately followed by `0x8048837 CALL 0xb10cdc7` — a target **outside the image**. Decoding from 0x8048838 instead yields the real stream (`MOV EAX,[EBP + 0xc]; ADD EAX,[EBP + 0x8]; MOVSX EDX,[EAX]`). kuna does commit to the byte at 0x8048837 and never reconsiders the conditional-branch target that lies *inside* that instruction's encoding.
- **Detection is low-risk**: a conditional-branch target that falls strictly inside an already-decoded instruction cannot occur in well-formed code, and here there is a second conservative signal — the overlapping CALL's target is unmapped.
- **Where wrong output would come from is the policy, not the detection**: which stream owns the overlapped bytes, and what happens to flow and dataflow already built from the discarded one. A builder must answer that explicitly (and gate it), or the fix trades bogus out-of-image code for a silently truncated function.

## Reference

- `objdump -d -Mintel --start-address=0x804881c --stop-address=0x80488ae {{BIN}}` — The bytes at 0x8048838 decode coherently as `mov eax,[ebp+0xc]`; following that overlapping stream exposes the input-byte XOR and table comparison used to recover the password.

## Instances

- `5ab77f6333c5d40ad448ca52` (round 2, tester t-r2-5ab77f63)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_REFUTE (captain): hypothesis **UPHELD** by direct disassembly (JNZ 0x8048838 followed by a linear CALL at 0x8048837 whose target is outside the image). Detection is low-risk; the wrong-output risk is the ownership policy for the overlapped bytes and the already-built flow, which the builder must answer explicitly and gate.
- round 2 T_TRIAGE (captain): scope small -> LARGE. T_REFUTE upheld the symptom by direct disassembly (JNZ 0x8048838 at 0x8048835 followed by a linear CALL 0xb10cdc7 at 0x8048837 whose target is out of image) and identified the real risk as the OWNERSHIP POLICY for the overlapped bytes -- which decode wins, and what becomes of the instructions attributed to the loser. That is a code-vs-code partitioning decision in P1/P2 with corpus-wide blast radius, and the refutation says the builder must answer and gate it. A design-only [PROPOSAL] is the right gate for a blocker whose fix moves instruction boundaries; detection itself is cheap and low-risk (a branch cannot target mid-instruction in well-formed code), it is the resolution that needs a decision.
- closed: acceptance a-52c2ad89b522 now PASSES at b08c30ac6668
