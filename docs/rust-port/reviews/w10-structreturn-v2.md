# w10-structreturn-v2
verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (Opus 4.8, round 2)
date: 2026-06-17

gate:
  - `cargo test -p kuna-decomp --test verify_w10_structreturn_v2` -> 5/5 PASS (AT1-AT5).
    AT5 (the Round-1 F1 regression test that FAILED against the shipped `< 0`) now PASSES.
  - `cargo test --workspace` -> 197 test-result lines, 3613 passed / 0 failed (ZERO failures;
    grep for `[1-9] failed` empty). >> the >=333 floor.
  - `cargo clippy --lib -p kuna-decomp` -> clean.
  - `cargo test -p kuna-decomp --test print_b5_boolless --test printc_parity` -> ok (11/0/1ign + 1/0).
  - `cargo test -p kuna-decomp --test verify_w10_struct_corpus --test verify_w10_struct_return
     --test verify_w10_condexe_structure_adversarial` -> ok (5/0/1ign + 19/0 + 8/0).
  - `cargo test -p kuna-decomp --test universalaction_listing` (B0) -> ok (1/0). B0 unchanged.
  - C++ oracle: diff is RUST-ONLY (no C++ / spec / python / baseline byte touched), so the recorded
    207/207 + 675/675 PARITY OK is structurally untouched.

scope: diff is `git diff rust-port...rport/w10-structreturn-v2` = two commits over merge-base 7b86a03
  (e1da274 the substrate; 1298f0f the Round-1 F1 tie-break fix). Touches ONLY:
  coreaction_cleanup.rs, coreaction_infertypes.rs, funcdata_varnode.rs, variable.rs, + the test
  file tests/verify_w10_structreturn_v2.rs. +604/-0. Rust-only.

reserved files: NONE touched. Verified each of coreaction_protos.rs, coreaction_stackptr.rs,
  funcdata_spacebase.rs, database.rs, varmap.rs, funcdata.rs, ruleaction_3.rs, ruleaction_1.rs,
  action.rs is ABSENT from the changed-file list. CLEAN.

special-casing hunt: CLEAN. Whole-word grep of the added production lines for struct-name /
  field-name (a/b/A/B) / function-name string keying and magic-offset / value keying -> NONE.
  The only numeric literal in new prod code is `slot == 1` (the C++ PIECE slot-0/slot-1
  big-endian convention, op.cc:866 — structural, not special-casing). The magic-offset grep hits
  (coreaction_infertypes.rs:1110/1860/1959/2000, funcdata_varnode.rs:600/2162/2571) are all
  PRE-EXISTING lines outside the added hunks. No HashMap/HashSet/sort_unstable/todo!/unimplemented!.

round-1 finding status:
  - F1 (REJECT-class in round 1): FIXED. `piece_find_root`'s multi-PIECE tie-break now reads
    `if self.op_compare_order(op, prev) != 0 { piece_op = Some(op); }` (funcdata_varnode.rs:1273) —
    the faithful transcription of the C++ literal predicate `if (op->compareOrder(pieceOp)) pieceOp = op;`
    (op.cc:871). `compareOrder` returns -1/+1/0 (op.cc:808); the C++ `if(non-zero)` REPLACES the
    incumbent on BOTH -1 and +1 and keeps it only on 0 (incomparable). `!= 0` reproduces this;
    `< 0` (the round-1 ship) replaced only on -1. The C++ comment "Attach this to earliest one" is a
    known upstream comment quirk — the CODE replaces on any defined order; the port faithfully
    follows the code, not the comment. Verifier AT5 (drives the production `Funcdata::piece_find_root`
    end-to-end through a 2-PIECE same-block +1 case) now PASSES.
  - cast nit (round 1, non-correctness): CLOSED. `other_size as i64` carries a `// cast:`
    justification (funcdata_varnode.rs:1255-1257).

hunt list:
  - Signedness: clean. `establish_group_symbol_offset` keeps C++ `int4 off` and the two `off < 0`
    checks verbatim (variable.cc:628,631). `slot >= 1` == C++ `slot != 0` for non-negative slots
    (RETURN seed) / `slot == 1` PIECE convention. `op_compare_order != 0` is the 3-valued-result fix.
  - Integer widths: clean. `int4` throughout; the single widening `other_size as i64` (i32->i64) is
    lossless/sign-preserving and now `// cast:`-justified.
  - Wrapping: clean. `off -= piece.getOffset()` is bounded ([0,symboloffset] minus a small
    non-negative group offset) — cannot overflow i32; the C++ throw-on-negative is faithfully `Err`.
    `&addr + other_size as i64` routes through the existing `Address + i64` operator. The
    `sym_off = piece_group_off + group_sym_off` add is the C++ `piece->getOffset() +
    group->getSymbolOffset()` (variable.cc:259) on small bounded offsets — no wrap.
  - Comparator totality: clean (post-fix). `pcodeop_compare_order` returns -1/0/1 (block.rs:3398),
    a faithful `PcodeOp::compareOrder` (op.cc:808); used as `!= 0` per op.cc:871.
  - Iteration-order provenance: clean. `descend_snapshot` preserves the C++ std::list
    `beginDescend`..`endDescend` order exactly (funcdata_varnode.rs:976); `name_local_highs_angr`
    iterates the pre-existing local order and dedups via an ordered set. NO HashMap/HashSet introduced.
  - Off-by-one / do-while / reverse iterators: clean. `piece_find_root`'s `loop{...break}` keeps the
    C++ `while(proto||tied)` pre-test (guard re-checked at top each iteration). `renormalize` Err ->
    `continue` matches the C++ "skip on bad join addr" path. `bb == null` arm of the RETURN seed is
    implicit (a live RETURN always has a parent), documented.
  - Erase-while-iterating: N/A (no erase during traversal in scope).
  - Exception->Result partial-state: clean. `establish_group_symbol_offset` returns `Err` BEFORE
    writing `group.symbol_offset` (matches the C++ throw-before-setSymbolOffset ordering,
    variable.cc:631-633 — no partial mutation); `bind_proto_partial_piece` bails to the normal `vN`
    allocator (the round-trip fall-through) on the Err, never binding a bogus offset.

