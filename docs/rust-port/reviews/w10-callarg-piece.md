# w10-callarg-piece

verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (Opus 4.8 [1m], round 2)
date: 2026-06-17
branch: rport/w10-callarg-piece @ 0d08349 (two commits over merge-base 54c0afb = rust-port HEAD)

## Decision

ACCEPT-WITH-LOSSES. The Round-1 REJECT blocker (the change edited the reserved file
`coreaction_protos.rs`) is **RESOLVED**: commit 0d08349 ("round-2 repair") relocates the
callee-proto copy OUT of `coreaction_protos.rs` and into `flow.rs::build_call_specs`. All
eight reserved do-not-edit files are now clean. The change is faithful, not special-cased,
passes real oracle parity on the targeted field-writes, and carries ZERO scored regression
(independently confirmed: full-suite regressed set EMPTY). It ships render-churn LOSSES on
three failing datatests (one disclosed+pinned, two undisclosed) — recorded below and in
losses.md — none of which is a scored regression. No REJECT trigger is met.

## Scope (full changed-file set)

`git diff rust-port...rport/w10-callarg-piece` = exactly 6 files, Rust-only (+583/-1):
  - rust/crates/kuna-console/src/ifacedecomp.rs                            (+59/-1)  — `apply_prototype_to_symbol` (C++ `Architecture::setPrototype`), wired into `parse line extern`
  - rust/crates/kuna-decomp/src/dtype.rs                                   (+11)     — `Datatype::get_code_prototype` (`TypeCode::getPrototype` accessor)
  - rust/crates/kuna-decomp/src/flow.rs                                    (+64)     — RELOCATED home of the `ActionDefaultParams::apply` copy arm (the round-2 repair)
  - rust/crates/kuna-decomp/src/seams.rs                                   (+33)     — `Architecture::query_callee_proto` (address-keyed global-query)
  - rust/crates/kuna-decomp/tests/verify_w10_callarg_piece.rs             (+308, new) — 4 adversarial tests (AT1-AT4)
  - rust/crates/kuna-harness/tests/verify_w10_callarg_piece_switchhide_guard.rs (+109, new) — switchhide render-delta disclosure pin

No C++ / spec / python / baseline byte touched (verified: `git diff --name-only` has zero
non-rust entries) -> the recorded C++ oracle 675/675 PARITY OK is structurally untouched
(independently re-confirmed: C++ `decomp_test_dbg` full suite reports 675 Success). B0
unchanged (verified: `universalaction_listing` 1/0; the relocation adds NO schedule node —
the only `ActionDefaultParams` mentions in the diff are comments).

## Reserved-file check (Round-1 blocker resolution)

CLEAN. Each of database.rs, funcdata.rs, coreaction_render.rs, coreaction_infertypes.rs,
**coreaction_protos.rs** (the R1 violation), coreaction_cleanup.rs, printc.rs, dynamic.rs is
ABSENT from the changed-file list. The load-bearing copy now lives in `flow.rs`
(`build_call_specs`, flow.rs:1806-1865) — NOT a reserved file. The charter's stated premise
("edits ONLY subflow.rs/funcdata_varnode.rs") still does not match the implementation (this
item is a callee-Funcdata-copy plumb, not a RulePieceStructure re-port) — but the
reserved-list constraint is independent of that premise and is now satisfied.

## Faithfulness — the relocation is sound (no observable divergence)

The C++ `ActionDefaultParams::apply` (coreaction.cc:2379-2391) does, for every call with
`!fc->hasModel()`: `fc->copy(otherfunc->getFuncProto()); if((!isModelLocked)&&!hasMatchingModel(evalfp)) setModel(evalfp);`
(the `otherfunc != 0` arm), else `setInternal(evalfp, void)`. The Rust now performs the
`copy`+`setModel` arm at flow-construction time (`build_call_specs`), and leaves the UNCHANGED
reserved `ActionDefaultParams::apply` (coreaction_protos.rs:389, the `set_internal` arm) in
place. Verified the interaction is idempotent and correct:
  - When `query_callee_proto` returns Some, the flow-time copy calls `fc.proto.copy(callee)`
    then `set_model(evalfp)` -> the spec now `has_model()==true` -> at `ActionDefaultParams`
    time the `!has_model()` guard is FALSE -> the unchanged action SKIPS it (no double-apply,
    no `set_internal` clobber). Net effect == C++ (`copy` then `setModel`).
  - When `query_callee_proto` returns None (unknown callee — the common datatest case), the
    flow-time copy is a no-op, `has_model()` stays false, and `ActionDefaultParams` applies
    the pristine `set_internal(evalfp, void)` arm exactly as before. Untouched path.
  - Line-faithful predicates: `!fc.proto().is_model_locked() && !fc.proto().has_matching_model(&evalfp)`
    == C++ `(!fc->isModelLocked())&&!fc->hasMatchingModel(evalfp)`. `copy`/`has_matching_model`/
    `set_model`/`is_model_locked`/`has_model` are all pre-existing faithful FuncProto ports
    (fspec.rs:4795/4841/4866/5016/4833), not stubs.
  - `query_callee_proto` (seams.rs) is the faithful
    `getScopeLocal()->getParent()->queryFunction(entry)->getFuncProto()` restricted to the
    global snapshot — keyed on `e.first == start` (exact storage-entry address), borrowing the
    FunctionSymbol's `TypeCode::getPrototype()`. Address-keyed only.
  - `apply_prototype_to_symbol` (ifacedecomp.rs) is the faithful `Architecture::setPrototype`
    (architecture.cc:393): `queryFunction(basename)` -> `getTypeCode(pieces)` -> retype the
    symbol; missing/non-function symbol is a no-op (the kuna stash-pieces path still handles a
    name that is only ever the current decompile target).

