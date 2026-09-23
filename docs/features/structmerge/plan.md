# structmerge -- plan

## Option

`structmerge off|siblings`, P5, subphase `aggregate-union`, `change_kind =
structure-recovery`, tier `transform`, default `off`. Registered in
`p0_knowledge/options.rs`, applied in `infra/architecture.rs`, carried into the
`ArchSeam` in `build_arch_handle` and into `substrate/context.rs`.

## Code

* `p5_types/kuna_structmerge.rs` -- the mode, the option parse, and `merge()`:
  the compatibility test (`shared_claims`), the evidence bar
  (`agreement_is_evidence`), the merged member list (`plan`), and the partner
  choice (most shared claims, then smallest, then first minted).
* `p5_types/kuna_structsynth/ledger.rs` -- `lookup_or_mint` takes the mode and
  calls `kuna_structmerge::merge` when nothing held answers and before minting
  the layout alone; `merge_window` bounds the partner search by size. The
  layout algebra (`FieldKey`, `Layout`, `Member`, `layout_of`,
  `layout_of_fields`, `keeps_unclaimed`, `answers_for`) widens from
  `pub(super)` to `pub(crate)` so the new module can use it.
* `p5_types/kuna_structsynth/shard.rs` -- a worker's recording lookup passes
  `Off`, so a `--jobs N` run never merges.
* `KUNA_STRUCTMERGE_TRACE=1` prints one line per candidate considered, with the
  number of shared claims and the verdict.

## Tests

* `p5_types/kuna_structmerge/tests.rs` -- seven unit tests: the option surface,
  the union of two overlapping readers (with the first record superseded), a
  contradicted claim at the same offset and one over the same bytes at a
  different offset, a single shared claim, the growth bound, the unclaimed-bytes
  veto, and `off` minting a record per reader.
* `tests/stages/structmerge-siblings.xml` -- two arms over the same four
  hand-assembled readers: with `option structmerge off` each of the first pair
  keeps its own record and re-deciding the first changes nothing; with
  `siblings` the second pair is answered by the union and re-deciding the first
  moves it onto the union too.

## Gates and measurements

* `layoutscore.py` both arms (the precision gate), `dedup_heldout.py score` and
  `absorb` over 177 builds (precision and record identity off the tuning set).
* 444-slice typesweep `--option structmerge siblings`.
* `decompile-all` before/after over ten binaries, every changed function
  classified (`hunks.md`).
* Interleaved min-of-15 `decompile-all` on `fmt`, `ls`, `sort` and `bash` at -O2.
