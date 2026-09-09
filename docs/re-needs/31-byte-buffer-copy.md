---
need_id: 31-byte-buffer-copy
title: 31-byte buffer copy loses its middle fifteen bytes
track: quality
status: open
severity: major
probe_id: p-b341a757d974
acceptance_id: a-7790b11de793
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [6927c8d12d267f28f69b8131]
rounds: [11]
first_seen_round: 11
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Faithful C for the buffer copy before the second transform.

> **31-byte buffer copy loses its middle fifteen bytes** (major, `6927c8d12d267f28f69b8131`)
> Four machine-code stores cover offsets 0..15 and 15..30. After explicit char[31] type assertions, C copies bytes 0..7 then 23..30, omitting 8..22. Relevant width, alias and argument-recovery options did not restore them.

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
    "sub_15dc",
    "--assert",
    "type v6 char[31]",
    "--assert",
    "type v7 char[31]",
    "--option",
    "arraycoverwidth",
    "on",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "\\b([A-Za-z_]\\w*)\\[7\\]\\s*=\\s*([A-Za-z_]\\w*)\\[7\\];\\s*\\1\\[0x17\\]\\s*=\\s*\\2\\[0x17\\];"
    ]
  },
  "target": {
    "binary_rel": "bin/0xjam3z-evolvingsbox.zip.__x/0xJam3z-Medium",
    "binary_sha256": "c1e78f2a0ba7a15d9bf6a563f0874656437112b617b94d42cd858e3e477ef665",
    "binary_size": 14440,
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
    "sub_15dc",
    "--assert",
    "type v6 char[31]",
    "--assert",
    "type v7 char[31]",
    "--option",
    "arraycoverwidth",
    "on",
    "--option",
    "calleearity",
    "on",
    "--option",
    "varargstackargs",
    "on"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "\\b([A-Za-z_]\\w*)\\[7\\]\\s*=\\s*([A-Za-z_]\\w*)\\[7\\];\\s*\\1\\[0x17\\]\\s*=\\s*\\2\\[0x17\\];"
    ]
  },
  "target": {
    "binary_rel": "bin/0xjam3z-evolvingsbox.zip.__x/0xJam3z-Medium",
    "binary_sha256": "c1e78f2a0ba7a15d9bf6a563f0874656437112b617b94d42cd858e3e477ef665",
    "binary_size": 14440,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Possibly lost pieces during stack-copy merging; not established.

## Refutation

Builder b-r12-31-byte-buffer-c, round 12. The symptom stands; both the filed
hypothesis and the round-11 triage note are overturned.

- The loss is NOT assertion-driven. `kuna decompile <bin> sub_15dc` with no
  `--assert` and no options emits `v7 = v6;` and `v10 = v9;` and nothing else,
  so two of the four stores are already gone at the default. The `char[31]`
  assertions only change how the two SURVIVING stores are spelled.
- It is not stack-copy merging either. The five stack-to-stack COPYs are still
  correct and complete after the third heritage pass; the `ActionDeadCode` that
  follows deletes three of them.
- The discriminator is `Varnode::isStackStore()`, which is false on every piece
  `Heritage::refineWrite` cuts a store into and true on the two stores
  refinement never touched. `ActionDirectWrite` reads exactly that flag, and
  `ActionDeadCode` clears `addrforce` on whatever it did not mark.
- Closed by `option splitstorekeep` (default on, DIV-153), which carries the
  mark onto the refinement pieces.

## Reference

- `objdump -d --start-address=0x17da --stop-address=0x1812 target/0xjam3z-evolvingsbox.zip.__x/0xJam3z-Medium` — Four qword stores to rbp-0x1b0, rbp-0x1a8, rbp-0x1a1 and rbp-0x199 cover all 31 destination bytes.

## Instances

- `6927c8d12d267f28f69b8131` (round 11, tester t-r11-6927c8d1)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 TRIAGE (captain): filed by splitting instance 2 out of live-simd-string-copies, which had merged it on a shared clause shape. Bound its probe target to the dataset binary like the round's other 14. NOT the same gap as its former parent: measured on main, all four qword stores are emitted correctly at the default (offsets 0..7, 8..15, 15..22, 23..30) from four plain `48 89` movs at 0x17da..0x1812, with no SIMD in the function. The sparse element-wise copy appears ONLY under `--assert type v6 char[31] --assert type v7 char[31]`, so this is a type-assertion / array-cover rendering defect. A builder should develop against that assertion pair, not against the store path. Hypothesis not yet refuted -- it was filed after T_REFUTE closed.
