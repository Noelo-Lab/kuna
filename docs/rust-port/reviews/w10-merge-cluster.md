# w10-merge-cluster
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Opus 4.8, automated)
date: 2026-06-16
branch: rport/w10-merge-cluster (single commit 67ddb6b on merge-base de9be68)

## Scope

The branch changes EXACTLY ONE file — `rust/crates/kuna-decomp/src/coreaction_cleanup.rs` (+58/-7) — and nothing else (no C++ source, specs/, Makefile, or baseline). The change adds one early-return GATE to the pre-existing kuna W4-ScopeLocal stand-in `mark_output_storage_addr_tied`: a return register whose `marker` writes (CPUI_MULTIEQUAL phi / CPUI_INDIRECT survival) ALL join `persist` (global) values is left UN-tied, so `Merge::mergeMarker` folds it into the persist HighVariable instead of inserting the trim COPY that `mergeTestRequired`'s tied-addr-mismatch arm (merge.cc:111-115) would otherwise force.

NOTE: `mark_output_storage_addr_tied` is NOT a port of any single C++ function — it is an acknowledged W4-ScopeLocal stand-in seam (the merged tree has no register-local symbol recovery yet). This diff is a faithful REFINEMENT of that seam, modeling C++ `ScopeLocal::inScope` (funcdata_varnode.cc:993): the local range tree never contains a global address, so a register merely carrying a global through a join is never restructured into a whole-function local and stays un-tied.

## Gates (all run fresh)

- item gate `cargo test -p kuna-decomp`: ok (1722 lib + all integration, 0 failed).
- `cargo test --workspace` (MAIN tree, with the new adversarial test added): green, 0 failed.
- `cargo clippy -p kuna-decomp --lib`: clean (0 warnings); clippy on the new test file clean.
- C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json`: 207/207 unit + 675/675 datatest assertions, **PARITY OK** (structurally untouched — rust-only diff).
- rust-engine datatests (correct `KUNA_DECOMP_TEST` override, both binaries freshly built): base(de9be68) **312/675** -> branch(67ddb6b) **313/675**, floor 312 met.
- passing-SET diff (sorted `Success --` sets): branch − base = {+`Partial Merge #11`}; base − branch = {} (ZERO regressions). The +1 is exactly the targeted assertion.

## REAL-parity evidence (not fabricated)

- On the BASE binary `Partial Merge #11` = FAIL; on the BRANCH binary = Success; the C++ oracle = Success (consistent with 675/675). `#11` regex = `return glob1\.a \+ 7;`.
- KUNA_DUMP B5: branch renders `return glob1.a + 7;` (the persist global folded straight into the return), where base rendered the register round-trip `v1 = v1 + 7; return v1;`. The new pass is genuine parity on the asserted return line with the C++ oracle.
- The cited byte-identical clusters render BYTE-IDENTICAL branch==C++ oracle B5 (re-verified per function via KUNA_DUMP): `boolless` (`v1 = dat_52; if (dat_52 <= 10) { v1 = 1; } return v1;`), `readstruct` (`return ptr->array[b + a];`), `condconst_conn` (`if ((x == 0) && (y != 10)) { v1 = 0x14; } return v1;`), `condconst_copy` (the persist-writing void). `global_cross` / Store-cross / Global-cross assertions unchanged.

## Mandatory hunt list

- Signedness: clean — the gate does NO arithmetic; `num_input(): int4` is non-negative, `0..nslots` iterates i32 with the `nslots==0` guard, `get_in(i): int4` indexes safely within bounds.
- Integer widths: clean — no `int4*int4`/`uintb` promotion; only slot counts and flag tests.
- Wrapping: clean — no arithmetic op at all in the added code.
- Comparator totality: clean — no comparators added.
- Iteration-order provenance: clean — `marker_writes` derives from `vbank().iter_loc_size_addr` (loc-tree order); the `.all()` over markers and `(0..nslots).all()` are order-independent (boolean AND). No HashMap/HashSet.
- Off-by-one / do-while / reverse iterators: clean — `(0..nslots)` exclusive bound is correct; INDIRECT `nslots=1` (slot 0 only) verified against `newIndirectOp` (slot 1 = iop annotation). No do-while.
- Erase-while-iterating: clean — no mutation during iteration; the gate is a pure read followed by an early `return`.
- Exception -> Result partial-state parity: clean — the early `return` leaves no partial mutation (it skips the addr-tie marking entirely, faithfully leaving the register un-tied, which IS the modeled C++ behavior). Match arms returning `false` are pure reads.

