# w10-structuring
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M) — independent verifier, round 1
date: 2026-06-15
gate: `cd rust && cargo test -p kuna-decomp` -> 3408 passed / 0 failed (workspace); lib clippy clean; C++ oracle 207/207 + 675/675 PARITY OK

## Scope

Diff `rust-port...rport/w10-structuring -- rust/` (279 lines, 2 files): the switch
guard-fold machinery ported faithfully from the C++ but landed DORMANT behind a
keystone gate.

- `funcdata_block.rs`: `Funcdata::push_branch` — port of `Funcdata::pushBranch`
  (funcdata_block.cc:421).
- `jumptable.rs`: `fold_in_guards_impl` (C++ `JumpBasic::foldInGuards`,
  jumptable.cc:1709), `fold_in_one_guard` (C++ `JumpBasic::foldInOneGuard`,
  jumptable.cc:1391), `no_intervening_statement` (C++
  `BlockBasic::noInterveningStatement`, block.cc:2760), and the live
  `JumpBasicModel::fold_in_guards` impl now routing through the gate
  `FOLD_IN_GUARDS_KEYSTONE_READY = false`.

NO keystone-reserved file is touched (merge/funcdata/funcdata_merge/funcdata_union/
varnode/variable/coreaction_cleanup/cast/typeop/dtype/unionresolve/printc all clean).
C++ source, specs, and docs/baseline.json are byte-untouched (pure Rust diff).

## Faithfulness — line-by-line vs C++

- `push_branch` vs pushBranch(funcdata_block.cc:421): exact. `opRemoveInput(cbranch,1)`
  -> `op_remove_input`; `opSetOpcode(...,CPUI_BRANCH)` -> `op_set_opcode_code`;
  `bblocks.moveOutEdge` -> `move_out_edge`; `structureReset` -> `structure_reset`.
  C++ THROWS on the two preconditions (non-CBRANCH/sizeOut!=2; non-BRANCHIND
  target); the port returns `Err` at exactly those points. The empty-block
  `lastOp()==NULL` case (block.cc:2351) becomes a defensive `Err` instead of a
  C++ null-deref — same outcome for every caller-reachable input.
- `fold_in_one_guard` vs foldInOneGuard(jumptable.cc:1391): exact, including the
  `getFlipPath()` path inversion, the `(indpath==0)!=isBooleanFlip()` value
  select, `newConstant(getIn(0)->getSize(),val)`, `addBlockToSwitch(...,NO_LABEL)`
  + `setLastAsDefault` + `pushBranch` (new-default arm) vs `setDefaultBlock(pos)`
  (existing-edge arm), and the trailing `setFoldedDefault` + `guard.clear()`.
  `pos`/`getDefaultBlock` are `int4` on both sides (signedness faithful).
- `fold_in_guards_impl` vs foldInGuards(jumptable.cc:1709): exact — skip
  already-normalized guards (`getBranch()==null`), clear dead-branch guards,
  fold the rest, OR the change flag.
- `no_intervening_statement` vs noInterveningStatement(block.cc:2760): exact
  control flow — isMarker/isBranch continue; special+call|STORE|NEW -> false;
  non-special COPY/SUBPIECE continue; then `getOut()` addr-tied check (false) and
  descend-escapes-block check (false). The only deviation is the documented
  `getOut()==None` -> `return false` guard (C++ derefs unconditionally; every
  reachable op has an output, so behavior-identical).
- `JumpTable::fold_in_guards` delegates to the model (C++ header inline
  jumptable.hh:621 `return jmodel->foldInGuards(fd,this)`); the take/restore of
  `jmodel` is the borrow dance, semantically a no-op.

## The keystone gate (the LOSS)

`fold_in_guards` is in the LIVE pipeline (coreaction_render.rs:1817) but the
ported body is gated OFF by `FOLD_IN_GUARDS_KEYSTONE_READY = false`, so the live
behavior is `Ok(false)` — byte-identical to the pre-diff base. The porter's claim
is that on the current pre-S6-merge IR the enabling gate
`no_intervening_statement(switchbl)` diverges from the C++ oracle and the fold
fires spuriously, growing a bogus `case 0xbad1abe1bad1abe1:` (the
`JumpValues::NO_LABEL` sentinel, jumptable.cc:35) on the folded default and
regressing already-green `case N:` matches.

VERIFIED DIRECTLY: flipping the gate to `true` and re-running the switch corpus
produces exactly `case 0xbad1abe1bad1abe1:` on switchmulti, failing both
verify_w10_jts_chain (switchmulti/rangemeld) and my AT1/AT3 — the porter's
regression claim is accurate, and dormancy is the correct decision. Restored to
`false`; the corpus is clean. This is a faithful PARTIAL with ZERO regression =
ACCEPT-WITH-LOSSES (LOSS-178).

## Hunt list

- Signedness: clean. `indpath`/`pos`/`getDefaultBlock` are `int4` on both sides;
  `1 - indpath` over {0,1}; the only loop index (`for i in 0..selectguards.len()`)
  matches C++ `int4 i < size()` in index order.
