# Rust extension review

The production change is confined to Rust integer emission: a twelve-line
sign-extension branch, a bounded signed-text-width helper, and a language
capability guard on C's implicit-extension shortcut. The input is pushed once.
Text storage and declared pointer types retain their previous spelling.
The helper refuses non-core text, unsigned input, non-integer outputs, unsupported
widths and non-widening shapes. Native execution checks sign interpretation and
narrow overflow independently of the implementation.

All 481 changed Rust bodies are compared as parsed expression trees. Added casts
retain every original cast and every non-cast AST field, including operator
structure, declaration types and literal values. Original comments also match.
The two explicit parser-only identifier repairs are counted identically in both
inputs; emitted bodies retain those existing defects. All 41,461 C/Go records
are identical at the function-record level. All 16,122 Go bodies parse.

Source-position metadata changes when a hidden operation becomes an explicit
cast. All 71 added source-address entries are original decoded instructions:
byte/word zero extensions, 32-bit register writes or arithmetic, and a boolean
SETE. No prior source address is lost. Every variable's name, declared type,
kind, argument index, stack offset and size is unchanged. Three already-expanded
recovered bodies span beyond their symbols; those cases retain their original
recovered extent and are recorded in the comparison.

The standalone partial Rust type audit reduces affected function records from
425 to 384 and selected messages from 15,133 to 14,002. No new function acquires
these diagnostics, and no previously compiling function stops compiling.
Two already-invalid quicksort bodies gain messages at boolean arithmetic:
the original i64 + bool is rejected; explicit boolean-to-u64 conversion reveals
the existing signed/unsigned addition gap. Exact original expressions, unchanged
boolean declarations and original compiler diagnostics are retained in
type-review.json. This is not a claim of whole-program Rust compilation;
existing unsupported-control-flow markers and other recovery gaps remain.

Three native compiler inputs each execute the six unedited emitted bodies under
three Rust compilers. All 85,212 results match their producer's native oracle.
Overflow checks are disabled explicitly, matching the original wrapping code.
The before signed-byte body alone compiles but disagrees on 798 of 1,578 rows
for every producer; the before six-body program also fails type checking.

Both comparison arms and all gates were rerun against main889c. The intervening
main change affects the same 42 function records on each arm; an independent
comparison proves that all 84 records differ only by consistent bijective
renaming of generated structs, with all other fields preserved. The Rust
extension comparison remains against the fresh main baseline.

All required gates pass, including 10,050 workspace tests. Four 31-pair CPU77
timing cases run after this task's builds, full-image scans and compiler audits
finish; no own build/scan remains. The direct heavy-function witness has 34 added
casts in both the full scan and exact single-function timing command. Maximum
increases remain below 5%.
