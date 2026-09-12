---
need_id: bytecode-reader-return-discarded
title: Bytecode-reader return is discarded despite immediate use of RAX and RDX
track: quality
status: closed
severity: major
probe_id: p-352a35192231
acceptance_id: a-675031196902
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [673da52e9b533b4c22bd2eeb]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: callretpair
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: https://github.com/Noelo-Lab/kuna/pull/565
closed_in_round: 12
closing_pr: "565"
reject_reason: null
---

## Symptom

Connect the returned payload and tag to the reader call.

> **Bytecode-reader return is discarded despite immediate use of RAX and RDX** (major, `673da52e9b533b4c22bd2eeb`)
> After explicit argument assertions, the call remains a standalone statement followed by a cast of an unassigned RDX local. Disassembly immediately compares EDX to 6 and saves RAX/RDX.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/crackme.x86_64.elf",
    "binary_sha256": "19351f6b9a6b8af7399470a3ff22ddf1b2b8f44a85f1e0497cbb6e4ed20fbb8a",
    "binary_size": 977264,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_9d9c0",
    "--assert",
    "param sub_875e0::0 RDI void *ctx",
    "--assert",
    "param sub_875e0::1 RSI char *buf",
    "--assert",
    "param sub_875e0::2 RDX unsigned long len",
    "--assert",
    "param sub_875e0::3 RCX unsigned int flags",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_matches": [
      "(?m)^\\s+sub_[0-9a-f]+\\([^\\n;]+\\);\\n\\s+v[0-9]+ = \\(int4\\)v[0-9]+;"
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
    "binary_rel": "bin/crackme.x86_64.elf",
    "binary_sha256": "19351f6b9a6b8af7399470a3ff22ddf1b2b8f44a85f1e0497cbb6e4ed20fbb8a",
    "binary_size": 977264,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "sub_9d9c0",
    "--assert",
    "param sub_875e0::0 RDI void *ctx",
    "--assert",
    "param sub_875e0::1 RSI char *buf",
    "--assert",
    "param sub_875e0::2 RDX unsigned long len",
    "--assert",
    "param sub_875e0::3 RCX unsigned int flags",
    "--assert-strict"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "(?m)^\\s+sub_[0-9a-f]+\\([^\\n;]+\\);\\n\\s+v[0-9]+ = \\(int4\\)v[0-9]+;"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x9d9c0 --binary ./target/crackme.x86_64.elf --json` — Unavailable: load exited 1 because the server exited before registering; decompile exited 1 with no matching running server. No output comparison was possible.

## Instances

- `673da52e9b533b4c22bd2eeb` (round 8, tester t-r8-673da52e)

## Decision log

- filed by cluster.py from 1 observation(s)
- captain T_DEDUP (r8): SPLIT from `bytecode-reader-call-site` even though both are the same caller (`sub_9d9c0`), the same callee (`sub_875e0`) and the same run — i.e. the r7 BCrypt double-finding shape, which is normally one root cause. The evidence says otherwise here: THIS probe already supplies the other need's fix by hand (`--assert param sub_875e0::0..3`) and the return is STILL discarded. A fix for the argument half therefore does not close this one, so merging them would have retired this probe for free.
- round 8 TRIAGE (captain): SIBLING NOTICE -- see bytecode-reader-call-site, same binary, same caller sub_9d9c0, same callee sub_875e0, opposite side of the same prototype. Note this need was filed with an EMPTY hypothesis and no refuter ran on it, so its `inconclusive` is the filed default and not a verdict. Its acceptance and the sibling's are compatible; one PR may close both.
- closed: acceptance a-675031196902 now PASSES at 5490ae7c30be
- round 12 reconciliation: PR #565 (squash `12a86b24f101cf5c3302b8ab66014d8efd9a8e0d`) implemented `callretpair` for this need and passed the acceptance gate. Its squash introduced this record with the pre-closure `open` front matter, while the later acceptance-state write remained outside the merged commit; that stale snapshot is why the shipped option and durable status disagreed.
- current controls at `5490ae7c30be05fdeb586aaddd96402e998b9506`: the exact dataset acceptance passes with default `callretpair on`, connecting `sub_875e0` to a 16-byte result and extracting both halves; adding `--option callretpair off` restores the filed bare call followed by a cast of an unassigned `// rdx` local. The promoted in-repo acceptance at `tests/cli/bytecode-reader-return-discarded.json` also passes against `callretpair_x86_64`.
- index reconciliation constraint: at `5490ae7c30be05fdeb586aaddd96402e998b9506`, the authoritative committed index contains 197 active and 1 rejected records, but the clean commit exposes only 139 active Markdown sources and no rejected source directory to `needs reindex`. A sparse-source regeneration would destructively discard 58 active rows and the rejected ledger row, so this closure preserves the authoritative index byte-for-byte except for its `closed`/`open` totals and this need's object. Regenerate only after the missing sources have landed.
