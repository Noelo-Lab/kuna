# w10-struct-return
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8-1m (independent verifier, round 2)
date: 2026-06-15
branch: rport/w10-struct-return @ 8aaa0ea (porter) / 10841d1 (verifier r2 tests committed)
gate: cargo test --workspace -> 150 "test result: ok" lines, 0 failed (incl. the
      8 verifier tests in verify_w10_struct_return); cargo clippy --lib (production
      kuna-decomp/kuna-console/kuna-base) -> clean; clippy on verify_w10_struct_return
      -> clean (the only --tests clippy warnings are pre-existing in printc/tests.rs,
      double.rs, ruleaction_*/tests.rs, typeop/tests.rs — none touched by this branch);
      C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` ->
      207/207 unit + 675/675 datatest, PARITY OK (untouched — branch changes only `rust/`).
cpp blobs at review (rust-port tree): fspec.cc 903b587e, coreaction.cc 28f6157f,
      funcdata.cc 6571bfe9.

## Round-1 status

Round 1 (@ b5c6bc3) was REJECT on finding F1: the "pointer ALSO unassignable"
failure (C++ fspec.cc:1601 `throw ParamUnassignedError(...)`) was ported as a
generic `KunaError::lowlevel(...)`. That is the wrong variant —
`ParamUnassignedError` is a dedicated `LowlevelError` subclass caught by
`ProtoModel::assignParameterStorage(...,ignoreOutputError=true)` (fspec.cc:2441)
to degrade the output to void; a generic `LowlevelError` is NOT caught and
hard-aborts the assignment, diverging from C++ on a reachable path.

## F1 resolution (round-2 verification)

Commit 8aaa0ea changes fspec.rs:2582 from `KunaError::lowlevel(...)` to
`KunaError::param_unassigned("Cannot assign return value as a pointer")`. Verified:
- `KunaError::param_unassigned` constructs the `ParamUnassigned` variant
  (error.rs:202-204), the C++ `ParamUnassignedError` analogue (error.rs:129-135).
- `ProtoModel::assign_parameter_storage` catches exactly that variant on the
  `ignore_output_error` arm (`Err(KunaError::ParamUnassigned { .. })` at
  fspec.rs:3857), clearing `res` and emplacing one void piece — line-faithful to
  C++ fspec.cc:2441-2447. Anything else routes through `Err(e) => return Err(e)`,
  matching C++'s "let other exceptions propagate" semantics.
- The OTHER new error in the function (the kuna-added no-default-data-space guard
  at fspec.rs:2553) correctly stays a generic `lowlevel` — C++ assumes
  `getDefaultDataSpace()` non-null (no catch), so the conservative guard must NOT
  be swallowed by the ignore arm.
F1 is fixed; the two error paths are correctly distinguished. Mutation-checked:
reverting the fix (`param_unassigned` -> `lowlevel`) fails the new round-2 tests
`pointer_unassignable_raises_param_unassigned_not_generic_lowlevel` and
`shared_status_lets_second_assign_reattempt_same_entry_and_fail`.

## Scope

Two commits beyond rust-port (b5c6bc3 + the F1-fix 8aaa0ea), 5 files, only `rust/`:
- `fspec.rs` `assign_map_standard_out` — closes the hidden-return seam (too-big
  return -> indirect-storage output + hidden pointer input param), wired through
  `AddrSpaceManager::get_default_data_space` + `TypeFactory::get_type_pointer`.
  C++ `ParamListStandardOut::assignMap` (fspec.cc:1571-1614).
- `ifacedecomp.rs` — closes the `IfcMapReturn` (`map return`, ifacedecomp.cc:635-648)
  and `IfcTypeVarnode` (`type varnode`, ifacedecomp.cc:1734-1762) console seams,
  plus a `parse_varnode` helper (grammar.cc:3055-3084).
- `varmap.rs` — `set_symbol_isolated`/`full_name` thin delegates for `IfcTypeVarnode`
  (`Symbol::setIsolated` database.cc:256, `Scope::getFullName` database.cc:1448).
- `fspec/tests.rs` — one porter unit test for the hidden-return path.
`set_output`/`attach_internal_store`/`get_func_proto_mut`/`get_type_pointer`/
`get_default_data_space` are pre-existing on rust-port (NOT introduced here — confirmed
the diff defines none of them).

## Faithfulness

`assign_map_standard_out` is a line-faithful transcription of fspec.cc:1571-1614:
void short-circuit (1577 / rs:2532), `assignAddress(outtype)` then
`fail -> hiddenret_ptrparam` (1582-1585 / rs:2538-2543), the tri-state response
gate (1586 / rs:2544-2547), spacebase-or-default-data-space pointer sizing
(1589-1594 / rs:2550-2561), void-special-reg vs pointer-retype branch with the
inner `assignAddress(pointertp)->fail->throw` (1595-1603 / rs:2562-2586),
`indirectstorage` on the output AFTER both arms (1604 / rs:2587), and the trailing
hidden pointer param with `hiddenretparm`-iff-special (1606-1612 / rs:2590-2601).
Flag constants match (INDIRECTSTORAGE=4, HIDDENRETPARM=2, TYPELOCK=16, NAMELOCK=8).
The single `status` vector is reused across both `assign_address` calls exactly as
C++ shares one `vector<int4> status`. `IfcMapReturn`/`IfcTypeVarnode`/`parse_varnode`
verified against ifacedecomp.cc:635-648/1734-1762 and grammar.cc:3055-3084.

## Mandatory hunt list

- Signedness: clean. `spc.get_addr_size() as int4` (fspec.rs:2558) mirrors C++
  `int4 pointersize = spc->getAddrSize()` (uint4->int4, silent under
  -Wno-sign-compare); `wordsize` stays `u32`->`uint4 ws` matching C++ `getTypePointer`.
  (The `as int4` lacks a `// cast:` comment — protocol nit, not a finding; it is a
  1:1 of the C++ implicit narrowing.)
