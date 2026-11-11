An implied struct field projection could discard the width of its SUBPIECE.
Returning that field through a narrower C return type masked the omission, but
widening it again retained unwanted upper bits. Go and Rust instead rejected
many of these expressions because the field and result widths differed.

The shared emitter now preserves the selected integer width with an explicit
conversion, after shifting interior field bytes into position when necessary.
The shift follows storage byte order, and the field expression occurs once.
The existing nocastprinting diagnostic option suppresses the conversion.
The new helper declines noninteger, enum, volatile, bitfield, incomplete and
out-of-range cases. Explicit-symbol accesses retain their existing path.

A source-authored fixture exercises 26 complete callers. Each C and Go caller
matches the source across 784 input pairs per run; C is checked at O0/O2 and
Go with versions 1.22.5 and 1.27.3. Twenty-one Rust callers pass at O0/O2,
including all 18 callers changed by this fix. Five unchanged Rust callers still
lack signedness conversions; their full failed compilation is retained, and
none is claimed to pass. The existing callback fixture also passes all eight
complete C and Go bodies, including the formerly ill-typed narrowed callback.
Its Rust callback retains preexisting keyword/function-pointer errors; only its
field extraction changes, and no native Rust callback claim is made.

The native proofs keep every emitted caller body intact and supply source-
equivalent typed pair-producing helpers. Each call is counted. Final captures
verify that the tested body records match the final release build exactly.
A byte-oracle unit test covers every contained 1/2/4/8-byte slice of eight-byte signed and
unsigned fields under both byte orders, plus conservative rejection controls.

Code review caught an initial failure to honor nocastprinting. The prior
binary's counterexample and source are retained; incomplete validation was
stopped, two suppression assertions were added, and the corrected source was
validated again. The original 675-assertion baseline was not changed.

Final breadth: 61 images, 34416 distinct functions, 77964
language records. There are 57 changed records: 18 direct callers in
three languages and one narrowed callback in three languages. All 59 other
images have identical function records. Every changed hunk is classified in
breadth-review.json.gz. Fifteen inherited Go parser failures are unchanged.

Timing uses nine cases, 384 balanced paired rounds and one final look. All raw
samples and simultaneous CPU/wall bounds are retained. Maximum median change
is 0.194%, maximum upper bound 3.597%. Every comparison meets the predeclared 5% criterion.
