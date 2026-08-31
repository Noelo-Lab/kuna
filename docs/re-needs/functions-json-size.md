---
need_id: functions-json-size
title: kuna functions --json omits size, so the cheap inventory call cannot rank functions
track: tooling
status: closed
severity: minor
probe_id: p-d8f69800917d
acceptance_id: a-ddc5496835ba
hypothesis_status: overturned
credibility: 1.0
instances: 4
challenges: [6609e458cddae72ae250bf40, 67bd52114e1a16f76a1ad5bc, 61ffb07c33c5d46c8bcbfc1d]
rounds: [0]
first_seen_round: 0
attempts: 1
covered_by_option: null
touches: [decompiler/crates/kuna-cli/src, tests/cli, docs/cli.md]
scope: small
regression_of: null
pr: PENDING
closed_in_round: 1
closing_pr: null
reject_reason: null
---

## Symptom

`kuna functions <bin> --json` is the one cheap, whole-binary call kuna
offers: it loads once and answers "what is in here". Its records carry only
`name`, `address`, `address_hex` and `aliases` -- there is no extent. An agent
triaging a 250-function binary therefore has no way to order its work except by
address, and the standard first move ("decompile the three biggest functions")
costs a full `decompile-all` instead of one inventory call.

The field is not missing from kuna's model. `kuna decompile-all --json` emits
`size` on every record, from the same `FuncResult`. Only the cheap call drops it.

```
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64 --json
{
  "binary": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
  "count": 34,
  "functions": [
    {
      "name": "_DT_INIT",
      "address": 4096,
      "address_hex": "0x1000",
      "aliases": []
    },
```

This is the smallest real need in the corpus and is the designated first
live-builder target: one crate, one struct, no emitted C, no phases.toml row,
and a vendorable acceptance that promotes straight into `tests/cli/`.

## Reproduction

```json
{
  "schema": "re-probe/1",
  "probe_id": "p-d8f69800917d",
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "binary_sha256": "1a592a85f424cc2db8953d5a38c86676bcee5e37b242b6fb6244a2c9fccfeeef",
    "binary_size": 14408,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
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
        "op": "gt",
        "value": 0
      },
      {
        "path": "functions[0].size",
        "op": "absent"
      }
    ]
  },
  "notes": "Current bad behaviour: the function inventory carries no extent, so a caller cannot tell a 3-byte thunk from a 4 KB main without decompiling every function."
}
```

Replayed on the release build at `de9177fc` (`decompiler/target/release/kuna`,
`SLEIGHHOME=/home/mahaloz/github/kuna/specs`) -- **PASS**:

```
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64 --json \
    | python3 -c "import sys,json;d=json.load(sys.stdin);print(sorted(d['functions'][0]))"
['address', 'address_hex', 'aliases', 'name']
```

Same key set on three dataset binaries -- see `## Instances`.