- Integer widths: clean. `get_type_pointer(s: int4, pt, ws: uint4)` matches C++
  `getTypePointer(int4 s, Datatype*, uint4 ws)`.
- Wrapping: n/a (no arithmetic on uintb-lineage values added).
- Comparator totality: n/a.
- Iteration order: clean. Only loop added is the hex-digit scan in `parse_varnode`;
  no HashMap/HashSet/sort_unstable anywhere in the diff (the only ordered container,
  `gather_data_tests`'s set, is a pre-existing BTreeSet).
- Off-by-one / do-while / reverse iterators: clean. `parse_varnode`'s peek/consume
  scan matches the C++ `s >> ws >> tok` sequence; the `:uniq` hex scan stops at the
  first non-hex char like C++ `s >> hex`.
- Erase-while-iterating: n/a.
- Exception -> Result partial-state parity: CLEAN (round-1 F1 RESOLVED). The
  pointer-unassignable throw raises `ParamUnassigned` (the catchable variant) and
  leaves `res` empty before the trailing emplace_back — C++ throws at the same point,
  before `res.emplace_back()` for the hidden param. Verified by
  `shared_status_..._and_fail` (asserts `res.is_empty()` on the throw).

## Mechanical pass

- grep added prod lines: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`. Bare `as` casts added in prod: `c as char` (u8->char, infallible)
  and `spc.get_addr_size() as int4` (faithful C++ narrowing). Two
  `.expect("fd checked Some above")` are guarded by prior `fd.is_none()` checks.
- No special-casing: grep of the full diff for function-name/address/value/type-name
  hardcoding is clean — the only string literals are the verbatim C++ error/echo
  messages, a comment `%EAX`, and a test fixture proto name `"f"`. No magic addresses,
  no per-file decode shortcut.
- clippy: production `--lib` clean; verifier test clean. port-audit: crate not
  vendored on this branch (n/a).

## Realness / parity (measured directly)

The recovery is a FAITHFUL FOUNDATION, not the full struct-return capability — and
that gap is recorded as LOSS-165 (the (B) dormant-downstream pattern). Measured
against the rust-port base (03c4cef) with the Rust `decomp_test_dbg`:
- rust-port base: `retstruct.xml` and `stackreturn.xml` BOTH report
  "Error: Did not apply tests … engine integration not yet ported: parse_type +
  readVarnode / parse_machaddr + parse_type + FuncProto::setOutput" (exec-failure,
  0 applied).
- this branch: both now APPLY (11 assertions). The exec-failure seam is genuinely
  CLOSED (−2 exec-failures). But all 11 assertions FAIL the content match: the Rust
  render is `{ return; }` where the oracle emits the typed struct-field body
  (`v1.a = tmp; v1.b = y; return v1;`). The typed-body render is the unported
  downstream — ZERO new positive datatest assertion.
- No prior byte-parity functions regress: print_b5_boolless (11 pass, 1 ignored),
  verify_w10_struct_corpus incl. readstruct + condconst (16 pass) — both files are
  unchanged on the branch; full workspace 0 failed. C++ oracle 675/675 untouched.

This matches the wave's established (B) partial-restoration pattern (LOSS-156/158/
159/160/161/162/163): a genuinely-closed, faithful, exercised foundation whose
headline capability is dormant pending an upstream-of-render downstream item.

## Non-blocking observations (not findings)

- `IfcMapReturn` attaches a `ProtoStoreInternal` before `set_output` where C++ relies
  on the `ProtoStoreSymbol` from `setScope` (the branch-wide W4 merged-load seam).
  Output-invisible (no datatest covers `map return`'s output text).
- `IfcTypeVarnode` skips `Scope::discoverScope` and binds to the function-local scope
  (W4 seam); for register storage this is the same arm C++ takes. The success echo
  renders an empty scope name ("Successfully added tmp to scope "). Console-only and
  gate-invisible — the harness matches `print C` text, not the `optr` echo (LOSS-164
  precedent). Folded into LOSS-165's surface notes.
- `parse_varnode`'s `None` (EOF) arm falls through where C++ `s.peek()` on EOF would
  enter `parse_machaddr`; a console-only error path with no datatest reach. Not a
  divergence on any tested input.
- porter unit test comment "4-byte register" describes an 8-byte model entry (the
  struct is 24 bytes so it overflows either way) — harmless comment typo.

## Adversarial tests

`rust/crates/kuna-decomp/tests/verify_w10_struct_return.rs` (8 tests, committed @ 10841d1):
Round 1 (carried, all pass):
- `hidden_return_emits_indirect_output_and_trailing_pointer_param`
- `void_return_is_single_unflagged_piece`
- `fitting_return_assigns_directly_with_no_hidden_param` (negative control)
- `hidden_return_pointer_size_tracks_default_data_space` (no-hardcode proof)
- `hidden_return_without_data_space_errors_cleanly`
Round 2 (F1 regression suite, mutation-checked):
- `pointer_unassignable_raises_param_unassigned_not_generic_lowlevel` — pins the F1
  fix; FAILS on the round-1 `lowlevel` mutation.
- `no_data_space_guard_is_not_param_unassigned_so_catch_arm_cannot_swallow_it` —
  proves the no-data-space guard stays a NON-catchable variant (catch arm cannot
  over-swallow a structural wiring failure).
- `shared_status_lets_second_assign_reattempt_same_entry_and_fail` — locks the
  shared-status reuse across both `assign_address` calls and the empty-`res`
  partial-state on the throw; FAILS on the round-1 mutation.

## Losses cited

- LOSS-165 — w10-struct-return: hidden-return / struct-return recovery NOT realized
  end-to-end; faithful foundation closes the retstruct/stackreturn exec-failure seam
  (−2) with ZERO new positive assertion (dormant-downstream render). Appended this
  round.
