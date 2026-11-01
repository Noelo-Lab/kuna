# Bounded Go byte-array views

The output selection already promises Go spelling. A native two-word result
currently uses artificial members on a [16]byte local, which Go cannot compile.
Correct those members in P9 without changing the inferred result ABI or adding
source tuple contracts.

The planner in p9_emit/kuna_gobyteview.rs accepts bounded unsigned/unknown byte
arrays. Whole-array covers use the array value. Integer views of widths 1, 2, 4
and 8 use one unsafe typed dereference at the existing byte offset, preserving
the named path. Unsupported widths, types, positions and volatile/annotation
values decline. Anonymous partial-array types require the cached scalar form;
unknown values carrying floating-point data decline. The facing IR type and
the rendered Go value type must both qualify, and the pointer view uses the
rendered type so assignments preserve their signedness.

Execute unchanged native and compressed/plain fixture bodies on Go 1.22.5 and
1.27.3 for signed extremes, zero and wrapping additions. Check bounds and type
refusals, end-to-end stage output, unchanged C/Rust output, every changed Go
hunk, parser validity, four repository gates and the speed budget. Validation uses the merged
package-type foundation.

This does not resolve arbitrary source tuples, SUB168 extraction helpers in
callers, arbitrary-width pieces or pre-existing Go array argument decay.
