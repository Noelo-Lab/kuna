---
need_id: argument-recovery-knobs-still
title: call-argument recovery options are inert: enabling calleearity/varargstackargs changes nothing
track: quality
status: open
severity: major
probe_id: p-ea5af8c3b2f7
acceptance_id: a-a94fa26848a4
hypothesis_status: overturned
credibility: 1.0
instances: 2
challenges: [6547b4d50f4238b24302b588, 69a3822f7b3cc38c80464da4]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-decomp]
scope: small
regression_of: call-args-discarded
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Recover the allocation size passed to sub_140002c90 in the checker.

> **Argument recovery knobs still emit a zero-argument allocation call** (major, `6547b4d50f4238b24302b588`)
> Even with both newly advertised recovery options enabled, one branch emits sub_140002c90() while adjacent branches emit sub_140002c90(v11 + 0x27). The option-enabled output is byte-for-byte identical to the default output.

> **Recovered NtWriteFile call loses its native API arguments even with both call-argument recovery options** (major, `69a3822f7b3cc38c80464da4`)
> Default output emits NtWriteFile(v10,0,0). Re-running with --option calleearity on and separately with --option varargstackargs on produced the same three-argument call, despite NtWriteFile having a larger fixed native prototype.

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
    "0x1400011c0",
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
      "sub_140002c90\\(\\);",
      "sub_140002c90\\(v[0-9]+ \\+ 0x27\\)"
    ]
  },
  "target": {
    "binary_rel": "bin/trappy attack.exe",
    "binary_sha256": "4872962a99da4def9048556345bdd1699a41b77dc0a148c8c28412b8adb3a06e",
    "binary_size": 26112,
    "binary_source": "dataset"
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x1400011c0",
    "--addr"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_absent": [
      "sub_140002c90\\(\\)",
      "sub_140002c90\\([^)]*,"
    ],
    "stdout_matches": [
      "sub_140002c90\\([^),]*\\+ 0x27\\)"
    ]
  },
  "cwd": "{{WORK}}",
  "repeat": 1,
  "probe_id": "a-a94fa26848a4",
  "target": {
    "binary_rel": "bin/trappy attack.exe",
    "binary_sha256": "4872962a99da4def9048556345bdd1699a41b77dc0a148c8c28412b8adb3a06e",
    "binary_size": 26112,
    "binary_source": "dataset"
  }
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The recovered callee arity is not applied consistently at this conditional call site.
- The imported NtWriteFile prototype may be incomplete, so calleearity cannot distinguish the prepared stack arguments from unrelated locals.

## Refutation

**OVERTURNED (captain, round 2 T_REFUTE)** — the symptom stands, the diagnosis does not, and the acceptance probe as written is satisfiable by WRONG output.

- **The options are not inert.** `kuna catalog` reports `calleearity` **default on** (DIV-102) and `varargstackargs` **default on** (DIV-101), so "I enabled them and the output was byte-for-byte identical" is the expected result of enabling something already enabled — it is not evidence of an unwired option. A/B on the probe target (`trappy attack.exe` @ 0x1400011c0): default vs `--option calleearity off --option varargstackargs off` differs by **0 lines**, i.e. they change nothing in either direction here, while `--option compareform canonical` moves **104 lines** and `--option spillargtrial reload` moves **8** on the same command. `--option` reaches this path; `calleearity` declines at this site.
- **A fix that forces the argument in produces wrong arguments — demonstrated, not argued.** `spillargtrial reload` (shipped, default-off) already makes this need's ACCEPTANCE pass: `sub_140002c90();` disappears. It does so by giving *both* sites a second argument — `sub_140002c90(v11 + 0x27,v15)` and `sub_140002c90(v11,v15)` — fabricating a trailing argument at the site that was already correct and rewriting the size at the other. The acceptance probe only asserts `stdout_absent: sub_140002c90\(\);`, so it cannot tell a recovery from a fabrication.
- **Neither hypothesis bullet survives.** "Arity not applied consistently" describes the symptom, not a cause; the open mechanism question is why `calleearity` does not promote from the *earlier* sibling site (line 232 carries `v11 + 0x27`, line 241 does not) — its own `use_when` says that is precisely the case it exists for, so either the trial at 241 has no real Varnode left to promote or the reconciliation is not reached. The NtWriteFile bullet is off-mechanism: `calleearity` reconciles sibling *internal* call sites and has no bearing on an imported prototype.
- **For the builder:** strengthen the acceptance to assert the argument *value* at the second site and the *absence* of a second argument, then answer the promote-failure question. Do not build a new forcing mechanism — one already exists and it over-recovers here.

