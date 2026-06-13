# w7-m1-closure
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent verifier subagent)
date: 2026-06-13
branch: worktree-wf_f5333dc7-250-25 (closure commit 29bfed3; verify commit b490bee)

## What this item is

`w7-m1-closure` is a TRIAGE / coverage-closure item, not a code port. Its single
commit (29bfed3) touches ONLY the three in-scope unit-test files
(`testfuncproto.rs`, `testparamstore.rs`, `testtypes.rs`), un-ignores NOTHING,
adds NO library code, and:
  - refreshes every `#[ignore]` reason string from `"blocked: ..."` to a
    `"DEFER(W8): ..."` form,
  - adds an inline `// DEFER(W8): ...` comment above each ignored `#[test]`,
  - adds a `W7 TRIAGE (item w7-m1-closure)` header to each of the three files
    explaining why the 31 deferred bodies are NOT on the W7
    (HighVariable/Cover/Merge/RegionId) unblock path.

Verified mechanically: `git show 29bfed3 --name-only` = exactly the three test
files; `git diff rust-port..branch` ignore-attribute counts are IDENTICAL on
both sides (testfuncproto 21, testparamstore 4, testtypes 6) — the item flips
zero ignore->un-ignore transitions. The two un-ignored testtypes bodies
(`enum_matching`/`enum_matching2`) and the six ignored cast/ordering bodies were
already in that state at the rust-port base (an earlier dtype/harness wave); this
item did not author them.

