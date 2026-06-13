# w9-con-rulecompile-unify
verdict: ACCEPT
verifier: Claude Opus 4.8 (1M context)
date: 2026-06-13
gate: `cd rust && cargo build -p kuna-console` -> ok (Finished dev profile, exit 0); checklist gate `cargo test -p kuna-console` -> ok (5 passed, 0 failed)

## Summary

This is a **deferred stub** item, not a logic port. The diff replaces two
1-line placeholder modules with two fail-closed marker types
(`RuleCompile::not_ported`, `UnifyState::not_ported`) that unconditionally
return the LowlevelError-class LOSS error, plus a shared `NOT_PORTED` message
const. The central factual claim — that `rulecompile`/`unify` have **zero
oracle exposure** — was independently re-derived from the C++ sources and holds
completely.

### Zero-oracle-exposure re-derivation (the load-bearing claim)

- `decompiler/cpp/rulecompile.cc` is entirely inside `#ifdef CPUI_RULECOMPILE`
  (`:16` open, `:889` `#endif`; lines 891-1013 are a `/* ... */` comment block
  holding the original flex source — no live code outside the guard).
- `decompiler/cpp/unify.cc` is entirely inside `#ifdef CPUI_RULECOMPILE`
  (`:16` open, `:1647` `#endif`). With the macro undefined both `.cc` files
  compile to **empty translation units** — there is **no always-linked
  surface**, contrary to a naive reading of the checklist note (the note's
  "only the always-linked surface needs faithful porting" resolves to *nothing*
  here).
- Every include of `rulecompile.hh`/`unify.hh` is itself guarded:
  `ifacedecomp.hh:27` and `architecture.cc:21` both `#ifdef CPUI_RULECOMPILE`.
  The `IfaceDecompData::experimental_file` field (`ifacedecomp.hh`) and the
  `IfcParseRule`/`IfcExperimentalRules` **registration** (`ifacedecomp.cc:143-145`)
  and **execute bodies** (`ifacedecomp.cc:3194`) are all CPUI_RULECOMPILE-guarded.
- The macro is **never** defined in any kuna build: `decompiler/cpp/Makefile`
  documents it only as an optional flag (`:107`, a comment); `COMMANDLINE_*`,
  `TEST_DEBUG`, `GHIDRA_DEBUG` never pass `-DCPUI_RULECOMPILE`. Grep for
  `-DCPUI_RULECOMPILE` across all Makefiles/mk/toml: never defined.
- No datatest (`decompiler/datatests/`) references `parse rule`/`experimental
  rules`; no unit test (`decompiler/unittests/`) references
  `RuleCompile`/`UnifyState`/`ConstraintGroup`/`UnifyCPrinter`.

Conclusion: the Rust port and the C++ default (oracle) build are behaviorally
identical — neither exposes `parse rule`. There is **no divergence from the
kuna oracle**, so this is an ACCEPT, not ACCEPT-WITH-LOSSES, in the strict
sense. A `losses.md` entry (LOSS-121) is appended anyway to make the deferral
of ~3.6k dormant LOC auditable.

## Hunt list

- Signedness: clean (vacuous — no comparisons in the ported code).
- Integer widths: clean (vacuous — no integers; the only values are a `&str`
  const and zero-sized marker structs).
- Wrapping: clean (vacuous — no arithmetic).
- Comparator totality: clean (vacuous — no `Ord`/comparator ported).
- Iteration-order provenance: clean (vacuous — no loops, no containers).
- Off-by-one / do-while / reverse iterators: clean (vacuous — no loops).
- Erase-while-iterating: clean (vacuous — no mutation/traversal).
- Exception -> Result partial-state parity: clean. The two ported entry points
  unconditionally `Err(...)` with no prior mutation, mirroring the C++ throw
  sites (`IfaceExecutionError`/`IfaceParseError` in the guarded
  `IfcParseRule::execute`) which also fail before persistent state changes.
  Fail-closed semantics match: an undefined-command in C++ never runs; here the
  entry points always error.

## Mechanical pass

- `cargo clippy -p kuna-console --all-targets -- -D warnings`: clean (no output).
- grep `todo!`/`unimplemented!` (macros): none (only the word "unimplemented"
  in doc comments). `HashMap`/`HashSet`/`sort_unstable`: none. bare `as` casts:
  none. `unwrap()`/`expect()`: only inside `#[cfg(test)]`.
- `cargo run -p port-audit -- --item w9-con-rulecompile-unify`: **N/A** — the
  `port-audit` package does not exist in this worktree's Rust workspace
  ("package(s) `port-audit` not found in workspace"). Not a blocker for this
  item; flagged as a protocol-tooling gap.

## Observations (non-blocking)

- O1 (info): `RuleCompile::not_ported` / `UnifyState::not_ported` are `pub` but
  currently have **no caller** (the whole `kuna-console` crate is W9 scaffold;
  `parse rule` is unported). `pub` suppresses dead-code warnings, so this is
  benign scaffolding for the deferred feature.
- O2 (info): the crate's `Cargo.toml` already declared `kuna-num`,
  `kuna-sleigh`, `kuna-decomp` at the `rust-port` baseline; this item uses only
  `kuna-base`. Those three are currently unused, but were **not** introduced by
  this diff (the diff only updated `Cargo.lock` to reflect the now-`use`d
  `kuna-base`), and `unused_crate_dependencies` is not enabled. They will be
  consumed by the sibling W9 console modules. Not a finding against this item.

## Adversarial tests (added + committed on branch, all pass)

- `unify::tests::w9_con_rulecompile_unify_fail_closed_for_any_type` — the LOSS
  result is independent of the monomorphized `T` (`()`, `u64`, `Vec<i64>`,
  `fn()->i32`, `u8`, `String`), and the error payload is identical across `T`.
- `unify::tests::w9_con_rulecompile_unify_is_lowlevel_variant_with_stable_message`
  — asserts the `Lowlevel` variant (mirrors `throw LowlevelError`), pins the
  exact message bytes, and checks `Display` surfaces it verbatim.
- `rulecompile::tests::w9_con_rulecompile_unify_shared_loss_message` — the two
  halves of the subsystem yield byte-identical errors (single recognizable
  message, as the module doc claims).

Test run: 5 passed (2 original + 3 new), 0 failed.

## findings

none (blocker/major).

## losses

LOSS-121 (appended by verifier): dynamic rule compiler (rulecompile + unify)
deferred — CPUI_RULECOMPILE, no oracle exposure, no behavioral divergence from
the kuna oracle build.
