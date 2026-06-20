# w8-s9-prettyprint
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> prettyprint: 38/38 lib + 8/8
  adversarial PASS. One unrelated pre-existing failure
  (`corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`) is an environmental
  missing-`.sla` build artifact ("Could not find .sla file for x86:LE:64:default"),
  not introduced by this item (corpus_bootstrap does not reference prettyprint;
  `.sla` are gitignored built artifacts per CLAUDE.md). `cargo clippy -p
  kuna-decomp -- -D warnings` clean.

## Scope

C++: decompiler/cpp/prettyprint.{cc,hh} (blob shas verified against checklist:
cc f85f0185…, hh bd65f4e2…, both match HEAD). Rust: rust/crates/kuna-decomp/
src/prettyprint.rs (2635 lines) + src/prettyprint/tests.rs. The byte-exact line
breaker (`EmitPrettyPrint` + `EmitNoMarkup`) is the load-bearing surface the
datatests `stringmatch`; the markup back-end (`EmitMarkup`) and `TokenSplit` /
`circularqueue` are the supporting cast.

## Hunt list

- **Signedness**: clean. All counters are `int4` (i32) matching C++ `int4`. The
  one comparison that matters — the short-break heuristic `(numspaces <=
  spaceremain) && (val - spaceremain < 10)` (prettyprint.cc:705) — is ported as
  signed i32 (prettyprint.rs:1916), reproducing C++ exactly. Verified by ADV3
  (fills to width without spurious break).
- **Integer widths**: clean. `size = tok.size()` (size_t->int4) -> `tok.len() as
  int4`; `off as int4` mirrors C++ `(int4)off`; color `hl as i32 as u64` matches
  the C++ enum->unsigned cast. No promotion-into-wider-type traps (no `int*int`).
- **Wrapping**: NOTE (accepted, F3). The running totals use plain `+=` not
  `wrapping_add` (`rightotal += getNumSpaces()` with the 999999 sentinel,
  prettyprint.rs:2036/2043). C++ wraps silently; Rust panics in debug. Overflow
  is unreachable for real decompiler output (>2147 consecutive `tagLine` at the
  999999 sentinel needed) and a wrapped `rightotal` would mis-compute in C++
  too — i.e. the C++ algorithm itself relies on no-wrap. Faithful for all
  reachable inputs; flagged minor, not a blocker.
- **Comparator totality**: N/A — no comparators / sorts / floats in this module.
- **Iteration-order provenance**: clean, and this is the key determinism point.
  No `HashMap`/`HashSet` anywhere. State is a `Vec<int4> indentstack` and two
  `CircularQueue` (Vec-backed, raw-index addressed) — every loop is over a
  deterministic index range (`overflow` top-down `(0..len).rev()` ==
  C++ `for(i=size-1;i>=0;--i)`; `expand_buffers` `0..max` with `max` read
  *before* expand == C++; `advanceleft`/`scan`/`flush` all index/ring-driven).
- **Off-by-one / do-while / reverse iterators**: clean. `circularqueue::expand`
  `while i != right` then copy-rightmost is a test-first while (not do-while),
  matching C++; `advanceleft` `while l>=0` reads size first, matching C++. The
  ring math `(right+max-1)%max`, `(left+1)%max`, `(right+1)%max`,
  `left==(right+1)%max` is transcribed verbatim. ADV6 verifies the wrapped-left
  expand contract (leftmost -> reference 0, contents preserved in order).
- **Erase-while-iterating**: clean. The queues use `popbottom`/`pop` (single
  end-pointer moves), not mid-container erase; `indentstack` uses `push/pop`
  back only. No retain/collect-remove translation needed.
- **Exception -> Result partial-state parity**: clean. C++ `LowlevelError`
  throw sites (`flush` missing-group-end, `setMaxLineSize` range, `print` of a
  bump_t/line_t, `print_token` empty indentstack) map to `KunaError::lowlevel`
  via `?`; the `Emit`-trait wrappers `.expect()` the internal-invariant ones the
  way an uncaught C++ `LowlevelError` aborts. ADV7 + `flush_missing_group_end`
  exercise the flush throw with the same partial state (negative-size group left
  at queue bottom).

## Mechanical pass

- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`panic!` in
  prettyprint.rs + tests: NONE.
