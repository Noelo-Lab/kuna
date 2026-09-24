# typesweep castsign=off vs on (default)

## Review round 3, on the branch merged onto 52202840d

Arms: a fresh build of origin/main `52202840d` (castarith landed) against the
default-on build of this branch merged onto it, same instrument and pin as below.
1,615 -> 1,615 perfect (15.03%), mean .3697 -> .3697; 0 moved on, 0 moved off,
0 improved, 0 worse; per level O0 1110 -> 1110, O2 89 -> 89, O2-noinline
416 -> 416. All 10,748 functions have identical values in both arms.

## Earlier rounds, on base 5458b7ab5

slices scored: 444 (coreutils grep gzip diffutils bzip2 findutils tar shadow x O0/O2/O2-noinline)
functions scored: 10748
metric: decbench type_match pinned at 625e892 (DECBENCH_PIN=_final-d/db625), DECBENCH_NO_CACHE=1
arms: the same final build, `--option castsign off` against the default

## result

- type_match PERFECT: off 1615 -> on 1615 (+0)
- mean type_match: off 0.3697 -> on 0.3697
- moved ONTO perfect: 0
- moved OFF perfect: 0
- improved (not perfect): 0
- worsened (not perfect): 0
- per level: O0 1110 -> 1110, O2 89 -> 89, O2-noinline 416 -> 416
- every one of the 10,748 functions scores identically in both arms, and the off arm
  matches origin/main's own sweep (castimplied's landing arm) row for row

castsign changes C declarations only; the frame Symbol types that `variables[]`
reports are untouched (and `variables[]` is byte-identical off vs on over the six
extra-corpus binaries, 6,673 functions).

After the type-lock check (commit `51f5e35ac`) the default-on arm was re-run: its
`rows.json` is identical to the arm above on all 444 slices. decbench scores
stripped binaries, which carry no locked Symbol.

After the arithmetic rule and the printed-cast rule (review round 2) the
default-on arm was re-run on the final build: its `rows.json` is identical to the
off arm on all 444 slices and all 10,748 functions (1,615 perfect, mean .3697).
The option changes C declarations and casts only; `variables[]` is byte-identical
off vs on over 15 disjoint binaries (8,841 functions) and 10 DWARF binaries, so
no variable or argument is added or removed.
