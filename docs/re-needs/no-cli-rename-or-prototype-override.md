---
need_id: no-cli-rename-or-prototype-override
title: an agent cannot rename or retype anything, or force a prototype
track: tooling
status: open
severity: major
hypothesis_status: upheld
credibility: 1.0
instances: 1
rounds: [2]
first_seen_round: 2
attempts: 0
touches: [decompiler/crates/kuna-cli/src, decompiler/crates/kuna-console/src/kuna_console.rs]
scope: large
---

## Symptom

`rename`, `retype`, `map param`, `map return`, `override prototype` and `parse line extern ...` are all functional in the console and none is reachable from `kuna`. Renaming a FUNCTION is absent even there (`rename` resolves only against the current function's local scope). `kuna decompile --kassert` exists and routes to `IfcKunaAssert`, but only its `Rename` (naming-policy) and `Option` arms do real work -- `prototype-source` and `type-propagation` return engine_unavailable. For an agent, a rename that does not persist is the difference between reading a decompilation once and actually working through one.

## Reproduction

Every named CLI flag is rejected with `error: unknown option`, verified on the round-1 merge build; the stubs are verified by driving `decomp_dbg` directly and reading back `engine integration not yet ported`.

## Hypothesis

ADVISORY. The cheap half is exposure, not implementation: most of these commands already work and only lack a path from the `kuna` binary. The expensive half is the stubs. A builder should measure which is which before choosing a design, and should NOT assume a `kuna console` passthrough is the right shape -- a scriptable console is a different product from a set of flags an agent can compose.

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should confirm the demand.
- round 2 T_TRIAGE (captain): track tooling / touches [kuna-cli/src, kuna-console/src/kuna_console.rs] / scope large CONFIRMED. New this tick: keyboard-callback-uses-undefined is a tester-filed demand witness for it -- an agent that could state a callback's prototype would not need kuna to infer R9D/R8D at all. Still waits behind the function-boundary proposal for the delivery vehicle.
- round 2 wave 9 B_PLAN (captain): DISPATCHED as the override family's one design, because the delivery vehicle this need was waiting behind now EXISTS -- no-cli-function-boundary-override closed via #374 (`kuna decompile --define-function`), an agent-supplied fact injected from the `kuna` binary. Three standing instructions for the builder. (1) Design ONE override plane that covers this need AND its two siblings (no-cli-data-code-override, no-cli-structuring-override); the captain will not approve three separate large designs, and a unified proposal lets the siblings collapse into it rather than each burning a builder. (2) Answer the Hypothesis's cheap/expensive question as a table: for every command named in the Symptom, whether it already works in the console and only lacks a `kuna` path, or is an `engine_unavailable` stub. (3) The proposal MUST define a concrete `acceptance` probe for this need. It has none today (`verify --acceptance-suite` reports it unrunnable), so nothing built on it could ever be closed by B_DONE, and a design that leaves it unrunnable is not approvable.