## Reference

_none recorded_

## Instances

- `6547b4d50f4238b24302b588` (round 2, tester t-r2-6547b4d5)
- `69a3822f7b3cc38c80464da4` (round 2, tester t-r2-69a3822f)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 2 T_DEDUP (captain): FORCED MERGE of two observations cluster.py kept apart (`Argument recovery knobs still emit a zero-argument allocation call`, 6547b4d5 PE x86-64; `Recovered NtWriteFile call loses its native API arguments`, 69a3822f PE x86-64). Their probe signatures differ only in an `exit_code` clause, but both testers report the SAME checkable fact: they enabled the round-1 options and the output was byte-for-byte identical to the default. That shared fact, not the two different hypotheses (inconsistent arity at a conditional call site vs an incomplete imported prototype), is what this need asserts.
- round 2 T_DEDUP (captain): `regression_of: call-args-discarded` is KEPT, and it is a CLAIM, not a verified regression. `verify --acceptance-suite --need call-args-discarded` returns `indeterminate: no acceptance probe on the record or in the probe store` -- round 1 closed that need on prose ('Shipped: varargstackargs and calleearity') with no probe, so nothing can mechanically flip it to `regressed`. Kept because the loop's worst failure mode is believing a capability shipped when it did not, and two independent testers on two binaries say it did not.
- round 2 T_DEDUP (captain): three MORE sightings in this family did not reach clustering and must not be read as absent -- 5ab77f55 obs1 (blocker, 'closed call-arity options do not recover checker comparison arguments') gated not-reproducible with the arms INVERTED, and 69e005b6 obs0/obs1 (both blocker: a parser emitted void although its caller consumes a success value; calleearity leaves an argument dropped at a two-argument rotate call) were lost to the probe JSON-escape defect, intact in the arena report.json. Six sightings, five challenges.
- round 2 T_REFUTE brief: the hypothesis to attack is NOT either tester's -- it is 'the options are inert'. Check whether calleearity/varargstackargs even reach these call sites (a byte-identical output is equally consistent with an unwired option and with an option correctly declining), and whether a fix that forces arguments in would emit WRONG arguments at the conditional call site.
- round 2 T_REFUTE (captain): hypothesis **OVERTURNED**, symptom stands. Both options are default-ON, so the tester's "enabling them changed nothing" is expected; A/B with them OFF is also byte-identical while `compareform`/`spillargtrial` move this same output, so `--option` reaches the path and `calleearity` simply declines here. `spillargtrial reload` already makes the ACCEPTANCE pass by fabricating a second argument at both sites -- the acceptance probe must be strengthened to assert argument VALUES before this need is dispatched.
- round 2 T_TRIAGE (captain): acceptance REPLACED a-90b7aa54ca53 -> a-a94fa26848a4. The old probe asserted only `stdout_absent: sub_140002c90\(\);`, which the shipped default-off `--option spillargtrial reload` ALREADY satisfies -- by fabricating a second argument at BOTH sites (`sub_140002c90(v11 + 0x27,v15)` / `sub_140002c90(v11,v15)`), i.e. by rewriting a site that was already correct. A builder could have closed this need with wrong C. Re-measured this tick on `trappy attack.exe` @0x1400011c0: default gives `sub_140002c90(v11 + 0x27)` at line 232 and `sub_140002c90();` at 241; the surrounding code is the MSVC aligned-new idiom (the `else if (v11)` small-allocation arm), so the correct argument at 241 is `v11`. The new acceptance asserts three clauses -- no zero-argument call, no call carrying a second argument, and the already-correct site still passing exactly one `... + 0x27` argument. Verified by replay: it FAILS on default output (the bug) and FAILS on `spillargtrial reload` output (the fabrication), so it discriminates a recovery from a forcing. Argument SHAPE is asserted rather than the literal `v11` so a fix that renumbers locals still passes; the accepted residual is that a wrong single value would pass.
- round 2 T_TRIAGE (captain): track/touches/scope CONFIRMED as filed (quality / kuna-decomp / small) with one binding constraint: `calleearity` and `varargstackargs` are BOTH DEFAULT-ON (DIV-102/DIV-101), so any change to their promotion rule changes output for every binary in the corpus. That must ship behind a NEW default-off option, never as a silent edit to a default-on rule. If the investigation concludes the fix cannot be expressed that way, STOP and escalate to a [PROPOSAL] rather than flipping default behaviour under a small-scope contract.