INLINE EXCLUSION (`!is_inline_call` guard on the flow-time copy): a reasoned, conservative
addition with NO direct C++ counterpart, and it does NOT introduce an observable divergence.
Rationale verified against C++: an inline call's FuncCallSpecs is deleted by `injectPcode`
(flow.cc:1340-1353 `deleteCallSpec`) BEFORE `ActionDefaultParams` runs — but ONLY when the
inline injection SUCCEEDS. The Rust does the copy at flow time (BEFORE `inject_pcode`,
flow.rs:1585), so it cannot observe whether injection will succeed; excluding inline calls is
the conservative choice that (a) for successfully-injected inlines matches C++ (neither copies)
and (b) for the seamed inject_id path leaves the call to render un-inlined with the default
internal proto from the unchanged `ActionDefaultParams` (which is the pre-existing,
separately-recorded SEAM behavior). The theoretical divergence window — an inline callee that
ALSO has a declared extern proto AND whose injection fails — produces NO corpus delta: the
full-suite regressed set is EMPTY (see No-regression). FAITHFUL-PARTIAL, not a finding.

## Special-casing hunt — CLEAN

Whole-line grep of every NON-comment added production line across flow.rs / seams.rs /
dtype.rs / ifacedecomp.rs for: struct-name (glob2struct/mystruct/foo/fooshort), field-name
(a/b/c/d/A/B), function-name string keying, magic offsets (0x...), value/`== N` keying,
HashMap/HashSet/sort/todo!/unimplemented!, `.name ==`/`name.eq` -> ALL EMPTY. There are ZERO
string literals in the added production code. The only name-keyed construct is
`query_function_by_name(scope, &pieces.name)` — keyed on the USER-DECLARED signature name
(faithful C++ `queryFunction(basename)`), not a corpus identifier. AT2 independently disproves
special-casing: it drives a DIFFERENT struct (`fooshort`) with DIFFERENT field names (a/b/c/d)
and asserts oracle parity — a hardcoded foo/A/B decode could not satisfy it.

## Real parity (independently reproduced vs C++ oracle B5)

Dumped `concat.xml` from the branch `decomp_test_dbg` and the C++ oracle (`-sleighpath` +
`-path datatests`, KUNA_DUMP=1):
  - `concatregparam`: BRANCH render BYTE-IDENTICAL to oracle, incl. the field writes
    `v1.B = regp2; v1.A = regp1;` and `structparam(v1);`. The call-arg PIECE recovery
    produces per-field writes exactly as the oracle.
  - `replaceshort` (the GENERIC different-struct case): statement content byte-identical to
    oracle — `v1.b = replace1; v1.a = globshort.a; v1.c = globshort.c; v1.d = globshort.d;
    globshort.b = replace1; structshort(v1); globshort.d = replace2;`. The ONLY residual is
    the decl-line storage comment (`fooshort v1; // tmp` oracle vs `fooshort v1;` branch) —
    a pre-existing, orthogonal variable-comment-plane gap that AT2 explicitly strips and
    documents. NOT introduced by this item.
  - concat.xml scored: branch 18/18, oracle 18/18 — exact match.

## No-regression — CONFIRMED (independent full-suite diff)

