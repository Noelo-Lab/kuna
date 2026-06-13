# w6-s4-fspec-2
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-13
round: 2

gate: `cd rust && cargo test -p kuna-decomp` -> PASS (fully green). lib 1027 pass;
  verify_w6_s4_fspec_2 9 pass; verify_w6_s4_fspec_2_r2 (verifier round-2) 5 pass; every
  other integration suite pass. No failures across any suite. Unlike round 1, the
  `.sla` build artifacts are present in this worktree, so `corpus_bootstrap` runs and
  passes too — no environment caveat this round.
  - clippy: `cargo clippy -p kuna-decomp --lib -- -D warnings` CLEAN (forced recompile,
    4.53s, zero warnings). `cargo clippy -p kuna-decomp --test verify_w6_s4_fspec_2_r2
    -- -D warnings` CLEAN.

scope: decompiler/cpp/fspec.cc ~2268-4928 + fspec.hh — ProtoModel, ScoreProtoModel,
  ProtoModelMerged, ProtoParameter/ParameterBasic, ProtoStore/ProtoStoreInternal,
  FuncProto core.

## round-1 -> round-2 delta

Round 1 REJECTed on **F1 (major, exception->Result parity)**. The repair commit
`7af2e13` is a minimal, exactly-on-target fix (8 src lines, confirmed via
`git diff 877e822 7af2e13 -- src/fspec.rs` to touch ONLY the two catch sites +
`unassigned_err`):
  1. `unassigned_err` (fspec.rs:3231) now builds `KunaError::param_unassigned(...)` (the
     dedicated `ParamUnassigned` discriminator) instead of `KunaError::lowlevel(...)`.
  2. `ProtoModel::assign_parameter_storage` ignore-output branch (fspec.rs:3790) and
     `FuncProto::update_all_types` (fspec.rs:5223) now match
     `Err(KunaError::ParamUnassigned { .. })` for the recovery path and
     `Err(e) => return Err(e)` for everything else — matching C++
     `catch(ParamUnassignedError&)` letting all other exceptions escape.

Independently re-verified that **every** C++ `ParamUnassignedError` throw site reachable
in the ported subset maps to `param_unassigned` (so the tightened catch still RECOVERS
them): fspec.cc:797/803/814 (input `ParamListStandard::assignMap`) and :1529
(`ParamListRegisterOut`) all flow through `unassigned_err` (now ParamUnassigned).
fspec.cc:1601 (`ParamListStandardOut` "Cannot assign return value as a pointer") lives
BEHIND the SEAM(W4) hidden-return stub (fspec.rs:2533, a `lowlevel` error) — it is
unported, so no genuine `ParamUnassignedError` is mis-tagged as `lowlevel`. F1 is fully
resolved for the ported surface.

## hunt list

- **Signedness.** Clean (re-derived). `int4 i < container.size()` loops -> `for i in
  0..len`; `getNumInputs`/`numParams` keep `as i32`. doScore's `nextfree`/`p.slot`/
  `p.size` all `int4`. No signed/unsigned trap.
- **Integer widths.** Clean. `(int4)addr.getOffset()` -> `addr.get_offset() as i32`
  (faithful truncation); `EffectRecord::get_size` `range.size (uint4) as i32` matches
  C++ `int4 getSize(){return range.size;}`. The doScore penalty table is fixed
  `[16,10,7,5]` + `penaltyfinal=3`, `mismatchpenalty=20` — exact.
- **Wrapping.** **FINDING F2 (minor, LOSS-086).** `resolveExtraPop` first sum uses
  `wrapping_add` (correct) but the alignment step `(cur + 3)` is a plain i32 add that
  PANICS in debug where C++ wraps. Round 1 called this MINOR-presumed-unreachable; round
  2 confirms it IS reachable (reproducing input: stack offset 0x7fff_fffb) but debug-only
  (release wraps == C++) and non-physical. Recorded as LOSS-086, not a blocker. See below.