- Integer widths: clean. `val` is `uintb` 0/1; `insize` from `getIn(0)->getSize()`.
  No promotion surprises.
- Wrapping: clean. No `uintb` arithmetic in the added logic; `1 - indpath` and
  `pos += 1` are bounded, cannot wrap.
- Comparator totality: n/a (no comparator added).
- Iteration-order provenance: `selectguards` is a Vec (index order = C++ vector);
  `descend_iter` is an insertion-ordered SmallVec mirroring `list<PcodeOp*>` (and
  the descend loop's predicate is order-independent anyway). No HashMap/HashSet.
- Off-by-one / do-while / reverse iterators: clean. The `while pos < nout` linear
  scan matches the C++ `for` exactly; `no_intervening_statement` is a forward
  walk via the intrusive op list (head + next), matching `op.begin()..op.end()`.
- Erase-while-iterating: n/a — `no_intervening_statement` is read-only; the guard
  loop mutates only `selectguards[i]` in place (no erase).
- Exception -> Result partial-state parity: `push_branch` returns `Err` at the
  same points C++ throws, BEFORE any mutation (both checks precede
  opRemoveInput) — partial state identical. The fold path is never live (gated).

## Mechanical pass

- `cargo test -p kuna-decomp` / `cargo test --workspace`: 3408 passed / 0 failed
  (incl. the 7 new verifier tests).
- grep added prod lines: no `todo!`/`unimplemented!`/`panic!`/`HashMap`/`HashSet`/
  `sort_unstable`/`partial_cmp`; no bare numeric `as` cast (the only `as` is the
  `use ... as pf` import alias); no address/case-value/target/function-name
  hardcoding (the `0xbad1abe1`/`switchmulti` strings appear ONLY in comments; the
  live `NO_LABEL` is the named C++ constant `jumptable.cc:35`).
- `cargo clippy -p kuna-decomp --lib`: clean (only pre-existing unrelated
  unused-import warnings).
- C++ oracle (`python -m kuna.run_tests --all --baseline docs/baseline.json`):
  207/207 + 675/675, PARITY OK.

## Findings

- F1 (LOSS, non-blocking): the faithful guard-fold body is DORMANT behind
  `FOLD_IN_GUARDS_KEYSTONE_READY = false` — `JumpBasic::foldInGuards` is a no-op
  in the live pipeline until the S6 merge wave shapes the switch block's op list
  so `no_intervening_statement` matches the C++ oracle. Recorded as LOSS-178.
       cpp: decompiler/cpp/jumptable.cc:1709 (foldInGuards), :1412
         (`!switchbl->noInterveningStatement()` gate), block.cc:2760
       rust: rust/crates/kuna-decomp/src/jumptable.rs:2374
         (FOLD_IN_GUARDS_KEYSTONE_READY), :2620 (live impl), :2410
         (no_intervening_statement)
- F2 (informational, non-finding): `no_intervening_statement` adds a defensive
  `getOut()==None -> return false` where C++ derefs unconditionally; documented in
  the port, behavior-identical for every caller-reachable op.
       cpp: decompiler/cpp/block.cc:2783
       rust: rust/crates/kuna-decomp/src/jumptable.rs:2435-2443

## Adversarial tests (land with the verdict, on the porter branch)

End-to-end dormancy tripwires (rust/crates/kuna-decomp/tests/verify_w10_structuring.rs):
- `w10_structuring_switchmulti_no_spurious_folded_default_gate_dormant` (AT1) —
  FAILS with the gate flipped ON (`case 0xbad1abe1...` appears), PASSES dormant.
- `w10_structuring_switchind_no_spurious_folded_default_gate_dormant` (AT2).
- `w10_structuring_corpus_wide_no_folded_default_leak` (AT3) — FAILS gate-ON,
  PASSES dormant; whole-corpus NO_LABEL tripwire.
- `w10_structuring_render_is_deterministic` (AT4) — order-stability of the gated
  path.

push_branch error-parity unit tests
(rust/crates/kuna-decomp/src/funcdata_block.rs, `#[cfg(test)]`):
- `verify_w10_structuring_push_branch_rejects_non_cbranch_source` (AT5).
- `verify_w10_structuring_push_branch_rejects_non_branchind_target` (AT6).
- `verify_w10_structuring_push_branch_rejects_cbranch_wrong_arity` (AT7).

The gate-ON flip was verified to reproduce the porter's exact regression
(`case 0xbad1abe1bad1abe1:`) and then reverted; the committed state is `false`.

## losses

- LOSS-178 (this verdict): the guard-fold machinery is a faithful transcription
  landed DORMANT behind `FOLD_IN_GUARDS_KEYSTONE_READY`; zero live behavior change,
  zero regression. Restoration: flip the const true once the S6 merge wave shapes
  the switch block's op list to match the C++ oracle, then re-verify against B5.
