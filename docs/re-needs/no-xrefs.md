---
need_id: no-xrefs
title: kuna cannot answer what references this address, function or string
track: tooling
status: open
severity: blocker
probe_id: p-7e35d5065714
acceptance_id: a-c9b199f423c1
hypothesis_status: upheld
credibility: 1.0
instances: 4
challenges: [6609e458cddae72ae250bf40, 67bd52114e1a16f76a1ad5bc, 61ffb07c33c5d46c8bcbfc1d]
rounds: [0]
first_seen_round: 0
attempts: 0
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src, decompiler/crates/kuna-analysis/src, tests/cli, docs/cli.md]
scope: large
regression_of: null
pr: null
closed_in_round: null
closing_pr: null
reject_reason: null
---

## Symptom

"What references this?" is the single most-used question in reverse
engineering, and kuna cannot answer it in any form. There is no `xrefs`, no
callers/callees query, no call graph, no string-reference lookup:

```
$ kuna xrefs <bin> --to 0x1030
kuna: unknown subcommand "xrefs"
usage: kuna <decompile|decompile-all|decompile-project|functions|test|catalog|modes|specs|fid> ...
$ echo $?
2
```

The nine subcommands are `decompile`, `decompile-all`, `decompile-project`,
`functions`, `test`, `catalog`, `modes`, `specs`, `fid`. Four of them are the
same operation (bytes to C text). None of them is a query.

This is a `blocker`, and it is the one need on this list that changes what an
agent *can* do rather than what it costs. The standard RE loop is: find the
interesting string or constant, ask who touches it, decompile only those
functions. Without step two, the loop degenerates into decompiling everything
and grepping the C -- which is why "no xrefs" and "whole-binary latency" are the
same complaint seen from two ends. An agent that hits this either writes its own
xref pass over `decompile-all --json` output (unreliable: the C text has already
lost the addresses of anything the optimizer folded) or leaves for the reference
stack, and leaving is the loudest signal this pipeline collects.

`kind: absence`. The diagnosis is not interesting -- the capability is simply not
there -- which is exactly the class `REPIPE_REFUTE_MODE=absence-skip` exists to
stop spending refuter runs on.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "probe_id": "p-7e35d5065714",
  "kind": "absence",
  "cmd": [
    "{{KUNA}}",
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x1030"
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
    "selector": "0x1030",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "eq": 2
    },
    "stdout_bytes": {
      "eq": 0
    },
    "stderr_matches": [
      "unknown subcommand \"xrefs\""
    ]
  },
  "notes": "Current bad behaviour: there is no cross-reference query of any kind. The subcommand does not exist, so kuna rejects it before loading the binary."
}
```

Replayed on the release build at `de9177fc` -- **PASS**, uniformly, and on three
dataset binaries too (see `## Instances`):

```
$ kuna xrefs decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64 --to 0x1030
kuna: unknown subcommand "xrefs"      # stderr
                                      # stdout: 0 bytes
exit=2
```

The probe is `kind: absence` and asserts the *absence itself* -- the argument
parser rejecting the subcommand -- rather than any behaviour of a binary. It is
therefore stable against every kuna change except the one this need asks for,
which is the property that makes it a usable regression witness.

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-c9b199f423c1",
  "kind": "absence",
  "cmd": [
    "{{KUNA}}",
    "xrefs",
    "{{BIN}}",
    "--to",
    "0x1030",
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
    "selector": "0x1030",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "count",
        "op": "gt",
        "value": 0
      },
      {
        "path": "xrefs[0].address_hex",
        "op": "exists"
      }
    ]
  },
  "notes": "Desired: a cross-reference query answering 'what references this address', in the address/address_hex record idiom kuna functions --json already uses."
}
```

Replayed on the same build -- **FAIL**: exit 2, no stdout, so neither
`exit_code.eq 0` nor `stdout_is_json` holds.

Target address `0x1030` on the fixture is the `__cxa_finalize` PLT stub. It has
at least one code reference, verified today, so `count > 0` is a real assertion
and not a tautology:

```
$ objdump -d decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64 | grep 'call.*1030'
    1102:  e8 29 ff ff ff    call   1030 <__cxa_finalize@plt>
