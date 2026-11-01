# Rust keyword machine calls

The x86 IN user operation renders as `in(...)`, which Rust rejects as a keyword.
Other supported architectures use `break()` and `yield(...)`; those names must
remain function calls. The functional CALLOTHER emitter currently copies each
SLEIGH user-operation name without Rust identifier escaping.

Prefix representable strict/reserved Rust keywords with `r#` at that emission
point. Raw identifiers preserve the underlying name. Keep ordinary names,
nonfunctional user-operation displays, and other output languages unchanged.
Rust's forbidden raw names are outside this spelling rule. No type, variable,
namespace, or global-symbol renaming is involved. This is a syntax correction
under the existing Rust output selection, with no new option.

Use real x86 IN/OUT, AVR BREAK, and MIPS YIELD instruction fixtures. Compile and
execute the unchanged bodies against recording stubs to check callee identity,
argument widths, returned low bytes, and single evaluation. This verifies the
emission contract; it does not execute privileged I/O or emulate AVR/MIPS.
Compare all changed corpus functions token by token, allowing only a raw-prefix
insertion at a keyword callee and retaining comments and source addresses. Run
the required gates and measure Rust output plus default C and Go controls.

Rust syntax references: [raw identifiers](https://doc.rust-lang.org/reference/identifiers.html#raw-identifiers)
and [keywords](https://doc.rust-lang.org/reference/keywords.html).
