---
need_id: push-immediate-ret-transfer
title: PUSH immediate / RET transfer to the original entry disappears
track: quality
status: open
severity: major
probe_id: p-8dae1261fd45
acceptance_id: a-d88748a4d135
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [5ab77f5433c5d40ad448c1c1]
rounds: [12]
first_seen_round: 12
attempts: 0
covered_by_option: pushimmediateret
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

See the transfer to 0x40365a after unpacking.

> **PUSH immediate / RET transfer to the original entry disappears** (major, `5ab77f5433c5d40ad448c1c1`)
> Disassembly shows CALL 0x4f7990; PUSH 0x40365a; RET. Default decompilation emits only the first call, without a warning or the original-entry transfer.

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
    "0x4f7001",
    "--addr",
    "--define-function",
    "0x4f7001",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "functions[0].error",
        "op": "eq",
        "value": null
      }
    ],
    "stdout_absent": [
      "40365a",
      "warn:"
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

_none recorded_

## Instances

- `5ab77f5433c5d40ad448c1c1` (round 12, tester t-r12-5ab77f54)

## Decision log

- filed by cluster.py from 1 observation(s)
- split by captain at T_DEDUP from round 12's `wrong-output|decompile|exit_code,stdout_matches` bucket: cluster.py's signature (kind|subcommand|clause-shape) collapsed unrelated defects, so the crop was hand-partitioned one observation per need and filed via `--from-file`. See `.kuna-repipe/rounds/12/dedup/MARKER.json`; do NOT run `cluster --round 12` bare.
- sibling of `call-pop-pointer-helper` — sibling packer-stub idiom; kept separate (Jaccard 0.0), different idiom and different fix.
- round 12 REFUTER: hypothesis **upheld** (was inconclusive). REFUTED IN-TICK (captain, round 12). NO HYPOTHESIS WAS EVER FILED (the section reads "none offered"), so there was no cause to overturn; this verdict attaches to the SYMPTOM, which is exact, and it supplies the diagnosis the need lacked. Verdict recorded as upheld rather than inconclusive deliberately: "inconclusive" IS the filed default string in this backlog, so leaving it there is indistinguishable from nobody having looked.

THE SYMPTOM IS EXACT, FROM THE BYTES. At 0x4f7001: E8 8A 09 00 00 = CALL 0x4f7990 (0x4f7006 + 0x98a), then 68 5A 36 40 00 = PUSH 0x40365a at 0x4f7006, then C3 = RET at 0x4f700b, then four 90 pads. Default output on b9029a8f is exactly two lines -- a call to sub_4f7990 and nothing else -- and the string 40365a does not appear anywhere in stdout. No warning at ANY warnstyle: --option warnstyle banner is byte-identical to the default, so the loss is genuinely silent, not a warning-rendering artifact.

THE MACHINERY IS ALREADY THERE IN FULL -- THE ONLY MISSING PIECE IS THE AUTO-DETECTION. This is the load-bearing finding and it makes the fix small. --assert 'flow 0x4f700b branch' emits, on the unpatched tree:

    void sub_4f7001(void)
    {
      sub_4f7990();
      (*(void *)0x40365a)();
    }

So the PUSH is decoded, the constant IS propagated into the RET's target, and the transfer renders. Nothing needs to be built in dataflow or in the printer. What does not exist is the rule that recognises "a RET whose return address was overwritten by an immediate PUSH in the same block is a branch to that immediate" and applies the override itself.

FOUR NEGATIVES, ALL MEASURED, SO A BUILDER DOES NOT RE-SPEND THEM.
(1) funcboundflow off is byte-identical to the default -- flow is not being bounded at a neighbouring function entry, which was the obvious first guess given the sibling need.
(2) warnstyle banner is byte-identical -- there is no suppressed warning to surface.
(3) The tail-call family does not cover this. tailcalljump and tailcallframe are both default-ON and both keyed on a direct JMP; the idiom here is PUSH/RET, so neither can fire. calltrampoline (default-ON, the sibling need's option) is also keyed on the callee discarding the pushed return address and jumping BACK into the stream -- the opposite direction.
(4) flow 0x4f700b branch and flow 0x4f700b callreturn produce IDENTICAL output. The override is accepted but lands as an indirect CALL through a constant, not as a jump, in both directions.

THE TRAP THAT WOULD MAKE A CORRECT-LOOKING FIX WRONG, AND IT IS THE WHOLE POINT OF THIS NEED. Adding --define-function 0x40365a on top of the override upgrades the line to a clean named call:

    void sub_4f7001(void) { sub_4f7990(); sub_40365a(); }

That looks like the ideal fix and a builder will be tempted to synthesize the function. DO NOT. 0x40365a IS STILL ENCRYPTED ON DISK. The bytes there are 29 60 8C 1D 23 33 2B 30 B8 4D 4C C3 02 54 C0 2B -- ciphertext that the unpacker at 0x4f7990 rewrites at run time -- and decompiling it statically yields a garbage body carrying "warn: Function flows into unmapped memory". The tester's own task line was "See the transfer to 0x40365a AFTER UNPACKING". So the deliverable is the ADDRESS, not a callee: emit the constant transfer (or a warning naming the original entry) and leave 0x40365a undefined. A fix that defines a function at the OEP trades one silent loss for a page of invented code, and it would do that on every packed PE, not just this one.

SECOND-ORDER MODELLING NOTE. The override renders the transfer as an indirect call, which models the stub as calling the OEP and coming back. It does not come back -- PUSH imm / RET is a tail transfer and the stub never regains control. A fix that reproduces the override's output verbatim therefore still gets the control flow wrong in the same way, and any code the decoder puts after that line is unreachable in truth.

THE ACCEPTANCE IS STRUCTURALLY SOUND BUT WEAK, AND IT CAN BE PASSED BY A WARNING. Reproduction demands stdout_absent 40365a and "warn:"; acceptance demands stdout_matches 40365a. They are exact complements, which is the right shape and is worth saying because several round-12 acceptances are not. But the only thing acceptance checks is that the six characters 40365a appear somewhere in stdout, so a one-line warning naming the original entry passes it without recovering any control flow at all. That may well be the right product decision here, given the encrypted-OEP trap above -- just do not read a pass as proof that the transfer was recovered.

RISK OF THE INDICATED FIX: low, and lower than the sibling's. The detection is local and syntactic -- an immediate PUSH reaching a RET in the same basic block with no intervening stack adjustment -- and the semantics are exact rather than heuristic: that RET provably transfers to the pushed constant. The guard to keep is the ordinary case where a pushed constant is an ARGUMENT and the RET belongs to a later frame; requiring the PUSH and the RET to be adjacent in one block, with the PUSH being the last write to the slot the RET reads, excludes it.
- implementation candidate: `pushimmediateret` uses bounded raw-p-code stack provenance and seeds BRANCH, not CALL, for the proven one-store form. The two-store form remains owned by `entryretdispatch`; ordinary, argument-push, adjusted, overwritten, computed, conditional, and opaque cases decline; explicit flow facts win; no target function is synthesized.
- acceptance_id changed to `a-d88748a4d135` and the acceptance was reduced to the vendored CryptoME-shaped PE fixture. It requires the unpacker call followed by the full `switch(0x401000) { case 0x401000:` control-flow construct inside `functions[0].code`, so destination digits appearing only in a warning cannot pass. The encrypted target section has no function symbol.
