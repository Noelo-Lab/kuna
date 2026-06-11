# w2-sleigh-translate

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (round 1; not the porter)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-sleigh` -> ok (16 lib + 9 verifier tests, 0 failed);
      `cargo test -p kuna-base` -> ok (124 lib + all integration suites, 0 failed);
      full `cargo test --workspace` -> 32 suites ok, 0 failed (the
      `Address::renormalize` signature change broke no other crate).
      `cargo run -p port-audit -- --item w2-sleigh-translate`: tool does not
      exist in the workspace yet (skipped; blob shas re-checked by hand:
      `git ls-tree` translate.cc=8cca59a..., translate.hh=5abc655... match
      the checklist `cpp_blob_sha`).

## Scope reviewed

C++ in scope: decompiler/cpp/translate.{cc,hh} (1658 LOC). The diff also
restores the LOSS-012 deferred arms transcribed from space.cc
(JoinSpace::overlapJoin/encodeAttributes/decodeAttributes/printRaw/read,
AddrSpace::read register branch) and address.cc (renormalize,
decode-varnode/Range ATTRIB_NAME branches); those were re-derived against
their own C++ bodies as part of this review. Rust diff:
rust/crates/kuna-sleigh/src/translate.rs (+1486),
rust/crates/kuna-base/src/space.rs (+1916/-92),
rust/crates/kuna-base/src/address.rs (+82/-12). Single commit
3fee3d1 on worktree-wf_6343a4ba-016-2.

Architecture mapping (verified coherent): `AddrSpaceManager` + `JoinRecord`
+ `SpacebaseSpace` + resolver list extend the W1 manager in kuna-base
(`VarnodeStorage` mirrors the VarnodeData triple below kuna-num, with
boundary converters and an Ord-parity test); the abstract `Translate` splits
into `RegisterLookup` (kuna-base supertrait, installed on the manager as the
register half of the C++ `trans` back-pointer), the `Translate` trait, and
`TranslateBase` (concrete base-class state). `UnimplError`/`BadDataError`
were already `KunaError::Unimpl{instruction_length}`/`BadData` (W1).
The marshal id table (4 attributes, 10 elements, ids 43-46/27-36) is
complete and value-checked against translate.cc, including the
kuna-num-defined `ATTRIB_CODE`(43)/`ELEM_SPACEID`(30).

## C++ oracle harness

Expectations for the fragile paths were pinned by compiling a standalone
oracle (minimal concrete `Translate`, registers always-throwing) from
decompiler/cpp/{translate,space,address,pcoderaw,marshal,opcodes,float,xml,
globalcontext}.cc and probing: big-endian renormalizeJoinAddress re-split,
big-endian overlapJoin traversal (incl. pointSkip), the
`(int4)(offset-unified.offset)` truncation quirk, the join allocation gap
(rounding pad offsets are *not* covered), `findAddJoin` uint4 size-sum wrap,
istringstream u32 saturation ("0x100000000" and "-1" -> 4294967295),
`AddrSpace::read` suffix parsing ("0x10:2+1" -> (0x11,2); "0x10+3:5" ->
(0x13, default size); ":2" -> (0,2)), and mergeSequence in a spacebase
space. The Rust matched the oracle on every probe.

## Hunt list

- signedness: clean. Every mixed C++ comparison resolved deliberately with a
  comment: `resolveConstant` ind-vs-size_t (negative-index skip matches the
  C++ size_t conversion), binary-search `int4 min/max` with `len as i32 - 1`
  (empty-list -1 matches the C++ size_t-underflow-then-truncate), the
  `pos > MAX_PIECES` int4 compare, `pos < pieces.size()` loop bounds.
- integer widths: clean. uintb->uint4 truncations commented
  (TruncationTag::decode, logicalsize, intb->int4 decodeOp isize);
  `(int4) smallOff` / sizeTrunc1/2 mixed-width arithmetic oracle-pinned by
  verify tests (truncation quirk + BE re-split).
- wrapping: clean. wadd/wsub/wrapping_add throughout uintb/uint4 lineage;
  uint4 piece-size-sum wrap and `(totalsize+15) & ~0xf` rounding pinned;
  `(size-1)` sign-extension into uintb pinned.
- comparator totality: clean. `VarnodeStorage::Ord` transcribes
  VarnodeData::operator< (space index, offset, BIG sizes first) and is total
  (documented fall-through on a same-index/distinct-object tie, unreachable
  within one manager); `JoinRecord::Ord` transcribes the lexicographic
  comparator line-for-line; cross-transcription Ord parity with
  kuna_num::pcoderaw::VarnodeData is asserted by a porter test.
- iteration-order provenance: clean. splitset BTreeSet == std::set with
  JoinRecordCompare (lookup only); splitlist Vec push order == vector
  push_back (binary-search sortedness follows from monotonic joinallocate,
  same as C++); name2space/shortcut2space BTreeMap == std::map (W1);
  getAllRegisters BTreeMap<VarnodeData,_> == map with operator< (size
  tie-break pinned by verify test); piece decode in stream order; no
  HashMap/HashSet (clippy-denied workspace-wide).
- off-by-one / do-while / reverse iterators: clean. Little-endian reverse
  piece walks (`pos = n-1; pos >= 0`) and the `i != endPiece; i += dir`
  data-order traversal transcribed and BE-pinned; the C++ `pos > MAX_PIECES`
  (not >=, so 65 pieces fit) quirk preserved; findJoinInternal containment
  vs findJoin exact-match boundary pinned via the allocation-gap test.
- erase-while-iterating: n/a — no erasures in scope; insertResolver's
  delete-then-replace becomes drop-then-assign (same observable).
- exception -> Result partial-state parity: clean. find_add_join validates
  before any mutation (as C++ throws before touching joinallocate);
  setBaseRegister errs before assignment; renormalize errs before mutating
  the address; JoinSpace::read assigns *size only after findAddJoin
  succeeds; encodeAttributes writes the space attribute before the
  MAX_PIECES throw (same partial-encode state); TruncationTag::decode
  assigns name-then-size in C++ order.

## Findings

- F1 (minor): the porter's test module fails the repo's customary clippy
  scope: `cargo clippy -p kuna-sleigh --all-targets -- -D warnings` -> 2
  errors (clippy::type_complexity at translate.rs:1202 `CollectEmit::ops`;
  clippy::mutable_key_type at translate.rs:1437 missing the `#[allow]` the
  trait method carries). The protocol's literal gate
  (`cargo clippy -p kuna-sleigh -- -D warnings`, lib only) and
  `-p kuna-base --all-targets` are clean; prior reviews (w1-base-xml,
  w1-num-pcode-semantics) reported `--all-targets`/`--tests` clean, so this
  is a hygiene regression in test-only code. No behavior impact. Fix in the
  next touch of the file.
       cpp: n/a (tooling)
       rust: rust/crates/kuna-sleigh/src/translate.rs:1202, 1437
