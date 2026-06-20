# w10-bitfield-activate
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-15
branch: rport/w10-bitfield-activate (substantive commit 0fbda33; verifier test commit aa4c1df)
base: rust-port @ b34b855 (merge-base == rust-port tip)

gate: `cd rust && cargo test -p kuna-decomp` -> ok, 0 failed
        (lib: 1698 passed = 1688 base + 10 new bitfield tests; the verifier
         integration `verify_w10_bitfield.rs`: 9 passed = 5 prior + 4 new vrfy2_*).
      Whole-suite `cargo test --workspace` -> exit 0, 0 failed (175 "test result: ok").
      Base regression check: rust-port lib = 1688 passed; branch lib = 1698 passed
        -> every base test still green, +10 new, ZERO regression.
      C++ oracle: branch touches ONLY rust/ (`git diff --name-only rust-port...HEAD`
        has no non-rust path) -> decompiler/cpp byte-untouched -> decomp_test_dbg
        B5 207/207 + 675/675 PARITY OK by construction, untouched.
      `cargo clippy -p kuna-decomp --lib` -> clean (no warnings/errors).

## What this branch actually is (read the diff, not the name)

Despite the name "activate", the diff does **not** activate any end-to-end
bitfield rendering. The single substantive commit (0fbda33) re-lands the held
bitfield scaffolding — previously reviewed and ACCEPTED on `rport/w10-bitfield`
(a17a467) — onto the now-live `rust-port` tree (which since absorbed
cast-strategy / return-narrow / elf-loader). `git merge-base rport/w10-bitfield
rport/w10-bitfield-activate` == 22cc48c, and `rust-port...activate -- rust/` is
exactly the bitfield base + type-query layer, with the rule `apply_op` bodies
STILL returning 0 at the SEAM and NO printc `pushBitfield`/`checkBitFieldMember`
and NO transform subclass. So there is no new decompile pass to validate, no
`bitfields2.xml`/B5 parity gain to confirm, and nothing fabricated — the wave
delivers the prerequisite type-query layer onto the live tree, re-verified fresh.

## Scope of the diff

4 Rust files, +893/-16:
- `bitfield.rs` (+201): the `BitFieldTransform` base — `new` ctor
  (bitfield.cc:96-116, incl. the TYPE_PARTIALSTRUCT unwrap) and
  `establish_fields` (bitfield.cc:57-91), the worklist builder both subclasses call.
- `dtype.rs` (+324): the type-query layer — `BitFieldTriple` + `compare`
  (type.cc:932-949), `TypeField::compare_max_byte` / `TypeBitField::compare_max_byte`
  (type.hh:322-324, 339-341), `upper_bound_idx` (libstdc++ `upper_bound`),
  `collect_bit_fields` (type.cc:1804-1826), `has_bit_fields_in_range`
  (type.cc:1828-1852), and `TypeBitField::bits()`.
- `bitfield/tests.rs` (+123): 5 porter unit tests (ctor + establish_fields).
- `tests/verify_w10_bitfield.rs` (+261 porter, +~120 verifier): the adversarial set.

No reserved file touched (`git diff --name-only` over rust-port...HEAD =
bitfield.rs, bitfield/tests.rs, dtype.rs, verify_w10_bitfield.rs only — none of
funcdata.rs / database.rs / architecture.rs / heritage.rs / merge.rs /
funcdata_merge.rs / variable.rs / varmap.rs).

## Faithfulness (line-by-line against the C++ in scope)

- `BitFieldTransform::new` — matches bitfield.cc:96-116. TYPE_STRUCT -> parent +
  off; TYPE_PARTIALSTRUCT -> `get_partial_base()` (== C++ `getParent()` ==
  `container`, type.hh) + `get_partial_offset()` (== `getOffset()` == `offset`),
  `initialOffset = off + part_off` iff parent is TYPE_STRUCT; else parentStruct
  null, initialOffset -1, containerSize -1. Faithful.
