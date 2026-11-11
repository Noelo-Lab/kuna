# Rust signed primitive operators

Rust spells recovered signed byte storage as `u8`. A signed comparison or arithmetic
right shift must interpret those bits as signed before operating on them.
For byte 255 versus zero, the old emitted comparison returns false instead of
true; shifting byte 255 right by two produces 63 instead of preserving 255.

Recovered unsigned primitives can also feed signed machine operations.
The correction adds exact-width signed interpretations at INT_SLESS,
INT_SLESSEQUAL and INT_SRIGHT. Shift results return to their existing storage
type. Declarations and shift counts stay unchanged, and each input is evaluated
once. Compound assignment is declined when unsigned storage cannot perform the
signed operation. Core type and width checks bound the plan; unsigned operators,
non-core aliases, other output languages and explicit cast suppression retain
their behavior. This fixes existing Rust output without adding an option.

Eight native functions cover comparisons, arithmetic and logical shifts, stores,
a call operand, loops and unsigned controls. Each tests all 65,536 byte pairs,
with nonzero upper input bits and loop counts through 31. Three fresh producer
images each supply unchanged emitted bodies executed under Rust 1.90, 1.93.1
and 1.99.1. All 4,718,592 returned results match native execution, with 589,824
checks that the call operand executes exactly once. The old bodies compile but
disagree on 32,768 rows for each of five signed cases and 31,744 loop rows per
producer. Both unsigned controls remain correct. Overflow checks are explicitly
disabled for the execution comparison.

An additional in-place loop uses the exact native x86-64 fixture bytes as an
execution oracle. Both signed and unsigned recovered input types match all
8,192 byte/count inputs under each compiler; the old bodies differ on 3,968.
Nine further raw functions cover comparisons and shifting loops at 8, 16, 32
and 64 bits. All 71,824 pairs include every byte pair and each width's sign
boundary and maximum. Their exact machine bytes agree with three Rust source
oracles. Unchanged corrected bodies match all 1,939,248 returned results across
three compilers. The old complete programs fail type checking on three wider
shift assignments because their signed results are not converted to unsigned
storage. Wider comparison controls retain their existing correct form.

The eighteen-image comparison covers 25,339 distinct
functions and 50,733 records. All 41,461
C/Go records are identical, and 16,122 Go bodies parse.
The 255 changed Rust bodies add 10,174 integer casts
across 1,149 hunks. Independent AST comparison retains every
original cast, all other expression structure, declarations, literals and
comments. Ten unchanged `_::` prefixes receive identical parser-only repairs;
the emitted code is untouched by that verifier. Instruction address sets and
per-variable provenance are unchanged. Three added i64 casts in two C-source functions already declaring signed
variables are redundant. One u32 cast restores a signed shift result to its
unsigned destination; raw-instruction execution separately tests that case.
The standalone partial type audit introduces no new selected diagnostic
messages or compilation failures. Existing unresolved names and recovery gaps
prevent a claim of whole-program compilation.
Baseline: `394d3149f8305c0c458956ab53eec102994a88b0`.

All 10,066 workspace tests pass, with 39 ignored across 71 suites.
Original assertions remain 675/675; stages pass 3,502/3,502, with the new stage
improving from 2/8 to 8/8. Tooling, lint, spec, catalog and counter checks pass.
The original baseline is unchanged.

Four timing cases cover default C, a native signed-byte comparison, a Rust
runtime function with 396 added casts, and Go. Each uses 31 alternating pairs
after three warmups per arm, after local builds and scans finish. Maximum
measured changes are +2.07% CPU and +2.12% wall,
within the 5% budget. Raw samples are retained.

The heavy case was repeated once after the initial run had wall-only pauses
in both arms, ranging up to 379 ms. That run measured +5.50% by the ratio of
wall medians but +0.12% by the median of paired changes, with +0.46% CPU.
After a fresh idle check, the repeat measured +0.66% wall and +0.64% CPU;
its largest wall-minus-CPU gap was below 1.9 ms. No code changed between
runs, and every initial sample is retained in the timing noise review.
