# w10-indproto
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4.8 (independent verifier, structural separation honored — no porter notes consulted)
date: 2026-06-16
gate: cargo test --workspace -> green (0 failed); decomp_test_dbg datatests (full corpus) -> 675 applied / 272 passing, indproto.xml EXEC-FAILURE 1->0

## Scope

3 porter files (rust-only diff, vs merge-base cc925d3 = parent of 9adceea):
- rust/crates/kuna-console/src/grammar.rs (`TypeModifier::Function` `mod_type` arm: build PrototypePieces + `get_type_code_proto`)
- rust/crates/kuna-decomp/src/architecture.rs (`init` wires `defaultfp` + addr-space manager into the standalone factory via `set_proto_context`)
- rust/crates/kuna-decomp/src/dtype.rs (`make_type_code_proto` = `getTypeCode(PrototypePieces)`/`setPrototype`; activated `compare_code_basic`/`compare_code`/`compare_dependency_code` arms previously `SEAM(W6)` Err-stubs)

C++ oracle (decompiler/cpp/, specs/, datatests/, baseline.json) byte-identical to base — `git diff cc925d3...rport/w10-indproto -- decompiler/cpp/ specs/ decompiler/datatests/ docs/baseline.json` empty.

## (1) FAITHFULNESS — getTypeCode(PrototypePieces)/setPrototype is a real transcription

CLEAN. `make_type_code_proto` (dtype.rs) is statement-for-statement vs `TypeFactory::getTypeCode(PrototypePieces)` (type.cc:4476-4482) + `TypeCode::setPrototype` (type.cc:3177-3190):
- `TypeCode()` ctor: `Datatype(1,1,TYPE_CODE)` + `type_incomplete` (0x400); setPrototype ORs `variable_length` (128); `markComplete()` clears incomplete. Flag bits verified against type.hh:179/182.
- `new FuncProto()` -> `setInternal(sig.model, voidtype)` -> `updateAllTypes(sig)` -> `setInputLock(true)` -> `setOutputLock(true)` — all real, fully-implemented W6 machinery (`update_all_types` runs `ProtoModel::assign_parameter_storage`, sets output, iterates inputs w/ HIDDENRETPARM handling, `ParamUnassigned` error path). `set_internal` is faithful to fspec.cc:3896-3902 (`if model.is_none() set_model`). NOT a shortcut to parse one declaration — it drives genuine prototype storage assignment.
- `sig.model` = `decl->getModel(glb)` (grammar.cc:2347-2357) = `glb->defaultfp` for a model-less fn-pointer field; supplied via `set_proto_context`. Architecture wires it once after `build_default_proto()` finalizes `defaultfp` (verified ordering, architecture.rs:1766-1775).
- `compare`/`compareDependency`/`compareBasic` arms (type.cc:3252-3350) faithful: surface comparison, `nump != opnump` tie-break (`opnump < nump`), model-name string ordering, the `level-=1; if level<0` early-out in `compare` (present), absent in `compareDependency` (correctly absent). `compareDependency`'s `param < opparam` raw-pointer compare -> `compare_dependency_ptr` (`Rc::as_ptr as usize`), same total order as upstream within a run.

## (2) NO SPECIAL-CASING

CLEAN. grep over added executable lines: zero type-name / function-name / declaration-string / address / magic-number hardcoding. The only string literals are diagnostic ids (`"compare"`/`"compareDependency"`) passed to `code_invariant_err`. The rendered field/param come from the parsed PrototypePieces, never a hardcode. `first_var_arg_slot: -1` is a literal but is the faithful transcription of the C++ dead branch (see hunt list), not a special-case.

## (3) indproto REALLY decodes

CONFIRMED. Re-ran decomp_test_dbg on both branches against identical built `.sla`:
- base cc925d3: indproto.xml EXEC-FAILs (`Error in C syntax: ... getTypeCode(PrototypePieces) not yet ported / Execution error: Bad C syntax / Aborting process`); its 3 assertions counted as not-applied (672 applied / 271 passing).
- branch: indproto.xml decodes; 675 applied / 272 passing. EXEC-FAILURE 1->0.
- KUNA_DUMP on branch shows a clean body (no "Exceeded maximum restarts"): `(*ptr->get)()` / `(*ptr->peek)()`. `Indirect prototype #1` (negative restart guard) = Success. `#2`/`#3` (positive `ptr->peek)(a)` / `ptr->get)(a)`) still FAIL — the indirect-call ARGUMENT is not yet pulled (downstream call-arg recovery, owned by other waves). This is the PARTIAL -> LOSS-195.

## (4) NO REGRESSION

