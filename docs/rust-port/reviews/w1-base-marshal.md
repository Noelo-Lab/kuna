# w1-base-marshal

## Round 1

verdict: REJECT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_cea42066-041-13 (worktree `.claude/worktrees/wf_cea42066-041-13`, reviewed at 325cbc7; adversarial tests committed as ec80296)
cpp blob shas: both in-scope files match the checklist entry (`git hash-object` verified: marshal.cc 66b15623, marshal.hh 9817f7b0)

gate: `cd rust && cargo test -p kuna-base` -> **FAILS** after the verifier
adversarial tests land: 2 of the 8 new tests pin finding F1 and fail against
the current port (`verify_marshal_indexed_attr_dec_suffix_matches_cpp_oracle_f1`:
got id 31, C++ oracle gives 33; `verify_marshal_indexed_attr_wraps_like_cpp_on_huge_index_f1`:
panics "attempt to add with overflow" at `marshal.rs:1136`).  As delivered
(before the verifier tests) the gate was green: 89 lib + 1 golden_addrsort +
6 foundation-verify + 5 xml-verify (+1 ignored corpus dumper) + 5 xml-r2-verify.
clippy: `cargo clippy -p kuna-base --tests -- -D warnings` -> clean.
port-audit: `cargo run -p port-audit -- --item w1-base-marshal` still not
runnable — no `port-audit` crate exists (carried note, F6); blob SHAs checked
manually.

## Method

Re-derived all of marshal.hh (753 lines) + marshal.cc (1277 lines) —
AttributeId/ElementId registration, the Decoder/Encoder virtual surfaces,
XmlDecode/XmlEncode over the DOM, the PackedFormat byte protocol, and the
PackedDecode chunk-list machinery (inline getBytePlus1/getNextByte/
advancePosition, endIngest padding) — and diffed against
`rust/crates/kuna-base/src/marshal.rs` (+2414 lines on the branch).

Built two C++ oracle probes from the vendored source (g++ -std=c++11,
/tmp/kuna-marshal-verify, transcribing the exact statement sequences):

1. `XmlDecode::getIndexedAttributeId`'s suffix parse
   (`istringstream >> dec >> uint4` + `getId() + (val-1)`), 13 suffixes —
   this exposed F1 (see findings).
2. The `readSignedInteger`/`readUnsignedInteger` parse core
   (`istringstream` with `unsetf(ios::dec|ios::hex|ios::oct)`), 22 cases
   including +/-2^63, +/-2^64-1, 2^64, 22-digit overflows, "0x", "0xg",
   "08", "+0x10" — `cxx_parse_signed`/`cxx_parse_unsigned`
   (marshal.rs:396-481) matched the libstdc++ oracle on **all** probes
   (clamping to i64::MAX/MIN, u64::MAX saturation, modular negation of
   unsigned magnitudes, base-0 prefix detection, endptr placement).

The PackedEncode `writeInteger` length-code ladder (marshal.cc:1069-1133) was
verified literal-by-literal (0x80/0x4000/0x200000/0x10000000/0x800000000/
0x40000000000/0x2000000000000/0x100000000000000/0x8000000000000000) against
marshal.rs:1982-2032, and the packed byte protocol checked against the
porter's exact-byte probes plus new chunk-boundary tests.

## Hunt list

- signedness: **clean.** `attributeIndex` int4 -> i32 with the -1/1000
  sentinels preserved; `nextIndex < getNumAttributes()` is i32-vs-i32 on both
  sides; `advancePosition`'s ptrdiff_t-vs-uint4 comparison (marshal.hh:637)
  transcribed with a commented u64 widening (marshal.rs:1447-1449);
  `readSpace`'s uint8-vs-int4 `numSpaces()` comparison (marshal.cc:1010)
  commented at marshal.rs:1898-1899.
- integer widths: **F1** (the `uint4 val` suffix handling); otherwise clean —
  string-skip lengths reproduce the C++ uint8->uint4 truncation
  (marshal.cc:653,669 -> marshal.rs:1531,1549), `writeHeader` reproduces the
  uint1 |= uint4 truncation (marshal.hh:666 -> marshal.rs:1966),
  `readInteger`'s shift-off-the-top of u64 matches C++ unsigned shl.
