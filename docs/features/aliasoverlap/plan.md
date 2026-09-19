# aliasoverlap: implementation plan

A strict correctness fix with no option: it only stops the printer from moving
a load past a store that overwrites some of its bytes.

## 1. The change (`p6_variables/coreaction_cleanup.rs`)

- `check_implied_cover` passes both access widths to `is_possible_alias`: the
  STORE's value size and the LOAD's output size, as an `AliasSpan`
  `{w1, w2, scale}`.
- `AliasSpan::overlaps_at(d)`: with the second pointer `d` units past the
  first, the ranges `[0, w1)` and `[d*scale, d*scale + w2)` intersect.
- `is_possible_alias_step` returns `Option<bool>`: when one pointer is the other
  plus a constant (`INT_ADD`/`PTRSUB`, or `PTRADD` times its element size), the
  answer is whether the ranges intersect at that distance; for `INT_XOR` with a
  constant it is whether they can intersect at the constant's lowest set bit,
  the least the two can differ by; `None` when no such step links them.
- `is_possible_alias`: two constant pointers alias iff their ranges intersect;
  in the `INT_ADD` arm, two different constants off functionally equal bases
  alias iff the ranges intersect at the constants' difference (taken signed at
  the pointer size, so `base - 1` against `base` is distance -1). Recursing
  into two `PTRADD` indices multiplies `scale` by the element size; recursing
  through a negation (`INT_2COMP`/`INT_NEGATE`) negates it. The commuted
  `a0 == b1` / `a1 == b0` matches are only taken for byte-scaled adds, where
  they mean the same distance.

## 1b. The split-load path (`p3_dataflow/subflow.rs`)

- `SplitDatatype::split_load` keeps the split at the LOAD, and the COPY in
  place, unless `RuleDoubleLoad::no_write_conflict` (`p5_types/double.rs`, now
  `pub(crate)`) clears the ops between the LOAD and its lone COPY.

## 2. Tests

- `kuna-cli/tests/decompile_all_cli.rs`
  `a_load_is_not_printed_after_a_store_into_its_bytes`: fixture
  `kuna-analysis/tests/fixtures/aliasoverlap_x86_64` (+ `.c`, README row); checks
  statement order in six functions and compiles the printed C against the
  source. Main: 3 of 6 functions return a different value; fix: 0.
- `tests/stages/kuna-aliasoverlap.xml`: the same six functions as a bytechunk,
  plus four functions over a typed `S *` for the split-load path (a byte store
  into the read, a store through a second pointer, a call, and a control with
  nothing between the read and its COPY); eight assertions, main fails #1-#3
  and #5-#7. The split-load shapes print `v1._0_1_ = ...` partial writes, which
  do not compile, so they are pinned by statement order rather than a round
  trip.

## 3. Measurement

Corpus diff (`decompile-all`, base vs fix, every changed function classified),
444-slice typesweep (`--baseline-only` per binary), interleaved min-of-15 speed
on fmt/ls/sort O2 and bash O2. Results in `record.json`.