CLEAN.
- Passing SET base->branch: regressions EMPTY (`comm -23 base_pass branch_pass` empty); gains = {`Indirect prototype #1`} only. Strict superset.
- boolless `Boolean thru Less-than` 2/2, condconst `Conditional Constant` 12/12 + `Immediate Conditional` 4/4 + `Modified conditional constant` 1/1 — byte-identical base==branch (the `condconst_conn`/`readstruct` names in the task map to these condconst/struct assertions; all unregressed). No `readstruct.xml`/`condconst_conn.xml` exists in the corpus.
- No NEW exec-failures (0 "Execution failed" in the full branch run; the single removed one is indproto's).
- Reserved files NOT edited: merge.rs / printc.rs / unionresolve.rs / funcdata_union.rs all absent from the diff (only grammar.rs / architecture.rs / dtype.rs touched).
- cargo test --workspace green (0 failed across all crates). clippy `-p kuna-decomp -p kuna-console --lib` clean; harness tests + new test clippy clean.
- C++ oracle 675/675 PARITY OK untouched (Rust-only diff, C++ tree byte-identical).

## Mandatory hunt list

- Signedness: clean. New loops `for i in 0..nump` (nump: int4) = C++ `for(int4 i=0;i<nump;++i)`; both signed; negative/zero nump -> empty loop both sides.
- Integer widths: clean. No int promotion in added arithmetic; flag bits exact (128, 0x400).
- Wrapping: clean. No arithmetic on uintb-lineage in added lines (only flag OR/AND-NOT on `flags`).
- Comparator totality: clean. `compare_dependency_ptr` is a total order (`Rc::ptr_eq` equality, `as_ptr as usize` strict order) — faithful to C++ `param < opparam` raw-pointer compare; feeds `TreeKey::cmp` (BTreeSet) consistently. `compare_code_basic` tie-breaks (nump, model-name, comparable-flags) are total. The `.unwrap_or(0)` in TreeKey::cmp is pre-existing; the new Err paths (`compareBasic==2` implies both protos present) are provably unreachable, matching C++'s unconditional deref.
- Iteration-order: clean. getInTypes `for p in paramlist` = C++ vector index; compare loops `0..nump` = C++ vector index. No HashMap/HashSet in any touched file.
- Off-by-one / do-while / reverse iter: clean. No do-while; `compare`'s `level<0` early-out reproduced; `compareDependency` has none (correct).
- Erase-while-iterating: n/a (no container mutation during traversal in added code).
- Exception->Result partial-state: clean. The added `?`/`ok_or_else` points are on unreachable invariants; `update_all_types`'s `ParamUnassigned` branch sets the error flag and continues exactly as C++ (already W6-verified).

## Mechanical pass

- No `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`panic!`/`unwrap()`/`expect()` in added lines.
- No bare `as` casts in added src lines (the one `as usize` in `compare_dependency_ptr` is pre-existing and carries `// cast:`).
- clippy `-p kuna-decomp -p kuna-console --lib -- ` clean; new test crate clippy clean.

## Adversarial tests (committed, branch 504a8a7)

rust/crates/kuna-harness/tests/verify_w10_indproto_e2e.rs (3 e2e tests over the real datatest engine):
- `w10_indproto_real_file_decodes_and_first_assertion_passes` (T1, regression fence): real indproto.xml decodes, all 3 assertions applied, `Indirect prototype #1` Success.
- `w10_indproto_varargs_nested_fnptr_decodes` (T2): `void (*peek)(int4 a, ...)` — pins the `first_var_arg_slot:-1` hardcode (dead C++ branch).
- `w10_indproto_multiparam_nonvoid_return_fnptr_decodes` (T3): `int4 (*get)(int4 a, char *b)` — exercises getInTypes over 2 params + output-type assignment + the activated compare/compareDependency recursion.
ALL THREE FAIL on parent cc925d3 (un-ported seam) and PASS on this branch — genuine discriminating fences (verified by running both branches against identical built specs).

## Findings

- F1 (LOW / process): the porter committed ZERO adversarial tests for this item (diff touches only 3 src files). Protocol asks >=3. The verifier added 3 (above). Not blocking.
- F2 (informational, not a defect): `Indirect prototype #2`/`#3` (call-argument recovery) do not pass — downstream surface, recorded as LOSS-195. The transcription itself is complete and faithful; no edit to this item is needed when the call-arg pipeline lands.

## losses

LOSS-195 (docs/rust-port/losses.md) — nested fn-pointer type DECODES (exec-failure closed, +1 = `Indirect prototype #1`), but indirect-call argument recovery (`#2`/`#3`) is deferred to the call-arg pipeline wave. Faithful PARTIAL, zero regression.
