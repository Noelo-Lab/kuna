# Two dimension — ActionMarkImplied alias guard (w10-two-dimension) — ACCEPT, +6

Wave (2026-06-18) integrated at rust-port (merge of `rport/w10-two-dimension` @ `36bbfe0`, base
556c23e/490; re-gated onto 501). Verdict: **ACCEPT**.

## Root + fix

The task framing (array-index recovery / PTRADD collapse) was a red herring — the engine already
recovers `myarray[gi][valin]` and the raw p-code is byte-structurally identical to C++. The real
root: a STUBBED alias guard in `ActionMarkImplied`. `check_implied_cover` (`coreaction_cleanup.rs`)
was `{ true }` (always allow implied → always inline), so a LOAD read by two ops was inlined into two
copies instead of factored into the `v1` temp C++ keeps. C++ `ActionMarkImplied::checkImpliedCover`
(coreaction.cc:3479) forces a LOAD explicit when its Cover crosses a STORE/CALL at a possibly-aliasing
address (`isPossibleAlias`/`isPossibleAliasStep`, coreaction.cc:3382-3470). Ported
`is_possible_alias`/`is_possible_alias_step` + the LOAD/CALL-crossing arms (reading the merge-built
Cover via lazy `update_varnode_cover`). The `Merge::inflateTest` arm is left at C++-default (allow) —
it only ever yields MORE inlining than the oracle, so it cannot regress.

## Effect

490 → 496 on its base; +6 on the integrated 501 tree → **507**. Gained: Two dimension #1/#2,
Partial splitting #8/#9/#10/#11 (one coherent alias-guard root). Three dimension #1/#2/#3 stay passing.

## Gate (re-run at integration onto 501)

- `cargo test --workspace --no-fail-fast`: 0 failures.
- datatests: `[675, 507]`; passing-set diff vs base 501: **regressed-set EMPTY** (gained the 6).
  Single file (coreaction_cleanup.rs), disjoint from the heritage-callwrite chain.
- switch cluster switchind 16/16, switchmulti 9/9 unchanged; oracle **PARITY OK**; C++ byte-untouched.

## No special-casing

Faithful port of `checkImpliedCover`/`isPossibleAlias`; no name/address/value hardcoding.
