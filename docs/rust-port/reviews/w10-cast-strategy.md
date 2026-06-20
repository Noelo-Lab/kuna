# w10-cast-strategy
verdict: ACCEPT
verifier: independent-verifier (Opus 4.8)
date: 2026-06-15
gate: cargo test --workspace -> 3428 passed / 0 failed; cargo clippy -p kuna-decomp --lib -> clean; C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 + 675/675 PARITY OK

## Scope

Single commit `d34a3a0`, two files, `rust/` only:
- `rust/crates/kuna-decomp/src/printc.rs` (+337): wires `INT_SEXT`/`INT_ZEXT`
  through `CastStrategyC` in the C printer.
- `rust/crates/kuna-decomp/tests/verify_w10_implied_vars_adversarial.rs` (±44):
  one pre-existing test's assertions updated from `ZEXT(a0)` to the new `(uint8)a0`
  cast render.

The cast-strategy PREDICATES (`isZextCast`/`isSextCast`/`isExtensionCastImplied`/
`castStandard`/`localExtensionType`) live in `rust/crates/kuna-decomp/src/cast.rs`,
which is UNCHANGED on this branch (ported in a prior wave). This wave is purely
the printc WIRING + a read-only `PrintCastContext` bridge. The faithfulness risk is
therefore the DECISION COMBINATION, the argument order fed to the predicates, the
`option_hide_exts` gate, and the `readOp==0` threading — all verified below.

## Faithfulness (the central focus)

`op_int_zext_ir`/`op_int_sext_ir` (printc.rs:2772/2825) are line-faithful to
`PrintC::opIntZext`/`opIntSext` (printc.cc:806/819):
- predicate order: `isZextCast(out_def_facing, in_read_facing)` first, then
  `option_hide_exts && isExtensionCastImplied(op, readOp)`, else `opTypeCast`,
  else `opFunc`. Matched exactly.
- argument order: `sext_zext_facing_types` returns `(out->getHighTypeDefFacing(),
  in0->getHighTypeReadFacing(op))` and passes `is_*_cast(&outtype, &intype)` —
  same (out, in) order as C++ (cast.cc:443/457). Verified against the
  signedness gate (input-UINT for ZEXT, input-INT for SEXT).
- `op_hidden_func_ir` faithful to `opHiddenFunc` (printc.cc:494):
  `pushOp(&hidden,op); pushVn(op->getIn(0),op,mods)`.
- `op_type_cast_ir` is pre-existing; the `isPointerToArray`/`checkAddressOfCast`
  arm is a documented seam that cannot fire for scalar-integer INT_SEXT/INT_ZEXT
  outputs.

`readOp` threading: only `INT_SEXT`/`INT_ZEXT` consume `readOp` (typeop.hh:421/430
are the only two `push` overrides that forward it; every other override drops it).
The Rust `op_push_ir` mirrors this — the match dispatches the two extension
opcodes to the readOp-aware path and ignores `read_op` everywhere else. The two
live callsites are faithful: printc.rs:2482 (top of statement, `None`, == printc.cc
:2579 `(PcodeOp*)0`) and printc.rs:3017 (implied-var recurse, `Some(op)` where
`op` is the READING op, == printlanguage.cc:539 `defOp->getOpcode()->push(this,
defOp,op)`). The three other C++ callsites (printc.cc:2186/2190 `pushImpliedField`,
printc.cc:2600 `emitBitFieldStore`) have NO Rust equivalent yet (those emit paths
are unported) — so there is nothing to mis-thread; correct for the current scope.

`PrintCastContext` (the read-only `&Funcdata` bridge) faithfully reproduces the
IR accessors the const `isExtensionCastImplied` dereferences. It interns
`VarnodeId`/`OpId` behind `VnRef`/`OpRef` via `RefCell<Vec<_>>` (linear scan,
deterministic, no HashMap), reproducing C++ pointer identity. The two mutator
methods (`vn_set_unsigned_print`/`vn_set_long_print`) are `unreachable!()` — correct,
because `isExtensionCastImplied` is provably read-only (cast.cc:249-301 reads only
IR shape + read-facing metatypes; the mutators belong to markExplicit*, never
reached from this query).

`cast_strategy_for(arch)` rebuilds `CastStrategyC::new(arch.types_rc())` per call;
`types_rc()` clones the Rc of the BOUND factory, and `CastStrategyC::new` sets
`promote_size = get_size_of_int()`, so the rebuild is exact (the strategy is
stateless apart from factory + promoteSize). `PrintC` installs `CastStrategyC`
(printc.cc:137), so building `CastStrategyC` (not `CastStrategyJava`) is correct.

## No special-casing

