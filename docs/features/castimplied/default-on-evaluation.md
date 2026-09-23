# castimplied: default-on evaluation

Measured on base `dbe854ba3` (origin/main when measured), both arms from the same
build: the default-on binary with `--option castimplied off` as the old default.

| | criterion | result |
|---|---|---|
| (a) | `make test` with the new default | 674/675 before the opt-out. `Union #26` (`union_datatype.xml`, upstream) pins `res = (int4)ptr->b->bCharField;` into the `int4` local `res`, exactly the intended change; the test now carries `<com>option castimplied off</com>` (it pins upstream's form). 675/675 PARITY OK, `docs/baseline.json` untouched. |
| (b) | `make test-stages` | 1331/1341 before; the 10 moved assertions all pinned a widening this option leaves out, and now match the new form (list below). 1341/1341 PARITY OK; `docs/baseline-stages.json` gains only the 12 `castimplied #N` keys of the new stage test. |
| (c) | `make test-cli` | 236/236 with the new default; no probe moved. |
| (d) | 444-slice typesweep (pinned metric, 8 projects x O0/O2/O2-noinline) | perfect 1,609 -> 1,609, aggregate 3963.45 -> 3963.45, 0 moved on, 0 moved off, 0 improved, 0 worse. `variables[]` is byte-identical in all 10,748 scored functions, so no variable or argument is added or removed. `typesweep-report.md`. |
| (e) | speed, interleaved min-of-15, `decompile-all --json` | fmt -O2 -0.08% min / +5.90% median; fmt -O2 (re-run, load 8) +1.54% min / +2.41% median; ls -O2 +1.05% min / +1.86% median; sort -O2 +0.31% min / -3.86% median; bash -O2 +0.47% min / +6.59% median. Worst min +1.54% (within +5%). The first pass's fmt and bash medians (+5.9%, +6.6%) were taken while two other lanes timed the same binaries; the fmt re-run at a quieter moment gives +2.41%. `speed.json`, `speed.py`. |
| (f) | whole-corpus `decompile-all` before/after, 8 binaries outside the cast corpus (bzip2 O0, bash O2, kmod O2, dpkg-divert O2, chage O0, crontab O2, dash O2-noinline, crond O0) | 1,947 casts removed; every changed line is the old line with integer cast tokens (and their parentheses) deleted, nothing else. By position: 1,253 assignment, 192 return, 451 stacked inner, 51 call argument. All 541 `v = (T)v2;` statements re-checked against the printed declarations: `v` is declared `T`, and `v2`'s declared type widens into `T` without a sign change. `corpus-hunks.json`. |
| (g) | `modes.rs` | coherent: the catalog default is on, so every preset (reliable, aggressive, fast) inherits it; `aggressive_carries_every_default_off_option` needs no entry. |
| (h) | castbench full (45 binaries, 4,815 functions shared with IDA) | 45,126 -> 43,760 casts (-1,366, -3.0%), 236.8 -> 229.6 per kloc, 1.193 -> 1.157 x IDA; 570 functions fewer, **0 functions more**. Whole corpus: 3,360 casts removed, all cast-only. |

## The ten stage assertions that moved (b)

Each pinned a cast of another option's output that C performs by itself:

- `undefname #4`: `v12 = (uint8)((uint4)v12 + 8);` -> `v12 = (uint4)v12 + 8;` (`uint8 v12`).
- `charbyte #1 #2 #4 #7 #8`: `return (int4)(v1 == 0x2d);` -> `return v1 == 0x2d;` (and the `'-'`, `0x80 <=`, `0xe9`, `'\xe9'` forms), from functions declared `int4`.
- `PEBNAMES #18`: `return (uint8)v1->NtGlobalFlag;` -> `return v1->NtGlobalFlag;` (a `uint8` function).
- `ptrfromuse #10`: `return (int4)(char)table[a0];` -> `return (char)table[a0];` (the `(char)` stays).
- `TIEDPHITRIM #1 #2`: `v2 = (int8)v3;` -> `v2 = v3;` (`int8 v2`, `int4 v3`).

Each assertion still pins what its own option decides (the char literal, the PEB
field name, the named global, the loop variable).

## Value preservation

`kuna-cli/tests/decompile_all_cli.rs` `an_implied_cast_round_trips_through_the_printed_c`
compiles the printed functions of `castimplied_{gcc,clang}_O0_x86_64` with the
option off and on, with gcc and with clang, and requires every build to print what
the binary prints (negative `char`s, `0xfffffff0u`, negative `short`s, a varargs
`printf`, a sign change under a widening that must stay). Mutating the printed C to
drop a kept cast (`return (int)a0;` -> `return a0;`, or the varargs `(long)`) makes
it print a different value, so the harness can see a wrong removal.

## Found during evaluation

The first cut took an explicit operand's C type from the IR when its declaration
did not match, and removed `v36 = (unsigned long)v9;` with `int *v9` (a merged
register variable): an implicit pointer-to-integer conversion. The assignment
re-check in (f)/(h) caught it; the operand's type now comes from its declaration,
and a declaration that spells none of the IR's types refuses the drop. The numbers
above are after the fix (it cost 401 of the 1,767 removals the first cut made).

## Re-measured on the landing base

Rebased onto `22df4282c` (#719, calleevote's redo budget, which changes types and
calls), both arms from the rebased build, `--option castimplied off` against the
default:

- 444-slice typesweep: 1,615 -> 1,615 perfect, 0 moved on or off, 0 improved, 0 worse;
  `variables[]` scores identically in all 10,748 functions, and the off arm matches
  origin/main's own sweep row for row.
- castbench full: 45,039 -> 43,673 casts (-1,366), 236.3 -> 229.2 per kloc,
  1.191 -> 1.155 x IDA; 570 functions fewer, 0 more. The off arm is byte-identical to
  origin/main's castbench output.
- Hunk classification: whole corpus 3,360 casts removed, the same class counts as
  above, 0 OTHER lines; the 4-binary corpus (bzip2 O0, kmod, dpkg-divert, crontab O2)
  555 removed, identical to the measurement on `dbe854ba3`. All assignments re-checked
  (890 and 48): 0 flagged.

## Decision

Default ON.
