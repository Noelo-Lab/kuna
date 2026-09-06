---
need_id: name-based-xrefs-rejects
title: Name-based xrefs rejects memcmp although its candidates are aliases
track: tooling
status: open
severity: minor
probe_id: p-264f6f6dbdd7
acceptance_id: a-6576d0dff8a6
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [68d9ee36224c0ec5dcedc3fc]
rounds: [3]
first_seen_round: 3
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/xrefs.rs, decompiler/crates/kuna-analysis/src/listing/xrefs.rs]
scope: small
regression_of: null
pr: "441"
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Find references to memcmp by name.

> **Name-based xrefs rejects memcmp although its candidates are aliases** (minor, `68d9ee36224c0ec5dcedc3fc`)
> Reported ambiguity between 0x140007a1b and 0x140009280. Querying the latter address identifies the former as an alias and returns references to both.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "xrefs",
    "{{BIN}}",
    "--to",
    "memcmp",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "ambiguous",
      "memcmp"
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.exe",
    "binary_sha256": "30849bed966c92e64009a23df62210e615a2b3e3342a79372866af53cdffa540",
    "binary_size": 74752,
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
    "xrefs",
    "{{BIN}}",
    "--to",
    "memcmp",
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
        "op": "gt",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme.exe",
    "binary_sha256": "30849bed966c92e64009a23df62210e615a2b3e3342a79372866af53cdffa540",
    "binary_size": 74752,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Ambiguity checking precedes import-thunk alias resolution.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `68d9ee36224c0ec5dcedc3fc` (round 3, tester t-r3-68d9ee36)

## Decision log

- filed by cluster.py from 1 observation(s)
captain T_TRIAGE r3: track tooling CONFIRMED (kind=bad-ux, name resolution in `kuna xrefs --to`), touches widened to the analysis-tier xref index the CLI resolves against. No emitted C, so no option.
captain T_TRIAGE r3: repaired the missing probe/acceptance `target` block (binary_rel + sha256 + size, source dataset) -- without it {{BIN}} could not resolve and the need was unclosable by B_DONE and invisible to regression detection. Verified: acceptance now RUNS and FAILS on cf5234ac, which is the state a filed need must be in.
- round 4 BUILDER (b-r4-name-based-xrefs): hypothesis **UPHELD**. The ambiguity really is decided before the alias class exists -- and it could not have been otherwise where the check stood: `alias_class` is the connected component of the DECODED `jmp [slot]` forwarding relation, and the walk that decodes it (`build_with_focus`) runs after target resolution because it needs the target address to focus on. Fixed by not deciding at lookup time: a contested name carries its candidates into the walk as its focus set and is settled against the alias class afterwards (`Resolution::settle`, kuna-cli/src/xrefs.rs). Candidates that are all one class resolve to the class's code half (the veneer); candidates that are not keep the refusal with every candidate named. Acceptance a-6576d0dff8a6 PASSES. Sweep over all 148 duplicate-name entries in 14 binary fixtures: 129 folded, 18 refused, 0 mismatches -- every fold answers byte-identically to each of its own candidate addresses.