## No special-casing

- ZERO function-name / address / value / type-name literal in the added PRODUCTION code. The `glob1` / `partial_restore` / `0x100670` tokens appear ONLY in doc-comments (grep-verified). The gate keys purely on `is_persist()` / `is_marker()` / `CPUI_INDIRECT` / slot counts.
- INDIRECT slot-1 handling verified faithful (slot 0 = data input; slot 1 = `newVarnodeIop` annotation; indirect-creation slot 0 = `newConstant(0)`, non-persist -> register stays tied — conservative).
- No pipeline / action-order change (universalAction listing unchanged); B0 unchanged. Rust-only diff -> C++ oracle structurally untouched.

## findings

  - F1 (low): the persist fold delivers `Partial Merge #11` but `partial_restore` is NOT byte-identical to the C++ oracle B5 — it still emits `glob1.a = glob1.a;` in both branches (a pre-existing trim/COPY-render seam, present on base; the diff comment implies the gate removes it, but it persists) and the un-normalized `if (10 < a1) {...} else {...}` vs the oracle's `if (a1 <= 10) {...}` swapped form. Only the asserted return line reaches parity. This is the cited LOSS, not a defect of this gate.
       cpp: decompiler/cpp/funcdata_varnode.cc:985-1010 (the real inScope/addrtied the seam stands in for); decompiler/cpp/merge.cc:108-115
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:256-311
  - F2 (info): doc-comment accuracy — the diff comment ("the trim COPY renders the `glob1.a = glob1.a` self-assign") over-claims; the self-assign is not eliminated by this change. Cosmetic, no behavior impact.
       cpp: n/a
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:262-265

## adversarial tests (5 added, committed to MAIN tree)

`rust/crates/kuna-decomp/tests/verify_w10_merge_cluster.rs` — drive the committed Rust `decomp_test_dbg` (engine under test) with `KUNA_DUMP=1`, honoring `KUNA_RUST_DECOMP_TEST` for an out-of-tree branch build. All assertions derived from the C++ oracle B5.

  - `w10_mc_a1_partial_restore_persist_fold_matches_cpp_return` — the persist-passthrough fold renders the oracle `return glob1.a + 7;` (no `v1 + 7` register round-trip, no `+10` constant corruption). Fully exercises the gate against the branch binary; SKIPs cleanly (loud diagnostic, stays green) against a pre-gate binary so MAIN stays green pre-merge.
  - `w10_mc_a2_boolless_constant_join_stays_tied_byte_identical` — SELECTIVITY / over-fire guard: the `acc` register joins a CONSTANT (`#1`), not all-persist, so the gate must NOT fire; boolless byte-matches the oracle. Pins the `ALL`-persist (not `any`) semantics and that constants are not persist.
  - `w10_mc_a3a_readstruct_pointer_return_byte_identical` — a pointer-deref return (no marker write) is byte-identical to the oracle.
  - `w10_mc_a3b_condconst_stack_and_void_persist_untouched` — `condconst_conn` (stack-local return; stack not persist) and `condconst_copy` (persist-WRITING but VOID, no return register) both byte-identical; pins the persist-RETURN-VALUE vs persist-PRESENCE distinction.
  - `w10_mc_a4_persist_fold_keeps_global_store_no_overmerge` — the fold keeps the global STORE `glob1 = a0;` (no over-merge dropping the store) and the `#3` negative (`return glob1.a + 10;` absent) stays satisfied.

All 5 pass against the branch binary (A1 fully exercising the gate, verified non-skip); against the pre-gate MAIN binary A1 SKIPs and A2/A3/A4 pass (byte-identity holds in both states). `cargo test --workspace` green with the file added.

## losses

LOSS-205 (faithful PARTIAL: the gate delivers `Partial Merge #11` `return glob1.a + 7;` with zero regression, but the `partial_restore` body retains the residual `glob1.a = glob1.a` self-assigns and the un-normalized if/else condition, so it is not byte-identical to the C++ oracle B5 — pre-existing seams out of this change's scope).

## Verdict rationale

A faithful PARTIAL with ZERO regression and a REAL new parity pass (#11, matched against the C++ oracle), no special-casing, no pipeline-order change, no fabricated pass, no prior-parity regression. Per `verification.md` this is ACCEPT-WITH-LOSSES, citing LOSS-205.