- `establish_fields` — matches bitfield.cc:57-91 statement-for-statement:
  `vnBitSize = vn_size*8`; the BitRange container; `collectBitFields(0,...)`;
  `sort(BitFieldTriple::compare)`; the per-triple translateLSB/numBits, the two
  `> vnBitSize` clamps, the `field_pos > pos` leading-hole (gated), the
  `code == 0 || code == 3` field push vs the `else` gated hole, `pos = fieldEnd`,
  and the trailing `pos < vnBitSize` whole-hole. The `is_int` datum read for the
  field record is `triple.bitfield.field_type.get_metatype() == TYPE_INT` — the
  exact datum the C++ BitFieldNodeState field ctor reads (bitfield.cc:26).
- `collect_bit_fields` / `has_bit_fields_in_range` — match type.cc:1804-1852:
  `upper_bound(compareMaxByte)` start, the `code==1 break` / `code==-1 continue`
  / collect dispatch, the field-loop `curField.offset >= offset+sz` half-open
  break, the `!= TYPE_STRUCT` / `!hasBitfields()` skips, and the recursion args
  `(base+field.offset, res, offset-field.offset, sz)`. Faithful; the two are the
  same walk with collect-all vs short-circuit, exactly as upstream.
- `BitFieldTriple::compare` — matches type.cc:934-949: endianness keyed off op1,
  folded byteOff = `offset + bitfield.byteOffset`, BE `>` / LE `<` byte term, then
  leastSigBit `<` tie-break, else false. The known op1-keyed (theoretically
  non-total under MIXED endianness) comparator is transcribed faithfully — this
  is the correct port-the-C++ choice, and real bitfields in one struct are
  homogeneous-endian, so the sort is total in practice.
- `compare_max_byte` x2 — verbatim of type.hh:323 (`off < field.offset +
  field.type->getSize()`) and type.hh:340 (`off < byteOffset + byteSize`).
- `BitFieldTriple` dropping `immedContainer` — VERIFIED non-finding: grep over
  decompiler/cpp confirms `immedContainer` is assigned in the ctor and
  **never dereferenced anywhere**; establishFields/compare read only
  `bitfield->bits` and `offset`, both preserved by the owned-clone projection.

## NO special-casing (the primary REJECT trigger — clean)

grep over the NON-test source (bitfield.rs, dtype.rs) for field-name / mask /
offset / function-name hardcoding: every `"field3"/"sfield4"/"fieldb"/"inner"/
"pad"/"myfoo"` literal lives inside `#[cfg(test)] mod tests` or the integration
test; **zero** in src. The only integer literals in the ported source paths are
the overlapTest result codes (`code == 0 || code == 3`, `code == 1`, `code ==
-1`) — these are the C++ overlapTest return enum, not data-dependent constants.
Field offsets/widths flow from `TypeStruct`/`TypeBitField` (`byte_offset`,
`byte_size`, `least_sig_bit`, `num_bits`, `offset`), never constants in src.

## Mandatory hunt list

- Signedness: clean. `int4 = i32` (signed). The C++ `for(int4 i=0;i<overlap.size())`
  signed/unsigned compare is sidestepped — the Rust iterates `for triple in
  &overlap` (no index/size compare). `upper_bound_idx` uses `usize` first/count
  with the libstdc++ math.
- Integer widths: clean. All arithmetic on `int4` (small bit counts / byte
  offsets); no int4*int4->intb promotion in scope. `vn_size*8`, `offset+sz`,
  `base+offset` never approach i32 range.
- Wrapping: clean — none needed, none added. `field_pos-pos` runs only under
  `field_pos > pos`; `field_end-pos`/`vnBitSize-pos` are non-negative by the
  `pos <= field_end <= vnBitSize` loop invariant. Signed i32, no debug panic.
- Comparator totality: `BitFieldTriple::compare` is irreflexive + asymmetric
  (verified by adversarial tests under both endians); faithfully reproduces the
  C++ op1-keyed comparator (same homogeneity assumption as upstream).
