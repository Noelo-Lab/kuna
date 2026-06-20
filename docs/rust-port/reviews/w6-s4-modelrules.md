# w6-s4-modelrules
verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (Opus 4.8)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> 1074 passed, 1 failed; the single
      failure is `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`
      ("Could not find .sla file for x86:LE:64:default") — a pre-existing
      missing-gitignored-build-artifact ENVIRONMENT failure, NOT touched by this
      item (the file is not in the commit) and unrelated to modelrules. The
      modelrules lib suite (38 tests) and the verifier's adversarial binary
      (5 tests) are all green. `cargo clippy -p kuna-decomp -- -D warnings` clean.

scope: commit 88c9a32 isolates the item to `modelrules.rs` (+ `modelrules/tests.rs`);
       `fspec.rs` is untouched. cpp blob shas match the recorded values exactly
       (cc 3f4bb8ef…, hh 572a133b… — no drift).

## Hunt list

- **Signedness.** clean. All loop indices over containers use `int4`-typed
  bounds compared as `... < x.len() as int4` (matching C++ `int4 < size_t` under
  `-Wno-sign-compare`); `get_group()`/`get_size()` are `int4`; status indices
  cast `as usize` only after the C++ already indexed with the same value.
- **Integer widths.** clean. `uintb->u64` (VarnodeData.offset), `uint4->u32`
  (VarnodeData.size), `int4->i32`. The `read_unsigned_integer()? as int4`
  narrowings into `minSize/maxSize/afterBytes/maxPrimitives` mirror the C++
  `int4 x = decoder.readUnsignedInteger()` narrowing exactly.
- **Wrapping.** verified-and-pinned. The only arithmetic on uintb/uint4-lineage
  values is `justifyPieces` (`vndata.offset += offset`, `vndata.size -= offset`,
  modelrules.cc:691-693). Reached only when `sizeLeft < 0`, i.e. ≥1 piece was
  pushed and the piece size strictly exceeds the padding being removed, so the
  `u32 -=` cannot underflow on well-formed input — adversarial test
  `v_w6_s4_modelrules_justify_pieces_big_endian` exercises it (offset 0x10→0x11,
  size 4→3) without panic. parse-list overflow truncation is LOSS-091 (a).
- **Comparator totality.** clean. No `operator<`/sort/`partial_cmp` in the file;
  the `sizes` set is `BTreeSet<int4>` (total order on i32), and `*iter.next()` /
  `*iter.next_back()` reproduce C++ `*sizes.begin()` / `*--sizes.end()`.
- **Iteration-order provenance.** clean. Every loop is index iteration over a
  `Vec` (`tiles`, `base_tiles`/`alt_tiles`, `primitives`, `subQualifiers`,
  `preconditions`, `sideeffects`) reproducing the C++ `vector` index order, or
  `for entry in resource.get_entry()` reproducing the C++ `list<ParamEntry>`
  insertion order (ExtraStack). No HashMap/HashSet anywhere (grep clean).
- **Off-by-one / do-while / reverse iterators.** clean. No `do/while`; the
  `--sizes.end()` idiom is `next_back()`; `getFirstUnused` (modelrules.cc:1086)
  keeps the same "return current iter when unused, advance on consumed" shape and
  is NOT advanced by the caller after a consume (re-scanned from the same index),
  matching C++.
- **Erase-while-iterating.** clean. No erase/remove during traversal;
  `commonRefinement` builds a fresh `common` and `*first = common` (C++
  `first.swap(common)`); the active elements are cloned before the loop body so
  the push-into-`common` does not alias the read-from-`first`/`second` — values
  are copies in both languages.
- **Exception -> Result partial-state parity.** clean. `initializeEntries`/`Entry`
  throw `LowlevelError` BEFORE any caller mutation (decode tails), so `?`-propagation
  leaves the same (un)state. `ModelRule::assignAddress` runs preconditions+assign on
  a `tmp_status` copy and only `status.copy_from_slice(&tmp_status)` on
  non-fail before side-effects — identical to C++ (modelrules.cc:1660-1671).
