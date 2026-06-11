# w3-ir-block
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus (independent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (87 unit + verify suites green; 0 failed)

## Scope
C++: `decompiler/cpp/block.cc` (3774 LOC) + `block.hh` (930 LOC), blob shas
matched against the checklist (`aa154772…` / `1cae714e…`). Rust:
`rust/crates/kuna-decomp/src/block.rs` (2942 LOC, single file) +
`src/block/tests.rs` (21 porter tests). The port realizes the C++ pointer web as
a `slotmap` `BlockArena` keyed by `BlockId` (ADR 0001); every edge primitive that
touches two blocks is a `BlockGraph` method over the arena, preserving the C++
two-sided aliasing (`a->addInEdge(b)` mutates both endpoints). The subtype
hierarchy is collapsed into one `FlowBlock` struct + a `BlockKind` payload enum
(op.rs convention), and `get_type()` is a `match` on `kind`.

## Hunt list
- **Signedness.** clean (1 low note). Edge/index loops use `int4` slots and
  `len() as int4`; every signed/unsigned C++ compare was re-derived. The C++
  `slot < intothis.size()-1` (signed-vs-unsigned, size_t) is ported as
  `(slot as usize) < len() - 1` — equivalent given the len>=1 precondition every
  caller guarantees (they index `[slot]` first). `isJumpTarget`'s `index-1`,
  `findIrreducible`'s `preorder.size()-1`, `findBlock`'s `max=size-1` all match
  on the empty/zero edge (verified `find_level_block` on empty resolver returns
  None — adversarial test). One LOW note: `decodeNextInEdge`'s
  `outofthis.size() <= reverse_index` is unsigned in C++ but signed in Rust
  (`len() as int4 <= rev`); diverges only for a *negative* serialized
  reverse_index (corrupt/hostile input), never for kuna's own output. Not a
  blocker.
- **Integer widths.** clean. `int4→int4`, `uint4→uint4`, `uintb→uintb`
  (`CaseOrder.label`). No promotion-sensitive arithmetic — block.cc computes on
  small indices and bitwise label flags, not on wrap-prone `uintb` values.
- **Wrapping.** clean. No `uintb`-lineage arithmetic in scope; `CaseOrder::compare`
  uses only `!=`/`<` on the label. No bare arithmetic that can wrap on valid input.
- **Comparator totality.** clean. `compare_block_index` is a strict `<` on a
  unique `index`; `CaseOrder::compare` is total (`label` then `depth`, both
  integer `Ord`). No float comparisons anywhere.
- **Iteration-order provenance.** clean. Zero `HashMap`/`HashSet`. Every loop is
  over a `Vec` (the C++ `vector<FlowBlock*> list`/`intothis`/`outofthis`) walked
  by index, or over a cloned snapshot of `list` (the C++ `for(iter=list...)`),
  preserving insertion/reverse-post order. `BlockMap` uses a `Vec` + stable
  `sort_by` (indices unique → tie behavior irrelevant vs C++ `std::sort`).
- **Off-by-one / do-while / reverse iterators.** clean. `find_common_block_set`
  ports the C++ `do {} while(bl!=0)` as `loop{...; if bl.is_none(){break}}`
  keeping at-least-once semantics; `structure_loops` ports `do {} while
  (needrebuild)` the same way; `find_irreducible` walks `preorder` in reverse
  (`xi = len-1; while xi>=0`); `find_spanning_tree`'s `for(repeat=0;repeat<2)`
  is a `loop` with `if repeat==1 panic` matching the C++ throw on the 2nd pass.
- **Erase-while-iterating.** clean. `half_delete_{in,out}_edge` ports the C++
  slide-down + mirror `reverse_index -= 1` faithfully (adversarial test
  `half_delete_corrects_mirror_reverse_index` removes a *middle* in-edge whose
  mirror lives among several out-edges and asserts the whole-graph mirror
  invariant). `self_identify` / `eliminate{In,Out}Dups` keep the C++
  "don't-increment-i on redirect" idiom. `identify_internal` mark/clear-and-rebuild
  `list` matches. `removeFromFlow` reroutes via `intothis[0].reverse_index`
  repeatedly (adversarial test confirms all in-edges land on the single out).
- **Exception → Result partial-state.** clean. `LowlevelError` throws →
  `KunaError::lowlevel` `Err(?)` at the same sites (`set_goto_branch`,
  `force_false_edge`, `splice_block`, `new_block_if_goto`, `get_start_block`,
  `BlockEdge::decode`); the two unrecoverable C++ throws in `find_spanning_tree`
  / `calc_forward_dominator` (internal-invariant "Could not generate spanning
  tree" / "Problems finding root node") become `panic!` per ADR 0004. Partial
  state at the throw points matches (each is a guard before mutation, or a
  terminal invariant).
- **SeqNum/create_index allocation-order.** n/a to block.cc (no SeqNum/create_index
  here); block `index` is reverse-post-order assigned by the spanning tree, and
  the arena insertion order is irrelevant (handles, not indices, are the keys).
  `pcodeop_compare_order` reads `get_seq_num().get_order()` exactly as C++
  `start.getOrder()`.
- **Intrusive-list splice-order.** clean. `BlockGraph::list` is a `Vec<BlockId>`;
  `add_block`/`set_start_block`/`identify_internal`/`build_copy` preserve C++
  push/slide/rebuild order. `splice_block` flag inheritance (`fl1|fl2`) and
  out-edge transfer order verified by adversarial test.

## Dominator / spanning-tree re-derivation (the high-risk core)
- `find_spanning_tree`: rootlist swap-to-front/back, `origrootpos` handling,
  the irreducible-edge skip, tree/back/forward/cross labeling, `rpostcount`
  decrement, `numdesc` accumulation, and the 2-pass extraroots rebuild all match
  line-for-line. `rpostorder` holes panic (stricter than C++ null) but cannot
  occur (every node visited once).
- `find_irreducible` (Tarjan): reverse-preorder walk, `copymap` FIND/UNION,
  `reachunder` worklist, the `(x.vc > yp.vc) || (x.vc + x.numdesc <= yp.vc)`
  irreducibility test, tree-edge-needs-rebuild, and collapse-into-x all match.
- `calc_forward_dominator` (Cooper/Harvey/Kennedy): the `postorder[numnodes-i]`
  layout, the two virtual-root creation branches, `b->immed_dom=b`, the
  finger1/finger2 intersection (`numnodes - idom->index`), the changed-fixpoint
  loop in `postorder.len()-2 .. 0`, and the virtualroot excision all match.
  `postorder` is `vec![default; listlen]` fully overwritten (no default leak).
- `build_dom_tree`/`build_dom_depth`/`build_dom_sub_tree`/`dominates`/
  `find_common_block`(both) match (adversarial + porter diamond/loop/irreducible
  tests pass).

## encode / decode re-derivation
- `encode`/`encode_header`/`encode_body`/`encode_edges` order matches; the
  subtype `<target>` extras (`BlockGoto`/`BlockMultiGoto`/`BlockIf`-goto) are
  emitted *after* the recursive component encode, exactly as the C++
  `BlockGraph::encodeBody(); ...targets...`. MultiGoto omits `ATTRIB_TYPE`
  (matches), Goto/If write it as `writeUnsignedInteger` (matches). `body_type_name`
  reproduces the `ifgoto`/`properif`/`ifelse` size distinction.
- `decode`/`decode_graph_body` interleave decode+addBlock per component (matches),
  `BlockMap::create_block`→`sort_list`→`find_level_block` binary search matches
  `findBlock` (`max=size-1`, `min<=max`). Nested-graph round-trip + unknown-endpoint
  rejection verified by adversarial tests.

## Mechanical pass
- `cargo run -p port-audit -- --item w3-ir-block`: tool **not present** in the
  workspace yet (`package port-audit not found`); blob-sha drift checked manually
  against the checklist instead (match).
- grep: `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` → **none** in
  block.rs. Bare `as` casts: 153, **none** carry `// cast:` — consistent with the
  accepted standard for this port (op.rs has 58 bare `as`, 0 comments, ACCEPTed at
  w3-ir-op); all are mechanical index conversions (`i as usize`, `len() as int4`)
  or widening writes (`as i64`/`as u64`), no value-narrowing casts.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean. (`--tests` surfaces one
  pre-existing doc-lazy-continuation warning in `verify_w3_ir_op.rs`, not in scope.)

## Findings
- F1 (loss / major-by-precedent): ~25 public/virtual `block.cc`/`block.hh`
  methods are seam-deferred (printing W8; Funcdata/Varnode dataflow W7; JumpTable
  switch family W4/W7) with **no `losses.md` entry** at submission — the same
  ledger-accountability gap the w3-ir-block module is in scope for, identical to
  w3-ir-op finding F3 (closed by LOSS-030). Resolved by the verifier appending
  **LOSS-033**. No ported function depends on a deferred one (verified). Downgraded
  to ACCEPT-WITH-LOSSES on that basis.
       cpp: decompiler/cpp/block.cc:709 (compareFinalOrder), :1907 (newBlockSwitch), :404 (restrictedByConditional), :842 (findCondition), :2546 (nextInFlow) + the print/emit family
       rust: rust/crates/kuna-decomp/src/block.rs:45-73 (Deferred surfaces doc; methods absent)
- F2 (low): `decode_next_in_edge` compares `outofthis.len() as int4 <= rev`
  (signed) where C++ promotes to unsigned (`size() <= reverse_index`); differs
  only on a negative serialized `reverse_index`, unreachable from kuna's own
  encode output. Defensive-only; not a blocker.
       cpp: decompiler/cpp/block.cc:89 (`while(inedge.point->outofthis.size() <= inedge.reverse_index)`)
       rust: rust/crates/kuna-decomp/src/block.rs:2593 (`while (…outofthis.len() as int4) <= rev`)

## Adversarial tests added
`rust/crates/kuna-decomp/tests/verify_w3_ir_block.rs` (committed on branch, 7 tests, all green):
- `w3_ir_block_half_delete_corrects_mirror_reverse_index`
- `w3_ir_block_remove_from_flow_reroutes_all_inedges`
- `w3_ir_block_splice_block_inherits_flags_and_out_edges`
- `w3_ir_block_remove_from_flow_split_noflip_maps_parallel`
- `w3_ir_block_blockmap_empty_resolver_misses_cleanly`
- `w3_ir_block_nested_graph_encode_decode_roundtrip`
- `w3_ir_block_edge_decode_rejects_unknown_endpoint`

## losses
LOSS-033 (block.cc/block.hh structuring/printing/dataflow methods seam-deferred at w3-ir-block).
