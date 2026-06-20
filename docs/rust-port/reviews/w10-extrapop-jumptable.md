# w10-extrapop-jumptable
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-16

gate: `cd rust && cargo test -p kuna-decomp` -> ok (0 failed; lib 1707 passed + all integration bins green); `cargo test --workspace` -> ok (189 result lines, 0 FAILED); `cargo clippy -p kuna-decomp --lib` -> clean; rust-engine datatests (decomp_test_dbg) -> 675 applied / 298 passing; C++ oracle (`kuna.run_tests --all --baseline docs/baseline.json`) -> 207/207 unit + 675/675 datatest, PARITY OK.

## Summary

`ActionExtraPopSetup::apply` stays DEFERRED (`return 0`) — it inserts NO per-call
spacebase op. This is the documented LOSS-199/LOSS-201 partial, NOT a regression:
the merge-base (`10fc2ab`, the prior "defer premature apply" repair) already
returns 0. This branch keeps it deferred, corrects the STALE prior diagnosis
(`propagateSpacebaseRef` was wrongly blamed; it is already ported), and adds
FAITHFUL, INERT supporting machinery for the eventual co-land:

- `FuncCallSpecs::clone_relinked` (truncatedFlow call-spec qlst clone),
- `StackSolver::indirect_callspec_extrapop` (the INDIRECT IOP->call-spec hard
  `rhs=extrapop` equation), and
- the `analyze_extra_pop` INDIRECT-companion `setEffectiveExtraPop` write.

All three are statement-faithful to the C++ and behaviorally inert on the current
suite (the INDIRECTs they consume are only produced by the deferred `apply`).

This is a faithful PARTIAL with precise diagnosis and zero harm -> ACCEPT-WITH-LOSSES.

## Mandate checks (from the work order)

1. FAITHFULNESS — PASS. `ActionExtraPopSetup::apply` is a genuine no-op
   (`return 0`), not a heuristic that strips RSP unconditionally; the full
   faithful loop is preserved as a code comment. The universalAction (B0) SLOT
   is byte-equal to C++ coreaction.cc:5737-5746: Start, Constbase, NormalizeSetup,
   DefaultParams, **ExtraPopSetup("base", stackspace)**, PrototypeTypes, FuncLink,
   FuncLinkOutOnly — and `rust/.../universalaction.rs` was NOT touched by the diff.
   The realized machinery (`clone_relinked`, `indirect_callspec_extrapop`, the
   companion `setEffectiveExtraPop`) matches coreaction.cc:215-235 / 311-326 and
   fspec.cc:4969-4982 / funcdata_op.cc:803-815 line-for-line.
2. NO SPECIAL-CASING — PASS. Greps of the changed production lines for
   register/function-name/address/offset literals find only doc-comments and
   `#[cfg(test)]` constants (`0x1000` test addr). The new code is generic IOP
   decode + qlst lookup; no hardcoding.
3. NEW PASSES ARE REAL — N/A. The branch produces ZERO new datatest passes
   (passing SET byte-identical to base). Nothing to fabricate; the "raw RSP
   eliminated" effect does not fire because `apply` is deferred.
4. NO REGRESSION — PASS (CRITICAL, re-verified with compiled specs). The
   rust-engine datatest passing SET is byte-identical base(10fc2ab) == branch:
   `comm -23 pass_base pass_head` empty AND `comm -13` empty (zero regressions,
   zero new passes; 298==298 passing / 675 applied on both). The switch/jumptable
   files are byte-identical: switchind #1-#7/#9-#14 Success (#8/#15/#16 pre-existing
   fails), switchmulti #1 Success, switchhide/ifswitch/switchloop/switchreturn
   unchanged (`diff` of all switch/jump lines base vs head empty). boolless
   (`Boolean thru Less-than #1/#2`) and condconst identical base vs head.
   cargo test --workspace green; C++ oracle 675/675 PARITY OK untouched (rust-only
   diff — decompiler/specs/kuna all untouched on the branch). B0 order byte-equal.

## Mandatory hunt list

