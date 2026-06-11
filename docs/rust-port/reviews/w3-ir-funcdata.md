# w3-ir-funcdata
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier, round 2)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (104 lib + 7 + 11 + 13 + 9 + 6 integration, 0 failed; 150 passing test lines; includes my 3 new verify_r2_* tests + the 8 round-1 verify tests).
clippy: `cargo clippy -p kuna-decomp -- -D warnings` -> clean. (`--tests` surfaces one PRE-EXISTING `doc list item without indentation` lint in `tests/verify_w3_ir_op.rs:434` — another item's file, NOT in this item's diff; `verify_w3_ir_funcdata.rs` is clippy-clean.)
blob-sha: funcdata.cc 6571bfe9…, funcdata.hh a9935d59…, funcdata_block.cc cad42f18… — all MATCH the recorded `cpp_blob_sha` (no drift).
scope note: round 2 adds commit 02e34c9 (the F1 repair) on top of the round-1 item commits d010513 (funcdata.rs/funcdata_block.rs/seams.rs) and 897a910 (round-1 verify tests). The branch also carries block.rs / verify_w3_ir_block.rs from item `w3-ir-block` (commits 45562c5 / 624921e, separately verified) — reviewed only as the API surface this item builds on, not re-judged here.

## Round-1 disposition
Round 1 REJECTed on F1 (MAJOR/blocker): `replace_reads_thunk` omitted the C++
`if (op->output == newvn) continue;` self-definition skip (VarnodeBank::replace,
varnode.cc:1362) and ran a blanket `destroy_descend(oldvn)`. Round 2 commit
02e34c9 **fixes F1 correctly** — re-verified below. No new blockers found.

## Hunt list (re-derived fresh for round 2)
- **Signedness.** clean. `force_goto`/`install_switch_defaults`/`node_join_create_block` loop `0..size_out()`/`0..num_jump_tables()`/`0..bblocks_get_size()` over non-negative int4 sizes; no int4-vs-uint4 comparisons. `get_slot`/slot indices are int4 matching C++.
- **Integer widths.** clean. `uintm=u32` SeqNum order; `int4=i32`; `uint4=u32` flags. The repaired thunk's `get_slot` returns int4 (note: C++ getSlot returns `i==n` on no-match, NOT -1; the Rust port faithfully returns `n` — op.rs:599). The thunk's `if i < 0 { continue; }` guard is therefore effectively DEAD against the real return convention; informational only — see F6.
- **Wrapping.** clean. `bb_insert_op` midpoint `(ordafter/2).wadd(ordbefore/2)`, the `ordafter = ordbefore.wadd(0x1000000)` `<= ordbefore -> uintm::MAX` clamp, `ordafter.wsub(ordbefore) <= 1`, and `bb_set_order`'s `(uintm::MAX / n).wsub(1)` all transcribe block.cc:2262/2686 unsigned arithmetic exactly. No new arithmetic in the round-2 diff.
- **Comparator totality.** N/A — funcdata has no custom Ord; `jumpvec` is a `Vec`.
- **Iteration-order provenance.** clean. The repaired thunk snapshots `oldvn.descend` in push_back order (`descend_iter`) and replays it 1:1 against the C++ `while(iter!=descend.end())` walk; `erase_descend(oldvn, op)` removes the FIRST matching occurrence, mirroring the C++ `descend.erase(tmpiter)` cursor (verified for multiplicity below). `jumpvec` Vec preserves C++ `vector` order; `bb_ops`/`bb_set_order` walk the intrusive head→next chain (C++ `list` order). No HashMap/HashSet anywhere (grep = 0).
- **Off-by-one / do-while / reverse iterators.** clean. `bb_insert_op` neighbour computation and the `op.begin()`/`op.end()` boundary cases match C++. No do-while/reverse iterators in scope.
- **Erase-while-iterating.** clean (F1 RESOLVED). The thunk now mirrors C++ `VarnodeBank::replace`'s `tmpiter=iter++; descend.erase(tmpiter)` exactly: snapshot-then-per-entry-`erase_descend`, with the `op->output==newvn` skip restored (the skipped op keeps its oldvn descend link and oldvn-reading slot; the blanket `destroy_descend` is gone). The multi-read / interleaved-self-def cases preserve descend multiplicity and the `getSlot` slot-advance — verified by my 3 new tests.
- **Exception -> Result partial-state parity.** clean. `splice_block_basic` returns `Err` BEFORE mutating on the invalid-chain, trailing-branch (opDestroy seam), and leading-MULTIEQUAL paths (matching the C++ throw points before any op move / flag clear); `remove_from_flow_split` checks `bb_empty_op` first. The repaired thunk only `?`-propagates on `add_descend`'s "Free varnode has multiple descendants" guard, which is the SAME guard C++ `addDescend` throws (varnode.cc:336) — partial state on that throw is identical.

