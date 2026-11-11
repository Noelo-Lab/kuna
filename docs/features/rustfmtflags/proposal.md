# Recover static primitive Rust format specifications

Add a `rustfmtflags` P9 option, initially off, for verified Rust 1.93.0/1.93.1
little-endian 64-bit print argument templates. Extend the existing primitive
argument machinery with static width, precision, alignment, fill, sign, alternate
and zero-padding options, plus exact 8-, 16-, 32- and 64-bit integer formatting
trait identities. Pointer-sized integers use the verified 64-bit access width.

The loader must retain full Display, Debug, LowerHex, UpperHex, Octal, Binary,
LowerExp and UpperExp identities before symbol normalization. Preserve signedness
and access width. Conflicting aliases invalidate the address; generic reference
callbacks, pointer formatting and non-integer traits remain unknown. Collect the
facts in one symbol walk and retain the current Display-only behavior when the
new option is off.

Decode the verified template fields with the existing byte, piece and argument
bounds. Require valid Unicode fill, a zero reserved high bit, representable flag
combinations and consistent width/precision presence. Reject indirect width or
precision, unrepresentable fill/alignment combinations, brace fills, unsupported
callback identities and malformed templates. Preserve explicit/repeated indices,
typed reads of original argument pointers, all descriptor setup and surrounding
IR. C output must be unchanged.

Tests must compile recovered macro expressions and compare stdout/stderr with
native Rust 1.93.1 programs covering each accepted trait, signed/narrow values,
static alignment/fill, width/precision and repeated arguments. Negative controls
must cover alias conflicts, generic callbacks, invalid scalars and flags, dynamic
counts and malformed field encodings. Existing primitive-only tests explicitly
keep their narrower option boundary. Add an end-to-end stage testcase, review
all whole-function changes in real and deterministic fixtures, and measure a
before/off/on speed delta. Enable by default only after unchanged original
corpus assertions and the five-percent speed budget pass.

This feature prepares the format specifications needed by angr's `hex_width`
expectation; recovering the `format!` allocation/result call remains a separate
feature. It does not infer the user-defined Debug type used by `print_two`.