- wrapping: **F1** (`get_id() + (val - 1)` bare add, panics in debug where
  C++ uint4 wraps) and **F3** (note: `write_string_indexed`'s `index + 1`).
  Signed-integer encode/decode negation uses `wrapping_neg` with comments on
  both sides (i64::MIN round-trips; pinned by the porter's boundary tests).
- comparator totality: **clean.** No comparators or sorts in scope;
  `PartialEq` by id mirrors the C++ `operator==` family exactly.
- iteration-order provenance: **clean.** XmlDecode children/attributes are
  Vec index order (C++ `List`/attribute vectors, insertion order);
  `in_stream: Vec<Vec<u8>>` preserves the C++ `list<ByteChunk>` append
  order; `IdRegistry` BTreeMap is keyed-lookup-only (no iteration); no
  HashMap/HashSet (grep + clippy clean).
- off-by-one / do-while / reverse iterators: **clean.**
  `closeElementSkipping`'s do-while becomes a while over a stack seeded
  non-empty (at-least-once preserved, marshal.rs:1681-1696);
  `getBytePlus1`/`getNextByte`/`advancePosition` boundary conditions
  (`ptr == end`, `current != end` post-increment, `<=` chunk-hop) are
  transcribed exactly and the cross-chunk paths are pinned by the new
  straddle/multichunk tests; `endIngest`'s bufPos==BUFFER_SIZE extra-chunk
  path pinned by the porter's bufferpad test (1024-byte exact fill).
- erase-while-iterating: **n/a.** Nothing in scope erases during traversal.
- exception -> Result partial-state parity: **clean.** Every PackedDecode
  type-mismatch path performs `skipAttributeRemaining` + `attributeRead =
  true` *before* erroring exactly where C++ does (read_signed/unsigned/
  string/space); `readBool` sets attributeRead before its check (as C++);
  `readSpace`'s "Invalid/Unknown address space index" errors leave
  attributeRead untouched like the C++ throw sites; find-by-id failures
  leave curPos mid-scan on both sides; `XmlDecode::openElement(ElementId)`
  advances the child iterator / consumes the root *before* the name-check
  throw on both sides; NUL-terminated `ingestStream` semantics
  (`istream::get(...,'\0')` + `peek() > 0`) reproduced and pinned by test.

## Findings

- F1 (**major**): `XmlDecode::getIndexedAttributeId` does not reproduce the
  C++ suffix parse or its wrapping id arithmetic.  C++ parses the
  name-suffix with `istringstream >> dec >> val` — decimal ONLY — into a
  `uint4` (num_get: overflow stores UINT_MAX, a `-` negates modularly), then
  returns `attribId.getId() + (val-1)` in well-defined wrapping uint4
  arithmetic.  The port reuses `cxx_strtoull_core` (base-0 strtoul
  semantics: 0x -> hex, leading 0 -> octal), saturates negatives to
  u32::MAX, and uses a bare non-wrapping add.
       cpp: decompiler/cpp/marshal.cc:243-260
       rust: rust/crates/kuna-base/src/marshal.rs:1108-1137 (add at 1136)
       claim (port): suffix parsed like strtoul(,,0); id add cannot overflow.
       observed (C++ oracle, g++/libstdc++, base id 24 = ATTRIB_VAL):
         "010"  -> C++ id 33 (decimal 10)   vs Rust id 31 (octal 8)  — silent wrong id
         "08"   -> C++ id 31 (decimal 8)    vs Rust LowlevelError    — success/error flip
         "0x2"  -> C++ LowlevelError (val 0) vs Rust id 25 (hex 2)   — error/success flip
         "4294967296" -> C++ id 22 (UINT_MAX, wrapped add) vs Rust **panic**
           "attempt to add with overflow" (debug build; silently matches in release)
         "-5"   -> C++ id 18 (val 4294967291, wrapped add) vs Rust panic (same site)
       The path is reachable from any XML document handed to XmlDecode
       (getNextAttributeId returning ATTRIB_UNKNOWN then
       getIndexedAttributeId — the generic indexed-attribute decode loop), so
       a crafted document yields divergent attribute ids or aborts a debug
       build where C++ has fully defined behavior.  Violates ADR 0003
       (mandatory explicit wrapping where C++ wrap is reachable; debug and
       release must compute identically) and ADR 0004 (panic on a
       recoverable, non-UB input).
       repro: `cargo test -p kuna-base --test verify_w1_base_marshal` —
       `verify_marshal_indexed_attr_dec_suffix_matches_cpp_oracle_f1` and
       `verify_marshal_indexed_attr_wraps_like_cpp_on_huge_index_f1`
       (committed failing, ec80296); oracle probe transcribed in the test
       header doc.
       repair sketch: parse the suffix decimal-only with num_get-for-uint4
       semantics (optional sign, decimal digits, overflow -> UINT_MAX,
       minus -> modular negate), and compute
       `attrib_id.get_id().wrapping_add(val.wrapping_sub(1))`.
- F2 (minor): marshal.rs uses zero `// cast:` justification comments while
  carrying ~60 bare `as` casts; sibling xml.rs established the convention
  this protocol greps for.  The truncating/sign-changing ones were each
  verified correct against the C++ (uint8->uint4 string-skip truncation
  marshal.cc:653,669 -> marshal.rs:1531,1549; uint1 header truncation
  marshal.hh:666 -> marshal.rs:1966-1967; int4->uint8 sign-extension of
  `getIndex()` marshal.cc:1218 -> marshal.rs:2123), so this is a
  convention/auditability gap, not a behavior bug — but the truncating casts
  must gain justifications on the repair round.
       cpp: decompiler/cpp/marshal.cc:653 (and above anchors)
       rust: rust/crates/kuna-base/src/marshal.rs:1531 (and above anchors)
- F3 (note): `XmlEncode::write_string_indexed`/`PackedEncode::
  write_string_indexed` compute `index + 1` / `get_id() + index` with bare
  adds; C++ uint4 wraps, Rust debug panics at index == u32::MAX.  Encoder
  indexes are caller-controlled (not stream-controlled) and tiny in-tree;
  fix opportunistically with F1.
       cpp: decompiler/cpp/marshal.cc:562,1192
       rust: rust/crates/kuna-base/src/marshal.rs:867,2084
- F4 (note): `PackedDecode::read_space` panics via `.expect` when the
  stack/join special space is unregistered; C++ returns the manager's null
  pointer to the caller (which then dereferences it — UB downstream).
  Documented in a code comment; consistent with the ADR 0004 "C++ UB ->
  panic" reading.  No change requested.
       cpp: decompiler/cpp/marshal.cc:1019-1023
       rust: rust/crates/kuna-base/src/marshal.rs:1908-1921
- F5 (note): `readOpcode`/`writeOpcode` (6 methods) are deliberately absent
  from the Rust Decoder/Encoder traits and deferred to the kuna-num opcode
  wave (`w1-num-pcode-semantics` owns opcodes.cc/hh), documented in the
  module docs.  Acceptable layering given the dependency direction, but the
  deferral is currently tracked only in a doc comment: on the repair round
  either record it as a `losses.md` deferral entry or extend the
  w1-num-pcode-semantics notes to name these marshal.cc functions
  (XmlDecode::readOpcode x2 with the `get_opcode`/"Bad encoded OpCode"
  error, PackedDecode::readOpcode x2 with the `val < 0 || val >= CPUI_MAX`
  range check, XmlEncode/PackedEncode::writeOpcode) so the surface cannot be
  silently dropped.
       cpp: decompiler/cpp/marshal.cc:429-456,1046-1062,1224-1229; marshal.hh:273-287,370-374
       rust: rust/crates/kuna-base/src/marshal.rs:27-32,643-644,710-711 (doc notes)
- F6 (note, carried): no `port-audit` crate exists anywhere in the workspace,
  so the protocol's `cargo run -p port-audit -- --item w1-base-marshal`
  cannot run (carried from the w1-base-foundation and w1-base-xml reviews).
  Blob SHAs verified manually instead.

## Adversarial tests

`rust/crates/kuna-base/tests/verify_w1_base_marshal.rs` (committed ec80296,
8 tests; 2 pinned failing per F1):

- verify_marshal_indexed_attr_happy_and_error_paths (pass)
- verify_marshal_indexed_attr_dec_suffix_matches_cpp_oracle_f1 (**fails — F1**)
- verify_marshal_indexed_attr_wraps_like_cpp_on_huge_index_f1 (**fails — F1, panic**)
- verify_marshal_packed_string_spans_three_chunks (pass)
- verify_marshal_packed_extended_header_straddles_chunk_boundary (pass)
- verify_marshal_packed_ingest_nul_truncation_and_empty (pass)
- verify_marshal_packed_expect_string_paths (pass)
- verify_marshal_xml_content_tagstatus_exact_bytes (pass)

## Losses

None cited (verdict is REJECT; F5's opcode deferral needs either a losses.md
entry or checklist-note coverage on the repair round).

## Verdict rationale

F1 is a confirmed, oracle-demonstrated behavioral divergence (two
error/success flips, one silent wrong-id, and a debug-build abort on
defined-behavior C++ input) in a public decode entry point, with two failing
tests committed as the work order.  Everything else re-derived — the packed
protocol bit-for-bit, the chunk machinery, the istream parse cores, the
tagStatus machine, partial-state error parity — matches the C++ oracle.  The
item returns to `todo` with findings F1 (mandatory), F2 (mandatory), F3/F5
(cheap, do alongside) as the repair order.

## Round 2 (repair review)

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter; same protocol,
fresh oracle probes — porter notes/transcripts not consulted)
date: 2026-06-11
branch: worktree-wf_cea42066-041-13 (reviewed at repair commit 60bcd4d;
round-2 adversarial tests committed as 31b53dd)
cpp blob shas: re-verified in both the main tree and the worktree
(`git hash-object`: marshal.cc 66b15623, marshal.hh 9817f7b0 — match the
checklist entry)

gate: `cd rust && cargo test -p kuna-base` -> **PASSES**: 90 lib (+1 new
porter unit test `test_marshal_cxx_num_get_u32_dec`) + 1 golden_addrsort +
6 foundation-verify + 8 marshal-verify round 1 (the two F1 work-order tests
now pass) + 4 marshal-verify round 2 (new) + 5 space-address-verify (+1
ignored) + 5 xml-verify (+1 ignored corpus dumper) + 5 xml-r2-verify + 0
doctests.  The marshal verify suites were additionally run `--release`
(wrap parity must hold in both profiles): identical results.
clippy: `cargo clippy -p kuna-base --tests -- -D warnings` -> clean.
mechanical grep: no todo!/unimplemented!/HashMap/HashSet/sort_unstable in
marshal.rs or its verify tests.
port-audit: still no `port-audit` crate in the workspace (F6, carried);
blob SHAs checked manually as above.

### Repair scope

Since the round-1 review point (325cbc7), the only non-test change in this
item's scope is the repair commit 60bcd4d: marshal.rs +103/-12 (the other
commit on the branch, 7a34a97, is the w1-base-space-address verifier's test
file — out of scope here).  Re-derived every hunk of the repair against the
C++:

