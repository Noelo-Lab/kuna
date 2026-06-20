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

---

# w10-highvar-naming — char/char* follow-up (commit e9e6d11)
verdict: REJECT
verifier: Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
gate: `cd rust && cargo test -p kuna-decomp` -> ok (2591/0: 2585 prior + 6 new
verifier char* adversarial); `cargo test --workspace` -> 3161/0; the Rust datatest
oracle harness (`decomp_test_dbg -path ../decompiler/datatests -sleighpath ../specs
datatests`) -> `Total passing tests = 197` at HEAD vs **194** at base 3d655be;
`cargo clippy -p kuna-decomp --lib` -> clean; C++ oracle (MAIN tree, untouched —
branch makes ZERO `.cc/.hh/spec/docs` edits) `python -m kuna.run_tests --all
--baseline docs/baseline.json` -> 207/207 unit + 675/675 datatests, PARITY OK.

## Scope reviewed
The char* follow-up is exactly the SLESS/SLESSEQUAL split in
`coreaction_infertypes.rs` (`git diff 3ba22d7..e9e6d11 -- rust/`: +22/-5, one file).
The reject is NOT about that diff in isolation (which is FAITHFUL) — it is the
mandated **full passing-SET diff vs base 3d655be**, which the task fixes as the
gate ("REGRESSED must be EMPTY (Promotion #2 must pass)").

