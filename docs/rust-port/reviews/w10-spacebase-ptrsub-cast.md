# w10-spacebase-ptrsub-cast
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-17 (round 1)
gate: cd /tmp/kuna-sbcast/rust && cargo test --workspace (debug, canonical) -> 0 failed (3613 passed across the workspace); cargo test -p kuna-decomp --lib -> 1744 passed (== base, B0 unchanged); clippy -p kuna-decomp --lib -> clean. C++ oracle (main tree): 207/207 unit + 675/675 datatest assertions, PARITY OK (rust-only diff, byte-untouched).

## One-line

A FAITHFUL, non-special-cased port of `TypeSpacebase::getSubType` (type.cc:3411-3433)
into `TypeOpPtrsub::getOutputToken` (typeop.cc:2351 -> `TypePointer::downChain`
type.cc:1221-1257), so the spacebase-PTRSUB output TOKEN resolves the global symbol and
equals the output high type — `ActionSetCasts::castOutput`'s `tokenct == outHighType`
short-circuit (coreaction.cc:2635) then fires and the SPURIOUS `(int4 *[5])` array-pointer
cast is dropped. Lands **+3 real parity** (rust-engine datatests 333 -> 336) with **ZERO
regression** (regressed assertion set EMPTY); the remaining typeseed render gap
(declaration dedup) stays gated in the reserved printc plane. ACCEPT_WITH_LOSSES (LOSS-214).

## Branch / diff scope (reserved-file check: CLEAN)

- branch 5553a07, parent/base 1596ad6; ONE commit.
- `git diff rust-port...rport/w10-spacebase-ptrsub-cast --name-only` = 5 files, ALL under
  `rust/`: `coreaction_casts.rs`, `dtype.rs`, `funcdata_spacebase.rs`, `seams.rs`,
  `tests/verify_w10_spacebase_ptrsub_cast.rs`.
- NO reserved file touched: none of coreaction_infertypes.rs / subflow.rs /
  funcdata_varnode.rs / variable.rs / coreaction_cleanup.rs (struct-return-v2 wave) or
  coreaction_protos.rs / coreaction_stackptr.rs (RSP shelf) appears in the diff.
- No non-rust file changed (C++ source / specs / baseline byte-untouched -> the C++ oracle
  is definitionally B5/675-untouched; re-confirmed PARITY OK).
