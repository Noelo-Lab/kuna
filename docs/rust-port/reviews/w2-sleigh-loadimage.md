# w2-sleigh-loadimage

verdict: ACCEPT
verifier: independent verifier agent (round 1; not the porter)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-sleigh` -> ok (31 lib + 6 loadimage
      verifier tests + 9 translate verifier tests, 0 failed); full
      `cargo test --workspace` -> 33 suites ok, 0 failed.
      `cargo run -p port-audit -- --item w2-sleigh-loadimage`: tool does not
      exist in the workspace (skipped; blob shas re-checked by hand:
      `git ls-tree worktree-wf_6343a4ba-016-7` gives loadimage.cc=5582299...,
      loadimage.hh=76dce9b..., loadimage_xml.cc=6847a5e...,
      loadimage_xml.hh=350ccfb..., all matching the checklist
      `cpp_blob_sha`).

## Scope reviewed

C++ in scope: decompiler/cpp/loadimage.{cc,hh} (346 LOC) and
loadimage_xml.{cc,hh} (358 LOC). Rust diff: exactly commit 7a693e3 on
worktree-wf_6343a4ba-016-7 — rust/crates/kuna-sleigh/src/loadimage.rs
(+530) and loadimage_xml.rs (+952). The branch also carries a merge of
worktree-wf_6343a4ba-016-2 (w2-sleigh-translate: translate.rs, space.rs,
address.rs, its verifier tests); those are not re-reviewed here, but the
dependency surfaces this item leans on were re-checked against C++:
`Address::operator+(int8)` -> `impl Add<i64> for &Address` via
`wrap_offset(offset.wadd(off as u64))` (address.hh:454/address.rs:545),
`AddrSpace::wrapOffset` signed-remainder transcription
(space.hh:383/space.rs:941), `addressToByte(uintb,uint4)` as the only
overload named `addressToByte` — so the `long adjust` argument converts
through uintb, matching `adjust as u64` (space.hh:514/space.rs:1824), and
`Address::overlap`'s wrapOffset/sign-extended `dist >= size` compare
(address.rs:303).

Architecture mapping (verified coherent): abstract `LoadImage` becomes a
trait with the C++ inline no-op defaults as provided methods; `filename`
becomes a per-implementor field behind required `get_file_name`;
`DataUnavailError : LowlevelError` was already `KunaError::DataUnavail`
(W1 error.rs, inside the Lowlevel catch hierarchy); the `mutable
map<Address,string>::const_iterator cursymbol` becomes a
`RefCell<Option<Address>>` next-key cursor; `map`/`set` members become
`BTreeMap`/`BTreeSet` over the ADR-0002 Address Ord; the unused-outside-
open() `manage` back-pointer is dropped per the workspace convention;
ATTRIB_ARCH(135)/ELEM_BINARYIMAGE(230)/ELEM_BYTECHUNK(231) match
loadimage_xml.cc:21-24 and register via `register_loadimage_xml_ids`.

## C++ oracle harnesses

Two standalone differential oracles were compiled and run for this review
(g++ -O0 -Wno-sign-compare):

1. The hex-content scan of `LoadImageXml::open` (loadimage_xml.cc:111-135
   copied verbatim over an istringstream). Pins: "0 a" -> [f0] (a space
   consumed as the second digit of a pair goes through the `<= '9'` branch
   as -16), "G@!~" -> [09 37], "ab\ncd e f" -> [ab cd d0], "0:" -> [03],
   "abc" -> [ab] (dangling nibble dropped), "ab\xc3..." -> [ab] (high-bit
   byte assigns negative into the signed char and ends the scan),
   whitespace-only -> [].
2. The full `pad()` body (loadimage_xml.cc:154-201 control flow verbatim,
   Address modeled as (space,offset) with the std::map order and verbatim
   wrapOffset/getHighest). Pins: an empty chunk at offset 0 has wrapping
   end uintb-max and erases every later same-space chunk, then self-pads
   512 into its own vector; exact-end ties (end1 == end2) erase; erase
   chains keep comparing against the same survivor; an empty chunk inside
   a gap survives (end = offset-1) and clamps the predecessor pad; a
   next chunk in a different space neither clamps nor erases; a chunk
   ending flush at the space end pads only to `getHighest`; a chunk whose
   endaddr wraps below its key gets no pad at all.

The Rust port matched both oracles on every probe (the six
`verify_w2loadimage_*` tests below).

## Hunt list

- signedness: clean. `while(size>0)` int4 kept as `i32`; the signed-char
  `(c1>0)&&(c2>0)` scan condition kept on `i8`; `(uintb)maxsize > room`
  transcribed as `maxsize as i64 as u64` (sign-extension preserved);
  `dist >= size` inside `Address::overlap` sign-extends as in C++. Noted,
  not a finding: encode()'s `for(int4 i=0;i<vec.size();++i)` sign-compare
  becomes `enumerate()`, divergent only for >2^31-byte chunks (unreachable
  through the XML decoder, and the C++ side is UB there).
- integer widths: clean. `int4 size` = slice length with `// cast:`
  (requests >2^31-1 unsupported in C++ too); `int4 chnksize = chnk.size()`
  truncation kept; `(int4)room` truncations kept; `int4 off =
  addressToByte(adjust,ws)` keeps the long->uintb->int4 chain (truncation
  pinned by adversarial test: adjust 0x1_0000_0010 moves by 0x10); the
  `(uint1)val` truncation kept.