```

The record idiom -- `count` plus an `xrefs` array whose elements carry
`address_hex` -- is deliberately the one `kuna functions --json` already uses,
so the surface stays self-consistent. The acceptance says nothing about
`--from`, string references, or a call graph: it is the minimum that makes the
capability exist and be testable. Widening it is what makes a `blocker` never
ship.

## Hypothesis

ADVISORY, not binding. kuna's analysis tier already recovers the
information -- it has to, to build call sites and resolve targets during
decompilation -- but nothing exposes it, because every command in the CLI is
shaped as "produce C text" and there is no query surface to hang it on.

If that is right, the work is a projection of existing analysis state, not new
analysis. If it is wrong -- if the reference set is built per-function and
discarded, and answering "who references X" means a whole-binary pass -- then
this is a much larger change, and that is the single fact that decides its
scope. `scope: large` is set on the pessimistic reading.

The builder is not bound to any of this.

## Refutation

Skipped by policy. `kind: absence` and `REPIPE_REFUTE_MODE=absence-skip`
is shipped ON: "there is no `xrefs` subcommand" has no interesting root cause,
and paying two agent-runs to confirm the obvious is waste. `hypothesis_status`
is recorded `upheld` on the absence itself, which is what the probe proves --
**not** on the mechanism sketched in `## Hypothesis`, which is untested.

The judgment that is NOT skipped is scope. `scope: large` routes this to a
`[PROPOSAL]` draft PR and the captain adjudicates before any implementation
work, which is where the "projection vs. new pass" question gets answered by
someone who has read the analysis tier.

## Reference

`decompiler xref_to` (DecLib over IDA Pro 9.2,
`~/.virtualenvs/decbench/bin/decompiler`):

```
$ decompiler xref_to --help
usage: decompiler xref_to [-h] [--decompile] [--id ID] [--binary BINARY]
                          [--backend {angr,binja,ghidra,ida}] [--json]
                          target

positional arguments:
  target                Function name or address (hex/decimal).
options:
  --decompile           Ask the backend to decompile first (picks up more refs on Ghidra).
  --json                Emit JSON output instead of text.
```

And it is not alone: the reference CLI ships `xref_to` (code **and** data
references), `xref_from` (callees), `get_callers` (call sites only),
`list_strings`, and `search` -- five query surfaces to kuna's zero, each with
`--json`, all against a persistent per-binary server so the second query is
free. That server is why the comparison is uncomfortable: kuna's cold load per
invocation makes a query surface expensive in exactly the workflow that needs it
most.

Scope discipline: this need asks for `xref_to` only. `xref_from`,
`get_callers`, `list_strings` and persistence are separate needs and must not be
merged into this one.

## Instances

Four reproductions on the release build at `de9177fc`. The subcommand is
rejected during argument parsing, before any binary is opened, so the result is
independent of the target -- these rows establish that it is the tool and not
the corpus.

| # | binary | challenge | result |
|---|---|---|---|
| 1 | `decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64` | in-repo fixture | exit 2, `kuna: unknown subcommand "xrefs"` |
| 2 | `challenges/6609e458cddae72ae250bf40/bin/puzzle.bin` | `6609e458cddae72ae250bf40` | exit 2, same |
| 3 | `challenges/67bd52114e1a16f76a1ad5bc/bin/crackme` | `67bd52114e1a16f76a1ad5bc` | exit 2, same |
| 4 | `challenges/61ffb07c33c5d46c8bcbfc1d/bin/hidden_password` | `61ffb07c33c5d46c8bcbfc1d` | exit 2, same |

The probe targets the in-repo fixture so the acceptance is vendorable into
`tests/cli/` and runs in CI with no dataset.

## Decision log

- r0 filed by hand while seeding the backlog. Probe and acceptance replayed at
  `de9177fc` before filing: probe PASS, acceptance FAIL.
- r0 severity `blocker`: this is a capability gap, not a cost or quality gap.
  It is the need most likely to appear in round 1 as a `gave_up_reason`.
- r0 `kind: absence`, so refutation is skipped by `REPIPE_REFUTE_MODE`
  (`absence-skip`, shipped on). `hypothesis_status: upheld` records the absence,
  not the mechanism.
- r0 `scope: large` -> `[PROPOSAL]` route, captain approves before
  implementation. The scope call rests on an unverified reading of the analysis
  tier; if the proposal shows the reference set already survives whole-binary
  analysis, re-scope to `small` and dispatch directly.
- r0 kept deliberately narrow: `xref_to` only. `xref_from`, `get_callers`,
  `list_strings` and a persistent server are separate needs.
