---
need_id: zero-functions-exit-0
title: kuna reports total function-discovery failure as success: count 0, exit 0, silent stderr
track: tooling
status: open
severity: major
probe_id: p-8598042c5bf7
acceptance_id: a-99418905784a
hypothesis_status: inconclusive
credibility: 1.0
instances: 2
challenges: [64f1f7afd931496abf909525, 60be2a6033c5d410b8842c91]
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

On a section-header-stripped PIE ELF, `kuna functions --json` recovers
nothing and says so in the voice of a successful run: exit code 0, well-formed
JSON, `count: 0`, empty `functions`, and **not one byte on stderr**.

```
$ kuna functions challenges/64f1f7afd931496abf909525/bin/snake --json
{
  "binary": "/home/mahaloz/github/kuna-re-dataset/challenges/64f1f7afd931496abf909525/bin/snake",
  "count": 0,
  "functions": []
}
$ echo "exit=$?"      # exit=0, stderr empty
```

The binary is real and runnable: `ELF 64-bit LSB pie executable, x86-64,
statically linked, no section header`, 11364 bytes, mode 755.

An agent cannot distinguish this from "this file genuinely has no functions".
That is the whole severity: a silent zero is worse than an error, because the
loop above it -- a tester, a script, a captain reading `count` -- treats it as a
measurement rather than as a failure, and moves on. It is the difference between
kuna being wrong and kuna being *undetectably* wrong.

This need is about the report, not the recovery. Making kuna discover functions
in a section-header-stripped PIE is a separate, `loader`-track, `scope: large`
need; this one says the current answer must stop claiming success.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "probe_id": "p-8598042c5bf7",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
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
    "binary_rel": "challenges/64f1f7afd931496abf909525/bin/snake",
    "binary_sha256": "907b4dc30be0c0e53fda3ad1905ef3d804735a472f1c9484d3cfb5d3180e26c1",
    "binary_size": 11364,
    "binary_source": "dataset",
    "in_repo_path": null,
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "eq",
        "value": 0
      },
      {
        "path": "functions",
        "op": "len_eq",
        "value": 0
      }
    ],
    "stderr_absent": [
      "(?i)error",
      "(?i)warn"
    ]
  },
  "notes": "Current bad behaviour: total discovery failure is reported as an empty success. Exit 0, valid JSON, count 0, and nothing on stderr."
}
```

Replayed on the release build at `de9177fc` -- **PASS** on both witnesses:

```
$ kuna functions challenges/64f1f7afd931496abf909525/bin/snake --json
{"binary": "...snake", "count": 0, "functions": []}      exit=0  stderr=""

$ kuna functions challenges/60be2a6033c5d410b8842c91/bin/Pyaz.zip.__x/xvm --json
{"binary": "...xvm", "count": 0, "functions": []}        exit=0  stderr=""
```

The `stderr_absent` clause is load-bearing: it is what makes this probe assert
*silence*, not merely emptiness. A build that started warning on stderr while
still exiting 0 would flip this probe to FAIL and the need would correctly be
re-gated rather than quietly kept.

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-99418905784a",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "functions",
    "{{BIN}}",
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
    "binary_rel": "challenges/64f1f7afd931496abf909525/bin/snake",
    "binary_sha256": "907b4dc30be0c0e53fda3ad1905ef3d804735a472f1c9484d3cfb5d3180e26c1",
    "binary_size": 11364,
    "binary_source": "dataset",
    "in_repo_path": null,
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "exit_code": {
      "ne": 0
    },
    "stderr_matches": [
      "(?i)no functions"
    ]
  },
  "notes": "Desired: recovering zero functions from a non-empty executable is a failure and must exit non-zero with a diagnostic on stderr. Says nothing about improving discovery."
}
```

Replayed on the same build -- **FAIL**: exit is 0, so `exit_code.ne 0` is false
and stderr carries nothing to match.