- Signedness: clean. `effective_extrapop`/`extrapop` are `int4` in C++ and Rust;
  `soln - soln2` is signed int4-int4 (matches C++); `comp as usize` guarded by
  `comp >= 0`. EXTRAPOP_UNKNOWN comparison is int4==int4.
- Integer widths: clean. EXTRAPOP_UNKNOWN=0x8000 == fspec.hh:772. `StackEqn.rhs`/
  `soln`/`companion` are int4 matching C++.
- Wrapping: clean. `(soln as uintb) & calc_mask(sz)` sign-extends i32->u64 (verified
  -4 -> 0xffff_ffff_ffff_fffc), matching C++ `soln & calc_mask(sz)` implicit
  int4->uintb promotion. (pre-existing path; the new INDIRECT branch feeds it.)
- Comparator totality: clean. No new comparators (StackEqn::compare pre-existing).
- Iteration-order provenance: clean. qlst clone iterates `0..src_n` via
  `get_call_specs(i)` == C++ `for(i=0;i<qlst.size();++i)` index order. No HashMap/
  HashSet introduced.
- Off-by-one / do-while / reverse iters: clean. `analyze_extra_pop` loop is
  `1..vnlist.len()` == C++ `for(i=1;i<getNumVariables();++i)`. No do-while in the
  diff.
- Erase-while-iterating: clean. qlst clone only pushes; `analyze_extra_pop` reads
  the INDIRECT opcode + writes setEffectiveExtraPop BEFORE the in-place INT_ADD
  opcode rewrite (faithful ordering vs coreaction.cc:311-326).
- Exception -> Result partial-state: clean. `solver.build(...).is_err() -> return 0`
  matches C++ catch returning early (no ops rewritten yet; the omitted warning
  header is cosmetic and documented).

## Mechanical pass

- grep `HashMap`/`HashSet`/`todo!`/`unimplemented!`/`sort_unstable` in changed
  files: none in the new logic.
- bare `as` casts in the diff hunks: only `i as int4` (loop index, == C++
  `eqn.var1 = i`) and `comp as usize` (guarded index) — index-style, not
  value-corrupting.
- `cargo clippy -p kuna-decomp --lib`: clean. `cargo clippy -p kuna-harness
  --tests`: clean.

## One faithful-by-model divergence (not a finding)

`get_call_specs_index` does only the linear `qlst` scan, omitting C++
`getCallSpecs`'s fast IPTR_FSPEC-in(0) path (funcdata.cc:481-493); and the qlst
clone elides the truncatedFlow IPTR_FSPEC-in(0) replacement. Both follow the
established op-indexed W4 fspec-handle model used by every other call-spec lookup
in the port (the cloned annotation round-trips; the relinked spec resolves by
`get_op()`). It predates this branch and is datatest-inert (298/298 unchanged).
Not introduced here, not observable. Tracked under the W4 model, not flagged.

findings: none (faithful PARTIAL; the deferred `apply` is a declared loss, not a
defect).

adversarial tests (committed to the branch, commit 45e4a98):
  - fspec::tests::adv_w10_extrapop_clone_relinked_copies_every_field — pins
    `clone_relinked` mirrors C++ `clone` (relink to newop + every copied field +
    FuncProto portion; active trials skipped).
  - fspec::tests::adv_w10_extrapop_qlst_clone_resolves_by_relinked_op — the
    truncatedFlow qlst-clone round-trips through the op-indexed
    `get_call_specs_index`; an unregistered op resolves to None.
  - verify_w10_extrapop_jumptable_noregress::w10_extrapop_switchind_case_labels_survive
    — structural FENCE: vendored switchind.xml's recovered switch keeps its
    `case N:` / `casefuncN();` labels (the exact 13-assertion passing set); a future
    premature-`apply` re-enable that corrupts the stack-local switch index flips
    these red.
  - verify_w10_extrapop_jumptable_noregress::w10_extrapop_switchmulti_first_assertion_survives
    — second, structurally-distinct switch fence on the multi-stage table path.

losses: LOSS-199 (the original deferred-`apply` partial), LOSS-201 (this round's
refinement — corrected diagnosis + the faithful, inert clone/lookup/companion
machinery).
