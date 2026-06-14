# w10-stacklocal-typing
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-14
gate: cargo test --workspace -> 3245 passed / 0 failed; cargo clippy --workspace (lib) -> clean; C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK; corpus stringmatch tally (rust engine) -> 5 pos / 32 neg PASS, 628/10 FAIL, BYTE-IDENTICAL to rust-port base.

## Summary

This branch is the re-submission of the previously-REJECTED `ScopeLocal::restructureVarnode`
tail (LOSS-145). It ports four faithful pieces and wires them into the live pipeline:

1. `ScopeLocal::applyTypeRecommendations` (varmap.cc:1574) + `addTypeRecommendation`/
   `hasTypeRecommendations` — driven from `run_infer_types` head (coreaction.cc:5654).
2. `ScopeLocal::annotateRawStackPtr` (varmap.cc:386) + the `restructureVarnode` tail
   `sortAlias` / `alias[0]==0` placeholder gate (varmap.cc:1279-1285).
3. `ActionInferTypes::propagateRef`/`propagateSpacebaseRef` (coreaction.cc:5464/5521) +
   `TypeSpacebase::getAddress` (type.cc:3522) — wired into `run_infer_types` tail.
4. The `TypeOpCall::getInputLocal` / `TypeOpCallind::getInputLocal` callee-param arm
   (typeop.cc:689/740) — flows a typed call argument onto the argument Varnode.

All four are line-by-line faithful transcriptions, verified against the C++ source.

The typed-stack chain is **DORMANT by default**: the `resetLocalWindow` enabler is gated
behind `KUNA_RESET_LOCAL_WINDOW`. I verified the gating is honest, the chain is REAL when
enabled (it builds a typed stack array from recovered RangeHints, not a hardcoded shortcut),
and the default output is byte-identical to the rust-port base (no fabricated parity, no
regression). Accepted under LOSS-156.

## Faithfulness verification (against the C++ oracle source)

- `applyTypeRecommendations`: EXACT — `findVarnodeInput(dt->getSize(), addr)` then
  `updateType(dt,true,false)`. The Rust `update_type_locked` matches varnode.cc:493-508
  (UNKNOWN-always-unlocked, the two early-return guards, the typelock flag flip); the
  C++ in-method `high->typeDirty()` is fulfilled externally by the wrapper (faithful split).
- `annotateRawStackPtr`: EXACT — the `hasTypeRecoveryStarted` guard, the `findSpacebaseInput`
  bail, the `special && !isCall` + INT_ADD/PTRSUB/PTRADD reader filter, and
  `newOpBefore(op,PTRSUB,spVn,newConstant(spVn->getSize(),0))` + `opSetInput(op,out,slot)`.
- `restructureVarnode` tail: the ported subset (`sortAlias`, `alias[0]==0 -> annotateRawStackPtr`)
  is in C++ order; the skipped steps are a recorded residual (LOSS-156/D3, the open half of LOSS-145).
- `propagateRef`: the PTR/SPACEBASE/UNKNOWN early returns, the `[beginLoc(addr),endLoc(endaddr))`
  window (incl. the wrap-to-space-end arm), the annotation/no-descend/typelock skips, the
  `curoff+cursize > ct->getSize()` guard, `getExactPiece`, and the `typeOrder<0 -> setTempType +
  propagateOneType` adopt — all faithful. The snapshot-then-iterate is equivalent (propagateOneType
  only mutates temptypes, never the loc set structurally — verified).
- `propagateSpacebaseRef`: the absolute-type PTR-to-SPACEBASE gate and the COPY/INT_ADD/PTRSUB/
  PTRADD switch (incl. the PTRADD `off = in1->offset * in2->offset`) are faithful.
- `TypeSpacebase::getAddress`: EXACT — the `localframe.isInvalid() -> sz=-1` global-spacebase arm
  and `resolveConstant`. (Rust folds a resolve error to None, which callers skip — conservative.)
- `call_input_type_local`: faithful to the CALL arm (fspec gate, `getParam(slot-1)`, type-locked
  `!=VOID && size<=argsize`, this-pointer PTR-to-STRUCT). One INFIDELITY on the CALLIND arm — see F1.

