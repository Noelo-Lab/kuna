---
need_id: decompile-rejects-subcommand-help
title: decompile rejects subcommand --help
track: tooling
status: open
severity: minor
probe_id: p-24c7355a0b22
acceptance_id: a-a47f53062e49
hypothesis_status: inconclusive
credibility: 1.0
instances: 2
challenges: [67b480dee36dd9b0e79b30c8, 6989ed7dfb46458f1ef6cee4]
rounds: [4]
first_seen_round: 4
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src/decompile.rs, decompiler/crates/kuna-cli/src/main.rs, decompiler/crates/kuna-cli/src/specs.rs, docs/cli.md, tests/cli]
scope: small
regression_of: null
pr: "448"
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

Discover decompile flags through subcommand help.

> **decompile rejects subcommand --help** (minor, `67b480dee36dd9b0e79b30c8`)
> kuna decompile --help exits 2 with error: unknown option --help.

> **decompile --help rejects the help flag** (minor, `6989ed7dfb46458f1ef6cee4`)
> Exited 2 with error: unknown option --help. Other inspected subcommands provide usage.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "timeout_s": 60,
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "--help"
  ],
  "expect": {
    "exit_code": {
      "eq": 2
    },
    "stderr_matches": [
      "unknown option --help"
    ]
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
    "--help"
  ],
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stderr_absent": [
      "unknown option --help"
    ]
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

- `67b480dee36dd9b0e79b30c8` (round 4, tester t-r4-67b480de)
- `6989ed7dfb46458f1ef6cee4` (round 4, tester t-r4-6989ed7d)

## Decision log

- filed by cluster.py from 2 observation(s)
- round 4 REFUTER: hypothesis **inconclusive**. REFUTER RAN; VERDICT IS inconclusive FOR THE ONLY HONEST REASON -- the tester offered NO hypothesis ('_none offered_'), so there was no diagnosis to uphold or overturn. Do not read this as 'could not decide'. The symptom is CONFIRMED and the mechanism is located; what follows is what a builder needs and none of it is a guess (sha 6188e204). SYMPTOM: `kuna decompile --help` prints 'error: unknown option --help' to stderr and exits 2. THE SET IS THREE, NOT ONE. I ran --help against all 14 subcommands: decompile, catalog and test reject it (rc 2); decompile-all, decompile-project, decompile-graph, disassemble, xrefs, strings, functions, docs, unpack and fid all print usage. So the need's title understates it -- a builder who fixes only `decompile` leaves two subcommands with the same defect one command away, and a round-5 tester will file them again. THE LINE: decompile.rs:1105-1108, the `s if s.starts_with("--")` catch-all at the end of the arg loop. There is no '-h' | '--help' arm anywhere in decompile.rs, catalog.rs or test.rs, while the other eleven subcommands each have one (decompile_all.rs:2107, decompile_project.rs:65, decompile_graph.rs:74, disassemble.rs:991, xrefs.rs:514, strings.rs:418, docs.rs:90, unpack.rs:180, fid.rs:46). This is an omission in three parsers, not a dispatch bug: main.rs:80 handles top-level -h/--help/help fine, and `kuna decompile` with no args already exits 2 with a proper 'requires <binary> and <func>' message, so only the --help path is missing. THE TEXT ALREADY EXISTS, so this is not a docs-writing job: main.rs:93 usage() already carries a per-subcommand line for all three ('kuna decompile <binary> <func> [--addr] [--json] ...', 'kuna test [--all|...]', 'kuna catalog [--json|--markdown|--check] ...'). The eleven working subcommands each print their own richer multi-line usage, so the design choice a builder faces is whether to extract those three lines or write three fuller blocks in the style of xrefs.rs/strings.rs; the second matches the surrounding code and is what a tester actually wanted (the observation says 'Discover decompile flags through subcommand help', and the one-liner does not mention --kassert or --assert semantics). ACCEPTANCE SHAPE: the probe wants rc 0 and no 'unknown option --help' on stderr. usage() writes to STDERR and that still satisfies it, so a builder need not move the stream -- but note the acceptance does NOT check stdout, so 'help goes to stdout' is an open style question the probe will not answer for them. NO OVERTURN RISK I CAN FIND: the arm is additive, the match's catch-all is already last so an explicit arm wins, and --help cannot collide with a value position because every value-taking flag consumes its argument through take_value() before the loop re-dispatches.
- round 4 BUILDER (b-r4-decompile-reject): symptom CONFIRMED, refuter's located mechanism CONFIRMED, its enumeration INCOMPLETE. The refuter swept 14 subcommands and found three rejecting `--help`; sweeping all 16 finds a FOURTH, `kuna specs`, which forwards its argv to `slacomp` and so answered `Unknown option: --help` with exit 1 rather than exit 2 -- a different message and a different code, which is why a grep for the filed stderr string missed it. Fixed all four by adding the `-h | --help` arm the other twelve already had (decompile.rs, main.rs cmd_test/cmd_catalog, specs.rs ahead of the passthrough), each printing its own multi-line usage block rather than reusing the one-liner in main.rs `usage()` -- the observation asked to "discover decompile flags", and the one-liner names no `--assert` vocabulary, no `--define-function` contract and no JSON shape. `-h` was equally broken and is equally fixed: it does not start with `--`, so it was being read as a positional (`decompile requires <binary> and <func>`, `unexpected argument "-h"`). Acceptance a-a47f53062e49 PASSES. Second, unfiled defect closed on the way: `verify.vendorable()` admits a probe that needs no binary, but `clitests.run_one()` re-derived a stricter rule and refused one, so the promoted probe FAILED `make test-cli` the moment it landed -- this need's acceptance is the first binary-less probe the corpus has ever held. clitests now calls `vendorable()` instead of duplicating it.
