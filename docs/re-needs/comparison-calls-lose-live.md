---
need_id: comparison-calls-lose-live
title: Duplicate-name memcmp veneer loses two live arguments
track: quality
status: closed
severity: major
probe_id: p-63355bb7f652
acceptance_id: a-bd4d2e88aeb8
hypothesis_status: overturned
credibility: 1.0
instances: 1
challenges: [6869d705aadb6eeafb399027]
rounds: [10]
first_seen_round: 10
attempts: 0
covered_by_option: libcsigs
touches: [decompiler/crates/kuna-analysis]
scope: small
regression_of: null
pr: 575
closed_in_round: 12
closing_pr: 575
reject_reason: null
---

## Symptom

The imported `memcmp` call keeps only its first buffer even though RDX and R8
remain live at the call.

> **Comparison calls lose live arguments with both recovery options enabled**
> (major, `6869d705aadb6eeafb399027`)
> `memcmp` has one argument. Enabling `calleearity` and `varargstackargs`
> produces byte-identical output. Disassembly confirms RDX and R8 supply the
> second buffer and byte count.

The filing originally included `CRYPTO_memcmp`. That was split out after
refutation established a different cause; see **Split disposition** below.

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
    "0x140001000",
    "--addr",
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
      "\\bmemcmp\\([^,\\n()]+\\)"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/libcsigs_pe_x86_64.exe",
    "binary_sha256": "9334121f1d2710f13b313d0ebcdce71c568b4bf501862b0cb7e87e18e5c63117",
    "binary_size": 1536,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/libcsigs_pe_x86_64.exe",
    "selector": "0x140001000",
    "selector_kind": "addr"
  },
  "notes": "Vendored reduction of the dataset witness: the resolver names memcmp at the imported IAT slot 0x140002050 and FF 25 veneer 0x140001080, plus a provenance-distinct defined export at 0x140001060; the caller reaches the veneer with both buffers and length 3 live."
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
    "0x140001000",
    "--addr",
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
      "\\bmemcmp\\([^,\\n]+,[^,\\n]+,[^,\\n]+\\)"
    ]
  },
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/libcsigs_pe_x86_64.exe",
    "binary_sha256": "9334121f1d2710f13b313d0ebcdce71c568b4bf501862b0cb7e87e18e5c63117",
    "binary_size": 1536,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/libcsigs_pe_x86_64.exe",
    "selector": "0x140001000",
    "selector_kind": "addr"
  },
  "notes": "Positive three-argument assertion: deleting or renaming the call cannot satisfy this acceptance."
}
```

## Hypothesis

**Overturned.** Argument scoring is not responsible. The `memcmp` signature is
known, but it was parked by name while the call-site read path asks by entry
address.

## Refutation

The dataset PE registers `memcmp` twice: an `FF 25` veneer at `0x140012acc`
and its IAT slot at `0x140014308`. A by-name prototype binds one symbol, while
the direct call resolves to the other. The same resolver model is reproduced by
the in-repo fixture at `0x140001080` and `0x140002050`; its defined export at
`0x140001060` proves both halves of the safety rule: resolver provenance keeps
same-named local code out of the address-keyed stream, and an import/export name
collision suppresses the ambiguous global by-name prototype entirely.

## Split disposition

`CRYPTO_memcmp` remains an explicit-knowledge case and is not claimed by this
fix. It is a third-party OpenSSL import absent from both built-in libc tables;
the measured dataset experiment showed that
`--assert 'prototype CRYPTO_memcmp int CRYPTO_memcmp(void *a,void *b,unsigned long n)'`
restores its third argument. Adding that one name ad hoc would contradict the
table's measured libc/POSIX scope. Automatic third-party coverage belongs in a
separate proposal for a header/archive-derived prototype database; until then,
the honest route is an explicit prototype assertion.

## Reference

- `ida-decompile load target/crackmes.rar.__x/crackmes/crackmes.exe` — server
  exited before registration. No reference conclusion was available.

## Instances

- `6869d705aadb6eeafb399027` (round 10, tester t-r10-6869d705)

## Decision log

- filed by cluster.py from 1 observation(s)
- closed: acceptance a-bd4d2e88aeb8 now PASSES at eea2938094bd in PR 575
- round 10 REFUTER: hypothesis overturned. `CRYPTO_memcmp` is a plain
  missing-prototype case repaired by an explicit assertion. `memcmp` is the
  duplicate-symbol case: veneer `0x140012acc`, IAT `0x140014308`, with the
  prototype parked by name and read by entry address.
- round 10 CAPTAIN: cleared the unproven `regression_of: call-args-discarded`
  link and bound both original arms to the dataset binary.
- implementation split: this need now owns only the separable resolver-address
  defect. Its vendored acceptance positively requires
  a three-argument `memcmp(...)`; `CRYPTO_memcmp` is explicitly deferred to supplied
  knowledge or a general third-party prototype-database proposal.
