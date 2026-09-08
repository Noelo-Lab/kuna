---
need_id: upx-lzma-compression-blocks
title: UPX LZMA compression blocks application analysis
track: loader
status: open
severity: blocker
probe_id: p-cd22131ac715
acceptance_id: a-4b6190dae910
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [68ae3bfc8fac2855fe6fb8ee]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-analysis/src/upx]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Unpack this executable to inventory and decompile application functions.

> **UPX LZMA compression blocks application analysis** (blocker, `68ae3bfc8fac2855fe6fb8ee`)
> functions reported zero functions with an explicit UPX diagnostic; decompile-all refused. unpack exited 1 with unsupported UPX image: compression method 14 (LZMA). The manual documents this limitation; the catalog offers no enabling option.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "unpack",
    "{{BIN}}",
    "-o",
    "notes/unpacked",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "unsupported UPX image: compression method 14 \\(LZMA\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/keygenme",
    "binary_sha256": "63fe1e3eb10c62a59ef9a84dd36d70e562bbfc0eaf05dcef6a64ad46654abe0c",
    "binary_size": 1994512,
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
    "unpack",
    "{{BIN}}",
    "-o",
    "notes/unpacked",
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
    ],
    "stderr_absent": [
      "unsupported UPX image"
    ]
  },
  "target": {
    "binary_rel": "bin/keygenme",
    "binary_sha256": "63fe1e3eb10c62a59ef9a84dd36d70e562bbfc0eaf05dcef6a64ad46654abe0c",
    "binary_size": 1994512,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The unpacker implements NRV methods but lacks the documented LZMA decoder path.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile load target/keygenme` — Server exited with status 1 before registering. No analysis returned; this infrastructure failure does not establish whether IDA can unpack the image.

## Instances

- `68ae3bfc8fac2855fe6fb8ee` (round 8, tester t-r8-68ae3bfc)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the mechanism is exactly as filed -- but the need is mis-scoped and mis-touched, fix both at T_TRIAGE. MEASURED at HEAD bfacf0c6 on the arena image (copied out to /tmp, arena shim untouched): kuna unpack exits 1 with 'unsupported UPX image: compression method 14 (LZMA)'. The refusal is a single by-name arm in method_codec (kuna-analysis/src/upx/elf.rs:507-527), which maps methods 2..10 onto NRV2B/NRV2D/NRV2E x {Byte,Le16,Le32} and returns Unsupported for everything else; upx/nrv.rs is a UCL 1.03 port of those three variants only ('every compression method below LZMA', line 2) and grep finds NO LZMA decoder anywhere in the tree -- the only hits are method_name(14)="LZMA" (upx/mod.rs:354) and the negative test. The ELF walk is NOT the blocker: the packheader parses (d[UPX!+6]==14 in the file matches what the error names), so everything upstream of the block decode already works and a decoder is the whole gap. THREE THINGS THE FILING GETS WRONG, all of which would bite a builder: (1) touches is [kuna-cli] but the code is entirely in decompiler/crates/kuna-analysis/src/upx/ -- the cli side is only unpack.rs's JSON writer. Retouch it or the lease is wrong. (2) scope: small is wrong. There is no lzma/xz crate in [workspace.dependencies], so closing this means either a NEW third-party dependency (the tree has precedent -- object, cpp_demangle, rustc-demangle are all documented dependency substitutions) or a hand-port, AND UPX does not emit a container: it is a raw LZMA1 stream whose lc/lp/pb come from the block header, so a stock 'decompress a .lzma file' API does not apply. That is a design call with a new dep => medium at best, and a [PROPOSAL] is the honest route. (3) A builder MUST update kuna-cli/tests/unpack_cli.rs:206-226 (an_unimplemented_method_is_refused_by_name) or make rust-test goes red. Note that fixture is an NRV stream RE-HEADED as method 14 with a repaired checksum, so after a real decoder lands it must STILL fail -- as a decode error, not as an unimplemented-method refusal. The acceptance probe is well-formed and closable: unpack --json really does emit "count" (= blocks.len(), kuna-cli/src/unpack.rs:156), so count>0 is reachable.
- round 8 TRIAGE (captain): RETOUCHED + RESCOPED off the r8 refutation. touches was [kuna-cli] and every line of code is decompiler/crates/kuna-analysis/src/upx/ (method_codec in elf.rs:507-527 maps methods 2..10 onto NRV2B/2D/2E x {Byte,Le16,Le32} and refuses the rest by name; nrv.rs is a UCL 1.03 port of those three only; there is no LZMA decoder anywhere in the tree). track tooling -> loader for the same reason. scope small -> LARGE, deliberately: there is no lzma/xz crate in [workspace.dependencies] and UPX emits a RAW LZMA1 stream (no container, lc/lp/pb off the block header), so closing this is a new third-party dependency or a hand-port -- a design call, i.e. the [PROPOSAL] route. (scope has no 'medium': the writer accepts small|large only.) A builder MUST also update kuna-cli/tests/unpack_cli.rs:206-226 (an_unimplemented_method_is_refused_by_name) or make rust-test goes red -- that fixture is an NRV stream RE-HEADED as method 14, so after a real decoder lands it must STILL fail, as a DECODE error rather than an unimplemented-method refusal. Acceptance is closable as written: unpack --json really emits count (= blocks.len(), kuna-cli/src/unpack.rs:156).
