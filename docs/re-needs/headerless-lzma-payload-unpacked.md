---
need_id: headerless-lzma-payload-unpacked
title: Headerless LZMA payload cannot be unpacked with analyst-supplied parameters
track: tooling
status: open
severity: blocker
probe_id: p-c9020a5d7fe1
acceptance_id: a-810fe11558d8
hypothesis_status: upheld
credibility: 0.85
instances: 1
challenges: [68ac5cd78fac2855fe6fb898]
rounds: [9]
first_seen_round: 9
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli, decompiler/crates/kuna-analysis]
scope: medium
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover an analyzable image, with an explicit compressed-range and codec-parameter override when UPX header discovery fails.

> **Headerless LZMA payload cannot be unpacked with analyst-supplied parameters** (blocker, `68ac5cd78fac2855fe6fb898`)
> unpack exits 1 with no UPX PackHeader found and states this file is not one. Help and catalog expose no unpack override. Bounded decompilation identifies the decoder and input at 0x1403ea000. Python LZMA recovered 5489913 bytes containing KeygenMeForm RTTI. Recovery is incomplete: no end marker was seen, and imports, relocations, and executable reconstruction remain undone.

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
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 1
    },
    "stderr_matches": [
      "no UPX PackHeader found"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "target": {
    "binary_rel": "bin/keygenme.exe",
    "binary_sha256": "d0eeafd9b889a954199137ab947f9ac08f1b816eb302be39072ed014f35c9e09",
    "binary_size": 1400832,
    "binary_source": "dataset"
  },
  "cmd": [
    "{{KUNA}}",
    "unpack",
    "{{BIN}}",
    "-o",
    "{{TMP}}/payload.bin",
    "--raw-lzma",
    "0x1403ea000:0x14053e47c",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "unpacked_size",
        "op": "eq",
        "value": 5489913
      },
      {
        "path": "output",
        "op": "matches",
        "value": "payload\\.bin$"
      }
    ],
    "stderr_absent": [
      "no UPX PackHeader found"
    ]
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- PackHeader discovery is mandatory even when stream parameters can be recovered independently. The acceptance exercises automatic unpacking; an explicit override would need an updated invocation.

## Refutation

_not yet refuted_

## Reference

- `python3 notes/recover_lzma.py` — Recovered 5489913 bytes; end marker seen: False. Payload contains KeygenMeForm and KeygenMe RTTI.

## Instances

- `68ac5cd78fac2855fe6fb898` (round 9, tester t-r9-68ac5cd7)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 9 REFUTER: hypothesis **upheld** (was inconclusive). CAPTAIN REFUTATION (r9 tick 04:10-04:30Z), verdict UPHELD -- and the tester's own second sentence is the important half: THIS ACCEPTANCE CANNOT BE CLOSED BY THE FEATURE THE TITLE ASKS FOR.

FILED CAUSE, PART 1 -- VERIFIED. 'kuna unpack --help' prints exactly 'usage: kuna unpack <binary> [-o OUT] [--json]'. There is no range, codec, props or dict-size override, so PackHeader discovery really is mandatory. Repro on the release binary against arena/9/68ac5cd78fac2855fe6fb898/target/keygenme.exe (1,400,832 bytes): exit 1, 'no UPX PackHeader found' plus 'kuna unpack handles UPX-packed executables; this file is not one'. Both messages are CORRECT -- this file genuinely is not UPX.

