# w10-convert17-equate-rebind — independent verifier, round 2: ACCEPT_WITH_LOSSES

verdict: ACCEPT_WITH_LOSSES
verifier: independent-verifier (Opus 4.8 [1m], round 2)
date: 2026-06-17
branch: rport/w10-convert17-equate-rebind @ 2bbfd5f (1 commit over merge-base 79e51bd = rust-port-base@400/675)
worktree: /tmp/kuna-convert17 (sla present, 148); merge-base harness rebuilt at /tmp/kuna-baseline (79e51bd, 400/675)

## Decision

ACCEPT_WITH_LOSSES. This is the faithful **round-2 re-spin** of the item REJECTED in round 1
(LOSS-226). The round-1 single dispositive blocker — the `copy_symbol`/`copy_symbol_if_valid`
SymbolEntry-copy primitives landing on the RESERVED `funcdata.rs` (+67 lines) — is **RESOLVED**:
those two methods are relocated to the non-reserved `funcdata_op.rs`, and `funcdata.rs` is now a
0-line diff. The diff touches exactly **3 rust files** (funcdata_op.rs, ruleaction_3.rs, printc.rs),
**none on the reserved set**. Net **+1 → 401/675**, regressed-set EMPTY, Convert #17 genuinely
passes, no special-casing, hash math untouched, C++ oracle untouched. The sole residual is a
coordination loss: the one-line `printc.rs` flip is on a file the two open printc waves contend (not
a reserved-file edit on this charter → not a REJECT, but must coordinate at merge).

## Round-1 blocker — RESOLVED

Reserved files all 0-line diff (independently confirmed):
funcdata.rs, funcdata_block.rs, blockaction.rs, coreaction_infertypes.rs, funcdata_spacebase.rs,
varmap.rs, coreaction_casts.rs — **all 0 diff lines**. Also 0-diff: dynamic.rs, coreaction_cleanup.rs,
database.rs. The `copy_symbol` + `copy_symbol_if_valid` methods now live in `funcdata_op.rs` (non-
reserved). The whole diff is rust/-only: funcdata_op.rs (+73), printc.rs (+14/-1), ruleaction_3.rs
(+228 incl. tests).

## Everything verified independently (re-run, not from porter notes) — all GREEN

### Faithfulness — equate re-bind/re-find IS faithful; calc_hash UNCHANGED
- **`PcodeOp::collapse` markedInput (op.cc:473-495).** Line-faithful: `getIn(0)->getSymbolEntry()!=0`
  sets markedInput on every path; `getIn(1)` checked only on the binary arm. Rust `dc_collapse` sets
  `marked_input` from `kuna_symbol_entry()` on in0 (always) and in1 (binary only) — matches.
- **`PcodeOp::collapseConstantSymbol` (op.cc:527-564).** Line-faithful opcode switch: SUBPIECE requires
  `getIn(1)->getOffset()==0` (high-byte trunc) else early-return; COPY/ZEXT/NEGATE/2COMP/LEFT/RIGHT/
  SRIGHT → in0; ADD/MULT/AND/OR/XOR → in0, falling to in1 when in0 unmarked; default → return; then the
  `copyVn->getSymbolEntry()==0 → return` guard; then `copySymbolIfValid`. Rust `collapse_constant_symbol`
  reproduces each arm exactly incl. the low-byte-SUBPIECE early return and the "first input must be
  marked" guard.
- **RuleCollapseConstants call site (ruleaction.cc:3891-3894).** `if (markedInput) { op->
  collapseConstantSymbol(vn); }` ↔ Rust `if marked_input { collapse_constant_symbol(op, vn, data); }`.
  Faithful.
- **`Varnode::copySymbol`/`copySymbolIfValid` (varnode.cc:512-541).** Faithful via the merged-tree
  SymbolEntry stand-in: copy_symbol parks the Symbol id on dst and mirrors onto the High for an EQUATE;
  copy_symbol_if_valid pulls the EquateSymbol value then gates on `equate_is_value_close`. Documented,
  sound deviation: the Rust does NOT copy `type=vn->type` (the equate's type is getBase(1,TYPE_UNKNOWN);
  the render facet rides the Symbol, so skipping the retype avoids a spurious size-1 retype). Behavioral
  faithfulness confirmed by the empty-regressed-set + Convert #17 pass.
- **op_set_input copySymbol seam (funcdata_op.cc:111-113).** `Varnode *cvn=newConstant(...); cvn->
  copySymbol(vn); vn=cvn;` ↔ Rust `let cvn=self.new_constant(...); self.copy_symbol(cvn, vn); vn=cvn;`.
  Same-size, so no value-close gate (matches C++). Faithful.
- **`EquateSymbol::isValueClose` (database.cc:641-658).** `equate_is_value_close` is PRE-EXISTING,
  0-diff in this branch; the value-close gate (==, ~, -, +1, -1; sign-ext mask guard) is reused as-is.
- **printc `doEmitWideCharPrefix() && sz>1` (printc.cc:1417, doEmit==true :1572).** The lone runtime
  `format_integer_token` call (printc.rs:4835) now passes the wide-char-prefix param `true` instead of
  `false`; `format_integer_token` already preserves the `wide_char_prefix && sz > 1` gate. Restores the
  `L` on a size>1 force_char constant. Faithful.
