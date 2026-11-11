# Manual review

`combine` calls its producer once, stores the entire result in the same local,
and reads both eight-byte words. `write_pair` reads both words with the unsigned
type of its actual destination pointee. Neither body was edited for execution.
Boundary inputs include zero, signed extrema, all-one bits and wraparound.

The planner requires a real whole local declaration and refuses implied call
expressions and global storage. It checks both the facing result and rendered
store pointee; a pointer-valued destination cannot silently become an integer
read. Unknown values carrying floating-point data also decline. Checked bounds
and endian conversion preserve the original `SUBPIECE` bit range.

The independent token verifier permits only an original `SUB{size}{width}`
call to become a typed read of that same local and bit range. It checks the
declared array extent for every conversion. Arithmetic, offset, width, comment
and declaration mutations are rejected by the verifier self-check.

Parsing alone missed an initial signedness defect in stores. A separate Go
type-checking pass found it, and a reduced native execution regression now
covers it. Six remaining diagnostic records in two function names reproduce
from unchanged baseline bodies after declaring the old helper's int64 result.
They are existing arithmetic-width and recursive-prototype gaps; unresolved
external symbols mean the diagnostic comparison is partial.

The final comparison uses a fresh build of main after the read-provenance
change and a fresh build containing this fix. Both sides are run over all
sixteen images. The original assertion baseline is unchanged.