FILED CAUSE, PART 2 -- ALSO VERIFIED, AND IT IS THE REASON A BUILDER MUST NOT TAKE THIS NEED AS WRITTEN. The acceptance runs bare 'kuna unpack {{BIN}} --json' and demands exit 0. It passes NO override, so satisfying it requires kuna to AUTOMATICALLY recognise and unpack the payload. What that payload actually is (from the tester's own notes/packed-data.json + recover_lzma.py, both re-read this tick): a raw LZMA1 stream at 0x1403ea000..0x14053e47c, 1,393,788 packed bytes, prefixed by a TWO-BYTE APP-SPECIFIC parameter header -- byte0 low 3 bits = pb (0x1a -> 2), byte1 = lp<<4|lc (0x03 -> lp 0, lc 3), which is neither the 13-byte .lzma container nor the standard single props byte (lc+9*lp+45*pb would be 0x5d). The tester only knew those two bytes because they read the decoder's setup code. Auto-detecting that is not 'headerless LZMA support', it is recognising one packer's private header.

THE TRAP, SAME SHAPE AS THE r9 BERIA ONE: an 'exit_code 0 + stderr_absent' acceptance on a MISSING-CAPABILITY need is passable by DEGRADING THE ERROR PATH. Making 'unpack' exit 0 with a warning on any non-UPX file passes this acceptance, ships a regression of a correct diagnostic, and delivers nothing.

AND EVEN THE HONEST VERSION DOES NOT YIELD AN ANALYZABLE IMAGE. notes/decompressed.bin is 5,489,913 bytes and starts '48 8d 05 49 e6 01 00 / 48 89 05 5a 5a 4f 00 / c3' -- raw x86-64 code, no MZ, no PE header, no section table, no imports, no relocs; the decoder also reported end marker seen: False. So an override that writes this blob out gives a file kuna cannot load as a program. The need's own Reference says the same ('Recovery is incomplete ... imports, relocations, and executable reconstruction remain undone').

WHAT I RECOMMEND AT T_TRIAGE / TO THE BUILDER. Keep the need, keep severity, but REWRITE THE ACCEPTANCE to exercise the override the title asks for, not automatic unpacking: invoke 'kuna unpack {{BIN}} -o <out> --raw-lzma 0x1403ea000:0x14053e47c --lzma-props <pb,lp,lc or the 2-byte prefix>' (spelling is the builder's call) and assert exit 0 PLUS a positive witness on the produced artifact -- byte length 5489913, or the JSON reporting the payload's load address -- so the clause cannot be passed by an error-path change. With that acceptance the work is a bounded kuna-cli change and scope=small holds. WITHOUT it, the need reads as 'reconstruct a PE from a raw code blob', which is a plane and belongs in a PROPOSAL, not a builder slot. Do NOT dispatch this need until its acceptance is rewritten.

### 2026-09-08T05:51Z — captain, round 9 B_PLAN: NOT DISPATCHED (acceptance over-scoped)

Ranked #2 and lease-free, but deliberately skipped. Two reasons, both measured:

- The acceptance as filed is `kuna unpack {{BIN}} --json` exiting 0 with no
  "no UPX PackHeader found". That demands kuna **automatically** detect and unpack a
  headerless LZMA payload out of a non-UPX PE — strictly more than this need's own title
  ("with analyst-supplied parameters") asks for, and a questionable default besides.
- `kuna unpack --help` today is exactly `usage: kuna unpack <binary> [-o OUT] [--json]` —
  there is no override surface at all, so the real deliverable is a new CLI surface plus
  LZMA recovery. The filing tester itself recorded the recovery as incomplete (no end
  marker; imports, relocations and executable reconstruction undone). `scope: small` is a
  bad inference.

No target was bound, so it stays `indeterminate` in the acceptance suite rather than
reading as a genuine failure. Before it is dispatched, T_TRIAGE should rewrite the
acceptance around an explicit analyst-supplied override (codec + compressed range, with
the tester's measured input at 0x1403ea000 and 5489913 recovered bytes as the witness)
and re-scope it. The symptom is real; only the acceptance is wrong.

### 2026-09-08T06:20Z — captain, round 9 B_DRAIN: acceptance REWRITTEN and target BOUND

The T_TRIAGE rewrite the last two ticks asked for, done. Measured this tick against the
bound image (`bin/keygenme.exe`, 1,400,832 B, sha `d0eeafd9…`), not re-derived from the
tester's notes:

- **The file is UPX-sectioned.** Its three sections are `UPX0` / `UPX1` / `.rsrc`, image
  base `0x140000000`. `UPX1` starts at VA `0x1403ea000`, raw offset `0x400` — so the
  tester's payload address is not an arbitrary blob location, it is byte 0 of `UPX1`.
  Only the *PackHeader* is missing. "Headerless LZMA payload" undersells it: this is a
  UPX image with a stripped PackHeader.
- **The two prop bytes at that VA are `1a 03`** — and that is bit-for-bit the layout
  `kuna_analysis::upx::lzma::Properties::split` already parses (`pb = head[0] & 7` = 2,
  `lc = head[1] & 0x0f` = 3, `lp = head[1] >> 4` = 0). kuna therefore already owns a raw
  LZMA1 decoder for exactly this stream. The deliverable is *exposing* it, not writing one.
- **The witness constant is reproducible.** Decoding `0x400+2 .. 1394812` as raw LZMA1
  (lc 3 / lp 0 / pb 2) yields **5,489,913 bytes** starting `48 8d 05 49 e6 01 00 48`,
  matching the recorded blob. Note `eof=False` — the stream has **no end marker** — and a
  1 MiB dictionary fails where 4 MiB succeeds.

**The new acceptance, and why each clause is there.** It invokes the override the title
asks for and asserts a *positive* witness, so it cannot be passed by degrading the
`unpack` error path (the trap the refutation flagged):

```
kuna unpack {{BIN}} -o {{TMP}}/payload.bin --raw-lzma 0x1403ea000:0x14053e47c --json
```

- `unpacked_size == 5489913` is the load-bearing clause. **It passes no size in**, so it
  is non-circular: only a real decode produces it.
- Because no size is supplied, the builder must decode **to input exhaustion**.
  `upx::lzma::decompress(src, u_len)` today errors on a stream that does not reach `u_len`
  exactly, and an analyst with a stripped PackHeader does not know `u_len`. An extra
  analyst-supplied-size convenience flag is fine; it cannot be the *only* path.
- `output` matching `payload\.bin$` pins that the artifact was actually written.

**The flag spelling above is binding**, not a suggestion — the acceptance is the contract
and its id hashes the cmd. VA endpoints (what an analyst reads out of the decompiler, as
this tester did); accepting file offsets too is the builder's option.

`scope` small → **medium**: CLI surface + VA→offset mapping + a decode-to-exhaustion mode
in the analysis crate. Driver/loader tier, no emitted-C change, so no option row. This is
now dispatchable — the "do NOT dispatch" hold from 05:51Z is **lifted**.
