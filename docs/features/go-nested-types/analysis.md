# Complete Go value dependencies before their parents

A stripped Go 1.22.5 program with `type Z struct { X int }` and
`type A struct { Z; Tail int }` prints `A` with eight padding bytes where `Z`
belongs. The importer declares struct shells first, then fills them in typelink
order. If the parent comes first, its child's incomplete shell has size zero
and is incorrectly dropped as a zero-size field.

The bounded iterative dependency walk now orders named by-value struct fields
before their enclosing layouts, following child descriptors even when they have
no independent typelink. Existing complete factory layouts are left alone.
Pointer fields introduce no value-layout dependency. Cycles and malformed
metadata are refused, and a still-incomplete field whose descriptor has nonzero
size refuses its enclosing layout. Actual zero-size markers retain the old
behavior. No new rendering decision or option is introduced.

This is part of the type-layout groundwork exercised by the pinned angr Go
runtime-descriptor and Go-type tests, including their field-offset comparisons.
It does not complete Go type parity: generic instantiations, arrays, anonymous
types and recursive pointer completion still need separate work.

## Evidence

The factory test makes the child absent from the typelinks, verifies that its
field survives at byte 8, and checks both a cyclic value graph and an invalid
child extent. The real stripped amd64/arm64 test includes the embedded value at
byte zero and runs with Go 1.18.1 and Go 1.22.5.

On the small Go 1.22.5 witness, 23 struct definitions recover missing nested
fields, including `main.A`, runtime records, reflection records and polling
state. `sync.entry` becomes opaque: its generic child is outside this provider's
scope, so the previous all-padding definition was false layout knowledge. The
full normalized type-surface diff and the per-type classification are retained.
Whole-image C comparisons and speed measurements are in `record.json`.

The three whole-image comparisons cover 4,472 functions across Go 1.18.1 and
Go 1.22.5, with zero errors and no changed emitted C. The per-function budget
was 60 seconds. A 31-pair interleaved timing comparison after two warmups
measured 382.70 ms before and 394.23 ms after (+3.01%), within the
5% budget. The baseline is the exact parent commit, with matching release flags.
