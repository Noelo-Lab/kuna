# foldcallretphi — plan

## Scope

One decision point: `check_implied_cover`'s `Merge::inflate_test` arm, for a call
output that `foldcallret` has already accepted. Nothing else moves.

- `p6_variables/kuna_foldcallretphi.rs` (new): `conflict_is_self_call_effect`
  plus the option gate.
- `p6_variables/coreaction_cleanup.rs`: one guarded call in the inflate loop.
  Short-circuited on `Architecture::fold_call_ret_phi`, so the option OFF is
  byte-identical and costs nothing.
- `infra/architecture.rs`, `substrate/context.rs`: the flag and its ArchSeam copy.
- `phases.toml` + `p0_knowledge/options.rs`: the settable row and registration.

## Explicitly not in scope

- `foldcallret`'s order-safety predicate (`call_output_foldable`). A call may not
  cross a call, load, store or callother; that is the safety property the whole
  discount rests on and it is untouched.
- The `num_instances() > 1` veto in `base_explicit`. Measured (141 sites over six
  binaries) and rejected on the evidence: the multi-instance highs are
  speculative datatype merges with up to 22 members, not the single trim COPY the
  relaxation would need. See `analysis.md`.
- `check_implied_cover`'s "a call output whose cover crosses any call" arm (30
  sites). That one is about the candidate's *own* cover stretching over an
  unrelated call, which the fold really would reorder.

## Default

Ships **off**. It is a presentation change with no correctness claim, and the
corpus sweep is the evidence a later flip would need. `record.json` carries the
datatest/stages ablation with the default flipped on, so the flip PR has its
numbers ready.

## Verification

- Two-pass stage test `tests/stages/kuna-foldcallretphi.xml` (off = the spill, on
  = the fold) over a bytechunk whose one-instruction control (a constant argument
  instead of the global) folds with `foldcallret` alone.
- Unit tests for the option parse + the shipped default.
- Whole-corpus `decompile-all` off/on over six -O2 binaries with every hunk
  classified and a call-evaluation-order equality check over all 1871 functions.
- `timeit` interleaved min-of-N on `decompile-all`.
