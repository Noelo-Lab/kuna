# w10-baseexplicit-piece
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent verifier subagent)
date: 2026-06-16 (round 1)
gate: cd /tmp/kuna-baseexplicit/rust && cargo test --workspace -> ok (0 failed); cargo test -p kuna-decomp --lib -> 1744 passed (== base, B0 unchanged); cargo clippy -p kuna-decomp --lib -> clean

## One-line

A line-faithful, non-special-cased port of the addrtied SUBPIECE-`overlapJoin` /
INT_ZEXT-`contains` / CPUI_PIECE-`findRoot`/`isPartialRoot` refinement block
(`ActionMarkExplicit::baseExplicit`, coreaction.cc:3120-3147) plus `PieceNode::findRoot`
(op.cc:854-883). The three arms and `find_root` match the C++ statement-for-statement,
the predicate is PURE addrtied/CONCAT geometry (no type/name/value keying), and the
render is BYTE-IDENTICAL to base across the entire 83-datatest corpus (zero regression).
The wave delivers NO new byte-match flip (the recombine temp stays EXPLICIT on both base
and branch because the addrtied/mapped flags that would make the new arms decisive are
set in a still-seamed varmap/dynamic layer) and carries one conservative-direction
control-flow order divergence. ACCEPT-WITH-LOSSES (LOSS-212).

## Branch / diff scope (reserved-file check: CLEAN)

- branch cf480f7, base (parent / merge-base with rust-port) 47059f6; ONE commit.
- `git diff rust-port...rport/w10-baseexplicit-piece --name-only` = 2 files, BOTH under
  `rust/crates/kuna-decomp/`:
  `src/coreaction_cleanup.rs`, `tests/verify_w10_baseexplicit_piece_adversarial.rs`.
- Edits ONLY `coreaction_cleanup.rs` (the required source file). NO reserved file
  touched: none of funcdata_merge.rs / merge.rs / variable.rs / dynamic.rs / database.rs
  / varmap.rs (the concurrent mergepiece-dynsym wave) appears in the diff.
- No non-rust file changed (C++ source / specs / baseline byte-untouched -> the C++
  oracle 207/207 + 675/675 PARITY OK is untouched by construction; spot-checked
  `decomp_test_dbg ... boolless.xml` -> "Total passing tests = 2").

## (1) FAITHFULNESS — line-by-line vs the C++ oracle: PASS (one order note, F1)

`find_root` (op.cc:854-883):
- `while(vn->isProtoPartial() || vn->isAddrTied())` -> `loop { if !(proto_partial||
  addr_tied) break; }` (faithful, at-least-zero, breaks on entry mismatch).
- the descendant walk snapshots `vn->beginDescend()..endDescend()` via `descend_snapshot`
  (a SmallVec in push_back/insertion order == C++ std::list order; NO HashMap/HashSet).
- `if (op->code()!=CPUI_PIECE) continue;` -> faithful; `slot=op->getSlot(vn)` -> `get_slot`
  (returns 0/1 for the PIECE input; `1-slot` is 1/0, never OOB).
- BIG-ENDIAN SLOT FIXUP `if (addr.getSpace()->isBigEndian()==(slot==1)) addr = addr +
  op->getIn(1-slot)->getSize();` -> `if addr.is_big_endian()==(slot==1) { addr = &addr +
  other_size as i64 }`; `is_big_endian` delegates to space, `Address + i64` == C++
  `operator+(int8)` (`base->wrapOffset(offset+off)`). FAITHFUL.
- `addr.renormalize(vn->getSize())` -> `addr.renormalize(vn_size, manage)` (IPTR_JOIN-only
  mutation, no-op otherwise) — see F4 for the error-path nuance.
- `if (addr == vn->getAddr())` -> `if addr == vn_addr` (full Address eq).
- compareOrder TIEBREAK `if (pieceOp!=0) { if (op->compareOrder(pieceOp)) pieceOp=op; }` ->
  `if pcodeop_compare_order(...) != 0 { piece_op = Some(op) }`. POLARITY CORRECT: C++ uses
  the result as a truthiness test (non-zero == any defined order, -1 OR 1), and the Rust
  transcribes `!= 0` exactly; only a 0 (no absolute order) keeps the prior pieceOp. The
  misleading "earliest" comment is in BOTH the C++ and the Rust doc; the Rust mirrors the
  CODE, which is what matters. `pcodeop_compare_order` (block.rs:3398) is the faithful
  parent/start-order/findCommonBlock transcription (op.cc:808).
- `if (pieceOp==0) break; vn = pieceOp->getOut();` -> faithful.

