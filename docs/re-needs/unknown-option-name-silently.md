---
need_id: unknown-option-name-silently
title: An unrecognized --option NAME is silently accepted and changes nothing
track: tooling
status: open
severity: major
probe_id: p-8013d19d67fd
acceptance_id: a-4c01c2fcb1ed
hypothesis_status: inconclusive
credibility: 0.9
instances: 1
challenges: [605443e333c5d42c3d016f59]
rounds: [4]
first_seen_round: 4
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile.rs, decompiler/crates/kuna-cli/src/decompile_all.rs, decompiler/crates/kuna-console/src/ifacedecomp.rs]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Turn a decision off for one run and see whether it was the cause.

> **A misspelled option name is accepted with no diagnostic** (major, `605443e333c5d42c3d016f59`)
> kuna decompile KataVM_L1 0x12d0 --addr --option zzzznotanoption off exits 0, writes nothing
> to stderr, and prints output byte-identical to the run with no --option at all. The same is
> true of LOWEREDSWITCH, lowered_switch and loweredswitc. Only the exactly spelled
> loweredswitch does anything, and nothing distinguishes the two cases from outside.

Measured matrix at `68d27c99`, same binary and address, sha256 of stdout:

| `--option` argument | rc | stderr | stdout sha256 |
|---|---|---|---|
| (none) | 0 | empty | `af7dfc7999d9` |
| `loweredswitch off` | 0 | empty | `c4a4013bf5b4` |
| `zzzznotanoption off` | 0 | empty | `af7dfc7999d9` |
| `LOWEREDSWITCH off` | 0 | empty | `af7dfc7999d9` |

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "target": {
    "binary_rel": "bin/KataVM_Level_1.7z.__x/KataVM_Level_1/KataVM_L1",
    "binary_sha256": "95c300aedc728b643bf97c39b5e8db88e9ddc40bf4cf337cd6c777929684a5f9",
    "binary_size": 28682,
    "binary_source": "dataset"
  },
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "0x12d0",
    "--addr",
    "--option",
    "zzzznotanoption",
    "off"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "(?i)zzzznotanoption"
    ],
    "stdout_matches": [
      "(?s)\\bswitch\\s*\\("
    ]
  }
}
```

## Acceptance

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "main",
    "--option",
    "zzzznotanoption",
    "off"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 60,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "main",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "ne": 0
    },
    "stderr_matches": [
      "zzzznotanoption"
    ],
    "stdout_absent": [
      "(?s)\\bmain\\s*\\("
    ]
  },
  "notes": "Vendored stand-in: the option NAME is judged in the parser, before anything is loaded, so any real binary asks the same question. The dataset original (KataVM_L1 0x12d0 --addr) stays the witness in Reproduction and passes the same assertions; this target is in-repo so the probe runs where there is no dataset."
}
```

## Hypothesis

**Advisory - the builder is not bound by this.** In the sibling campaign 3 of 8 filed
diagnoses were overturned while the symptom stood in all 8.

- The engine already rejects the name; the CLI loses the rejection. `kuna` lowers each
  `--option NAME VALUE` into an `option NAME VALUE` script line for `decomp_dbg`
  (`decompile.rs:161`), and the console's handler raises `IfaceError::execution("Unknown
  option")` when the name is in neither `KUNA_OPTION_NAMES` nor the ElementId registry
  (`ifacedecomp.rs:873`). But `decomp_dbg` prints that class of failure to **stdout** as
  `Execution error: ...` and still exits 0 - directly observed on an unrelated bad command:
  a script of `restore-file <path>` + `option zzzznotanoption off` printed `ERROR: Invalid
  command` and `Execution error: No load image present` on stdout with rc=0. So the CLI has
  no failing exit status to notice and no stderr to forward.
- Therefore the fix is most likely on the CLI side of that seam - either validate the name
  against the catalog `kuna` already ships (`catalog.rs` knows every settable name) before
  spawning, or make the driver treat a console `Execution error:` line as fatal. The first is
  cheaper and gives a better message; the second also catches every other silently swallowed
  script error, which may be a larger blast radius than this need needs.
- There are seven `--option` parse sites (`decompile.rs`, `decompile_all.rs`,
  `decompile_project.rs`, `decompile_graph.rs`, `disassemble.rs`, `xrefs.rs`, `strings.rs`);
  a shared validator covers all of them, and only `decompile` is probed here.
- Do not regress the working case: `--option loweredswitch off` must keep changing the
  output, and `--option setlanguage`/driver-injected names must keep working.

## Refutation

_not yet refuted_

## Reference

- `ida-decompile` — not applicable; this is a kuna-only CLI contract.

## Instances

- `605443e333c5d42c3d016f59` (round 4, captain-verified)

## Decision log

- filed by the captain at round 4 B_DRAIN, not by cluster.py. Carried in the round notes for
  four ticks as "file this as a tooling need" and verified before filing: the matrix above was
  run directly, and the console-side mechanism was read at `ifacedecomp.rs:855-885` and
  confirmed by a `decomp_dbg` script showing `Execution error:` on stdout with rc=0.
- Why it matters to this loop and not only to users: every tester sentence of the form
  "option X did not remove the defect" is worthless unless X is spelled exactly as
  `kuna catalog` spells it, and one such sentence is already in the round-4 record
  (`unsigned-byte-vm-selector`). This makes every future ablation honest.
- The CLI-tier change recipe applies: no new option, no stage XML, no catalog counts.
- round 4 REFUTER: hypothesis **inconclusive**. Captain-run, not a spawned refuter. SYMPTOM PROVEN, exactly: at 68d27c99 on KataVM_L1 0x12d0 --addr, the four spellings loweredswitch / LOWEREDSWITCH / lowered_switch / zzzznotanoption all exit 0 with EMPTY stderr, and only the exact spelling changes stdout (sha c4a4013bf5b4 vs af7dfc7999d9 for the other three AND for the no-option default). MECHANISM HALF-PROVEN. Verified by reading: kuna lowers each --option into an 'option NAME VALUE' script line for decomp_dbg (decompile.rs:161), and the console handler raises IfaceError::execution Unknown option when the name is in neither KUNA_OPTION_NAMES nor the ElementId registry (ifacedecomp.rs:869-874). Verified by running: decomp_dbg fed 'restore-file <path>' + 'option zzzznotanoption off' printed ERROR: Invalid command and Execution error: No load image present on STDOUT and exited 0. NOT verified: that the Unknown-option error specifically behaves the same once an image IS loaded, because the no-load-image error preempted it in that script. So the causal chain is one observation short, and a builder should confirm it before choosing between the two candidate fixes. Either fix closes the acceptance regardless -- validating the name against the catalog kuna already ships does not depend on the swallow being the cause.
