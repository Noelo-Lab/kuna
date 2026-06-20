# w4-fw-arch-frontends
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Claude Opus 4.8)
date: 2026-06-11
round: 1

gate: `cd rust && cargo test -p kuna-decomp` -> PASS (23 test binaries, all `ok`,
0 failed; corpus_bootstrap 5/5; the new verifier suite
`verify_w4_fw_arch_frontends` 10/10). clippy `-p kuna-decomp` clean.

blob-sha: all six in-scope blobs match the checklist `cpp_blob_sha` exactly
(sleigh_arch.cc cb0848a4, .hh 3ee506bf; raw_arch.cc 48dcc8b1, .hh 8952c581;
xml_arch.cc ff76eac1, .hh 92a72b8a) — no upstream drift.

## Scope

Ported: `LanguageDatabase` (the explicit replacement for the
`description`/`specpaths` statics; the `translators` reuse cache is a documented
non-port, see below), `CompilerTag`/`LanguageDescription` decode + accessors +
`getCompiler` fallback, the `.ldefs` scan
(`loadLanguageDescription`/`collectSpecFiles`/`getDescriptions`/
`scanForSleighDirectories`), `resolveArchitecture`,
`normalizeProcessor/Endian/Size/Architecture`, `buildSpecFile`,
`buildTranslator`, `modifySpaces`, `encodeHeader`/`restoreXmlHeader`, and the
`Xml`/`RawBinary` capabilities + leaf `buildLoader`/`resolveArchitecture`/
`postSpecFile`-tail/`restoreXml` walks. `Architecture::init`/`restoreFromSpec`,
`buildTypegrp`/`buildSymbols`/`buildContext`-decode, and the full `Encoder`
marshaling are explicitly seamed to later waves (`// SEAM(...)` inline). The
inheritance chain is modeled by composition (each leaf owns a
`SleighArchitecture` owns an `Architecture`).

## Mandatory hunt list

- **Signedness.** clean. The two `int4`-vs-`size()` C++ loops (`getCompiler`'s
  `for(int4 i=0;i<compilers.size();++i)`, `resolveArchitecture`'s
  `for(i=0;i<description.size();++i)`) -> Rust `enumerate()` (usize) and a
  `let mut i:int4` walk `while (i as usize) < len`; both reproduce index order
  with no underflow. `getCompiler`'s `int4 defaultind=-1` -> `i64 defaultind`,
  the `!= -1` sentinel and the `compilers[0]` (empty-vec UB) fallthrough are
  faithful (mirrored as a panic).
- **Integer widths.** clean. `LanguageDescription::size` is the lone
  narrowing: C++ `int4 size = decoder.readSignedInteger(...)` truncates int8->
  int4; Rust `read_signed_integer_id(...)? as int4` does the same (carries a
  `// cast:` note). `size` is informational (`getSize()` only). `adjustvma` is
  `long` -> `i64` (== `long` on the 64-bit target). No promotion bugs.
- **Wrapping.** clean. The only wrap-prone op is `parse_auto_base`'s
  `wrapping_neg` for the negative case (correct for `i64::MIN`, matching C++
  two's-complement). No `uintb`-lineage arithmetic in scope (`lastAddr+lastSize`
  lives in the seamed `buildSymbols`).
- **Comparator totality.** n/a — no sort/`operator<`/`partial_cmp` anywhere in
  the three files (grep clean).
- **Iteration-order provenance.** clean. Every loop is over a `Vec` in
  document/registration/insertion order (`description`, `compilers`,
  `truncations`, the scanForSleighDirectories dir lists, the restoreXml child
  slice). No `HashMap`/`HashSet` in scope (grep clean). The `.ldefs` attribute
  decode reads positional-by-id then `getNextAttributeId()`-scans for
  `deprecated`; re-derived against C++ `XmlDecode::readString(AttributeId)`
  (which uses `findMatchingAttribute` and does NOT advance `attributeIndex`),
  so the Rust (also leaving `attribute_index == -1` after by-id reads) re-scans
  all attributes from index 0 identically.
- **Off-by-one / do-while / reverse iterators.** clean. `normalizeSize`'s
  erase-first-"bit"-then-first-'-' verified byte-for-byte vs a g++ oracle
  (incl. `bit-`, `-bit`, `bitbit`, `a-bit-c`). `normalizeArchitecture`'s
  `find(':', curpos+1)` (first search starts at index 1, so a leading colon is
  skipped) + substr boundaries verified vs a g++ oracle (incl. the i==3 default
  append, the i==4 tail keep `a:b:c:d:e:f`->`a:b:c:d:e:f`, and the
  `:LE:64:v`->throw leading-colon case). No do-while idioms; the C++ `for(;;)`
  peek/attribute loops -> Rust `loop {}` with `break` on the 0 sentinel
  (at-least-zero, same as C++).
- **Erase-while-iterating.** n/a — all loops are append-only (`push`) or
  read-only; no erase during traversal.
