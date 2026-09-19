# ptrfromuse — plan

## What ships

`ptrfromuse off|byte|void`, default `off`. P5, subphase `type-propagation`, tier
`transform`, change_kind `opt-in-tool`, `source_decompiler = angr`.

A function parameter whose only memory role is to be a `LOAD`/`STORE` base gets a
pointer data-type, as one more vote in `ActionInferTypes::buildLocaltypes`'
existing `getLocalType` fold. `byte` points at one unknown byte (`char *` once
`realtypes` renders it), `void` points at nothing (`void *`).

The walk's one ambiguous input is a constant addend, because `p->field` and
`table[i]` lower to the same `INT_ADD`. It is graded: a constant that resolves to
a global object refuses the candidate, one that is address-like but names nothing
is neutral, one below the space's pointer lower bound is a field offset.

## Files

| file | what |
|---|---|
| `decompiler/crates/kuna-decomp/src/p5_types/kuna_ptrfromuse.rs` | the rule: `PtrFromUseMode`, `OptionPtrFromUse`, `pointer_from_use`, the descendant walk and its refusal list |
| `decompiler/crates/kuna-decomp/src/p5_types/kuna_ptrfromuse/tests.rs` | 5 unit tests (option parse, the fold precedence in both directions, `Rc` stability) |
| `decompiler/crates/kuna-decomp/src/p5_types/coreaction_infertypes.rs` | the one call site, in `build_localtypes` after the seed/`get_local_type` choice |
| `decompiler/crates/kuna-decomp/src/infra/architecture.rs` | `ptr_from_use` field, default, `reset_defaults`, `set_kuna_option`, the `ArchSeam` copy |
| `decompiler/crates/kuna-decomp/src/substrate/context.rs` | the `ArchSeam` field the pass reads |
| `decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs` | registration |
| `decompiler/crates/kuna-decomp/phases.toml` | the `[[settable]]` row |
| `tests/stages/kuna-ptrfromuse.xml` | the three-pass end-to-end test (11 assertions: one witness, three controls) |
| `docs/spec/05-types.md` | §5.2 prose, next to `ptrdepthcap` and `codescalar` |

## Order of work

1. Bisect the causation with an instrumented throwaway build before writing the rule.
2. Implement the rule as a FOLD, never a replacement seed.
3. Three-pass stage test; unit tests.
4. Whole-corpus `decompile-all` sweep over 16 stripped binaries (8 projects at O2
   and at O0), both arms, every hunk classified — this is what found the
   `FuncProto::possible_input_param` gap, and the O0 half is where the
   statement-fold class shows.
5. Bidirectional `typesweep` over the campaign slice set, both arms, with
   `--dump-decisions` for the per-variable verdict: the ship gate.
6. Guard ablation (rebuild with the constant-addend test short-circuited, re-run
   the byte arm over the same 16 binaries) to price the refusal.
7. Speed, the 7-pass-ceiling count, and the gates.

## Counts this moves

`settables` 211 → 212, tiers `(68, 80, 63)` → `(68, 81, 63)`,
`SETTABLE_TABLE` len, `catalog-json-records` 210 → 211,
`catalog_bytecompat` fixture (recaptured, never hand-edited),
`tests/stages/kuna-catalog.xml` `#6 source_decompiler angr` 32 → 33 and
`#8 change_kind opt-in-tool` 20 → 21, corpus file count 300 → 301 (one file),
`docs/baseline-stages.json` re-recorded, `docs/options.md` regenerated.
`ptrfromuse` is added to the no-live-reader allowlist in `kuna_phases/tests.rs`
(its three values cannot be expressed by the codegen's bool `live_true`/`live_false`
pair — the same reason `pebnames` is there), so
`option_values_live_value_present_for_87` does **not** move.

## Default flip (after `protoorder` #669)

1. Re-measure both non-off values on the tree with `protoorder`: `typesweep`
   (444 slices) per value, pick the higher perfect count, then aggregate, with
   worse ≤ improved — `void`.
2. `make test` / `make test-stages` / `make test-cli` with the default flipped;
   read every moved assertion and probe.
3. Whole-corpus `decompile-all`, off against each value, every changed function
   classified and every unexplained one read (`flip-hunks.txt`), with the arity,
   argument and variable counters beside it.
4. Speed, interleaved min-of-15 (`speed.py`).
5. Fix what the sweep finds rather than document it: the store split came from a
   `protoorder` vote, so the refusal went there.

The evidence is `default-on-evaluation.md`; `record.json` `default_on_flip`
carries the numbers.
