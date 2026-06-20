# w8-s9-coreaction-render
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> PASS (1434 unit + all integration tests, incl. 4 new verifier tests; 0 failed; the only ignored are pre-existing); `cargo clippy -p kuna-decomp -- -D warnings` -> clean

## Scope note (item id vs delivered set)

The checklist title ("coreaction 4/5: casts/naming/types") and notes
("ActionSetCasts..ActionMarkImplied + ActionInferTypes") describe the S9 render
plane.  Those render-plane classes (`ActionSetCasts`, `ActionMarkExplicit`,
`ActionMarkImplied`, `ActionNameVars`, `ActionAssignHigh`, etc.) were **already
ported by the sibling `coreaction_cleanup.rs`** (item `w7-s37-coreaction-cleanup`,
LOSS-114).  Per the verifier prompt's explicit directive — *"coreaction_early/
protos/cleanup.rs module docs record what is already taken — port the REMAINDER"*
— this item correctly delivers the 21 `Action` classes that none of the three
sibling modules had taken.  I verified the partition is exact:

- The union `coreaction_early ∪ coreaction_protos ∪ coreaction_cleanup ∪
  coreaction_render` = the **63 live** `class Action* : public Action` in
  coreaction.hh@bc278439, with **no gap and no overlap**.
- The 3 "missing" classes are exactly the upstream **commented-out** declarations
  (`ActionCse` hh:152, `ActionParamShiftStart` hh:787, `ActionParamShiftStop`
  hh:798) — correctly not ported.
- `ActionPrototypeTypes` (hh:658, declared `Action...: public` with no space) is a
  live class, correctly ported in `coreaction_protos`.

This is a faithful, complete partition; the item-id/title mismatch is a checklist
artifact, not a port defect.

## What is realized vs seamed

This is a SEAM port.  Every `apply` returns 0 with the C++ body carried as
`// SEAM(W8-funcdata)`-noted commented pseudocode (the `Funcdata` analysis/scope/
jumptable/type surface is not in the merged tree).  The **realized** surface — and
the only thing that can diverge today — is the engine-owned class shell:

- exact `name()` string + `flags` from the C++ ctor (all 21 re-derived
  byte-for-byte against coreaction.hh — table below);
- `clone(grouplist)` group filter + parameterization carry-through
  (`stackspace`, `propagateIndirect`);
- the five stateful actions' `reset` overrides (`localcount`/`numpass`/
  `analysis_finished` zeroing) with **C++ non-chaining semantics**;
- `ActionStackPtrFlow::apply`'s two realized early-out branches
  (analysis_finished short-circuit + the "No stack" latch, cc:498-502).

### Ctor-fact re-derivation (all 21 match)

| class | C++ flags | C++ name | C++ reset | rust |
|---|---|---|---|---|
| ActionStackPtrFlow | 0 | stackptrflow | analysis_finished=false | match |
| ActionLaneDivide | rule_onceperfunc | lanedivide | — | match |
| ActionSegmentize | 0 | segmentize | localcount=0 | match |
| ActionForceGoto | 0 | forcegoto | — | match |
| ActionMultiCse | 0 | multicse | — | match |
| ActionShadowVar | 0 | shadowvar | — | match |
| ActionConstantPtr | 0 | constantptr | localcount=0 | match |
| ActionDeindirect | 0 | deindirect | — | match |
| ActionDirectWrite | 0 | directwrite | — (carries propagateIndirect) | match |
| ActionLikelyTrash | 0 | likelytrash | — | match |
| ActionRestructureVarnode | 0 | restructure_varnode | numpass=0 | match |
| ActionMappedLocalSync | 0 | mapped_local_sync | — | match |
| ActionMapGlobals | rule_onceperfunc | mapglobals | — | match |
| ActionDeadCode | 0 | deadcode | — | match |
| ActionConditionalConst | 0 | condconst | — | match |
| ActionSwitchNorm | 0 | switchnorm | — | match |
| ActionUnjustifiedParams | 0 | unjustparams | — | match |
| ActionDynamicMapping | 0 | dynamicmapping | — | match |
| ActionDynamicSymbols | rule_onceperfunc | dynamicsymbols | — | match |
| ActionInternalStorage | rule_onceperfunc | internalstorage | — | match |
| ActionInferTypes | 0 | infertypes | localcount=0 | match |

