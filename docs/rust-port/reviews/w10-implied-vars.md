# w10-implied-vars
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent verifier, round 1)
date: 2026-06-14
branch: rport/w10-implied-vars (8473933) vs base rust-port (fde75bb)

## Scope of the diff under review

The diff is small and single-file (102 lines, `rust/crates/kuna-decomp/src/coreaction_cleanup.rs`), three changes:

1. `ActionMergeCopy::apply` un-seamed: now drives `Merge::merge_opcode(data, CPUI_COPY)` over the `MergeContext for Funcdata` bridge (was a `// SEAM` no-op).
2. `ActionCopyMarker::apply` un-seamed: now drives `Merge::mark_internal_copies(data)` (was a `// SEAM` no-op).
3. `mark_output_storage_addr_tied` refined: the recovered return-value storage is left UN-tied (eligible to be marked IMPLIED + inlined) only for the genuine declared-output / single-def / pure-transient lone-COPY shape — a kuna structural stand-in for `ScopeLocal::inScope`.

NOTE on the task framing: `base_explicit` (port of `ActionMarkExplicit::baseExplicit`), the `ActionMarkExplicit`/`ActionMarkImplied` apply walks, and `check_implied_cover` (port of `checkImpliedCover`) were introduced in the PRIOR commit `82bdfc4` (rport/w10-structure-printbody), NOT this diff. They were re-derived line-by-line against the C++ anyway (below) because this item is the first to make them actually FIRE (the inlining was previously a no-op seam). `check_implied_cover` is a `return true` stub (D2 in LOSS-155); the over-inlining risk that creates is bounded by the D1 un-tie guard and verified clean on the corpus.

gate: `cd rust && cargo test -p kuna-decomp` -> 2405 passed, 0 failed (run fresh on the branch worktree). Full workspace `cargo test --workspace` -> 3234 passed, 0 failed.

## Faithfulness audit (the merge wiring + the marking walk)

- `Merge::merge_opcode` (merge.rs:1194) vs `Merge::mergeOpcode` (merge.cc:326): FAITHFUL. Linear block order (`bblocks_get_block(i)` = `bblocks.getBlock(i)`), op iteration within block, `mergeTestBasic(vn1)` guard, then for each input `mergeTestBasic(vn2)` + `mergeTestRequired(h1,h2)` + `merge(...,false)` required merge. h1/h2 re-fetched per input iteration (matches the C++ `vn1->getHigh()` re-read, safe under intervening merges). Crucially it merges out-with-each-in, never inputs to each other — preserved.
- `Merge::mark_internal_copies` (merge.rs:2091) vs `markInternalCopies` (merge.cc:1444): FAITHFUL transcription of the COPY/PIECE/SUBPIECE cases, the `multi_copy` collection, `hasCopyIn1`/`setCopyIn1`/`setCopyIn2`, the shadowedVarnode no-descend mark, and the big/little-endian piece-offset geometry. (Ported in the merge wave; this diff only invokes it.)
- The `Merge::new(HighIntersectTest::new(PcodeOpSet::new(...)))` construction at the two new sites (cleanup.rs:736, :1015) is IDENTICAL to the pre-existing accepted `ActionMergeRequired` site (:326) — not a novel/ad-hoc construction.
- `ActionMarkImplied::apply` (cleanup.rs:580) vs coreaction.cc:3519: FAITHFUL DFS — `beginLoc..endLoc` seed order, free/explicit/implied skip, depth-first descend walk, classify-on-exhaustion with `count += 1`, `checkImpliedCover` -> setImplied / else setExplicit, push only un-classified descendant outputs.
- `base_explicit` (cleanup.rs:362) vs `baseExplicit` (coreaction.cc:3105): FAITHFUL for the realized arms (no-def/marker/call/CPUI_NEW/-2, addrtied lone-ZEXT, mapped/protoPartial, PIECE-protoPartial, INSERT-STORE, PTRSUB-spacebase maxref lift, desccount>maxref). The addrtied SUBPIECE/PIECE `contains`/`isPartialRoot` refinements take the conservative `return -1` (always-explicit) over-approx — never inlines MORE than the oracle. (Documented, LOSS-155 D2.)

## NO SPECIAL-CASING (grep the diff + the file)

- No function-name/address hardcoding anywhere in the diff or `coreaction_cleanup.rs` (grepped `"boolless"`, `"condconst"`, `"global_cross"`, `"main"`, `EAX/RAX/ACC`, `0x52/0x58`, `kuna_name() ==`, `get_name() ==`, `symbol_name`, `addr == 0x`, `offset ==`). The D1 conditions are all structural: proto-lock state + IR shape (written count, lone CPUI_COPY, source `is_addr_tied`/`is_spacebase`/stack-space-index). CLEAN.

## The inlining is REAL (corpus differential vs the oracle direction)

Differential `print_c` over the whole 83-file datatest corpus, base (fde75bb) vs branch (8473933): the branch removes the redundant trim-COPY `// rax` local across many functions (`v2 = ZEXT(...); v1 = v2; return v1;` -> `v1 = ZEXT(...); return v1;`), matching the C++ oracle's single-named-local return (the condconst datatest's `Conditional Constant #10/#11` `v1 = x;`/`v1 = 0x14;` form). It also collapses the `doublecompare` `v1 = RDX;` trim (the RDX scratch register merges into RAX/v1 via the required COPY merge — standard Ghidra coalescing). Cross-checked it is the genuine merge engine over live IR, not a printer hardcode.

