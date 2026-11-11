# Recover String-valued format calls without changing the lowered ABI

Pinned angr's template test expects `format!("0x{:08x} {:>5}", ...)` for
fmtdemo `hex_width` at 0x11100. Kuna currently prints
`alloc::fmt::format::format_inner(a0,0x6de2,&mut v2)` and leaves the hidden result
pointer explicit. The static specifications and exact u32 formatter identities
are now available, but the print-only consumer refuses this three-input call.

The native Rust1.93.1 program in `repro.rs`, built with optimization level two,
also calls `format_inner(a0, template, descriptors)` and returns `a0`. The call
has no own output varnode. The visible return is part of the existing lowered
ABI and need not change to recover the formatting expression. A typed pointer
write preserves initialization semantics; assignment through a String lvalue
would incorrectly drop uninitialized destination storage.

Scope is limited to this exact standard-library sink and verified layout. Generic
wrappers, `format` fast paths, unknown callbacks, indirect calls and returned call
values remain explicit. `make_string` also needs a generic string formatter and
is not claimed by this feature.

The native local-output case still declares its three String words separately.
Rendering a typed String write into only the first primitive would suggest a
complete local allocation that has not been recovered. This feature therefore
requires a non-spacebase entry destination; local-buffer calls remain explicit.

## Behavioral checks

Five console integration tests cover three native result functions and three
hermetic call shapes, including integer and pointer/reference destinations.
Recovered expressions compile into MaybeUninit<String> initialization, preserve
exact native String contents and terminal LF, and each initialized String is
dropped once. Nineteen negative wrappers retain their complete original output:
five native unsupported/storage cases and fourteen malformed or unsupported
assembly calls. Version, source/output language, sink namespace, formatter
identity and parent/static-option boundaries are checked independently.

The shared helper extraction preserves all 8,814 previously captured functions
from three complete Rust images with the result option off. This is a separate
baseline comparison, not merely a comparison between two settings of new code.

## Speed

The final measurement uses 31 interleaved baseline/off/on rounds after three
warmups per mode, pinned to CPU 79. The baseline is 81238846a, the merged static
format-specification feature. Median default-on wall time changes are -1.025%
for the native result fixture, -1.103% for the hermetic fixture, and -0.649% for
angr's fmtdemo hex_width. Off-to-on changes are -0.541%, -0.473% and +0.208%; all
remain within the five-percent budget. Full samples and CPU times are retained.

## Regression setup

The original corpus passes all 675 assertions off and on. The stage corpus now
passes all 3,438 assertions; only five new expectations and the catalog count
name enter its baseline. The first stage attempt used stale PowerPC/SPARC specs
from the primary checkout and failed seven unrelated existing assertions. The
retry uses the same 154 compiled specs as the passing static-formatting feature;
all prior assertions pass without baseline relaxation. The x86 specs were already
byte-identical, so the Rust output and timing comparisons are unaffected.

## Complete-function comparison

Eight serial sweeps cover 10,040 function/output-language surfaces and 9,441
distinct functions with zero errors. Exactly seven bodies change: three native
result wrappers, three hermetic destination/namespace witnesses and fmtdemo's
hex_width. Every diff consists of one runtime-call line replaced by a typed
initialization containing format!. All surrounding statements and returns match.
The 599 C function records, including variables, types and mappings, match in
full. The two existing print fixtures and all 7,763 older Rust functions are
unchanged. All changed hunks and fixture hashes are retained in this directory.

The final workspace suite passes 9,968 tests with zero failures and
39 intentional ignores. Spec, catalog, counter, tooling and lint checks
also pass. No original baseline expectation changes.