### `reset` non-chaining (the most fragile faithful-port detail)

C++ `ActionGroup::reset` (action.cc) calls the **virtual** `(*iter)->reset(data)`.
A stateful subclass override is `{ localcount = 0; }` — it does **not** chain to
`Action::reset`, so a stateful leaf's base `status`/`flags` are NOT reset by its
own reset.  The Rust port reproduces this exactly: the trait default `reset` calls
`reset_base()` (status_start + clear warnings_given, mirroring `Action::reset`),
and the five stateful overrides zero only their counter without calling
`reset_base()`.  A naive port would have "defensively" added `self.base_mut().
reset_base()` to the override and diverged — this port got it right.  Pinned by a
new adversarial test (`w8_reset_overrides_do_not_chain_to_base`).

## Mandatory hunt list

- **Signedness** — clean.  Only realized integer is `localcount`/`numpass: i32`
  (C++ `int4`); all comparisons against them live in the unrealized seam bodies.
- **Integer widths** — clean.  `int4 -> i32` mapping correct; `count: int4`
  matches the base.  No promotion-prone arithmetic in the realized surface.
- **Wrapping** — clean.  No live arithmetic; the `count += 1`/`localcount += 1`
  points are all inside commented seam bodies.
- **Comparator totality** — n/a (no comparators in this item).
- **Iteration-order provenance** — n/a for the realized surface; `render_actions`
  is a fixed `vec!` in coreaction.hh definition order (round-trip only, with a
  doc-note that it is NOT a `universalAction` slice).  The seam bodies' loops are
  commented with their C++ iteration order preserved for the future closure.
- **Off-by-one / do-while / reverse iterators** — n/a (seamed); ActionStackPtrFlow
  realized branches re-derived against cc:497-502 — exact.
- **Erase-while-iterating** — n/a (seamed).
- **Exception -> Result partial-state parity** — n/a; no `?`/throw in the realized
  paths.  The C++ `throw` in `ActionSegmentize` ("Segment op in wrong form") is in
  the seam body, correctly carried as a comment.

## Mechanical pass

- grep `coreaction_render.rs`: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`/`panic!`/`.unwrap()`; no bare numeric `as` casts — clean.
- `cargo clippy -p kuna-decomp -- -D warnings` — clean.
- gate `cargo test -p kuna-decomp` — PASS (1434 unit incl. 10 in-module tests + 4
  new verifier tests).

## findings

(none blocking)

- F1 (loss, accepted): all 21 `apply` bodies are seams returning 0 — see LOSS-118.
  cpp: decompiler/cpp/coreaction.cc:496..5630 (the 21 apply bodies)
  rust: rust/crates/kuna-decomp/src/coreaction_render.rs (each `apply` `// SEAM(W8-funcdata)`)
- F2 (observation, within seam contract): `ActionStackPtrFlow` with a stack never
  latches `analysis_finished` (the latch lives in the unrealized `checkClog==0`
  path), so the scheduler re-applies it each pass — but `count` stays 0, so the
  action fixpoint is unaffected.  Covered by LOSS-118.
  cpp: decompiler/cpp/coreaction.cc:504-515
  rust: rust/crates/kuna-decomp/src/coreaction_render.rs:190-194

## adversarial tests

rust/crates/kuna-decomp/tests/verify_w8_s9_coreaction_render.rs (committed to the
branch, 4 tests, all passing):

- `w8_render_ctor_facts_match_cpp_partition` — full (name, flags) partition vs C++
  ctors with the group fed as a non-schedule token (proves no hard-coded group
  literal); isolates exactly the four `rule_onceperfunc` actions.
- `w8_reset_overrides_do_not_chain_to_base` — stateful overrides zero the counter
  but leave base status/flags dirty (non-chaining); a non-overriding leaf
  (`deadcode`) DOES reset its base via the default.
- `w8_seam_totality_under_repetition` — every action stays `count==0` / returns 0
  across 16x repetition, incl. both StackPtrFlow stack (seam, no latch) and
  no-stack (realized latch + short-circuit) paths.
- `w8_clone_filtered_independent_and_parameterized` — group filter honored,
  `stackspace`/`propagateIndirect` carried, clone is an independent object whose
  counter mutation does not leak to the source.

## losses

LOSS-118 (this item, appended to docs/rust-port/losses.md).
