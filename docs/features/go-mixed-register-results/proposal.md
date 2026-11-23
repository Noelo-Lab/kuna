# Recover one declared AMD64 Go mixed result shape

Extend `godwarfsig` to accept a unique complete little-endian AMD64 Go
ABIInternal declaration of exactly `(int64, float64)`. Begin with that order
and those exact built-in type identities. Require the normal Go producer and
register-ABI provenance, validated inputs, and two complete eight-byte
result formals. For each present result location, accept the exact register
or an empty compiler location expression. The latter uses the complete
ABIInternal declaration to supply placement. Decline absent location
attributes, incomplete or conflicting DIEs, stack and ABI0 cases, additional
results, and other type combinations. Without usable DWARF the ordinary
analysis remains in effect.

The integer and float banks allocate independently. Use the existing
custom return-storage path to bind one locked 16-byte output to the exact
RAX and XMM0 ranges. Require distinct eight-byte architectural storage.
The first source result is the low word at offset zero and the second is the
word at offset eight. Keep the generic compiler spec's integer result
class. Share construction and verification of the explicit join with the
named two-float result feature once its implementation is available. Each
shape retains its own admission proof and logical field types.

Carry the certified struct through existing address-bound prototype and
call machinery. Reuse Go multiple-result declaration and emission, with
compatibility checks on both the ordered field types and active output join.
Preserve one producer evaluation when both values are extracted, one is
ignored, or the complete pair is forwarded. The float projection is a
float64 value. Its integer representation is a bit transfer when needed.
A complete all-zero carrier should emit a typed zero tuple. Existing
explicit `prototype`, `return`, and `callproto` assertions determine the
result and storage wherever present. An overridden or missing output join
cannot qualify for mixed tuple rendering merely because the type name is
still in the type factory.

Test the actual emitted definitions and callers on Go 1.22.10 and 1.27.2
with the early zero arm, signed-minimum divided by −1, non-integral results,
negative and rounded float values, and one-result consumers. Require the
same results in both normal decompile entry points. Test conflicting,
absent, and valid empty result locations separately, with stripped and
disabled cases and explicit scalar storage at the definition and a call.
Retain ordinary scalar and integer-pair behavior, C rendering of the
ordinary typed carrier on a Go input, and off-target C and Rust controls.
Measure default-on cost on named Go input and an unaffected control.

A per-declaration register join is reusable implementation support for
other shapes. This proposal admits one AMD64 result shape. Float-first,
unsigned, machine-sized `int` spelling, strings, AArch64, and stripped
inference need additional proof and should be reviewed separately.