`base_explicit` addrtied block (coreaction.cc:3120-3146):
- SUBPIECE `overlapJoin` pre-check (3121-3127) runs FIRST inside `if(isAddrTied)`, BEFORE
  loneDescend, only when `def->code()==SUBPIECE`; inner `if(vin->isAddrTied())` then
  `if(vn->overlapJoin(*vin)==def->getIn(1)->getOffset()) return -1;`. Rust mirrors the
  exact order and only EARLY-RETURNS on match, else FALLS THROUGH to lone_descend.
  `overlap_join`/`overlap` (varnode.rs:1327-1351) and `Address::overlap[_join]`
  (address.rs:303-327) are faithful big/little-endian transcriptions. The `oj as uintb ==
  off1` widening matches C++ `int4 == uintb` integral promotion (sign-extend then
  reinterpret); the negative-overlap (`oj==-1`) case correctly yields `0xFFFF..` != small
  offset == C++ (no false match).
- INT_ZEXT `contains` (3130-3134): `if ((!vnout->isAddrTied())||(0!=vnout->contains(*vn)))
  return -1;` else fall through. Rust: `if !out_addr_tied { return -1 } if 0 != contains {
  return -1 }` else fall through. POLARITY CORRECT (`!addrtied || contains!=0 -> -1`).
  `contains` receiver/arg order CORRECT (`outv.contains(vv)` == `vnout->contains(*vn)`);
  `Varnode::contains` (varnode.rs:1235) is the faithful -1/0/1/2/3 transcription.
- CPUI_PIECE (3135-3143): `rootVn=findRoot(vn); if (vn==rootVn) return -1; if
  (rootVn->getDef()->isPartialRoot()) return -1;` else fall through. Rust mirrors;
  `vn==root_vn` is slotmap-key eq == C++ pointer eq; `is_partial_root` (op.rs:799) reads
  the `concat_root` addlflag. NULL-DEF GUARD correct (toward EXPLICIT/-1, the conservative
  side — F1-adjacent but here it matches the C++ invariant that the root has a def).
- else arm (3144) `return -1;` -> faithful.

## (2) NO SPECIAL-CASING: PASS

- grep of the ADDED production lines for `float|float8|float10|wide|metatype|get_type|
  typeclass|datatype|is_float|get_name|register|value` (whole-word) -> the only hits are
  in DOC-COMMENTS ("wide-float PIECE recombine temp", "wide-value PIECE recombine case");
  ZERO predicate keys on a type, function name, variable name, register, or value. The
  refinement is PURE addrtied/CONCAT geometry (`is_addr_tied`/`is_proto_partial`/opcode/
  `overlap_join`/`contains`/`find_root`/`is_partial_root`).
- no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/`partial_cmp` in the diff.

## (3) NEW PASSES — REAL? : NO NEW FLIP (honest non-gain, F2/LOSS-212)

- C++ B5 oracle `doublemove::loaddouble` renders `return glob1 + glob1;` (recombine temp
  IMPLIED). BOTH base (47059f6) AND this branch render `float8 v1; ... v1 = dat_424318 +
  dat_424318; return v1;` (temp EXPLICIT) — i.e. the new arms are faithful but currently
  NON-DECISIVE: the function does NOT byte-match the oracle on either branch, and the
  branch produces NO new byte-match. The recombine temp stays explicit because the
  addrtied/mapped flags (and the global `map addr` naming) that would route execution into
  the new ZEXT/PIECE-implied fall-through are owned by the still-seamed varmap/dynamic
  layer. The diff's own adversarial tests correctly assert DETERMINISM + non-regression,
  NOT a new byte-match — so no fabricated pass is claimed.
- The branch therefore does NOT advance the 331 datatest count via this change; it lays a
  faithful substrate that activates when the flag-setting layer lands.

## (4) NO BROAD REGRESSION (the key risk): PASS — REGRESSED SET EMPTY

- WHOLE-CORPUS render diff: rendered every function of all 83 datatests on BOTH the base
  (47059f6, with the same built `.sla`) and the branch; the two outputs are BYTE-IDENTICAL
  (the only diff lines were cargo's "running for over 60 seconds" framing + the elapsed-ms
  number). The REGRESSED SET IS EMPTY and there is no silent over-inline anywhere in the
  corpus.
- boolless / condconst_conn explicitly verified byte-identical to base AND structurally to
  the C++ oracle: boolless keeps `v1 = dat_52;` + `return v1;` (the addrtied accumulator
  stays EXPLICIT, NEVER `return dat_52;`); condconst_conn keeps `int4 v1; ... if (...) v1 =
  ...; return v1;` with the compare connective intact. (readstruct is not a datatest in
  this corpus; the structurally-equivalent struct/recombine carriers — retstruct,
  piecestruct, longdouble — are all in the byte-identical set.)
- INT_ZEXT contains-polarity correct (see (1)); PIECE/ZEXT null-def guards point toward
  EXPLICIT not implied (conservative); over-inline impossible in the corpus.

## Mandatory hunt list

- Signedness: clean. `slot==1`/`1-slot` int4 (0/1); `get_in(int4)` indexes `as usize` but
  slot is provably 0/1 for a PIECE input; `oj as uintb == off1` matches C++ int4==uintb.
- Integer widths: clean. `other_size as i64`, `off1 as int4`, `oj as uintb` all match the
  C++ promotions (int4->int8 in `addr+sz`; int4==uintb). [F3: missing `// cast:` token.]
