# w8-s9-stringmanage-pack
verdict: ACCEPT
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
round: 2 (prior round 1 = REJECT on F1; fix landed in d4b1766)

gate: `cd rust && cargo test -p kuna-decomp` ->
      lib 1470/1470 pass; catalog_bytecompat 5/5 pass;
      verify_w8_s9_stringmanage_pack 8/8 pass (the round-1 F1 test
      `f1_no_terminator_within_max_returns_empty_like_cpp` now PASSES — it was
      failing-by-design in round 1).  The only failing test is
      `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`, which is an
      environment/build-artifact failure unrelated to this item: it needs the
      compiled x86 `.sla` inside the worktree's `specs/` dir, and only RISCV/x86
      `.sla` exist in the MAIN tree, not in this worktree.  `corpus_bootstrap.rs`
      is not touched by this item (verified via `git diff --name-only`), and the
      item touches no corpus/sla/sleigh machinery.

blob-sha check: all six in-scope C++ blobs match the checklist `cpp_blob_sha`
  exactly:
    stringmanage.cc       f042000ba2b55f050e62748e1806acc68a8a9ab5  OK
    stringmanage.hh       e4c1c4e32ecf9a489c6e44f2335ce99098a04f37  OK
    kuna_naming.cc        aed28df5cc50fb17e926d0c39a4b25a5e999e759  OK
    kuna_naming.hh        9ab0cee950541f135f18477c729ecdc282c3302f  OK
    kuna_arraynotation.cc 6538855fe411106773d2c19711cafc768de4ec21  OK
    kuna_arraynotation.hh a4940a87a603837418d9d6efc4517a733380a241  OK

## F1 (round-1 blocker) — RESOLVED

The round-1 REJECT blocker (major): `StringManagerUnicode::get_string_data`
emitted a (truncated) string literal where C++ `getStringData` emits nothing,
because the `amount == 0` no-terminator path was a `break 'fill Ok(())` that fell
through to `check_characters` / `assign_string_data` instead of the C++ early
`return stringData.byteData` (stringmanage.cc:455-457).

Fix (d4b1766, rust/crates/kuna-decomp/src/stringmanage.rs:637-644): the
`amount == 0` branch now `return &self.base.string_map[addr].byte_data;`
directly, mirroring the C++ `return` of the still-empty cached buffer BEFORE
`checkCharacters`/`assignStringData` run.  Re-derived line-for-line against the
C++ do-while; the `try`/`catch(DataUnavailError)`, `numChars < 0`, and terminator
paths are unchanged and still correct.  Confirmed empirically: the round-1 proof
test (32 non-zero bytes, maximumChars=32, no terminator) now returns `[]` and
passes.  Two new round-2 tests (g3) re-confirm the fix at a non-multiple-of-32
maximumChars boundary and check the symmetric control (terminator on the last
byte of maximumChars still decodes — the fix did not over-clamp).

## Hunt list (round-2 full re-derivation)

- **Signedness**: clean.
    * `get_string_data`: `new_buffer_size > maximum_chars` — C++ compares
      `uint4 newBufferSize` vs `int4 maximumChars`; both small non-negative here,
      agree (port keeps both as `int4`, equivalent).  `amount = new_buffer_size -
      cur_buffer_size` never goes negative (clamped path only).
    * `get_codepoint` UTF-32: `(buf[k] as int4) << shift` can be a negative
      `int4` when the top byte >= 0x80; the resulting negative codepoint is
      returned RAW (the `codepoint >= 0xd800` gate is skipped for negatives) and
      the caller (`check_characters`/`write_unicode`) rejects it via
      `codepoint < 0` — matches C++ exactly.  Pinned by g2.
    * `decode_hex_bytes`/`get_char`: C++ `char` is signed and the loop guard is
      `c > 0`; the port reads `content[pos] as i8 as i32` so EOF (-1) and NUL/high
      bytes stop the loop identically.
- **Integer widths**: clean.  `int4`=i32, `uint1`=u8, `uint8`=u64, `intb`=i64,
  `uintb`=u64 mappings consistent.  UTF-16 surrogate combine
  `(cp<<10)+trail+(0x10000-(0xD800<<10)-0xDC00)` and UTF-8 1/2/3/4-byte assembly
  fit `int4` exactly as C++.  `calc_internal_hash`: `(reg as uint8) << 32`
  matches `((uint8)reg)<<32` (no sign extension — `reg` is u32).
