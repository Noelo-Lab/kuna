# w6-s4-coreaction-protos
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> item lib suite (1048) + 6 item unit tests + 5 verifier tests PASS; the only failure across the crate is `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix`, an environment-only missing-`.sla`-artifact failure (pre-exists on `rust-port`, requires `make specs`, touches no path this item owns).

## What this item is

A **SEAM transcription** port of the 13 S4 prototype-recovery `Action` classes from
`coreaction.{cc,hh}`: `ActionPrototypeTypes`, `ActionDefaultParams`,
`ActionExtraPopSetup`, `ActionFuncLink`, `ActionFuncLinkOutOnly`, `ActionParamDouble`,
`ActionActiveParam`, `ActionActiveReturn`, `ActionReturnRecovery`, `ActionRestrictLocal`,
`ActionInputPrototype`, `ActionOutputPrototype`, `ActionPrototypeWarnings`.

Only the class shells are realized: exact `name()`/group/`flags` strings,
`clone_filtered` (mirrors C++ `clone(grouplist)`), the `proto_actions` enumeration, and
the single realized control-path `ActionExtraPopSetup`'s `stackspace==NULL` early-return.
Every `apply` body is a no-op returning `0`, with the C++ logic carried verbatim as a
`// SEAM(W7/W8-funcdata)`-noted comment block (same iteration order, tie-breakers, and
`count += 1` points). This is the established `coreaction_early` convention; the sibling
`early_actions` helper does exactly the same.

I confirmed the SEAM premise is true, not an excuse: `Funcdata::funcp` is the empty
`seams::FuncProto` placeholder (funcdata.rs:157, type at seams.rs:183 `pub struct
FuncProto;`); `Funcdata` has no `numCalls`/`getCallSpecs` (qlst omitted, funcdata.rs:130),
no `getActiveOutput`/`initActiveOutput`/`clearActiveOutput`, no `getOverride`. The
realized accessors the comments name (`getFirstReturnOp` funcdata_op.rs:1214, `isHighOn`/
`isDoublePrecisOn`/`getScopeLocal` funcdata.rs) exist but are only entry conditions inside
the seamed work, so no change can be applied. All 13 transcriptions were re-derived
line-by-line against the C++ (`coreaction.cc` 1452/1490/1565/1619/1632/1641/1769/1817/
1880/1954/2003/2369/4824/4843/4941/4999/5140) and match.

## Hunt list

- **Signedness.** Clean. The only realized arithmetic-adjacent code is the
  `stackspace.is_none()` early return; no `int4` vs `uint4`/`uintb` comparison exists in
  the realized surface (all such logic is deferred inside the comment blocks).
- **Integer widths.** Clean. No realized integer arithmetic; `stackspace: Option<i32>`
  replaces C++ `AddrSpace *` and is only null-tested.
- **Wrapping.** Clean. No realized `uintb`-lineage arithmetic. (The C++
  `wrapOffset(spacebaseOffset+addr.offset)` in `ActionRestrictLocal` and the
  `getExtraPop()` add in `ExtraPopSetup` are inside the SEAM comment, not executed.)
- **Comparator totality.** N/A — no comparator/sort in this item's realized surface.
- **Iteration-order provenance.** Clean. The only realized iteration is `proto_actions`
  building a `Vec` in C++ definition order (and `all_actions`/test maps). No `HashMap`/
  `HashSet` anywhere (grep-confirmed). The C++ per-call `for(i=0;i<numCalls();++i)`
  vector walks are transcribed in the comments with their order noted.
- **Off-by-one / do-while / reverse iterators.** Clean. No realized loop with boundary
  math; the C++ `j -= 1` re-check in `ParamDouble` and the `buildReturnOutput`
  index walks are commented, not executed.
- **Erase-while-iterating.** Clean. None in the realized surface.
- **Exception -> Result parity.** Clean. The realized `apply` bodies cannot throw; the
  C++ `LowlevelError` sites (`ActiveParam` rethrow, `funcLinkOutput` unique-output error,
  `PrototypeWarnings` warnings) are all inside the SEAM comments. Verifier test asserts
  `PrototypeWarnings` emits nothing (the warning channel IS realized, so a future leak is
  caught).
- **Aliasing.** Clean (`AliasChecker.gather` in `ActiveParam` is seamed).

## Findings

  - F1 (minor, non-blocking): `proto_actions` docstring says it enumerates "in C++
    definition order ... for the W8 universalAction assembler", but the real
    `universalAction` registers these actions interleaved with other items, in distinct
    groups (`base`/`protorecovery`/`noproto`/`localrecovery`/`fixateproto`) and across
    `actmainloop`/`actfullloop`. The flat single-group `proto_actions` is a per-item
    enumeration, not the pass order; the sibling `early_actions` docstring states this
    caveat explicitly while `proto_actions` omits it. Non-blocking because every body is a
    no-op and `proto_actions` has no caller yet; recorded in LOSS-094's restorer NOTE so
    W8 assigns the true group/loop rather than trusting the flat order.
       cpp: decompiler/cpp/coreaction.cc:5740-6030 (universalAction registration)
       rust: rust/crates/kuna-decomp/src/coreaction_protos.rs:954-976 (proto_actions)

## Mechanical pass

- `cargo run -p port-audit -- --item w6-s4-coreaction-protos`: NOT RUN — no `port-audit`
  crate exists in this worktree's workspace (protocol-tooling absence, not an item defect).
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare `as`: none in
  `coreaction_protos.rs` (the `as ` grep hits are all doc-comment prose: "as appropriate",
  "as parameters", "as locals", "as a formal part").
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean. Test file clippy `-D
  warnings`: clean.

## Adversarial tests

Added `rust/crates/kuna-decomp/tests/verify_w6_s4_coreaction_protos.rs` (5 tests, all
pass), targeting the SEAM-specific fragile invariants:

- `every_proto_action_apply_is_inert` — every action (incl. `ExtraPopSetup(Some(idx))`
  falling through the realized null-check) returns 0 and leaves `count == 0`; the warning
  sink stays empty. A spurious `count += 1` would falsely drive the repeat-apply fixpoint.
- `repeated_apply_stays_inert` — re-applying a SEAM 5x stays at count 0 (no hidden state).
- `clone_filtered_total_and_preserves_identity` — for all 13 (+the extra ExtraPopSetup),
  in-group clone keeps name/group/flags; out-of-group clone is `None`.
- `proto_actions_flags_match_cpp_constructors` — every `proto_actions` entry carries its
  exact C++ flags word (`rule_onceperfunc` vs `0`), and `extrapopsetup` stays excluded.
- `extrapop_null_and_nonnull_both_inert_for_now` — pins the realized-vs-seam boundary;
  trips when a future W8 wiring makes `Some(idx)` actually mutate.

## losses

LOSS-094 (the entire `apply` behavioral surface of all 13 actions is a SEAM transcription
gated on the W7/W8 `Funcdata`<->call-spec/proto bridge; only shells + the ExtraPopSetup
null-guard are realized). F1 (the `proto_actions` order/group caveat) is recorded inside
LOSS-094's restorer NOTE.
