# w6-s5-constseq-prefersplit
verdict: ACCEPT-WITH-LOSSES
verifier: subagent (independent verifier, round 1)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> GREEN for every in-scope test (lib 1033 pass incl. the 3 new constseq probes; verify_w6_s5_constseq_prefersplit 3 pass; catalog_bytecompat 5 pass). The ONLY red is the pre-existing, out-of-scope `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`, which fails with "Could not find .sla file for x86:LE:64:default" — a gitignored build artifact absent from the fresh worktree's `specs/` (the file is untouched by this branch; the x86 `.sla` exists only in the MAIN tree, not the worktree). Item-independent environment gap.

scope: C++ `decompiler/cpp/constseq.{cc,hh}` + `prefersplit.{cc,hh}`. Ported in commit 1877f0c: the reusable `ArraySequence` base (constants/`WriteNode` order comparator/`interfere_between`/`check_interference`/`form_byte_array`/`select_string_copy_function` structure) and the FULL `PreferSplitManager` (record total order, `find_record`, `fillin_instance`, all `test*`/`split*` pairs, `split_varnode`/`split_record`/`split_temporary`/`split_additional`, `ELEM_PREFERSPLIT`). `StringSequence`/`HeapSequence` transform halves + `RuleStringCopy`/`RuleStringStore` deferred (LOSS-088).

## hunt list
- **Signedness**: clean. `check_interference`'s `pos == size()` not-found uses `move_ops.len()` sentinel (constseq.rs:203-211) — matches C++ `pos == moveOps.size()`. The `(ending_pos as int4) - (starting_pos as int4) < MINIMUM_SEQUENCE_LENGTH` window check (constseq.rs:237) is signed-int4, matching C++ `endingPos - startingPos < MINIMUM_SEQUENCE_LENGTH`. `find_record` size compare is `u32` (Reverse) matching C++ `uint4 storage.size` (prefersplit.rs:81). The `suboff = ...getOffset() as int4` casts (prefersplit.rs:510,530,940) match C++ `(int4)op->getIn(1)->getOffset()`.
- **Integer widths**: clean. `int4`->`i32`, `uint8`/`uintb`->`u64`, `uint1`->`u8`, `uint4`->`u32` consistent. `WriteNode.order` cached as `u32` (`getSeqNum().getOrder()` is `uintm`) — the C++ dereferences the live `PcodeOp*` at compare time; caching at construction is sound (order is immutable once sequenced) and documented.
- **Wrapping**: F1 (LOSS-087, minor). `form_byte_array`'s `byte_pos - root_off` correctly uses `wrapping_sub` + `<0` skip (constseq.rs:275), and `max_off = root_off.wrapping_add(...)` (constseq.rs:312) is correct, BUT `byte_pos + el_size` (constseq.rs:276) and the constant-split shifts `origval >> (8*losize)` (prefersplit.rs:272,450) are plain ops that DEBUG-panic at the int4-MAX / shift-by-64 boundaries where C++ wraps. Debug-only, release-identical, non-physical inputs — ledgered LOSS-087.
- **Comparator totality**: clean. `PreferSplitRecord::cmp_key` = `(i32 space_index, Reverse<u32> size, u64 offset)` is a derived lexicographic total order, exactly C++ `operator<` (space index asc, size DESC, offset asc). No floats. Brute-forced antisymmetry+transitivity over a mixed-space fixture in `w6s5_record_comparator_is_total_order` (PASS).
- **Iteration-order provenance**: clean. `split_record`/`split_additional` collect `iter_loc_size_addr`/`descend_iter` into `Vec` worklists (provenance: the C++ `VarnodeLocSet`/`list` order, reproduced by the previously-verified loc-probe + descend iterators). `find_record` uses `partition_point` over the sorted `Vec` (C++ `vector`+`lower_bound`). NO HashMap/HashSet (grep clean).
- **Off-by-one / do-while / reverse iterators**: clean. `check_interference`'s backward walk (`starting_pos: i64 = pos-1 .. >=0`, then `+1`) and forward walk (`ending_pos = pos+1 .. < len`) reproduce the C++ index loops exactly, incl. the inclusive/exclusive window arithmetic (constseq.rs:214-245). `find_record` lower_bound (`partition_point(|r| key < tkey)`) + `if tkey < rec.key` equality probe match C++ `lower_bound` + `if (templ < *iter)` (prefersplit.rs:862-869) — verified at all boundaries in `w6s5_find_record_lower_bound_boundaries` (PASS).
- **Erase-while-iterating**: clean. `split_temporary` re-fetches `descend_iter().next()` each loop iteration (prefersplit.rs:818-825), matching C++ `while(beginDescend()!=endDescend()) readop=*beginDescend()`; SUBPIECE rewrite (input 0 reassigned to lo/hi) and STORE destroy both remove `readop` from `vn`'s descend list, so the loop makes progress. The added `default => break` (prefersplit.rs:836) is on a path `testTemporary` proves unreachable (SUBPIECE/STORE-only readers) — behavior-equivalent. `split_record` does `pos += 1` (the C++ `++iter`) BEFORE `split_varnode` and re-collects only on success (the C++ "regenerate iterators") — exact. `split_additional`'s descend snapshot is over a non-mutating push-only loop — snapshot == live.
- **Exception -> Result parity**: clean. No C++ throw sites in scope; the port has no `?` early-returns mid-mutation. The `.ok()` on `op_set_*`/`op_get_out` and `.expect(...)` on invariant-guaranteed lookups (e.g. `written but no def`) mirror C++ unconditional pointer use; all `expect`s are on paths the C++ likewise assumes non-null (`vn->getDef()` after `isWritten()`, etc.).
- **This wave's extra hunt** (Datatype::compare total-order, ParamEntry/ProtoModel sort, TypeFactory cache): the relevant surface here is `PreferSplitRecord`'s sort order — verified total and direction-correct (size DESC) via `w6s5_initialize_full_order_across_spaces` (PASS), which pins the full cross-space materialized order so a flipped `Reverse` would be caught.

