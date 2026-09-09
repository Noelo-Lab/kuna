---
need_id: byte-overlay-assertion-recovered
title: No byte-overlay assertion for recovered self-modifying code
track: tooling
status: open
severity: major
probe_id: p-a0a4c1957f1e
acceptance_id: a-09471914203f
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f6633c5d40ad448cbec]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Supply recovered plaintext at original virtual addresses through durable assertions, then analyze subsequent loader stages.

> **No byte-overlay assertion for recovered self-modifying code** (major, `5ab77f6633c5d40ad448cbec`)
> The bytes directive is rejected with exit 2. Applying the recovered 2967-byte transform required Python and a copied executable. The first decrypted layer exposed further encrypted code and exception-based control flow; no serial was recovered.

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
    "0x43d060",
    "--addr",
    "--define-function",
    "0x43d060-0x43d0c6=decrypt_stage1",
    "--assert",
    "bytes 0x43d0c6 8bd581c21f324000",
    "--assert-strict",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 2
    },
    "stderr_matches": [
      "unknown directive \"bytes\""
    ]
  },
  "target": {
    "binary_rel": "bin/LoaderCrackMev2.0.zip.__x/corrupt.exe",
    "binary_sha256": "bb6cd6045dff9e299f60299e8dae22abb57eafd125221bb72f3d671f138f00fc",
    "binary_size": 220798,
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
    "0x43d060",
    "--addr",
    "--define-function",
    "0x43d060-0x43d0c6=decrypt_stage1",
    "--assert",
    "bytes 0x43d0c6 8bd581c21f324000",
    "--assert-strict",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "stderr_absent": [
      "unknown directive"
    ]
  },
  "target": {
    "binary_rel": "bin/LoaderCrackMev2.0.zip.__x/corrupt.exe",
    "binary_sha256": "bb6cd6045dff9e299f60299e8dae22abb57eafd125221bb72f3d671f138f00fc",
    "binary_size": 220798,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

_none offered_

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `5ab77f6633c5d40ad448cbec` (round 12, tester t-r12-5ab77f66)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `missing-capability|decompile|exit_code,stderr_absent,stdout_is_json` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). REFUTER (round 12, captain in-tick, pre-dispatch check). VERDICT UPHELD. No hypothesis was ever filed for this need -- the section offers none -- so there was no diagnosis to overturn. I recorded upheld against the symptom rather than inconclusive, because that string IS the filed default and would read as "nobody looked".

WHY THIS WAS REFUTED AT ALL, given it is effectively an absence need: for a missing-capability ask the load-bearing question is not "is the root cause right" but "does the capability already exist under another spelling", because a builder that starts here will otherwise ship a duplicate of a mechanism kuna already has. That check is the whole of this refutation and it came back clean.

MEASURED ON b9029a8f, release binaries:

1. The directive vocabulary is closed and does not contain bytes. From kuna decompile --help:
       function, typedef, prototype, data, param, return,
       name, type, comment, flow, readonly, volatile
   The rejection is a real closed-set rejection, emitted at
       decompiler/crates/kuna-cli/src/assertdecl.rs:319
   as: --assert {raw}: unknown directive {other} (want one of ...). So the symptom reproduces exactly as filed and exit 2 is the parser, not a downstream failure.

2. NO ADJACENT MECHANISM SUPPLIES BYTE CONTENT. I searched the whole 176-option catalog for overlay / patch / selfmod / self-mod / smc / byte content. It returned 15 apparent hits and ALL FIFTEEN ARE FALSE POSITIVES -- every one is the substring "patch" inside DISPATCH in jump-table prose (V850 jmp reg, lowered-switch cascade, CMOV dispatch, image-base-relative tables, the MSVC Duff's-device tables). There is no overlay option. DO NOT RE-RUN THAT GREP; it is answered.

3. kuna docs, the whole embedded manual, has ZERO matches for patch, overlay, self-modif or --bytes.

4. The nearest neighbours are not substitutes. --assert data declares a TYPE at an address, not content. --raw-image sets a load base for a headerless image; it does not overlay bytes into an already-mapped PE. The tester's recorded workaround -- Python plus a copied executable -- is exactly the evidence that no in-tool route exists.

CONCLUSION FOR THE BUILDER: the gap is real and it is a genuinely new directive, not a rename or a re-exposure of existing machinery. Scope stands at small and track stands at tooling: this is CLI/assertion-parser surface (kuna-cli/src/assertdecl.rs), not a decompiler-phase change, so it should need no option, no phases.toml row and no catalog-count bump -- which is why it holds only its own cluster lease and none of the counter:* or file:* leases.

ONE HAZARD WORTH CARRYING: the probe writes bytes at 0x43d0c6 INSIDE the range 0x43d060-0x43d0c6 that the same command defines as decrypt_stage1, and the acceptance is on a self-modifying stage-1 unpacker. Ordering will matter -- an overlay applied after the image is mapped but before code is lifted is the only ordering that can work, and the range-property-ordering precedent in this repo (readonly/volatile are silently inert if painted after symbols are mapped) is the direct analogue. Get the ordering wrong and the directive parses, accepts, and does nothing.
