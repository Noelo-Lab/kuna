# Package-local Go record names

A recovered Go record pointer currently uses its qualified metadata name inside
its own package. `main.scale` therefore prints `func scale(p *main.point, k
int64)`. Go cannot refer to its current package through that qualifier. The
recovered `point` layout and the two multiplications are already correct.

The baseline is `b46ca80e2d300ea6007c9c66a629e60b85101c85`. The native repro
declares `type point struct { x,y int64 }`, multiplies both fields in a noinline
function, and calls that function from main. Go 1.27.3 reproduces the invalid
qualification with `--language go --option godwarfsig on`.

The emitter now carries the current Go function's package in its type-spelling
context. Only simple type names with that exact package prefix lose their
qualifier. The context reaches pointer and array elements, declarations and
conversions. External and compound type names retain their existing spelling.
The function's import path, receiver or generic arguments do not become a type
qualifier. Other source languages and output languages receive no Go package
context. Interned metadata and the call ABI are unchanged.

This is a strict correction under the existing Go output selection, without a
new option. The owned module is `p9_emit/kuna_gopackage.rs`; the per-document
context uses a shared string so recursive type spelling does not copy the
package name. C and Rust output are checked for exact equality.

The existing hermetic signature check now compiles and executes all twelve
supported bodies in both compressed and uncompressed images, including scale.
Its record test verifies that `(3,-7)` multiplied by four becomes `(12,-28)`.
The independent native Go repro performs the same execution check with the
untouched emitted body. Unit cases cover exact versus external prefixes,
receivers, import paths, generic arguments and nested pointer/array spelling.
The new end-to-end assertion fails on the baseline and passes with the fix.

The sixteen-image comparison covers 17,026 distinct
functions and 34,107 C/Rust/Go records. All
17,985 C/Rust records are identical. The
3,562 changed Go bodies remove
8,839 current-package qualifiers across
5,986 textual hunks. A token comparison permits only those exact
prefix removals and requires every other token, literal and comment to remain
identical. All 16,122 Go bodies parse, with no
decompilation errors. The raw diffs and per-hunk classification are retained.

Validation: 10,010 workspace tests pass, none fail, and 39 are ignored across
71 suites. Original datatests remain 675/675; stages are 3,472/3,472. Tooling
checks pass 114 tests with one optional skip. Lint, spec links, the catalog and
shared counters pass. The workspace execution checks use Go 1.27.3, and the native witness also
executes under Go 1.22.5. The original baseline is
unchanged.

Speed uses 31 alternating baseline/patched pairs after three warmups per arm,
on CPU 79 after this task's builds and sweeps complete. The maximum measured
change is -0.66% CPU and
-1.03% wall across default C, the Go record
witness and a Rust control. Raw samples, ratios of medians and median paired
changes are included in each timing artifact; all remain within 5%.
