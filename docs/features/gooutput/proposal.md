# Go output language on the existing emission plane

Pinned angr exposes a Go flavor and tests Go signatures, declarations, operators,
loops, switches and complete statement coverage. Kuna already recognizes Go
images and recovers runtime metadata, but its output-language registry contains
only C and Rust. Even `main.add` cannot be requested in Go syntax.

Extend the existing OutLang/LangProfile/TypeSpeller plane with Go. Keep the shared
RPN printer, IR and structured blocks. A new P9 `kuna_langgo.rs` owns the Go
profile, precedence tokens, type spelling and language-specific PrintC methods;
existing dispatch sites select those methods. Do not fork the C printer.

Expose `--language go` and `option setlanguage go-language`, plus a cataloged
`outlang c|rust|go` alias to the same selector so agents can discover and flip
this decision. The engine default remains C and CLI auto-selection retains its
current policy. Explicit Go output works for any source language, like Rust
output. The C-only project exporter continues to reject non-C documents.

The implementation includes func signatures, name/type parameters and returns,
var declarations, Go scalar/pointer/array type spelling, nil, valid character
and string escapes, conversion expressions, unary/binary operator precedence,
if/else, all loop shapes, switch cases/fallthrough, break/continue/goto/labels and
function calls. Definitions use a legal Go identifier with the original qualified
symbol retained in a comment, following the existing Rust output convention.
It must preserve every represented operand and side effect.
Go's operator tiers differ from both C and Rust; equal-tier right operands and
non-associative nesting require explicit parentheses. Boolean XOR uses inequality,
not the integer-only bitwise XOR operator. Go semicolon insertion requires opening
braces and else clauses to stay on their required lines regardless of C formatting
preferences; switching flavors must not mutate those persistent preferences. Pointer/integer conversion
must carry unsafe.Pointer/uintptr rather than inventing a legal direct cast.

Go forbids ternary and comma expressions. Disable the optional ternary rendering
of recovered if blocks for Go so the existing branches remain statements; residual
expression forms need an evaluation-order-preserving Go encoding, not a placeholder
or dropped operand. Bottom-tested loops must keep continue
edges testing the original condition. Switch fallthrough is explicit in Go,
and case labels with a shared body are grouped. Declarations stay before gotos
so no new jump crosses a variable initialization.

Use recovered source type identities when sound, otherwise width-correct scalar
or storage types. This emission foundation does not invent missing multi-result
ABIs, synthesize string/slice/interface semantics, remove runtime calls, or infer
receiver signatures. Those remain distinct parity features. Unknown machine
operators retain named helper calls with all operands.

This is a large feature because it extends language infrastructure and several
emission forms. Open a draft proposal before implementation. The user's standing
instruction authorizes completing and merging each feature after validation.

Validation: native Go 1.22.5 fixtures and hermetic stage inputs; execute compiled
recovered expressions/functions for arithmetic, conversion and control-flow
cases; cover the pinned angr operator-precedence cases and compare supported
Go spelling/statement behavior. Whole-binary C and Rust controls must remain
identical. Inspect all changed Go output bodies, keep negative/malformed-type
controls, measure explicit Go versus baseline C and unchanged default throughput,
and run all repository gates before each commit.

Primary semantic reference: https://go.dev/ref/spec (operators, conversions,
statements, switches and loops). Reference tests: pinned angr
6afa0c79a83d9f677a46b0275cdb63bf16c1979e,
`test_go_codegen_handlers.py` and `test_go_decompiler.py`.

Implemented and validated; final behavior, evidence and limits are in `analysis.md`.
