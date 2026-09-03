---
need_id: no-cli-data-code-override
title: an agent cannot mark a range as code or data, or set a type at an address
track: tooling
status: open
severity: major
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

`map address <addr> <typedeclaration>`, `parse line`, `type varnode`, `readonly <addr+size>` and `volatile <addr+size>` all work in the console and none is reachable from `kuna`. `--option readonly on|off` is a GLOBAL toggle, not a range. `global add`/`global remove` -- the commands `phases.toml` names as the exposure for the `code-data-partition` subphase -- are `engine_unavailable` stubs, so marking a gap as code is unavailable even in the console. This is the lever the round-1 need `runtime-decrypted-code-opaque` wants: a buffer decrypted at runtime is data at load time and code afterwards, and nothing can say so.

## Reproduction

Every named CLI flag is rejected with `error: unknown option`, verified on the round-1 merge build; the stubs are verified by driving `decomp_dbg` directly and reading back `engine integration not yet ported`.

## Hypothesis

ADVISORY. The cheap half is exposure, not implementation: most of these commands already work and only lack a path from the `kuna` binary. The expensive half is the stubs. A builder should measure which is which before choosing a design, and should NOT assume a `kuna console` passthrough is the right shape -- a scriptable console is a different product from a set of flags an agent can compose.

## Decision log

- seeded for round 2 from a source survey of the override surface, after round 1 showed testers hitting obfuscated images with no lever to correct kuna with. Not tester-filed: round 2 should confirm the demand.
