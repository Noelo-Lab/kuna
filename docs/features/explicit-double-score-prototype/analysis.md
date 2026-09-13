# Refined XMM0 cells must preserve one declared `double`

## Witness

RE-friction need `explicit-double-score-prototype` points at PolyMLP.exe
`score` (`0x140003150`). The function computes its mean with `DIVSD` at
`0x1400034fe`, copies the result to XMM0 at `0x140003515`, calls the exact MSVC
security-cookie checker at `0x14000351f`, and returns. With explicit prototypes
for `score`, `log`, and the checker, the dependency-base output ended in
`CONCAT44` of an undefined upper half and the low half of the division.

The exact dataset image is SHA-256
`01652e8b03e2bec127f5726320db4c9925d495d75583a8bb4534ac640f6ec230`
(54,272 bytes). The committed 3.5 KB PE twin forces the same refinement and is
SHA-256 `9454da07fedd100366e7a5955a29d218eb7f05bbb5bf5c3aa96f7d50f1c7deb9`.

## Root cause

The preceding `declaring-double-score-return` fix preserved an exact ABI output
slice when the call's killed range contained that slice. Here later low-dword
traffic makes heritage partition the eight-byte XMM0 return into two four-byte
cells. At the cookie call the low cell is `ContainsJustified` in the checker's
preserved ABI output and survives. The adjacent high cell is
`ContainsUnjustified`: it is still inside the same eight-byte `double`, but the
exact-cookie helper intentionally accepted only the justified edge. The
caller's return read therefore combines one defined and one killed cell.

## Chosen seam

Permit the existing exact-cookie proof to preserve a
`ContainsUnjustified` refinement cell only when the enclosing caller has an
output-locked prototype and characterizes the same cell as part of its declared
output. This second, caller-side gate distinguishes the high half of the
declared `double` from unrelated XMM scratch storage.

No generic proof is widened. The exact-cookie body/effect evidence is reused
unchanged, so a known ABI-output write, an output-space STORE, or an explicit
effect still vetoes preservation. `ContainsJustified` continues through the
old helper; `NoContainment` scratch cells remain killed.

## Result

With all three explicit prototypes, the exact PolyMLP target now retains the
division and returns it directly. `calleeretpreserves off` restores the
undefined concatenation. The refined stage control also proves that an
inferred/unlocked caller does not gain this exception, while disabling the
independent `calleepreserves` option leaves the repaired output unchanged.

The no-cookie-prototype shape remains out of scope: that path incorrectly
infers XMM0 as an argument to the checker and is an argument-modeling defect,
not return-storage preservation.
