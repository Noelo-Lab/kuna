---
need_id: function-inventory-silently-lists
title: Function inventory silently lists only imports when no code is discovered
track: tooling
status: open
severity: major
probe_id: p-df21f1ca2667
acceptance_id: a-61916bbbfe69
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5433c5d40ad448c1c1]
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

Know that discovery found no implementation functions and needs an explicit entry declaration.

> **Function inventory silently lists only imports when no code is discovered** (major, `5ab77f5433c5d40ad448c1c1`)
> functions returns six zero-sized import entries, exit 0, error null. decompile-all correctly diagnoses the non-executable entry section and suggests --define-function, but the inventory gives no equivalent diagnostic.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "error",
        "op": "eq",
        "value": null
      },
      {
        "path": "functions[*].size",
        "op": "eq",
        "value": 0
      }
    ]
  },
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
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
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "json": [
      {
        "path": "error",
        "op": "ne",
        "value": null
      }
    ]
  },
  "target": {
    "binary_rel": "bin/CryptoME.zip.__x/CryptoME.exe",
    "binary_sha256": "bcfe3d24b0b7453c65476eb11778224b69f47efb7555db4d2240d32b966cf8bd",
    "binary_size": 139776,
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

- `5ab77f5433c5d40ad448c1c1` (round 12, tester t-r12-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `bad-ux|functions|json,stdout_is_json` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD, and the mechanism is a ONE-CONDITION gate that is already written -- no hypothesis was filed, so what this adds is the exact site and two hazards. REPRODUCED on aa18e56d: kuna functions CryptoME.exe --json returns count 6, total 6, error null, exit 0, and every one of the six is a zero-size IMPORT thunk (GetProcAddress 0x4f7f5c, GetModuleHandleA 0x4f7f60, LoadLibraryA 0x4f7f64, EndDialog 0x4f802f, CreateCompatibleDC 0x4f8037, PlaySoundA 0x4f803f). decompile-all on the same file exits 1 and says exactly the right thing: 'no functions discovered ...: its entry point 0x4f7001 lies in section .NEOpack, which the image does not flag executable -- pass --define-function 0x4f7001 to decompile there anyway'. THE DIAGNOSTIC IS ALREADY WIRED INTO functions. run_functions calls it (kuna-cli/src/decompile_all.rs:1120-1124): . The six import thunks make  NON-EMPTY, so the gate is false and the message is never built. So this is not 'add a diagnostic to functions' -- it is 'the emptiness gate counts import stubs as discovered code'. The fix is the predicate: no IMPLEMENTATION function (no entry with size > 0) rather than . THE SAME GATE IS DUPLICATED ONE FUNCTION DOWN: run_summary (decompile_all.rs:1178-1181) has the byte-identical expression, so  has the identical hole on this image. Fix both or the sibling surface stays silent. EXIT CODE IS NOT A TRAP HERE, AND I EXPECTED IT TO BE. The acceptance pins only error != null and says nothing about exit_code, which is the shape that got a sibling need rejected. But the contract is already settled in-repo: tests/cli/zero-functions-exit-0.json is a PROMOTED probe that demands  exit NONZERO with /no functions/ on stderr, and emit_with_discovery_error (decompile_all.rs:1095) returns i32::from(discovery_error.is_some()). So firing the gate makes functions exit 1 automatically and consistently with decompile-all, and that is the established contract, not a regression. THE REAL ACCEPTANCE WEAKNESS IS COUNT, NOT EXIT. The acceptance asserts error != null and pins NOTHING else, so it also goes green if the run simply fails and lists nothing. The six import entries are correct output and must survive: require count 6 with the six names still present alongside the populated error. NO FALSE-POSITIVE RISK ON HONEST EMPTIES: zero_discovery_error already returns None for CodeEvidence::None (decompile_all.rs:1934-1945), so a resource-only PE or a data-only .o does not start erroring; the message only fires on an image that demonstrably carries code. THIS NEED IS ONLY CORRECT WHILE BARE DISCOVERY FINDS NOTHING, AND THAT IS WORTH SAYING OUT LOUD.  ALREADY returns count 7 with a real 11-byte  at 0x4f7001 and error null. So a future fix that makes bare discovery reach into .NEOpack would flip this acceptance RED while strictly improving kuna. It is a diagnostic need by construction; if discovery ever lands first, close this by supersession rather than treating the red acceptance as a regression. NOT IN CONFLICT WITH ITS TWO SIBLINGS ON THE SAME BINARY: batch-silently-omits-explicitly is CLOSED (#495) and pins decompile-all WITH --define-function to count > 0, which the gate change does not touch; neopack-payload-requires-external targets  and is a different surface. Scope is genuinely small -- one predicate, duplicated twice, in one file.
- round 12 REFUTER: hypothesis **upheld**. CORRECTION to the line above -- five code fragments were eaten by shell backtick substitution when it was written. Restoring them verbatim, in order of appearance: (1) the gate at decompile_all.rs:1120-1124 is all.is_empty().then(|| zero_discovery_error(&args.binary)).flatten(); (2) "the six import thunks make ALL non-empty" -- the variable is the local named all; (3) the predicate to replace is all.is_empty(); (4) the sibling surface with the byte-identical gate is kuna functions --summary via run_summary at decompile_all.rs:1178-1181; (5) the promoted probe tests/cli/zero-functions-exit-0.json runs kuna functions BIN --json on decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_x86_64 and expects exit_code ne 0 plus stderr matching (?i)no functions; (6) the command that already recovers the payload function is kuna functions BIN --json --define-function 0x4f7001-0x4f700c=entry, which returns count 7 with an 11-byte entry at 0x4f7001; (7) the sibling neopack-payload-requires-external targets the unpack subcommand. Nothing about the verdict or the evidence changed.
