---
need_id: prototype-parser-rejects-valid
title: Prototype parser rejects a valid parameter named code
track: tooling
status: closed
severity: minor
probe_id: p-4a5243ba1588
acceptance_id: a-555cfdcc80e2
hypothesis_status: overturned
credibility: 0.7
instances: 1
challenges: [69761b7a39e9c4d85c2f9fc1]
rounds: [8]
first_seen_round: 8
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-console/src/grammar, decompiler/crates/kuna-cli/src/assertdecl.rs]
scope: small
regression_of: null
pr: null
closed_in_round: 10
closing_pr: null
reject_reason: null
---

## Symptom

Specify the interpreter's code pointer through a prototype assertion.

> **Prototype parser rejects a valid parameter named code** (minor, `69761b7a39e9c4d85c2f9fc1`)
> The assertion is rejected with a syntax-error caret at code. Changing only code to bytes makes the declaration apply. Separately, the known checker-discards-live-allocation issue was encountered and repaired with an allocator prototype; it is not re-filed.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/graphy-release.zip.__x/graphy",
    "binary_sha256": "1fb1f75b6a3939e3d80a25ca65aeb092d62a6968a53bd1db499a7cee864bed42",
    "binary_size": 40472,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x1005350",
    "--addr",
    "--assert",
    "prototype 0x1005350 unsigned long vm(unsigned char *code,unsigned int index,void *ctx)",
    "--json"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "rejected"
      }
    ],
    "stderr_matches": [
      "Syntax error"
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-555cfdcc80e2",
  "kind": "cli",
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "authenticate",
    "selector_kind": "name"
  },
  "timeout_s": 120,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "authenticate",
    "--json",
    "--assert",
    "prototype authenticate unsigned long vm(unsigned char *code,unsigned int index,void *ctx)",
    "--assert",
    "prototype read int read(unsigned char code)"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "assertions",
        "op": "len_eq",
        "value": 2
      },
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "assertions[1].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "unsigned char *code"
      }
    ],
    "stderr_absent": [
      "Syntax error",
      "Multiple type specifiers",
      "Bad C syntax"
    ]
  },
  "notes": "Desired: a parameter may be named after a type. `code` is a core type every compiler spec registers, so the lexer hands it over as TYPE_NAME; both spellings were rejected -- `unsigned char *code` as a Syntax error on the name, `unsigned char code` as Multiple type specifiers. Retargeted onto the in-repo fauxware fixture so it runs with no dataset; both measured rejected on 7e77fb8c."
}
```

## Hypothesis

**Advisory — the builder is not bound by this.** In the sibling campaign 3 of 8 filed diagnoses were overturned while the symptom stood in all 8.

- The parser treats code as an internal reserved word.

## Refutation

_not yet refuted_

## Reference

_none recorded_

## Instances

- `69761b7a39e9c4d85c2f9fc1` (round 8, tester t-r8-69761b7a)

## Decision log

- filed by cluster.py from 1 observation(s)
- round 8 TRIAGE (captain): RETOUCHED from the coarse [kuna-cli] to the actual parser. The 'Bad C syntax' answer is raised in kuna-console/src/ifacedecomp.rs and the C dialect it enforces is kuna-console/src/grammar/; kuna-cli/src/assertdecl.rs only forms the directive line. track stays tooling: accepting a valid parameter name corrects a parser that is wrong on its own terms, so it is a strict fix with no option.
- closed: acceptance a-555cfdcc80e2 now PASSES at 7e77fb8c9dca
- round 10 REFUTER: hypothesis **overturned** (was inconclusive). Mechanism overturned, symptom stands. `code` is not a reserved word: Architecture::setup_types registers it as a core data-type (set_core_type("code", 1, TYPE_CODE, false)), and CParse::lookup_identifier classifies every spelling findByName answers as TYPE_NAME. The class is therefore every interned type name -- the core types, any tag/typedef declared earlier in the run, and on a -g binary every DWARF type name. The symptom also has two halves with two diagnostics, not one: `unsigned char *code` dies in direct_declarator (Syntax error) and `unsigned char code` dies in the greedy specifier run (Multiple type specifiers).
