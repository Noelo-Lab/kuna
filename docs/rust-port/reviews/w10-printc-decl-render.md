# w10-printc-decl-render
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-17 (round 1)

## One-line

A FAITHFUL, non-special-cased port of three printc render seams — (A) the
`emitScopeVarDecls` proto-partial / multi-entry declaration collapse
(`isPiece` + `getFirstWholeMap() != entry`, printc.cc:2688/2697), (B) the
local-scope-only declaration walk (`emitLocalVarDecls` ->
`emitScopeVarDecls(fd->getScopeLocal())`, printc.cc:2336/2667, so a GLOBAL-scope
symbol is named in the body but never locally declared), and (C) the
`checkAddressOfCast` + `opTypeCast` pointer-to-array address-of arm
(printc.cc:396-438 / 468-484, a `&sym` render dropping the spurious `(T(*)[n])`
cast). Lands **+1 real parity** (rust-engine datatests 348 -> 349, the
newly-passing assertion is EXACTLY `Pointer to array #7` `paiGlob = &c;`) with
**ZERO regression** (regressed assertion set EMPTY). The SEAM-A/B decl deletions
are render-only and move EVERY one of the 8 changed corpus stems CLOSER to the
C++ oracle. ACCEPT_WITH_LOSSES (LOSS-217 — SEAM A is largely DORMANT on the
corpus; only SEAM C newly flips an assertion).

## Branch / diff scope (reserved-file check: CLEAN)

- branch 01cca2b, merge-base/parent rust-port@80c65fe; ONE commit
  (`git log rust-port..rport/w10-printc-decl-render` = exactly 1; the "(+1)" in
  the message is the parity delta, not a second commit).
- `git diff rust-port...rport/w10-printc-decl-render --name-only` = 4 files, ALL
  under `rust/`: `funcdata_varnode.rs`, `printc.rs`, `variable.rs`,
  `tests/verify_w10_printc_decl_render.rs`. +644/-22. RUST-ONLY.
- RESERVED FILES UNTOUCHED: neither `coreaction_cleanup.rs` (running f0-flag-v2
  wave) nor `dynamic.rs` (Convert shelf) appears in the diff. CLEAN.
- `funcdata_varnode.rs` (the `info_is_global` flag set in `linkSpacebaseSymbol`)
  and `variable.rs` (the new `kuna_global` field + 2 accessors + a deterministic
  `HighVariableBank::iter()`) are NOT reserved; their edits are the SEAM-B flag
  plumbing and are faithful. The bulk of the feature is in `printc.rs`
  (the owned printc plane LOSS-211/214 freed).
- NO non-rust file changed: C++ `printc.cc` is BYTE-IDENTICAL to main (`diff -q`),
  no specs / baseline / `.cc` / `.hh` / `.json` byte touched -> the C++ oracle is
  definitionally B5/675-untouched (re-confirmed 675/675 below). Working tree has
  no non-rust modification.

## (1) FAITHFULNESS — line-by-line vs the C++ oracle: PASS

- `PrintC::checkAddressOfCast` (printc.cc:396-438) -> `check_address_of_cast`
  (printc.rs:2960+): transcribed arm-for-arm in order — dt0 = out
  HighTypeDefFacing, dt1 = in0 HighTypeReadFacing, the dual `!=TYPE_PTR` gate, the
  `getPtrTo()` walk, `base0->getMetatype()!=TYPE_ARRAY -> false`, `arraySize =
  base0->getSize()`, `base0 = TypeArray::getBase()`, the dual `while(getTypedef())`
  unwrap, the `base0 != base1 -> false` identity check, the
  `getSymbolEntry() && getSymbolOffset()==-1 -> symbol->getType()` whole-symbol
  arm, the `isWritten() -> def==CPUI_PTRSUB -> rootType TYPE_PTR -> getPtrTo() ->
  getSubType(off,&off) -> off!=0 ? return false` PTRSUB arm, the
  `symbolType==0 -> false` miss, and the final
  `metatype!=TYPE_ARRAY || size!=arraySize -> false`. Every branch present and in
  the C++ order. `get_ptr_to` returns the ptrto for both `Pointer` and
  `PointerRel` (faithful: a C++ `TypePointerRel` IS-A `TypePointer`, so the
  `(TypePointer*)` cast + `getPtrTo()` resolves the same).