Scope of the contract, stated so a builder does not over-deliver: the acceptance
demands a loud failure, **not** successful discovery. `exit_code != 0` plus a
stderr line containing "no functions" (case-insensitive) is the whole ask. It is
deliberately silent on which non-zero code, so the builder can fit kuna's
existing exit-code taxonomy instead of having one dictated by a probe.

Note this acceptance also passes trivially if someone later teaches kuna to find
functions in this file -- no, it does not: it would then find functions, exit 0,
and print nothing, so the acceptance would still FAIL. The two needs are
genuinely independent and neither closes the other.

## Hypothesis

ADVISORY, not binding. The `functions` command treats the discovery
result as data rather than as a status, so an empty result set takes the same
path as a populated one. Nothing consults "did the loader find anything to
analyse at all" before printing.

The builder is not bound to this. Two things a refuter or builder should settle
before writing code:
1. whether an empty result is ever legitimate for a file kuna agreed to load
   (an object file with no code? a resource-only PE?) -- if so, the loud failure
   must be conditioned on the binary having executable content, not on the count
   alone;
2. whether the same silence exists on `decompile-all` and `decompile-project`.
   If it does, fixing only `functions` leaves the hole open one command over,
   and the acceptance should be widened before dispatch rather than after.

## Refutation

Not yet refuted. `kind: cli`, so absence-skip does not apply and a
refuter is due before dispatch.

The specific claim a refuter must attack: that this is a *reporting* defect and
not a loader defect wearing a reporting costume. If the honest fix turns out to
require knowing why discovery failed -- and the only way to know is inside the
ELF loader -- then this need is mis-tracked as `tooling` and belongs in
`loader` with `scope: large`, which changes both its lease set and its route
(proposal, captain review).

## Reference

DecLib's `decompiler load` fails loudly when a backend cannot produce an
analysis, and `list_functions` on a loaded binary that yielded nothing is
distinguishable from a load that never happened, because the load step has its
own status.

The structural difference is not the message -- it is that kuna has one call
where the reference stack has two (`load`, then `list_functions`), so kuna has
nowhere to put a load-level failure and folds it into an empty answer. A builder
who wants the smallest correct change should note this: the missing thing is a
status, not a subcommand.

## Instances

Two reproductions, both from the design session, both re-run against the
release build at `de9177fc` immediately before filing.

| # | challenge | binary | `file` | result |
|---|---|---|---|---|
| 1 | `64f1f7afd931496abf909525` | `bin/snake` (11364 B, sha256 `907b4dc3...`) | ELF 64-bit LSB **pie executable**, x86-64, statically linked, **no section header** | `count: 0`, exit 0, stderr empty |
| 2 | `60be2a6033c5d410b8842c91` | `bin/Pyaz.zip.__x/xvm` (12520 B, sha256 `2078795d...`) | ELF 64-bit LSB **shared object**, x86-64, statically linked, **no section header** | `count: 0`, exit 0, stderr empty |

Both are stripped of section headers; both are x86-64, which is kuna's
best-supported architecture, so this is not an architecture gap. Witness 2 also
ships mode 644 -- one of the 54 dataset binaries without the exec bit, which the
arena builder chmods 0755; irrelevant to this probe, which only reads.

## Decision log

- r0 filed by hand while seeding the backlog. Both witnesses replayed at
  `de9177fc` before filing: probe PASS on both, acceptance FAIL on both.
- r0 severity `major`, not `blocker`: an agent that notices the zero can fall
  back to another tool, so it does not stop the work outright -- it silently
  corrupts the conclusion of anyone who does not notice. Escalate to `blocker`
  if a round-1 tester reports concluding "this binary has no code" from it.
- r0 track `tooling`, not `loader`, **on the acceptance as written**: the
  contract is an exit code and a stderr line, both of which live in `kuna-cli`.
  If the refuter overturns that (see `## Refutation`), re-track before dispatch.
- r0 the underlying discovery gap on section-header-stripped PIE ELFs is
  deliberately NOT filed here. It is a separate `loader` need.