1. **F1 fix** — new `cxx_num_get_u32_dec` (marshal.rs:493-534) replacing the
   strtoul-semantics parse in `get_indexed_attribute_id`
   (marshal.rs:1162-1191), plus `get_id().wadd(val.wsub(1))` replacing the
   bare add.  Rebuilt a fresh C++ oracle (g++ -std=c++11, libstdc++,
   /tmp/kuna-marshal-verify-r2) transcribing the exact marshal.cc:252-259
   statement sequence (`uint4 val = 0; istringstream s(suffix); s >> dec >>
   val;` + the val==0 throw + `getId() + (val-1)`), 33 probe strings: every
   one matches the Rust helper, including the three distinct num_get
   branches clustered at the u32 boundary — exact "4294967295" (no
   overflow, stored as-is), "-4294967295" (modular negation -> 1),
   "-4294967296" (negative OVERFLOW of an unsigned destination stores
   `__max` = UINT_MAX, not a negated value — the subtlest line of the
   repair, libstdc++ `_M_extract_int`'s `__negative && __is_signed` test),
   plus "010"->10, "08"->8, "0x2"->throw, "007"->7, "12abc"->12, "1-2"->1,
   "12.7"->12, ".5"->throw, "-0"->throw, "+7"->7, "  12"->12 (sentry
   whitespace), "-"->throw, "+-3"->throw, "- 5"->throw,
   "99999999999999999999"->UINT_MAX.  Overflow-flag parity holds
   structurally too: libstdc++ stops accumulating after the flag but keeps
   consuming digits; the port keeps accumulating wrapped values but both
   return UINT_MAX whenever the flag is set, and the consumed-length
   difference is unobservable (the istringstream is discarded).  The id
   arithmetic is now `wadd`/`wsub` (`Wrap` trait = `wrapping_add`/`_sub`,
   types.rs:91-130), identical debug/release — ADR 0003 satisfied; the
   panic path is gone — ADR 0004 satisfied.  The `wsub(1)` genuinely cannot
   wrap (val != 0 just checked); comment says exactly that.
   cpp: decompiler/cpp/marshal.cc:243-260
   rust: rust/crates/kuna-base/src/marshal.rs:493-534,1162-1191
2. **F3 fix** — `XmlEncode::write_string_indexed` now formats
   `index.wadd(1)` (marshal.rs:920-921) matching C++ `<< dec << index + 1`
   uint4 wrap (marshal.cc:562), and `PackedEncode::write_string_indexed`
   writes `attrib_id.get_id().wadd(index)` (marshal.rs:2147-2148) matching
   `writeHeader(ATTRIBUTE, attribId.getId() + index)` (marshal.cc:1192).
   Both bodies otherwise re-checked statement-for-statement (space, name,
   `="`, xml_escape, closing quote; header/typecode/length/payload).
3. **F2 fix** — `// cast:` justifications now present at the five anchored
   truncating/sign-changing casts: the two uint8->uint4 string-skip-length
   truncations (marshal.cc:653,669 -> marshal.rs:1583-1586,1604-1607), the
   two uint1 write_header truncations (marshal.hh:666-667 ->
   marshal.rs:2024-2030), and the int4->uint8 getIndex sign-extension
   (marshal.cc:1218 -> marshal.rs:2186-2188).  Spot re-checked the
   remaining risky casts: advance_position's ptrdiff-vs-uint4 widening
   (marshal.rs:1499-1501), read_space's uint8-vs-int4 comparison
   (marshal.rs:1956-1957), the signed-integer wrapping negations
   (marshal.rs:1813-1817,2123-2125) — all carry semantic comments and match
   the C++.
4. The porter's new unit test `test_marshal_cxx_num_get_u32_dec`
   (marshal.rs:2480-2503) — every asserted value independently confirmed by
   the round-2 oracle run.

No other code paths changed; round-1 hunt-list conclusions for the
untouched 95% of the file stand.

### Hunt list (round 2, repair scope)

- signedness: clean (no new mixed comparisons; re-checked the two carried
  commented sites).
- integer widths: clean — the F1 helper accumulates in exactly uint4 width
  with exact overflow detection (overflowing_mul/add ≡ libstdc++'s
  `__smax`/`__max - __digit` pre-checks).
- wrapping: clean — all three round-1 wrap sites now `wadd`/`wsub`/
  `wrapping_neg`; debug == release verified by running the verify suites in
  both profiles.
- comparator totality: n/a (unchanged).
- iteration-order provenance: n/a (no loops over containers added; the
  helper's byte loop is index order over the suffix, same as the C++
  stream).
- off-by-one / do-while: clean — sign consumed at most once, digit loop
  breaks on first non-digit, `any` flag reproduces the "no digits ->
  failed extraction -> 0" C++11 num_get path (and the C++ `val` is
  pre-initialized to 0, so the result is dialect-robust).