`git diff ... | grep '^+'` over the production code: NO opcode-value hardcoding
beyond the legitimate `CPUI_INT_SEXT`/`CPUI_INT_ZEXT` dispatch arm (which mirrors
typeop.hh's per-opcode `push` override table); NO type-name string compares, NO
magic offsets/values, NO function-name matching. The only `(uint8)a0`/`ZEXT(a0)`
literals are in the TEST file's assertions. Clean.

## Reserved files

`merge.rs`, `funcdata_merge.rs`, `variable.rs`, `varmap.rs`, and the
merge-action/ActionNameVars code are NOT touched. Only `printc.rs` + one test.

## hunt list
- Signedness: clean. `1 - slot` (int4, slot in {0,1}) cannot underflow; the
  signedness gate (`isZextCast` requires UINT/BOOL input) verified by V1.
- Integer widths: clean. `vn_size`/`promote_size` are int4; `getSize() >
  promoteSize` boundary matched (V3).
- Wrapping: clean. No new wrapping-prone arithmetic in the added code.
- Comparator totality: n/a (no comparator added).
- Iteration-order provenance: clean. Interning Vec is linear-scan deterministic;
  NO HashMap/HashSet in the added code.
- Off-by-one / do-while / reverse iterators: clean. The `> promoteSize` boundary
  is inclusive-at-equal, exclusive-above — pinned by V3.
- Erase-while-iterating: n/a.
- Exception -> Result partial-state: clean. The print path returns `()`; missing
  strategy/types degrade to `op_func_ir` (the pre-cast-routing functional form),
  a faithful fail-open fallback.

## Mechanical pass
- `cargo build --workspace`: green.
- `cargo test --workspace`: 3428 passed, 0 failed.
- Passing SET diff base(`rust-port` 3d655be) vs branch: IDENTICAL (0 regressions,
  0 swapped). The one modified test (`w10_implied_multiwrite_return_stays_explicit
  _not_overinlined`) passes on both; its `(uint8)a0` assertion reflects the REAL
  new engine output (confirmed: `v1 = (uint8)a0;` printed live), a genuine
  move-toward-C++-parity render, not a fabricated pass.
- B5 boolless byte-parity (`print_b5_boolless`): 11 passed, intact.
- `cargo clippy -p kuna-decomp --lib`: clean (the lib-TEST clippy error in
  heritage.rs:3122 is PRE-EXISTING, last touched by `rport/w10-callarg-values`,
  not in this diff).
- C++ oracle: 207/207 + 675/675 PARITY OK, untouched (no C++/spec/baseline edits).
- grep added lines for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`
  /bare `as`: NONE (one comment mentions "HashMap" to explain its absence).

## findings
None blocking. One noted (non-blocking) divergence:
  - N1 (info / pre-existing seam): `sext_zext_facing_types` and
    `PrintCastContext::vn_high_type{_read_facing}` resolve `getHighTypeDefFacing`/
    `getHighTypeReadFacing` through the BARE Varnode type accessors, not the
    HighVariable. This is the established W10 printc convention (the same
    `SEAM(W8 union findResolve)`, used in 4 places in printc.rs incl. the
    pre-existing `op_type_cast_ir`/`op_float_int2float_ir`). It differs from pure
    C++ only for union-resolved varnodes (the W8 deferral, already covered by
    LOSS-050 / LOSS-082); on the integer corpus INT_SEXT/INT_ZEXT operate over it
    is byte-exact. This wave introduces nothing new here — it reuses the existing
    convention — so no new loss entry is required.
       cpp: decompiler/cpp/printc.cc:809 / :822 (getHighType*Facing)
       rust: rust/crates/kuna-decomp/src/printc.rs:2835 (sext_zext_facing_types),
             :3300-3320 (PrintCastContext type accessors)

## adversarial tests
Added `rust/crates/kuna-decomp/tests/verify_w10_cast_strategy_adversarial.rs`
(5 tests, MAIN tree; pass on both base and branch — they pin the predicate
contract the wiring depends on, which is identical because cast.rs is unchanged):
  - w10_cast_strategy_zext_signed_input_stays_functional  (V1: signedness gate;
    ZEXT-of-signed and SEXT-of-unsigned -> functional, not cast)
  - w10_cast_strategy_null_readop_never_hidden            (V2: readOp==None ->
    isExtensionCastImplied false -> explicit cast at statement top, never hidden)
  - w10_cast_strategy_big_constant_boundary_flips_implied (V3: const ==promoteSize
    implied/Hidden, ==promoteSize+1 not-implied/Cast — the `> promoteSize`
    off-by-one)
  - w10_cast_strategy_hide_exts_gate_is_and               (V4: hide_exts OFF turns
    an implied extension from Hidden to Cast, nothing else)
  - w10_cast_strategy_metatype_mismatch_not_implied       (V5: mismatched-metatype
    other operand defeats the implied test -> explicit cast)

## losses
None. (The sole divergence, N1, is the pre-existing W8 union read-facing seam,
already cited by LOSS-050 / LOSS-082; non-output-affecting on this wave's corpus.)
