# w3-ir-funcdata-op
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (round 1)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (184 tests: 131 lib incl. 20 in-module funcdata_op + sibling lib tests, 7 verify_w3_ir_funcdata_op, + sibling verifier suites; 0 failed)

## Scope

cpp_blob_sha verified: `decompiler/cpp/funcdata_op.cc` hashes to
`d3a0c4965965a97bca5ee246edfe42570d7a3f5f` (matches checklist). The branch is
built on the verified deps (w3-ir-block, w3-ir-funcdata); the ONLY commit that is
this item is `f93dc73`, touching exactly `rust/.../funcdata_op.rs` (+1028) and
`funcdata_op/tests.rs` (+509). The block.rs/funcdata.rs/funcdata_block.rs/seams.rs
deltas in `git diff rust-port...` belong to the already-verified dependency items
and were not re-reviewed except for the supporting-API surfaces this item calls
(`bb_insert_op`/`bb_remove_op`/`bb_op_head`/`bb_op_tail`, the op-bank
create/mark/iter methods, `add_descend`/`erase_descend`/`set_def`) — spot-checked
faithful.

## Hunt list

- Signedness: clean. `op_destroy_recursive`'s `int4 pos` vs `scratch.size()` C++
  sign-compare is ported as `usize pos < scratch.len()`; pos only increments from 0
  and the vector is non-empty, so no boundary divergence. `op_set_all_input` index
  loops are `0..num_input()` over an `int4`-typed count (small, non-negative).
- Integer widths: clean. No `intb*intb`-into-wider promotions in this file; the
  only arithmetic (`1 - subtest`) is on a 0/1/2 `int4` flip-result, faithful.