- **Datatype compare / ParamEntry-ProtoModel sort / TypeFactory cache (this wave).**
  The one ptr-identity site is `HomogeneousAggregate::filter`'s
  `primitives.get(i).dt != base` (modelrules.cc:426), ported as `Rc::ptr_eq`
  (modelrules.rs:616). This is the correct Rust analogue of C++ `Datatype*`
  identity and is faithful *given* TypeFactory dedup — array elements share one
  `Rc` (`get_array_base` returns `Rc::clone(arrayof)`), struct fields share the
  stored field `Rc`. The dependency on dedup is documented (test
  `v_w6_s4_modelrules_homogeneous_aggregate_ptr_identity` pins both the
  shared-Rc ACCEPT and the distinct-Rc REJECT in the no-dedup harness).

## Findings

- F1 (loss, not a defect): the ported `ModelRule`/`AssignAction` machinery is a
  faithful but **standalone** transcription — it is never invoked by the live
  `ParamListStandard::assign_address`/`fillin_map`, which still fall through to
  `assign_address_fallback` (the `SEAM(w6-modelrules)` stub). No `Vec<ModelRule>`
  is owned by `ParamListStandard` (its `ModelRule` placeholder is `fspec.rs`-owned,
  out of scope). Recorded as **LOSS-090**.
       cpp: decompiler/cpp/fspec.cc ParamListStandard::assignAddress; modelrules.hh:531-554
       rust: rust/crates/kuna-decomp/src/fspec.rs:2389-2403; rust/crates/kuna-decomp/src/modelrules.rs:23-46
- F2 (loss, unreachable): `initFromSizeList` parse divergences — (a) a size in
  (2^31,2^63] truncates `as int4` vs C++ `INT_MAX` clamp; (b) Unicode
  `char::is_whitespace` vs C-locale `isspace`. Non-physical inputs only; the
  realistic comma/space/positive-int domain is bit-for-bit faithful (test
  `v_w6_s4_modelrules_size_list_parse_parity`). Recorded as **LOSS-091**.
       cpp: decompiler/cpp/modelrules.cc:280-294
       rust: rust/crates/kuna-decomp/src/modelrules.rs:2188-2237

No blocker or major finding stands. The decode dispatch, all ten AssignAction
assign/fillin bodies, the PrimitiveExtractor recursion (struct hole/align flags,
union common-refinement, array element offset stepping), and the filter predicates
re-derive to the C++ line-for-line.

## Adversarial tests
(`rust/crates/kuna-decomp/tests/verify_w6_s4_modelrules.rs`, committed 278185e)
- `v_w6_s4_modelrules_size_list_parse_parity` — comma/space size-list parsing,
  set-derived min/max membership, the `val<=0` "Bad filter size" throw, and the
  sizes/minsize mixing guard.
- `v_w6_s4_modelrules_justify_pieces_little_endian` — odd (3-byte) type, LE
  `<join>` default: addOffset=false, offset stays 0x10.
- `v_w6_s4_modelrules_justify_pieces_big_endian` — odd type, BE `<join>`:
  addOffset=true (BE^mostSig^justRight), offset 0x10→0x11, the `u32 size-=1`
  truncation does not panic.
- `v_w6_s4_modelrules_homogeneous_aggregate_ptr_identity` — array-of-4-floats
  ACCEPT, 5-float maxprimitives REJECT, mixed-struct REJECT, distinct-Rc struct
  REJECT (no-dedup), shared-Rc struct ACCEPT, int-aggregate REJECT.
- `v_w6_s4_modelrules_varargs_default_and_boundary` — INT_MIN/INT_MAX decode
  default, the `firstVarArgSlot<0` short-circuit, inclusive `[first,last]`,
  `first="0"`/`first="-1"`/`first="0" last="0"` ranges.

## losses
LOSS-090 (ModelRule not wired into live ParamListStandard — integration SEAM),
LOSS-091 (initFromSizeList overflow-clamp / whitespace-locale edge divergence).