## Acceptance

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-ddc5496835ba",
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
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "binary_sha256": "1a592a85f424cc2db8953d5a38c86676bcee5e37b242b6fb6244a2c9fccfeeef",
    "binary_size": 14408,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64",
    "selector": null,
    "selector_kind": "none"
  },
  "expect": {
    "json": [
      {
        "path": "functions[0].size",
        "op": "exists"
      }
    ]
  },
  "notes": "Desired: every functions --json record carries size, the field decompile-all --json already emits. Vendorable (in-repo fixture) so this promotes into tests/cli/ verbatim."
}
```

Replayed on the same build -- **FAIL**, which is what admits the need:
`functions[0].size` does not exist, so the `exists` predicate is false.

The clause is deliberately the single field. `size` is the only thing being
asked for; pinning exit code or record count as well would make the acceptance
fail for reasons that are not this need, and it is promoted verbatim into
`tests/cli/` where over-specification becomes a brittle CI guard. The target is
an in-repo fixture (`binary_source: in-repo`, `in_repo_path` set), so the
promoted test runs in CI with no dataset present.

## Hypothesis

ADVISORY, not binding. The `functions` command builds its JSON records
by hand rather than reusing the `decompile-all` record type, and `size` was
simply not copied across when the inventory command was added. Expect a struct
in `kuna-cli` with four fields where the decompile-all one has five.

The builder is not bound to this. The only contract is that the acceptance
flips to PASS with the four gates and `kuna catalog --check` green.

Two questions the builder must answer rather than assume:
1. what `size` means for a function whose body kuna could not fully recover --
   `decompile-all` reports `0` for `_DT_INIT` on this fixture, so `0` is an
   established, honest answer and must stay expressible;
2. whether `size` is the extent of the entry block or of the whole recovered
   body. `decompile-all --json` already answers this; match it exactly rather
   than inventing a second meaning for the same field name.

## Refutation

Not yet refuted. `kind: cli`, not `absence`, so
`REPIPE_REFUTE_MODE=absence-skip` does not exempt it -- a refuter is due before
dispatch. The cheap check a refuter owes: confirm that the `size` on
`decompile-all --json` records is computed at inventory time and not as a
by-product of decompiling the function, because if it is the latter this stops
being a one-field copy and becomes a cost question.

## Reference

`decompiler list_functions --json` (DecLib over IDA Pro 9.2,
`~/.virtualenvs/decbench/bin/decompiler`) emits `{"addr", "size", "name"}` per
record -- `declib/cli/decompiler_cli.py:1036-1037`:

```python
size = getattr(func, "size", 0) or 0
entries.append({"addr": addr, "size": int(size), "name": name})
```

Its text mode prints the same three columns. So the reference an RE agent
reaches for treats extent as part of the inventory, not as something you pay a
decompile for.

kuna already agrees with that everywhere except here:

```
$ kuna decompile-all <fixture> --json | jq '.functions[0] | keys'
["address","address_hex","aliases","code","error","line_mappings","name","size","variables"]
```

## Instances

Four reproductions, one in-repo and three dataset, all on the release
build at `de9177fc`. Every one returns the same four-key record.

| # | binary | challenge | keys in `functions[0]` |
|---|---|---|---|
| 1 | `decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64` (34 fns) | in-repo fixture | `address, address_hex, aliases, name` |
| 2 | `challenges/6609e458cddae72ae250bf40/bin/puzzle.bin` (6 fns) | `6609e458cddae72ae250bf40` | same |
| 3 | `challenges/67bd52114e1a16f76a1ad5bc/bin/crackme` (22 fns) | `67bd52114e1a16f76a1ad5bc` | same |
| 4 | `challenges/61ffb07c33c5d46c8bcbfc1d/bin/hidden_password` (16 fns) | `61ffb07c33c5d46c8bcbfc1d` | same |

The probe targets the in-repo fixture rather than a dataset binary so the
acceptance is vendorable; the dataset rows are the breadth evidence.

## Decision log

- r0 filed by hand while seeding the backlog, not by a tester. Probe and
  acceptance both replayed against `decompiler/target/release/kuna` at
  `de9177fc` before filing: probe PASS, acceptance FAIL.
- r0 severity `minor` on impact (nothing is wrong, something is absent) but
  designated the **first live-builder target** on tractability: smallest real
  change in the corpus, Track T, no counters, vendorable acceptance. Escalate to
  `major` if a round-1 tester reports it as a give-up cause rather than a
  nuisance.
- r0 `covered_by_option: null` -- `kuna catalog` options gate emitted C only;
  none of the 127 settables can add a field to `functions --json`.
- REGRESSED: acceptance a-ddc5496835ba fails again at deadbeef
- r1 builder: acceptance a-ddc5496835ba flips to PASS. `functions[0].size` = 27.
- r1 **hypothesis OVERTURNED.** The filed diagnosis -- "`size` was simply not
  copied across ... a struct in kuna-cli with four fields where the decompile-all
  one has five" -- is wrong, and the symptom's premise with it. `decompile-all
  --json` does emit `size`, but its VALUE is `0` on every record of every binary
  (34/34 on this fixture; 0 nonzero across every fixture in the tree), because
  `FuncResult.size` read `Funcdata::get_size()`, the caller's *requested* flow
  bound, which `decompile_targets` always passes as `0` ("UNBOUNDED"). Copying
  the field would have satisfied the acceptance's `exists` clause with a column
  of zeroes and closed nothing. The symptom stands; the diagnosis did not.
- r1 the real gap was therefore broader than filed: kuna had no function-extent
  value on ANY surface. Closed by reconstructing it at inventory time as the
  address-contiguous clip `[entry, min(next_entry, end_of_CODE_section))`
  (`kuna-console/src/funcextent.rs`) -- the model `analyzers/fid/extent.rs` and
  `noreturn_disc` already use -- and reporting the one number on `functions`,
  `decompile-all`, `decompile-project` and the wasm inventory alike.
- r1 the Refutation section's cheap check, answered: `decompile-all`'s `size` is
  NOT computed at inventory time, and it is not a by-product of decompiling
  either -- it is a constructor argument that no CLI surface ever sets. The cost
  question it anticipated was real but resolved the other way: the clip needs no
  decode at all, and measures at -0.58%..+0.21% (interleaved, 9 reps) on the
  inventory path.
- r1 an upper bound, deliberately: 0 undershoots / 1428 functions with ELF
  `st_size` ground truth, median overshoot +8B (alignment padding). `st_size`
  itself was rejected as the source -- this fixture is stripped, so it would
  have produced an all-zero column on the very binary the acceptance targets.
