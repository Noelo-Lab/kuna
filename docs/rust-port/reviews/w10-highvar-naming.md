# w10-highvar-naming
verdict: ACCEPT-WITH-LOSSES
verifier: Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
gate: cd rust && cargo test -p kuna-decomp -> ok (2585/0; 2582 base + 3 verifier adversarial); cargo test --workspace -> 0 failed; cargo clippy -p kuna-decomp --lib -> clean; C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatests, PARITY OK (rust-only diff; no C++ edited)

## Scope of the diff
3 src files + 3 test files (`git diff rust-port...rport/w10-highvar-naming`):
- `coreaction_cleanup.rs`: (a) live the `high->numInstances()>1 -> explicit`
  check in `base_explicit` (was seamed as a no-op); (b) replace the `name_local_highs_angr`
  `dat_`/`vN` gate (`v_input || v_persist || !v_addrtied`) with the faithful single
  `dat_` exclusion (`v_persist || is_global_data`).
- `printc.rs`: switch the body-decl param-skip discriminator from
  `category_for_varnode` (findOverlap) to `containing_category_for_varnode` (findContainer).
- `varmap.rs`: add `containing_category_for_varnode` (C++ `Scope::findContainer`).
- test files: 2 bootstrap-only additions (`install_register_lookup`, no assertion
  change) + 2 regex broadenings in struct-corpus (add `// r..` storage-comment clause).

## hunt list
- Signedness: clean. `num_instances() > 1` compares two `int4`s (faithful to C++
  `int4 numInstances()`); no signed/unsigned mix introduced.
- Integer widths: clean. `int4`/`uintb` mappings preserved; `format!("v{base}")` on `int4 base`.
- Wrapping: clean. No arithmetic added in src beyond `base += 1` (bounded by the
  finite high count); the `resolve_default_name` `wrapping_sub`/`wrapping_add` are
  pre-existing (W7), not in this diff.
- Comparator totality: clean. Naming order is driven by the pre-existing `LocKey::cmp`
  total order (varnode.rs:251, faithful transcription of `VarnodeCompareLocDef`);
  this diff adds no comparator.
- Iteration-order provenance: clean. `name_local_highs_angr`/`emitLocalVarDecls`
  walk `vbank().iter_loc()` = `loc_tree.values()` (BTree in `LocKey` order) =
  C++ `beginLoc(spc)..endLoc(spc)`. Per-high dedup via a `BTreeSet<HighVariableId>`
  (deterministic). NO HashMap/HashSet in the changed src.
- Off-by-one / do-while / reverse iterators: clean. `base` seeds at 1 (C++
  `apply()`); `dat_`/persist highs `continue` BEFORE `base += 1` (matches C++
  `buildDefaultName` returning in the `dat_` arm without `base++`). Verified gapless
  v1.. via the new numbering test.
- Erase-while-iterating: n/a (no container erase in the diff).
- Exception->Result partial-state: clean. The one `expect("named high rep has no
  space")` is on a name representative that by construction has a storage space; no
  `?`-propagation added.

## Faithfulness
- `numInstances()>1 -> -1` (explicit): byte-faithful to coreaction.cc:3119;
  `num_instances()`/`get_instance` are the verbatim `inst.size()`/`inst[i]` (variable.cc).
- `vN` numbering ORDER: faithful by construction — the location-set BTree walk +
  the single shared `base` counter advancing across BOTH the `resolve_default_name`
  whole-symbol-rename path and the explicit `format!("v{base}")` path reproduces the
  C++ namerec-in-loc-order numbering (`lookForFuncParamNames` takes namerec const and
  does NOT reorder/append; `recoverNameRecommendationsForSymbols` operates on symbols,
  not namerec). Multi-var order spot-checked: loopcomment renders `v1//eax, v2//rax,
  v3//rsp, v4, v5//stack` in cross-space loc order; readstruct renders exactly one
  `v1//rax`. The canonical multi-var oracle reference (floatcast `v1=(float8)a; v2=
  (float8)b; ...`) is NOT rendered end-to-end in Rust yet (float cluster blocked,
  LOSS-185) so the order match there is by the structural argument, not a live diff.
