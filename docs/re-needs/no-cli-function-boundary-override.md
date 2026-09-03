---
need_id: no-cli-function-boundary-override
title: an agent cannot tell kuna where a function starts or ends
track: tooling
status: open
severity: blocker
hypothesis_status: upheld
credibility: 1.0
instances: 1
rounds: [2]
first_seen_round: 2
attempts: 0
touches: [decompiler/crates/kuna-cli/src, decompiler/crates/kuna-console/src/ifacedecomp.rs]
scope: large
---

## Symptom

The console has `map function <addr> [name] [nocode]` and `load addr <addr> [name]`, both functional. The `kuna` binary can emit neither: `decompile.rs::build_script` produces a fixed vocabulary (load file / option / read symbols / load function|addr / kassert / decompile / print) and there is no `kuna console`, no `--script`, and no passthrough. Worse, `function F spans [start,end)` does not exist ANYWHERE -- extent is derived in `kuna-console/src/funcextent.rs` as [entry, min(next_entry, section_end)) with no override, and `phases.toml` has no subphase for function-entry identification at all. On an obfuscated or packed image, where discovery is exactly what fails, the agent has no lever. Round 1 hit this three times: `kuna functions` returning count 0, a phantom entry at 0xfe6dca9f, and `disassemble main` on an image whose main was never discovered.

## Reproduction

Every named CLI flag is rejected with `error: unknown option`, verified on the round-1 merge build; the stubs are verified by driving `decomp_dbg` directly and reading back `engine integration not yet ported`.

## Hypothesis

ADVISORY. The cheap half is exposure, not implementation: most of these commands already work and only lack a path from the `kuna` binary. The expensive half is the stubs. A builder should measure which is which before choosing a design, and should NOT assume a `kuna console` passthrough is the right shape -- a scriptable console is a different product from a set of flags an agent can compose.

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should confirm the demand.