## Findings
- F1 (MAJOR — round-1 blocker) — **RESOLVED in round 2 (commit 02e34c9).** The thunk now has `if obank.get(op).and_then(|o| o.get_out()) == Some(newvn) { continue; }` before repointing, severs exactly one descend link per repointed entry via `erase_descend(oldvn, op)`, and no longer runs a blanket `destroy_descend(oldvn)`. Faithful transcription of varnode.cc:1359-1368. Re-derived line-by-line against the C++ oracle and confirmed by `thunk_repoints_self_def_input_cpp_skips_it` (now asserts the C++-correct `Some(oldvn)`) plus my 3 multiplicity tests.
      cpp: decompiler/cpp/varnode.cc:1351-1368 (VarnodeBank::replace, incl. :1362 skip)
      rust: rust/crates/kuna-decomp/src/funcdata.rs:543-589 (replace_reads_thunk, :569-571 skip, :581 single erase)
- F2 (minor): three bare `as` casts without the protocol `// cast:` justification comment — `funcdata.rs:510` `self.jumpvec.len() as int4`, `:514` `self.jumpvec[i as usize]`, `:757` `n as uintm` (the setOrder size_t/uintm divisor). All semantically faithful to the C++ implicit conversions; documentation-only. Unchanged from round 1; not a blocker.
      cpp: decompiler/cpp/funcdata.hh (numJumpTables/getJumpTable inline), block.cc:2693 (setOrder)
      rust: rust/crates/kuna-decomp/src/funcdata.rs:510,514,757
- F3 (informational): `splice_block_basic` moves outbl's ops via per-op `bb_insert_op(.., None)`, which can set `f_switch_out` on `bl` if a moved op is BRANCHIND — C++ `list::splice` (funcdata_block.cc:957) has no flag side-effect. Reconciled by the trailing `bb_set_order(bl)`; the splice caller never splices a block whose successor leads with a branch, and the method already `Err`s on a trailing branch. Behaviorally inert. Unchanged from round 1.
      cpp: decompiler/cpp/funcdata_block.cc:957 vs block.cc:2289-2291
      rust: rust/crates/kuna-decomp/src/funcdata_block.rs:357-360
- F4 (informational): `bb_set_order` divisor `uintm::MAX / n as uintm` panics (div-by-zero) only if `n` is a nonzero multiple of 2^32 (>4 billion ops in one block) — unreachable. C++ `step/op.size()` promotes to size_t. Theoretical extreme differs; neither reachable.
      cpp: decompiler/cpp/block.cc:2693
      rust: rust/crates/kuna-decomp/src/funcdata.rs:757
- F5 (accepted W4/W3-op seam — now ledgered as LOSS-034): `clear_jump_tables` drops the whole vector (C++ preserves overrides), `structure_reset` omits the dead-table sweep, `install_switch_defaults` is a no-op body, and the op-graph data-flow methods + `spliceBlockBasic`'s trailing-branch `opDestroy` return `Err`/are deferred. All explicit `// SEAM(W4)`/`// SEAM(W3-op)`; jumpvec holds opaque `JumpTableId` handles only. Round 1 left this unledgered ("subsumed"); round 2 records LOSS-034 for ledger completeness (matches the LOSS-030/LOSS-033 file-scoped-deferral pattern).
      cpp: decompiler/cpp/funcdata_block.cc:42,706,723,942
      rust: rust/crates/kuna-decomp/src/funcdata_block.rs:78,94,188,335