## The mandatory hunt list

- Signedness: CLEAN. The `curoff = curvn->getOffset() - off` (uintb) and the `curoff+cursize >
  ct->getSize()` guard use `wrapping_sub`/`wrapping_add`; the iter window guarantees
  `curoff_abs >= off`, so no wrap occurs. The CALL/CALLIND size compare is `int4 <= int4`.
- Integer widths: CLEAN. `off: uint8`, `sz: int4` in `spacebase_get_address` match C++ `uintb`/`int4`.
  The PTRADD `voff.wrapping_mul(in2off)` matches C++ `vn->getOffset() * op->getIn(2)->getOffset()` (uintb*uintb).
- Wrapping: CLEAN. `Address + i64` wraps via `wrap_offset` (== C++ `operator+(int8)`); the wrap arm
  of `iter_loc_addr_range` matches `endaddr.getOffset() < off -> endLoc(space)`.
- Comparator totality: N/A (no new comparator).
- Iteration-order provenance: CLEAN. `iter_loc_addr_range` filters the ordered `loc_tree`
  (BTreeMap == C++ VarnodeLocSet order). The `descend_iter()` snapshots (== beginDescend list order).
  The `num_calls()` 0..n scan in `call_input_type_local` is a keyed lookup (unique op match), order-independent.
  No HashMap/HashSet in the added logic.
- Off-by-one / boundaries: CLEAN. `[start,end)` is half-open (verified by an adversarial test:
  `[0x100,0x110)` excludes 0x110). `annotateRawStackPtr` splices BEFORE the reader (newOpBefore).
- Erase-while-iterating: CLEAN. Every C++ live-iterator loop is collected to a Vec first; the
  in-loop mutations (`set_temp_type`, op splice) do not structurally alter the snapshot's set.
- Exception -> Result: CLEAN. `getExactPiece`/`resolveConstant` errors fold to None and are skipped
  (conservative); no partial-state divergence (all bails precede mutation).
- bare `as` casts: each carries a `// cast:` justification (ct_size as i64, cursize/ct_size as uintb,
  curoff as int4 — all bounded by the preceding guards).
- todo!/unimplemented!/HashMap/HashSet/sort_unstable: NONE in the added production code.

## NO-SPECIAL-CASING audit

Grepped the diff for function-name / address / type-name / stack-offset hardcoding and
string-literal/equality discriminators in code paths: CLEAN. The only `mystruct`/`condconst`/
`boolless` tokens are in doc comments as illustrative examples. The only string literal in a
code path is the generic feature gate `KUNA_RESET_LOCAL_WINDOW` (not a per-function selector).
The enabled-path typed array (`xunknown1 [4]`, `$$undef00000002[0] = a0`) is built by the
pre-existing `create_entry` from recovered RangeHints (`concretize`+`getTypeArray`) — proven
data-driven, no hardcoded type/offset.

## The "typing is REAL" proof

With `KUNA_RESET_LOCAL_WINDOW` OFF (default), condconst renders the scalar `v1 = ZEXT(a0); return v1;`.
With it ON, the same function renders a real typed stack array `xunknown1 $$undef00000002 [4]`
with `$$undef00000002[0] = a0` member access — confirming the chain fires end-to-end (restructure
gathers hints -> create_entry builds the typed symbol -> the access renders), is NOT dead code, and
is NOT a hardcoded shortcut. Enabling it regresses `condconst.xml` "Conditional Constant #10" (a
faithful degraded-pipeline consequence: no sized stack Varnode on restructure pass 1) — which is
precisely why the porter gates it. Honest and accurate.

## No-regression / no-fabrication proof

The corpus stringmatch tally (`decompile_func` + `print_c` over all 83 datatests, 675 assertions)
is BYTE-IDENTICAL between rport/w10-stacklocal-typing and the rust-port base: 5 positive PASS /
32 negative PASS / 628 positive FAIL / 10 negative FAIL on both. The execution-failure count does
not regress. boolless (print_b5_boolless, 11 tests) and readstruct/struct-corpus (16 tests) parity
suites remain green. The new live wiring (`apply_type_recommendations` no-op on recommendation-free
functions; `annotate_raw_stack_ptr`; `propagate_spacebase_ref`) produces no observable change on the
default pipeline.

