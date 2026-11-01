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
