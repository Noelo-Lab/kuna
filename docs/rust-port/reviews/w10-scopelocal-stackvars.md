# w10-scopelocal-stackvars
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-14
gate: `cd rust && cargo test --workspace` -> 3184 passed / 0 failed; `cargo clippy -p kuna-decomp -p kuna-console --lib -- -D warnings` -> clean; C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` (MAIN tree, untouched) -> 207/207 unit + 675/675 datatests, PARITY OK.

## Scope of the branch (what actually changed)

rust-only diff (no `.cc`/`.hh`/spec touched; verified). varmap blob shas match the
recorded oracle (varmap.cc `27330a23…`, varmap.hh `5143b24b…`). Five files:

- `varmap.rs`: new `ScopeLocal` struct + `new`/`reset_local_window`/`add_symbol`/
  `add_code_label`/`set_attribute`/`adjust_fit`/`create_entry`/`restructure`/
  `build_variable_name_stack` (varmap.cc:341-660, 1294-1325).
- `funcdata.rs`: `localmap` is now a real `Option<ScopeLocal>` built in the ctor
  (funcdata.cc:54-71); `get_scope_local{,_mut}` + a deferred `reset_local_window`.
- `database.rs`: `public_make_name_unique` (ScopeLocal owns its own `Database`).
- `ifacedecomp.rs`: `IfcMapaddress`/`IfcMaplabel` fd-local branches wire into
  `getScopeLocal()->addSymbol`/`addCodeLabel` (ifacedecomp.cc:550-577, 709-723),
  replacing the prior `engine_unavailable` seam.

## Faithfulness (transcription, not shortcut)

- `ScopeLocal::ScopeLocal` ctor (varmap.cc:341): field inits 1:1 (`minParamOffset=~0`,
  `maxParamOffset=0`, `stackGrowsNegative=true`, …). The owned-`Database` self-containment
  is documented and matches `ScopeInternal`'s private-member model; only `makeNameUnique`
  and `setRange`/`removeRange` cross the boundary, all consulted on the scope's OWN tree.
- `resetLocalWindow` (varmap.cc:432): range-locked early return, local+param range copy
  into a fresh `RangeList` (BTreeSet, sorted — matches C++ `set<Range>` order), `setRange`
  into the scope's own rangetree. Faithful. Lazy-call deferral (proto model set in the
  proto-recovery wave) is LOSS-136.
- `adjustFit` (varmap.cc:587): `longestFit`, the shrink-to-`maxsize`, the `findOverlap`
  shrink to `entry.addr-start`, the type-size floor — all transcribed. The `entry.getAddr()
  <= addr` test is narrowed to `same_space && offset<=offset`; I checked `find_overlap` is
  space-local (indexes the maptable by `addr`'s own space), so the C++ cross-space branch of
  `Address::operator<=` is unreachable here — the narrowing is benign, not a divergence.
- `createEntry` (varmap.cc:617): `concretize`, `size/alignSize` int4-division (guarded for
  align 0), `getTypeArray` when `num>1`, `addSymbol("")`. The empty name is correct: like
  C++ `Scope::addSymbol -> addSymbolInternal`, the Rust `add_symbol_internal` applies
  `build_undefined_name()` to an empty name; the `Stack_*` name is applied LATER by the
  naming pass (a separate seam), not at restructure time. So the unit test asserting
  *mapped symbols* (not names) at restructure is the right, non-vacuous assertion.
- `restructure` (varmap.cc:1294): the `initialize`-false early-out, the
  `next.sstart < cur.sstart+cur.size` (signed) intersect test, `merge`/`attemptJoin`, the
  `open`-range `size = next.sstart-cur.sstart` (int4 truncation), `adjustFit`+`createEntry`.
  KEY: C++ `RangeHint::merge` NEVER returns true (every reachable `return` is `false`; the
  only non-false exit is the locked-vs-locked `throw`), so the `overlapProblems=true` branch
  is effectively dead — the Rust faithfully reproduces this (merge returns only `Ok(false)`
  or `Err`). Pinned by two new adversarial tests.
- `buildVariableName` (varmap.cc:548): the `(addrtied|persist)==addrtied && in-space &&
  in-local-range` gate, `byteToAddress`+`sign_extend`, the `stackGrowsNegative` negate, the
  X (caller-region) / Y (unusual-region, `minParamOffset<maxParamOffset`) markers, the
  `<TypeBase><Space>[XY]_<hex>` assembly, `makeNameUnique`. The C++ `else { if(A&&B) }` is
  collapsed to `else if A { if B }` — provably equivalent. String-build order (typebase,
  spacename, X/Y, '_', hex) matches C++ exactly. Faithful.

## No special-casing

`git diff … | grep '^+' | grep -E '== "|starts_with|0x40|addr ==|offset =='` over the
added lines -> EMPTY. No function-name / address / literal-local-name branch anywhere in the
diff. The naming is a pure structural convention over the stack space; the gate predicates
are `in_space`/`in_local_range`/flag bits, not literals. CLEAN.

## Are the named locals REAL? Are new passes REAL?

- The promotion CODE is real and faithful, and the in-module restructure->createEntry->
  mapped-symbol chain works (verified by the porter's + my tests). BUT it is NOT driven by
  the engine: `ActionRestructureVarnode::apply` (coreaction_render.rs:810) is STILL a no-op
  seam (returns 0, never calls `restructure`/`getScopeLocal`), and `restructure` /
  `build_variable_name_stack` / `Funcdata::reset_local_window` have NO pipeline callers
  (only tests). Confirmed by grep. So engine-produced `Stack_*` locals cannot be observed on
  any datatest, and I could not spot-check a function's locals vs the C++ oracle for named
  stack vars — that path does not run yet.
- This is NOT a hidden weakening: the commit message states it plainly (localmap stays None
  on every datatest because the stack `SpacebaseSpace` is not created — LOSS-012 — and the
  positive-pass count "holds at 20/391"). NO new datatest passes are claimed, so there is no
  fake/vacuous parity. The branch's real, pipeline-reachable effect is closing the
  `getScopeLocal()->addSymbol`/`addCodeLabel` ERROR seam (8->0 occurrences) so those files
  reach the NEXT blocker — a faithful, honest increment.
- The fd-local console path IS reachable end-to-end for register-space addresses (e.g.
  `map label mylabel r0x100913` with an fd loaded routes into the owned `ScopeLocal` db); the
  stack-address form is blocked upstream by LOSS-012, not by this code.

## Hunt list

- Signedness: CLEAN. `next.sstart < cur.sstart.wrapping_add(cur.size as int8)` is the C++
  signed `intb` compare (size int4 -> intb promotion); the `as u64`/`as int4` casts in
  `adjust_fit` are value-identical to C++'s implicit int4<->uintb conversions (checked the
  pathological negative case too).
- Integer widths: CLEAN. `id = 0x57AB12CD<<32 | (off&0xffffffff)` matches funcdata.cc:62-64
  uint8. `diff+b.size` in merge uses int4 signed compare matching C++.
- Wrapping: CLEAN. `wrapping_add`/`wrapping_sub`/`wrapping_neg` on the offset/size lineage;
  C++ comment "implementation does not allow a range to contain both ~0 and 0" so the wraps
  are defensive and match practical C++ behavior.
- Comparator totality: N/A here (RangeHint::compare ported + verified in W7; unchanged).
- Iteration order: CLEAN. `reset_local_window` iterates `RangeList` (BTreeSet, sorted) ==
  C++ `set<Range>`; no HashMap/HashSet introduced.
- Off-by-one / do-while: CLEAN. `restructure`'s `while state.get_next()` + `next()` capture
  mirrors `getNext()`/`next()` (iter advance then end-check), the terminal artificial range
  is not entered, matching C++.
- Erase-while-iterating: N/A (no erase-during-traversal in the added code; `markNotMapped`'s
  symbol-removal loop is NOT ported here — it stays under LOSS-109).
- Exception->Result parity: CLEAN. The single throwing site in `merge`
  ("Overlapping forced variable types") -> `Err`; pinned by a new adversarial test. The
  `public_make_name_unique` swallow-to-raw-name fallback matches "makeNameUnique only throws
  when it cannot uniquify, which the stack convention always can."
- Bare `as` casts WITHOUT `// cast:` comments (F1, minor): the new code has ~8 bare casts
  (`a.size as u64`, `maxsize as int4`, `cur.size as int8`, `start as uintb`, …) that the
  verification protocol asks to carry a `// cast:` justification; they instead rely on
  inline prose comments. Each is value-faithful to the corresponding C++ implicit/explicit
  conversion (verified individually), so this is a convention/process gap, not a correctness
  bug — no behavioral effect. Noted, not blocking.