- `containing_category_for_varnode`: faithful port of `Scope::findContainer`
  (database.rs:2128); the param-skip now uses containment (a wider local merged onto
  a register that also holds a narrower param is NOT contained -> still declared),
  which matches the C++ `emitScopeVarDecls(no_category)` Symbol-by-category semantics.

## No special-casing
Mechanical grep of the added src lines for register-name / function-name / address
hardcoding (`"rax"|"rsp"|...|"main"|"readstruct"|getOffset()==0x..|name=="..."`):
NONE. The register/global discrimination goes through `manage->getRegisterName(...)`
(data-driven), never a literal. PASS.

## findings
- F1 (medium, accepted as LOSS-187): the `dat_<addr>` route substitutes an
  `IPTR_PROCESSOR && !is_register` global-data proxy for the C++ `(nflags & persist)`
  flag (the W4 global-scope `queryProperties` persist-painting is seamed). Faithful
  for registers (excluded -> `vN`), uniques/stack (not IPTR_PROCESSOR -> `vN`), and
  persistent global RAM (-> `dat_`). The only gap is a NON-persistent global-RAM
  local (C++ `vN`, proxy `dat_`), a degenerate case absent from the corpus — ZERO
  observable divergence (suite green, oracle untouched). Documented restoration path.
       cpp: decompiler/cpp/database.cc:1778-1782
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:1181-1184
- F2 (info, no action): the two struct-corpus regex broadenings (accept `// r..`
  storage comment in addition to the raw `RSP` token) are NECESSARY adaptations, not
  weakenings — verified that with the new naming the raw 64-bit register token is GONE
  from the body (`raw_tok=0`, `cmt_tok=2` on loopcomment), so the OLD regex would now
  FAIL; the realmode-rejection half of each test is unchanged, preserving its
  discriminating power.

## No regression (CRITICAL — naming touches every body)
- boolless: `print_b5_boolless` 11/11 green (incl. `w10_boolless_full_byte_parity_modulo_type_inference`,
  `w10_boolless_if_body_contains_assignment`); v1 naming unchanged.
- readstruct (nestedoffset): `verify_w10_struct_corpus` byte/binding tests green;
  single `v1` decl, gapless.
- condconst: `w10_naming_conditional_other_function_gets_no_vn_name` green
  (pre-existing test, not new in this branch — it predates the diff in rust-port).
- whole suite: kuna-decomp 2585/0 (base 2582 + my 3); the diff ADDS NO new test
  functions to the base set and DELETES NO assertion (only a bootstrap line + an
  OR-clause in 2 regexes), so the passing SET is a strict superset — no silent swap.
  cargo test --workspace: 0 failed. C++ oracle 675/675 PARITY OK, 207/207 unit,
  untouched (rust-only diff, `git diff` over `decompiler/cpp/` empty for this branch).

## adversarial tests (committed to the branch, 3ba22d7, in verify_w10_struct_corpus.rs)
- verify_w10_hvnaming_register_local_gets_vn_not_dat — a register-backed coalesced
  local takes the `vN` arm with a `// r..` comment; no register rendered as `dat_`
  (pins the F1 persist-proxy boundary, the keystone of this diff).
- verify_w10_hvnaming_vn_indices_are_gapless_and_unique_from_v1 — the declared `vN`
  indices form a gapless, duplicate-free 1.. run within a single function (pins the
  shared-base-counter numbering + per-high dedup; this test caught a multi-function
  index-repeat nuance during development, confirming teeth).
- verify_w10_hvnaming_two_renders_are_byte_identical — naming is deterministic across
  runs (pins the total-order loc walk vs a HashMap).

## losses
LOSS-187 (this review, appended) — the `dat_`/`vN` persist-proxy. F1 above is the
only accepted divergence; it is the cited losses.md entry per protocol.
