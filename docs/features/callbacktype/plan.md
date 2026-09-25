# `callbacktype` — plan

1. **Census** (done, `census.md`): scan the 444 slices for functions whose address is handed
   to a declared callback slot; check every one against the DWARF twin; count the
   ground-truth parameters that would move.
2. **Module** `decompiler/crates/kuna-decomp/src/p4_calls/kuna_callbacktype.rs`: the slot
   table, the run's ledger, the declared-pieces builder and the body refusals.
3. **Driver** `callback_park_round` in `decompiler/crates/kuna-cli/src/decompile_all.rs`:
   the image half of the escape question (`open_function_entries`,
   `CallGraph::address_taken_refs`), the park, and the redo of the callback and its direct
   callers, before the `calleevote` rounds so a redone callback's types are what the vote
   decides against.
4. **Option** `callbacktype on|off`, default `off` until the flip evaluation:
   `phases.toml` row, `p0_knowledge/options.rs`, `Architecture` field + dispatch,
   `docs/options.md`, the catalog counts.
5. **Tests**: 13 unit tests on the table and the ledger; a negative-control stage test
   (`tests/stages/kuna-callbacktype.xml`) for the one-function path; two `tests/cli` probes
   over a new fixture `callbacktype_x86_64` covering both halves and the escape control.
6. **Spec**: a `callbacktype` section in `docs/spec/04-calls-and-prototypes.md`.
7. **Measurement**: the 444-slice typesweep, a whole-corpus `decompile-all` hunk
   classification, and interleaved min-of-15 speed.