- erase-while-iterating: n/a.
- exception->Result partial-state parity: clean — the val==0 error fires at
  the same point as the C++ throw (after the prefix check, before any
  state mutation; `attribute_index` untouched on both sides), pinned by
  the round-2 round-trip test which continues decoding after the error.

### Findings (round 2)

- F1: **RESOLVED, verified.**  33/33 fresh oracle probes match; both
  round-1 work-order tests pass; four new boundary cases pinned (see
  adversarial tests).  No residual divergence found.
- F2: **RESOLVED** at the anchored sites.  Residual note (no action
  required): ~90 bare `as` casts in marshal.rs still lack the literal
  `// cast:` marker, but all remaining ones are widening (u8->u32/u64),
  mask-then-cast provably in-range, or oracle-verified sign
  reinterpretations already covered by semantic comments — re-graded from
  the round-1 minor to note.
- F3: **RESOLVED, verified** (both encoders; byte-exact + decode round-trip
  pinned by the round-2 tests).
- F4 (note, unchanged): read_space `.expect` on unregistered special space
  — C++ UB -> panic per ADR 0004; no change requested.
- F5: **RESOLVED.**  The porter could not write docs from its sandbox, so
  per the protocol ("the verifier may append, never edit") the deferral is
  now durable as **LOSS-013** in docs/rust-port/losses.md, naming all six
  marshal.cc opcode methods (marshal.cc:429-456,583-598,1046-1062,
  1224-1229; marshal.hh:273-287,370-374) and binding restoration to
  w1-num-pcode-semantics.
