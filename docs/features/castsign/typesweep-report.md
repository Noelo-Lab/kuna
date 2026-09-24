# typesweep castsign=off vs on (default)

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
