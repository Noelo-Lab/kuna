---
need_id: no-cli-rename-or-prototype-override
title: an agent cannot rename or retype anything, or force a prototype
track: tooling
status: closed
severity: major
acceptance_id: a-a58fc408288b
hypothesis_status: overturned
credibility: 1.0
instances: 1
rounds: [2]
first_seen_round: 2
attempts: 1
touches: [decompiler/crates/kuna-cli/src, decompiler/crates/kuna-console/src/assertions.rs]
scope: large
pr: "389"
closed_in_round: 2
closing_pr: "389"
---

## Symptom

`rename`, `retype`, `map param`, `map return`, `override prototype` and `parse line extern ...` are all functional in the console and none is reachable from `kuna`. Renaming a FUNCTION is absent even there (`rename` resolves only against the current function's local scope). `kuna decompile --kassert` exists and routes to `IfcKunaAssert`, but only its `Rename` (naming-policy) and `Option` arms do real work -- `prototype-source` and `type-propagation` return engine_unavailable. For an agent, a rename that does not persist is the difference between reading a decompilation once and actually working through one.

## Reproduction

Every named CLI flag is rejected with `error: unknown option`, verified on the round-1 merge build; the stubs are verified by driving `decomp_dbg` directly and reading back `engine integration not yet ported`.

## Acceptance

Asserts the DESIRED behaviour: an agent states three facts kuna cannot be told
today -- the function's prototype, a local's type and a local's name -- on one
`kuna decompile --json` invocation, and all three land in the emitted C while the
run reports each one's fate as machine-readable JSON. FAILS at e3db5512: every
`--assert` is rejected with `error: unknown option`.

```json
{
  "schema": "re-probe/1",
  "probe_id": "a-a58fc408288b",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "authenticate",
    "--json",
    "--assert",
    "prototype authenticate int4 authenticate(char *user,char *pass)",
    "--assert",
    "type v2 char[16]",
    "--assert",
    "name v2 credbuf"
  ],
  "cwd": "{{WORK}}",
  "env": {
    "SLEIGHHOME": "{{SPECS}}"
  },
  "stdin": null,
  "timeout_s": 120,
  "repeat": 1,
  "target": {
    "binary_rel": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "binary_sha256": "c2d90645a45e99221593547e55c601a901b80f807ae96f94c60a7661df0b3e0b",
    "binary_size": 8776,
    "binary_source": "in-repo",
    "in_repo_path": "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
    "selector": "authenticate",
    "selector_kind": "name"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "assertions",
        "op": "len_eq",
        "value": 3
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
        "path": "assertions[2].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "authenticate(char *user,char *pass)"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "char credbuf [16];"
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "char v2 [8]"
      }
    ]
  },
  "notes": "Desired: `--assert <directive>|@FILE` (repeatable), the one override plane. Three directives -- `prototype` (a C signature, lowering to `parse line extern`), `type` (`retype`) and `name` (`rename`) -- land in the emitted C, and each one's fate is reported in a machine-readable `assertions[]`. The asserted C was measured end-to-end through decomp_dbg at e3db5512. In-repo fixture: promotes verbatim."
}
```

**Closed WITHOUT an acceptance flip.** Filed by hand, so this record carries no `probe_id`/`acceptance_id` and the gate never closed it -- a human did. The design says only the acceptance probe may close a need; this one did not go through that. The durable evidence that the capability works is:
- `closed by #389 (--assert name / prototype)`

Recorded rather than back-filled: a probe retro-fitted now would assert TODAY's behaviour, not the behaviour at closing time, and would look like gate evidence while being nothing of the kind.

## Hypothesis

ADVISORY. The cheap half is exposure, not implementation: most of these commands already work and only lack a path from the `kuna` binary. The expensive half is the stubs. A builder should measure which is which before choosing a design, and should NOT assume a `kuna console` passthrough is the right shape -- a scriptable console is a different product from a set of flags an agent can compose.

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should confirm the demand.
- round 2 T_TRIAGE (captain): track tooling / touches [kuna-cli/src, kuna-console/src/kuna_console.rs] / scope large CONFIRMED. New this tick: keyboard-callback-uses-undefined is a tester-filed demand witness for it -- an agent that could state a callback's prototype would not need kuna to infer R9D/R8D at all. Still waits behind the function-boundary proposal for the delivery vehicle.
- round 2 wave 9 B_PLAN (captain): DISPATCHED as the override family's one design, because the delivery vehicle this need was waiting behind now EXISTS -- no-cli-function-boundary-override closed via #374 (`kuna decompile --define-function`), an agent-supplied fact injected from the `kuna` binary. Three standing instructions for the builder. (1) Design ONE override plane that covers this need AND its two siblings (no-cli-data-code-override, no-cli-structuring-override); the captain will not approve three separate large designs, and a unified proposal lets the siblings collapse into it rather than each burning a builder. (2) Answer the Hypothesis's cheap/expensive question as a table: for every command named in the Symptom, whether it already works in the console and only lacks a `kuna` path, or is an `engine_unavailable` stub. (3) The proposal MUST define a concrete `acceptance` probe for this need. It has none today (`verify --acceptance-suite` reports it unrunnable), so nothing built on it could ever be closed by B_DONE, and a design that leaves it unrunnable is not approvable.
- round 2 B_PLAN (builder): filed the acceptance probe this need lacked, `a-a58fc408288b`. Its target C was measured end-to-end through `decomp_dbg` before the probe was written (`parse line extern` + `retype` + `rename` on the in-repo `fauxware` fixture), so the probe asserts an output the engine demonstrably already produces -- what is missing is only the path from the `kuna` binary.
- round 2 B_DONE (builder): SHIPPED as `kuna --assert <directive> | @FILE`, Stage A of the
  approved proposal. Nine directives -- function, typedef, prototype, data, param, return,
  comment, name, type -- each lowering to the console command that already implements it,
  each with a test that asserts the emitted C CHANGED rather than that the command returned
  Ok. Acceptance `a-a58fc408288b` PASS; promoted verbatim to
  `tests/cli/no-cli-rename-or-prototype-override.json`.
- round 2 B_DONE: `hypothesis_status` reconciled to `overturned`, matching this need's
  `record.json` and the proposal's own verdict (the frontmatter still said `upheld`). The
  cheap half is much LARGER than filed -- 11 console commands were measured reaching emitted
  C -- and the expensive half is not "the stubs": it is two shipped commands that lied.
  `map return` is fixed here (it parked output-only `PrototypePieces`, and
  `ParamListStandardOut::assignMap` dereferences `outtype` unconditionally, so the command
  aborted the process the moment its function was decompiled).
- round 2 B_DONE: **Stage-A residue, three items, none of them blocking.**
  (1) `override prototype <addr> <decl>` is still accepted-and-inert, re-measured on this
  branch (`strcmp` at `0x400689`, 2 params -> 3: no change). The store and the consume are
  BOTH present and look correct -- `IfcProtooverride` stashes into `pending_proto_overrides`,
  `IfcDecompile` re-seeds it, `build_and_follow_flow_with_override_and_protos` installs it on
  the fresh `Funcdata`, and `ArchFlowEnv::build_override_proto` (decompile_drive.rs:325)
  builds a real `FuncProto` from the pieces -- so the proposal's "one of the two is stale" is
  itself wrong and the defect is somewhere else in that chain. Time-boxed per the captain's
  dispatch: the `prototype` directive lowers to `parse line extern` (measured working, and
  what the acceptance asserts), so per-CALL-SITE prototype override is the residue, not the
  function's own prototype. Worth its own need.
  (2) `label` is not shipped: `map label` has no observable effect on emitted C on any
  fixture tried, so it cannot satisfy the standing "the test asserts the C changed" rule.
  (3) The two CLI-level `--kassert` defects this need's proposal recorded (the `naming-policy`
  arm emitted before the first `decompile` and therefore inert; `--kassert` rejected outright
  with `--json`) are NOT fixed. `--assert` supersedes both -- it is the intent-keyed writer,
  it reports into `assertions[]`, and it works with `--json` -- so fixing the raw phase-keyed
  writer is optional cleanup rather than a gap an agent can feel.
