---
need_id: entry-point-push-return
title: Entry-point push/return transfer disappears from C
track: quality
status: closed
severity: major
probe_id: p-b18b63fe0527
acceptance_id: a-d88748a4d135
hypothesis_status: inconclusive
credibility: 0.85
instances: 1
challenges: [5ab77f5433c5d40ad448c1c1]
rounds: [8, 12]
first_seen_round: 8
attempts: 1
covered_by_option: pushimmediateret
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: 603
closed_in_round: 12
closing_pr: "603"
reject_reason: null
---

## Symptom

See the transfer to 0x40365a following the unpacker call.

> **Entry-point push/return transfer disappears from C** (major, `5ab77f5433c5d40ad448c1c1`)
> Disassembly shows CALL 0x4f7990; PUSH 0x40365a; RET. Default C contains only the first call. An existing flow assertion restores the transfer.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x4f7001",
    "--addr",
    "--json"
  ],
  "expect": {
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 1
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "40365a"
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/push_immediate_ret_i386.exe",
    "binary_sha256": "12a564ed5035a19b27958597f26a51694deb9428d89ee61320c18500c4ec0d90",
    "binary_size": 1536,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/push_immediate_ret_i386.exe",
    "selector": "0x402000",
    "selector_kind": "addr"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x402000",
    "--addr",
    "--define-function",
    "0x402000-0x40200b=push_immediate_ret",
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
        "path": "functions[0].error",
        "op": "eq",
        "value": null
      },
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "(?s)sub_402010\\(\\);\\s*switch\\(0x401000\\) \\{\\s*case 0x401000:"
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "sub_401000"
      }
    ]
  },
  "notes": "Vendored CryptoME-shaped PE: require the unpacker call followed by the full recovered tail-transfer construct. Destination digits in a warning cannot satisfy this predicate; the encrypted target has no function symbol."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

- `ida-decompile decompile 0x4f7001 --binary ./target/CryptoME.zip.__x/CryptoME.exe --json` — No running server matched; subsequent backend startup failed. Kuna's own disassembly and successful flow override establish the omitted transfer.

## Instances

- `5ab77f5433c5d40ad448c1c1` (round 8, tester t-r8-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- implementation candidate: `pushimmediateret` uses bounded raw-p-code stack provenance and seeds BRANCH, not CALL, for the proven one-store form. The two-store form remains owned by `entryretdispatch`; ordinary, argument-push, adjusted, overwritten, computed, conditional, and opaque cases decline; explicit flow facts win; no target function is synthesized.
- acceptance_id changed to `a-d88748a4d135` and the acceptance was reduced to the vendored CryptoME-shaped PE fixture. It requires the unpacker call followed by the full `switch(0x401000) { case 0x401000:` control-flow construct inside `functions[0].code`, so destination digits appearing only in a warning cannot pass. The encrypted target section has no function symbol.
- closed: acceptance a-d88748a4d135 now PASSES at ae980f25ccff
