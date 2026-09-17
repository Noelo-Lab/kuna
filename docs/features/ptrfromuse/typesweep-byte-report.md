# typesweep ptrfromuse=byte

slices scored: 444
functions scored: 10748

## control

- baseline vs published type_match: 9505 agree / 1243 differ (88.44% agreement)
- byte-identical variables in both arms: 8802 functions, 0 scored differently (must be 0)

## result

- type_match PERFECT: base 959 -> test 964 (+5)
- aggregate type_match: base 3037.05 -> test 3045.57 (+8.51)
- moved ONTO perfect: 5
- moved OFF perfect: 0
- improved (not perfect): 15
- worsened (not perfect): 0
