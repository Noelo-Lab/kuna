# Verified Rust format-result rendering

Add one P9 `rustfmtresult` option, initially off, at the existing direct-call
rendering hook. Recognize only exact fully-qualified or scoped
`alloc::fmt::format::format_inner` calls in verified stable Rust 1.93.0/1.93.1
little-endian 64-bit images. Require the hidden destination plus template and
argument-array inputs, no call result, and a word-sized entry output pointer.
Reuse the current `rustfmtargs` template/descriptor/type proofs and its
`rustfmtflags` extension. Keep the parent argument option as an explicit prerequisite.

Render `std::ptr::write(destination as *mut String, format!(...))` to preserve
the lowered output-buffer ABI without dropping an uninitialized String. Preserve
all descriptor setup, all IR, the caller's existing return and every terminal
newline in the format string. Do not rewrite signatures, infer generic callback
types, recognize unrelated wrappers or guess a returned call value. C is unchanged.

Factor only the existing argument-plan construction and macro-style selection so
print and result calls share the exact validation. The new module owns the sink
identity and output-pointer proof; PrintC emits the nested write and format call.

This is a small feature under the repository's scope rules: one production module,
one existing emission hook, two ported core anchors (PrintC and Architecture), no
new pass type, ABI model or phase infrastructure. The template helper is an
ordinary extraction of existing P9 logic.

Compile recovered expressions against native Rust1.93.1 Strings using
MaybeUninit storage, compare contents including terminal LF, and drop exactly one
initialized String. Add real and hermetic negative controls for wrong namespaces,
wrong call shape, invalid destinations and unsupported formatter/template cases.
Add a stage off/default testcase, update counters/catalog/stage baseline, measure
speed and full changed bodies, and run all four required checks before committing.

Native inspection showed a local String destination split into three separate
primitive declarations. Keep those calls explicit until complete String storage
is recovered; this feature accepts only non-spacebase entry destinations.