- **Comparator totality.** Clean (re-derived). `EffectRecord::compare_by_address`
  (fspec.rs:1673) is `Address::cmp` = (space-index, offset), matching C++
  `compareByAddress` (space->getIndex(), offset) — deliberately NOT including size, a
  strict-weak-order; `intersect_effects` then gates the `Equal` arm on full `operator==`
  (type + range incl. size), exactly as C++ (fspec.cc:2800-2805). `PEntry`
  `sort_unstable_by_key(|p| p.slot)` matches `std::sort` + slot-only `operator<`
  (both unstable; equal-slot relative order unspecified in BOTH). `VarnodeData::cmp`
  (used by intersect_registers) total. No `partial_cmp`/float-cmp anywhere in the file.
- **Iteration-order provenance.** Clean. Zero `HashMap`/`HashSet` in the file (grep
  confirmed). `foldIn` unions `model.localrange`/`paramrange` via `RangeList::iter`
  (sorted set order) — matches C++ `set<Range>` iteration. `selectModel` walks
  `modellist` (Vec, insertion order) keeping the FIRST strict-min, `bestscore==0`
  early-break — byte-faithful to fspec.cc:2887-2906. `update_all_types` Ok-path loop
  walks `pieces` by index with the hidden-ret `i-1`/`j`-skip exactly per fspec.cc:4215.
- **Off-by-one / do-while / reverse iterators.** Clean. `lookupEffect`/`lookupRecord`
  use `upper_bound_by` (predicate `cmp(val,mid) != Less`, == `std::upper_bound`'s
  `!comp(val,*mid)`) + `idx-1` with the `idx==0` begin-deref branch reproduced
  (fspec.rs:3837-3886). doScore's `nextfree < 4` is the penalty-table bound; the
  verifier's penaltyfinal test pins the >=4 saturation. `clearInput` shift-down +
  trailing-null pop is faithful and STRICTLY safer than C++'s `inparam.back()` on an
  empty vector.
- **Erase-while-iterating.** Clean. `intersectEffects`/`intersectRegisters` build a
  fresh `newlist` then `self.x = newlist` — identical to the C++ two-pointer merge +
  `swap`. `clearInput` renumber via `take()` matches the C++ shift + null-out.
- **Exception -> Result partial-state parity.** **F1 RESOLVED** (was the round-1 major).
  Both catch sites now discriminate on `ParamUnassigned`. Partial-state verified: the
  pre-`try` mutations (`set_model`/`clearAllInputs`/`clearOutput` in update_all_types;
  the `res.clear()`+void-push in the ignore-output catch) match C++ exactly; the Ok-path
  body (`set_output`/`set_input`) is non-throwing in both languages, so wrapping it in
  the `Ok(())` arm (vs inside the C++ `try`) is behavior-identical.

## findings

- **F1 (was major) — RESOLVED in round 2.** Exception->Result parity. The
  ParamUnassignedError-only catch is now faithfully ported (discriminator +
  propagate-everything-else). Re-verified all throw sites + both catch sites.
    cpp: decompiler/cpp/fspec.cc:2441, 4225 (catch); :797/803/814/1529 (ParamUnassigned
         throws, all -> `unassigned_err`); :1601 (behind SEAM(W4), unported)
    rust: rust/crates/kuna-decomp/src/fspec.rs:3790, 5223 (catch); :3231 (`unassigned_err`
          -> `param_unassigned`)
  Pinned by the porter's `..._propagates_nonparamunassigned_w6s4f2` /
  `..._propagates_seam_lowlevel_w6s4f1` AND the verifier's round-2
  `update_all_types_genuine_param_unassigned_sets_flag_w6s4f1_r2` (genuine ParamUnassigned
  STILL recovered -> the fix did not over-tighten) and
  `assign_parameter_storage_ignore_output_only_catches_param_unassigned_w6s4f1_r2`.