- wrapping: clean. `curaddr -= vma`/`vma += adjust` via wsub/wadd;
  `offset + size - 1` chunk-end arithmetic via wadd/wsub (the uintb-max
  wrap for empty chunks at 0 reproduced exactly — oracle caseA);
  `getHighest - offset + 1` room via wsub/wadd; hex-digit conversion via
  wrapping_* (mod-256 associativity makes the intermediate i8 wraps equal
  the C++ int-then-truncate); operator+ wraps through wrapOffset.
- comparator totality: clean. Both BTreeMaps and the BTreeSet are keyed by
  Address using the ADR-0002 Ord (W1-verified total order); all keys are
  built over registered non-null spaces. The clippy `mutable_key_type`
  allow on adjust_vma's maps is correctly justified (Ord reads only the
  immutable space index + offset).
- iteration-order provenance: clean, per loop. encode() chunk and symbol
  loops: std::map key order -> BTreeMap iteration. open(): XML child
  document order in both. pad() erase loop: C++ (lastiter,iter) pairs over
  the live map -> pre-collected sorted key list; after an erase,
  `iter = lastiter; ++iter` is exactly the next pre-collected key (only
  the current key was removed) — verified equivalent, oracle caseA/caseC.
  pad() insert loop: C++ inserts while iterating, but every insert lands
  in [current, next-original) (guards proven: same-space inserts are
  bounded by the next offset, cross-space inserts sort before any
  higher-index space), so the live-map walk visits exactly the original
  survivors -> pre-collected list equivalent, oracle caseB/caseD.
  loadFill: explicit `++iter` via `range((Excluded(key), Unbounded))`.
  get_readonly: map order. adjust_vma: drains the old map in order,
  inserts into the new (same overwrite-on-collision semantics as
  `newchunk[newaddr] =`). No HashMap/HashSet anywhere.
- off-by-one / do-while / reverse iterators: clean. The
  `upper_bound(curaddr); if (iter != begin()) --iter;` idiom becomes
  `range(..=curaddr).next_back()` with a first-key fallback for the
  "upper_bound == begin() stays at begin()" case — equivalence re-derived
  and exercised (read below the first chunk errors with the original
  address; oracle caseD's ram2:0x10f pin). `i%20 == 19` newline placement
  kept. Pad extents pinned to the exact byte (last pad byte readable,
  next byte DataUnavail) in all adversarial tests.
- erase-while-iterating: clean (the pad() erase loop is the one instance;
  see iteration-order entry and oracle caseA/caseC).
- exception->Result partial-state parity: clean except F1 (minor, below).
  LoadImageXml::open errors (unknown tag, decoder failures) leave the
  partially-populated chunk/addrtosymbol/readonlyset and skip pad() in
  both languages; LoadImageXml::loadFill errors leave the destination
  partially written in both; clear() faithfully does NOT touch readonlyset
  and adjustVma faithfully does NOT re-key it (both pinned by porter
  tests); RawLoadImage::loadFill's zero-fill covers exactly offset..len
  (offset+size == len invariant re-derived).

## Findings

- F1 (minor): after a failed `RawLoadImage::open()`, C++ deletes the
  ifstream but leaves `thefile` dangling non-null, so a second open()
  throws "loadimage is already open" while Rust resets to `None` and a
  retry re-attempts the open.
     cpp: decompiler/cpp/loadimage.cc:59-64
     rust: rust/crates/kuna-sleigh/src/loadimage.rs:280-306
  Not a blocker: every continued C++ use of the object after that throw is
  UB-poisoned (the destructor at loadimage.cc:49-52 double-deletes the
  dangling pointer), so no well-defined C++ execution observes the
  difference; the safe-state choice follows ADR 0004's spirit. Worth a
  one-line code comment on a later pass; no loss entry warranted.
- F2 (informational): `open()` seek/tell failure (loadimage.cc:66-67) in
  C++ silently yields `filesize = (uintb)tellg() = uintb-max` on a failed
  stream; Rust reports the open error. Unreachable for regular files (the
  only kind a raw image can be); the code documents the no-oracle call.
  rust/crates/kuna-sleigh/src/loadimage.rs:293-303.
- F3 (out of scope, flagged for the wave gate): `cargo clippy -p
  kuna-sleigh --all-targets -- -D warnings` fails on two pre-existing
  lints in the merged w2-sleigh-translate test code
  (translate.rs:1202 type_complexity, translate.rs:1437
  mutable_key_type) — not part of this item's diff; this item's two files
  and its verifier test target are clippy-clean (`clippy -p kuna-sleigh`
  lib and `--test verify_w2_sleigh_loadimage` both pass with
  `-D warnings`).

Mechanical pass: no todo!/unimplemented!/HashMap/HashSet/sort_unstable in
either file; every value-changing `as` cast carries a `// cast:` (or
adjacent block) justification; the remaining `as` are infallible literal
casts (`b'9' as i8` etc.) covered by the conversion comment block.

## Adversarial tests

rust/crates/kuna-sleigh/tests/verify_w2_sleigh_loadimage.rs (committed on
the port branch as 2ae908c, all passing):

- verify_w2loadimage_hex_scan_istream_char_semantics
- verify_w2loadimage_pad_empty_chunk_at_zero_wrapping_erase
- verify_w2loadimage_pad_exact_end_tie_chain_and_empty_gap_chunk
- verify_w2loadimage_pad_cross_space_no_clamp
- verify_w2loadimage_space_end_wrap_chunk_no_pad_and_wrapped_overlap
- verify_w2loadimage_zero_length_reads_and_adjust_vma_int4_truncation

losses: none (no checklist-declared losses; no new divergences confirmed —
F1/F2 sit behind C++ UB / no-oracle OS failures).
