# `libcwiden` -- plan

## Scope

One change: 333 rows appended to `LIBC_EXT` in
`decompiler/crates/kuna-analysis/src/analyzers/protos/kuna_libcsigs.rs`. No new
module, no new inference, no new option -- `libcsigs` already gates the table and
its `off` arm still renders exactly what the 28-entry base table renders.

## Steps

1. Census the gap first (`gap.md`): re-derive the round-F ptr_char deficit on
   this main under the pinned metric and split it into the variadic save area,
   the untypable imports, and the rest. Only then decide what to build.
2. Derive: `corpus_imports.py` over the frozen corpus's 665 dynamically linked
   binaries; `tu.c` + `gcc -aux-info` + `reduce.py` for the declarations;
   `cands.py` + `gentable.py` for the selection and the Rust fragment.
3. Measure both arms of the same build: a 444-slice pinned typesweep, a 12-binary
   whole-corpus `decompile-all` hunk classification, an interleaved min-of-15
   speed run on fmt/ls/sort/bash -O2.
4. Test: `tests/stages/kuna-libcwiden.xml` (two passes over one real ELF, with
   `option libcsigs off` in pass 1) plus three table-invariant unit tests.
5. Gates: `make test`, `make test-stages`, `make test-cli`, `make check-spec`
   (+ `--strict`), `kuna catalog --check`, `counters --check`, `make rust-test`.

## Decisions taken

* **No new option.** Section 5 of `analysis.md`.
* **No new `Ty`.** `strtoll`, `strtoull`, `strtoimax`, `strtoumax` and `llabs`
  stay out, pinned absent by a unit test. A fixed-width 64-bit member of `Ty` is
  a separate PR with its own argument about every compiler spec kuna ships.
* **The `*64` LFS aliases go in as `void *`**, not as `libctypes` named
  pointees. `stat64` filling a `void *` is honest and better than an integer;
  naming it `stat *` belongs in `LIBC_EXT_NAMED`, which is the `libctypes`
  option's table and is a follow-up, not this PR.
* **The variadic register save area is not in this PR.** `gap.md` measures it
  (52 `ptr_char` rows, 306 phantom parameters over 42 sampled functions) and
  states the ABI geometry that identifies it; it is an arity change in p4_calls
  and needs its own option and its own corpus diff.

## Follow-ups this leaves

1. The variadic save area (above), the next ranked lever.
2. A fixed-width 64-bit `Ty`, which unlocks the two most widely imported names
   kuna still cannot type.
3. `LIBC_EXT_NAMED` rows for the `*64` aliases and for the new names whose
   `void *` slots have a libc aggregate behind them (`getmntent`, `statvfs`,
   `utimensat`, `select`).
