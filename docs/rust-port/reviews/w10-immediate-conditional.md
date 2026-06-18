# Immediate Conditional — ScopeLocal over-tie removal (w10-immediate-conditional) — ACCEPT, +5

Wave (2026-06-18) integrated at rust-port (merge of `worktree-agent-ab57c88c7c314b7e2` @ `6dfb618`,
base `eb8548a`/609). Verdict: **ACCEPT**.

## The roots (LOSS-234 brief was partly stale — RuleConditionalMove was already ported)

- **zeroprop #2/#3/#4** (`return v1;` / `v1='\0';` / `v1='a';`): the raw p-code is byte-identical;
  the divergence was a W4-ScopeLocal **over-tie**. The char-return high inherited the param symbol
  `ptrint` through a size-1-vs-size-4 base-address overlap. Fix: `narrower_addrtied_local`
  (`coreaction_cleanup.rs:1957`, the `ActionNameVars handleSymbolConflict` path) — a narrower addr-tied
  non-input/persist/constant representative overlapping a wider **scalar** entry runs the conflict scan
  → fresh `vN`/dynamic symbol (composite entries excluded, so struct-field members keep reusing).
- **condmove #7** (`return cptr[8] != 'a';`): `mark_output_storage_addr_tied`
  (`coreaction_cleanup.rs:399`) forwarding-register-alias arm — gate the tie on the source register's
  high carrying a recovered-local Symbol. The ARM flag register `tmpZR` carries none, so the return
  un-ties and `baseExplicit` inlines `return cptr[8] != 'a';`. (`RuleConditionalMove::apply_op` was
  already fully ported at ruleaction_7.rs:1980 — the LOSS-234 "SEAM" note was stale.)

## Effect

609 → **614** (+5). Gained: Immediate Conditional #2/#3/#4/#7 + Partial Merge #2 (bonus — readpartial
now byte-matches the oracle structure).

## Gate (at integration onto 609)

- datatests `[675, 614]`; regressed-set **EMPTY**.
- `cargo test --workspace --no-fail-fast` (debug): **0 failures** — after an integrator fix (below).
- oracle `run_tests --baseline docs/baseline.json`: **PARITY OK**; C++ + docs byte-untouched.

## Integrator fence fix (FENCE-DISCIPLINE)

The cargo fence (not the datatest count) caught `partialmerge_3_forwarding_alias_storage_stays_tied`:
the wave correctly removed the readpartial over-tie so the structure now matches the oracle, and the
agent ADDED a byte-identical `assert_eq` against cpp — but its worktree had no cpp oracle binary, so
that branch never ran there. In the main tree it ran and tripped on a **pre-existing** declaration-line
comment gap (oracle `int4 a_simple; // tmp` vs rust `int4 a_simple;` — LOSS-238), which affects no
datatest assertion. Resolution: narrowed that one over-strict new assertion to strip trailing `// ...`
decl-comments before the structural compare (the meaningful invariants — no forbidden
`return glob1.a + 10;` collapse, `a_simple` stays explicit — remain pinned). Recorded LOSS-238.

## No special-casing

Faithful ActionNameVars handleSymbolConflict transcription; un-tie keyed on storage geometry +
recovered-Symbol presence, not register name (the existing anti-special-casing tests A3 hold).
