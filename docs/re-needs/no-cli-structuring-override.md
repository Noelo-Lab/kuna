---
need_id: no-cli-structuring-override
title: an agent cannot reclassify a flow from the CLI -- `--assert` has no `flow` directive
track: tooling
status: open
severity: major
probe_id: p-a862d011f87f
acceptance_id: a-158ef4220dee
hypothesis_status: upheld
credibility: 1.0
instances: 1
rounds: [2]
first_seen_round: 2
attempts: 0
touches: [decompiler/crates/kuna-cli/src/assertdecl.rs, decompiler/crates/kuna-console/src/ifacedecomp.rs, decompiler/crates/kuna-decomp/phases.toml]
scope: small
---

## Symptom

`override flow <addr> branch|call|callreturn|return` is fully ported and works -- it is the one
structuring override that is not a stub -- and it is reachable only from the console. The `kuna`
binary's `--assert` plane carries eleven directives (`function`, `typedef`, `prototype`, `data`,
`param`, `return`, `comment`, `name`, `type`, `readonly`, `volatile`) and `flow` is not among them,
so an agent driving the CLI cannot correct a misclassified call/branch/return at all.

The effect it is denied is not cosmetic. On `aif_gap_x86_64`, `sub_13c9` decompiles as a 682-byte
extent with 25 `// eax` temporaries because flow follows an indirect `call *%rdx` at `0x1405` into
its neighbours; `override flow 0x1405 return` in `decomp_dbg` collapses the same function to
`return dat_4014;`. Both halves of that were measured on merged main 7604f8be.

This record was NARROWED in round 2 (see the decision log): as filed it also covered the three
`engine_unavailable` structuring stubs, which are a different and much larger piece of work.

## Reproduction

Demonstrates the gap on merged main: the directive is rejected by the CLI parser before the engine
is ever reached (`kuna-cli/src/assertdecl.rs:275`), exit 2, nothing on stdout.

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "--addr",
    "0x13c9",
    "--json",
    "--assert",
    "flow 0x1405 return",
    "--assert-strict"
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
    "selector": "0x13c9",
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
      "unknown directive \"flow\""
    ]
  },
  "notes": "The gap as it stands: the one structuring override whose engine path IS ported (IfcFlowOverride, kuna-console/src/ifacedecomp.rs:2911) has no spelling on the --assert plane, so an agent driving the kuna binary cannot reclassify a flow at all.",
  "probe_id": "p-a862d011f87f"
}
```

## Acceptance

Asserts the desired behaviour on the same command: `--assert 'flow <addr> branch|call|callreturn|
return'` reaches the ported `IfcFlowOverride` path, is reported in the `assertions` fate array like
every other directive, and produces the output the console produces today. The two output clauses
are MEASURED, not guessed -- `return dat_4014;` present and `v25` absent is exactly what
`override flow 0x1405 return` yields in `decomp_dbg` on this fixture. In-repo fixture, so it
promotes to `tests/cli/` verbatim.

```json
{
  "schema": "re-probe/1",
  "kind": "cli",
  "cmd": [
    "{{KUNA}}",
    "decompile",
    "{{BIN}}",
    "--addr",
    "0x13c9",
    "--json",
    "--assert",
    "flow 0x1405 return",
    "--assert-strict"
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
    "selector": "0x13c9",
    "selector_kind": "addr"
  },
  "expect": {
    "exit_code": {
      "eq": 0
    },
    "stdout_is_json": true,
    "json": [
      {
        "path": "assertions[0].kind",
        "op": "eq",
        "value": "flow"
      },
      {
        "path": "assertions[0].status",
        "op": "eq",
        "value": "applied"
      },
      {
        "path": "functions[0].code",
        "op": "contains",
        "value": "return dat_4014;"
      },
      {
        "path": "functions[0].code",
        "op": "not_contains",
        "value": "v25"
      }
    ]
  },
  "notes": "Desired: `--assert 'flow <addr> branch|call|callreturn|return'` reaches the ported IfcFlowOverride path and is reported in the assertions fate array. The output clause is MEASURED: `override flow 0x1405 return` in decomp_dbg today collapses the 65-line sub_13c9 body (v1..v25, 682-byte extent) to `return dat_4014;`. In-repo fixture, promotes verbatim.",
  "probe_id": "a-158ef4220dee"
}
```

## Hypothesis

