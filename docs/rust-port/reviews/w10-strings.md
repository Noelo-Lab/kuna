# w10-strings
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier (round 1)
date: 2026-06-15
branch: rport/w10-strings @ 4a65fb2 (porter 253e0df + this verifier's adversarial commit)
gate: `cd rust && cargo test --workspace` -> all green, 0 failed (constseq lib module 20/20,
      incl. 3 new W10 probes); `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean
      (forced rebuild of constseq.rs); C++ oracle `python -m kuna.run_tests --all --baseline
      docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK (C++ tree byte-untouched).

## Scope

Diff touches ONE file: `rust/crates/kuna-decomp/src/constseq.rs` (+ this verifier's tests in
`constseq/tests.rs`). It ports the **gathering half** of `HeapSequence` (the STORE-through-pointer
driver for `RuleStringStore`, constseq.cc:486-713,934-948) and wires the type-guard half of
`RuleStringStore::applyOp`. The `transform()` half stays a documented keystone seam (declines,
byte-identical to the rule being disabled).

## Keystone-reserved file check: PASS

`git diff rust-port...rport/w10-strings --name-only` = `constseq.rs` only. None of
merge.rs / funcdata.rs / funcdata_merge.rs / funcdata_union.rs / varnode.rs / variable.rs /
coreaction_cleanup.rs / cast.rs / typeop.rs / dtype.rs / unionresolve.rs / printc.rs is touched.
No collision with the concurrent keystone wave.

## No special-casing: PASS

grep for function-name / address / value / string-content hardcoding in the logic: none. The only
string literals are Rule group/name registration (`"stringstore"`, `"stringcopy"`, `"analysis"`)
matching the C++ `Rule(grp,0,"stringstore")` constructor — not input special-casing.

## Hunt list

- Signedness — see FINDING F1. The PTRADD-mult compare at `find_base_pointer` (constseq.rs:478)
  and `find_initial_stores` (:589) truncates `getIn(2)->getOffset()` to `int4` before comparing to
  `ptr_add_mult`; the C++ keeps full `int8`/`uintb` width. `calc_ptradd_offset` (:656) is faithful
  (full `uint8` compare). Internally inconsistent; minor (LOSS-177).
- Integer widths — `uintb`=`uint8`=u64, `int4`=i32 mappings consistent. `ptrAddMult =
  byteToAddressInt(alignSize, wordSize)` ported as `byte_to_address_int(... as i64) as int4` —
  matches C++ `int4 ptrAddMult` (constseq.cc:937). `arrSize = moveOps.size()*alignSize` as int4 — faithful.
- Wrapping — clean. `calc_add_elements` `res += / *=` -> `wrapping_add`/`wrapping_mul` (uint8, C++
  unsigned wrap); `calc_ptradd_offset` `off *= mult`, `res += off` -> wrapping; `collect_store_ops`
  `(curOffset - baseOffset) & wrapMask` -> `wrapping_sub(...) & wrap_mask` (the C++ "allow wrapping"
  comment); `find_duplicate_bases` `off *= getIn(2)` -> `wrapping_mul`. All verified by the new
  `w10_calc_add_elements_depth_and_wrap` probe (u64::MAX+5 wraps to 4, no debug panic).
- Comparator totality — `sets_equal` is element-wise equality on caller-ordered Vecs (C++
  `setsEqual`), not an `operator<`; no ordering hazard. `move_ops` ordering is decided by the base's
  `check_interference` sort (already ported/verified), not by gather collection order.
- Iteration-order provenance — clean. Every container walk reproduces a C++ order:
  * `find_duplicate_bases`/`find_initial_stores` descend loops -> `descend_snapshot` = the
    `Varnode` def-use `list` order exactly (funcdata_varnode.rs:888, doc-asserted).
  * `find_initial_stores` worklist `while(pos < ptradds.len())` with index `pos` + push-back =
    C++ `while(pos < ptradds.size())`; faithful BFS over the same list.
  * `find_duplicate_bases` reverse loop `for i in (0..offset.len()).rev()` = C++
    `for(int4 i=offset.size()-1;i>=0;--i)`; `duplist.swap(midlist); duplist.clear()` = C++
    `duplist.swap(midlist); duplist.clear()` (mem::swap). No HashMap/HashSet anywhere.
- Off-by-one / do-while — `find_duplicate_bases`'s C++ `do {...} while(getIn(1)->isConstant())`
  ported as `loop { ... ; if !is_constant break }` keeping at-least-once (the pre-loop guard already
  proved iter-1's const-offset op). Verified by `w10_find_duplicate_bases_do_while_at_least_once`.
- Erase-while-iterating — N/A. The gather half is **read-only over the IR** (op/var banks only);
  no list/map mutation during traversal. `descend_snapshot` returns an owned Vec precisely so no
  alias/erase hazard exists.
- Exception -> Result partial-state — N/A. The ported methods never throw/`?`; the constructor's
  early `return seq` on `collect_store_ops`/`check_interference` failure leaves `num_elements==0`
  (`is_valid()==false`), exactly as the C++ ctor leaves the object invalid (constseq.cc:940-943).

## Mechanical pass

- `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`partial_cmp`/`unwrap` in the new
  code: NONE.
- Bare `as` casts: present without `// cast:` comments (process nit per verification.md). The
  material ones are the two `as int4` truncations (F1); the rest (`as i64`/`as uint8`/`as usize`/
  `as u64`) faithfully transcribe C++ implicit widening conversions.
- clippy `-p kuna-decomp --lib -D warnings`: clean after a forced recompile of constseq.rs.

## Findings

- F1 (minor / faithfulness, debug-and-release-reachable but non-physical): the PTRADD element-size
  compare truncates the 64-bit offset to `int4` before testing equality with `ptr_add_mult`, so a
  crafted PTRADD whose mult constant has high bits set but low-32 == `ptr_add_mult` is ACCEPTED by
  the Rust back-walk where the C++ full-width compare REJECTS it. Two sites:
       cpp: decompiler/cpp/constseq.cc:491-492 (`int8 sz = op->getIn(2)->getOffset(); if (sz != ptrAddMult) break;`)
            decompiler/cpp/constseq.cc:584 (`if (op->getIn(2)->getOffset() != ptrAddMult) continue;`)
       rust: rust/crates/kuna-decomp/src/constseq.rs:478-479 and :589 (`... as int4 ... != self.ptr_add_mult`)
  Note site C (`calc_ptradd_offset`, constseq.rs:656) is faithful (`mult != self.ptr_add_mult as uint8`,
  full u64), making the truncation internally inconsistent. Reachable only via overflowing PTRADD
  mults (in real lifted code the element multiplier is a small constant), so the divergence is benign
  in practice -> recorded as LOSS-177, not a blocker. Pinned by
  `w10_f1_find_base_pointer_truncates_mult_diverges_from_cpp`.

## Losses

- LOSS-088 (extended): the constseq TRANSFORM half stays keystone-seamed. This branch ADVANCES
  LOSS-088 by porting the `HeapSequence` gather half (ctor, findBasePointer, findDuplicateBases,
  findInitialStores, calcAddElements, calcPtraddOffset, setsEqual, testValue, collectStoreOps) and
  the `RuleStringStore::applyOp` type-guard chain (getTypeReadFacing/TYPE_PTR/getPtrTo/isCharPrint/
  isOpaqueString -> HeapSequence::new -> is_valid). `HeapSequence::transform` (constseq.cc:954-967:
  buildStringCopy/gatherIndirectPairs/deduplicatePairs/removeStoreOps -> getInternalString/
  newUniqueOut/updateType/newVarnodeIop/markIndirectCreation/opInsertBefore/totalReplace + the
  `builtin_strncpy(...,"...",n)` printc rendering) declines (returns false), so `applyOp` returns 0
  — byte-identical to the rule being disabled. The universalaction-listing parity test still treats
  the string rules as effectively unported (still green).
- LOSS-177 (new): the F1 `as int4` PTRADD-mult truncation at find_base_pointer / find_initial_stores.

## Adversarial tests (committed to rport/w10-strings @ 4a65fb2)

In `rust/crates/kuna-decomp/src/constseq/tests.rs` (white-box; the gather methods are
`pub(crate)`/private and unreachable from an integration `tests/` crate):
- `w10_f1_find_base_pointer_truncates_mult_diverges_from_cpp` — pins F1 (the truncation accepts a
  PTRADD the C++ rejects).
- `w10_calc_add_elements_depth_and_wrap` — depth-3 sum + non-const leaf collection, maxDepth==0
  whole-node-as-leaf, and u64 wrap (u64::MAX+5 -> 4, no panic).
- `w10_find_duplicate_bases_do_while_at_least_once` — do-while runs once on a single-PTRSUB base and
  the reverse offset-match retrace re-collects exactly the base pointer.

## Regression checks

- C++ oracle: 207/207 + 675/675 PARITY OK; C++/spec/baseline files byte-untouched by the branch.
- Rust whole-suite `cargo test --workspace`: 0 failed; parity-fence tests intact
  (`printc_parity`, `universalaction_listing`, `print_b5_boolless` 11/11, B3 SSA parity, byte-parity
  edges). The 187-function e2e parity count is unchanged (transform declines, so behavior is still
  the disabled-rule baseline).
- clippy `-p kuna-decomp --lib`: clean.
