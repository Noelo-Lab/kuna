# w10-spacebase-typing
verdict: ACCEPT
verifier: claude-opus-4-8[1m] (independent verifier)
date: 2026-06-16
round: 1

gate:
- `cargo build --workspace` -> green; `cargo test --workspace` -> **3546 passed / 0 failed** (3542 baseline + 4 verifier tests).
- `cargo clippy -p kuna-decomp --lib` -> clean (forced recompile, 0 warnings).
- Rust datatest oracle (`decomp_test_dbg -usesleighenv datatests`) -> **Total tests applied = 675, Total passing tests = 298**.
- C++ oracle untouched: the branch touches ONLY `rust/` (`git diff rust-port...HEAD -- ':!rust/'` is empty), so 207/207 unit + 675/675 datatest PARITY OK is byte-for-byte unchanged.
- B0 (`universalaction_listing` `decompile_tree_dump_is_byte_equal_to_oracle`) -> PASS, byte-equal to oracle. No action-registration file is in the diff.

## TL;DR
A **faithful, non-special-cased** transcription that closes the keystone the W10 RSP plane needed: `TypeSpacebase::getSubType` / `nearestArrayedComponentForward` / `nearestArrayedComponentBackward` realized against the function's `ScopeLocal` symbol table, plus the `TypePointer::isPtrsubMatching` `TYPE_SPACEBASE` arm wired through it (`Funcdata::is_ptrsub_matching_scope`). This is exactly the C++ shape: `TypeSpacebase` carries the Scope, a pure `Datatype` method cannot reach it, so the spacebase arms are dispatched to the scope-aware helpers. The typing that triggers this comes from the cspec stackpointer decl via the pre-existing (unchanged) `Funcdata::spacebase()` (`spc.num_spacebase()`/`spc.get_spacebase(i)` + `getTypeSpacebase`, typed only on `is_input`), **not** any register-name match. Behavioral effect is verified end-to-end: `castFixupPtrsub` now keeps a valid `PTRSUB(sp, off)` instead of tearing it into raw `RSP + off` — the raw-RSP additive form is genuinely eliminated. Zero datatest regressions (the 298-member passing SET is byte-identical to the `rust-port` baseline, switch files included). No new losses are introduced by this item.

## RSP-cleanup proof (re-run, dirty-RSP fn vs baseline)
`noforloop_alias` rendered, parent `rust-port` (10fc2ab) vs this branch (7b145f8):
- baseline:  `undefined8 v1; // rsp` and `sub_40067b((int8)v1 + 0xffffffffffffffe8);` — raw RSP-relative additive form.
- this item: `sub_40067b((xunknown1 *)PTRSUB(v1,0xffffffffffffffe8));` and `v2 = PTRSUB(v1,...)` — the recognized spacebase `PTRSUB(sp, off)`; raw `(int8)v1 + const` gone.
Mechanism (verified): pre-item, `coreaction_casts.rs:1158` called the pure `Datatype::is_ptrsub_matching`, whose `TYPE_SPACEBASE` arm (`dtype.rs`) calls `ptrto.get_sub_type()` on the `TypeSpacebase` datatype — which cannot reach the scope, returns `None`, so `castFixupPtrsub` saw `false` and undid the PTRSUB into INT_ADD. The new `is_ptrsub_matching_scope` resolves `getSubType` through `ScopeLocal`, returns `true`, the PTRSUB survives.

