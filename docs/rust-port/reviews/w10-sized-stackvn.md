# w10-sized-stackvn
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (subagent verifier)
date: 2026-06-14
gate: cargo test --workspace -> EXIT 0, 3250 passed / 0 failed / 36 ignored (3247 base + 3 verifier); cargo clippy -p kuna-decomp --lib -> clean; C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json --engine cpp` -> 207/207 unit + 675/675 datatests, PARITY OK (untouched)

## Scope of the diff (`git diff rust-port...rport/w10-sized-stackvn -- rust/`)

Rust-only, 6 files, +205/-22. The diff does NOT touch ruleaction (no
`RuleStoreVarnode`/`RuleLoadVarnode` port) despite the branch name. What it
actually delivers:
- `Database::clear_unlocked_category_negative` (database.rs) — the C++
  `ScopeInternal::clearUnlockedCategory(-1)` `cat<0` arm (database.cc:2120-2136).
- `ScopeLocal::clear_unlocked_category_negative` wrapper (varmap.rs) + its call
  as the FIRST statement of `Funcdata::restructure_varnode`
  (funcdata_spacebase.rs:237-246) — always-on, matching varmap.cc:1259.
- `ScopeLocal::resolve_default_name` (varmap.rs) — the `ActionNameVars` namerec
  `vN` rename (coreaction.cc:3063+3087-3094), wired into `name_local_highs_angr`
  (coreaction_cleanup.rs).
- coreaction_protos.rs: comment rewrite only; the `reset_local_window` call stays
  env-gated behind `KUNA_RESET_LOCAL_WINDOW` (default OFF).

## Hunt list

- **Signedness**: clean. `get_category() < 0` is a signed `int4` compare matching
  C++ `sym->getCategory() >= 0` (database.cc:2125). `sym_off` in
  `resolve_default_name` reuses the already-reviewed `name_for_varnode` cast
  pattern verbatim.
- **Integer widths**: clean. `int4`/`uintb` mappings match (`get_offset()` is
  `uintb`, `base`/`sym_off` are `int4`).
- **Wrapping**: clean. The one arithmetic site
  (`addr.get_offset().wrapping_sub(entry_addr_off) as int4).wrapping_add(entry_off)`)
  is byte-identical to the established `name_for_varnode` line; `*base += 1` is a
  bounded small-int increment.
- **Comparator totality**: n/a (no new comparators; `nametree` is a name-ordered
  `BTreeMap`).
- **Iteration-order provenance**: clean. `clear_unlocked_category_negative`
  iterates `nametree.values()` — a `BTreeMap<NameKey,SymbolId>` in name-key order,
  matching C++ `SymbolNameTree` (`std::set<Symbol*, SymbolCompareName>`) name
  order. No `HashMap`/`HashSet`. The outcome is order-independent (each symbol is
  independently removed or renamed; no inter-symbol dependency).
- **Off-by-one / do-while / reverse**: n/a (no do-while; the gate compares are
  `== 0`/`== entry_size`).
- **Erase-while-iterating**: clean and the key fidelity point — the C++ does
  `Symbol *sym = *iter++;` then mutates the tree; the Rust collects the cat<0 ids
  first (`.collect()`) then acts. Verifier test
  `..._removes_all_in_one_pass` proves every unlocked cat<0 symbol is removed in a
  single pass (no skip from mid-walk mutation) with an interleaved type-locked
  keeper.
- **Exception -> Result partial-state**: F1 finding — the `let _ = ...rename_symbol`
  in `resolve_default_name` and `let _ = lm.clear_unlocked_category_negative()`
  swallow `KunaResult`. The C++ `clearUnlockedCategory` cannot throw on this path;
  the rename error is only reachable for a name collision, see F1.

## Mechanical pass

- `cargo run -p port-audit` — NOT RUN: no `port-audit` crate exists in this tree
  (the protocol references it but it is absent here).
- grep added production lines for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable` — none. One bare `as int4` cast (a verbatim copy of the
  reviewed `name_for_varnode` line) lacks a `// cast:` comment; F3 (cosmetic).
- `cargo clippy -p kuna-decomp --lib -- ` — clean. (`--tests` surfaces ONE
  pre-existing `overly_complex_bool_expr` at heritage.rs:3122, identical on the
  rust-port base, untouched by this diff and by the verifier additions.)

## No special-casing

Grep of all added production (non-comment) lines: no function name, address, or
size literal. The only literals are test-fixture addresses (0x100/0x200/0x300)
and the genuine whole-symbol gate `sym_off == 0 && size == entry_size` (the
faithful translation of C++ `getSymbolOffset() < 0`). PASS.

## Faithfulness findings

