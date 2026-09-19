# typesweep protoorder=types

slices scored: 444
functions scored: 10748

## control

- baseline vs published type_match: 9123 agree / 1625 differ (84.88% agreement)
- byte-identical variables in both arms: 6838 functions, 0 scored differently (must be 0)

## result

- type_match PERFECT: base 986 -> test 1105 (+119)
- aggregate type_match: base 3111.18 -> test 3363.95 (+252.77)
- moved ONTO perfect: 119
- moved OFF perfect: 0
- improved (not perfect): 838
- worsened (not perfect): 3
