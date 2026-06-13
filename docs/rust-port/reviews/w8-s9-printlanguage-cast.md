# w8-s9-printlanguage-cast
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent verifier)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> lib 1403/1403 pass (incl. all cast + printlanguage unit tests and the 10 new verifier tests). One integration test `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` FAILS — environmental only: it needs a built `.sla` (gitignored build artifact, absent in the worktree); the test file is untouched by this diff and unrelated to this item. `cargo clippy -p kuna-decomp -- -D warnings` clean.

## Scope / provenance
- cpp blob shas: all six modules match the checklist `cpp_blob_sha` exactly (printlanguage.cc/.hh, cast.cc/.hh, printjava.cc/.hh).
- Rust diff (rust-port...worktree-wf_6d02cead-117-1, rust/): cast.rs (+1167), cast/tests.rs (+663), printlanguage.rs (+938), printlanguage/tests.rs (+295), printjava.rs (+48 placeholder).
- Re-derivation: the **pure** halves (cast.cc lattice methods, printlanguage.cc `parentheses`/`mostNaturalBase`/`formatBinary`/`unicodeNeedsEscape`/`unnamedField` + mod/scope state) are transcribed line-for-line and are testable now. The **emitter/IR-walking** halves of printlanguage.cc (`pushOp`/`pushAtom`/`recurse`/`emitOp`/`emitAtom`/`opBinary`/`opUnary`/`emitLineComment`) are explicitly seam-noted (depend on the stub `Emit` driver + `PrintC` virtuals, both separate W8 items) — not transcribed against stubs. The IR-walking cast methods are ported behind a `CastContext` trait (mirroring `variable::HighContext`), control-flow transcribed verbatim.

## Wave hunt (PrintC token-emission order / cfmt formatting / CastStrategy decisions / universalAction order)
- **PrintC token-emission order**: the `parentheses` decision (printlanguage.cc:270-327) is transcribed token-for-token across all six `tokentype` arms incl. the kuna GH-2786 adjacent-sign fix; pointer-identity `topToken==op2` -> `std::ptr::eq`; HiddenFunction `revpol.size()>1` -> `prev_token.is_some()`. Verified by 2 new tests + 8 ported. The token-*pushing* order (`pushOp`/`pushAtom`/`opBinary`/`recurse`) is the seam driver — out of scope here.
- **cfmt number formatting**: `mostNaturalBase` (uintb `%10`/`&0xf` runs + the `countdec` tie-break switch) and `formatBinary` (8/16/32/64 width clamp) transcribed exactly; verified at the switch-arm and width boundaries. `%g`/precision/hex float formatting lives in cfmt/printc, not this item.
- **CastStrategy getInputCast/getOutputCast decisions**: `castStandard` (C + Java), `arithmeticOutputStandard`, `isSubpieceCast*`, `isSext/ZextCast`, the promotion ladder (`localExtensionType`/`intPromotionType`/`checkIntPromotionFor*`/`isExtensionCastImplied`), `markExplicit{Unsigned,LongSize}` all present and faithful; the `getInputCast`/`getOutputCast` callers are W9 (typeop) — seam-noted.
- **universalAction registration order**: N/A — no action/rule registration in these six modules.