- The `getSubType` virtual dispatch is handled FAITHFULLY: for a spacebase root
  (`root_ptr_to.get_metatype()==TYPE_SPACEBASE`) the call routes to
  `Funcdata::spacebase_get_sub_type` (the ported `TypeSpacebase::getSubType`,
  funcdata_spacebase.rs — the SAME helper the accepted spacebase-PTRSUB-cast wave
  uses), every other root uses the bare `Datatype::get_sub_type`; a `get_sub_type`
  Err (the `SEAM(W6)` arm) -> `return false` (conservative: never collapses a cast
  it cannot prove redundant). This mirrors C++ where `getSubType` is virtual and
  `TypeSpacebase::getSubType` reaches the symbol table the bare `Datatype` cannot.
- `PrintC::opTypeCast` (printc.cc:468-484) -> `op_type_cast_ir` (printc.rs:2900+):
  the `dt->isPointerToArray()` gate + `checkAddressOfCast(op)` -> `pushOp(&addressof)
  ; pushVn(in0); return;` arm is added exactly above the existing
  `!option_nocasts -> pushOp(&typecast); pushType(dt)` + `pushVn(in0)` tail (the
  prior `// SEAM(...)` placeholder is now realized). `is_pointer_to_array` reads
  the `pointer_to_array` flag (dtype.rs:1010), faithful to C++ `isPointerToArray`.
- SEAM A — `emitScopeVarDecls` (printc.cc:2667-2724): C++ skips an entry on
  `entry->isPiece()` (:2688) and on `isMultiEntry() && getFirstWholeMap() != entry`
  (:2697). The kuna stand-in (`is_proto_partial_piece`, printc.rs:1824) models "a
  non-whole entry of a multi-entry Symbol whose first-whole-map is a DIFFERENT
  entry": composite `kuna_symbol_type` (STRUCT/ARRAY/UNION — a scalar field varnode
  never carries the root whole-struct type on its own) AND `kuna_symbol_offset >= 0`
  (the pieces carry `>= 0`; the root keeps the `-1` whole-symbol-match default) AND
  a same-`name` sibling high with `kuna_symbol_offset == -1` (the
  `getFirstWholeMap()` ROOT). The root (offset `-1`) is NOT skipped -> exactly ONE
  declaration per group, == C++ "only emit the first SymbolEntry". A referenced
  WHOLE local (`&a`, composite type, its own offset `-1`) has NO `-1` sibling of
  its name except itself (excluded by `id != except`) -> stays declarable (T2b).
  `high_name_has_whole_sibling` is an `.any()` existence test over a `BTreeMap`
  (`HighVariableBank::iter()`) — deterministic and order-independent. The kuna
  per-function `vN` allocator makes names unique, so the only same-name sibling is
  the group root. FAITHFUL to the OBSERVABLE C++ behavior within the kuna naming
  model (the proto-partial offsets are stamped by `bind_proto_partial_piece`,
  variable.cc:259, per the accepted structreturn-v2 wave).
- SEAM B — `emitLocalVarDecls` (printc.cc:2336/2667) walks ONLY
  `fd->getScopeLocal()`, so a GLOBAL-scope Symbol is never locally declared. The
  kuna discriminator: `info_is_global = local_hit.is_none()` in `linkSpacebaseSymbol`
  (funcdata_varnode.rs:1099) — true EXACTLY when the LOCAL-scope
  `query_container_for_link` missed and the GLOBAL-scope `name_for_global_varnode`
  fallback hit (the C++ `sb->getMap()` returns `glb->getGlobalScope()` for a ram
  spacebase). Carried onto the high as `kuna_global` and OR-ed into the existing
  `is_persist` global test in `emit_local_var_decls` (printc.rs:1791). A local-frame
  `&a`/`&c` resolves via the local query (local hit) -> `info_is_global` false ->
  decl prints. FAITHFUL and SCOPE-driven, not const-driven (T2b pins this).

## (2) NO SPECIAL-CASING: PASS

- Whole-diff (3 src files, ADDED production lines only) string-literal grep yields
  EXACTLY ONE token: `"not in `fd->getScopeLocal()`"` — inside a DOC COMMENT, not a
  key. ZERO `myarray` / `glob` / `concat` / `paiGlob` / `"v1"` / `"foo"` / `".A"` /
  field-name / function-name / hex-address literals in production code.
- Numeric-literal grep over the same added lines: only `0` (offset / `>= 0`), `1`
  (offset / instance index), `10` (the `Datatype::compare` recursion-DEPTH `level`,
  type.cc:216 — a standard value used factory-wide, not a magic key), and `-1`
  (`kuna_symbol_offset() == -1`, the C++ whole-symbol-match `getSymbolOffset()==-1`,
  printc.cc:417). All structural.
- The three decisions are GENERIC over the declaration / scope / cast geometry: the
  composite-metatype test, the scope-resolution miss flag, and the pointer-to-array
  `+` `checkAddressOfCast` predicate — never opcode-, name-, value-, or
  address-keyed. The symbol names (`myarray`/`paiGlob`/`concat`/`c`) appear ONLY in
  the verifier test assertions (correct).