- **Wrapping**: clean.
    * UTF-32 byte-disjoint sum `(b3<<24)+(b2<<16)+(b1<<8)+b0`: each term occupies
      a distinct byte position so the i32 add never carries and never overflows
      in debug; verified with all-ones input (g2) — no panic, yields exactly -1.
    * `kuna_storage_comment` stack offset: `soff.unsigned_abs()` reproduces
      C++ `(uintb)(-soff)` incl. `i64::MIN` (both -> 0x8000000000000000);
      positive branch `soff as uintb` == `(uintb)soff`.  Prior-verified, re-checked.
    * `calc_internal_hash`/`crc_update` operate on u32 with the prior-verified
      kuna-base CRC (no arithmetic in scope here).
- **Comparator totality**: clean.  `string_map` is `BTreeMap<Address, StringData>`;
  `Address: Ord` is a total order (space-index then offset), prior-verified.  No
  float/`partial_cmp` anywhere in scope.
- **Iteration-order provenance, per loop**:
    * `encode` iterates `self.string_map.iter()` (BTreeMap ascending by Address)
      == C++ `map<Address,StringData>` ascending — the marshalled `<string>` order
      is deterministic and matches.
    * `decode` loop reads `<string>` children in stream order, `insert`-ing each;
      the only map is the BTreeMap, so re-encode order is canonical regardless.
    * `write_unicode`/`check_characters`/`has_char_terminator`/`get_codepoint`
      loops are index walks over a byte slice — index order, identical to C++.
    * No `HashMap`/`HashSet` anywhere in the in-scope Rust (grep: NONE).
- **Off-by-one / do-while / reverse iterators**:
    * `get_string_data` do-while: the round-1 F1 bug (the only do/while in scope)
      is now fixed; the `loop { ... }` correctly preserves at-least-once semantics
      (the body runs before any terminator check) and the `amount==0` early return
      matches.  Pinned by g3.
    * `has_char_terminator` outer `i += charsize` step and inner `0..charsize`
      scan match C++; `write_unicode`/`check_characters` `i += skip` advance match.
    * `encode` `i % 20 == 19` break is the exact C++ `i%20==19` 20-byte line break
      (pinned by round-1 f4).
    * `decode_hex_bytes` two-char `is.get()` pairing + `is >> ws` skip matches the
      C++ `istringstream` loop byte-for-byte.
- **Erase-while-iterating**: clean.  No remove/retain/drain during any traversal
  in scope.  `register_internal_string_data` / `get_string_data` build a detached
  `StringData` and `insert` once (the borrow-checker workaround for the C++
  in-place `stringMap[addr]` mutation) — the single map slot semantics are
  preserved for the normal (fresh-entry) path.
- **Exception -> Result partial-state parity**: clean.  The C++
  `catch(DataUnavailError)` -> `return empty` maps to
  `Err(KunaError::DataUnavail{..}) => return empty` (and a catch-all `Err(_)` ->
  empty, conservatively matching).  `write_utf8`'s two `LowlevelError` throws
  (negative / >21-bit codepoint) map to `panic!` with identical text — these are
  unreachable on validated input (`check_characters` gates first) exactly as in
  C++, so no partial-state observable.

## Mechanical pass

- grep `HashMap|HashSet|todo!|unimplemented!|sort_unstable` over the four
  in-scope Rust source/test files: **NONE**.
- bare `as` casts in scope: present (e.g. `codepoint as uint1`, `buf[k] as int4`,
  `*b as int4`, `size as usize`).  These are direct transcriptions of the C++
  integer casts (`(uint1)`, `(int4)`, indexing) inherent to byte/codepoint
  manipulation; each mirrors a specific C++ cast and was re-derived against it.
  They do not carry per-cast `// cast:` comments — noted as a documentation-style
  observation (consistent with round 1, which also did not treat them as
  findings); none change C++ semantics.
