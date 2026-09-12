---
need_id: cancelling-byte-arithmetic-splits
title: Cancelling byte arithmetic splits a declared string buffer
track: quality
status: closed
severity: minor
probe_id: p-6e4b95aaf73b
acceptance_id: a-88e307f76db7
hypothesis_status: upheld
credibility: 0.7
instances: 1
challenges: [69d7f15a8afd9d6c48b48871]
rounds: [11]
first_seen_round: 11
attempts: 1
covered_by_option: cancelbytearithmetic
touches: [decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_4.rs]
scope: small
regression_of: null
pr: 621
closed_in_round: 12
closing_pr: "621"
reject_reason: null
---

## Symptom

Read the constant usage message after defining its buffer as char usage[22].

> **Cancelling byte arithmetic splits a declared string buffer** (minor, `69d7f15a8afd9d6c48b48871`)
> The data assertion works, but usage[0xf] retains -2*x + 'w' + (x<<1) modulo 256. This splits the string around its constant w. readonly on leaves the same output. Similar expressions obscure other message bytes.

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
    "0x1406fa160",
    "--assert",
    "data 0x1408f1c08 char usage[22]",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "json": [
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "usage\\[0xf\\] = [^;]*dat_[0-9a-f]+[^;]*<<"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "36bfae11c18fb5fa214110d7f17cdc92026bce53e28e9a1965b4193e59c1a6a1",
    "binary_size": 9384960,
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
    "0x1406fa160",
    "--assert",
    "data 0x1408f1c08 char usage[22]",
    "--json"
  ],
  "expect": {
    "stdout_is_json": true,
    "stdout_absent": [
      "usage\\[0xf\\] = [^;]*dat_[0-9a-f]+"
    ],
    "json": [
      {
        "path": "functions[0].code",
        "op": "matches",
        "value": "builtin_strncpy\\(usage,\"Usage: %s <password>\\\\n\",0x15\\);"
      }
    ]
  },
  "target": {
    "binary_rel": "bin/crackme_shroud.exe",
    "binary_sha256": "36bfae11c18fb5fa214110d7f17cdc92026bce53e28e9a1965b4193e59c1a6a1",
    "binary_size": 9384960,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Narrowing casts may prevent cancellation of multiply and shift terms.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69d7f15a8afd9d6c48b48871` (round 11, tester t-r11-69d7f15a)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 11 REFUTER: hypothesis **upheld** (was inconclusive). UPHELD IN SUBSTANCE -- the narrowing IS what blocks the cancellation, and the exact refusing line is named below -- but TWO corrections to the filed story, one of which changes the repro.

CORRECTION 1: THE DATA ASSERTION IS NOT INVOLVED. The need is written as if the char[22] assertion were part of the defect ("The data assertion works, but usage[0xf] retains..."). Measured: drop the assertion entirely and run bare `decompile BIN 0x1406fa160 --json` -- the identical uncancelled expression appears at the same byte, as dat_1408f1c17 = (char)(dat_1408f1008 & dat_1408f100c) * '\xfe' + 'w' + (char)((dat_1408f1008 & dat_1408f100c) << 1). (0x1408f1c08 + 0xf = 0x1408f1c17.) So this is a plain expression-simplification gap reproducible with NO options and NO assertions; the assertion only renames the destination. A builder should develop against the bare command.

CORRECTION 2: THE ARITHMETIC IS SOUND, SO THIS IS A MISSED SIMPLIFICATION, NOT A REFUSAL TO BE WRONG. '\xfe' is -2 and the store is 8-bit, so the value is (-2X + 2X + 'w') mod 256 = 'w' = 0x77. Truncation order does not threaten it: 2*(X mod 256) = 2X mod 256, so the two addends cancel at any width. The surrounding bytes confirm the answer independently -- 0x1408f1c08.. spells "Usage: %s <pass" then this byte then "ord>", i.e. the string is "password" and the byte must be 'w'.

THE BLOCKING LINE. p3_dataflow/ruleaction_4.rs:695-710, RuleSubCommute (upstream ruleaction.cc:4465). The rule DOES handle this exact shape -- CPUI_SUBPIECE over CPUI_INT_LEFT, offset 0 -- but gates it: it reads the shift's own input and returns 0 unless that input is defined by CPUI_INT_ZEXT or CPUI_PIECE (and returns 0 outright if the input is not written). Here the shift's input is the INT_AND of two loads, so the rule bails, the SUBPIECE never moves inside the shift, the two addends never reduce to a common base varnode, and term collection cannot add the coefficients 0xfe + 2 = 0x100 = 0 mod 256. This is upstream Ghidra's own gate faithfully ported, NOT a kuna porting defect -- so any fix is a deliberate DIVERGENCE and needs a DIV row plus an option, not a "repair the port" framing. Confirm the operand shape with a p-code dump before writing code: the INT_AND read is inferred from the emitted C, which is strong but is not a dump.

WRONG-FIX TRAP. The ZEXT/PIECE gate is conservatism, not correctness -- commuting SUBPIECE to the low piece through a left shift is arithmetically safe. The risk in relaxing it is RULE OSCILLATION: RuleSubCommute deliberately "prefers short forms" and other rules re-widen operands, so a broadened gate can ping-pong with them, and it fires everywhere, not just here. Budget for a full datatest run and a speed delta; a narrow relaxation (offset 0 and a left shift by a constant less than the output bit width) is far likelier to survive than removing the gate.

*** ACCEPTANCE HOLE -- THE THIRD OF THIS ROUND. DO NOT DISPATCH BEFORE FIXING IT. *** The acceptance is stdout_absent on "usage\[0xf\] = [^;]*dat_[0-9a-f]+" plus a json clause requiring only that the code MATCHES "usage". Absence is satisfied by the line VANISHING, and the "usage" clause is satisfied by any other line -- builtin_strncpy(&usage[0x10],"ord>\n",5) already appears in today's output and would carry it alone. So a change that DROPS the store entirely scores as a pass while destroying a write; that is not hypothetical in this tree given the marker-copy-propagation store-deletion hazard (DIV-105/#381). Tighten to a POSITIVE clause requiring the constant, e.g. code matches "usage\[0xf\] = 'w'" (or 0x77), and keep the stdout_absent as a secondary.

ALSO FOR T_TRIAGE: this need is filed track: perf. It is a wrong-output simplification gap with no timing content -- it should be track: quality. touches is empty; it should name decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_4.rs.
- round 11 TRIAGE (captain): track perf -> quality, touches [] -> p3_dataflow/ruleaction_4.rs, as the refuter directed. Nothing here is timing: it is an uncancelled -2X + 2X + 'w' the simplifier will not fold because RuleSubCommute (ruleaction_4.rs:695-710) bails when the shift's input is an INT_AND. Filed as perf it would have skipped the quality counter leases, and this fix is a deliberate divergence from an upstream gate -- it needs a DIV row and an option, which is exactly what those leases serialize.
- round 11 TRIAGE (captain): REPAIRED THE ACCEPTANCE -- the third hole of the round. Both clauses were satisfiable by a change that DESTROYS the write: the stdout_absent is met by the line vanishing, and the json clause required only that the code match `usage` somewhere, which an unrelated `builtin_strncpy(&usage[0x10],...)` already carries today. That is not hypothetical given the marker copy-propagation store-deletion hazard (DIV-105/#381). Replaced the json clause with a positive one demanding the folded value, `usage\[0xf\] = (?:'w'|0x77|119);`, accepting all three plausible spellings of the byte; the stdout_absent stays as the secondary guard. Measured against today's output (`usage[0xf] = (char)(dat_1408f1008 & dat_1408f100c) * '\xfe' + 'w' + ...;`): no match, so it still fails today.
- round 12 captain: acceptance_id corrected a-fbc0ab08e1eb -> a-e9bac45bb85d. The id is derived from cmd+expect; an earlier tightening rewrote the assertion without recomputing it, so the front-matter label no longer named the assertion that runs. Body unchanged -- this is a relabel only. It matters at promotion (the opportunity selector is the acceptance_id) and at dedup (cluster.py trusts a stored id over recomputing, so a re-filing of the same probe would have opened a duplicate need instead of adding an instance).
- closed by #621: acceptance a-88e307f76db7 now PASSES. The exact modulo-256 cancellation joins the complete `"Usage: %s <password>\n"` initializer; the option-off control preserves all three original writes, and the corpus sweep found no non-target body changes.
