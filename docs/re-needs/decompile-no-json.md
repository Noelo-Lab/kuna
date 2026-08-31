---
need_id: decompile-no-json
title: kuna decompile has no --json: the one-function call is text-only and exits 2 on the flag
track: tooling
status: open
severity: major
probe_id: p-e9cc6a296ca5
acceptance_id: a-f618026c4005
hypothesis_status: inconclusive
credibility: 1.0
instances: 4
challenges: [6609e458cddae72ae250bf40, 67bd52114e1a16f76a1ad5bc, 61ffb07c33c5d46c8bcbfc1d]
rounds: [0]
first_seen_round: 0
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src, tests/cli, docs/cli.md]
scope: small
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

`kuna decompile <bin> <func>` is the call an agent makes most, and it is the
only one of kuna's four decompile commands with no machine-readable mode.
Passing `--json` does not degrade gracefully -- it is a hard argument error:

```
$ kuna decompile <fixture> _DT_INIT --json
error: unknown option --json
$ echo $?
2
```

`decompile-all --json` exists and emits everything an agent wants (`name`,
`address`, `size`, `code`, `error`, `line_mappings`, `variables`), but it
decompiles the **whole binary** to answer a question about one function. On the
dataset's mid-size ELFs that is the difference between one function and two
hundred.

So the agent's options today are: scrape C out of a text stream with no
delimiters and no error channel, or pay a whole-binary decompile per function.
Both are bad, and the second is the one that makes kuna feel slow -- the cost is
not kuna's decompiler, it is the missing flag.

The text mode also has no error channel: a function that fails to decompile is
reported in prose on a stream an agent is simultaneously trying to parse as C.
`decompile-all --json` has a per-record `error` field for exactly this.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "probe_id": "p-e9cc6a296ca5",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "_DT_INIT",
    "--json"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "binary_sha256": "1a592a85f424cc2db8953d5a38c86676bcee5e37b242b6fb6244a2c9fccfeeef",
    "binary_size": 14408,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "selector": "_DT_INIT",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 2
    },
    "stdout_bytes": {
      "eq": 0
    },
    "stderr_matches": [
      "error: unknown option --json"
    ]
  },
  "notes": "Current bad behaviour: the single-function decompile has no machine-readable mode at all; --json is rejected by the argument parser before any work happens."
}
```

Replayed on the release build at `de9177fc` -- **PASS**, and identically on
three dataset binaries (see `## Instances`):

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64 _DT_INIT --json
error: unknown option --json          # stderr
                                      # stdout: 0 bytes
exit=2
```

`stdout_bytes: {"eq": 0}` is in the probe deliberately: it asserts the flag is
rejected *before* any output, which is what distinguishes "unsupported flag"
from "flag ignored". A build that started ignoring `--json` and printing text
anyway would flip this probe to FAIL, and the need would be re-gated instead of
silently surviving on a stale assertion.

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-f618026c4005",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "_DT_INIT",
    "--json"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "binary_sha256": "1a592a85f424cc2db8953d5a38c86676bcee5e37b242b6fb6244a2c9fccfeeef",
    "binary_size": 14408,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "selector": "_DT_INIT",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "functions[0].name",
        "op": "eq",
        "value": "_DT_INIT"
      },
      {
        "path": "functions[0].code",
        "op": "exists"
      }
    ]
  },
  "notes": "Desired: decompile --json emits the decompile-all record shape for the one selected function. Same shape, one element -- not a second, incompatible JSON dialect."
}
```

Replayed on the same build -- **FAIL**: exit is 2 and stdout is empty, so
neither `exit_code.eq 0` nor `stdout_is_json` holds.

The acceptance pins the **record shape to `decompile-all --json`'s** -- a
`functions` array whose element carries `name` and `code` -- rather than
inventing a flat single-function object. That is a deliberate constraint on the
builder, and the reason for it is the promoted regression test: `tests/cli/`
will hold both this and any future `decompile-all` probe, and two shapes for the
same record is how a tool surface starts drifting. One element in the array is
the honest representation of "you asked for one function".

Verified today that the target shape is real -- `decompile-all --json` on this
fixture emits `functions[0].name == "_DT_INIT"` with a `code` string. So the
acceptance asks for something kuna already knows how to build.

## Hypothesis

ADVISORY, not binding. `decompile` predates the JSON surface and renders
straight to stdout, while `decompile-all` was written later around a serializable
record; the single-function path never got retrofitted onto that record.

The builder is not bound to this. Questions worth settling first:
1. whether `decompile --addr 0x...` and the `--slice`/`--language` variants all
   route through the same emit point, because a `--json` that works for one
   selector kind and not the others is worse than none;
2. what `--json` should do about the `--kassert` output, which is a second
   stream of information the text mode interleaves;
3. whether a one-function `--json` can reuse `decompile-all`'s serializer
   without also inheriting its per-function `decomp_dbg` spawn -- the cost
   question is real but belongs to a `perf` need, not to this one.

## Refutation

Not yet refuted. `kind: cli`, so absence-skip does not apply.

The claim a refuter should attack hardest: that `decompile-all --functions <one>`
is not already an adequate substitute. If it is -- if it decompiles exactly the
named function and nothing else -- then this need is `already-supported` and
belongs in `rejected/`, and that verdict is worth more than the feature. The
refuter must run it, not reason about it.

## Reference

`decompiler decompile --json` (DecLib over IDA Pro 9.2). Every one of the
reference CLI's subcommands takes `--json`; `decompile --help` lists it beside
`--id`, `--binary` and `--backend`:

```
$ decompiler decompile --help
  --json                Emit JSON output instead of text.
```

That uniformity is the point of comparison, not the specific schema: on the
reference stack an agent never has to ask *which* calls are machine-readable.
kuna answers "some of them", and the one an agent reaches for first is in the
`no` half.

## Instances

Four reproductions on the release build at `de9177fc`. Identical
`error: unknown option --json` on stderr, empty stdout, exit 2 in every case --
the flag is rejected during argument parsing, so the binary is never even
loaded, which is why this reproduces uniformly.

| # | binary | challenge | selector | result |
|---|---|---|---|---|
| 1 | `decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64` | in-repo fixture | `_DT_INIT` | exit 2, `error: unknown option --json` |
| 2 | `challenges/6609e458cddae72ae250bf40/bin/puzzle.bin` | `6609e458cddae72ae250bf40` | `main` | exit 2, same |
| 3 | `challenges/67bd52114e1a16f76a1ad5bc/bin/crackme` | `67bd52114e1a16f76a1ad5bc` | `main` | exit 2, same |
| 4 | `challenges/61ffb07c33c5d46c8bcbfc1d/bin/hidden_password` | `61ffb07c33c5d46c8bcbfc1d` | `main` | exit 2, same |

The probe targets the in-repo fixture so the acceptance is vendorable into
`tests/cli/`; the dataset rows are breadth evidence that this is not a property
of one binary.

## Decision log

- r0 filed by hand while seeding the backlog. Probe and acceptance replayed at
  `de9177fc` before filing: probe PASS, acceptance FAIL.
- r0 severity `major`: there is a workaround (`decompile-all --json`), so it is
  not a `blocker`, but the workaround costs a whole-binary decompile per
  question and is the single largest avoidable cost in an agent's loop.
- r0 acceptance deliberately constrains the record shape to `decompile-all`'s.
  If a builder argues for a flat object instead, that is a proposal-level
  decision and goes to the captain, not a unilateral shape change.
- r0 `covered_by_option: null`: the 127 catalog settables gate emitted C; none
  adds an output mode.