## Mandatory hunt list
- Signedness: clean. `markExplicitUnsigned` `numInput()==2` / `1-slot` (int4) faithful; no int4-vs-container-size loops other than `arithmeticOutputStandard`'s `i<numInput()` (both int4) — matches C++.
- Integer widths: clean. int4/uint4/uintb map to i32/u32/u64. `markExplicitLongSize` guards `promoteSize*8-1` (=31) and `promoteSize*8` (=32) reproduced exactly (i32 arithmetic). `getHighTypeReadFacing` vs `getHigh()->getType()` distinction preserved (read-facing for markExplicitUnsigned/arith; plain getHigh for markExplicitLongSize).
- Wrapping: clean and verified. The single wrap site, `isSubpieceCastEndian` `intype->getSize()-1-offset` computed in uint4 (cast.cc:437-439), is `(get_size() as uint4).wrapping_sub(1).wrapping_sub(offset)` with a `// cast:` justification — reproduces the C++ unsigned wrap (offset>=size -> 0xFFFF...). New test `subpiece_cast_endian_bigend_boundary_and_wrap` covers offset==size and offset>>size with no panic.
- Comparator totality: N/A — no `operator<`/sort in scope (`type_order` returns a deterministic int4 from W6; not introduced here).
- Iteration-order provenance: clean. The only ported loops are over fixed input slots (`op->getIn(i)`, index order) and digit/bit shifts; no HashMap/HashSet anywhere in the ported code (grep clean). The `PrintLanguageCapability::thelist` vector (default-at-front insertion order) is in the seam driver, not ported here.
- Off-by-one / do-while / reverse iterators: clean for the ported surface. The C++ `do{}while` (pushAtom) and erase-style stack pops are all in the seam driver. `mostNaturalBase`/`formatBinary` while-loops verified at boundaries.
- Erase-while-iterating: N/A — none in the ported functions.
- Exception->Result partial-state parity: clean. `arithmeticOutputStandard`/`setLineCommentIndent`/`setIntegerFormat` map C++ throws to `Err`/`?` at the same points; `popMod` panics on empty (C++ `back()` on empty is UB) with balanced-by-caller invariant — acceptable.

## Mechanical pass
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`: none in cast.rs/printlanguage.rs/printjava.rs.
- bare `as`: two, both justified — `self as int4` (IntPromotionCode discriminant -> raw value, doc'd at cast.hh:48-54) and `intype.get_size() as uint4` (carries the inline width-justification comment, cast.rs:893-895).
- clippy `-p kuna-decomp -- -D warnings`: clean (lib + new test file).

## Findings
- F1 (loss, not a blocker): the entire `printjava.cc` emitter is deferred. `PrintJava::new()` returns `Err`; none of `printUnicode`/`pushTypeStart`/`isArrayType`/`needZeroArray`/`opLoad`/`opStore`/`opCallind`/`opCpoolRefOp`/the `instanceof` token/`PrintJavaCapability` are ported. The checklist title names "PrintJava". Justified: `PrintJava : public PrintC` and `w8-s9-printc` is still `todo`, so the base class does not exist in Rust yet; and no datatest selects `java-language` (verified empty), so it is off every parity path. The Java *cast* surface (`CastStrategyJava`) IS ported. Recorded as **LOSS-115**.
       cpp: decompiler/cpp/printjava.cc (whole file)
       rust: rust/crates/kuna-decomp/src/printjava.rs:36-47
- No blockers or majors. The two pure-and-IR-walking halves that are present are faithful line-for-line transcriptions; the seam-noted emitter methods are correctly identified as depending on not-yet-landed W8 items (`Emit` driver / `PrintC`) rather than mis-ported against stubs.

## Adversarial tests added
rust/crates/kuna-decomp/tests/verify_w8_s9_printlanguage_cast.rs (10 tests, all pass):
- subpiece_cast_endian_bigend_boundary_and_wrap (width/uint4-wrap)
- mark_explicit_long_size_negative_int_boundary (signbit `>=31` off-by-one)
- mark_explicit_long_size_unsigned_boundary_and_size_guard (`>=32` + size<=promote guard + non-shift)
- most_natural_base_tie_break_boundaries (countdec switch arms + u64::MAX)
- format_binary_width_boundaries (msb 7/8/15/16/63 width edges)
- unicode_needs_escape_range_boundaries (0x1f/0x20/0x7e/0x7f/0xa0/0xa1/0x2fa1f/0x2fa20/0xd7fb/0xd7fc)
- parentheses_hiddenfunction_equal_precedence_keeps_parens (the `<`-guard fall-through)
- parentheses_binary_associative_pointer_identity (`topToken==op2` identity vs distinct)
- cast_standard_pointer_chain_forces_care_uint_int (care_uint_int reset ordering + care_ptr_uint/PTR arm)
- int_promotion_code_discriminants_match_cpp (the load-bearing -1/0/1/2/3 + bit-algebra)

## losses
LOSS-115 (PrintJava emitter deferred).
