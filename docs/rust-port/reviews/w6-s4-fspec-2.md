# w6-s4-fspec-2
verdict: REJECT
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-13
round: 1

gate: `cd rust && cargo test -p kuna-decomp` -> PASS (1027 lib tests + all integration
  suites green once the gitignored `.sla` build artifacts are present in the worktree).
  - The lone gate failure observed first run — `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`
    ("Could not find .sla file for x86:LE:64:default") — is a missing-build-artifact
    ENVIRONMENT issue in the worktree, not a port defect: the diff touches only
    `fspec.rs` + `fspec/tests.rs`, the failing test exercises no fspec code, and copying
    the main-tree `x86/ARM/MIPS *.sla` into the worktree makes it pass. Verified.
  - clippy: `cargo clippy -p kuna-decomp -- -D warnings` is CLEAN. (`--tests -D warnings`
    surfaces only pre-existing `bool_assert_comparison` style nits in OTHER waves'
    verify_*.rs files; zero in this item's logic or in verify_w6_s4_fspec_2.rs.)

scope: decompiler/cpp/fspec.cc ~2268-4928 + fspec.hh — ProtoModel, ScoreProtoModel,
  ProtoModelMerged, ProtoParameter/ParameterBasic, ProtoStore/ProtoStoreInternal,
  FuncProto core.

## hunt list

- **Signedness.** Clean. `int4 i < container.size()` loops become `for i in 0..len`
  with `len as i32` where a signed count is needed (e.g. `getNumInputs`); `res.size`
  (`uint4`) vs `param->getSize()` (`int4`) compares are reproduced with
  `param.get_size() as u32 > res.size`, matching the C++ `-Wno-sign-compare` unsigned
  promotion (fspec.cc:4479 / fspec.rs:5442). No signed/unsigned trap found.
- **Integer widths.** Clean. `resolveExtraPop`'s `(int4)addr.getOffset()` is
  `(addr.get_offset() as i32)` (explicit truncation, faithful); the alignment mask
  `(cur+3)&0x0ffffffc` is transcribed exactly (fspec.cc:3992 / fspec.rs:5073).
- **Wrapping.** Mostly clean. `resolveExtraPop` uses `wrapping_add` for the
  `offset+size` term; the subsequent `(cur + 3)` is a plain `i32` add (would panic in
  debug if `cur == i32::MAX-2`). C++ wraps silently. Stack-param offsets make this
  unreachable in practice; noted as MINOR (F2), not blocking.
- **Comparator totality.** Clean. `PEntry` is slot-only (`sort_unstable_by_key(|p|
  p.slot)`, matching `std::sort` + slot-only `operator<`). `compare_by_address`
  (address-only `Ordering`) and `VarnodeData::Ord` (space-idx, offset, then REVERSED
  size — "BIG sizes come first", fspec/pcoderaw) are total and faithful;
  `intersect_effects`/`intersect_registers` consume them correctly. `proto_parameter_eq`
  is pointer-identity on type (matches C++ `Datatype*` compare).
- **Iteration-order provenance.** Clean. No HashMap/HashSet in ported logic. `foldIn`
  walks `model.localrange/paramrange` (`set<Range>` -> kuna `RangeList::iter`, sorted)
  and unions; `selectModel` walks `modellist` (Vec, insertion order) keeping the FIRST
  min — order-faithful. The builder seam `push_effect` uses `sort_by` (STABLE) where
  C++ decode uses `std::sort` (UNSTABLE); only observable for two effects at the SAME
  address, which real specs never emit — noted MINOR (F3).
- **Off-by-one / do-while / reverse iterators.** Clean. `lookupEffect`/`lookupRecord`
  `upper_bound` + `--iter` is `upper_bound_by(...)` + `idx-1` with the `idx==0` guard
  and the begin()-deref branch reproduced (fspec.rs:3826-3885). `clearInput`'s
  shift-down + trailing-null pop is faithful, and the Rust `while matches!(last(),
  Some(None))` is strictly safer than C++'s `inparam.back()` on an empty vector (same
  result for all reachable inputs).
- **Erase-while-iterating.** Clean. `intersectEffects`/`intersectRegisters` build a
  fresh `newlist` then swap (`self.x = newlist`), identical to the C++ two-pointer
  merge + `swap`.
- **Exception -> Result partial-state parity.** **FINDING F1 (major).** See below.

## findings