## Faithfulness (verified line-by-line vs the C++ in scope)
- `ScopeLocal::spacebase_get_sub_type` vs `TypeSpacebase::getSubType` (type.cc:3411-3433): `getMap()` -> this ScopeLocal (the localframe Funcdata's local scope) is the correct reduction; `glb->resolveConstant(spaceid, byteToAddress(off), -1, ...)` reduces to `Address(spc, wrapOffset(addressToByte(byteToAddress(off))))` for the stack space because the stack space has **no** AddressResolver (only segmented spaces get one via `Architecture::initializeSegments` -> `insertResolver`; verified). `queryContainer(addr,1,null)` -> `find_container(self.scope,...)` is faithful for stack addresses: `mapScope(this,...)` returns `this` (no namespace scope claims the stack space), and `stackContainer` from ScopeLocal up through the global parent finds nothing in the global scope for a stack address, so it == `ScopeLocal::findContainer`; the `inScope` early-return (database.cc:962) does NOT set `*addrmatch`, so the null/`None` cases coincide. `*newoff = (addr-smallest.addr)+smallest.offset` -> `wrapping_sub(...) as int8 + entry_off as int8` (uintb wrap matches C++; verified mid-symbol = 0x10 by AT1). No-symbol fallback returns NON-null `getBase(1,TYPE_UNKNOWN)`, `newoff=0` — faithful (this is why `hasMatchingSubType` always succeeds off a spacebase ptr).
- `spacebase_nearest_arrayed_forward` vs `nearestArrayedComponentForward` (type.cc:3435-3480): `smallest==0 || getOffset()!=0 -> nextAddr=addr+32`; in-symbol-base path defers to the symbol type's forward probe, returns `(distance, structOff, elSize)` with the `distance>max -> -1` clamp; the `nextAddr < addr` wrap guard (offset compare, faithful for same-space); the next-slot probe `distance - newoff` with the clamp. Return-tuple order is `(distance, newoff, elSize)`, matching the C++ return-value-vs-out-param split (verified against `Datatype::nearest_arrayed_component_forward` which also returns `(distance, new_off, el_size)`).
- `spacebase_nearest_arrayed_backward` vs type.cc:3482-3496: `getSubType` then defer to the sub-type's backward probe with the `distance>max -> -1` clamp. Faithful.
- `Funcdata::is_ptrsub_matching_scope` vs `TypePointer::isPtrsubMatching` (type.cc:1260-1312): ONLY the `TYPE_PTR -> TYPE_SPACEBASE` arm is special-cased to the scope; everything else delegates to the pure `is_ptrsub_matching` (which is itself faithful for ARRAY/STRUCT/UNION/default). The SPACEBASE arm transcribes lines 1265-1273 exactly: `addressToByteInt(off, wordsize)` (the pointer's wordsize, distinct from the space wordsize used inside getSubType — verified separation), `getSubType`, `if (subType==0 || newoff!=0) return false` (subType never null here, matching the C++ UNKNOWN fallback), `addressToByteInt(extra, wordsize)`, the `extra<0 || extra>=getSize()` -> `testForArraySlack` slack check, `return true`. `multiplier` is correctly unused in the SPACEBASE arm (as in C++).
- `addtreestate.rs::base_get_sub_type`/`base_nearest_backward`/`base_nearest_forward` + `has_matching_sub_type` vs `AddTreeState::hasMatchingSubType` (ruleaction.cc:6086-6130): the spacebase metatype is dispatched to the scope-aware helpers, non-spacebase to the pure `Datatype` methods; the `arrayHint==0`/`typeBefore<0 && typeAfter<0`/`offAfter==offBefore`/`elSize` arbitration/`distBefore`/`distAfter` tie-break are all transcribed verbatim. The `(None, off)` returns on the non-spacebase path are benign (caller discards the offset when the type is None/false).
- `Datatype::test_for_array_slack` visibility widened to `pub(crate)` (dtype.rs:2941) — body unchanged; needed so `is_ptrsub_matching_scope` can call the same C++ `TypePointer::testForArraySlack` (type.cc:1131). Faithful.
- `propagateSpacebaseRef`/`propagateRef` (the downstream consumers named in the brief) are UNCHANGED on this branch (`git diff rust-port...HEAD -- coreaction_infertypes.rs` empty) — this item only enables them by making the spacebase type resolvable.

## Hunt list
- **Signedness**: clean. The new comparisons (`extra_b >= sub_type.get_size() as i64`, `distance > max`, `next_addr.get_offset() < addr.get_offset()`) are all signed-int8 vs int8 or uintb vs uintb; `get_size()` (int4) is widened with `as i64` exactly where C++ promotes int4 to int8. No int4-loop-index-vs-size cases introduced.
- **Integer widths**: clean. `addressToByte`/`byteToAddress`/`addressToByteInt`/`byteToAddressInt` map verbatim (`*`/`/`); for the stack space `ws==1` so they are identity. The two wordsizes (pointer wordsize in `is_ptrsub_matching_scope`, space wordsize inside `getSubType`) are kept distinct, matching C++.
- **Wrapping**: clean. The uintb offset subtractions (`addr-smallest.addr`) use `wrapping_sub` matching C++ unsigned wrap; `address_to_byte` uses `wmul`; `&addr + 32` / `entry.get_addr() + sz` route through the wrap-applying `Address` `Add` impl (matches C++ `operator+`). The `distance - newoff` (type.cc:3474) uses plain signed `-` on small bounded values (≤max=128) — matches C++ signed subtraction; no realizable overflow.
- **Comparator totality**: N/A — no comparator/`operator<` added.
- **Iteration-order provenance**: clean. `find_container` rides kuna-base `rangemap` (ordered, mirrors C++ `rangemap<SymbolEntry>`), traversed reverse via `find_subsorts`+`next_back` to pick the smallest — the pre-existing, unchanged `findContainer` port. No `HashMap`/`HashSet` in the diff (grep: 0 hits).
- **Off-by-one / do-while / reverse iterators**: clean — no do-while; the next-slot single probe matches the C++ single `queryContainer(nextAddr,...)`.
- **Erase-while-iterating**: N/A — these methods are read-only against the symbol table.
- **Exception -> Result partial-state parity**: clean. `getSubType`'s `?` on `get_base` and the `Err(_) => return false`/`(-1,off,0)` arms leave no partial mutation (read-only); C++ has no throw site here.

## Mechanical pass
- `port-audit` is not present in this workspace (no `port-audit` package) — blob-sha drift check N/A.
- greps over the added source: `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` = **0**. Register/function-name/hardcoded-address special-casing = **0** (typing keyed on `get_metatype()==TYPE_SPACEBASE`, a cspec-seeded structural property). Bare `as` casts are all numeric width conversions transcribing C++ implicit int conversions (no `// cast:` comments — minor style nit, not a finding).
- `cargo clippy -p kuna-decomp --lib -- ` clean.

## No-regression (CRITICAL)
- Whole datatest suite: **298 passing** (>= 298 target met). The passing SET is **byte-identical** to the `rust-port` baseline run on the same oracle: `comm -23 base_pass branch_pass` and `comm -13` are BOTH empty — zero regressions, zero gains.
- Switch/jumptable files specifically: `switchind.xml` (Switch Indirect #1-7, #9-14 Success / #8,#15,#16 FAIL), `switchloop.xml`, `switchmulti.xml` — all members of the passing set are unchanged (covered by the set-equality above and pinned by the new switch guard).
- `boolless` (`print_b5_boolless`), `readstruct`/`condconst_conn` golden tests (`verify_w10_struct_corpus` 19/19, `verify_w10_input_params` 4/4, `verify_w10_merge_facing` 3/3) — all PASS.
- `cargo test --workspace` green; C++ oracle 675/675 PARITY OK untouched (no C++ in diff); B0 byte-equal.

## Findings
None. (The `RulePtrsubUndo::apply_op` SEAM(W6) stub — which returns 0 / never undoes a PTRSUB and so never calls the new helper — is a PRE-EXISTING W6 loss, not introduced or affected by this item. It is conservatively safe for the spacebase goal: it cannot wrongly destroy a valid stack `PTRSUB(sp,off)`. The helper's doc-comment naming `RulePtrsubUndo::applyOp` as a future caller is accurate intent, not a claim that it is wired this round. No new loss is created.)

## Adversarial tests (committed to the branch with this verdict, commit af4c36a)
- `varmap::tests::w10_spacebase_typing_get_sub_type_midsymbol_newoff_is_byte_offset_into_symbol` — the `(addr-base)+smallest.offset` newoff at a NON-base offset (= 0x10, not 0/absolute), the only place the C++ subtraction is non-trivial.
- `varmap::tests::w10_spacebase_typing_nearest_forward_no_symbol_returns_negative` — the `(-1, off, 0)` no-array fallthrough on an unmapped offset with empty next slot.
- `varmap::tests::w10_spacebase_typing_nearest_backward_resolves_array_distance` — the getSubType->backward delegation into a mapped `int4[16]`.
- `verify_w10_spacebase_typing_switch_guard::w10_spacebase_typing_switchind_passing_set_unregressed` — runs the real `switchind.xml` and pins the EXACT 13 baseline-passing assertions + no whole-file EXEC-FAILURE. Negative control verified (adding the known-FAIL #8 makes the guard fail as expected).

## losses
None cited (clean ACCEPT — no new divergence introduced by this item).
