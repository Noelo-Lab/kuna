# w10-console-family
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
round: 1
gate: KUNA_ENGINE=rust run_tests --datatests (count rises) -> 549->620 applied, 59->66 passing (monotonic, zero regression); cargo test --workspace -> 3336 passed / 0 failed; cargo clippy --workspace --lib -D warnings -> clean; C++ oracle -> 207/207 + 675/675 PARITY OK (untouched)

## Summary

The wave wires the console map/print/retype/rename/equate/force-format command
family plus the `option noreturn` flow-halting path, unblocking 8 of 10
EXEC-FAILURE datatest files so they decode+apply (EXEC-FAILURES 16->8) and
landing 7 new monotonic datatest passes (59->66) with ZERO regression. Every
transcription is FAITHFUL to its C++ source; there is NO special-casing; and the
7 new passes are REAL oracle matches (a strict subset of the C++ oracle's passes
on each file — never a coincidental substring). The cluster files are only
PARTIALLY parity (pointerrel 3/8, partialsplit 1/20, revisit 2/9 of each file's
assertions pass), but the remaining per-file FAILs are pre-existing
decompiler-engine gaps (relative-pointer `ADJ()`, partial-splitting,
delayed-alias SSA revisit) owned by other items, NOT defects in this wave's code.
Recorded as LOSS-170.

## Hunt list

- Signedness: clean. The `int4` parameter position (`s >> dec >> i` -> `read_int`)
  and the `int4` block/op indices in printRaw are read/compared faithfully.
- Integer widths: clean. `set_display_format` `format << 12` is `uint4`; the
  SeqNum `uniq` is `u32` rendered with `{:x}`; format codes are `uint4` 1..5.
- Wrapping: clean. `render_varnode_no_markup` uses `off.wrapping_sub(point_off)`
  matching C++ unsigned `loc.getOffset()-point.offset`. `retype_symbol` does no
  arithmetic that can wrap.
- Comparator totality: n/a (no new comparators).
- Iteration-order provenance: clean. printRaw's "Raw operations" path iterates
  `obank.iter_all()` (BTreeMap by SeqNum = C++ `beginAll`); the block walk uses
  `bblocks_get_block(0..n)` (the C++ `bblocks.list` order); `bb_ops` preserves op
  order. No HashMap/HashSet in ported logic.
- Off-by-one / do-while / reverse iterators: clean. The BlockGraph walk
  (`first`, then `1..n` with implied-goto between) mirrors C++
  `list.begin()` + `++iter` loop exactly.
- Erase-while-iterating: clean. `retype_symbol`'s addr-tied rebuild erases the
  single map entry (`erase_mapentry`), pops `mapentry`, zeroes `whole_count`,
  swaps the type, then re-adds the map point — matching ScopeInternal::retypeSymbol
  database.cc:2206-2222 step-for-step (single entry, no traversal).
- Exception -> Result partial-state parity: clean. The console commands `?`-
  propagate at the same points C++ throws (parse_type/parse_machaddr failure,
  unknown symbol/type), before any mutation; `readSymbol`'s namespace/global
  arms error VISIBLY rather than silently returning empty.

## Findings

