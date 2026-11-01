# Rust signed primitive operators

Signed byte comparisons and arithmetic right shifts currently print with Rust's
unsigned byte storage type. For example, native `(-1i8 < 0)` is true, while the
emitted `(255u8 < 0)` is false; an arithmetic shift of byte 255 by two should
preserve 255, while the emitted unsigned shift returns 63.

Keep storage declarations unchanged and reinterpret core unsigned or signed text operands
at INT_SLESS, INT_SLESSEQUAL and INT_SRIGHT. Use the signed integer of the exact
operation width. Restore a shift result to its existing storage type so the
following assignment, call or extension keeps its type. Emit each operand once
and bypass compound assignment when it cannot express the signed operation.
Require core integer types and matching widths. Leave unsigned operators,
non-core aliases, unrelated operations, explicit cast suppression and other
output languages unchanged. This is a correctness fix under existing Rust
output selection, with no new option.

Use eight native Rust functions for <, <=, arithmetic/logical shifts, an unsigned
comparison, a pointer store, a call operand and a shifting loop. Exercise every
byte pair with nonzero upper input bits, loop counts 0 through 31, and a counter
that proves the call operand executes once. Run unchanged emitted bodies against
the native output under three Rust compiler versions. A raw-instruction stage
covers both signed comparisons, arithmetic shifts, storage restoration and an
unsigned control, plus unsigned recovered inputs and compound assignment.
Nine additional raw-instruction functions cover signed comparisons and arithmetic
shifts on unsigned 8-, 16-, 32- and 64-bit primitives, including sign boundaries
and maxima. Execute their exact bytes as an independent oracle and cross-check
three Rust compilers. Compare all affected corpus bodies, retain source metadata,
run the repository gates and measure Rust plus C/Go control timings.
