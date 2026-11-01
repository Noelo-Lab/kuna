# Go byte-array scalar extraction

A recovered two-word call returns a `[16]byte` value. Its caller still uses
opaque `SUB168` helpers, preventing execution of otherwise complete Go output.
The correction reads bounded integer words from the existing local, preserving
the one producer call and the recovered result ABI. The baseline is `b848b3640c337a292802227f5de2700931d15bf3`.

Only whole, explicit, nonvolatile local byte arrays qualify. The actual printed
declaration and Go value type must agree on the complete storage size. A
constant offset must fit a one-, two-, four- or eight-byte integer result.
Big-endian storage reverses the low-end byte offset. Ordinary reads use the
facing integer type; direct stores use the rendered pointer's integer pointee
type, after checking that both it and the facing type qualify. Unsupported
types, widths, globals and implied expressions retain their existing handling.

The sixteen-image comparison covers 17,026 distinct
functions and 34,107 C/Rust/Go records. All
17,985 C/Rust records are identical. The
2,239 changed Go bodies contain 9,122
typed extractions across 3,400 textual hunks. Each extraction is
independently checked against its original helper, local array declaration,
width and bounded memory offset. Every other token, literal and comment stays
identical. All 16,122 Go bodies parse; no decompilation
errors occur. Raw diffs and classifications are retained here.

Independent Go type checking exposed a direct-store signedness error during
development that the initial execution examples missed. The final store
regression compiles and executes its unchanged emitted body under Go 1.22.5
and 1.27.3, checking both result words, two guard words and one producer call
for six boundary inputs. The final partial type check retains six function
records with diagnostics, representing two pre-existing gaps: arithmetic
widths in `runtime.(*mcache).refill` and recursive-call prototypes in
`runtime.typesEqual`. Adding only an int64-returning declaration for the old
`SUB168` helper to the baseline reproduces every remaining diagnostic.
Unresolved external symbols mean this is not a whole-program type check.

Four fresh native inputs cover Go 1.22.5 and 1.27.3 with compressed and plain
DWARF. Every emitted pair/caller body compiles and executes unchanged under
both compilers for six signed boundary and wraparound inputs. These examples
explicitly enable `godwarfsig`; default inference does not yet guarantee
agreement between independently decompiled caller and callee prototypes.
The raw unsigned-caller test separately checks one producer call and both
result words. Unit coverage includes 106 reads across both memory orders,
ten invalid bounds/width combinations and noninteger/volatile type refusals.

Validation passes 10,044 workspace tests in 71 suites,
with no failures and 39 ignored. Original datatests remain
675/675; stages are 3,488/3,488. Tooling passes 114 tests with one optional skip.
Lint, spec, catalog and counter checks pass. The original baseline is unchanged.
The five-assertion regression improves from 2/5 to 5/5 against the baseline.
An initial full run hit the existing one-second watchdog in its small follow-up
under unbounded test concurrency. Its isolated retry and the complete suite
with eight test threads pass; the original failure log is retained.

Speed uses 31 alternating baseline/patched pairs after three warmups per arm,
on CPU 79 after this task's builds and sweeps complete. Across default C,
the native Go caller, a Go function with 50 new extractions, and Rust, the
maximum change is +0.60% CPU and +1.94% wall, within
the 5% budget. Raw samples and paired changes are retained.

This repairs the recovered byte-array representation. Source multi-result
signatures, aggregate calling conventions, recursive prototype agreement and
the remaining Go arithmetic conversion gaps require separate work.