- clippy: `cargo clippy -p kuna-decomp --tests` reports one `error`
  (`overly_complex_bool_expr`) and several warnings, ALL in OUT-OF-SCOPE files
  (`heritage.rs:1602`, `unionresolve.rs`, `double.rs`, `kuna_regionid.rs`,
  `constseq/tests.rs`) ported in earlier waves.  Grepping the clippy output for
  the four in-scope files returns ZERO diagnostics — the stringmanage / kuna_naming
  / kuna_arraynotation port is clippy-clean.  The crate-wide pre-existing clippy
  error is not introduced or owned by this item.

## Findings (round 2)

- O1 (observation, not blocking): `StringManager::is_string_cached` is a
  cache-only check, whereas C++ `isString(addr,charType)` calls the virtual
  `getStringData` (which decodes through the loadimage and populates the cache)
  and returns `!buffer.empty()`.  The port deliberately split this: the
  decode-triggering form is left to the caller that holds the loader (the same
  documented seam as `get_string_data`/`register_internal_string_data`, where
  `glb->loader` / `AddrSpaceManager` are threaded in).  No in-scope consumer; the
  out-of-scope callers (printc/typeop) own the loader.  Consistent with the
  already-accepted seam pattern — no behavior is silently lost, the function is
  renamed to make the cache-only nature explicit.
      cpp:  decompiler/cpp/stringmanage.cc:166-172
      rust: rust/crates/kuna-decomp/src/stringmanage.rs:273-278
- O2 (observation, not blocking): `decode` builds a fresh `StringData` and
  `insert`s it, whereas C++ `stringMap[addr]` reads the existing entry and
  `push_back`s onto its `byteData` (append).  These diverge ONLY when the same
  Address appears in two `<string>` children of one `<stringmanage>` stream
  (malformed/unusual input); on the normal single-pass decode (fresh empty
  entry) append == replace, so well-formed round-trips are identical (pinned by
  round-1 f4).  Not reachable on the data kuna emits.
      cpp:  decompiler/cpp/stringmanage.cc:239,262
      rust: rust/crates/kuna-decomp/src/stringmanage.rs:335-342
- O3 (observation, carried from round-1 F2, not blocking): `kuna_is_generated_name`
  slices `&nm[0..6]` after a `nm.len() >= 6` byte guard; if byte index 6 split a
  multibyte UTF-8 char the slice would panic, where C++ `nm.compare(0,6,...)` is a
  byte op that never panics.  Symbol names are ASCII, so unreachable in practice.
      cpp:  decompiler/cpp/kuna_naming.cc:96
      rust: rust/crates/kuna-decomp/src/kuna_naming.rs:95

No blocker or major stands.  The kuna_naming address helpers (`dat_`/`sub_`/
`label_`/`a<n>`, re-exported from database.rs), `kuna_to_lower`,
`kuna_storage_comment` (register/stack/join/tmp branches), `kuna_angr_naming`,
`OptionNameStyle::apply`, `on_or_off`, and `OptionArrayNotation::apply`
(incl. the exact `"namestyle must be ..."` / `"Must specify toggle value, on/off"`
/ `"Array notation ... turned on|off"` messages) were re-derived against the C++
and are faithful.

## Adversarial tests

Committed to the item branch (27237fa,
rust/crates/kuna-decomp/tests/verify_w8_s9_stringmanage_pack.rs), 8 total
(5 from round 1, all now passing; 3 new for round 2):
  - f1_no_terminator_within_max_returns_empty_like_cpp  (round 1; now PASSES — F1 fixed)
  - f2_terminated_just_under_max_decodes                (round 1)
  - f3_register_internal_legality_and_constant_keying   (round 1)
  - f4_stringmanage_roundtrip_and_20byte_break          (round 1)
  - f5_utf16_surrogate_pair_boundary                    (round 1)
  - g1_truncation_reencode_path_caps_at_maximum_chars   (round 2; assignStringData
        truncation re-encode path, numChars >= maximumChars, isTruncated)
  - g2_utf32_range_gate_and_no_add_overflow             (round 2; UTF-32 range gate
        + byte-disjoint i32 assembly / negative reinterpret / no-panic)
  - g3_no_terminator_returns_empty_but_edge_terminator_decodes (round 2; F1 fix at
        a non-/32 maximumChars boundary + last-byte-terminator control)

losses: none.