## gate
- `python -m kuna.port_audit` -> 207/207, "PORT COMPLETE" (re-run after my
  commit: 2483 #[test] fns found; 8/8 suites, 0 missing).
- in-scope item suites (verifier-prompt gate), re-run fresh:
    testfuncproto  -> 0 passed / 21 ignored
    testparamstore -> 0 passed /  4 ignored
    testtypes      -> 2 passed /  6 ignored
    testfloatemu   -> 30 passed / 1 ignored  (the 1 = LOSS-025, w1/w2 scope)
    testkunaregion -> 3 passed
  All match the closure commit message exactly.
- `cargo test --workspace` -> ONE failure, `ldefs_pspec_cspec_parse_matrix`
  (corpus_bootstrap.rs:846): "Could not find .sla file for x86:LE:64:default".
  PRE-EXISTING and OUT OF SCOPE — corpus_bootstrap.rs is byte-identical to the
  rust-port base (empty diff) and the failure is purely an environment artifact
  (`.sla` files are gitignored build artifacts; zero exist in the worktree; the
  test panics on a missing artifact instead of SKIPping like its own
  spec-dir-absent branch at line 824). Not introduced or touched by this item.
  Recorded as F2 (informational) below.

## hunt list
- Signedness: N/A — item adds no comparisons/arithmetic. The underlying
  `enum_get_matches` (dtype.rs:2986, NOT this item) was re-derived: `int4 size`
  vs `u64 val`, `calc_mask(size)`/`coveringmask` faithful. Clean.
- Integer widths: N/A for the diff. enum oracle uses u64 throughout, matches
  C++ uintb. Clean.
- Wrapping: N/A for the diff. (PartialEnum path uses wrapping_shl/wrapping_mul,
  not exercised by the in-scope tests; not this item.) Clean.
- Comparator totality: the enum fixture's `BTreeMap<u64,String>` reproduces
  C++ `map<uintb,string>` ascending key order (total on u64). Independently
  pinned by my `..._insertion_order_independent` test. Clean.
- Iteration-order provenance: `make_enum` BTreeMap key order == std::map key
  order; the `upper_bound`->`--iter` predecessor walk in `enum_get_matches`
  faithfully ports type.cc:1557-1559 (range Excluded(target).. then
  Excluded(k)..next_back). Re-verified by extra-boundary tests. Clean.
- Off-by-one / do-while: the `for count in 0..2` two-pass complement loop ports
  the C++ `for(count=0;count<2;++count)` exactly (type.cc:1543). Clean.
- Erase-while-iterating: N/A. Clean.
- Exception->Result parity: C++ `getMatches` returns void; Rust `get_matches`
  returns `KunaResult` and `Err`s only on a non-enum kind (can't occur for a
  well-formed fixture). Tests `.unwrap()` faithfully. Clean.
- THIS WAVE (HighVariable merge/Cover-intersection order, varmap stack-layout
  order, block-structuring schema precedence): N/A to a test-triage item; those
  belong to w7-s6-merge / w7-s6-variable-cover / w7-s6-varmap, reviewed
  separately. Clean for this item.

## Faithfulness of the triage (the core review question)

The danger in a deferral item is deferring tests that SHOULD have been portable
on the wave's foundation (a hidden weakening). Re-derived against the C++
sources in scope and grepped the Rust library tree:

- testfuncproto (21 TESTs): every body needs `getModel("__model1")` ->
  `glb->protoModels[nm]` (a prototype-model registry on a spec-loaded
  `Architecture` built by `buildArchitecture`+`init`, decoding `<prototype>`
  XML via `ProtoModel::decode`), `parse_protopieces` (prototype grammar), and
  `register_equal` -> `glb->translate->getRegister(name)` (name->VarnodeData on
  a spec-loaded Translate). [testfuncproto.cc:192-208, 217-296]
- testparamstore (4 TESTs): each builds a REAL per-arch `Architecture`
  (x86-64/PPC/MIPS/AARCH64) from a full SLEIGH spec and runs `parse_C`
  (testparamstore.cc:78) + `parse_protopieces`. [testparamstore.cc:38-79]
- testtypes (6 ignored): the five `cast_*` need
  `CastStrategy::getInputCast`/`markExplicitLongSize`/`TypeOpCast` over
  `parse_type`-built operands; `type_ordering` needs the W8 TypeFactory
  base-type catalog. [testtypes.cc:74-118]

Grep over `rust/crates/*/src` (excluding tests/comments/stages.toml):
`parse_type` / `parse_C` / `parse_protopieces` / `CastStrategy` /
`getInputCast` / `markExplicitLongSize` / `TypeOpCast` -> ZERO library hits.
`assign_parameter_storage` (fspec.rs:3793) and `derive_input_map`
(fspec.rs:3673) ARE ported (matching the triage text), but cannot be driven
without the grammar + spec-loaded registry. No `getRegister(name)->Address`
exists (only the reverse `getRegisterName(addr)->String`). No
HighVariable/union/ScoreUnionFields TEST exists in any in-scope file, so W7's
Merge/HighVariable delivery genuinely unblocks none of them. The triage is
FAITHFUL: every deferred body is un-portable on this wave's foundation; porting
by hand-reconstructing the spec-loaded stack would be an unfaithful
transcription (correctly declined).

The two UN-ignored bodies (`enum_matching`/`enum_matching2`) are faithful,
NOT weakened: every assertion (match-name sequences in order, complement flags,
the exact inputs 5/0xfffffffffffffff7/0/0x10/12/7/11) matches
testtypes.cc:215-261 verbatim, and the underlying `enum_get_matches` is a
careful port of `TypeEnum::getMatches`. My 17 extra-boundary adversarial inputs
(below) all produce the correct oracle outputs.

## Mechanical pass
- `port_audit` -> 207/207 (re-derived independently from the C++ TEST() source
  by my `name_parity` test, not trusting the Python tool).
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`partial_cmp`
  over the 3 in-scope test files: none (the only `as`/`HashMap` substrings are
  doc-comment prose; no bare casts).
- clippy: the touched files + my new test file are clean. `cargo clippy -p
  kuna-decomp` reports an unrelated PRE-EXISTING error in heritage.rs:1602
  (`... || true` tautology in a W6 test assertion) — heritage.rs is
  byte-identical to base and OUT OF SCOPE for this item. Recorded as F1 below.

## findings
  - F1 (informational, OUT OF SCOPE): `cargo clippy -p kuna-decomp` fails on a
    pre-existing `|| true` logic-bug lint.
       cpp: n/a
       rust: rust/crates/kuna-decomp/src/heritage.rs:1602
       note: unchanged vs rust-port base; a W6 issue, not this item. Flagged so
       the wave gate owner can route it; does not bear on w7-m1-closure.
  - F2 (informational, OUT OF SCOPE): `cargo test --workspace` has one red test,
    `ldefs_pspec_cspec_parse_matrix`, from a missing gitignored `.sla` artifact.
       cpp: n/a
       rust: rust/crates/kuna-decomp/tests/corpus_bootstrap.rs:846
       note: unchanged vs base; the test panics on a missing build artifact
       instead of SKIPping (its sibling branch at :824 does SKIP for absent
       spec dirs). Environment robustness, not a w7-m1-closure regression.
  - F3 (informational, OUT OF SCOPE): losses.md has duplicate ## headings for
    LOSS-040, -045, -087, -088, -094.
       note: pre-existing data-quality issue; verifiers append-only, not mine to
       edit. The two LOSS-094 entries (lines 868, 899) are distinct content;
       the second is the one covering this item's deferral.

## adversarial tests
rust/crates/kuna-decomp/tests/verify_w7_m1_closure.rs (6 tests, all green):
  - w7_m1_closure_name_parity_in_scope_suites — re-derives every C++ TEST()
    name from source and asserts a same-named Rust #[test] fn exists (21/4/8).
  - w7_m1_closure_every_ignore_has_defer_reason_and_inline_comment — every
    #[ignore] in the 3 files carries "DEFER" in its reason AND an inline
    `// DEFER(` comment above it.
  - w7_m1_closure_ignore_counts_match_triage — pins 21/4/6 so a future silent
    un-ignore/re-ignore without re-triage trips here.
  - w7_m1_closure_enum3_extra_boundary_inputs — 5 extra inputs (3, 15, 13,
    ~1, 6) hand-derived from TypeEnum::getMatches; faithfulness guard on
    enum_matching.
  - w7_m1_closure_enum4_extra_boundary_inputs — 4 extra inputs (6, 14, 11,
    0x10) exercising the non-power-of-two SIX member + the restricted-research
    branch; faithfulness guard on enum_matching2.
  - w7_m1_closure_enum_match_is_insertion_order_independent — same enum built
    with scrambled insertion order yields identical results over 12 probe
    values (proves the BTreeMap reproduces std::map key order).

## losses
- LOSS-095 (the 29 `#[ignore]`d name-match scaffolds: testfuncproto 21/21,
  testparamstore 4/4, testtypes 6/8) — already documented with exact
  cpp/rust anchors and restoration criteria; covers this item's deferral.
- LOSS-094 (second entry, losses.md:899: M1 unit-test body parity split across
  W6/W7) — already documented; its restoration criteria are satisfied by this
  triage for the W7-unblockable subset (none in scope) and correctly carry the
  remainder to W8.
- LOSS-025 — the testfloatemu `double_decimal_precision` DBL_MAX ignore is
  owned by w1-num-float-multiprec, correctly attributed by the closure.
No new loss appended: the deferral is fully covered by the existing entries and
the triage decision is faithful.

## verdict rationale
ACCEPT-WITH-LOSSES. The item is a faithful, conservative, fully-documented
deferral: it un-ignores nothing it shouldn't, every ignored body is genuinely
un-portable on the W7 foundation (grep-proven), every ignore carries a DEFER
annotation, and the only two un-ignored in-scope bodies are faithful (oracle
re-pinned over extra boundary inputs). port_audit = 207/207. The two red
workspace signals (F1 clippy, F2 corpus_bootstrap) are pre-existing and
byte-for-byte outside this item's diff. Losses cited: LOSS-095, LOSS-094.
