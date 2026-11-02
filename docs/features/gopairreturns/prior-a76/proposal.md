# Verified Go integer pair results

Add `gopairreturns on|off`, initially off in every mode. It requires
`godwarfsig on`; either option off preserves the existing import and emission
behavior. This is one feature spanning its metadata producer and consumers,
so the draft precedes production implementation.

## Evidence and representation

Support exactly two 64-bit signed or unsigned integer results for amd64 Go
ABIInternal functions with complete, consistent DWARF contracts. Retain the
current producer, origin, result-flag, name, address, location and scalar-input
checks. Result register allocation starts at RAX independently of inputs; the
second result uses RBX. Validate the assigned join's spaces, registers, sizes,
ordering and flags before accepting the signature. Reject ambiguous repeated
function entries and contradictory metadata.

Represent the output as an ordinary 16-byte struct with two scalar fields at
offsets zero and eight, accompanied by a Go pair-result `LangFacet`. The facet
contains stable names or scalar layout facts, never `Rc<Datatype>` values.
Register it only after the full contract passes. Refuse name collisions and
stale or inconsistent layouts. Each consumer checks the complete underlying
shape; the presence of a generated-name prefix is not evidence. A normal
two-field aggregate remains one result.

Keep `FuncProto`, pcode's output count and the metatype enum unchanged. The
existing join and partial-piece machinery carries the ordinary storage.
Narrow integers, floating-point or mixed pairs, aggregate results, three or
more results, stack ABI and stripped inference remain unsupported by this
option. AArch64 support is a later feature.

## Emission

Use the same pair-aware result-type helper for function declarations and
function-pointer types. Local joined values have valid ordinary struct types;
Go has no first-class tuple variable type. Share the existing type factory
through `SpellCtx` for the enabled Go profile so lookup does not copy the
entire facet table for every function.

A statement call assigns both fields of its output from one invocation.
Nested calls need an expression form that evaluates the call once and yields
its ordinary joined storage. A return can project a stable local or forward
a matching pair call directly. An implied expression must not be evaluated
twice to obtain its components. PIECE construction, SUBPIECE extraction,
partial writes and scalar conversions must agree with the same representation.
Preserve operation and variable associations in all emission paths.

A nested call can supply the sole arguments to a function literal that takes
the two scalar results and returns the ordinary struct. This evaluates the
original call outside the literal's parameter scope, avoiding accidental
capture of caller variables with the same names. The inverse adapter takes
one ordinary struct and returns its two fields. A language-level probe over
162 input pairs on Go 1.22/1.27 confirms single evaluation and name isolation;
it is a design check, not a test of the unimplemented emitter.

New emission logic belongs in the P9 `kuna_gopairreturns` module. Extend the
existing Go DWARF importer and the facet representation through narrow hooks.
Update the P1 and P9 specification chapters together with the option row,
generated catalog and shared counter fixtures. C and Rust may show the proven
ordinary struct when both options are enabled; their default output must be
unchanged. Every such enabled change needs review.

## Acceptance

Compile and execute complete emitted callees and callers without repairing
their bodies. Compare against authored Go source for signed and unsigned
boundaries, division and remainder, minimum integer divided by minus one,
divide-by-zero panic behavior, branches, discarded results, forwarding and a
side-effect counter. Check function-pointer signatures and calls when their
result spelling changes. Verify the corresponding ordinary-struct C behavior
where that output is changed.

Use Go 1.22 and 1.27 images, compressed and uncompressed DWARF, and malformed,
conflicting or missing-metadata controls. Exercise all four combinations of
the two options. Unsupported forms and default output remain unchanged.
Capture and review every changed function across the existing whole-image
corpus plus the authored fixtures, in all output languages.

Add an end-to-end stage testcase with both options off/on as appropriate and
all required catalog/corpus count updates. Run the four repository gates,
tooling and lint without changing the original baseline. Measure enabled and
disabled configurations against a matching independent parent with recorded
source, binary and processor-specification hashes. Preserve failed studies;
do not repeat unchanged code to seek a passing result. Default-on is a separate
decision requiring both parity and speed evidence.
