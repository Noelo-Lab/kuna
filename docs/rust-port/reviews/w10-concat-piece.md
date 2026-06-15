# w10-concat-piece
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
gate: `cargo test --workspace` (worktree /tmp/w10-concat-piece) -> 3343 passed / 0 failed;
      `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean;
      `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK

## Scope

Branch `rport/w10-concat-piece` (1 commit, 45a8e51). Rust-only diff. Ports:
- `PieceNode` + `isLeaf`/`gatherPieces` (op.cc:822-906) into `op.rs`,
- `RulePieceStructure::{determineDatatype, convertZextToPiece, findReplaceZext,
  separateSymbol, applyOp}` (ruleaction.cc:7496-7733) into `ruleaction_6.rs`
  (`spanningRange` was already ported in the W5 batch and is unchanged),
- `Varnode::getStructuredType` (varnode.cc:1156) into `varnode.rs`,
- a `MergeContext::gather_pieces` bridge (funcdata_merge.rs) + the
  `CPUI_PIECE -> (binary,"CONCAT")` w6_type_op entry (funcdata_op.rs).

The prompt's broader rule list (RuleConcatZext / RulePiece2Zext / RulePiece2Sext /
RuleConcatShift / RuleSubZext / RuleSubCommute) is NOT in this diff — those were
delivered in earlier waves; this item is the `RulePieceStructure` + `PieceNode`
foundation only. Verified accordingly.

## Verdict rationale

The transcription is FAITHFUL and contains NO special-casing, but the
structured-piece render is DORMANT end-to-end (zero new positive datatest
assertion) because the live analysis pipeline still no-ops on the struct corpus
(LOSS-131/132 dual-AddrSpaceManager gap), plus five conservative W4/W8 seams.
This is the established LOSS-156/158/159/160/162/163/165 dormant-foundation
pattern -> ACCEPT-WITH-LOSSES, ledgered as LOSS-168.

## Mandatory hunt list

- Signedness: CLEAN. `sz as usize > size_of::<uintb>()` (ruleaction_6.rs:938)
  faithfully reproduces C++ `sz > sizeof(uintb)` — for a real INT_ZEXT sz>0; an
  (impossible) negative sz wraps to a huge usize and bails, exactly as the C++
  signed->unsigned promotion bails. `size()` returns int4 (i32), so the
  subtraction is i32, matching C++ int4. The `root_big_endian == (i==1)` bool
  comparison (op.cc:900) is reproduced exactly.
- Integer widths: CLEAN. The 5 numeric casts (`rel_offset as i64`,
  `sz as usize`, `offset as int8`, `base_offset as i64`,
  `get_type_offset() as i64`) are all faithful int4->wider widenings for
  `Address` arithmetic / size compares; none narrow or change the C++ result.
- Wrapping: CLEAN. The only arithmetic is `&base_addr + offset as i64` /
  `&root_addr + rel_offset as i64` (Address operator+, the C++ idiom) and
  `base_offset + other.size`; offsets/sizes are small non-huge int4 — no wrap.
- Comparator totality: N/A (no ordering/comparator ported in this item).
- Iteration-order provenance: CLEAN. `gather_pieces` is a recursive append to a
  Vec in `for i in 0..2` order = C++ `vector<PieceNode>` push order (DFS:
  parent slot 0, recurse, ..., slot 1). The applyOp main loop iterates
  `stack.clone()` in index order = C++ `for(i=0;i<stack.size();++i)`. No
  HashMap/HashSet in the ported logic. Adversarial test at2 pins the DFS order
  AND the parent-relative offset accumulation (mutation-checked).
- Off-by-one / do-while / reverse iterators: CLEAN. The `for(;;){ gather;
  if(!findReplace) break; clear; }` loop is reproduced as
  `loop { clear; gather; match { true=>continue, false=>break } }` — I traced
  the clear-placement (C++ clears at END after a true; Rust clears at START of
  each iter): equivalent in all three cases (first iter, true-continue,
  false-break-with-populated-stack). The `convertZextToPiece` / `spanningRange`
  `while ct->getSize() > sz` / `for(;;)` subtype walks are faithful.
- Erase-while-iterating: CLEAN. The applyOp body mutates the graph (insert COPY,
  replace/delete varnode) while iterating the PRE-BUILT cloned stack; each node
  re-reads its live `getVarnode()` (= pieceOp->getIn(slot)) exactly as C++, and
  distinct (op,slot) pairs mean a prior node's input rewrite never disturbs a
  later node's read. Parent nodes are processed before children (DFS), and a
  parent's rewiring touches the GRANDPARENT edge, not the child read.