- **`dynamic.rs` / `calc_hash`: 0-line diff.** Hash math UNCHANGED. The fix is the constant's
  re-BINDING across the size-fold, never the hash.

### No special-casing
Whole-diff grep of added production (non-comment, non-test) lines for the convert constants/hashes
(`0x61`, `0x721eecc7`, `0x20c20f516a`, `fffffe00`), callee names (`recv_signed`/`recv_unsigned`),
`"convert"`, and size magic (`==4`/`==8`/`get_size()==`): ZERO hits in logic. The only `!= 0` is the
faithful SUBPIECE `getIn(1)->getOffset() != 0` high-trunc check. The forbidden literals appear only in
the 3 adversarial-test fixtures. The mechanism is generic over opcode/symbol-id/value geometry.

### Real parity (re-ran convert.xml on the Rust harness)
`./decomp_test_dbg -sleighpath specs -path decompiler/datatests datatests`:
- **Convert #17 = Success** — stringmatch `recv_signed\(L'a'\)` min=1 max=1 (a real regex requiring
  exactly one occurrence; NOT a vacuous min=0 pass). This is the char-equate surviving the size-4 fold
  and rendering with the wide-char prefix — the branch's exact target.
- Remaining convert FAILs {#2,#6,#10,#14} are the documented B3 signed-negate render seam in the
  reserved printc dispatch (b1b2 LOSS-220 #2), unchanged from base. Honest.

### No regression (the decisive check)
Built `decomp_test_dbg` at BOTH the merge-base (79e51bd, rebuilt fresh) and the branch; full
passing-set `comm` both directions:
- **merge-base = 400/675; branch = 401/675.**
- **REGRESSED set (passing@base, failing@branch) = EMPTY.**
- **GAINED set = exactly {Convert #17}** — single targeted assertion, zero collateral.
- displayformat (Display format), partialunion (Partial union), the other 16 convert equates,
  boolless, readstruct (Read struct), condconst (Conditional constant): ALL byte-identical
  (passing-set diff empty for each family).
- `cargo test --workspace` (independently run): **3653 passed / 0 failed.** The 3 adversarial tests ran
  + passed (`w10_equate_survives_size4_fold_via_collapse_subpiece`,
  `w10_non_folding_equate_still_rebinds_on_copy`, `w10_value_close_gate_drives_rebind_not_special_case`).
- `cargo clippy -p kuna-decomp --lib` (forced rebuild of changed files): clean.
- C++ oracle / `specs` / `docs/baseline.json`: untouched by construction (diff is rust/-only) →
  C++ 675/675 PARITY OK preserved; B0 unchanged.

### ≥3 adversarial tests — present, ran, passed; genuinely adversarial
- #1 `w10_equate_survives_size4_fold_via_collapse_subpiece`: size-8 equate folds to size-4 via SUBPIECE
  high-trunc → re-binds + recovers FORCE_CHAR; **plus a negative control** (low-byte SUBPIECE trunc →
  must NOT propagate). The convert #17 mechanism + its opcode-gate negative.
- #2 `w10_non_folding_equate_still_rebinds_on_copy`: same-size INT_ADD collapse re-binds FORCE_HEX —
  format follows the Symbol, not hardcoded.
- #3 `w10_value_close_gate_drives_rebind_not_special_case`: negation re-binds (isValueClose true), a
  distant constant (0x12345) does NOT — proves the gate keys on the equate value, not a hardcoded
  constant/name.

## Losses (see losses.md, LOSS-227)

1. **printc.rs one-liner on a contended file.** The `doEmitWideCharPrefix()` false→true flip (+14/-1)
   is faithful but lands on `printc.rs`, contended by the two open printc waves (b1b2 LOSS-220 #2).
   NOT a reserved-file edit on this charter (printc.rs not in the reserved set) → not a REJECT; must be
   coordinated with those waves at merge to avoid a textual collision.
2. **Charter-named edit surface vs actual.** The charter named "dynamic.rs/database.rs + tests"; the
   real (faithful) seam is funcdata_op.rs/ruleaction_3.rs/printc.rs + tests. dynamic.rs & database.rs are
   both 0-diff. Disclosed; not a defect.

## Anchors

- cpp-anchor: op.cc:473-495 (`PcodeOp::collapse` markedInput); op.cc:527-564
  (`PcodeOp::collapseConstantSymbol` switch + copySymbolIfValid); ruleaction.cc:3891-3894
  (RuleCollapseConstants `if(markedInput) collapseConstantSymbol`); varnode.cc:512-541
  (`Varnode::copySymbol`/`copySymbolIfValid`); funcdata_op.cc:111-113 (op_set_input copySymbol seam);
  database.cc:641-658 (`EquateSymbol::isValueClose`); printc.cc:1417/1572 (`doEmitWideCharPrefix()
  && sz>1`, doEmit==true). dynamic.cc:340 (`DynamicHash::calcHash`) — confirmed NOT involved;
  dynamic.rs 0-line diff.