- **F2 (minor, LOSS-086): `resolveExtraPop` `(cur + 3)` is a non-wrapping i32 add.**
    cpp: decompiler/cpp/fspec.cc:3992 (`cur = (cur+3)&0xffffffc;` — C++ wraps silently)
    rust: rust/crates/kuna-decomp/src/fspec.rs:5074 (`cur = (cur + 3) & 0xffffffc;`)
  DEBUG builds panic ("attempt to add with overflow") on a near-`i32::MAX` stack-param
  offset; RELEASE builds (overflow checks off) wrap and match C++ bit-for-bit. The input
  (~2GB locked stack offset) is non-physical for real frames; the divergence is a loud
  debug panic, never silent output corruption. One-char fix: `cur.wrapping_add(3)`.
  Reproduced + pinned (via `catch_unwind`) by the verifier's
  `resolve_extra_pop_near_i32_max_offset_w6s4f2_r2`. Recorded as **LOSS-086**.

  (Round 1's F3 — `push_effect` STABLE sort vs C++ `decode`'s UNSTABLE `std::sort` — is a
  test/builder seam only observable for two effects at the SAME address, which compiler
  specs never emit; it is NOT in the real `decode` path. Re-confirmed non-blocking and not
  re-raised; no behavioral divergence on any spec-derived input.)

## mechanical pass

- `grep` over the ported file: zero `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `partial_cmp`. `sort_unstable*` uses all map to C++ `std::sort` (unstable) — the
  in-scope one (`do_score`, `sort_unstable_by_key(|p| p.slot)`) matches the slot-only
  `PEntry::operator<`.
- bare `as` casts: the correctness-bearing ones for this item are
  `addr.get_offset() as i32` (intended `(int4)` truncation, fspec.cc:3991) and
  `range.size as i32` (matches `int4 getSize()`); both faithful. The remaining `as`
  casts are `len()`/index conversions (idiomatic; round-1 disposition unchanged).
- clippy `-D warnings`: clean (lib + the new verifier test).

## adversarial tests
round-2 file (verifier): rust/crates/kuna-decomp/tests/verify_w6_s4_fspec_2_r2.rs — 5
tests, all PASS, committed on the worktree at 1807f47:
  - update_all_types_genuine_param_unassigned_sets_flag_w6s4f1_r2  (F1: still recovers a
        genuine ParamUnassignedError -> Ok + error_inputparam; the fix did not over-tighten)
  - assign_parameter_storage_ignore_output_only_catches_param_unassigned_w6s4f1_r2  (F1:
        non-ParamUnassigned SEAM error propagates even on ignore_output_error=true)
  - do_score_penaltyfinal_used_past_slot_index_4_w6s4_r2  (doScore: 16+10+7+5+3==41, the
        nextfree<4 / penalty[] vs penaltyfinal boundary)
  - resolve_extra_pop_aligns_to_four_bytes_w6s4_r2  (control: (0x14+4+3)&0x0ffffffc == 24)
  - resolve_extra_pop_near_i32_max_offset_w6s4f2_r2  (F2/LOSS-086: pins the debug-build
        overflow panic via catch_unwind at the crafted near-i32::MAX offset)

round-1 file (still present, all PASS): rust/crates/kuna-decomp/tests/verify_w6_s4_fspec_2.rs
— 9 tests (the two F1 pins were UPDATED by the repair to assert the corrected propagation).

## losses
- LOSS-086 (F2): `resolveExtraPop`'s non-wrapping `(cur+3)` alignment add — debug-only
  panic vs C++ wrap; release-identical; non-physical input.

## disposition
ACCEPT-WITH-LOSSES. The round-1 blocker (F1, exception->Result parity) is fully and
faithfully repaired and independently re-verified from both directions (genuine
ParamUnassigned still recovered; non-ParamUnassigned propagates). The entire algorithmic
surface — ProtoModel query/effect-lookup, ScoreProtoModel scoring, ProtoModelMerged
foldIn/intersect/selectModel, ProtoStore clearInput/renumber, FuncProto
updateAllTypes/resolveExtraPop core — is re-derived as faithful. The single remaining
divergence (F2) is a MINOR, debug-only, non-physical wrapping edge, ledgered as LOSS-086
with a one-character restoration path and pinned by a verifier test. Gate fully green,
clippy clean.
