# Go 1.18 struct field offsets

A stripped Go 1.18 executable exposes `main.Point` and `main.Envelope` only as
opaque declarations before this fix. The field words in its runtime type graph
encode `byte_offset << 1 | embedded`; treating those words as byte offsets
causes valid structs to fail the extent or overlap checks. Go 1.19 removed this
packing. The reference encoding is visible in Go 1.18's
[`runtime/type.go`](https://github.com/golang/go/blob/go1.18/src/runtime/type.go)
and `reflect/type.go`.

The importer now selects the packed interpretation from a positively decoded
build-info version and shifts before applying the existing validation. Unknown
versions retain the previous direct-offset interpretation. This corrects a file
format interpretation under the existing `gotypes` option; it adds no new
output-style decision.

The pinned angr tests are `TestGoTypeDescriptors118.test_offset_anon_fields` and
`TestGoTypeDescriptorsPackedFieldOffsets.test_go118_field_offsets` at revision
`6afa0c79a83d9f677a46b0275cdb63bf16c1979e`. This change addresses their packed-offset
behavior on Kuna's existing 64-bit little-endian provider. The reference also
covers 32-bit targets, which remain separate work; no full test parity is claimed.

## Coverage

A hermetic factory-import test checks old embedded offsets, Go 1.19/1.27 direct
offsets, unknown-version behavior and corrupt extents. The real stripped fixture
checks `Point` fields at 0/8/16 and an `Envelope` with an embedded pointer at
byte 8, on both amd64 and arm64. Runs use Go 1.18 and Go 1.22.5.

The full printed type surface of the Go 1.22.5 fixture is unchanged after sorting
declarations. The Go 1.18 fixture goes from 93 opaque declarations to none;
its two application structs now print their actual fields and padding. The
checked-in type diffs capture this comparison. These counts do not claim full
recovery of all possible nested or anonymous Go types.

Whole-image comparisons cover 1,385 Go 1.18.1 functions and 1,545 Go 1.22.5
functions, with no changed C output. `unicode.init` hit the 10-second watchdog
on both sides; isolated retries with a 60-second budget completed successfully
and produced the same 43,751-byte body. Interleaved timing samples and their
controls are recorded in `record.json`.

A controlled 31-pair comparison against an unmodified checkout of the exact
base (`e27d88545`) measured 420.78 ms before and 420.90 ms after
(+0.03%). Both sides used the same release flags and two warmups. An
earlier comparison against the bounds-feature branch with its option disabled
measured +6.10%; those samples are retained, but that build differs beyond this
fix and is not the controlled baseline.
