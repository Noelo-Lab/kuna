---
need_id: no-cli-structuring-override
title: an agent cannot force a goto, a jump table, or a flow reclassification
track: tooling
status: open
severity: major
hypothesis_status: upheld
credibility: 1.0
instances: 1
rounds: [2]
first_seen_round: 2
attempts: 0
touches: [decompiler/crates/kuna-console/src/ifacedecomp.rs, decompiler/crates/kuna-decomp/phases.toml]
scope: large
---

## Symptom

Of the four structuring overrides `phases.toml` advertises in its own `exposure` fields, three are `engine_unavailable` stubs -- `force goto` (edge-virtualization), `override jumptable` (switch-model) and `structure blocks` -- and the fourth, `override flow <addr> branch|call|callreturn|return`, works but is console-only. The CLI's only per-address CFG lever is `--option outline <fn>:<head>-<exit>`. Round 1's `switch(0)` defect is the shape of this: `switchselector` fixes the cases whose selector cannot be re-read, but an agent looking at a differently-broken switch has nothing to say about it. `--kassert` is the interface that was MEANT to carry this and its `switch-model`, `edge-virtualization` and `flow-classification` dispatch arms all return engine_unavailable.

## Reproduction

Every named CLI flag is rejected with `error: unknown option`, verified on the round-1 merge build; the stubs are verified by driving `decomp_dbg` directly and reading back `engine integration not yet ported`.

## Hypothesis

ADVISORY. The cheap half is exposure, not implementation: most of these commands already work and only lack a path from the `kuna` binary. The expensive half is the stubs. A builder should measure which is which before choosing a design, and should NOT assume a `kuna console` passthrough is the right shape -- a scriptable console is a different product from a set of flags an agent can compose.

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should confirm the demand.
- round 2 T_TRIAGE (captain): track tooling / touches [kuna-console/src/ifacedecomp.rs, kuna-decomp/phases.toml] / scope large CONFIRMED, with one lease consequence worth stating: it is the only no-cli-* need that declares phases.toml, so it takes file:phases.toml and cannot run beside the round's option-adding quality builder even though its track is tooling. That is correct, not a mislabel -- it edits the exposure fields of subphases whose overrides are engine_unavailable stubs. Still waits behind the function-boundary proposal.