## (3) NEW PASSES ARE REAL: PASS (+1, byte-verified)

Built a baseline DEBUG harness at the merge-base (80c65fe; the originally-supplied
`/tmp/kuna-f0v2-base` worktree was deleted mid-run by the concurrent f0-flag-v2
wave — re-created `/tmp/kuna-verify-pdr` and rebuilt) and ran the FULL 675-assertion
datatest sweep on both, identical `.sla` set:
- base (80c65fe): `Total passing tests = 348`.
- branch (01cca2b): `Total passing tests = 349`.
- FULL per-assertion passing-SET diff (sorted `Success -- <name>`, `comm` both
  directions): NEWLY PASSING = {`Pointer to array #7`}; REGRESSED = **EMPTY**.
- `Pointer to array #7` requires `paiGlob = &c;` (ptrtoarray.xml:57). The branch
  renders EXACTLY `paiGlob = &c;` (SEAM C dropped the base's
  `(int4 *[16])c`) — byte-verified against the C++ oracle B5
  (`passPtrToArray` body identical on the address-of statement).
- Honest PARTIAL: `Pointer to array #8` (`floatarray(&a)`) / `#9`
  (`intarray(&myval.b)`) do NOT pass — they render `floatarray((float4 *)a)` /
  `intarray((mystruct *)&myval + 4)` (a different cast/pointer-arith seam not
  reached by the pointer-to-array `checkAddressOfCast` geometry). SEAM C lands #7
  only.

## (4) NO REGRESSION: PASS (the key risk — decl emission is corpus-wide — independently fenced)

- **cargo test --workspace (DEBUG/canonical) INDEPENDENTLY RUN**: 3628 passed / 0
  failed / 37 ignored across the whole workspace; per-target grep for
  `FAILED|[1-9][0-9]* failed` is EMPTY. >> the >=348 floor.
- **REGRESSED SET EMPTY**: full both-direction `comm` of the base(348) vs
  branch(349) `Success --` sets — every baseline-passing assertion still passes;
  +1 net (`Pointer to array #7`).
- WHOLE-CORPUS render byte-diff base<->branch (both DEBUG harnesses, KUNA_DUMP=1,
  all 83 stems): 8 stems changed
  {concat, copytrim, injectoverride, longdouble, ptrtoarray, retstruct, threedim,
  twodim}. For EVERY one the diff-line distance to the C++ oracle B5 is strictly
  CLOSER vs base (concat 28->16, copytrim 33->30, injectoverride 20->19,
  longdouble 33->24, ptrtoarray 39->37, retstruct 22->20, threedim 7->5,
  twodim 8->7) — NONE further. Spot-checks confirm correct direction: every change
  is a DELETION of a redundant/duplicate declaration (concat/retstruct drop the
  per-piece `int4 v1;` scalar dups, keeping one `foo v1; // rax` == oracle;
  copytrim/longdouble/twodim/threedim drop the duplicated GLOBAL-array decls
  `int4 myarr [2];` / `float10 ldarr [25];` / `undefined20 myarray [3];` /
  `undefined8000 myarray [100];` — the oracle declares NONE of those globals in the
  body) or the SEAM-C `(int4 *[16])c` -> `&c` flip. No over-deletion: where the
  oracle keeps a decl (concatreturn `foo v1;`), the branch keeps it.
- boolless (incl. its full body) + the 6 SWITCH datatests
  (switchind/switchmulti/switchhide/ifswitch/switchloop/switchreturn): render
  BYTE-IDENTICAL base<->branch (the struct-return / global-spacebase / address-of
  paths are dormant there). readstruct / condconst_conn: identical (both emit no
  KUNA_DUMP body — trivially equal, and pass/fail unchanged).
- C++ oracle: 675/675 PARITY OK, byte-untouched (rust-only diff; `printc.cc`
  byte-identical to main; independently re-ran the main C++ `decomp_test_dbg` ->
  `Total passing tests = 675`).
- B0 (`universalaction_listing`): 1/0 on the branch (action pass order untouched).
- lib (`cargo test -p kuna-decomp --lib`): 1749 passed on BOTH base and branch
  (== B0 lib baseline; the wave added NO lib test — the 5 new tests are all in the
  integration test file).
- `cargo clippy --lib -p kuna-decomp`: clean (0 warnings/errors).

## Hunt list

- Signedness: clean. `kuna_symbol_offset() >= 0` / `== -1` are the C++
  `getSymbolOffset() == -1` whole-match (int4). `off as int8`, `array_size`
  (`getSize()` int4), `get_size()` comparisons all int-typed, no narrowing.