Ran the FULL datatest suite (all 83 files, 675 assertion lines) on BOTH the baseline binary
(main tree @ 54c0afb = rust-port HEAD) and the branch binary, sorted the Success/FAIL lines,
and diffed the passing sets:
  - BASELINE passing = **358** (matches the charter floor exactly).
  - BRANCH passing   = **371** (+13, matches the commit message).
  - REGRESSED SET (baseline-pass -> branch-FAIL) = **EMPTY**. No masked regression.
  - GAINED (+13): Concat #1/#3/#4, Concat split #1, Pointer to array #1/#2/#3/#5/#8/#10,
    Union #10/#11/#29.
  - `cargo test --workspace` (independently re-run): **3638 passed / 0 failed** (zero `[1-9]
    failed`; cargo exit 0), 202 result lines. >> the 358/333 floor.
  - The two new test binaries: `verify_w10_callarg_piece` 4/0 (AT1-AT4 all PASS, AT1+AT2 do
    live oracle comparison), `verify_w10_callarg_piece_switchhide_guard` 1/0.
  - `cargo clippy --lib` clean (zero warnings/errors).
  - B0 `universalaction_listing` 1/0 (action schedule byte-identical to oracle — no node added).
  - C++ oracle full suite reports 675 Success (untouched).

byte-identity (branch-vs-baseline render dumps, KUNA_DUMP):
  - boolless: IDENTICAL. readstruct (function in nestedoffset.xml): IDENTICAL.
    condconst_conn (condconst.xml): IDENTICAL.
  - switch datatests: 8/9 IDENTICAL (ifswitch, switchind, switchloop, switchmulti,
    switchreturn — plus condconst). switchhide DIFFERS (the disclosed, pinned loss).

## Findings (LOSS-class — none is a REJECT trigger)

F1 (LOSS, disclosed+pinned) — `switchhide` render delta. `glob2struct();` ->
  `glob2struct(PTRSUB(v1,0xffffffffffffffe8));`. Caused by THIS item's mechanism
  (`switchhide.xml` runs `parse line extern void glob2struct(mystruct *ptr);`; the callee
  proto is now visible so the call recovers its pointer arg). NOT a scored regression:
  switchhide's 4 assertions FAIL on BOTH baseline and branch (switch structuring un-ported);
  the scored set does not move. Directionally CLOSER to the oracle (`glob2struct(&v1)`), but
  rendered as raw `PTRSUB` (stack-var typing incomplete on Rust side). DISCLOSED and PINNED by
  `verify_w10_callarg_piece_switchhide_guard.rs` (asserts the exact new render + that all 4
  assertions still FAIL). This is the same L1 carried from Round 1.

F2 (LOSS, UNDISCLOSED) — TWO further render deltas of the SAME class, NOT pinned by any guard
  test (the porter pinned only switchhide):
    - `piecestruct`: `print();` -> `print(PTRSUB(v1,0xffffffffffffffe8));` (plus a stack-var
      decl reshuffle). FAILs all 6 assertions on BOTH baseline and branch (0/6 each).
    - `packstructaccess`: `getstruct();` -> `getstruct(i);`. FAILs all 4 on BOTH (0/4 each).
  Both are caused by the same callee-proto-visibility mechanism and both are directionally
  closer to the oracle (the call now recovers an argument). NEITHER is a scored regression
  (the full-suite regressed set is empty; both stems were already 0-passing). The LOSS is that
  these two render churns are NOT disclosed by a pinning test — only switchhide is. Severity:
  un-pinned render churn on already-failing datatests, no scored movement. A re-spin should add
  guard pins for these two (or fold all three into one disclosure) for symmetry with switchhide;
  it is not blocking because there is no masked regression.

## Adversarial tests (5 committed; >= 3 required)

  - AT1 at1_by_value_struct_arg_renders_field_writes_not_inline_concat — PASS. Drives
    `concatregparam`; asserts no inline CONCAT survives, `structparam(v1)`, both field writes,
    AND `assert_eq!` the comment-stripped body against the LIVE C++ oracle.
  - AT2 at2_generic_struct_arg_split_no_name_keying — PASS. The ANTI-special-casing guard:
    DIFFERENT struct `fooshort`, DIFFERENT fields a/b/c/d, asserts all four generic field
    writes + oracle parity. A hardcoded foo/A/B decode fails this.
  - AT3 at3_callarg_split_uses_callee_param_type — PASS. Confirms the callee param type (not
    the function's own output) drives the split; regression-guards the return-side split.
  - AT4 at4_widefloat_callarg_concat_not_built_without_base_explicit — PASS. Negative guard:
    a float10 arg is NOT piece-structured, must not be gathered (base_explicit-v2 fold-in pin).
  - GUARD switchhide_callarg_render_delta_pinned — PASS. Pins F1's exact render + all-4-FAIL.

## Losses (recorded in losses.md)

LOSS-callargpiece-F1: switchhide render delta (disclosed+pinned) — see F1.
LOSS-callargpiece-F2: piecestruct + packstructaccess render deltas (UNDISCLOSED) — see F2.
Both are un-pinned/partially-pinned render churn on already-failing datatests with ZERO scored
movement; restoration is automatic when stack-var typing / RulePieceStructure prerequisites
land and these renders converge fully to the oracle.