- Iteration-order provenance: `for triple in &overlap` after `sort_by` reproduces
  C++ `sort(...,BitFieldTriple::compare)` then indexed for-loop. The bitfield/
  field walks iterate `&bitfield[start..]` / `&field[fstart..]` = std::vector
  insertion order from `upper_bound`, matching the C++ iterator walk. No HashMap/
  HashSet anywhere in the diff.
- Off-by-one / upper_bound boundary: `upper_bound_idx` is the exact libstdc++
  `upper_bound` (first element with `comp(value,elem)` true). For count==1:
  step=0, `count -= step+1` -> 0, no underflow. The `compareMaxByte` "ends AT
  offset is skipped" semantics verified (w10bf + vrfy2 tests). No do-while/
  reverse-iterator idioms in scope.
- Erase-while-iterating: N/A — the ported functions only read containers and
  push to `res`/`work_list`; no in-place erase.
- Exception->Result partial-state: N/A — no `?`-propagating fallible calls in the
  ported bodies; the W6 `getUnsizedId` seam (Err) is not on these code paths.

## Mechanical pass

- `grep todo!|unimplemented!|HashMap|HashSet|sort_unstable` over the 4 changed
  files: ZERO hits.
- bare `as` casts: the 3 `as int4` hits in dtype.rs (1586/2196 struct-compare,
  345 get_mask) are PRE-EXISTING (not in this diff — `grep -c` over the diff = 0);
  none introduced by this branch.
- `cargo clippy -p kuna-decomp --lib` clean.

## Adversarial tests (verifier-authored, this round; landed in verify_w10_bitfield.rs @ aa4c1df)

- `vrfy2_establish_fields_clamps_field_past_varnode_boundary` — a 2-byte struct
  field straddling the top of a 1-byte root Varnode; exercises the
  `fieldPos/fieldEnd > vnBitSize` clamp (bitfield.cc:70-73) + the partial-overlap
  `else` hole branch, gated by follow_holes. Under-exercised by both prior batches.
- `vrfy2_collect_overlap_codes_continue_then_break` — 3-byte container, middle-byte
  query: byte-0 field below (code -1 -> continue/skip), byte-1 overlap (collect),
  byte-2 above (code 1 -> BREAK). Proves break != continue and the break
  terminates (type.cc:1811-1813).
- `vrfy2_collect_empty_and_past_end_are_safe_noops` — empty struct + a query past
  every container; the binary-search `count -= step+1` math must not panic and
  must collect nothing; plus establish_fields on an empty struct -> exactly the
  one whole-Varnode trailing hole.
- `vrfy2_triple_compare_same_byte_lsb_tiebreak_both_endians` — within a byte the
  leastSigBit term has NO endian branch (type.cc:944-947): lsb0 < lsb4 under
  both BE and LE, and the sort places lsb0 first regardless of input order.

All 4 pass against the ported query layer (no failing test / no divergence — this
is an ACCEPT, not a REJECT).

findings:
  - none (no correctness finding; the partial scope is a recorded loss, below).

losses: LOSS-092 (the two transform engines `BitFieldInsertTransform`/
  `BitFieldPullTransform`, all six rule `apply_op` bodies, and — implied — the
  printc `pushBitfield`/`checkBitFieldMember` render remain unported; the rules
  return 0 at the SEAM and are not yet wired into a `universalAction`, so there
  is no live regression). LOSS-066 (the `expression.cc` bitfield-expression
  family). This wave delivers the prerequisite `BitFieldTransform` base +
  type-query layer onto the live tree; the end-to-end `bitfields2.xml`/B5
  rendering closes when LOSS-092's restoration criteria are met (the IR-mutation
  + new-op emission surfaces + relative-pointer `getPtrInto` + the printc render).
  No new losses.md entry required — the divergence is fully covered by the
  existing LOSS-092 ledger; this verdict re-confirms it against the live base.
