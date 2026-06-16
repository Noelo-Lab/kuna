# w10-revisit-ssa
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Opus 4.8)
date: 2026-06-16
gate: `KUNA_ENGINE=rust KUNA_RUST_PROFILE=debug python3 -m kuna.run_tests --datatests` (branch worktree /tmp/w10-revisit-ssa) -> 315/675 assertions passed (== base 30cf0b5; passing SET byte-identical, 0 gained 0 regressed)

## Scope

Single porting commit `d7d510f`: ports `Heritage::removeRevisitedMarkers`
(heritage.cc:245-298) into `remove_revisited_markers` and rewires the
`placeMultiequals` call site, replacing a prior `unimplemented_seam(...)` panic.
Diff is ONE file, Rust-only: `rust/crates/kuna-decomp/src/heritage.rs`
(+167/-7). No C++/specs/Makefile/baseline touched.

## Result summary

- FAITHFUL line-by-line transcription of heritage.cc:245-298 (the three opcode
  arms + the shared SUBPIECE-rewrite tail), verified statement-by-statement.
- NO special-casing: zero function-name/symbol-name/address/magic-value literals
  in added production code (greps clean; the `revisit.xml` token appears only in
  a doc-comment). The `4`/`1`/`0` literals all match C++ structure.
- ZERO new passes / ZERO regression. Instrumented (`eprintln` hit count) the
  function across the FULL datatest corpus: **0 hits** — the code is currently
  unreached because the Rust heritage multi-pass machinery does not yet populate
  `removevars`. rust-engine datatests 315/675 on base==branch, passing SET
  byte-identical (`set==set`, 0 gained 0 lost).
- `revisit.xml`: 2/9 on BOTH base and branch (C++ oracle 9/9 — the gap is the
  unported upstream revisit-trigger machinery, NOT this function).
- C++ oracle: 207/207 unit + 675/675 datatest PARITY OK, byte-untouched
  (Rust-only diff).
- boolless/readstruct/condconst_conn byte-parity tests PASS (print_b5_boolless
  11/11 + 1 ignored, struct corpus 16/16, const_prop_phi 6/6). B0/B5 unchanged.
- cargo test --workspace green (0 failed). cargo test -p kuna-decomp --lib 1729
  passed (incl. the 4 new adversarial tests). clippy -p kuna-decomp --lib clean.

This is a faithful PARTIAL with zero regression (a correctly-inert dormant
landing), which the verification protocol classifies as ACCEPT-WITH-LOSSES.

## Hunt list

- Signedness: CLEAN. C++ `for(int4 i=0;i<remove.size();++i)` -> `for &vn in
  remove` (no signed-index-vs-size compare). Arm dispatch is enum equality.
- Integer widths: CLEAN. The one bare cast `offset as uintb` (heritage.rs:1884)
  widens a non-negative int4 byte-overlap into `newConstant(4, uintb)`, matching
  C++'s implicit `int4 offset -> uintb val` conversion; it CARRIES the
  protocol-required `// cast:` comment at :1882-1883. `newConstant(4,..)`,
  `getIn(1)`, `deadremoved>0` literals all match C++.
- Wrapping: CLEAN. No arithmetic on uintb-lineage values (only the widening).
- Comparator totality: N/A — none ported.
- Iteration order: CLEAN. `remove` iterated in vector/insertion order (C++
  vector index order; `collect` pushes in `beginLoc` order); the MULTIEQUAL
  skip-loop walks the intrusive block list forward via `basic_neighbours().1`
  (== C++ `++pos`). No HashMap/HashSet.
- Off-by-one / do-while / reverse iters: CLEAN. The `while(pos!=endOp &&
  (*pos)==MULTIEQUAL) ++pos` is a while-not-do-while (zero-or-more), correctly
  ported as `while let Some(p)=after { if !multi break; ... }` after one initial
  `++pos`.
- Erase-while-iterating: CLEAN. Iterates a `std::mem::take`'d Vec while mutating
  fd's op graph (C++ iterates a `remove` copy too); the COPY-unlink and SUBPIECE
  re-insert touch only fd, never `remove`. One LATENT C++-faithful fragility
  noted (F2 below).
- Exception->Result partial-state: CLEAN. `op_set_all_input`'s Result
  `.expect`ed; stale-handle `.expect`s panic where C++ derefs invalid pointers
  (UB) — partial-state parity preserved.

## Mechanical pass

- todo!/unimplemented!/HashMap/HashSet/sort_unstable/unwrap/partial_cmp in added
  lines: NONE. The prior `unimplemented_seam` call was removed.
- bare `as` casts in added lines: ONE (`offset as uintb`), commented per
  protocol.
- special-casing greps (fn names / addresses / magic values): CLEAN.
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean. (clippy `--tests`
  shows 19 pre-existing errors in OTHER, unrelated test files — zero in
  heritage.rs; not introduced by this item.)

## Findings

  - F1 (informational): the ported `remove_revisited_markers` is DORMANT on the
    current corpus (0 instrumented hits across all 83 datatests) — it delivers 0
    new passes. Faithful but its value is deferred until the Rust heritage
    multi-pass revisit-trigger machinery populates `removevars`. -> LOSS-207.
       cpp: decompiler/cpp/heritage.cc:245
       rust: rust/crates/kuna-decomp/src/heritage.rs:1750
  - F2 (low, C++-faithful latent): the degenerate case where an INDIRECT marker
    immediately follows its OWN target op makes `++pos` land on the op being
    removed — a C++ `list::iterator` invalidation UB the Rust reproduces as a
    self-insert / infinite loop in `bb_ops`. Not corpus-reachable; surfaced only
    by a synthetic adversarial test (constructed with a spacer op to avoid it).
    Not a port-introduced divergence. -> LOSS-207 (residual loss 3).
       cpp: decompiler/cpp/heritage.cc:270-273
       rust: rust/crates/kuna-decomp/src/heritage.rs:1812-1828
  - F3 (cosmetic): the W8 `warningHeader("Heritage AFTER dead removal...")` text
    is suppressed (block comment; the dead-code-delay bump still runs, identical
    to the existing `heritage()` deadremoved-warning handling). -> LOSS-207
    (residual loss 2).
       cpp: decompiler/cpp/heritage.cc:253-257
       rust: rust/crates/kuna-decomp/src/heritage.rs:1764-1771

## Adversarial tests

Committed to the branch (commit `942da5c`), in
`rust/crates/kuna-decomp/src/heritage.rs` (tests module), all passing:
  - `w10_revisit_remove_markers_copy_is_unlinked` — return-form COPY marker is
    opUnlink'd (not rewritten to SUBPIECE; the COPY arm `continue`s).
  - `w10_revisit_remove_markers_multiequal_rewritten_after_phis` — MULTIEQUAL
    marker rewritten to SUBPIECE of a new full-size free Varnode, inserted AFTER
    the trailing run of MULTIEQUALs (the skip-trailing-phis loop), output
    write-masked, offset operand == byte overlap.
  - `w10_revisit_remove_markers_indirect_inserts_after_live_target` — INDIRECT
    marker rewritten to SUBPIECE inserted immediately after the live target op
    (decoded via op_iop_decode), clearAddrForce on the original output.
  - `w10_revisit_remove_markers_offset_widening_le` — LE high-half slice has
    overlap offset 2; exercises the int4->uintb widening at the SUBPIECE in1
    const (the one bare `as` cast).

## Losses

LOSS-207 (the dormant landing + the suppressed W8 warning + the latent
self-insert fragility). ACCEPT-WITH-LOSSES.