- F1 (info, NOT a defect): the new passes are PARTIAL parity, by design. The C++
  oracle passes 9/9 revisit, 8/8 pointerrel, 20/20 partialsplit; the Rust branch
  passes 2/3/1 respectively (a strict subset). The new Rust passes
  (pointerrel #4/#5/#6 + partialsplit #4 positive `min=1` lines, revisit #8/#9 +
  condconst #4 negative `min=0` guards) are genuine — confirmed by dumping the
  Rust `print raw`/`print C` output (KUNA_DUMP) and matching the exact oracle
  lines (`ptrrel = &ptr->c;`, `ptr->d = 0;`) and by an e2e test that also pins the
  unported lines still FAIL honestly. The per-file FAILs trace to engine gaps
  owned by other items (relative-pointer ADJ, partial-split, alias-revisit /
  LOSS-131/132 M3 pipeline). Cited as LOSS-170, kind (C) honest-partial-parity.
       cpp: decompiler/cpp/typeop.cc (TypeOp*::printRaw family) — faithful render of a divergent SSA form
       rust: rust/crates/kuna-decomp/src/funcdata_printraw.rs

- F2 (low): two bare `as` casts lack the `// cast:` justification the verifier
  protocol mandates. Both are small-positive widenings that cannot misbehave (a
  register size; a validated `< num_spaces` count); the third cast in the same
  file IS commented. Style-only.
       cpp: decompiler/cpp/varnode.cc:743 (point.size), op.cc:46 (space index)
       rust: rust/crates/kuna-decomp/src/funcdata_printraw.rs:260, :593

- F3 (trivial): the `Datatype::set_display_format` doc comment lists "2=dec,
  4=oct, 8=bin, 16=char" but `encodeIntegerFormat` yields 1,2,3,4,5; the CODE is
  correct (stores whatever value is passed, `<< 12`), only the comment's example
  codes are wrong.
       cpp: decompiler/cpp/type.cc:732 (encodeIntegerFormat hex=1..char=5)
       rust: rust/crates/kuna-decomp/src/dtype.rs:970-972 (doc comment)

None of F1/F2/F3 is a blocker.

## Faithfulness spot-checks (all PASS)

- IfcMapParam/MapReturn/Mapconvert/Retype/Rename/PrintRaw/ForceDatatypeFormat:
  line-for-line vs ifacedecomp.cc (the `More than one symbol named :` vs `:`
  space asymmetry between Retype and Rename is reproduced; the CALL-with-space vs
  CALLIND-no-space one-byte difference is reproduced).
- condconst null-guard: the pre-existing `special`-flag filter (condconst.rs:354,
  faithful to coreaction.cc:4490) means every output-less op (CALL/STORE/branch,
  all carrying `PcodeOp::special` per typeop.cc) returns BEFORE the guard; the
  guard reaches the same "no constant pushed" C++ outcome and can never wrongly
  trigger. The C++ never actually derefs a null `getOut()`.
- flow noreturn: `query_call_no_return` derives the flag purely from the symbol's
  `no_return` (set by `OptionNoReturn` resolving the function by NAME), looked up
  by ADDRESS — no hardcoding; the artificial-halt planting matches
  checkForFlowModification flow.cc:659-665 (the warning Comment + inline arm are
  documented seams).
- printRaw operator-name size suffixes: ZEXT/SEXT/SUBPIECE = name+in0+out, PIECE =
  name+in0+in1, CARRY/SCARRY/SBORROW = name+in0 — all match getOperatorName.
- SeqNum uniq-in-hex: faithful to the `Address::printRaw` stream-base carry-over,
  correct for BOTH callers (printRaw + IopSpace::printRaw); the old decimal was a
  latent bug, and the full suite + byte-parity tests confirm no caller regressed.
- retypeSymbol / setAttribute / addEquateSymbol(getBase(1,UNKNOWN)) /
  setDisplayFormat(clear force_format, OR format<<12): all faithful.

## No special-casing

Confirmed clean: ZERO hex literals in the added src lines; ZERO
function-name/address/case-value/magic-divisor hardcoding in src/. The corpus
names (`myptroff`/`ptrrel`/`__stack_chk_fail`/`mystruct`) appear ONLY in
test/`#[test]` modules. The hex/dec/oct/bin/char -> format-code mappings are
verbatim C++ `encodeIntegerFormat`/`IfcMapconvert` transcriptions.

## No regression

Diffed the base (merge-base cd44e73, 549/59) vs branch (620/66) per-assertion
Success sets: every base Success is still a Success on the branch (empty
regression set). boolless/readstruct/condconst_conn byte-parity preserved
(print_b5_boolless 11/11, verify_w10_const_prop_phi 6/6, verify_w10_struct_corpus
16/16; the Conditional-Constant assertion set is byte-identical base==branch).
cargo test --workspace 3336/0. C++ oracle 675/675 PARITY OK (no C++/specs change).

## Adversarial tests (8 added, committed)

rust/crates/kuna-decomp/tests/verify_w10_console_family.rs (4 unit):
  - seqnum_uniq_is_hex_not_decimal (uniq 0x2a/0xff/0 render hex, addr `0x`-prefixed)
  - constant_space_address_is_hex (the `#0xNN` constant token base)
  - display_format_bit_math_round_trips_and_replaces (codes 0..=5, replace not OR)
  - force_format_mask_holds_all_codes (the 3-bit 0x7000 field bounds)
rust/crates/kuna-harness/tests/verify_w10_console_family_e2e.rs (4 end-to-end):
  - pointerrel_retype_lines_are_real_parity (#4/#5/#6 Success, no EXEC-FAIL)
  - partialsplit_storeconst_line_is_real_parity (#4 Success via `option noreturn`)
  - revisit_printraw_negative_guards_pass_with_real_listing (#8/#9 over a real listing)
  - pointerrel_unported_body_still_fails_honestly (#1/#2/#3 still FAIL — anti-fake guard)
All 8 pass.

## losses

LOSS-170 (appended this review): w10-console-family — faithful command family
unblocks 8/10 EXEC-FAILURE files + 7 new monotonic passes, honest-partial-parity
(deeper bodies diverge via pre-existing engine gaps), two scoped console-command
seams (readSymbol namespace/global, attach_internal_store, noreturn warning
Comment).