## NO OVER-INLINING — a cover-/multi-def-conflicting varnode STAYS explicit

- `condconst_copy` (x86-64, return register WRITTEN TWICE, MULTIEQUAL-joined): STAYS explicit (`v1 = ZEXT(a0); ... v1 = ZEXT(a0); return v1;`), NOT collapsed to `return ZEXT(a0);`. D1's `written.len() == 1` guard ties it.
- `global_cross` (return value crosses a call `sub_100a00()`): UNCHANGED base->branch, stays `v1 = 0x58; ... return v1;` — its unlocked `xunknown8` output fails D1's `output_locked` guard, so it is never un-tied (the call-crossing case `checkImpliedCover` would also reject). NOT over-inlined.
- boolless `v1` (8051 ACC accumulator, multi-write): STAYS explicit, full byte parity with the B5 oracle.
- Corpus FAIL-negative (absence assertions wrongly matched) is FLAT at 10 base->branch — no spurious over-inlined token introduced anywhere in the corpus. PASS-positive flat at 5 (the new structural inlining is toward the oracle but the few inlined functions are not yet at full string parity for unrelated type/naming reasons — no regression).

## boolless byte-identical

`print_b5_boolless.rs` full byte-parity test passes on the branch; the boolless function is byte-identical between base and branch (not in the corpus diff). Its `v1` (ACC) stays explicit (`v1 = dat_52;` / `return v1;`, never `return dat_52;`).

## C++ oracle untouched

`python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit tests, 675/675 datatest assertions, PARITY OK. (Definitional — the diff is Rust-only.)

## Mandatory hunt list

- Signedness: CLEAN. `written.len() == 1` (usize vs literal), `get_index()` vs `get_index()` (same space-index type). No int4-vs-uint comparisons in the new code.
- Integer widths: CLEAN. No width-bridging arithmetic in the diff.
- Wrapping: CLEAN. No arithmetic on uintb-lineage values in the new code.
- Comparator totality: N/A (no comparators added).
- Iteration-order provenance: `merge_opcode` block order = bblocks index order (= C++ `getBlock(i)`); `mark_internal_copies` op order = `obank().iter_alive()` (= C++ `beginOpAlive`), a pre-existing accepted seam. `iter_loc_size_addr`/`written` collection is order-independent (count + index 0). No HashMap/HashSet introduced.
- Off-by-one / do-while / reverse iterators: N/A.
- Erase-while-iterating: N/A (no list mutation in the diff; the merge engine's surgery is pre-existing).
- Exception -> Result partial-state: `merge_opcode` returns `KunaResult`, the diff swallows it as `let _ = ...` (matches the C++ `apply` which discards `mergeOpcode`'s void return; the engine's internal `?` points are pre-existing and accepted).

## Mechanical pass

- grep diff for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare `as`: NONE.
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: CLEAN. (The `--all-targets -D warnings` run reports lints ONLY in pre-existing unrelated test files — `verify_w6_s4_modelrules`, `printc_parity`, `verify_w3_ir_op`, etc. — never in `coreaction_cleanup.rs` or the new adversarial test.)

## Adversarial tests (land with the verdict, MAIN tree)

`rust/crates/kuna-decomp/tests/verify_w10_implied_vars_adversarial.rs` (4 tests):

- `w10_implied_trim_copy_inlined_on_condconst_conn` — the inlining FIRES on a non-boolless function (FAILS on base fde75bb, PASSES on branch — proves it detects the real win, not a tautology).
- `w10_implied_multiwrite_return_stays_explicit_not_overinlined` — condconst_copy's multi-write return stays explicit, NOT inlined to `return ZEXT(a0);` (the core REJECT guard; passes on both base and branch).
- `w10_implied_boolless_acc_unregressed_byte_parity` — boolless ACC stays explicit + full B5 byte parity.
- `w10_implied_inlining_is_data_driven_no_boolless_leak` — condconst carries no boolless token (`dat_52`/`// acc`) AND the inlining is present on its own IR (FAILS on base, PASSES on branch).

Branch run: 4 passed, 0 failed. Base run (differential): 2 passed (the no-over-inline / byte-parity baselines), 2 FAILED (the two win-detectors), confirming the tests genuinely measure this diff's effect.

## findings

  - F1 (accept-with-loss): the un-tie decision in `mark_output_storage_addr_tied` is a STRUCTURAL stand-in for `ScopeLocal::inScope` (`rangetree.inRange`), not a transcription. It ties MORE than the real query (narrower un-tie), so it never inlines more than the oracle — the safe direction. NO name/address special-casing. -> LOSS-155 D1.
       cpp: decompiler/cpp/funcdata_varnode.cc:993-997, database.hh:599-600
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs (`mark_output_storage_addr_tied`)
  - F2 (accept-with-loss): `check_implied_cover` is a `return true` stub (pre-existing, 82bdfc4) — the LOAD/STORE/call cover and `inflateTest` intersection checks are unported. With the inlining now LIVE this residual is reachable, but the D1 un-tie guard restricts un-tying to single-def lone-COPY transients (no aliasing/cover conflict possible), so over-inlining is bounded and measured clean on the corpus. -> LOSS-155 D2.
       cpp: decompiler/cpp/coreaction.cc:3479-3517
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs (`check_implied_cover`)

## losses
LOSS-155 (this item: the realized inlining + the D1 inScope stand-in + the D2 checkImpliedCover stub). LOSS-137 and LOSS-138 (the pre-existing seams this item builds on) remain accurate.
