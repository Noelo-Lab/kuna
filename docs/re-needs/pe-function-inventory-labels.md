---
need_id: pe-function-inventory-labels
title: PE function inventory labels executable routines as vftables
track: tooling
status: closed
severity: major
probe_id: p-8e912566d7ea
acceptance_id: a-d6666365cbb6
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [6547b4d50f4238b24302b588]
rounds: [2]
first_seen_round: 2
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/functions.rs, decompiler/crates/kuna-analysis/src/listing]
scope: small
regression_of: null
pr: null
closed_in_round: 2
closing_pr: "382"
reject_reason: null
---

## Symptom

A trustworthy function inventory for triaging the PE.

> **PE function inventory labels executable routines as vftables** (major, `6547b4d50f4238b24302b588`)
> The inventory includes executable ranges named std::bad_alloc::vftable_1 and numerous std::basic_stringbuf::vftable_N entries as functions, including ranges hundreds of bytes long. These names are data-symbol aliases and misleadingly classify routines as vtable objects.

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
    "stdout_matches": [
      "std::bad_alloc::vftable_1",
      "std::basic_stringbuf::vftable_[0-9]+"
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
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
    "--json"
  ],
  "expect": {
    "stdout_absent": [
      "\"name\": \"[^\"]*::vftable_[0-9]+\""
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

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- Public data aliases near code addresses are winning the canonical-name selection for discovered function entries.

## Refutation

**Hypothesis OVERTURNED (round 2 builder `b-r2-pe-function-inve`); the symptom stands.**

The name is not a data-symbol alias winning canonical-name selection. `<Class>::vftable_<i>`
is a name kuna SYNTHESISES, and it is attached to a genuine function. The `rtti` pass (R3,
`decompiler/crates/kuna-analysis/src/analyzers/rtti/mod.rs`) walks each recovered vftable and
emits, per slot, a `SymFact{Function}` **at the address the slot points at** — the virtual
method itself, not the table. All 10 flagged entries on `trappy attack.exe` are real
`.text` routines (0x140001040/32B … 0x140001de0/496B), and the classification as functions is
correct. What was wrong was only the name: it spells the data object that points at the code,
and since an MSVC class under multiple inheritance really does own several vftables, the
indexed form reads as "this class's i-th vftable" rather than "the function in slot i".

Two consequences for the fix: nothing about function DISCOVERY changes (the captain's STOP
clause does not fire — this is pure labelling), and no option is needed.

Adjacent, NOT fixed here and worth a separate need: the MSVC pass has no defining-class
attribution, so a slot shared by a base and its derivatives gets one name per class
(`std::bad_alloc::vfunc_0` with `std::exception::vfunc_0` / `std::bad_array_new_length::vfunc_0`
as aliases; the canonical pick is arbitrary). The Itanium sibling already solves this; the MSVC
one does not. Its function symbols also still use the `vtable_<i>` stem and carry the same
naming defect, left alone here because correcting it edits `phases.toml`, leased this round.

## Reference

_none recorded_

## Instances

- `6547b4d50f4238b24302b588` (round 2, tester t-r2-6547b4d5)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 2 T_TRIAGE (captain): track quality -> TOOLING, touches kuna-decomp -> the inventory surface. Naming an executable range after a data symbol (std::bad_alloc::vftable_1) is wrong output with no judgment call in it, so it is a bug fix and needs no option. scope small. If the builder finds the alias comes from symbol application in the analysis tier and correcting it changes function DISCOVERY rather than labelling, STOP -- that is a different, gated change.
- closed: acceptance a-d6666365cbb6 now PASSES at fed78ec3c044