findings: none open. (Round-1 F1 resolved; cast nit resolved.)

  FAITHFUL (no finding):
  - `HighVariable::establishGroupSymbolOffset` (variable.cc:623) -> variable.rs:1440 — line-faithful
    (clamp symboloffset>=0, subtract piece offset, Err on <0, write group symbol offset).
  - `TypeOpReturn::getInputLocal` RETURN type-seed (typeop.cc:903-921) -> coreaction_infertypes.rs:97
    — faithful (slot>=1, non-VOID output type, size==input size, else generic `getBase(size,UNKNOWN)`;
    De-Morgan of the C++ `VOID || size!=` guard). The added `has_store()` guard is a documented W4
    SEAM precondition that only gates WHETHER to consult the output type (cannot inject a wrong type).
  - `Funcdata::linkProtoPartial` / `linkSymbol` proto-partial arm (funcdata_varnode.cc:1153,1180) ->
    coreaction_cleanup.rs `bind_proto_partial_piece` + the name_local_highs_angr branch — faithful to
    the kuna naming model (the `getSymbol()!=0` early-return is realized as the caller's
    `kuna_name().is_some()` skip; the `setSymbolEntry(sym->getFirstWholeMap())` stand-in stamps
    root-name + root struct-type + `piece.getOffset()+group.getSymbolOffset()` per variable.cc:259;
    pieces skip the `vN` allocator only when bound, the root keeps its `vN`).
  - `PieceNode::findRoot` (op.cc:854-883) -> `Funcdata::piece_find_root` (funcdata_varnode.rs:1189) —
    faithful (big-endian slot fixup `is_big_endian()==(slot==1)` then `+ getIn(1-slot)->getSize()`,
    `renormalize` Err->continue, the `!= 0` compareOrder tie-break).
  - `HighVariable::isSameGroup` (variable.hh:306) -> variable.rs:1427 — faithful (null piece -> false;
    same group compare). The Rust `ga.is_some() && ga == gb` is a defensive equivalent of the C++
    `piece->getGroup() == op2->piece->getGroup()` (returns false on a stale-piece None rather than
    a spurious `None==None` true).

no-regression: CONFIRMED.
  - Struct-corpus render byte-identical base(7b86a03 = merge-base)<->branch: all 675 datatest
    assertion lines identical, positive set unchanged at 21 (established by the Round-1 full-SLA
    corpus byte-diff; the Round-2 delta over Round-1 is ONLY the F1 `< 0`->`!= 0` and the `// cast:`
    comment, which affect `piece_find_root` ONLY when 2+ valid PIECE descendants of a proto-partial
    leaf exist — a case the dormant corpus never reaches, so the render delta remains zero).
  - boolless (incl. BE 8051 path), readstruct, condconst_conn: byte-identical (print_b5_boolless +
    struct corpus + condexe pass on the branch).
  - 6 switch datatests (ifswitch/switchhide/switchind/switchloop/switchmulti/switchreturn): identical
    renders (the struct-return path is dormant; condexe structure adversarial passes 8/8).
  - C++ oracle 675/675 + 207/207 PARITY OK (no C++/spec/baseline touched). B0 listing unchanged.
  - `cargo test --workspace` green (3613 passed / 0 failed); `cargo clippy --lib` clean.
  - REGRESSED SET EMPTY; whole-suite passing 3613 >= 333.

adversarial tests (5 committed in rust/crates/kuna-decomp/tests/verify_w10_structreturn_v2.rs):
  - at1_whole_symbol_root_pieces_offset_equals_group_offset (porter) — PASS
  - at2_nonzero_root_symbol_offset_shifts_all_field_keys (porter) — PASS
  - at3_incompatible_root_symbol_offset_is_err_not_panic (porter) — PASS
  - at4_groupless_high_is_noop (porter) — PASS
  - at5_findroot_tiebreak_must_replace_on_compareorder_nonzero_not_only_negative (VERIFIER, round 1)
    — now PASS: drives the production `Funcdata::piece_find_root` end-to-end for a 2-PIECE same-block
    +1 case; asserts the faithful `!= 0` replacement (root walks to `out_late`). Failed against the
    round-1 `< 0` ship; passes against the round-2 fix. This is the F1 regression guard.

losses: LOSS-215 (appended). A faithful PARTIAL: the RETURN type-seed + linkProtoPartial piece->root
  symbol binding + establishGroupSymbolOffset + PieceNode::findRoot substrate is line-faithful (F1
  fixed) but DORMANT on the current corpus — the `v1.A`/`v1.B` field render it is built for does not
  yet materialize because the upstream RulePieceStructure split (which produces the per-field
  proto-partial pieces this code binds) is not reached on any corpus function (its stack-var /
  HighVariable recovery prerequisites are seamed). ZERO regression, render dormant — the LOSS-197 /
  LOSS-198 / LOSS-212 / LOSS-214 dormant-substrate pattern. Restoration: the field render fires when
  RulePieceStructure + its prerequisites land; the ported substrate needs no edit.