## FAITHFULNESS of the char* change — CLEAN
The `CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL` arm is a byte-faithful transcription of
`TypeOpIntSless::propagateType` / `TypeOpIntSlessEqual::propagateType`
(typeop.cc:1035-1041 / 1061-1067):
  `if inslot==-1 || outslot==-1 { return None }` (input<->input only),
  `if alttype.get_metatype() != TYPE_INT { return None }` ("Only propagate signed
  things"), `Some(alttype)`.
The split correctly leaves EQUAL/NOTEQUAL/LESS/LESSEQUAL on `propagate_across_compare`
(typeop.cc:947/1011/1087/1111) and gives SLESS/SLESSEQUAL NO spacebase/relptr arm
(the C++ override does not call `propagateAcrossCompare`). `char` = `TypeBase(1,
TYPE_INT)` (dtype.rs:4430) confirmed, so the char/char* signedness hint is correct:
a `char` flows across a signed compare, a `uint1`/pointer does not. The previous
state (all six comparisons on `propagate_across_compare`) was the genuine bug.

## No special-casing — CLEAN
Mechanical grep of the SLESS region (coreaction_infertypes.rs:540-600) and the
changed test/src files for fn-name/type-name/address hardcoding: NONE. No `todo!`,
`HashMap/HashSet`, bare `as`, or `sort_unstable` in the diff. The metatype gate is
the data-driven `get_metatype() == TYPE_INT`, never a literal.

## hunt list (char* diff)
- Signedness: the entire change. Faithful — the `!= TYPE_INT` gate is the C++
  `getMetatype() != TYPE_INT`; `char`(INT)/`uint1`(UINT)/`ptr`(PTR) discriminated
  by metatype, pinned by 4 of my 6 new tests. CLEAN.
- Integer widths / Wrapping / Comparator / Iteration order / do-while /
  erase-while-iterating / exception->Result: n/a — the arm adds no arithmetic, no
  loop, no container, no `?`. CLEAN.

## char* change is a TRUE no-op on the datatest passing SET
Isolating the char* commit (`3ba22d7..e9e6d11`): the Rust datatest oracle passing
SET is byte-identical (197 -> 197, ZERO lost, ZERO gained). The porter's "datatests
unchanged at 197" claim for this commit is accurate. The char* change neither caused
nor restored any assertion.

## BLOCKER — the passing SET REGRESSES vs base 3d655be (Promotion #2)
Per-assertion diff of the Rust datatest oracle, base 3d655be vs HEAD e9e6d11:
  - REGRESSED (Success@base -> FAIL@HEAD): **`Promotion on compare #2`** (promotecompare.xml:19, `uint1` min=1 max=1).
  - GAINED: Dead Volatile #2, No for-loop iterator used #2, No for-loop iterator used #3, Signed byte #1.
  - Net 194 -> 197.
The REGRESSED set is NOT empty, which the task fixes as a hard REJECT condition
("any assertion regresses vs base 3d655be"). Attribution (intermediate run at
3ba22d7, post-naming/pre-char*): `Promotion on compare #2` already FAILs there, so
the regression was introduced by the **NAMING commit a2276c2** (HighVariable
coalescing: an un-simplified self-`INT_AND(x,x)` whose output types `uint1`
[getOutputLocal UINT, submeta 16<19] wins the HighVariable representative over the
`char`, so `v1` declares a 2nd `uint1`, breaking the `max=1` assertion). The char*
follow-up was the attempt to restore it via type inference and — by the porter's own
commit message and confirmed here — CANNOT (the AND is removed in C++ by
RuleTrivialArith, which is ported [universalaction.rs:399] but never reaches an
active pool for promotecompare; `clone_rule` reports "trivialarith NOT in
grouplist", an action-pipeline/structuring seam, not a char-inference seam).

This is the verifier-caught gap in the round-1 naming verdict above: that verdict's
"no regression" (lines 83-94) checked the cargo-test superset + boolless/readstruct/
condconst, but NEVER ran the `decomp_test_dbg` datatest oracle, which is where the
swap (194->197, with Promotion #2 going dark) is visible. boolless / condconst /
readstruct assertion outcomes ARE byte-identical base vs HEAD (verified) — the
regression is isolated to `Promotion on compare #2`.

## findings (char*)
- F-CP1 (blocking): `Promotion on compare #2` regresses (Success@3d655be ->
  FAIL@e9e6d11). Root cause in the NAMING commit a2276c2; not restorable by the
  char* follow-up. Recorded as LOSS-189.
       cpp: decompiler/cpp/coreaction.cc (ActionNameVars/HighVariable rep selection);
            decompiler/cpp/ruleaction.cc:2358 (RuleTrivialArith `V & V => V`)
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:386-394 (numInstances>1 explicit);
             rust/crates/kuna-decomp/src/universalaction.rs:399 (trivialarith in `analysis`, never cloned for promotecompare)

## adversarial tests (committed to the branch, 165fbff, in
verify_w10_charptr_signedness.rs) — 6, all green
- w10_charptr_slesss_propagates_a_signed_char — char (TYPE_INT,1) flows across INT_SLESS.
- w10_charptr_sless_declines_an_unsigned_byte — uint1 (TYPE_UINT) declines.
- w10_charptr_sless_declines_a_pointer — char* (TYPE_PTR) declines.
- w10_charptr_slessequal_matches_sless_exactly — SLESSEQUAL identical override.
- w10_charptr_sless_declines_when_a_slot_is_the_output — output side (-1) declines.
- w10_charptr_unsigned_int_less_still_propagates_a_uint1 — the contrast: the UNSIGNED
  INT_LESS arm propagates the very uint1 the signed arm rejects (the split has teeth).
(Plus the concrete divergence trace for F-CP1: the base-vs-HEAD `decomp_test_dbg`
datatest passing-SET diff, reproducible above.)

## losses (char*)
LOSS-189 (this review, appended) — `Promotion on compare #2` regression
(194->197 with the swap), root-caused to the naming commit's HighVariable rep
selection + the inactive RuleTrivialArith pool; NOT restorable via char inference.

## verdict rationale
The char* transcription itself is faithful and special-casing-free, and earns no
finding. But the task's gate is the FULL passing-SET vs base 3d655be with Promotion
#2 passing, and that assertion REGRESSES. Per the stated REJECT criterion this is a
REJECT. Sends w10-highvar-naming back to `todo` (attempts++): the work order is to
restore `Promotion on compare #2` — either by activating the `analysis` rule pool /
RuleTrivialArith for promotecompare so the self-`INT_AND(x,x)` is removed (the C++
path), or by fixing the HighVariable representative selection so the `char` wins over
the spurious `uint1`. If the orchestrator deems the +3 net gain acceptable and the
single regression a tracked carry-forward, the alternative is to re-pin the verdict
to ACCEPT-WITH-LOSSES citing LOSS-189 — but that is an owner decision, not the
verifier's, because the task fixed "Promotion #2 must pass" as the bar.

---

# w10-highvar-naming — char/char* follow-up (ROUND 2, re-verification)
verdict: REJECT
verifier: Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
gate (all re-run fresh, MAIN tree for the C++ oracle): `cargo test -p kuna-decomp`
-> 2591/0; `cargo test --workspace` -> 3437/0; `cargo clippy -p kuna-decomp --lib`
-> clean (Finished, 0 warnings); Rust datatest oracle (`decomp_test_dbg -path
../decompiler/datatests -sleighpath ../specs datatests`) -> **197 passing at HEAD
(165fbff) vs 194 at base 3d655be**; C++ oracle `python -m kuna.run_tests --all
--baseline docs/baseline.json` -> 207/207 unit + 675/675 datatests, PARITY OK
(branch makes ZERO `decompiler/`/`specs/`/`docs/` edits — `git diff 3d655be..HEAD`
over those paths is empty; rust-only diff).

## Branch is byte-identical to round 1
`rport/w10-highvar-naming` HEAD is still `165fbff` — NO new commits since the round-1
REJECT. The full passing-SET regression that drove round 1 is therefore unchanged.
This round independently RE-RAN every gate rather than trusting the prior verdict.

## The hard REJECT condition still fires — confirmed by a fresh per-assertion diff
`comm` of base-3d655be vs HEAD `decomp_test_dbg` Success sets:
  - REGRESSED (Success@base -> FAIL@HEAD): **`Promotion on compare #2`** — exactly ONE
    entry, NON-empty. (promotecompare.xml:19, `uint1` min=1 max=1.) Re-verified at the
    assertion level: base prints `Success -- Promotion on compare #2`, HEAD prints
    `FAIL -- Promotion on compare #2`.
  - GAINED (FAIL@base -> Success@HEAD): Dead Volatile #2, No for-loop iterator used #2,
    No for-loop iterator used #3, Signed byte #1 — net 194 -> 197.
The task fixes "REGRESSED must be EMPTY (Promotion #2 restored)" / "REJECT iff ... any
assertion regresses vs base 3d655be (Promotion #2 must pass)". REGRESSED is not empty.
REJECT stands.

## Re-confirmed faithful + no special-casing (no new finding)
- The char* arm (`CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL`, coreaction_infertypes.rs)
  is byte-faithful to typeop.cc:1035-1041 / 1061-1067 (`inslot==-1||outslot==-1 ->
  null`; `getMetatype()!=TYPE_INT -> null`; else `alttype`), with EQUAL/NOTEQUAL/
  LESS/LESSEQUAL correctly left on `propagateAcrossCompare` (typeop.cc:947-1011).
  Re-read the C++ at HEAD; matches exactly.
- Mechanical grep of the FULL src diff (3d655be..HEAD, src only) for register-name /
  fn-name / type-name / address literals (`"rax"|"rsp"|"main"|"readstruct"|"promote"|
  getOffset()==0x..|name=="..."|0x80662e0|"char"|"uint1"`): NONE. No `todo!`/
  `unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in added src. PASS.

## GAINED assertions are REAL parity (not fabricated)
The datatest XMLs under `decompiler/datatests/` are byte-identical to base (diff
empty), so every `Success` from `decomp_test_dbg` is a genuine C++-XML `stringmatch`
(`min/max` token count), not a porter-authored pass. The four gained assertions are
the highvar `vN`-naming renders (`Signed byte #1`: `v1 = ptr->thebyte`; `Dead Volatile
#2`: `v1` x2; `No for-loop iterator used #2/#3`: `v1 = 10;` / `v1 = v1 + 1;`) — real wins.

## No-regression sub-claims re-verified
- boolless / readstruct / condconst_conn: the named cargo tests are GREEN at HEAD
  (`w10_boolless_full_byte_parity_modulo_type_inference`, `w10_dc_condconst_conn_*`,
  `w10_eob_condconst_conn_*`, `verify_w10_symbol_naming_readstruct_body_uses_param_names_not_registers`,
  `verify_w10_infertypes_no_uint1_acc_leak_into_condconst`); their datatest assertion
  outcomes are byte-identical base vs HEAD (the only moved datatest assertions are the
  one REGRESSED + four GAINED above).
- Attribution unchanged: `Promotion on compare #2` regresses from the NAMING commit
  a2276c2 (`numInstances()>1 -> explicit`, coreaction_cleanup.rs:382 + HighVariable rep
  selection lets the un-folded self-`INT_AND(x,x)`'s `uint1` win), NOT the char* commit.
  `RuleTrivialArith` is in the `"analysis"` pool (universalaction.rs:399) which never
  reaches an active pool for promotecompare, so the char* follow-up cannot remove the
  AND. The isolated char* commit (3ba22d7..e9e6d11) is a true no-op on the passing set.

## findings (round 2)
- F-CP1 (blocking, unchanged): `Promotion on compare #2` regresses
  (Success@3d655be -> FAIL@165fbff). Root cause in naming commit a2276c2; not
  restorable by the char* follow-up. Recorded as LOSS-189 (already in main losses.md;
  not re-appended).
       cpp: decompiler/cpp/coreaction.cc (ActionNameVars / HighVariable rep selection);
            decompiler/cpp/ruleaction.cc:2358 (RuleTrivialArith `V & V => V`)
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:382 (numInstances>1 explicit);
             rust/crates/kuna-decomp/src/universalaction.rs:399 (trivialarith in `analysis`, never cloned for promotecompare)

## adversarial tests
The 9 already-committed adversarial tests (round 1) carry forward and are green at HEAD:
6 in `verify_w10_charptr_signedness.rs` (re-run: 6/6 ok) + 3 highvar-naming tests in
`verify_w10_struct_corpus.rs`. The concrete divergence trace for F-CP1 (the base-vs-HEAD
`decomp_test_dbg` passing-SET `comm` diff) is reproducible above. No new test added: the
branch is byte-identical to round 1 and the failing divergence is already pinned.

## verdict rationale (round 2)
Identical input to round 1 (branch unmoved), independently re-confirmed. The char*
transcription is faithful and special-casing-free; the suite, workspace, clippy, and
C++ oracle are all green. But the task's fixed bar is "Promotion #2 must pass / REGRESSED
must be EMPTY", and `Promotion on compare #2` STILL regresses vs base 3d655be. Per the
stated REJECT criterion this remains a REJECT. Work order unchanged: restore
`Promotion on compare #2` by activating the `analysis`/RuleTrivialArith pool for
promotecompare (fold the self-`INT_AND(x,x)`) or by fixing the HighVariable
representative selection so `char` wins over the spurious `uint1`. The +3 net gain vs
the single tracked regression (LOSS-189) is an owner ACCEPT-WITH-LOSSES decision, not
the verifier's, because the task fixed "Promotion #2 must pass" as the bar.