## Mechanical pass

- `grep '^+' diff | grep -E 'HashMap|HashSet|todo!|unimplemented!|sort_unstable|partial_cmp|raw .unwrap()'`
  -> EMPTY. CLEAN.
- cargo test --workspace: 3184 passed, 0 failed (matches the commit's claim; was 3180 pre-branch).
- lib clippy clean. (`--tests` clippy surfaces pre-existing doc/hex-casing lints in unrelated
  `tests/verify_w*.rs` files — not introduced by this branch, not in scope.)

## Adversarial tests (added by verifier, in `rust/crates/kuna-decomp/src/varmap/tests.rs`)

- `w10_scopelocal_build_variable_name_y_marker` — the param-region 'Y' branch guard
  (`minParamOffset<maxParamOffset`); proves no spurious 'Y' when boundaries are unset.
- `w10_scopelocal_restructure_empty_state_is_noop` — `initialize()`-false early-out: no
  symbols, no overlap flag.
- `w10_scopelocal_restructure_intersecting_nonlocked_does_not_flag_overlap` — pins the
  C++-faithful fact that `merge` never returns true, so two intersecting incompatible
  non-locked hints are reconciled into one unknown-typed cover and `overlapProblems` stays
  false.
- `w10_scopelocal_merge_locked_vs_locked_errors` — the one throwing site
  (locked-vs-locked irreconcilable types) -> `Err` (exception->Result parity).
- `w10_scopelocal_build_variable_name_out_of_range_falls_through` — the `in_local_range`
  gate and the wrong-space gate both return `None` (no name/address special-casing).

All 5 pass.

## findings

  - F1 (minor, non-blocking): bare `as` casts in the new `varmap.rs` ScopeLocal code lack the
    protocol-mandated `// cast:` justification comments; each is value-faithful to the C++
    conversion it ports (verified individually), so no correctness impact.
       cpp: decompiler/cpp/varmap.cc:601 (`a.size = (int4)maxsize`), :629 (`a.size/ct->getAlignSize`), :555 (`(intb) AddrSpace::byteToAddress`)
       rust: rust/crates/kuna-decomp/src/varmap.rs:1004,1019 (adjust_fit casts), :1077 (byte_to_address as int8), :1102 (start as uintb)

## losses

- LOSS-109: the IR-mutating `ScopeLocal` restructure pipeline driver (the `MapState` IR-walk
  gather, `syncVarnodesWithSymbols`, `markNotMapped`/`collectNameRecs`, and the no-op
  `ActionRestructureVarnode::apply`) stays seamed — so engine-driven `Stack_*` locals do not
  yet appear in decompilation output. This branch ports the layout/naming methods ON TOP of
  the ported core, exactly per LOSS-109's restoration criteria.
- LOSS-136: `funcp.setScope(localmap,baseaddr-1)` + the eager `resetLocalWindow()` in the C++
  ctor are deferred (proto model set in the proto-recovery wave; `reset_local_window` made a
  lazy/no-op-until-model call).
- LOSS-012: the stack `SpacebaseSpace` is not created (no cspec `<stackpointer>` ->
  `addSpacebase`), so `localmap` is `None` on every datatest and stack-address `map` commands
  cannot resolve `s0x…` — the upstream blocker that keeps the named-locals path from firing.

ACCEPT-WITH-LOSSES: the transcription is faithful, not special-cased, not weakened, and
claims no fake parity (no new datatest passes are asserted). All divergences from the C++ are
pre-existing, documented seams (LOSS-109/136/012). The branch is a correct, honest increment
that ports the ScopeLocal layout/naming layer and closes the console `getScopeLocal()` ERROR
seam; the engine-side promotion remains future work under LOSS-109.