- **F1 (major): the `ParamUnassignedError`-only catch is ported as a catch-all `Err(_)`,
  swallowing non-ParamUnassigned `LowlevelError`s that C++ propagates.**
    cpp: decompiler/cpp/fspec.cc:2437-2447 (`assignParameterStorage`,
         `catch(ParamUnassignedError&)`) and fspec.cc:4211-4227 (`updateAllTypes`,
         `catch(ParamUnassignedError&)`)
    rust: rust/crates/kuna-decomp/src/fspec.rs:3787-3800 (`assign_parameter_storage`
          ignore-output branch, `match ... Err(_) => { void fallback }`) and
          fspec.rs:5207-5226 (`update_all_types`, `match ... Err(_) => { ERROR_INPUTPARAM }`)

  Two compounding defects:
  1. `unassigned_err` (fspec.rs:3230-3236) builds `KunaError::lowlevel(...)` — NOT the
     dedicated `KunaError::ParamUnassigned` variant that exists in kuna-base
     (`error.rs:132`, ctor `param_unassigned` at :204) precisely to give this catch a
     discriminator. So even a type-tagged match could not currently separate it.
  2. Both catch sites use a bare `Err(_)`, so EVERY error from `output->assignMap` /
     `input->assignMap` is treated as the C++ `ParamUnassignedError` case.

  C++ `assignMap` can throw `LowlevelError`s that are NOT `ParamUnassignedError`
  (reachable in the ported subset): "Size mismatch when joining parameters"
  (fspec.rs:1491), "Missing next resource start" (fspec.rs:2667), and the SEAM(W4)
  hidden-return stub (fspec.rs:2533). For those, C++ lets the exception ESCAPE
  `updateAllTypes`/`assignParameterStorage` (caught higher up by restart/recovery
  machinery); the port instead (a) in `assign_parameter_storage`+`ignore_output_error`
  silently substitutes a `void` return, and (b) in `update_all_types` silently sets
  `error_inputparam` and returns `Ok`. This is exactly the exception->Result
  partial-state divergence the hunt list warns about: the error never reaches the
  caller, control flow diverges.

  Demonstrated by the verifier's tests
  `assign_parameter_storage_ignore_output_swallows_nonparamunassigned_w6s4f2` and
  `update_all_types_swallows_seam_lowlevel_as_input_error_w6s4f1` (both currently PASS,
  pinning the swallowing behavior).

  Fix (mechanical, low-risk; the infrastructure already exists and is unused at these
  sites): (1) make `unassigned_err` return `KunaError::param_unassigned(...)`; (2)
  change both catch sites to `Err(KunaError::ParamUnassigned { .. }) => <recover>` and
  `Err(e) => return Err(e)` for everything else.

- **F2 (minor): `resolveExtraPop` `(cur + 3)` is a non-wrapping `i32` add.**
    cpp: decompiler/cpp/fspec.cc:3992 (`cur = (cur+3)&0xffffffc;`, C++ wraps)
    rust: rust/crates/kuna-decomp/src/fspec.rs:5073 (`cur = (cur + 3) & 0xffffffc;`)
  Unreachable on real stack-param offsets (would require offset+size near i32::MAX),
  but for byte-exact wrap parity it should be `cur.wrapping_add(3)`. Non-blocking.

- **F3 (minor): builder seam `push_effect` sorts STABLE where C++ decode sorts
  UNSTABLE.**
    cpp: decompiler/cpp/fspec.cc:2698 (`sort(effectlist..., compareByAddress)` — std::sort)
    rust: rust/crates/kuna-decomp/src/fspec.rs:4114 (`self.effectlist.sort_by(...)`)
  `compareByAddress` is address-only, so the order between two same-address effects is
  unspecified in C++ and insertion-stable in Rust; `lookupEffect` (`--iter`) would pick
  a different record only for two effects at the SAME address — which compiler specs do
  not emit. This is a builder seam, not the real `decode` (a SEAM stub). Non-blocking.

## adversarial tests
(rust/crates/kuna-decomp/tests/verify_w6_s4_fspec_2.rs — 9 tests, all PASS, committed
on the worktree at 877e822)
  - assign_parameter_storage_ignore_output_swallows_nonparamunassigned_w6s4f2  (F1)
  - update_all_types_swallows_seam_lowlevel_as_input_error_w6s4f1               (F1)
  - update_all_types_happy_path_assigns_output_no_error_w6s4   (F1 control: error path isolated)
  - do_score_hole_at_slot0_costs_16_w6s4
  - do_score_duplication_and_mismatch_penalties_w6s4
  - set_return_bytes_consumed_is_monotone_decreasing_w6s4
  - select_model_ties_break_to_first_index_w6s4
  - select_model_empty_trials_returns_first_w6s4
  - fixture_model_uses_standard_resource_lists_w6s4

## disposition
REJECT on F1 (major, exception->Result parity, reachable in the ported subset). The
ported algorithmic surface (ProtoModel/ScoreProtoModel/ProtoModelMerged/ProtoStore/
FuncProto query+score+merge logic) is otherwise faithful — F1 is a localized 2-site fix
using already-present `KunaError::ParamUnassigned` infrastructure. F2/F3 are MINOR and
can ride along. No losses appended (REJECT). Work order = F1 (mandatory), F2/F3
(recommended).