- F6 (note, carried): no `port-audit` crate exists; blob SHAs verified
  manually.

No new findings.  No blocker or major findings stand.

### Adversarial tests (round 2)

`rust/crates/kuna-base/tests/verify_w1_base_marshal_r2.rs` (committed
31b53dd, 4 tests, all passing, debug and release):

- verify_marshal_indexed_attr_negative_overflow_boundary_r2 — the three
  num_get outcomes at the u32 boundary ("4294967295"->22,
  "-4294967295"->24, "-4294967296"->22) plus "val-0" error and "val007"
  decimal leading zeros; all through real XML documents ('-' is an XML
  NameChar, so the negative suffixes are stream-reachable).
- verify_marshal_indexed_attr_suffix_stops_at_first_nondigit_r2 —
  partial extraction ("1-2"->1, "12.7"->12, ".5"->error).
- verify_marshal_xml_write_string_indexed_index_wrap_bytes_r2 —
  XmlEncode index u32::MAX renders byte-exact `val0` (wrap) next to
  index 0 -> `val1`; round-trip confirms the C++ quirk that the wrapped
  name is undecodable (val 0 -> LowlevelError) while `val1` decodes to the
  base id.
- verify_marshal_packed_write_string_indexed_id_wrap_r2 — PackedEncode
  ATTRIB_VAL(24) + u32::MAX wraps to header id 23 (byte 0xd7 checked) and
  decodes as attribute id 23 with the intact string payload; non-wrap
  sanity at index 2 -> id 26.

### Losses

LOSS-013 (readOpcode/writeOpcode x6 deferred to the kuna-num opcode wave;
appended by the verifier this round).

### Verdict rationale (round 2)

The repair is exactly scoped to the round-1 work order and each fix is
oracle-verified: the new suffix parser matches libstdc++ num_get on every
probe including the negative-overflow branch, the id arithmetic and both
encoder adds now wrap identically in debug and release, the anchored casts
are justified, and the gate (including both rounds of verifier tests) is
green in both profiles.  The one remaining in-scope departure from the C++
surface — the six opcode marshal methods — is a deliberate, dependency-
direction-driven deferral now durably recorded as LOSS-013 with named
restoration criteria.  ACCEPT-WITH-LOSSES citing LOSS-013.
