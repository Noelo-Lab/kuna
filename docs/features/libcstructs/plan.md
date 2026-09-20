# Plan — `libctypes`, second round

## Goal

Name the libc/POSIX struct pointers the benchmark corpus's ground truth actually holds
and the shipped table cannot reach, under the existing `libctypes` option, without a
new option and without a wrong name anywhere.

## Steps

1. **Mine the pool.** Walk the DWARF of all 444 corpus twins with decbench's own DIE
   walk, record the pointee struct tag of every `ptr_struct` ground-truth variable
   (`mine_pool.py`). 11,687 variables, 196 tags.
2. **Measure reachability, three ways.** Which names a stripped image carries
   (`readelf --dyn-syms`, imported and defined separately); which libc function each
   variable's own function calls (`objdump -d` on the twin); which aggregate each libc
   function traffics in (`gcc -aux-info` over the installed headers). Rank by the
   variables a slot the table does NOT have would newly reach.
3. **Take the slots that pay, refuse the ones whose ABI is not derivable.** Every
   signature comes out of the `-aux-info` reduction. `_obstack_allocated_p` is refused
   for want of an installed declaration.
4. **One new table for the defined-name channel.** The `_obstack_*` entry points are
   reserved-namespace names gnulib links in and exports, so they are matched against a
   name the image defines as well as one it imports — the only exception to the
   imports-only rule, and the only way the 431-variable obstack pool is reachable.
5. **Measure.** 444-slice typesweep as a two-build A/B with the campaign's own driver
   and pinned metric; the `ptr_struct` class rate; arity and variable-count counters
   beside it (the new slots supply arity where there was none); whole-corpus
   `decompile-all` hunk classification over 14 binaries; interleaved min-of-15 speed.
6. **Gates.** All four plus `make test-cli`, `kuna catalog --check` and
   `counters --check`.

## Decisions

* **No new option.** Every name ships under `libctypes`, whose `opaque` value already
  means "the pointee names the declaration knows". `glibc` installs no layout for the
  new names, which is the existing behaviour for a name with no published layout.
* **Widths are glibc x86-64**, as the first round's are, and every one of the seven was
  cross-checked against the corpus's own DWARF. A width that is a few bytes off on
  another ABI can only change whether an access renders as a field or as a cast.
* **`obstack`'s size slots are `size_t`,** against the installed glibc header's `int`,
  because gnulib's copy of the same header — the one every obstack in this corpus is
  compiled from — says `size_t` and the corpus's debug info confirms it.
* **`spwd` and `utmpx` ship although they scored nothing** on this corpus: the
  declaration is derived and correct, and their pools sit one propagation hop further
  out than `protoorder` currently reaches.
* **`_ftsent`, `sgrp` and `argp_state` are refused** although they are libc-named and
  linked in like obstack: `fts_open`, `getsgnam` and `argp_error` are ordinary names a
  program may define itself, so the reserved-namespace argument does not cover them.

## Risks, and what pins them

| risk | pin |
|---|---|
| a defined name that is NOT the library's gets retyped | `LIBC_DEFINED_NAMED` is `_obstack_*` only, asserted by a unit test |
| an off-by-one in a record/stream argument list | `the_account_database_slots_name_both_aggregates` pins all nine |
| a named type replaces a CORRECT primitive pointer | the typesweep's newly-wrong list: 6 rows, none a primitive pointer losing to a named struct |
| a fabricated variable the metric cannot see | exported-variable counters, identical on both arms |
| `libctypes off` stops being the shipped behaviour | every new name is new to both shipped tables, asserted per row |
