# typesweep protoorder=types

slices scored: 4
functions scored: 3451

## control

- baseline vs published type_match: 3381 agree / 70 differ (97.97% agreement)
- byte-identical variables in both arms: 2526 functions, 0 scored differently (must be 0)

## result

- type_match PERFECT: base 208 -> test 217 (+9)
- aggregate type_match: base 506.93 -> test 539.88 (+32.95)
- moved ONTO perfect: 9
- moved OFF perfect: 0
- improved (not perfect): 112
- worsened (not perfect): 4
