# w10-convert-b1b2 — independent verifier, round 2: ACCEPT_WITH_LOSSES

verdict: ACCEPT_WITH_LOSSES
verifier: independent-verifier (Opus 4.8 [1m], round 2)
date: 2026-06-17
branch: rport/w10-convert-b1b2 @ d7b7319 (2 commits over merge-base ea42fa0 = rust-port@349/675)
worktree: /tmp/kuna-convertb1b2 (sla present, 148); merge-base harness rebuilt at /tmp/kuna-verify-mb

## Decision

ACCEPT_WITH_LOSSES. This is the **faithful round-2 re-spin** of the item REJECTED in round 1
(round 1 edited the reserved `printc.rs` to land the B3 signed-equate render — see the prior content
of this file and LOSS-219). Round 2 **removes the reserved-file edit**: `printc.rs` is byte-untouched,
the B3 signed-negate render is honestly deferred as a SEAM, and the branch lands only the faithful
prereq + B1 (CALL-arg constant sizing) + B2 (copy-elim mapped-survival) substrate. Net **+22 →
371/675** against its own merge-base, **regressed-set EMPTY**, no special-casing, no reserved file
touched, `calc_hash` (dynamic.rs) byte-untouched. The charter grades a faithful prereq+B2 substrate
with B1 reported and 0 regression as ACCEPT_WITH_LOSSES; this delivers B1 actually landing on top.

## What I verified independently (not from porter notes)

### Faithfulness — B1, B2, prereq all faithful; calc_hash UNTOUCHED
- **B1 (CALL-arg constant sizing from the callee proto).** Line-faithful transcription of
  `ActionDefaultParams::apply` (coreaction.cc:2382-2390): `otherfunc!=0 → fc->copy(getFuncProto());
  if(!isModelLocked()&&!hasMatchingModel(evalfp)) setModel(evalfp); else setInternal(...)`. The callee
  locked `FuncProto` (a W4 callee-Funcdata reference the kuna console never builds) is the
  source-declared `parse line extern` prototype, parked on the global FunctionSymbol
  (`Database::set_function_proto_pieces`), snapshotted onto the seam keyed by `(space_index, offset)`
  (`build_callee_proto_pieces`), read back by ADDRESS (`Architecture::callee_proto_pieces`), and
  re-seeded via `FuncProto::seed_locked_from_pieces` + `fc.copy()`. The locked `int4` param then drives
  `TypeOpCall::getInputLocal` (typeop.cc:689-720, `fc->getParam(slot-1)->getType()`) to size the
  CALL-arg constant to 4 — the dynamic-hash input the canonical stored hashes were authored against.
  Generic — keyed only by address + declared pieces.
- **B2 (copy-elim honoring is_mapped).** `Varnode::set_kuna_symbol_entry` faithfully mirrors
  `setSymbolEntry` (varnode.cc:448): stores the SymbolEntry's owning Symbol id (the `mapentry`
  stand-in) and marks `Varnode::mapped`. The idempotency guard `vn->getSymbolEntry()!=0`
  (funcdata_varnode.cc:1346-1352 attemptDynamicMapping, :1379-1382 attemptDynamicMappingLate) is now
  keyed on the Varnode-level binding (survives heritage rebuilding the High), fixing the rule_repeatapply
  loop. The mapped bit is consumed by the EXISTING `is_mapped()` arm in the reserved
  `coreaction_cleanup.rs:533` (C++ coreaction.cc:3148 `vn->isMapped() → return -1`) — the branch sets
  the flag without editing that reserved file. Faithful.
- **prereq (DynamicSymbolSpec category/dispflags/equate_value carry).** Faithful: the kuna console
  rebuilds the IR on `decompile`, so an EquateSymbol added by `map convert` is re-seeded as
  category=equate with its forced display format intact (database.rs scope_dynamic_symbol_specs +
  funcdata.rs seed_dynamic_symbols via add_equate_symbol), so ActionDynamicMapping's equate arm fires
  and pushConstant reads the forced format.
- **calc_hash (dynamic.rs): 0-line diff.** The round-1 disproof stands — the bug was never the hash;
  it was the constant's input SIZE. dynamic.rs is byte-untouched. Confirmed.

### No special-casing
Whole-diff grep over added production (non-comment, non-test) lines for the convert constants/hashes
(`0x721eecc7`, `0x20c20f516a`, `fffffe00`, `0x61`), callee names (`recv_signed`/`recv_unsigned`),
`"convert"`, magic sizes (`==4`/`==8`): ZERO hits in logic. The only literal match
(`fd.new_constant(4, 0x100)`) is an arbitrary fixture constant inside the B2 adversarial test. The
mechanism is generic over declaration/scope/symbol-id geometry.