- **Exception -> Result partial-state parity.** mostly clean; one documented
  observation (O1, below). The `resolveArchitecture` "No sleigh specification"
  throw, the `buildSpecFile` SleighError throws, and `normalizeArchitecture`'s
  LowlevelError are all faithful `Err` returns at the same points with the same
  text. `getDescriptions`'s `if (!s.str().empty()) throw` -> `if !s.is_empty()
  { return Err }` is faithful.

## Mechanical pass

- grep (sleigh_arch.rs/raw_arch.rs/xml_arch.rs): no `todo!`/`unimplemented!`,
  no `unwrap`/`expect`/`panic!` in non-test code, no `HashMap`/`HashSet`, no
  `sort`/`sort_unstable`, no `partial_cmp`. Bare `as` casts are all
  index/length conversions on non-negative bounded values (`i as usize`,
  `len() as int4`) plus the one semantic `int8->int4` with a `// cast:` note —
  consistent with the codebase convention (cf. w4-fw-options/architecture
  reviews).
- `cargo clippy -p kuna-decomp -- ` clean (no warnings).
- gate `cargo test -p kuna-decomp` green (corpus_bootstrap end-to-end over 8+
  corpus XMLs across architectures: ldefs decode -> resolveArchitecture ->
  buildSpecFile -> buildTranslator(.sla) -> binaryimage load -> FlowInfo).

## Findings

- F1 (minor): `parse_auto_base` (the `adjustvma` save-file attribute parse)
  returns 0 for any string with trailing non-digit garbage, where C++
  `istringstream >> long` with `unsetf(dec|hex|oct)` (and `strtoll(.,0,0)`)
  return the parsed PREFIX. Verified vs a g++ oracle: `"5x"` cpp=5/port=0,
  `"0x1fZ"` cpp=31/port=0, `"0789"` cpp=7/port=0. The doc comment claiming it
  matches `strtoll(s,0,0)` is incorrect. Clean integers (the only values real
  save files carry — `encode` writes pure decimal via `writeUnsignedInteger`)
  all match. Output-affecting only on malformed/hand-crafted attributes; no
  in-scope datatest/corpus fixture triggers it. Accepted as **LOSS-051**.
       cpp: decompiler/cpp/xml_arch.cc:124-128 (raw_arch.cc:105-109)
       rust: rust/crates/kuna-decomp/src/xml_arch.rs:307-325
- F2 (minor): `XmlArchitectureCapability::isFileMatch` skips leading whitespace
  with `is_ascii_whitespace()`, whose set omits vertical tab (0x0b); C++
  `std::ws` (classic-locale `isspace`) skips `\v`. A `<binaryimage>` file
  beginning with `\v` matches in C++ but not the port. The other five
  whitespace bytes (space/`\t`/`\n`/`\r`/`\f`) are handled identically. XML
  save files never begin with `\v`; `isFileMatch` is a best-effort probe (raw
  is the universal fallback). Accepted as **LOSS-052**.
       cpp: decompiler/cpp/xml_arch.cc:46-57
       rust: rust/crates/kuna-decomp/src/xml_arch.rs:94

## Observations (non-blocking)

- O1: `LanguageDatabase::load_language_description` wraps the
  `<language_definitions>` body decode in `if ...is_err() { errs.push_str(...) }`,
  converting a malformed-`<language>` decode error into a generic `errs`
  WARNING string. C++ has no try around that body, so a malformed `<language>`
  THROWS a `LowlevelError` (with the decode's specific text) straight out of
  `loadLanguageDescription`/`collectSpecFiles`. Net effect through
  `getDescriptions` is still an `Err` (errs becomes non-empty -> LowlevelError),
  but the message text differs, and the direct `collect_spec_files` path
  (called from leaf `buildLoader` as `collectSpecFiles(*errorstream)`) swallows
  it rather than aborting. The porter documents this inline (sleigh_arch.rs
  :460-467). Trigger is a malformed vendored `.ldefs` (a packaging error, not
  normal input); the vendored specs are well-formed. Not output-affecting for
  valid specs; recorded as an observation rather than a loss.
- O2 (carried, not this item): the C++ `static map<int4,Sleigh> translators`
  `.sla` reuse cache is intentionally not ported (`is_translate_reused()` always
  false; each build decodes a fresh `.sla`). Build-speed only, never output —
  documented in the module doc; faithful for output.

## Adversarial tests

Added `rust/crates/kuna-decomp/tests/verify_w4_fw_arch_frontends.rs` (10 tests,
all green, committed to the branch):
- `adjustvma_clean_decimal_hex_octal_signed_match_cpp` — clean inputs == oracle
- `adjustvma_trailing_garbage_diverges_from_cpp` — pins F1 (LOSS-051)
- `adjustvma_negative_zero_and_plus_prefix`
- `is_file_match_skips_classic_whitespace_then_matches_bi`
- `is_file_match_vertical_tab_diverges_from_cpp` — pins F2 (LOSS-052)
- `normalize_architecture_five_fields_extra_colon_keeps_tail`
- `normalize_architecture_leading_colon_is_skipped_and_can_throw`
- `normalize_architecture_three_fields_appends_default_compiler`
- `get_compiler_default_in_middle_is_preferred_over_first`
- `xml_restore_out_of_order_specext_before_binaryimage_stops_walk`

## Losses

LOSS-051 (parse_auto_base trailing-garbage truncation), LOSS-052 (isFileMatch
vertical-tab whitespace). Both minor, on practically-unreachable malformed-input
paths; clean/real inputs are byte-faithful and the corpus_bootstrap gate passes
end-to-end. No blocker or major finding stands -> ACCEPT-WITH-LOSSES.