## findings

  - F1 (low, infidelity — conservative+latent): `call_input_type_local` applies the
       `ct->getSize() <= op->getIn(slot)->getSize()` guard to the CALLIND type-locked arm, but
       C++ `TypeOpCallind::getInputLocal` checks ONLY `ct->getMetatype() != TYPE_VOID` (no size
       guard). The Rust is STRICTER for CALLIND — it would reject a type-locked param wider than
       the arg Varnode where C++ accepts it. Conservative (never types where C++ would not) and
       latent (CALLIND callee-param typing needs the LOSS-153 recovered-callee-proto path; not on
       the corpus). Recorded as LOSS-156/D2.
       cpp: decompiler/cpp/typeop.cc:766 (CALLIND, no size guard) vs typeop.cc:709 (CALL, size guard)
       rust: rust/crates/kuna-decomp/src/coreaction_infertypes.rs (call_input_type_local, unified arm)
  - F2 (low, conservative proxy): `propagateRef` skips a Varnode on `is_mapped()` where C++ skips
       on `getSymbolEntry() != 0`. Over-skips only when a Varnode has a SymbolEntry but is not
       `mapped`; conservative (can only MISS a propagation, never fabricate) and unreachable today
       (chain dormant). Recorded as LOSS-156/D1.
       cpp: decompiler/cpp/coreaction.cc:5490
       rust: rust/crates/kuna-decomp/src/coreaction_infertypes.rs:1298 (`if mapped { continue; }`)
  - F3 (informational, gated dormant enabler): the `resetLocalWindow` enabler — the load-bearing
       step that makes the whole typed-stack chain fire — is held behind `KUNA_RESET_LOCAL_WINDOW`
       to avoid a real `condconst.xml` #10 regression on the degraded pipeline. Faithful C++
       behavior held inert. Recorded as LOSS-156/G.
       cpp: decompiler/cpp/varmap.cc:432 (resetLocalWindow)
       rust: rust/crates/kuna-decomp/src/coreaction_protos.rs:182-184
  - F4 (informational, deferral): the restructure tail ports only `sortAlias`+`annotateRawStackPtr`;
       `clearUnlockedCategory`/`fakeInputSymbols`/`markUnaliased`/`checkUnaliasedReturn` remain
       seamed (the open half of LOSS-145). Does not affect the ported `alias[0]==0` placeholder
       gate. Recorded as LOSS-156/D3.
       cpp: decompiler/cpp/varmap.cc:1259/1275-1283
       rust: rust/crates/kuna-decomp/src/funcdata_spacebase.rs:280-297

## adversarial tests (3 added by the verifier, committed on the branch e78bb30)

In rust/crates/kuna-decomp/src/funcdata_spacebase/spacebase_tests.rs:
  - w10_stacklocal_typing_annotate_raw_stack_ptr_noop_before_type_recovery
       (pins the `if (!hasTypeRecoveryStarted) return;` guard — no PTRSUB spliced before recovery)
  - w10_stacklocal_typing_apply_type_recommendations_size_mismatch_no_lock
       (pins `findVarnodeInput(dt->getSize(), addr)` size discrimination — a 4-byte rec does not
        lock the 8-byte input)
  - w10_stacklocal_typing_apply_type_recommendations_unknown_stays_unlocked
       (pins the `updateType` UNKNOWN-always-unlocked rule on the recommendation path)
All 3 pass. They target the conservative arms the porter's positive tests do not exercise; none
can fabricate a type or a placeholder when the preconditions fail.

Plus the dormancy/realness measurement reproduced directly (condconst body byte-identical with the
env var off; regresses #10 with it on) — documented in LOSS-156.

losses: LOSS-156 (this item); residual of LOSS-145 (D3), LOSS-153 (D2 latency), LOSS-006/090 (G enabler deps).