### Real parity (re-ran convert.xml; inspected the equate render vs oracle B5)
Independently ran the Rust `decomp_test_dbg` harness (`-sleighpath specs -path datatests convert.xml`):
- branch: **12/17** passing — EXACTLY {#1,#3,#4,#5,#7,#8,#9,#11,#12,#13,#15,#16}; failing EXACTLY
  {#2,#6,#10,#14 (signed-negate) + #17 (`L'a'`)}. Matches PROGRESS precisely.
- baseline (merge-base ea42fa0): **0/17**. convert.xml contributes +12 net itself.
- Render byte-inspected via `KUNA_DUMP=1`: passing cases byte-correct (`recv_signed(256)`,
  `recv_unsigned(768)`, `recv_signed(0777)`, `recv_signed(0b10001000)`, …). The 4 failures render the
  CORRECT value AND format, missing only the two's-complement sign flip — `recv_signed(4294966784)` vs
  `-512`, `recv_signed(0xfffff448)` vs `-0xbb8`, `recv_signed(037777777445)` vs `-0333`,
  `recv_signed(0b…0100)` vs `-0b1100110011001100`. #17 renders `recv_signed(0x61)` vs `L'a'`. EXACTLY
  the documented B3 reserved-printc.rs render seam + the char-equate-through-SUBPIECE residual. Honest.

### No regression (the decisive check)
Built `decomp_test_dbg` at the merge-base AND on the branch from their OWN HEADs; full per-assertion
`Success --` comm both directions:
- **merge-base ea42fa0 = 349/675** (matches the charter's "baseline 349").
- **branch = 371/675.**
- **REGRESSED set vs merge-base = EMPTY.** GAINED = exactly 22 {Concat #1/#3/#4, Concat split #1,
  Convert #1/#3/#4/#5/#7/#8/#9/#11/#12/#13/#15/#16, Pointer to array #5/#8/#10, Union #10/#11/#29}.
- displayformat (Display format ×4) and partialunion (Partial union ×5): unchanged. boolless (×2),
  Switch family (×48), Double precision (×6), Modulo (×78): all byte-identical vs merge-base.
- cargo test --workspace: **3631 passed / 0 failed** (independently run). clippy --workspace --lib:
  clean on a forced rebuild of the changed crates.
- C++ oracle 675/675 PARITY OK: untouched by construction — the diff touches ONLY rust/ + PROGRESS.md
  (no decompiler/cpp, no specs, no docs/baseline.json). B0 unchanged.

### Reserved files — all UNTOUCHED
`dynamic.rs` (0-line diff, calc_hash FAITHFUL), `coreaction_cleanup.rs` (f0-flag-v2 wave), `printc.rs`
(the round-1 violation — now byte-untouched), `coreaction_infertypes.rs`: all 0-line diff.

### ≥3 adversarial tests committed and passing
3 new `#[test]`, all RAN + PASS: `callee_proto_pieces_roundtrip_only_for_declared_callees` (database.rs,
B1 — declared-only, address-keyed, generic), `varnode_symbol_entry_binding_marks_mapped_and_is_idempotent`
(funcdata.rs, B2 — mapped pin + no-High idempotency key), `resolve_integer_signed_equate_negates_under_
forced_format` (printc/tests.rs free-function, the B3 substrate the seam feeds — does NOT touch the
reserved printc.rs render dispatch).

## Losses (see losses.md, LOSS-220)

1. **Merge staleness — must rebase before landing.** The branch is cut from merge-base ea42fa0 (349)
   and never received the f0-flag-v2 sibling wave that landed on rust-port HEAD 54c0afb (358). Against
   the CURRENT rust-port HEAD the diff shows 9 "regressions" {Double precision move #1/#4, If/Switch
   #3/#5/#6, Leading zeros count #1, Mixed float/int #2/#6, ModuloAlt #1} — but these are EXACTLY the
   +9 f0-flag-v2 GAINED (per its own merge commit 15d6f9c message), absent here only because the f0
   changes live in the reserved `coreaction_cleanup.rs` this branch correctly did not touch. NOT a
   masked regression; a rebase onto current rust-port resolves it (the two waves are orthogonal). Re-run
   the regressed-set diff post-rebase to confirm it stays empty.
2. **B3 signed-negate render deferred as a reserved-file SEAM (#2/#6/#10/#14).** The two's-complement
   negation for an equated signed `int4` (`pushConstant` `sign = ct->getMetatype()==TYPE_INT`,
   printc.cc:1818-1836 + the negation at :1381-1391) lives in `PrintC::pushVnExplicit`'s integer arm in
   the RESERVED `printc.rs` (actively contended by two open printc waves). The substrate this branch
   lands makes the `int4` read-facing type present on those constants, so the one-line dispatch unblocks
   the 4 assertions the moment it lands on a printc wave or via charter amendment.
3. **#17 `recv_signed(L'a')` residual seam.** A separate char-equate-through-size-4-SUBPIECE seam (the
   char equate's stored hash is size-8 but renders through a size-4 SUBPIECE) — not part of B1/B2/B3.

## Anchors

- cpp-anchor: typeop.cc:689-720 (`TypeOpCall::getInputLocal` — `fc->getParam(slot-1)->getType()` sizes
  the CALL-arg from the callee proto); coreaction.cc:2382-2390 (`ActionDefaultParams::apply` callee
  `fc->copy(otherfunc->getFuncProto())`); varnode.cc:448 (`setSymbolEntry` marks `Varnode::mapped`);
  coreaction.cc:3148 (`vn->isMapped() → return -1`, copy-elim B2 guard); funcdata_varnode.cc:1346-1352/
  1379-1382 (`attemptDynamicMapping`/`Late` equate arm + `vn->getSymbolEntry()!=0` idempotency);
  printc.cc:1818-1836/1381-1391 (B3 seam, NOT ported).
- rust-anchor (all OWNED/non-reserved): coreaction_protos.rs (B1 ActionDefaultParams arm),
  database.rs/architecture.rs/seams.rs (`set_function_proto_pieces`/`build_callee_proto_pieces`/
  `callee_proto_pieces`, address-keyed), funcdata.rs + varnode.rs (B2 `set_kuna_symbol_entry` →
  `Varnode::mapped` + Varnode-level idempotency key), ifacedecomp.rs (re-park `parse line extern` callee
  protos by name), printc/tests.rs (B3-substrate free-function unit test).

Recorded by the w10-convert-b1b2 independent verifier (round 2).