- F2 (minor, ledgered as LOSS-019): the C++ per-manager join table
  (joinallocate/splitset/splitlist, translate.hh:233-235) lives inside the
  JoinSpace's kind and is therefore shared between managers after
  copy_spaces, where C++ gives the copying manager its own empty table.
  Unobservable in-tree (architecture.cc:637 copySpaces runs before the
  JoinSpace exists; only the Architecture manager ever owns one); porter
  documented it in module docs but had not appended the ledger entry.
       cpp: decompiler/cpp/translate.hh:233-235, translate.cc:659-703
       rust: rust/crates/kuna-base/src/space.rs (JoinState in AddrSpaceKind::Join)
- F3 (minor, ledgered as LOSS-020): `decode_spaces` cannot be called with a
  decoder constructed over the same manager (the C++ usage shape) — `&mut
  self` vs the Decoder's `&AddrSpaceManager`; callers must drive the loop
  body stepwise, and a decoder over a *different* manager would silently
  resolve `contain=`/space references against the wrong table. Documented
  in the method docs; the first real caller is the architecture wave.
       cpp: decompiler/cpp/translate.cc:285-302
       rust: rust/crates/kuna-base/src/space.rs (AddrSpaceManager::decode_spaces)
- F4 (note, no action): `PcodeEmit::decode_op` is an overridable provided
  trait method where the C++ `decodeOp` is a non-virtual member; an
  implementor could shadow it. No caller depends on non-virtualness; flag
  for the W2 engine reviewer if a Rust impl ever overrides it.
       cpp: decompiler/cpp/translate.hh:113
       rust: rust/crates/kuna-sleigh/src/translate.rs (PcodeEmit::decode_op)