ADVISORY. Exposure, not implementation: the engine path exists (`kuna-console/src/ifacedecomp.rs`
`IfcFlowOverride`, with the stashed facts re-seeded on each flow follow) and the CLI plane exists
(`--assert` + `--assert-strict` + the `assertions` fate report, shipped this round by #389/#391).
What is missing is the arm in `assertdecl.rs` that spells `flow` and routes it. A builder should
still confirm that the CLI's one-shot decompile re-seeds the override the way the console's
`load function` -> `override flow` -> `decompile` sequence does, and should decide whether the
subphase's `exposure` field in `phases.toml` needs to stop saying console-only.

## Refutation

Not refuted by an agent; the hypothesis's two load-bearing claims were measured directly by the
captain in round 2 wave 21 and both hold (the console override works; the CLI directive does not
exist). `kind` is effectively `absence`, which is the case `REPIPE_REFUTE_MODE=absence-skip` skips.

## Reference

- console command: `decompiler/crates/kuna-console/src/ifacedecomp.rs:2911` (`IfcFlowOverride`)
- CLI directive parser: `decompiler/crates/kuna-cli/src/assertdecl.rs:263-275`
- the stubs this record NO LONGER covers: `decompiler/crates/kuna-console/src/kuna_console.rs:718`
  (`Dispatch::ForceGoto`), `:724` (`Dispatch::MultistageJump`), `:727` (`Dispatch::FlowOverride`,
  the `--kassert` arm), and the `structure blocks` / switch-model entries routed through
  `engine_unavailable`.

## Instances

- seeded for round 2 from a source survey of the override surface (not tester-filed).

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers
  hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should
  confirm the demand.
- round 2 T_TRIAGE (captain): track tooling / touches [kuna-console/src/ifacedecomp.rs,
  kuna-decomp/phases.toml] / scope large CONFIRMED, with one lease consequence worth stating: it is
  the only no-cli-* need that declares phases.toml, so it takes file:phases.toml and cannot run
  beside the round's option-adding quality builder even though its track is tooling. That is
  correct, not a mislabel -- it edits the exposure fields of subphases whose overrides are
  engine_unavailable stubs. Still waits behind the function-boundary proposal.
- round 2 wave 21 (captain, B_DRAIN observe tick): **PROBE + ACCEPTANCE ADDED AND GATED**, and the
  record NARROWED to the flow slice. This was the last open need with `probe_id: None` and
  `acceptance_id: None` -- i.e. undispatchable by `select` and unclosable by B_DONE no matter what
  a builder shipped. Two-arm verdict on merged main 7604f8be: `admitted` (p-a862d011f87f PASSES,
  a-158ef4220dee FAILS on all six clauses).
- round 2 wave 21 (captain): the narrowing, and why it is not a quiet scope cut. The record as
  filed bundled four overrides. Three are `engine_unavailable` stubs whose desired output cannot be
  pinned without first designing them, so any acceptance covering all four would have had to be
  vague enough to pass on a no-op -- and a broad need closed by a narrow acceptance is the
  false-close this round has already caught twice. The fourth, flow reclassification, has a working
  engine path and a measurable effect, so it gets an exact contract. The three stubs are recorded
  under `## Reference` and are NOT closed by this need; re-file them with a witnessed instance
  (round 2 produced none -- this record was seeded, never tester-filed) rather than carrying them
  as unprovable clauses here.
- round 2 builder `b-r2-no-cli-structuri` (BUILT, acceptance PASSES): shipped as the twelfth
  `--assert` directive, `flow [<func>::]<addr> branch|call|callreturn|return`, lowering to the
  ported `IfcFlowOverride` on the script surface and seeded through
  `FunctionSeed::flow_overrides` on the in-process one. Hypothesis UPHELD in both halves --
  exposure, not implementation, and no engine work was needed. The two things the hypothesis
  asked a builder to confirm, both confirmed by measurement rather than by reading:
    * the CLI's one-shot decompile DOES re-seed the override the way the console's
      `load function` -> `override flow` -> `decompile` sequence does, and by a different
      route than the console's: `kuna decompile --json` is the IN-PROCESS load
      (`decompile_json` -> `decompile_all::load_program`), not the forked script, so the
      console's `pending_flow_overrides` stash is never consulted there. The directive is
      seeded into the same `DecompileSeed::flow_overrides` slot the analysis's `call
      error(nonzero,…)` no-return prunes already use, appended AFTER them so a caller-stated
      fact wins the map insert at an address both name. Both surfaces then render the same C,
      byte for byte, on this fixture.
    * the subphase `exposure` field DOES need to stop saying console-only -- and this PR does
      NOT change it, the one part of the brief it does not deliver. The replacement text was
      written and measured (`command override flow; kuna --assert 'flow <addr>
      branch|call|callreturn|return'; option noreturn`: regenerates through build.rs, catalog
      stays green, no count moves), then reverted, because `mergecheck` shape-C guards
      phases.toml as a keep-both table and rejects any line present on origin/main that is
      gone -- it cannot tell an edited row from a sibling's row eaten by a merge. Defeating a
      merge guard for a prose line is not a trade worth making, so the file ships
      byte-identical to main and the text is recorded as residue. The lease this need reserved
      for phases.toml was therefore not spent.
  One thing the record did not anticipate: `call` is the one word of the four the ENGINE can
  refuse. Forcing an indirect call to a direct `call` has no destination to make direct, so
  `Funcdata::overrideFlow` raises `Could not apply flowoverride`; the run reports that as the
  function's own error rather than decompiling as though nothing had been asserted, and that
  refusal is itself the proof the directive reaches the engine. `branch` at the same address
  is the interesting one for RE: it re-reads the indirect call as a computed jump and recovers
  its two-case table.

- round 2 wave 21 (captain): scope large -> small. Measured basis: the plane the need was waiting
  for shipped mid-round (#389 `--assert`, #391 the range directives), so the remaining work is one
  directive arm in `assertdecl.rs` routed to an already-ported engine command. `touches` KEEPS
  `phases.toml` -- the exposure field of the owning subphase plausibly changes, and an
  under-reserved lease lets two builders collide, which is worse than queueing.
