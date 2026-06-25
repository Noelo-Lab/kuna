# Implementation plan — `noreturn_propagate`

Implements the approved `proposal.md` (Phase 3 onward). The proposal already
ruled `scope: large` and was greenlit for implementation.

## The minimal feature

A new **`kuna-analysis` Listing-consumer pass** — modeled on `s1_noreturn_disc`,
NOT an in-pipeline `kuna_<slug>.rs` Action/Rule (the no-return flag is consumed
pre-S2-flow at `s2_lift/flow.rs:1838`, `&self`, so an Action runs too late).

- **Module:** `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs`
  (`NoReturnPropagatePass`, gate id `noreturn_propagate`).
- **Algorithm (worklist fixpoint):** seed the *terminal* (already-no-return) set
  from the **Known** list (`DiscoveredFunction::has_no_return`), then conclude a
  function `F` no-return when ALL of:
  1. its **last real instruction** (last by address, after skipping trailing NOP
     alignment padding) is a `CALL`/tail `JMP` whose static callee is in the
     terminal set;
  2. it has **no `RETURN`** instruction;
  3. it has **no computed/indirect jump**;
  4. every static jump target in the reachable body stays within `[entry,
     last_real]` (or is itself a terminal callee).
  Mark, add to the set, re-evaluate callers; iterate to a fixpoint. **No evidence
  threshold** (the key difference from `noreturn_disc`).
- **Output:** the existing `NoReturnFact` → the existing `commit_analysis_output`
  arm → `set_function_no_return` → the inherited `flow.rs` artificial-halt path.
  No new commit arm, no S7 work.

## Registration (the anchor edits, each `// (kuna)`)

| File | Edit |
|---|---|
| `kuna-analysis/src/lib.rs` | `pub mod s1_noreturn_propagate;` |
| `kuna-analysis/src/passes.rs` | add `NoReturnPropagatePass` to `listing_consumer_passes()` |
| `kuna-decomp/src/infra/architecture.rs` | `analysis_noreturn_propagate` flag (default-off in reset) + `set_kuna_option` arm |
| `kuna-decomp/src/p0_knowledge/options.rs` | `"noreturn_propagate"` in `KUNA_OPTION_NAMES` |
| `kuna-console/src/engine.rs` | `analysis_pass_enabled` gate arm |
| `kuna-decomp/stages.toml` | fully-populated `settableTable` row (provenance fields) |

## Default decision

**Default-OFF opt-in**, requires `option listing on`. The XML datatest path never
runs analysis passes ⇒ ablation is structurally 0/675 ⇒ `make test` stays PARITY
OK. A no-return flow heuristic can be wrong, so (like `noreturn_disc`) it ships
behind its own flag, real-ELF path only ⇒ byte-identical default. No DIV entry.

## Testing (no stages XML — analysis tier)

- **Cross-crate e2e** `verify_noreturn_propagate.rs` over the vendored differential
  fixture `noreturn_propagate_x86_64` (custom `my_die()` no-return wrapper called
  ONCE, ending in `call abort` + NOP padding): propagation eliminates the dead
  code; `noreturn_disc` does NOT (the differential); the wrapper itself is
  concluded no-return.
- **`kuna-analysis` unit tests** (pass-identity / inert-without-Listing).
- **Real-ELF demo** on `tee_O2` `x2nrealloc` (the PR's auto-captured before/after).
