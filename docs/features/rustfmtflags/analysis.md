# Static Rust formatting options

The pinned angr template test at
`6afa0c79a83d9f677a46b0275cdb63bf16c1979e` expects
`format!("0x{:08x} {:>5}", ...)` in `hex_width`. Its template at `0x6de2`
in the pinned fmtdemo image starts with
`02 30 78 c3 20 00 00 69 08 00 01 20 c3 20 00 00 28 05 00 00`.
The first placeholder uses static width eight and zero padding; the second uses
right alignment and width five. This feature covers those primitive trait and
option facts at existing print sinks before a separate format-result feature.

The Rust 1.93 implementation stores flags, width and precision independently.
`fmt::write` copies width and precision fields without setting their enable bits;
those bits live in `FormattingOptions::flags`. A decompiler must not interpret a
present width field as an enabled width when the flag is absent. The runtime also
reserves bit 31, stores a Unicode scalar in the low 21 bits, and uses alignment
value three for an unspecified alignment. Some byte combinations cannot be
represented by a Rust format string and must remain explicit runtime calls.

Sources: [Rust 1.93.1 encoding and runtime](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/fmt/mod.rs),
[integer implementations](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/fmt/num.rs),
and the pinned [angr template decoder](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/angr/rust/optimization_passes/macro/fmt_template.py).
Both supported Rust tags were inspected; broader versions remain unqualified.

A Rust 1.93.1 native witness (`show(x: u64)` printing `hex {:08x}`) was built
with optimization level two. Current Kuna renders `_print(0x4e50,&mut v2)`.
Its callback has both full `LowerHex for u64` and `LowerHex for usize` symbols
at the same address. Those aliases are equivalent in this verified 64-bit layout;
a Display/LowerHex conflict or different access width must still invalidate an
address.

## Implementation and boundary

The P9 extension shares the existing verified print sinks, bounded readonly
UTF-8 template reader, same-call stack descriptor cells and aligned local value
pointers. The loader retains exact integer type and trait identities in one symbol
walk. Display-only facts are derived from that map, so disabling `rustfmtflags`
retains the earlier `rustfmtargs` boundary. Known equivalent `usize`/`u64` and
`isize`/`i64` aliases are accepted only with the same trait; unknown or conflicting
aliases invalidate the address.

Static specifications retain alignment, Unicode fill, sign, alternate form,
zero-padding, width, precision and the trait suffix. The decoder rejects dynamic
counts, malformed or inconsistent fields, invalid Unicode, brace fill, conflicting
flags, a reserved high bit, non-space fill without alignment and unrepresentable
zero-width spellings. Debug hexadecimal flags require a Debug callback. Unknown,
generic, floating-point and pointer formatters stay explicit, as do other Rust
versions and call layouts. No IR or descriptor setup is removed, and no C output
is rewritten.

`rust_fmt_flags.rs` is compiled by the checked-in build script using Rust 1.93.1,
optimization level two and stripped debug information. Ten native wrappers cover
all eight integer traits and all supported signed/unsigned access widths, repeated
indices, Unicode/control/quote/backslash fill and static flags. The integration
test compiles the recovered macro expressions with explicit input bindings and
compares every stdout/stderr byte with the native executable. It does not claim
that entire decompiled functions are compilable Rust. Four native negative
wrappers and fourteen malformed/ambiguous assembly controls retain runtime calls.
The assembly fixture also tests enabled width/precision fields omitted from the
template, whose values are zero.

The existing primitive fixture's `reject_flags` and `reject_trait` are valid
explicit-default-flags and integer-exponent cases. They now recover when the new
extension is enabled; its original tests explicitly turn the extension off.
`hex_width` in angr's fmtdemo still calls `format_inner`: this change supplies
its static format-specification facts but deliberately leaves format-result
recovery to the next feature.

## Default and speed

The extension is enabled by default. The original corpus passes all 675
assertions with it off and on; the original baseline is unchanged. The new stage
adds six assertions, taking that corpus from 3,427 to 3,433, and the catalog count
moves from 358 to 359 options.

Against pre-feature `afe19872beb3b91aff7cb14e32e8c0f3b474a23c`, the final
31-sample interleaved measurements pinned to CPU 79 are:

| Case | Before | Off | On | Before/on wall delta | Off/on wall delta |
|---|---:|---:|---:|---:|---:|
| Native eight-trait print | 643.20 ms | 661.00 ms | 657.18 ms | +2.17% | −0.58% |
| Assembly hex width | 109.14 ms | 110.93 ms | 110.03 ms | +0.82% | −0.80% |
| Existing fmtdemo primitive print | 336.36 ms | 337.67 ms | 337.88 ms | +0.45% | +0.06% |

Before/on CPU deltas are +0.43%, +1.03% and +0.50%, respectively. All remain
below the five-percent budget. The three `timing-*.json` files retain warmup,
affinity, sample order and raw wall/CPU measurements. The final tree includes
main `e9b0a35f1e42b81de9a471871c63d57e664cc1d3`; the intervening changes are
Go forward-record completion and test-selection reporting, neither of which
changes the Rust-format recovery measured here.

## Breadth and validation

Serial off/on comparisons complete without errors on 9,431 distinct functions
and 10,050 language/function surfaces. Nineteen Rust functions change: four
assembly static-spec cases, ten new native wrappers, two valid cases in the older
assembly fixture, and three native Debug/exponent/width wrappers in the earlier
primitive fixture. Every changed body replaces exactly one runtime print-call
line; all descriptor setup and other lines are byte-identical. Full diffs and
per-function classifications are retained in this directory.

All 619 C function records, including their code and exported metadata, are
identical. The pinned fmtdemo's 455 functions and the older-layout control's
7,763 functions are unchanged. `breadth.json` contains fixture hashes and counts;
all runs use serial function order and a 600-second per-function watchdog so
callee-analysis outcomes cannot differ merely because one arm timed out.

Final validation passes 9,954 workspace tests (39 ignored), all 675
original assertions and all 3,433 stage assertions. Specification, catalog,
shared-counter, tooling and lint checks pass. The original baseline is unchanged.