- F6 (informational, new in round 2): the repaired thunk's `let i = ...get_slot(oldvn)...unwrap_or(-1); if i < 0 { continue; }` guard is dead against the real `get_slot` return convention — C++ `getSlot` and the Rust port both return `n` (num_input) on no-match, never -1, so `i < 0` only fires when `op` is a stale id (`get` returns None). The not-found case is in any event unreachable: each descend entry corresponds 1:1 to a live oldvn-reading slot, so `get_slot` always finds one (verified by the multiplicity tests). The comment at funcdata.rs:572-575 ("-1 only if a prior entry consumed the read") is slightly inaccurate (the port returns `n`, not -1; the guard catches the stale-op case). No behavioral consequence; documentation nit only.
      cpp: decompiler/cpp/op.hh:168 (getSlot returns i==n on no-match)
      rust: rust/crates/kuna-decomp/src/funcdata.rs:576-579, src/op.rs:599

## Faithful (re-derived fresh, no finding)
- `replace_reads_thunk` (round-2 body): line-by-line match to VarnodeBank::replace including the skip, the single-link erase, the descend-before-setInput ordering (`add_descend(newvn,op)` then `set_input(newvn,i)` mirrors C++ "addDescend then setInput AFTER descend updated"), and the multi-read slot-advance (getSlot returns 0 then 1 as slot 0 stops matching). The C++ `addDescend` free-varnode guard (varnode.cc:336) is preserved and exercised.
- `bb_insert_op`/`bb_remove_op`/`bb_set_order`: SeqNum order math, intrusive head/tail/len bookkeeping, BRANCHIND `f_switch_out` mark — transcribe block.cc:2262/2296/2686.
- `force_goto`, `switch_edge`, `remove_from_flow_split`, `node_join_create_block` (incl. `getOutIndex(exitb)` computed AFTER moving swapa's edge — faithful C++ evaluation order), `node_split_block_edge`, `set_basic_block_range`, the cover helpers — match funcdata_block.cc.
- `clear()`/constructor/flag toggles and the seams.rs `Architecture`/`Scope`/`FuncProto` W4 skeletons — seam placeholders with no hunt-list surface.

## Adversarial tests
Round-2 additions (rust/crates/kuna-decomp/tests/verify_w3_ir_funcdata.rs, commit 0186923, all pass):
  - verify_r2_multi_slot_read_repoints_every_slot — op reads oldvn in 2 slots; both repoint, newvn descend multiplicity = 2 (the erase_descend + getSlot-advance core of the F1 fix).
  - verify_r2_self_def_skip_preserves_other_readers — self-def op skipped while a sibling ordinary reader repoints; oldvn keeps ONLY the skipped link, newvn gains ONLY the repointed reader.
  - verify_r2_interleaved_selfdef_and_double_reader — self-def descend entry interleaved between a double-reader's two entries; getSlot still yields 0 then 1 for the double reader across the skip.
Carried from round 1 (8 tests, all pass): thunk_repoints_self_def_input_cpp_skips_it (now asserts C++-correct Some(oldvn)), thunk_repoints_ordinary_reader_matches_cpp, bb_insert_orders_strictly_increasing_and_midpoint, bb_insert_gap_collapse_triggers_full_reorder, bb_set_order_spreads_with_uniform_step, splice_empty_bl_inherits_outbl_ops_in_order, splice_clears_startbasic_on_merged_head, bb_remove_reinsert_roundtrips_links.

losses: LOSS-034 (funcdata.cc/funcdata_block.cc jump-table + op-graph methods seam-deferred at w3-ir-funcdata) — the only accepted divergence; all other findings are minor/informational and non-blocking.
