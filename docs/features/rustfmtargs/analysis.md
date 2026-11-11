# Dynamic Rust print arguments

At angr commit `6afa0c79a83d9f677a46b0275cdb63bf16c1979e`, TestRustFormatMacrosTemplateLayout expects print_one and reorder in fmtdemo-1.93.1 to render println! with recovered values and argument indices. Kuna exposes the two runtime words and stack descriptor array instead.

Rust 1.93.0/1.93.1 stores bounded literal pieces and placeholders in a readonly template; the other word points to pairs of value pointer and formatter callback. The final IR preserves descriptor cells as stack INDIRECT effects associated with the print call. Their input zero is the pre-call cell, whereas their output can describe a modified value and must not supply the callback identity.

Display, Debug and scientific formatter names can collapse when trait qualifiers are removed. Even the core::fmt::num::imp module contains both Display and exponent traits. A bare or normalized fmt name is insufficient evidence. Preserve exact raw-symbol trait/type identity before normalization and reject ambiguous aliases. Generic reference formatters also require pointee type recovery and remain outside this feature.

Primary sources: [Rust 1.93.0 encoding](https://github.com/rust-lang/rust/blob/1.93.0/library/core/src/fmt/mod.rs), [Rust 1.93.1 encoding](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/fmt/mod.rs), [primitive traits](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/fmt/num.rs), and the pinned [angr decoder](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/angr/rust/optimization_passes/macro/fmt_template.py) and [tests](https://github.com/angr/angr/blob/6afa0c79a83d9f677a46b0275cdb63bf16c1979e/tests/analyses/decompiler/test_rust_decompiler.py#L397). The real fixture is angr/binaries commit 76ad9e296dde8f82b8ede93ce06a578cd1933639, tests/x86_64/rust/fmt_arguments/fmtdemo-1.93.1. The pinned angr integration test was executed locally in an isolated environment and passed all eight expectations in 28.433 seconds. This confirms the reference target; it does not imply that Kuna meets those expectations.

## Implemented boundary

The loader reads full symbol identities directly from linked 64-bit little-endian
ELF files. The loader's ordinary function list is already normalized and is not
usable for this purpose. Repeated symbols must agree: usize/u64 and isize/i64 are
equivalent in this verified layout, while unknown aliases or different primitive
identities invalidate the address. Other compiler versions do not collect this
fact and cannot enter the renderer.

Sink names can reside in a single qualified name or in a leaf symbol plus its
namespace chain. An unqualified `_print` or `_eprint` qualifies only at the exact
function entry with the complete `std::io::stdio` chain. The argument array is
local, aligned stack storage; only inputs of INDIRECT effects tied to this call
supply its cells. Pure pointer and constant walks are bounded. No earlier call's
unknown memory effect can provide a formatter constant.

The renderer retains all descriptor assignments and emits borrowed typed reads
of the original pointers. It does not remove setup statements, reconstruct all
Rust syntax, or claim that a complete decompiled function compiles. Tests compile
the emitted macro expressions in explicit input-binding harnesses. One compares
all stdout/stderr bytes with the original Rust 1.93.1 binary; another checks
unsigned and signed extremes, a narrower integer read, and repeated indices.
The real compiler fixture keeps scientific, Debug and width formatting as
unchanged runtime calls. Deterministic assembly controls also reject malformed
UTF-8, unsupported template encodings, missing indices, mutable text, unknown or
variable callbacks, unrelated sinks, global values, return values and intervening
call clobbers. C output remains unchanged.

## Measurements

The option is enabled by default. Against main
`ab68632ee864a421742de2404d97b291a803d371`, 31 interleaved CPU-pinned
samples measured 340.78 ms before and 340.67 ms after (−0.03%). Turning
`rustfmtargs` off in the new binary measured 336.18 ms, a +1.34% off/on
delta. Raw wall and CPU samples are in `timing.json`.

The parallel breadth comparison covers 8,812 distinct functions and 9,406
language/function surfaces. Eleven functions replace exactly one runtime print
call with a macro; all other lines in those completed functions are unchanged.
This includes both new primitive expectations in angr's template-layout fixture.
The older-layout control has 7,763 functions and no changes. Serial comparisons
also recover the same eleven macros. Full changed-function hunks and fixture
hashes are retained alongside this record.

One large runtime function, `Cache::with_global` at `0x18736`, sometimes exceeds
the 180-second watchdog before emission. Those errors are retained in
`sweep-errors.json`; a missing body is not counted as a formatting rewrite.
A paired isolated Rust retry with a 600-second budget completed both sides with
identical code. Isolated and whole-batch analysis have different global type
context, documented by `retry-context.diff`, so that isolated result is not
substituted into the batch diff. The serial C run also has four prototype-driven
caller differences when only one side completes the large function. Their full
diffs are retained separately. The extended full-batch C retry completed all 566
functions on both sides with zero errors and byte-identical code throughout,
confirming that the four caller differences came from the asymmetric timeout.
See `breadth-serial600.json` and the aggregate code hash in `record.json`.

Validation passes 9,935 workspace tests (39 ignored), all 675 original corpus
assertions, all 3,424 stage assertions, specification/catalog/shared-counter checks, tooling tests
and lint. The original baseline is unchanged. The stage baseline adds four
format-argument assertions and updates the option count.