- Verified-equivalent notes (no finding): the flattened `isize < 0` check in
  decode_op (C++ nests it inside the `<= 16` stack-array path a negative
  always takes); the single heap buffer replacing the C++ 16-entry stack
  array (allocation-only difference); `String::from_utf8_lossy` on decoded
  strings (ASCII in-tree, consistent with W1 practice); UB->panic on null
  space pointers per ADR 0004 (each `expect` carries the C++-UB anchor);
  `register_translate_ids` as the explicit stand-in for static-constructor
  registration (complete table, value-checked).

## Adversarial tests

rust/crates/kuna-sleigh/tests/verify_w2_sleigh_translate.rs (committed
e0f8d58 "rport/w2-sleigh-translate: verifier adversarial tests"; all 9 pass,
clippy-clean):

- verify_w2translate_renormalize_big_endian_resplit_oracle (the pos1<pos2
  branch the porter's LE-only test never enters; piece truncation/rebase)
- verify_w2translate_overlap_join_big_endian_oracle (forward data order,
  pointSkip, size cut-off)
- verify_w2translate_equivalent_address_truncation_quirk_oracle (far
  offsets map back inside the record — C++ does NOT reject)
- verify_w2translate_join_gap_pastend_and_sum_wrap_oracle (rounding-pad
  offsets uncovered; "Join address range not covered"; uint4 sum wrap ->
  "Cannot create a zero size join")
- verify_w2translate_merge_sequence_spacebase_and_noname_join_oracle
  (IPTR_SPACEBASE merges skip the register-name check; informal merge keeps
  the original; contiguous-no-name falls through to a 2-piece JoinRecord)
- verify_w2translate_decode_attributes_u32_saturation_istream_parity
  ("0x100000000" and "-1" piece sizes saturate to UINT_MAX like num_get)
- verify_w2translate_read_offset_suffix_oracle (":size+off", "+off:size"
  — the size override after '+' is ignored — and bare ":2")
- verify_w2translate_decode_op_seventeen_inputs (the C++ heap-vector path)
- verify_w2translate_register_map_order_big_sizes_first (map tie-break)

## Losses

- LOSS-012: the join/spacebase/register-name deferred arms this entry
  tracked are now **restored and verified** (this item); still open under
  LOSS-012: FspecSpace/IopSpace printRaw and FspecSpace encode payloads
  (need FuncCallSpecs/PcodeOp, W3) — their "not yet ported" markers remain.
- LOSS-013: consistent — decode_op consumes the kuna-num `OpcodeDecoder`
  extension trait exactly as the entry's restoration path prescribes.
- LOSS-015: `create_const_from_space` stores the manager index (not the C++
  heap pointer), explicitly cited at the cast site.
- LOSS-019 (appended this round): join-record table on the JoinSpace,
  shared via copy_spaces, instead of per-manager.
- LOSS-020 (appended this round): decode_spaces un-callable with a
  self-borrowing decoder; stepwise driving until the architecture wave.