- round 2 B_DONE: the P4/P5/P9 `exposure` prose in `phases.toml` is DEFERRED, not skipped by
  choice: `file:phases.toml` was held by `b-r2-ppc64-localentry` for this whole wave. It is
  prose, no gate reads it, and this PR adds no settable row (so no catalog counters and no
  DIV row either).
- closed: acceptance a-a58fc408288b now PASSES at c40e02fa7931
- round 2 B_DONE (captain): CLOSED, mechanically. `needs apply-acceptance` on
  /tmp/b_verify_c40e02fa.acceptance.json (sha c40e02fa, all four gates + catalog + clitests
  green) flipped it; acceptance a-a58fc408288b PASSES (7 clauses, 391 ms, one run, not flaky)
  where it FAILED at e3db5512. pr/closing_pr 389 (merged 19:07:28Z, mergeCommit ed353fb2) set
  by hand because that suite covered TWO merges and apply-acceptance takes one --pr.
  covered_by_option stays null -- correct, this is tooling track and ships no settable row.
  Promotion re-run and byte-identical: tests/cli/no-cli-rename-or-prototype-override.json was
  already vendored by the builder and was 1 of the 21/21 clitests in the gate run.
  `hypothesis_status: overturned` is EARNED, not carried from prose: the merged diff itself
  shows the expensive half was not "the stubs" but a shipped command that aborted the process
  -- p4_calls/fspec.rs grows an `outtype.is_none()` guard for the output-only pieces `map
  return` parks, with a regression test, in the same PR.