## mechanical pass
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`panic!` over both src files: NONE.
- bare `as` casts: all are faithful int4-truncation / index conversions matching C++ implicit narrowing (e.g. constseq.rs:275 `as int4` = C++ `int4 bytePos = uint8 - uint8`; prefersplit.rs:198 `as int4` = C++ `(int4)getOffset()` in `getSpaceFromConst`). They lack `// cast:` justification comments (a documentation nit, not a correctness finding) — noted, not blocking.
- `cargo clippy -p kuna-decomp -- -D warnings`: CLEAN.

## findings
- F1 (minor): debug-build overflow/shift-panic divergence class. `form_byte_array` `byte_pos + el_size` and `fillin_instance`/`split_zext` `origval >> (8*losize)` panic in debug at int4::MAX / shift-by-64; C++ wraps (UB-but-wraps) and continues. Debug-only, release byte-identical, inputs non-physical (bounded `StringSequence` caller keeps `byte_pos` in `[0,sz)`; valid records keep `0 < splitoffset < size`). Same class as LOSS-086. ACCEPTED as LOSS-087.
       cpp: decompiler/cpp/constseq.cc:116 ; decompiler/cpp/prefersplit.cc:47,177
       rust: rust/crates/kuna-decomp/src/constseq.rs:276 ; rust/crates/kuna-decomp/src/prefersplit.rs:272,450

## adversarial tests
- rust/crates/kuna-decomp/tests/verify_w6_s5_constseq_prefersplit.rs (external, pub surface):
  - `w6s5_record_comparator_is_total_order` — antisymmetry+transitivity brute-force over mixed spaces/sizes/offsets + size-desc spot-check.
  - `w6s5_initialize_full_order_across_spaces` — full materialized sort order pinned (catches a flipped `Reverse`).
  - `w6s5_find_record_lower_bound_boundaries` — buried hit / past-end miss / before-start miss / first / last.
- rust/crates/kuna-decomp/src/constseq/tests.rs (in-crate, pub(crate) `form_byte_array`):
  - `w6s5_form_byte_array_offset_before_root_is_skipped_not_panic` — wrapping_sub + `<0` skip, no panic.
  - `w6s5_form_byte_array_huge_bytepos_diverges_debug_panic` — `#[should_panic]` pinning F1.
  - `w6s5_form_byte_array_little_endian_multibyte_shift` — `val >>= 8` per-element little-endian unpacking.

## losses
- LOSS-087 (F1 — debug-only overflow/shift divergence class in form_byte_array + fillin_instance/split_zext).
- LOSS-088 (deferred StringSequence/HeapSequence transform halves, RuleStringCopy/RuleStringStore, and `select_string_copy_function`'s `getTypeChar` identity check -> BUILTIN_MEMCPY fall-through).
