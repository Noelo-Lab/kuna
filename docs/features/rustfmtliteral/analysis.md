# Literal Rust print macros

The reference is angr commit `6afa0c79a83d9f677a46b0275cdb63bf16c1979e`,
`tests/analyses/decompiler/test_rust_decompiler.py`,
`TestRustFormatMacrosTemplateLayout.test_format_macros`. Its literal witness is
`fmtdemo-1.93.1` at native ELF address `0x10520` (angr rebases it to `0x410520`).
The binaries repository is pinned to `76ad9e296dde8f82b8ede93ce06a578cd1933639`.
This implements the literal witness; the seven dynamic-format expectations and
LTO-specialized stderr test remain separate work.

Rust's own `library/core/src/fmt/mod.rs` at tags
[1.93.0](https://github.com/rust-lang/rust/blob/1.93.0/library/core/src/fmt/mod.rs)
and [1.93.1](https://github.com/rust-lang/rust/blob/1.93.1/library/core/src/fmt/mod.rs)
define the odd second word as a literal byte length shifted left with bit zero
set. The first word points directly to the UTF-8 bytes. Even second words describe
placeholder templates and do not qualify here.

The renderer requires this verified stable version, Rust source detection, a
64-bit little-endian data space, an exact standard-library output sink, two
constant arguments and no call result. Named globals can introduce a zero-base
PTRSUB around the constant; at most four pure COPY/CAST/PTRSUB steps are followed.
Every byte is checked against readonly, nonvolatile memory ranges, then read and
validated as UTF-8. No IR operations are deleted or moved. The emitter replaces
only the known runtime call's spelling and its encoded literal operands.

The fixture's seven accepted cases cover stdout, stderr, empty text, non-ASCII
UTF-8, braces, quotes, backslashes, NUL and repeated LF. The emitted Rust is compiled
and executed, with stdout and stderr compared byte for byte. Refusal cases cover
mutable bytes, malformed UTF-8, unmapped spans, a nonconstant pointer, even and
oversized tags, a null empty pointer, another namespace's `_print`, an extra
argument, and a used call result. Missing, earlier, later and unverified patch
versions preserve the call, as does C output. Compiler-record unit tests cover
malformed, mixed and prerelease versions.

Whole-function diffs and timing are recorded beside this file. The feature does
not claim complete Rust formatting parity: placeholder templates, format!/write!,
LTO-specialized sinks, and older fmt::Arguments layouts remain unsupported here.

## Measured breadth and speed

On main `38b4be20b6ca9d9e86a1cf31afc1557d8d008324`, the option changes seven
of the 20 assembly-fixture functions and one of the 455 real Rust 1.93 demo
functions. All 7,763 functions in the older Rust coreutils `fmt` image are
unchanged. That is 8,238 distinct functions and eight changed functions, with
zero decompilation errors in either mode. The additional C rendering of the
20-function assembly fixture is byte-identical. Each changed hunk is a single
literal runtime call replaced by its corresponding macro; returns, surrounding
statements and the real witness's tail-call annotation remain unchanged.

Thirty-one interleaved samples on CPU 79, after three warmups per mode, measured
349.52 ms median with the option on versus 344.32 ms off (+1.51%). A separately
built copy of the exact main commit measured 347.07 ms (+0.71% for the feature),
including the load-time compiler-version reader. CPU-time deltas were +1.38% and
+0.70% respectively. Raw wall and CPU samples are in `timing.json`; the
five-percent budget passes. These measurements replace the earlier base's data.

Validation: 9,925 workspace tests passed (39 ignored), 675/675 original
assertions, 3420/3420 stage assertions, catalog, spec, lint and tooling checks.
The original baseline is unchanged. Relative to main, the stage baseline adds
only three new assertions and updates the catalog count assertion name.