- Wrapping: clean. The only arithmetic is `Address::operator+` -> `wrap_offset(offset.wadd
  (off as u64))` (faithful uintb wraparound).
- Comparator totality: clean. `pcodeop_compare_order` returns -1/0/1, used only as `!=0`.
- Iteration-order provenance: clean. `descend_snapshot`/`descend_iter` over a SmallVec in
  push_back order == C++ std::list `beginDescend()..endDescend()`. No HashMap/HashSet.
- Off-by-one / do-while: clean. `find_root`'s `while(...)` -> `loop{ if !(..) break }`
  (at-least-zero, faithful). No reverse iterators.
- Erase-while-iterating: N/A — `find_root`/`base_explicit` do not mutate the banks; the
  caller (`ActionMarkExplicit::apply`) pre-collects candidates into a Vec before the
  `&mut Funcdata` `base_explicit` calls (sound; baseExplicit only reads).
- Exception->Result partial-state: F4 — `Address::renormalize` returns `KunaResult<()>`
  and the port does `let _ = addr.renormalize(...)` (swallows the `LowlevelError` C++
  would PROPAGATE out of findRoot/baseExplicit). C++ throws only for an IPTR_JOIN address
  with no covering JoinRecord; on a non-join addr it is a no-op (the common case). Swallow
  leaves addr unchanged -> a non-matching address simply isn't chosen (conservative).
  Unobservable on the corpus (whole-corpus byte-identity). Faithful-enough; recorded.

## Findings

- F1 (low, faithfulness / order — conservative direction): the `def->code()==CPUI_PIECE &&
  def->getIn(0)->isProtoPartial()` check is, in C++, an `else if` IN THE SAME CHAIN as the
  addrtied branch (coreaction.cc:3158), so it is evaluated ONLY when vn is NOT addrtied /
  mapped / protoPartial. The Rust places it as an UNCONDITIONAL block AFTER the
  `if addr_tied {} else if mapped {} else if proto_partial {}` chain, so in the addrtied
  ZEXT-implied / PIECE-implied FALL-THROUGH cases it is ALSO evaluated. Effect: Rust may
  return -1 (EXPLICIT) where C++ would continue to the desccount tail (returning a positive
  count == implied-eligible) — i.e. Rust is MORE explicit in this corner (the SAFE
  direction; NEVER over-inlines). Not exercised by the 83-datatest corpus (whole-corpus
  render byte-identical). A real but inert order deviation.
       cpp: decompiler/cpp/coreaction.cc:3158 (the `else if`)
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:741-751 (unconditional block)
- F2 (informational / loss): no new byte-match flip; the addrtied PIECE/ZEXT-implied arms
  are faithful but non-decisive on the current corpus (the deciding flags live in the
  seamed varmap/dynamic layer). doublemove::loaddouble stays EXPLICIT on both base and
  branch, divergent from the oracle's IMPLIED `return glob1 + glob1;`. -> LOSS-212.
       cpp: decompiler/cpp/coreaction.cc:3135-3143 (oracle path)
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:695-727
- F3 (low, process): three bare `as` casts (`other_size as i64`, `off1 as int4`,
  `oj as uintb`) are SEMANTICALLY correct (match the C++ promotions, with nearby
  explanatory comments) but lack the protocol's literal `// cast:` justification token.
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:55, 167, 169
- F4 (low, exception->Result): `let _ = addr.renormalize(...)` swallows the LowlevelError
  C++ would propagate; see hunt-list. Conservative, unobservable on the corpus.
       cpp: decompiler/cpp/op.cc:868 (renormalize call) + translate.cc:858 (throw)
       rust: rust/crates/kuna-decomp/src/coreaction_cleanup.rs:75

## Adversarial tests (verifier, >= 3, committed to MAIN tree)

`rust/crates/kuna-decomp/tests/verify_w10_baseexplicit_piece_verifier.rs` (3 tests, all
PASS against the ported code in the feature worktree):
- `w10be_verifier_longdouble_pass_non_addrtied_wide_deterministic` — V1: the 80-bit-float
  carrier `longdouble::pass` (x86-64) exercises the new arms with the addrtied predicate
  false; render is deterministic + brace-matched (no panic in find_root/contains).
- `w10be_verifier_storedouble_second_concat_geometry_deterministic` — V2: a SECOND wide
  CONCAT geometry (`doublemove::storedouble`, stack+const re-piece) renders deterministically
  (find_root compareOrder tiebreak / descend-snapshot leak no nondeterminism).
- `w10be_verifier_condconst_conn_addrtied_carrier_unperturbed` — V3: the addrtied-register
  carrier `condconst::condconst_conn` renders deterministically with its `if`/compare
  structure intact (byte-identical to base; structurally == the C++ oracle).

(The porter's own 3 tests — wide_float_piece_path_is_deterministic, boolless_addrtied_
accumulator_unperturbed, single_bit_constant_is_bare_literal — were also re-run and PASS.)

## losses

LOSS-212 (F1 conservative order divergence + F2 honest non-gain / no new flip).