- Wrapping: clean (this file is graph mutation, not arithmetic). The bb-order
  midpoint math lives in funcdata.rs (`bb_insert_op`) and uses `wadd`/`wsub`
  (w3-ir-funcdata's, verified there).
- Comparator totality: n/a — no comparators defined here. `op_target`'s `dead_prev`
  BTreeMap is keyed by `OpId` (a total slotmap key); used only for predecessor
  lookup, not ordering. `find_op`/`iter_at`/`target_lower_bound` (op.rs) order by
  `SeqNum`, out of scope.
- Iteration-order provenance, per loop:
  - `get_first_return_op`: `obank.iter_code(CPUI_RETURN)` = the per-opcode
    intrusive list in list order (C++ `beginOp(RETURN)`). Faithful.
  - `find_primary_branch`: walks the caller-supplied `&[OpId]` in order (C++
    `PcodeOpTree::const_iterator` range). Order is the caller's responsibility.
  - `op_insert_before` backward / `op_insert_after`+`op_insert_begin` forward /
    `op_insert_end`: all walk the basic-block intrusive `prev`/`next` links
    (`basic_neighbours()`), re-deriving the C++ `--iter`/`++iter` exactly — traced
    statement-for-statement, all four boundary cases (begin/end/empty) match.
  - `op_destroy_recursive`: index walk over a worklist `Vec` (C++ `scratch[pos]`),
    same push order.
  - `op_target` dead case: `iter_dead()` (forward dead-list order) folded into a
    predecessor map; `--insertiter` == previous dead-list element. Faithful. No
    `HashMap`/`HashSet` in ported logic (the one BTreeMap is order-irrelevant).
- Off-by-one / do-while / reverse iterators: clean. `op_target`'s while-loop keeps
  the at-least-zero (returns op itself if it has `startmark`) C++ semantics
  (condition tested first). `op_insert_end`'s `--iter`/`++iter` flow-break dance
  is reproduced with the empty-block guard. No `do/while` in this file.
- Erase-while-iterating: n/a — no erase-during-traversal here (`op_destroy` is
  called after the per-op input scan completes; `op_destroy_recursive` mutates the
  worklist by append only, then destroys at the tail of each iteration).
- Exception -> Result partial-state parity: the four Err-returning stubs
  (`op_set_output`, `clone_op`, `op_destroy_raw`, `op_flip_in_place_execute`) leave
  the SAME partial state the C++ leaves before the deferred step
  (`op_set_output` runs both `opUnsetOutput`s before erroring; `clone_op` creates
  the op shell + copies flags before erroring). The const-reshare `?` in
  `op_set_input` propagates `add_descend`'s Err BEFORE `op->setInput` — see F1.
- SeqNum/create_index allocation-order fidelity (ADR 0001): `new_op`/`new_op_seq`
  delegate to the bank's `create_at`/`create_seq` (uniqid -> SeqNum.uniq);
  verified monotonic + the `create_seq` `uniqid = sq.getTime()+1` bump
  (in-module tests `new_op_allocates_uniq_monotonically`/`new_op_seq_bumps_uniqid`).
- Intrusive-list splice-order fidelity (ADR 0001): `op_insert`->`bb_insert_op`,
  `op_uninsert`/`op_destroy`->`bb_remove_op`, `op_dead_insert_after`->
  `insert_after_dead` — all keep the C++ splice positions (verified via the
  ordering tests and the dead-list discipline tests).

## Mechanical pass

- grep funcdata_op.rs(+tests): no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`. Two `as` casts, both benign index conversions
  (`vvec.len() as int4` at :297, `i as usize` at :301) — array length/index, not
  value-semantics truncation; a PcodeOp input array cannot exceed i32::MAX. (Minor:
  neither carries a `// cast:` justification; not load-bearing.)
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean (covers funcdata_op.rs).
  (`--tests -- -D warnings` errors only in the SIBLING `tests/verify_w3_ir_op.rs:434`
  doc-lazy-continuation lint, out of this item's scope; this item's lib + test file
  are clean.)

## Findings

- F1 (minor / documented-loss): `op_set_input` constant-reshare divergence.
  The C++ re-duplicates a constant that already has a descendant (constants must
  have a single descendant) via `newConstant`/`copySymbol` so the fresh copy can be
  linked. The Rust seam (funcdata_varnode `newConstant` not yet available) links the
  SAME constant directly, so `add_descend`'s "free varnode has multiple
  descendants" guard fires and the `?` returns Err — the link does NOT proceed. The
  common single-descendant case is faithful; only the reshared case diverges (Err
  vs dedup). The in-method comment (:255-256) "the link still proceeds, exactly as
  the C++ does after the guard" is imprecise for the reshared case. No silent
  corruption (Err is honest; failed link leaves the original reader intact —
  verified by `w3_ir_funcdata_op_const_reshare_errs_where_cpp_dedups`).
       cpp: decompiler/cpp/funcdata_op.cc:104-125
       rust: rust/crates/kuna-decomp/src/funcdata_op.rs:245-267
  -> ledgered as LOSS-035 (restore the dedup + fix the comment when newConstant lands).

- F2 (minor / documented-loss): four cross-wave Err stubs + two behavioral seams.
  `op_set_output` (setDef needs a banks_mut split-borrow), `clone_op`/`op_destroy_raw`
  (cloneVarnode/destroyVarnode), `op_flip_in_place_execute` (get_booleanflip+W6+
  replaceLessequal) return seam Errs with the C++-faithful partial state; `op_destroy`
  unsets the output instead of destroying it (graph stays consistent, orphan free
  Varnode left for destroy_varnode); `op_insert_after` skips the iop INDIRECT
  redirect (no-op for non-iop inputs). All are genuine factory/W6/two-bank seams
  absent at this parallel item's boundary and have no in-wave consumer.
       cpp: decompiler/cpp/funcdata_op.cc:70,203,253,373,616,1296
       rust: rust/crates/kuna-decomp/src/funcdata_op.rs:172,354,439,509,602,822
  -> ledgered as LOSS-035.

No blockers or majors. The ported half (op-graph mutation primitives, SeqNum
allocation, SSA ordering invariants, flip-list tracer, scans, control-flow walks)
is faithful line-against-line to the C++, with no silent semantic divergence.

## Adversarial tests

`rust/crates/kuna-decomp/tests/verify_w3_ir_funcdata_op.rs` (7, all pass, committed
on the branch as 054be41):
- `w3_ir_funcdata_op_const_reshare_errs_where_cpp_dedups` — PINS F1 (Err vs dedup,
  no corruption).
- `w3_ir_funcdata_op_nonconst_free_reshare_errs_like_cpp` — the plain addDescend
  invariant (control, distinguishes F1 from a generic reshare difference).
- `w3_ir_funcdata_op_insert_before_all_indirect_lands_at_begin` — backward walk
  reaches beginOp.
- `w3_ir_funcdata_op_insert_after_all_multiequal_lands_at_end` — forward walk
  reaches endOp.
- `w3_ir_funcdata_op_target_stops_at_nearest_startmark` — delay-slot: nearest of
  two startmarks.
- `w3_ir_funcdata_op_flip_bool_and_order_and_return` — fliplist [d0,d1,and] +
  return subtest1.
- `w3_ir_funcdata_op_next_op_through_empty_block` — single-out flow across an empty
  successor.

## Losses

LOSS-035 (funcdata_op.cc varnode-factory / W6-table / two-bank methods
seam-deferred). Appended to `docs/rust-port/losses.md` (main tree).
