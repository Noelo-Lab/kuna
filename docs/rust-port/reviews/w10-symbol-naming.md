# w10-symbol-naming
verdict: ACCEPT
verifier: claude-opus-4-8[1m] (independent verifier, round 1)
date: 2026-06-14
item: rport/w10-symbol-naming (bind recovered/locked Symbols to body HighVariables: ActionNameVars::linkSymbols walk + Funcdata::linkSymbol query + buildDefaultName aN/dat_/vN scheme + ProtoStoreSymbol::setInput param-symbol materialization + emitScopeVarDecls no_category param skip)

## Gate
- Item gate `cd /tmp/kuna-nam/rust && cargo test -p kuna-decomp` and full `cargo test --workspace` -> **143 ok test-result blocks, 0 FAILED, exit 0** (includes the 4 verifier adversarial tests).
- `cargo build --workspace` -> clean. `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean; `cargo clippy -p kuna-decomp --test verify_w10_struct_corpus -- -D warnings` (the only test file this item touches) -> clean.
  - NOTE: `cargo clippy -p kuna-decomp --tests -- -D warnings` reports ~19 errors, but ALL are in untouched test files (verify_w4_fw_architecture_r2.rs, verify_w6_s4_coreaction_protos.rs, verify_w3_ir_*.rs, printc/tests.rs, universalaction/tests.rs, …) — `git diff rust-port...rport/w10-symbol-naming` touches NONE of them. Pre-existing clippy-version drift (doc-list-indentation / const-is-empty lints), not introduced by this item. The lib and the one touched test file are both clean.
- C++ oracle UNTOUCHED: the diff touches ZERO files under `decompiler/` or `specs/` (only `rust/`). `python -m kuna.run_tests --all --baseline docs/baseline.json` -> **207/207 unit, 675/675 datatest, PARITY OK**.

## FOCUS 1 — FAITHFULNESS (transcription, not a shortcut): VERIFIED
The naming is a faithful transcription of the kuna-default (angr) naming scheme,
exactly the C++ `Scope::buildDefaultName` `kunaAngrNaming` branch
(`database.cc:1761-1785`):
- `aN` for `function_parameter` (`kunaArgName(catindex)`), `dat_<addr>` for a
  persistent non-register global (`kunaGlobalDataName`), `vN` (base seeded at 1,
  `++`) for every other in-scope local. The Rust `kuna_arg_name`/
  `kuna_global_data_name` and the inline `format!("v{base}")` reproduce this
  byte-for-byte. (The task's `local_<off>/iVar<n>` is the upstream-Ghidra scheme;
  kuna's default is the angr scheme per DIV-2 — correctly targeted.)
- The walk in `name_local_highs_angr` (coreaction_cleanup.rs:965) is the faithful
  `ActionNameVars::linkSymbols` (coreaction.cc:3028) location-order walk:
  `iter_loc()` == `beginLoc..endLoc`; `is_free()` skip (cpp 3050); name-rep dedup
  via `high_name_representative` == `getNameRepresentative()` (cpp 3056); the
  `!high->hasName()` gate (cpp 3058) via `high_has_name` -> faithful
  `HighVariable::has_name` (variable.rs:787 == variable.cc, incl. the unaffected-
  stackpointer / non-coverable / implied exclusions); the symbol query
  `name_for_varnode`/`category_for_varnode` == `Funcdata::linkSymbol` ->
  `queryProperties`/`findOverlap` (funcdata_varnode.cc:1177); the symbol-name-
  when-present rule (bind the high to the covering Symbol's display name +
  in-symbol byte offset) vs the `vN` fall-through for the in-scope, address-tied,
  non-input, non-persist local — all match.
- `Funcdata::link_proto_params` (funcdata_varnode.rs:148) + `ScopeLocal::
  add_param_symbol` (varmap.rs:966) faithfully reconstruct `ProtoStoreSymbol::
  setInput`'s `scope->addSymbol(nm,type,addr) + setCategory(sym,function_parameter,
  i)` (fspec.cc:3174) — the symbol-creation step that the kuna `ProtoStoreInternal`
  (the `FuncProto::setScope` seam) skips. The `find_overlap`-then-skip idempotency
  guard mirrors the `entry==0` arm of `setInput`/`linkSymbol`. `Database::
  set_category` (database.rs:1486) is a line-by-line port of `ScopeInternal::
  setCategory` (database.cc:2854) incl. the cat==0 honored-index / cat>0 append
  distinction and the trailing-None pop cleanup.
- `PrintC::emit_scope_var_decls` param skip (printc.rs:1692) faithfully ports
  `emitLocalVarDecls -> emitScopeVarDecls(scope, no_category)` (printc.cc:2336/
  2667): a high covering a `function_parameter` Symbol is excluded from the body
  decl block, so parameters render in the signature, not the body.

## FOCUS 2 — NO SPECIAL-CASING: VERIFIED (clean)
Grepped every `+` line of every source file for function-name / address /
symbol-name hardcoding (`readstruct|divopt|divu|main`, `0x4xxxx`, `== "…"`,
`contains("…")`, `starts_with`, address literals). ZERO hits in any of the six
production source files. The only two `contains("…")` hits in the diff are in
TEST assertions (`rendered.contains("divu")` / `.contains("readstruct")`),
checking that decompilation happened — not in the port logic. The naming is
parameterized entirely by `i` (category index), `base` (a running counter), and
the recovered Symbol's address/type. No special-casing of any kind.

## FOCUS 3 — THE BODY NAMES ARE REAL (oracle parity): VERIFIED
Drove the C++ oracle (`decomp_dbg`, kuna default = angr naming) on an x86-64
function with `(int *ptr, long a, long b)` and the unlocked recovery path the
Rust `render_corpus` exercises:
- C++ oracle:  `readstruct(int8 a0,int8 a1,int8 a2) { return *(...)(a0 + (a1 + 2 + a2) * 4); }`
- Rust render:  `readstruct(int8,int8,int8) { v2 = ZEXT(LOAD(3,a0 + (a1 + 2 + a2) * 4)); v1 = v2 & 0xff…; return v1; }`
Identical recovered-parameter names `a0/a1/a2` (the `kunaArgName` scheme) bound
INTO the body address expression `a0 + (a1 + 2 + a2) * 4`; locals get the `vN`
scheme. (The LOAD-vs-`*(...)` and ZEXT/mask altitude is the printc-body item's
concern, not this one's — the NAMING is exact parity.) C++ datatest corpus
remains 675/675 (the LOCKED `nestedoffset` path, oracle `return ptr->array[b+a];`,
is unaffected). The names come from the recovered Symbols + the C++ default-name
scheme — REAL, not fabricated.

## FOCUS 4 — NEW POSITIVE DATATEST PASSES ARE REAL PARITY: VERIFIED
The render is the REAL body (no W9-emit stub `WARNING: body emission` marker —
confirmed by the `render_is_deterministic` adversarial test). The porter's
modified divopt assertions are a sound, NON-weakening parity move: the old
`STORE(...,RDI` / `sixtyfour >= 1` matches now necessarily FAIL because the body
binds RDI to its parameter; the replacements (`STORE` through a bound single-token
name, FORBID raw `RDI`, FORBID `$$undef`, plus the wide-mask 64-bit discriminator)
are STRICTLY STRONGER. Proven by reverting only the six src files to base and
re-running my tests: 3 of 4 param-binding tests FAIL on rust-port (body shows raw
`RDI`, `a0` re-declared) and PASS on this branch.

## Hunt list (each explicitly checked)
- **Signedness**: clean. `num_params()->int4`/`get_param(i:int4)`, loop `0..num`
  (negative => empty range, no panic) == C++ `for(int4 i=0;i<numParams();++i)`.
  `set_category` `old_cat>=0`/`cat<0`/`cat>0` guards match the C++ `int2`/`int4`
  sign behavior; `Symbol.category` is `int4` (pre-existing widening of C++ `int2`,
  value-preserving for the small category enum).
- **Integer widths**: clean. `catindex` is `u16` (== C++ `uint2`); `ind as u16`
  faithfully reproduces C++ `sym->catindex = ind` (int4->uint2) truncation. No
  promotion-into-wider-type hazards in the new arithmetic.
- **Wrapping**: clean. The only arithmetic is `name_for_varnode`'s `sym_off`
  (`wrapping_sub`/`wrapping_add` on `uintb`-lineage offsets — pre-existing,
  correct for the C++ `uintb` subtraction) and `base += 1` (counter, cannot wrap
  in practice). No bare arithmetic on `uintb`-lineage values added.
- **Comparator totality**: N/A — no comparator ported. `decls.sort_by(a.1.cmp(b.1))`
  is pre-existing (string `Ord`, total) and untouched by this item.
- **Iteration-order provenance, per loop**: clean. The naming walk uses
  `iter_loc()` (BTreeMap loc_tree == C++ `VarnodeLocSet` order) with a `BTreeSet`
  `seen` dedup — deterministic, matches C++. `link_proto_params` iterates `0..num`
  (proto-store insertion order == parameter slot order). NO HashMap/HashSet in the
  ported path (the only two `seen` sets in scope are `BTreeSet`/`BTreeSet`). The
  `vN` `base` counter increments in location order, reproducing C++'s `namerec`-
  order numbering (C++ folds the namerec collection + buildDefaultName loop; the
  port assigns `vN` inline during the same-order walk — equivalent numbering).
  Constant-space varnodes (which C++ handles in a separate pass and never routes
  to `vN`) are correctly excluded by the `has_name()` (constants lack cover) and
  `!addr_tied` gates before reaching the `vN` arm.
- **Off-by-one / do-while / reverse iterators**: clean. `vN` base seeded at 1 (==
  cpp `int4 base = 1`); my adversarial test pins "no `v0`, `v1` present". No
  do-while / reverse-iterator idioms in scope. `set_category`'s trailing-None pop
  (`while matches!(list.last(), Some(None))`) correctly no-ops on an empty list
  (== cpp `while(!list.empty() && back()==0)`).
- **Erase-while-iterating**: N/A — no erase-during-traversal; the walk collects
  `vlist` up front, then mutates the high bank (disjoint from the iterated vbank).
- **Exception -> Result partial-state parity**: minor, inert. `high_has_name`
  maps C++ `HighVariable::hasName`'s `LowlevelError` (non-coverable/implied
  varnode merged with others) to a conservative `false` (skip the high) where C++
  would propagate the throw out of `ActionNameVars::apply`. Only reachable on
  genuinely malformed IR (a >1-instance high containing a non-coverable/implied
  member), which does not arise on valid SSA; observationally inert on the corpus
  (675/675 + workspace green). Same conservative pattern used elsewhere in the
  port. Not blocking.

## Mechanical pass
- Grep of the diff's `+` src lines: NO `todo!`/`unimplemented!`/`HashMap`/
  `HashSet`/`sort_unstable`/`partial_cmp`. Two `.unwrap()` (funcdata.rs:208,
  printc.rs:1705) are both guarded by a preceding `is_none()`/`n==0` check — safe.
- Bare `as` casts: the seven casts in `Database::set_category` (`old_cat as
  usize`, `catindex as usize`, `ind as u16`, `cat as usize`, `(cat as int4)`,
  `.len() as u16`) lack `// cast:` justification comments. All are value-
  preserving after their guards (and `ind as u16` faithfully mirrors the C++
  int4->uint2 truncation). Documentation-level protocol nit, NOT a correctness
  finding — recorded below, does not affect the verdict.

## Adversarial tests (4, committed to the branch at c6404d0, in `tests/verify_w10_struct_corpus.rs`)
- `verify_w10_symbol_naming_render_is_deterministic` — render twice byte-identical
  (guards against a HashSet/HashMap regression in the naming walk) + asserts the
  body is real, not the W9-emit stub.
- `verify_w10_symbol_naming_local_vn_counter_is_sequential_from_v1` — no `v0`,
  `v1` present (pins the `base=1` seed and `base++` numbering).
- `verify_w10_symbol_naming_divopt_binds_param_no_raw_reg_no_undef` — SECOND
  corpus: no raw `RDI`, no `$$undef` (guards against a nestedoffset-specific
  accident).
- `verify_w10_symbol_naming_param_not_redeclared_as_local` — a recovered param
  renders in the signature, never re-declared as a body local.
Discriminating power confirmed: reverting the six src files to rust-port makes 3
of these FAIL (raw `RDI` in body, `a0` re-declared); restoring makes all 4 pass.

## Findings
- F1 (nit, non-blocking): the seven bare `as` casts in `Database::set_category`
  (database.rs:1486-1517) carry no `// cast:` justification comment per the
  verifier protocol's mechanical pass. All are value-preserving / faithful-to-C++-
  truncation; no behavior change. Recommend adding the comments on a later touch.
       cpp: decompiler/cpp/database.cc:2854 (ScopeInternal::setCategory)
       rust: rust/crates/kuna-decomp/src/database.rs:1486

## Losses
None required. No deliberate departure from the C++ oracle on the realized path —
the param-symbol materialization is a faithful reconstruction of the C++
`ProtoStoreSymbol::setInput` behavior that the W4 `setScope` seam (LOSS-036)
deferred; this item closes that naming-side sliver rather than introducing a new
divergence. The `hasName` exception->false degradation is observationally inert
on valid IR and matches the port's established conservative pattern. ACCEPT.