- Wrapping: clean. No arithmetic in the new printc decl arms beyond the
  `getSubType` `off` thread (delegated to the already-faithful
  `spacebase_get_sub_type` / `get_sub_type`, which the accepted spacebase wave
  hunted). `off != 0 -> return false` matches C++ exactly.
- Containers / iteration order: clean. `HighVariableBank` is a `BTreeMap`;
  `iter()` is ordered; `high_name_has_whole_sibling` uses `.any()` (existence,
  order-independent). `emit_local_var_decls` dedups via a `BTreeSet<HighVariableId>`
  and iterates `vbank().iter_loc()` (the pre-existing local order). NO HashMap /
  HashSet / sort_unstable introduced.
- Type identity: the one DIVERGENCE (F1 below) — `base0 != base1` is C++ pointer
  identity; the Rust uses `Rc::ptr_eq || compare(...,10)==0`. The factory interns
  every data-type to a unique allocation (dtype.rs:2074), so `Rc::ptr_eq` IS the
  faithful identity check; the `compare==0` fallback is conservative-permissive and
  unobservable here.
- Exception->Result: clean. `get_sub_type` Err -> `return false` (conservative);
  every `.and_then`/`?` / `None ->` maps to the C++ early `return false` /
  `symbolType==0` miss. No partial mutation.

## Findings

- F1 (faithfulness nit, non-blocking): `check_address_of_cast`'s `base0 != base1`
  equality uses `Rc::ptr_eq(&base0,&base1) || matches!(base0.compare(&base1,10),
  Ok(0))`, whereas C++ (`base0 != base1`) is STRICT pointer identity. With the
  factory's type interning, `Rc::ptr_eq` is the faithful primary check; the
  `compare==0` structural fallback is MORE permissive than C++ (two distinct
  interned allocations that compare-equal would pass in Rust, fail in C++). It can
  only ever collapse a cast C++ would keep (never the reverse), and the regressed
  set is EMPTY + all 8 changed stems moved toward the oracle, so it is unobservable
  on the corpus. Tighten to bare `Rc::ptr_eq` (or drop the fallback) for exact
  faithfulness. Non-blocking.
- F2 (process, non-blocking): the `10` `compare` level and the few bare `as int8`
  casts in `check_address_of_cast` lack the verification.md `// cast:` justification
  comment (all benign — small valid offsets / standard recursion depth).

No correctness finding. No special-casing. No reserved-file edit. No regression.

## Adversarial tests (5 committed, all PASS, all RAN — not skipped)

In `rust/crates/kuna-decomp/tests/verify_w10_printc_decl_render.rs`; verified RAN
(no SKIP/panic) against the DEBUG harness via `KUNA_RUST_PROFILE=debug`:
- `t1_concat_struct_return_shares_one_whole_symbol_decl` — SEAM A: concat's
  register-returned struct collapses to ONE `foo v1;` (no per-piece `int4 v1;`),
  field writes intact; generalizes to `fooshort`.
- `t2_global_spacebase_array_is_not_locally_declared` — SEAM B: twodim/threedim
  `myarray` named in the body, NEVER locally declared.
- `t2b_local_frame_spacebase_symbol_still_declared` — SEAM B is SCOPE-driven not
  const-driven: ptrtoarray's stack array `c [16]` STILL declared.
- `t3_address_of_array_cast_collapses_to_ampersand_name` — SEAM C:
  `paiGlob = &c;`, no `(int4 *[16])` cast.
- `t4_seam_c_does_not_remove_legitimate_casts` — SEAM C is surgical: floatcast
  `(float8)`/`(float4)` and packstructaccess `(int4)` preserved; ptrToArray body
  intact.

## losses

LOSS-217 (appended): SEAM A (the `emitScopeVarDecls` proto-partial declaration
collapse) is FAITHFUL but largely DORMANT on the corpus — its corpus payoff
depends on the same RulePieceStructure split that LOSS-215 records as not-yet-reached
(it only DEDUPS already-materialized piece decls in concat/retstruct, no NEW
assertion flips from A). SEAM B (global-scope skip) and SEAM C (address-of-cast)
ARE decisive: SEAM C flips `Pointer to array #7` (+1) and SEAM A/B together delete
the per-reference duplicate global/piece decls that LOSS-214 flagged as the
reserved-printc-plane blocker — moving twodim/threedim/copytrim/longdouble CLOSER
to the oracle (but `Two dimension #1/#2` still need the unrelated implied-var CSE
seam, and `Pointer to array #8/#9` need a separate cast/pointer-arith seam). ZERO
regression; +1 real parity.