- bare `as` casts: present (index `as usize`, width `as i64`/`as int4`) without
  per-cast `// cast:` comments (F2, minor). All are mechanical and behavior-
  preserving (match C++ `(int4)off`, `tok.size()`, enum->unsigned color);
  `PackedEncode`'s own casts already carry `// cast:` notes in kuna-base.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean. (`--tests` surfaces
  pre-existing warnings in other suites — constseq/double/heritage/kuna_regionid
  /verify_w3..w6 — none in prettyprint or the new verify file.)

## Markup back-end (EmitMarkup) re-derivation

`PackedEncode` (kuna-base marshal.rs:2008) holds only `&mut Vec<u8>` and every
method appends — confirmed stateless. So the port's per-call fresh-encoder
design (`with_encoder`) yields the identical byte stream a single persistent C++
`Encoder*` would, validating the central design claim. Element/attribute ids,
the `if hl != no_color` color guard, the `if (ptr != 0)` -> `if let Some` guards,
and the `getUnsizedId()!=0` id guard all transcribe faithfully. `tagField`'s
whole NAME/ID/OFF/OPREF block is gated by the type presence exactly as C++ gates
on `ct != 0` (type_name is Some iff ct non-null). `tagBitField` writes NAME
under `if let Some` where C++ writes it unconditionally (F1, minor) — observably
identical given the C++ invariant that bitfield `ct` is always non-null, so the
caller always supplies `type_name = Some(..)`.

## Adversarial tests (added: rust/crates/kuna-decomp/tests/verify_w8_s9_prettyprint.rs)

All 8 expected values re-derived from a verbatim-C++-engine oracle harness
(EmitPrettyPrint/EmitNoMarkup/TokenSplit/circularqueue extracted from the
vendored prettyprint.cc; first validated to reproduce the porter's four golden
cases byte-for-byte, then run on each adversarial stream):
  - adv1_oversized_token_triggers_overflow — single token wider than the line
  - adv2_overflow_mid_line_keeps_trailing_space — exact trailing-space byte
  - adv3_short_break_heuristic_fills_to_width — the `< 10` suppression boundary
  - adv4_nested_indent_with_overflow — overflow rewriting indentstack.back()
  - adv5_queue_expansion_long_group — 120 tokens force `expand()` (x2) + the
    scanqueue `(ref+max-left)%max` reference fix-up
  - adv6_circularqueue_expand_wrapped_left — expand with left wrapped past right
  - adv7_flush_missing_group_end_rejects — LowlevelError partial-state parity
  - adv8_comment_fill_on_forced_break — `commentmode` + `commentfill` prefix
All PASS.

## findings

- F1 (minor): `EmitMarkup::tag_bit_field` writes ATTRIB_NAME under
  `if let Some(nm) = tname` whereas C++ writes `ct->getName()` unconditionally.
  Observably identical because bitfield `ct` is always non-null (the C++ would
  deref-crash otherwise), so the caller always passes `type_name = Some(..)`.
  Latent fragility only if a future caller violates that invariant.
       cpp: decompiler/cpp/prettyprint.cc:262
       rust: rust/crates/kuna-decomp/src/prettyprint.rs:883-885
- F2 (minor): bare `as` casts lack the protocol's `// cast:` justification
  comments. All are mechanical/behavior-preserving (index/width/enum-color).
       cpp: decompiler/cpp/prettyprint.hh:831 ((uintb)o), prettyprint.cc:427 ((int4)off)
       rust: rust/crates/kuna-decomp/src/prettyprint.rs:1386,1429,1626
- F3 (minor): running totals (`rightotal`/`leftotal`/`spaceremain`) use plain
  `+=`/`-=` not `wrapping_*`; C++ wraps silently on i32 overflow, Rust panics in
  debug. Unreachable for real output and a no-op semantic difference there (a
  wrapped total breaks the algorithm in C++ too).
       cpp: decompiler/cpp/prettyprint.cc:809,818
       rust: rust/crates/kuna-decomp/src/prettyprint.rs:2036,2043

None of F1-F3 is a blocker or major: each is either observably identical under
the C++ invariants/reachable inputs, or a style note. No correctness divergence
was found on any tested or reachable path.

## losses

- LOSS-115: `EmitMarkup` supports only the packed encoder; the unpacked
  `XmlEncode` markup path (`setPackedOutput(false)` -> `print C xml` unpacked) is
  not wired. Never on a tested/corpus path (packed is the default and is
  byte-faithful). Appended to losses.md by this verifier.
