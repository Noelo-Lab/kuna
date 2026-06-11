# w2-sleigh-symbol

## Round 1

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_6343a4ba-016-9 (worktree `.claude/worktrees/wf_6343a4ba-016-9`, reviewed at 162ccfa; adversarial tests committed as d7b824c)
cpp blob shas: both in-scope files match the checklist entry (`git hash-object` verified: slghsymbol.cc deadf3b5, slghsymbol.hh e3be244b)

gate: `cd rust && cargo test -p kuna-sleigh` -> ok. 72 passed (lib, incl. 16
new slghsymbol tests) + 6 (`verify_w2_sleigh_context`) + 6
(`verify_w2_sleigh_pattern`) + 6 (verifier tests
`verify_w2_sleigh_symbol`), 0 failed.
clippy: `cargo clippy -p kuna-sleigh -- -D warnings` (protocol command) ->
clean; `--all-targets` flags one test-module lint, see F3.
port-audit: `cargo run -p port-audit -- --item w2-sleigh-symbol` NOT
runnable — no `port-audit` crate exists in the workspace (carried from every
prior review); blob-SHA check performed manually instead (both match).

## Method

Re-derived the semantics of every ported function in `slghsymbol.{hh,cc}`
(the full 2549+638 lines; the unported compiler half re-derived far enough
to confirm it is compiler-only) and reviewed the item diff (commit 162ccfa:
`rust/crates/kuna-sleigh/src/slghsymbol.rs` +4546; the branch also carries
the already-reviewed w2-sleigh-context and w2-sleigh-pattern merges,
excluded from this review's scope).  The 24 sla FORMAT_SCOPE attribute ids
and 42 element ids defined in `slghsymbol::sla` were checked
constant-by-constant against `decompiler/cpp/slaformat.cc` (all match name
and number; the pattern-wave re-export covers ATTRIB_VAL/OFF/INDEX/MASK/
SHIFT/STARTBIT and ELEM_INTB, spot-verified).

The load-bearing cross-module re-derivation of this review: the C++
`OperandSymbol::decode` attribute loop (slghsymbol.cc:1108-1127) fetches
`getNextAttributeId()` at the TOP of the loop body, so the first fetched
attribute is never processed.  This is correct only because the caller's
find-by-id read of ATTRIB_ID (`SymbolTable::decode`, slghsymbol.cc:213)
resets `curPos = startPos` after reading (marshal.cc:620-636, 941-947), so
the skipped attribute is ATTRIB_ID itself, and because unmatched peeked
attributes are skipped by the next `getNextAttributeId` (`attributeRead`
machinery, marshal.cc:811-825).  The Rust marshal (kuna-base, W1-verified)
replicates both halves (`marshal.rs:1541-1567` find resets,
`read_*_id` reset `cur_pos = start_pos`, `get_next_attribute_id`
skip-if-unread), and the port transcribes the loop shape verbatim with a
correct explanatory comment (slghsymbol.rs:3246-3267).  Pinned by
adversarial test T1 with the full optional-attribute set (SUBSYM + CODE),
which the porter's own tests never encode.

Seam contracts re-checked against the C++ bodies they stand in for:
`SymbolWalker`/`SymbolWalkerChange` against the exact ParserWalker surface
slghsymbol.cc touches; `SleighBaseTrans` against `getConstantSpace()` +
`ConstructTpl::decode/encode` (semantics wave); the in-progress
`TableResolver` against `OperandValue::decode`'s
`tab->getNumConstructors()` validation (slghpatexpress.cc:824-838) — C++
calls `addConstructor` BEFORE `ct->decode`, so the live count during
constructor N's decode is N+1, which `current: (self_id, ct_index + 1)`
reproduces exactly (slghsymbol.rs:3412-3423).  LOSS-021's deferred
`isConstructorRelative`/`getName` sliver is implemented here through the
symbol table exactly as that entry's restoration criteria proposed
(slghsymbol.rs:2827-2845 vs slghpatexpress.cc:800-812) — that sliver is
now closed.

## Hunt list

- signedness: clean. Every int4-vs-size_t / intb-vs-size_t / uintm-vs-int4
  mixed comparison is transcribed explicitly with a comment: the
  `max < table.size()` checkTableFill tests (negative max converts huge ->
  false; slghsymbol.rs:483,533,647), the `ind >= size()` resolve guards
  (rs:499,550,664-666), `secnum < namedtempl.size()` (rs:1286, negative
  sign-extends huge -> None), printBody's `i < printpiece.size()`
  (rs:1413), `id >= numct` in DecisionNode::decode as uintm (rs:1688-1691,
  pinned by adversarial T2: -1 truncates to 0xFFFFFFFF and is REJECTED —
  a signed comparison would have accepted it), scope/symbol-id bounds in
  the table decode (rs:2910,2916,2996,3003).  All `int4 i < vec.size()`
  loops became iterators/usize ranges over Vecs.
- integer widths: clean. uintm->u32, intb->i64, uintb->u64 throughout;
  every decode-side implicit C++ truncation (`intb`->`int4`/`uint4`,
  `uintb`->`uintm`) is a commented `as` cast at the read site; the
  `(uint4)getValue()` print/handle truncations and `(uintb)intb`
  reinterprets are commented (rs:2136,2143,2149,2162,2253,2262,2281); the
  `'A'+index` int->char truncation in addOperand/OPPRINT decode is `as u8`
  with comment (rs:1216,1542).  Note: C++ `piece[1]-'A'` sign-extends a
  signed char (index 63+ would go negative -> UB indexing); Rust's u8 read
  diverges only inside that UB region (>62 operands; real constructors
  have <10).
- wrapping: clean. The only arithmetic in ported scope that can wrap is
  calc_maskword's mask construction and ContextOp::apply's shift; both use
  `Wrap::wshl/wshr` (x86-masked counts) with range proofs for valid fields
  and documented UB-resolution for decoded out-of-range shifts
  (rs:364-380, 957-972).  ContextOp::apply's intb->uintm truncation BEFORE
  the shift is pinned by adversarial T4 (value -1, shift 28 ->
  0xF0000000; value 0x1_0000_0005 -> 5).  No checked/saturating
  mismatches found.
- comparator totality: clean. The single ordering in scope, `SymbolCompare`
  (`a->getName() < b->getName()`, std::string byte order), became
  `BTreeMap<Vec<u8>, u32>` — same total lexicographic byte order, same
  keep-existing-on-duplicate insert semantics (rs:2536-2570).
- iteration-order provenance, per loop: clean. symbollist/table loops are
  Vec-index order matching the C++ vectors (encode header pass then content
  pass IN ORDER, rs:2868-2879); scope iteration (`symbol_ids`) is BTreeMap
  key order == std::set\<SymbolCompare\> order; printpiece/operands/context/
  children/list/namedtempl/valuetable/nametable/varnode_table are all Vecs
  in C++ vector order; VarnodeListSymbol::get_size takes the FIRST non-null
  entry in table order (rs:680-690).  No HashMap/HashSet anywhere in the
  item.
- off-by-one / do-while / reverse iterators: clean. printMnemonic's
  `endind = firstwhitespace==-1 ? size : firstwhitespace` and printBody's
  start at `firstwhitespace+1` are exact (rs:1374-1387, 1410-1422; pinned
  by adversarial T3 through the flow-through path); skipScope's
  `while(i>0)` parent walk stops at the root as upstream (rs:2672-2683).
  The only do-while loops in the C++ scope (orderOperands,
  chooseOptimalField) are compiler-side and unported (F1).
- erase-while-iterating: n/a in ported scope — the C++ erase patterns live
  in purge/renumber/replaceSymbol (compiler-side, F1).  orderPatterns'
  insertion-shift loop likewise unported.
- exception -> Result partial-state parity: clean with one judged-equivalent
  divergence. addSymbol/addGlobalSymbol push onto symbollist BEFORE the
  duplicate-name error, exactly as C++ throws after push_back (rs:2697-2710,
  comment at site); decodeSymbolHeader validates id/uniqueness/scope in the
  C++ order before installing (rs:2996-3016); the scope-decode captures a
  null parent POINTER for forward references and keeps it null, replicating
  the C++ pointer-capture timing (rs:2921-2930).  Divergence: per-kind
  content decode (ValueMap/Name/VarnodeList/Context/Operand) reads into
  locals and commits only on full success, where C++ sets `patval` before
  the table loop (a mid-loop throw leaves C++ partial state).  Both engines
  abort the whole `SymbolTable::decode` on the first error and the table is
  discarded at every C++ catch site (translator construction), so no caller
  can observe the difference; judged equivalent, not a finding.
- UB boundaries (ADR 0004): C++ blind casts/derefs become typed errors
  (`symbol`/`varnode_symbol`/`operand_symbol`/`subtable_symbol`,
  `decode_pattern_value`, null-entry derefs) or documented indexing panics
  (`children[val]` in DecisionNode::resolve, the post-resolve
  `valuetable[ind]`/`nametable[ind]` indexing), each annotated "C++
  dereferences/indexes/casts unchecked" at the site.  Virtual dispatch via
  the actual symbol kind reproduces C++ vtable dispatch even where the
  stored pointer was blind-cast (e.g. Constructor::print on a non-operand
  id prints via that symbol's own print, as the C++ vtable would).

## Findings

- F1 (loss, accepted): the compiler half of slghsymbol is not ported:
  SymbolTable::purge/renumber/replaceSymbol, Constructor::buildPattern/
  orderOperands (+ pattern/pateq members), SubtableSymbol::buildPattern/
  buildDecisionTree (+ beingbuilt/errors), DecisionNode::split/
  orderPatterns/chooseOptimalField/getScore/getNumFixed/getMaximumLength/
  consistentValues/addConstructorPair (+ the parent back-pointer),
  DecisionProperties, the getVarnode() VarnodeTpl virtuals, the
  OperandSymbol compile-side flag setters, and the Macro/Label/Section/
  Bitrange symbol classes.  Consumers verified to split three ways:
  slgh_compile/slghscan (LOSS-001), pcodecompile.cc getVarnode (item
  w2-sleigh-semantics), and pcodeparse.cc:230,3249 LabelSymbol — the last
  IS runtime-linked (snippet parsing for injections) and is item
  w2-sleigh-pcodeparse's claimed scope; `SymbolType` keeps all discriminants
  so the enum extends without churn.
     cpp: decompiler/cpp/slghsymbol.hh:120-132,266-278,607-635;
          decompiler/cpp/slghsymbol.cc:123-143,281-369,1730-1891,1974-2342
     rust: rust/crates/kuna-sleigh/src/slghsymbol.rs:20-53 (module docs)
     -> LOSS-022 (appended by this review; also records that LOSS-021's
        isConstructorRelative/getName sliver is closed by this item).
- F2 (minor, non-blocking): duplicate subtable content elements diverge on
  malformed streams.  C++ `SubtableSymbol::decode` has no "already decoded"
  guard: a second content element for the same subtable id APPENDS to the
  live `construct` vector (ids continuing from N) and overwrites
  `decisiontree` (leaking the old one); the Rust decode rebuilds locally
  and REPLACES both.  Unreachable from encoder-produced streams (the C++
  encode loop emits each symbol's content at most once, and skips
  not-fully-formed subtables), so hostile-input-only; recorded as a note,
  not a loss.
     cpp: decompiler/cpp/slghsymbol.cc:1950-1972
     rust: rust/crates/kuna-sleigh/src/slghsymbol.rs:3327-3333,3400-3433
- F3 (minor, non-blocking): `cargo clippy -p kuna-sleigh --all-targets -- -D
  warnings` fails on `clippy::drop_non_drop` for the porter test-module's
  `drop(enc)` (slghsymbol.rs:3964).  The protocol command (without
  `--all-targets`) is clean and lib code is clean; cosmetic test-only lint,
  flagged for the next touch of the file.
     cpp: n/a
     rust: rust/crates/kuna-sleigh/src/slghsymbol.rs:3964
- F4 (note): Start/End/Next2/Epsilon/Varnode pattern expressions are
  stateless and built on demand, so `get_pattern_expression` on an
  undecoded SHELL returns `Some(...)` where the C++ shell holds a null
  `patexp` until content decode (Start/End/Next2).  Unreachable through
  ported call paths — expressions are only fetched from symbols after a
  successful full-table decode; documented in the module docs.
     cpp: decompiler/cpp/slghsymbol.cc:1193-1199,1258-1264,1323-1329
     rust: rust/crates/kuna-sleigh/src/slghsymbol.rs:2032-2065
- F5 (note): `SubtableSymbol::decode` reads ATTRIB_NUMCT only to discard it
  (C++ uses it for `construct.reserve(numct)`, which can throw
  length_error/bad_alloc on absurd hostile values where Rust proceeds and
  fails later, if at all).  Allocation-behavior-only divergence; harmless.
     cpp: decompiler/cpp/slghsymbol.cc:1953-1954
     rust: rust/crates/kuna-sleigh/src/slghsymbol.rs:3406-3407

## Mechanical pass detail

- grep over the item diff: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable` anywhere in slghsymbol.rs (shipped or test code).  Every
  bare `as` cast carries a justification comment at the line or in the
  immediately preceding comment block (same accepted style as the prior W2
  reviews); the handful flagged by a context-free grep (e.g. rs:377-378,
  483, 1216, 1688) all have their justification in the adjacent comment.
- sla ids: all 24 attributes + 42 elements verified against
  decompiler/cpp/slaformat.cc; the pattern-wave re-export supplies the
  shared ids without duplication.

## Adversarial tests

`rust/crates/kuna-sleigh/tests/verify_w2_sleigh_symbol.rs` (committed
d7b824c on the port branch; all 6 pass first run):

- verify_w2sym_operand_decode_all_attributes_and_byte_roundtrip
  (the double-fetch/marshal-reset interplay with SUBSYM+CODE present,
  plus whole-table byte-identical re-encode)
- verify_w2sym_decision_pair_id_truncation_boundaries
  (intb->uintm pair-id truncation: -1 rejected, ==numct rejected,
  ==numct-1 stored)
- verify_w2sym_flowthru_mnemonic_body_recursion
  (flowthruindex decode + printMnemonic/printBody delegation with balanced
  pushOperand/popOperand)
- verify_w2sym_contextop_apply_value_truncation
  (truncate-before-shift trace: -1<<28 -> 0xF0000000; 0x1_0000_0005 -> 5)
- verify_w2sym_add_syntax_coalescing_and_first_whitespace
  (the addSyntax push/append/drop rules and firstwhitespace capture)
- verify_w2sym_name_symbol_truncated_table_and_tab_reencode
  (checkTableFill "_"->TAB rewrite, truncated-table resolve rejections,
  and the attribute-less NAMETAB on re-encode)

losses: LOSS-022 (F1; appended by this review), LOSS-001 cited as the
parent compiler-cut decision, LOSS-021 cited as the sibling entry whose
isConstructorRelative/getName sliver this item closes.
