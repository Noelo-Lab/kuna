# fieldtype — the default flip, evaluated

Measured with one build of this branch rebased onto origin/main 4f037dae4:
`--option fieldtype on` against the shipped default `off`. The default arm is
byte-identical to a fresh castbench arm of origin/main 4f037dae4 on all 45
castbench binaries.

| criterion | result | passes |
|---|---|---|
| (a) `make test` | 675/675, PARITY OK; the datatests load no file, so structsynth never runs there | yes |
| (b) `make test-stages` | 1490/1490, PARITY OK; only the 13 `kuna-fieldtype.xml` keys are this option's, no other key moved | yes |
| (c) `make test-cli` | 269/269; no probe pins a field this option re-types | yes |
| (d) 444-slice typesweep | 1,674 -> 1,674 perfect, 0 improved, 0 worse, 0 moved on or off perfect; `decomp_vars` identical in all 10,748 functions. A disjoint 165-slice sweep (e2fsprogs, dash, kmod, zlib, shadow, dpkg): 454 -> 454, 0 improved, 0 worse | yes |
| (e) speed, interleaved min-of-15 (fmt, ls, sort, bash -O2) | against origin/main 4f037dae4: default -0.15%, +0.05%, +0.37%, -0.10%; `on` +0.03%, +0.15%, +0.04%, -0.14% (speed.json); worst +0.37%. Before structsynth stopped keeping access values with the option off, two bash runs read default +4.42% / +5.02% and `on` +5.76% / +6.56% | yes |
| (f) whole-corpus hunks | 1,896 functions over 45 binaries, every one classified (corpus-hunks.txt), no hunk outside the documented effect; no function changes its argument count, variable count or variable sizes | yes |
| (g) modes.rs | no preset names the option; it is listed, with this evaluation, beside `charptr` in the preset bookkeeping test (`aggressive_carries_every_default_off_option`) | yes |
| (h) castbench full | 32,073 -> 31,974 casts on the 4,815 shared functions (0.848x -> 0.845x IDA; 27.0 -> 26.9 per 100 statements), 53 functions fewer (-134), 14 more (+35) | yes |
| layout, fields-only (the lane's extra gate) | precision .8713 -> .8693 (880 -> 878 of 1,010), recall .0932 -> .0930 | **no** |

## Why the layout criterion fails

A reader shares a synthesized record only when every field it claims agrees with
the record's field at the same offset, width and type (`ledger.rs`, "Agreement
is exact"). The option re-types a field in the function whose accesses hold it
as a pointer and leaves it alone in a function whose accesses do not, so two
readers of one object that agreed on `long` now disagree on `long` against
`void *`. In `du` -O0, `fts_read` sees `fts_cur` handed to a declared `void *`
parameter and its record's first field becomes `void *`; `fts_children` only
adds to it, keeps `long`, is no longer answered by that record, and mints its
own five-member view: three claimed fields that matched `FTS` are lost. In `du`
-O2 the opposite happens once (`struct exclude` is answered by a larger record
whose extra three claims miss). The evidence is already restricted to what the
program does with a value (a dereference, a call, a declared pointer parameter),
and the loss is unchanged from the first, looser rule (.8693): the disagreement is
between readers, not a weak vote in one.

## What would let it flip

The ledger would have to agree on a pointer-width field by pointer-ness rather
than by spelling, without letting an integer word stand in for a pointer (the
ledger's own measurement: an `undefined8` absorbed by any 8-byte field unified
du -O2 `sub_c090`'s double bit patterns with a pointer record). That is a
ledger change, measured on its own.