- Exception -> Result partial-state: ACCEPTABLE. The port adds `Err(_) =>
  return 0` / `... => continue` bails where C++ has no error (e.g.
  `addr.renormalize`, `op_set_input`). `renormalize` is a no-op for non-JOIN
  addresses (the corpus case), so the added Err path is unreachable there; the
  bails follow the established seam-decline pattern. No partial-state divergence
  on the corpus.

## Special-casing scan (REJECT trigger — none found)

`git diff rust-port...rport/w10-concat-piece -- rust/` grepped for
fn-name / datatest-address (0x100740/0x1006fa) / `v1.a` / `.arr` / magic-divisor /
type-name hardcoding in src: NONE. The only literal address (`0x100`) is a
`#[test]` fixture pc. The `piecestructure:`/`gather_pieces:` strings are panic
messages, not logic. The `CPUI_PIECE -> (binary,"CONCAT")` entry is a verbatim
`TypeOpPiece` opflags transcription (typeop.cc:2039), required for the
`opSetOpcode(zext, CPUI_PIECE)` callsite — not a special-case.

## Mechanical pass

- No `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the src diff.
- All `.unwrap()` in the diff are in `#[test]` fixtures.
- 5 bare numeric `as` casts (no `// cast:` prefix — a minor style nit, NOT a
  correctness finding): all faithful int4->i64/usize/int8 widenings for Address
  arithmetic, each adjacent to its C++-quoting comment.
- `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean.
- (`cargo run -p port-audit` is N/A — no port-audit crate exists in this repo.)

## Findings

- F1 (loss, accepted): structured-piece render DORMANT — `RulePieceStructure`
  never fires on a live CONCAT tree (LOSS-131/132 pipeline no-op); 0/6
  `piecestruct.xml` positive assertions pass, base==branch.
    cpp: decompiler/cpp/ruleaction.cc:7640
    rust: rust/crates/kuna-decomp/src/ruleaction_6.rs:1043
- F2 (loss, accepted): five W4/W8 conservative declines (getStructuredType
  mapped-partial arm; determineDatatype partial geometry; piece_is_leaf
  mapped-vn; separateSymbol symbol-entry test; convertZextToPiece/COPY union
  resolution transfer). Each fail-closed.
    cpp: decompiler/cpp/varnode.cc:1160, ruleaction.cc:7503/7504, op.cc:834,
         ruleaction.cc:7616, ruleaction.cc:7576/7706
    rust: rust/crates/kuna-decomp/src/varnode.rs:600,
          rust/crates/kuna-decomp/src/ruleaction_6.rs:914/1007,
          rust/crates/kuna-decomp/src/op.rs:1437, ruleaction_6.rs:964
- F3 (note, not a finding): `PieceNode::findRoot` (op.cc:854) not ported; its
  callers (linkProtoPartial, coreaction.cc:3136) are W4/W7, out of scope.

No BLOCKER. No faithfulness break. No regression. No special-casing.

## No-regression evidence

- boolless B5 byte-parity: print_b5_boolless green
  (w10_boolless_full_byte_parity_modulo_type_inference passes).
- readstruct + condconst_conn byte-parity: full workspace green (the parity
  files are unchanged on the branch; only src for the piecestructure rule moved).
- The 5 corpus-wide positive datatest passes (boolless / condexesub / retspecial
  / skipnext2 / varcross) are unchanged base==branch — none touch piece-structure.
- C++ oracle 207/207 + 675/675 PARITY OK (no C++/specs touched).
- Full workspace `cargo test`: 3343 passed / 0 failed (>= the prior fleet count).

## Adversarial tests (committed)

rust/crates/kuna-decomp/tests/verify_w10_concat_piece.rs (5 tests, all pass):
- at1_gather_pieces_big_endian_offsets_are_mirrored  (mutation-checked: a
  hardcoded slot-0-gets-size predicate FAILS it)
- at2_gather_pieces_nested_tree_dfs_order_and_accumulated_offsets  (mutation-
  checked: passing base_offset into the recursion FAILS it)
- at3_piece_is_leaf_when_two_descendants_force_lone_descend_null
- at4_piece_is_leaf_addrtied_address_mismatch  (both directions)
- at5_spanning_range_overrun_returns_false

## losses

LOSS-168 (appended to docs/rust-port/losses.md).