- F1 (low): `resolve_default_name` omits `makeNameUnique` and discards the rename
  `KunaResult`. C++ `buildDefaultName` ALWAYS routes the `vN` through
  `makeNameUnique` (database.cc:1784); the Rust does `format!("v{base}")` +
  `let _ = rename_symbol(...)`. The in-code comment even says "makeNameUnique then
  renameSymbol" but neither is honored. A second undefined-named whole-symbol
  stack local could collide on `vN` and silently keep its `$$undef...` name.
  Unreachable today (the typed-stack chain is dormant; `base` is fresh per
  function and these are the only `vN` producers), so not a blocker.
       cpp: decompiler/cpp/database.cc:1784
       rust: rust/crates/kuna-decomp/src/varmap.rs:1216
- F2 (low): the whole-symbol gate uses the SymbolEntry size (`size == entry_size`)
  where C++ `HighVariable::setSymbol` uses the symbol TYPE size and an explicit
  `!entry->isPiece()` (variable.cc:265-269). Equivalent for a non-piece whole
  entry; can differ for a piece. Unreachable today.
       cpp: decompiler/cpp/variable.cc:265
       rust: rust/crates/kuna-decomp/src/varmap.rs:1214
- F3 (cosmetic): bare `as int4` cast without `// cast:` (verbatim copy of the
  reviewed `name_for_varnode` pattern).
       rust: rust/crates/kuna-decomp/src/varmap.rs:1208

## The load-bearing CLAIM is unverifiable (LOSS-157)

The branch name and the in-diff comment assert the sized-stack-Varnode seam is
"CLOSED" and that "a scalar stack local now types as `int4` (NOT a spurious
`xunknown1 [N]` array — verified on condconst_conn)", fixing the LOSS-156
condconst #10 regression. This could NOT be reproduced:

- Driving condconst_conn / condconst1 / condconst_copy end-to-end through the Rust
  pipeline with `KUNA_RESET_LOCAL_WINDOW` ON vs OFF gives BYTE-IDENTICAL output:
  stack STOREs are still unfolded (`CALLOTHER(0,DS,...)`/`*Unique...`), there is
  NO `int4` stack local (only the AX-register `v1`/`v2`), no `$$undef`, no `[4]`
  array. The env var changes nothing.
- The SAME is true on the rust-port BASE (ad76d3a) with the env ON — so the
  `xunknown1 $$undef00000002 [4]` array LOSS-156 said the env-on path produces is
  NOT observed there either, and this `clearUnlockedCategory` has no regression to
  fix in this pipeline.
- condconst datatest #10 (`v1 = x;`) PASSES on both base and branch regardless of
  the env var (satisfied via register recovery, not a typed stack local). This
  item neither created nor restored it.
- Full Rust datatest tally is byte-identical base==branch: 47/456 assertions, 28
  execution-failure files (no regression, no fabricated gain, exec-fails flat).

Cause: the sized stack Varnode the whole chain depends on is produced by
`RuleStoreVarnode`/`RuleLoadVarnode` converting the spacebase STORE/LOAD into a
sized stack COPY — and that rule-layer conversion is NOT ported (despite the
branch name). So `gatherVarnodes` never sees a sized written stack Varnode on
pass 1; the typed RangeHint never forms; the open-array it would override never
forms; `resetLocalWindow`/`clearUnlockedCategory` have nothing to act on. The
ports are faithful upstream behavior but inert here.

## Why ACCEPT-WITH-LOSSES (not REJECT)

REJECT criteria are: special-cased / unfaithful / weakened, regresses condconst,
or boolless/readstruct break. None hold:
- `clearUnlockedCategory(-1)` and `resolve_default_name` are FAITHFUL to the C++
  (database.cc:2120-2136, coreaction.cc:3087-3094) and grep-clean of special
  casing.
- condconst is NOT regressed (byte-identical base==branch, env on/off; 4/12 both;
  #10 passes both).
- boolless full-byte-parity (`w10_boolless_full_byte_parity_modulo_type_inference`)
  PASSES; readstruct param-naming (`verify_w10_struct_corpus`) PASSES.
- execution-failure count flat at 28; `cargo test --workspace` all green; C++
  oracle 675/675 + 207/207 PARITY OK untouched.

The single loss is that the item's headline improvement is dormant/unverifiable
until the upstream RuleLoad/StoreVarnode spacebase-conversion seam lands; the
two ports are net-neutral and harmless, and `clearUnlockedCategory(-1)` is
legitimate always-on upstream behavior the pipeline will need.

## Adversarial tests (committed on the branch, 5f38983..79407f3)

- rust/crates/kuna-decomp/src/database.rs::tests::verify_w10_sized_stackvn_clear_unlocked_typelocked_name_handling
- rust/crates/kuna-decomp/src/database.rs::tests::verify_w10_sized_stackvn_clear_unlocked_removes_all_in_one_pass
- rust/crates/kuna-decomp/src/varmap/tests.rs::tests::verify_w10_sized_stackvn_resolve_default_name_whole_symbol_gate

losses: LOSS-157 (this item's unverifiable-improvement + F1/F2 deviations);
refines LOSS-156 (the env-gated dormant `resetLocalWindow` enabler).