- The prompt names `coreaction_casts.rs` as the wave's OWNED file; the edit is confined to
  the existing `get_output_token_ptrsub` helper (a new spacebase branch). `dtype.rs`
  (a pure `spacebase_parts()` accessor), `funcdata_spacebase.rs` (the new
  `Funcdata::spacebase_get_sub_type`), and `seams.rs` (a new `GlobalContainer.symbol_offset`
  field, sourced from the existing snapshot's `entry.get_offset()`) are NOT reserved; their
  edits are required for the feature and are faithful (see below).

## (1) FAITHFULNESS — line-by-line vs the C++ oracle: PASS

- `TypeOpPtrsub::getOutputToken` (typeop.cc:2351-2366): the `getHighTypeReadFacing` ->
  TYPE_PTR gate, `offset = addressToByte(in1->getOffset(), wordSize)`, `downChain(off,...,
  false)`, the `(off==0 && rettype!=0) -> rettype` exact-hit, else the
  `getTypePointer(outSize, getBase(1,UNKNOWN), wordSize)` `xunknown1 *` fallback — all
  present (the non-spacebase path is the UNCHANGED `down_chain`; the spacebase path is the
  new branch). Matches arm-for-arm.
- `TypePointer::downChain` spacebase arm (type.cc:1221-1257): for a pointer-to-spacebase
  the wrapping guard (:1225-1226) is correctly SKIPPED — the spacebase has
  `getAlignSize()==0`, so `ptrtoSize != 0` is false and the `off >= ptrtoSize` branch
  body never runs; the spacebase is not enum/array/struct, so the `!isArray ->
  getTypePointerStripArray(size, pt, wordsize)` branch (:1255-1256) applies. The Rust
  reproduces exactly this: metatype==TYPE_SPACEBASE -> `spacebase_get_sub_type` -> on
  `residual_off==0`, `get_type_pointer_strip_array(out_size, subtype, wordsize)`; on miss
  fall through to the `xunknown1 *` fallback. Faithful (the offset fed to
  `spacebase_get_sub_type` is the BYTE offset straight from `downChain`'s `off`, exactly as
  C++ `downChain` passes its byte `off` to `getSubType`).
- `TypeSpacebase::getSubType` (type.cc:3411-3433): `getMap()` (type.cc:3399-3409) routing
  — GLOBAL scope when `localframe.isInvalid()`, else `fd->getScopeLocal()` — is reproduced
  by the `localframe.is_invalid()` branch (`spacebase_parts()` reads
  `spaceid`/`localframe` off the `TYPE_SPACEBASE` data-type, matching type.hh:801-802). The
  global arm: `addrOff = byteToAddress(off, spaceid->getWordSize())`,
  `addr = glb->resolveConstant(spaceid, addrOff, -1, nullPoint, fullEncoding)`,
  `smallest = scope->queryContainer(addr, 1, nullPoint)`, the
  `smallest==0 -> *newoff=0; return getBase(1,UNKNOWN)` miss, the
  `*newoff = (addr.getOffset() - smallest->getAddr().getOffset()) + smallest->getOffset()`
  + `return getSymbol()->getType()` hit — all transcribed in order, with `wrapping_sub`
  for the uintb subtraction (faithful uintb wraparound). The local arm delegates to the
  pre-existing, UNCHANGED `ScopeLocal::spacebase_get_sub_type` (varmap.rs:1459), so the
  stack-spacebase path is untouched.
- `ActionSetCasts::castOutput` short-circuit (coreaction.cc:2632-2643): the fix is in
  `getOutputToken` ONLY — once the token equals the output high type, the EXISTING
  `castOutput` returns 0 (no cast). The fix does NOT touch `castStandard`, so the general
  cast plane (CastStrategyC::castStandard) is provably unaffected; legitimate casts are
  selected by the same unchanged code.
- `symbol_offset` thread: `GlobalContainer.symbol_offset` <- the snapshot's
  `entry.get_offset()` (database.rs:2639, the existing build site) -> copied at seams.rs:849
  -> consumed in `spacebase_get_sub_type` as `+ entry.symbol_offset as int8` ==
  `smallest->getOffset()`. The `resolve_constant`/`query_container_global(&addr,1,&null)`
  seam calls are the EXACT pair `ActionConstantPtr::isPointer` uses (coreaction_render.rs),
  not a new divergent symbol-resolution path.

## (2) NO SPECIAL-CASING: PASS

- Production-diff (the 4 src files, added lines only) string/hex-literal grep yields EXACTLY
  ONE token: `"spacebase ptrto present"` — an `.expect()` panic message guarded by the
  preceding `is_spacebase` test (cannot panic), NOT a key. ZERO hex constants, ZERO
  address / symbol-name / function-name / type-name literals in production code.
- The decision is driven entirely GENERICALLY: the only literal predicate is the
  `metatype == TYPE_SPACEBASE` check (generic) and the symbol-table snapshot
  (`GlobalContainer.symbol_type` <- `query_container_global`). `myarray` / `0x601060` /
  `paiGlob` / `globindex` / `twodim` appear ONLY in the verifier test assertions (correct).
  The single `myarray` token in `seams.rs` is a PRE-EXISTING doc comment, not in the diff.

## (3) NEW PASSES ARE REAL: PASS (+3, byte-verified)

Built a base harness at the branch PARENT (1596ad6, the typeseed-era base = 333) and the
branch harness; ran the FULL 675-assertion datatest sweep on both, identical `.sla`:
- base (1596ad6): `Total passing tests = 333`.
- branch (5553a07): `Total passing tests = 336`.
- FULL per-assertion passing-SET diff (sorted `Success -- <name>` lines, comm):
  NEWLY PASSING = {`Three dimension #1`, `Three dimension #2`, `Long double #14`};
  REGRESSED = EMPTY.
- `Three dimension #1/#2` and `Long double #14` are global-array spacebase-PTRSUB renders
  whose spurious `*[N]` array-pointer cast is now dropped, letting the symbol-subscript
  chain render. Independently byte-verified vs the C++ oracle B5:
  - twodim/threedim branch body shows `myarray[globindex][valin]` / `[globindex][v2][valin]`
    with NO interposed cast token (`*[5]`/`*[3]` GONE).
  - `Long double #14` branch render is BYTE-IDENTICAL to the C++ oracle:
    `ldarr[val] = ldarr[7 - val] + ldarr[val + 7];` (longdouble 4 -> 5 passing).
- The exact `myarray[globindex][valin]` single-statement (#1/#2) still does NOT pass — the
  body carries a residual per-reference declaration dup (`undefined20 myarray [3];` x2) from
  the reserved printc declaration-collection plane (LOSS-211 (2) / LOSS-214). Honest partial.

## (4) NO REGRESSION: PASS (the key risk — independently fenced)

- **cargo test --workspace (DEBUG, canonical profile)**: 0 failed; 3613 passed across the
  whole workspace; per-target grep for `FAILED|failures:|N failed|error[` is EMPTY.
  (NOTE: a `--release` run shows ONE spurious failure —
  `constseq::tests::w6s5_form_byte_array_huge_bytepos_diverges_debug_panic`, a `should_panic`
  test that relies on debug `overflow-checks`; `--release` disables them and it index-OOBs
  instead of "add with overflow". This is a PROFILE artifact of running `--release`, NOT a
  branch regression — the test PASSES in debug on the branch, and B0/lib is 1744 on both
  base and branch. The prior accepted typeseed wave gated in debug for exactly this reason.)
- Whole-suite passing 336 >= 333; FULL passing-SET diff: regressed EMPTY (above).
- THE 6 SWITCH DATATESTS (switchind/switchmulti/switchhide/ifswitch/switchloop/switchreturn):
  render BYTE-IDENTICAL base<->branch (dumped `print C`, diff -q == identical for all six).
  The local-frame (stack) spacebase getSubType routes through the UNCHANGED
  `ScopeLocal::spacebase_get_sub_type`, so jumptable index recovery is unperturbed.
- boolless / readstruct / condconst_conn: BYTE-IDENTICAL base<->branch (verified directly).
- LEGITIMATE CASTS SURVIVE: packstructaccess still has `(int4)` truncation casts (x2);
  floatcast still has `(float8)` (x4) + `(float4)` precision casts and is 9/9 passing on
  BOTH base and branch. The fix is surgical (getOutputToken token-equality), NOT a
  castStandard change.
- WHOLE-CORPUS render diff base<->branch: 19 stems changed
  (bitfields/bitfields2/copytrim/dupptr/enum/forloop_thruspecial/injectoverride/longdouble/
  loopcomment/noforloop_alias/offsetarray/partialmerge/partialsplit/ptrtoarray/threedim/
  twodim/varcross/wayoffarray). This is a BROADER set than a one-test fix, BUT: (a) the
  per-assertion regressed set is EMPTY (authoritative); (b) for EVERY changed stem the
  branch's diff-line distance to the C++ oracle is EQUAL-or-CLOSER vs base (varcross moved
  36->34 lines closer; all others unchanged-count; NONE further). Spot-checks confirm the
  changes are in the CORRECT direction: copytrim drops `(int4 *)myarr` -> `myarr[a0]` (==
  oracle `myarr[a0]`); offsetarray/wayoffarray refine the fallback cast `(xunknown1 *)` ->
  `(xunknown4 *)` (more-resolved sub-type, still LOSS-211-gated, no assertion regressed).
  These are already-failing tests whose render text changed but whose pass/fail status did
  not, all toward (or neutral with) the oracle.
- C++ oracle 675/675 PARITY OK, byte-untouched (rust-only diff). B0: lib 1744 on BOTH base
  and branch (no lib test added/removed; the +5 are integration tests). clippy --lib clean.

## Hunt list

- Signedness: clean. `slot`/`1-slot` int4; `entry.symbol_offset as int8` /
  `addr_off as ...` small valid values; `off as u64` in `byte_to_address` matches the C++
  `byteToAddress(off, ...)` uintb path.
- Wrapping: clean. `addr.getOffset() - smallest.getOffset()` uses `wrapping_sub`
  (type.cc:3431 uintb subtraction), `+ symbol_offset as int8` matches the C++ int add.
- Iteration order / containers: clean. No HashMap/HashSet/sort_unstable introduced; the
  symbol lookup is the same `find_container_entry` (smallest-covering, deterministic) the
  isPointer path uses.
- Exception->Result: clean. `spacebase_get_sub_type` returns `Option`; `None`/`?` map to the
  C++ miss (`getBase(1,UNKNOWN), newoff=0`) faithfully; `resolve_constant().ok()?` degrades
  to the no-cast fallback on the can't-happen factory-miss (C++ never errors here).
- `.expect("spacebase ptrto present")`: provably unreachable (guarded by `is_spacebase`).

## Findings

- F1 (cosmetic): the `entry.symbol_offset as int8` and `off as u64` bare `as` casts in
  `spacebase_get_sub_type` lack the verification.md `// cast:` justification comment (both
  benign — small valid offset / faithful uintb conversion). Non-blocking.
- F2 (test robustness, non-blocking): T3/T4 use `if let Some(body) = ...` rather than
  `let ... else { return }`, so they would pass-vacuously if the harness binary were
  missing. Independently confirmed via `KUNA_DECOMP_TEST_RUST` that all 5 tests RAN (no
  SKIP) and the assertions fired — so the gate is real for this verification, but the
  pattern is weaker than T1/T2/T5.

No correctness finding. No special-casing. No reserved-file edit. No regression.

## Adversarial tests (committed on the branch, all PASS, all RAN — not skipped)

- `t1_twodim_spacebase_array_renders_no_array_pointer_cast` — the fix (no `*[5]` cast;
  `myarray[globindex][valin]`).
- `t2_threedim_spacebase_array_indexes_three_deep_no_cast` — generalizes to 3D.
- `t3_legitimate_non_spacebase_casts_survive` — packstructaccess `(int4)`, floatcast
  `(float8)`/`(float4)` preserved (the surgical guard).
- `t4_local_spacebase_and_switch_cluster_intact` — ptrtoarray + all 6 switch renders intact,
  no spurious array-pointer cast leak.
- `t5_generic_rule_applies_uniformly_no_address_keyed_branch` — same generic
  getSubType rule on two independently-mapped arrays.

Verified RAN with `KUNA_DECOMP_TEST_RUST` set: 5 passed, 0 SKIP.

## losses

LOSS-214 (appended): the spurious `(int4 *[5])` array-pointer CAST that LOSS-211 (1) flagged
is now REMOVED (this wave is its restoration), but the exact `myarray[globindex][valin]`
single-statement render (Two dimension #1/#2) is still blocked by the per-reference symbol
RE-DECLARATION (`undefined20 myarray [3];` duplicated) — the printc declaration-collection
dedup that lives in the RESERVED printc plane (LOSS-211 (2), unchanged by this wave). Plus a
declared deferral: the segment-op near-pointer promotion is unrelated and untouched.
Zero-regression; +3 real parity now.
