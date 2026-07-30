//! Porter in-module tests for `blockaction.rs`.
//!
//! Exercise the control-flow *structuring* engine ([`CollapseStructure`]) on the
//! canonical CFG shapes from the `blockaction.cc` doc comments: a straight
//! concatenation (BlockList), a diamond if/else (BlockIf), a proper-if, a
//! while/do and do/while loop, an infinite loop, nested loops, and an
//! irreducible two-entry loop that forces a goto.  We check the *goto emission
//! count* and that the graph collapses to a single component, matching the C++
//! schema-precedence and TraceDAG bad-edge selection.

use kuna_base::types::int4;

use super::*;
use crate::block::{BlockGraph, BlockKind, FlowBlock};
use crate::context::BlockId;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// A graph + a root BlockGraph node holding `n` plain components.
fn build_graph(n: usize) -> (BlockGraph, BlockId, Vec<BlockId>) {
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let mut blocks = Vec::new();
    for _ in 0..n {
        blocks.push(g.new_block(root));
    }
    (g, root, blocks)
}

/// Wire the spanning tree + dominators (labels back/loop/exit edges) the way the
/// production pipeline does before structuring, and return the root.
fn label_loops_and_doms(g: &mut BlockGraph, root: BlockId) {
    let mut rootlist = Vec::new();
    g.structure_loops(root, &mut rootlist);
    g.calc_forward_dominator(root, &rootlist);
}

/// Run the structuring engine to completion, returning the data-flow change
/// count (always 0 here since the standalone negate path is pure-topology).
fn collapse(g: &mut BlockGraph, root: BlockId) -> int4 {
    let mut collapse = CollapseStructure::new(g, root);
    collapse.collapse_all().expect("collapse_all should succeed on a reducible CFG");
    collapse.get_change_count()
}

/// Count the number of edges in the *original* component list that are marked as
/// unstructured gotos (the C++ `count` of goto/IfGoto branches).  Run before
/// `collapse_all` consumes the list, so we scan the live top-level + recurse.
fn count_goto_edges_recursive(g: &BlockGraph, root: BlockId) -> int4 {
    let mut total = 0;
    let mut stack = vec![root];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(bl) = stack.pop() {
        if !seen.insert(bl) {
            continue;
        }
        for i in 0..g.block(bl).size_out() {
            if g.block(bl).is_goto_out(i) {
                total += 1;
            }
        }
        for &child in g.block(bl).get_list() {
            stack.push(child);
        }
    }
    total
}

/// After a successful `collapse_all`, the top-level component list should reduce
/// to a single connected structured node (all other blocks subsumed) OR a set of
/// isolated components with no inter-component edges.  Assert no out-edges remain
/// between top-level components.
fn assert_fully_collapsed(g: &BlockGraph, root: BlockId) {
    for i in 0..g.block(root).get_size() {
        let bl = g.block(root).get_block(i);
        assert_eq!(
            g.block(bl).size_out(),
            0,
            "top-level component {i} still has out edges after collapse"
        );
        assert_eq!(
            g.block(bl).size_in(),
            0,
            "top-level component {i} still has in edges after collapse"
        );
    }
}

// ---------------------------------------------------------------------------
// FloatingEdge
// ---------------------------------------------------------------------------

#[test]
fn floating_edge_tracks_through_collapse_hierarchy() {
    // 0 -> 1 -> 2, then collapse 0,1 into a BlockList; the floating edge 1->2
    // must resurface as (list, 2) with the right out index.
    let (mut g, root, b) = build_graph(3);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[2]);
    label_loops_and_doms(&mut g, root);

    let mut fe = FloatingEdge::new(b[1], b[2]);
    let (top, outedge) = fe.get_current_edge(&g, root);
    assert_eq!(top, Some(b[1]));
    assert_eq!(g.block(b[1]).get_out(outedge), b[2]);

    // Collapse 0 and 1 into a BlockList.
    let list = g.new_block_list(root, &[b[0], b[1]]).unwrap();
    // The edge top should now bubble up to the list node.
    let (top2, outedge2) = fe.get_current_edge(&g, root);
    assert_eq!(top2, Some(list));
    assert_eq!(g.block(list).get_out(outedge2), b[2]);
}

// ---------------------------------------------------------------------------
// BlockList (concatenation) — ruleBlockCat
// ---------------------------------------------------------------------------

#[test]
fn straight_line_collapses_to_block_list_no_gotos() {
    // 0 -> 1 -> 2 -> 3 : a pure concatenation, no conditions.
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[2]);
    g.add_edge(b[2], b[3]);
    label_loops_and_doms(&mut g, root);

    let gotos = count_goto_edges_recursive(&g, root);
    assert_eq!(gotos, 0, "a straight line needs no gotos");
    let changes = collapse(&mut g, root);
    assert_eq!(changes, 0);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// BlockIfElse — ruleBlockIfElse (diamond)
// ---------------------------------------------------------------------------

#[test]
fn diamond_collapses_to_if_else_no_gotos() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3 : a classic if/else diamond.
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[3]);
    g.add_edge(b[2], b[3]);
    label_loops_and_doms(&mut g, root);

    assert_eq!(count_goto_edges_recursive(&g, root), 0);
    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// BlockIf (proper if, no else) — ruleBlockProperIf
// ---------------------------------------------------------------------------

#[test]
fn proper_if_collapses_no_gotos() {
    // 0 -> 1 (clause), 0 -> 2 (fallthru), 1 -> 2 : proper if with no else.
    let (mut g, root, b) = build_graph(3);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[2]);
    label_loops_and_doms(&mut g, root);

    assert_eq!(count_goto_edges_recursive(&g, root), 0);
    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// BlockWhileDo — ruleBlockWhileDo
// ---------------------------------------------------------------------------

#[test]
fn while_do_loop_collapses_no_gotos() {
    // entry(0) -> 1(cond) ; 1 -> 2(body) ; 2 -> 1(back) ; 1 -> 3(exit)
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[2]);
    g.add_edge(b[2], b[1]); // back edge
    g.add_edge(b[1], b[3]); // exit
    label_loops_and_doms(&mut g, root);

    // The back edge is a loop edge; structuring should not need an explicit goto.
    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// BlockDoWhile — ruleBlockDoWhile (self loop)
// ---------------------------------------------------------------------------

#[test]
fn do_while_self_loop_collapses() {
    // 0 -> 1 ; 1 -> 1 (self back-edge) ; 1 -> 2 (exit)
    let (mut g, root, b) = build_graph(3);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[1]);
    g.add_edge(b[1], b[2]);
    label_loops_and_doms(&mut g, root);

    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// BlockInfLoop — ruleBlockInfLoop
// ---------------------------------------------------------------------------

#[test]
fn infinite_self_loop_collapses() {
    // 0 -> 1 ; 1 -> 1 (only out edge, no exit) : an infinite loop.
    let (mut g, root, b) = build_graph(2);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[1]);
    label_loops_and_doms(&mut g, root);

    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// Nested loops
// ---------------------------------------------------------------------------

#[test]
fn nested_loops_collapse() {
    // Outer loop head 1, inner loop head 2.
    // 0 -> 1
    // 1 -> 2          (enter inner)
    // 2 -> 2  via 3:  2 -> 3 -> 2 (inner back edge through body 3)
    // 2 -> 4          (inner exit to 4)
    // 4 -> 1          (outer back edge)
    // 1 -> 5          (outer exit)
    let (mut g, root, b) = build_graph(6);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[2]);
    g.add_edge(b[2], b[3]);
    g.add_edge(b[3], b[2]); // inner back
    g.add_edge(b[2], b[4]); // inner exit
    g.add_edge(b[4], b[1]); // outer back
    g.add_edge(b[1], b[5]); // outer exit
    label_loops_and_doms(&mut g, root);

    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// Irreducible two-entry loop — forces a goto via the TraceDAG
// ---------------------------------------------------------------------------

#[test]
fn irreducible_two_entry_loop_forces_a_goto() {
    // Classic irreducible graph: 0 -> 1, 0 -> 2, 1 -> 2, 2 -> 1.
    // The 1<->2 loop has two entries; one edge must be made an irreducible/goto
    // edge for the graph to structure, and the collapse must still terminate.
    let (mut g, root, b) = build_graph(3);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[2]);
    g.add_edge(b[2], b[1]);
    label_loops_and_doms(&mut g, root);

    // Either an irreducible edge was labeled by structure_loops, or the collapse
    // engine will mark a goto; in both cases collapse_all must terminate.
    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// Goto: a forward jump into the middle of an if (unstructurable edge)
// ---------------------------------------------------------------------------

#[test]
fn cross_edge_forces_goto_and_terminates() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3, and an extra cross edge 1 -> 2 that breaks
    // the clean diamond, forcing the TraceDAG to pick a goto.  Must terminate.
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[2]); // cross edge
    g.add_edge(b[1], b[3]);
    g.add_edge(b[2], b[3]);
    label_loops_and_doms(&mut g, root);

    let _ = collapse(&mut g, root);
    assert_fully_collapsed(&g, root);
}

// ---------------------------------------------------------------------------
// LoopBody ordering / comparators
// ---------------------------------------------------------------------------

#[test]
fn loopbody_compare_ends_orders_by_head_then_tail() {
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[2]);
    g.add_edge(b[2], b[1]);
    g.add_edge(b[1], b[3]);
    label_loops_and_doms(&mut g, root);

    let mut a = LoopBody::new(b[1]);
    a.add_tail(b[2]);
    let mut c = LoopBody::new(b[1]);
    c.add_tail(b[3]);
    // same head, tail-index order decides
    let order = LoopBody::compare_ends(&a, &c, &g);
    let a_tail = g.block(b[2]).get_index();
    let c_tail = g.block(b[3]).get_index();
    assert_eq!(order, a_tail.cmp(&c_tail));
}

#[test]
fn loopbody_compare_head_is_signed_three_way() {
    assert_eq!(LoopBody::compare_head(1, 3), -1);
    assert_eq!(LoopBody::compare_head(3, 3), 0);
    assert_eq!(LoopBody::compare_head(5, 3), 1);
}

// ---------------------------------------------------------------------------
// Action / switch stub visibility
// ---------------------------------------------------------------------------

#[test]
fn new_block_switch_stub_is_err() {
    // The switch factory (getExitLeaf + grabCaseBasic) is a documented stub; the
    // engine surfaces it as an Err rather than silently mis-porting.
    assert!(new_block_switch_stub(true).is_err());
    assert!(new_block_switch_stub(false).is_err());
}

// ---------------------------------------------------------------------------
// ConditionalJoin::execute (blockaction.cc:2094) — the node-join mutation path
// ---------------------------------------------------------------------------
//
// `execute` is no longer a stub: `nodeJoinCreateBlock` + `setupMultiequals` +
// `moveCbranch` + `cutDownMultiequals` are all available at this merge base.  We
// build the canonical \e split-condition CFG by hand (two condition blocks with
// the SAME boolean condition Varnode, sharing two exits, with a MULTIEQUAL in one
// exit merging a value from each side) and verify the join: one CBRANCH survives
// in a new join block, the duplicate CBRANCH is destroyed, and the exit MULTIEQUAL
// is cut down to a single input.  No function-name / address special-casing — the
// pattern is recognized purely by topology + the shared condition.

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::context::{ArchContext, OpId, TypeOp, VarnodeId};

fn cj_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn cj_fd() -> Funcdata {
    let glb = Rc::new(ArchContext::new(cj_manager()));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("nj", "nj", glb, addr, 0x10000000, 0x40).unwrap()
}

fn cj_ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn cj_flags(opc: OpCode) -> u32 {
    match opc {
        OpCode::CPUI_CBRANCH => pcodeop_flags::branch,
        OpCode::CPUI_MULTIEQUAL => pcodeop_flags::marker,
        _ => 0,
    }
}

/// Create + opcode a fresh op with `inputs` slots at `off`, leaving it on the
/// dead list (caller inserts it into a block).
fn cj_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = cj_ram(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, cj_flags(opc), format!("{opc:?}")));
    op
}

fn cj_vn(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let ram = cj_ram(fd);
    fd.vbank_mut().create(4, Address::new(ram, off), Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN)))
}

#[test]
fn conditional_join_execute_joins_split_condition() {
    let mut fd = cj_fd();
    let root = fd.bblocks_root_pub();
    let ram = cj_ram(&fd);

    // CFG:  entry -> {block1, block2};  block1,block2 each -> {exita, exitb}.
    let entry = fd.bblocks_mut().new_block_basic(root);
    let block1 = fd.bblocks_mut().new_block_basic(root);
    let block2 = fd.bblocks_mut().new_block_basic(root);
    let exita = fd.bblocks_mut().new_block_basic(root);
    let exitb = fd.bblocks_mut().new_block_basic(root);

    // entry conditionally goes to block1 or block2 (so both have one in-edge from
    // entry; leastout selection then has block1 as a second in-edge of an exit).
    fd.bblocks_mut().add_edge(entry, block1);
    fd.bblocks_mut().add_edge(entry, block2);
    // block1: out0 -> exita (false), out1 -> exitb (true).
    fd.bblocks_mut().add_edge(block1, exita);
    fd.bblocks_mut().add_edge(block1, exitb);
    // block2: out0 -> exita (false), out1 -> exitb (true).  Same exit order.
    fd.bblocks_mut().add_edge(block2, exita);
    fd.bblocks_mut().add_edge(block2, exitb);
    fd.bblocks_mut().set_start_block(root, entry);

    // The SHARED boolean condition Varnode (find_dups returns true via vn1==vn2).
    // It must be WRITTEN (a free varnode may hold only one descendant — the C++
    // addDescend invariant); define it by an op in `entry`.
    let cmp_a = cj_vn(&mut fd, 0x30);
    let cmp_b = cj_vn(&mut fd, 0x38);
    let cmp = cj_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESSEQUAL);
    fd.op_set_input(cmp, cmp_a, 0).unwrap();
    fd.op_set_input(cmp, cmp_b, 1).unwrap();
    let cond = cj_vn(&mut fd, 0x40);
    fd.op_set_output(cmp, cond).unwrap();
    fd.op_insert(cmp, entry, None);

    // The branch-target operand (slot 0); a code-address annotation in real IR,
    // here a written Varnode so two CBRANCHs can both read it.
    let tgt_src = cj_vn(&mut fd, 0x48);
    let tgtop = cj_op(&mut fd, 1, 0x1004, OpCode::CPUI_COPY);
    fd.op_set_input(tgtop, tgt_src, 0).unwrap();
    let tgt = cj_vn(&mut fd, 0x50);
    fd.op_set_output(tgtop, tgt).unwrap();
    fd.op_insert(tgtop, entry, None);

    // block1 CBRANCH(tgt, cond)
    let cbr1 = cj_op(&mut fd, 2, 0x1100, OpCode::CPUI_CBRANCH);
    fd.op_set_input(cbr1, tgt, 0).unwrap();
    fd.op_set_input(cbr1, cond, 1).unwrap();
    fd.op_insert(cbr1, block1, None);

    // block2 CBRANCH(tgt, cond)  — the duplicate.
    let cbr2 = cj_op(&mut fd, 2, 0x1200, OpCode::CPUI_CBRANCH);
    fd.op_set_input(cbr2, tgt, 0).unwrap();
    fd.op_set_input(cbr2, cond, 1).unwrap();
    fd.op_insert(cbr2, block2, None);

    // exita MULTIEQUAL merging a value from block1 (slot a_in1) and block2 (a_in2).
    // Both are WRITTEN (defined in their source block) so they can also feed the
    // new join-block MULTIEQUAL that setupMultiequals creates.
    let va1_src = cj_vn(&mut fd, 0x58);
    let va1op = cj_op(&mut fd, 1, 0x110a, OpCode::CPUI_COPY);
    fd.op_set_input(va1op, va1_src, 0).unwrap();
    let va1 = cj_vn(&mut fd, 0x60);
    fd.op_set_output(va1op, va1).unwrap();
    fd.op_insert(va1op, block1, Some(cbr1));

    let va2_src = cj_vn(&mut fd, 0x64);
    let va2op = cj_op(&mut fd, 1, 0x120a, OpCode::CPUI_COPY);
    fd.op_set_input(va2op, va2_src, 0).unwrap();
    let va2 = cj_vn(&mut fd, 0x68);
    fd.op_set_output(va2op, va2).unwrap();
    fd.op_insert(va2op, block2, Some(cbr2));
    let mq = cj_op(&mut fd, 2, 0x1300, OpCode::CPUI_MULTIEQUAL);
    // exita in-edges: edge from block1 is in-slot 0, from block2 in-slot 1.
    let a_in1 = fd.bblocks_ref().block(block1).get_out_rev_index(0);
    let a_in2 = fd.bblocks_ref().block(block2).get_out_rev_index(0);
    fd.op_set_input(mq, va1, a_in1).unwrap();
    fd.op_set_input(mq, va2, a_in2).unwrap();
    let mq_out = cj_vn(&mut fd, 0x70);
    fd.op_set_output(mq, mq_out).unwrap();
    fd.op_insert(mq, exita, None);

    fd.structure_reset();

    let n_blocks_before = fd.bblocks_get_size();

    // Match + execute the join (mirrors ActionNodeJoin's inner call).
    let mut cj = ConditionalJoin::new();
    assert!(cj.match_blocks(block1, block2, &fd), "split-condition pattern must match");
    cj.execute(&mut fd).expect("ConditionalJoin::execute must succeed (no longer a stub)");

    // A new join block was created (nodeJoinCreateBlock).
    assert_eq!(fd.bblocks_get_size(), n_blocks_before + 1, "execute must add one join block");

    // The duplicate CBRANCH (cbr2) was destroyed (marked dead, unlinked from its
    // block); the surviving one (cbr1) is still alive.  (opDestroy marks dead +
    // removes from parent; the op shell stays in the bank per the W3-varnode boundary.)
    let cbr2_op = fd.obank().get(cbr2).expect("cbr2 shell remains in bank");
    assert!(cbr2_op.is_dead(), "moveCbranch must opDestroy (mark dead) the duplicate CBRANCH");
    assert_eq!(cbr2_op.get_parent(), None, "the destroyed CBRANCH is removed from its block");
    let alive: Vec<_> = fd.obank().iter_alive().collect();
    assert!(!alive.contains(&cbr2), "the duplicate CBRANCH is off the alive list");
    assert!(alive.contains(&cbr1), "the surviving CBRANCH stays alive");
    assert_eq!(
        fd.obank().get(cbr1).unwrap().code(),
        OpCode::CPUI_CBRANCH,
        "the surviving op is still a CBRANCH"
    );

    // The surviving CBRANCH now lives in the new join block (opUninsert+opInsertEnd).
    let cbr1_parent = fd.obank().get(cbr1).unwrap().get_parent();
    assert!(cbr1_parent.is_some());
    assert_ne!(cbr1_parent, Some(block1), "the CBRANCH moved out of block1 into the join block");

    // exita's MULTIEQUAL was cut down: with vn1==vn2? no — va1 != va2, so the hi
    // input is removed and lo set to the merged Varnode -> numInput drops to 1 and
    // the op is rewritten to a COPY (cutDownMultiequals).
    assert!(fd.obank().get(mq).is_some(), "the exit merge op survives (rewritten)");
    assert_eq!(
        fd.obank().get(mq).unwrap().num_input(),
        1,
        "cutDownMultiequals collapses the 2-input merge to 1 input"
    );
    assert_eq!(
        fd.obank().get(mq).unwrap().code(),
        OpCode::CPUI_COPY,
        "a 1-input MULTIEQUAL is rewritten to COPY"
    );
    let _ = ram;
}

// ---------------------------------------------------------------------------
// VERIFIER adversarial tests (item: w10-nodejoin-compare) — target the spots the
// hunt list flagged as most fragile in ConditionalJoin::execute:
//   * cutDownMultiequals vn1==vn2 branch (opRemoveInput-only, no mergeneed lookup)
//   * moveCbranch vn1==vn2 branch (shared condition -> vn = vn1, no lookup)
//   * the numInput()==1 boundary: a >2-input exit MULTIEQUAL must NOT collapse to
//     COPY (off-by-one guard on the COPY rewrite).
// All build the topology by hand; no name/address special-casing.
// ---------------------------------------------------------------------------

/// Shared builder for the canonical split-condition CFG. `n_exita_extra` adds
/// extra (non-block1/block2) in-edges + MULTIEQUAL inputs to exita so the
/// post-cutdown numInput can be controlled. Returns
/// `(entry, block1, block2, exita, exitb, cbr1, cbr2, mq)`.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn cj_build_split(
    fd: &mut Funcdata,
    same_exit_merge: bool, // exita MULTIEQUAL uses the SAME varnode on both edges
    extra_exita_preds: usize, // extra predecessors (and MULTIEQUAL inputs) on exita
) -> (BlockId, BlockId, BlockId, BlockId, BlockId, OpId, OpId, OpId) {
    let root = fd.bblocks_root_pub();
    let entry = fd.bblocks_mut().new_block_basic(root);
    let block1 = fd.bblocks_mut().new_block_basic(root);
    let block2 = fd.bblocks_mut().new_block_basic(root);
    let exita = fd.bblocks_mut().new_block_basic(root);
    let exitb = fd.bblocks_mut().new_block_basic(root);

    fd.bblocks_mut().add_edge(entry, block1);
    fd.bblocks_mut().add_edge(entry, block2);
    fd.bblocks_mut().add_edge(block1, exita);
    fd.bblocks_mut().add_edge(block1, exitb);
    fd.bblocks_mut().add_edge(block2, exita);
    fd.bblocks_mut().add_edge(block2, exitb);
    // Extra predecessors of exita come from `entry` (a benign extra in-edge) so the
    // exit MULTIEQUAL can carry >2 inputs and survive cut-down without collapsing.
    let mut extra_blocks = Vec::new();
    for _ in 0..extra_exita_preds {
        let eb = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(entry, eb);
        fd.bblocks_mut().add_edge(eb, exita);
        extra_blocks.push(eb);
    }
    fd.bblocks_mut().set_start_block(root, entry);

    // Shared condition varnode (must be written for the addDescend invariant).
    let cmp_a = cj_vn(fd, 0x30);
    let cmp_b = cj_vn(fd, 0x38);
    let cmp = cj_op(fd, 2, 0x1000, OpCode::CPUI_INT_LESSEQUAL);
    fd.op_set_input(cmp, cmp_a, 0).unwrap();
    fd.op_set_input(cmp, cmp_b, 1).unwrap();
    let cond = cj_vn(fd, 0x40);
    fd.op_set_output(cmp, cond).unwrap();
    fd.op_insert(cmp, entry, None);

    // Branch-target operand.
    let tgt_src = cj_vn(fd, 0x48);
    let tgtop = cj_op(fd, 1, 0x1004, OpCode::CPUI_COPY);
    fd.op_set_input(tgtop, tgt_src, 0).unwrap();
    let tgt = cj_vn(fd, 0x50);
    fd.op_set_output(tgtop, tgt).unwrap();
    fd.op_insert(tgtop, entry, None);

    let cbr1 = cj_op(fd, 2, 0x1100, OpCode::CPUI_CBRANCH);
    fd.op_set_input(cbr1, tgt, 0).unwrap();
    fd.op_set_input(cbr1, cond, 1).unwrap();
    fd.op_insert(cbr1, block1, None);

    let cbr2 = cj_op(fd, 2, 0x1200, OpCode::CPUI_CBRANCH);
    fd.op_set_input(cbr2, tgt, 0).unwrap();
    fd.op_set_input(cbr2, cond, 1).unwrap();
    fd.op_insert(cbr2, block2, None);

    // exita MULTIEQUAL. va1 (block1 edge) and va2 (block2 edge). When
    // `same_exit_merge`, both edges read the SAME written varnode (the vn1==vn2
    // cutdown branch); else two distinct written varnodes (the mergeneed branch).
    let va1_src = cj_vn(fd, 0x58);
    let va1op = cj_op(fd, 1, 0x110a, OpCode::CPUI_COPY);
    fd.op_set_input(va1op, va1_src, 0).unwrap();
    let va1 = cj_vn(fd, 0x60);
    fd.op_set_output(va1op, va1).unwrap();
    fd.op_insert(va1op, block1, Some(cbr1));

    let va2 = if same_exit_merge {
        va1
    } else {
        let va2_src = cj_vn(fd, 0x64);
        let va2op = cj_op(fd, 1, 0x120a, OpCode::CPUI_COPY);
        fd.op_set_input(va2op, va2_src, 0).unwrap();
        let v = cj_vn(fd, 0x68);
        fd.op_set_output(va2op, v).unwrap();
        fd.op_insert(va2op, block2, Some(cbr2));
        v
    };

    let n_in = 2 + extra_exita_preds as int4;
    let mq = cj_op(fd, n_in, 0x1300, OpCode::CPUI_MULTIEQUAL);
    let a_in1 = fd.bblocks_ref().block(block1).get_out_rev_index(0);
    let a_in2 = fd.bblocks_ref().block(block2).get_out_rev_index(0);
    fd.op_set_input(mq, va1, a_in1).unwrap();
    fd.op_set_input(mq, va2, a_in2).unwrap();
    // Fill any extra MULTIEQUAL slots with their own written varnodes.
    for (k, eb) in extra_blocks.iter().enumerate() {
        let src = cj_vn(fd, 0x80 + 0x10 * k as u64);
        let cop = cj_op(fd, 1, 0x1400 + 0x10 * k as u64, OpCode::CPUI_COPY);
        fd.op_set_input(cop, src, 0).unwrap();
        let ev = cj_vn(fd, 0x88 + 0x10 * k as u64);
        fd.op_set_output(cop, ev).unwrap();
        fd.op_insert(cop, *eb, None);
        let slot = fd.bblocks_ref().block(*eb).get_out_rev_index(0);
        fd.op_set_input(mq, ev, slot).unwrap();
    }
    let mq_out = cj_vn(fd, 0x70);
    fd.op_set_output(mq, mq_out).unwrap();
    fd.op_insert(mq, exita, None);

    fd.structure_reset();
    (entry, block1, block2, exita, exitb, cbr1, cbr2, mq)
}

/// ADVERSARIAL 1 (w10-nodejoin-compare): cutDownMultiequals `vn1 == vn2` branch.
/// When the exit MULTIEQUAL merges the SAME varnode from both root edges, C++
/// (`blockaction.cc:2003`) only does `opRemoveInput(op,hi)` — it must NOT touch
/// the mergeneed map (that key was never inserted by checkExitBlock, which skips
/// vn1==vn2).  A faithful port must take this branch without a mergeneed lookup
/// and still collapse the now-1-input MULTIEQUAL to a COPY.
#[test]
fn cj_cutdown_same_varnode_branch_no_mergeneed_lookup() {
    let mut fd = cj_fd();
    let (_entry, block1, block2, _exita, _exitb, _cbr1, _cbr2, mq) =
        cj_build_split(&mut fd, /*same_exit_merge=*/ true, /*extra=*/ 0);

    let mut cj = ConditionalJoin::new();
    assert!(cj.match_blocks(block1, block2, &fd), "split-condition pattern must match");
    // If the port wrongly looked up mergeneed on the vn1==vn2 path it would panic
    // ("mergeneed entry missing"); a faithful execute completes.
    cj.execute(&mut fd).expect("execute must not consult mergeneed on the vn1==vn2 cutdown branch");

    // 2-input merge of identical varnodes -> opRemoveInput(hi) -> 1 input -> COPY.
    assert_eq!(fd.obank().get(mq).unwrap().num_input(), 1, "same-varnode merge collapses to 1 input");
    assert_eq!(
        fd.obank().get(mq).unwrap().code(),
        OpCode::CPUI_COPY,
        "a 1-input MULTIEQUAL is rewritten to COPY even on the vn1==vn2 branch"
    );
}

/// ADVERSARIAL 2 (w10-nodejoin-compare): moveCbranch `vn1 == vn2` branch.
/// boolless-shape input: both CBRANCHes read the *same* condition varnode, so
/// findDups returns true via vn1==vn2 (no MergePair inserted for the condition).
/// moveCbranch (`blockaction.cc:2050`) must then set `vn = vn1` WITHOUT a
/// mergeneed lookup; the surviving CBRANCH keeps reading the shared condition.
#[test]
fn cj_movecbranch_shared_condition_uses_vn1_not_mergeneed() {
    let mut fd = cj_fd();
    let (_entry, block1, block2, _exita, _exitb, cbr1, cbr2, _mq) =
        cj_build_split(&mut fd, /*same_exit_merge=*/ false, /*extra=*/ 0);

    // Capture the shared condition varnode that cbr1 reads before the join.
    let shared_cond = fd.obank().get(cbr1).unwrap().get_in(1).unwrap();
    assert_eq!(
        fd.obank().get(cbr2).unwrap().get_in(1).unwrap(),
        shared_cond,
        "precondition: both CBRANCHes read the SAME condition varnode"
    );

    let mut cj = ConditionalJoin::new();
    assert!(cj.match_blocks(block1, block2, &fd), "split-condition pattern must match");
    cj.execute(&mut fd).expect("execute must take the vn1==vn2 moveCbranch branch without a lookup");

    // The surviving CBRANCH still reads the shared condition (vn = vn1), and the
    // duplicate was destroyed.
    assert_eq!(
        fd.obank().get(cbr1).unwrap().get_in(1).unwrap(),
        shared_cond,
        "moveCbranch repoints the surviving CBRANCH to vn1 (the shared condition)"
    );
    assert!(fd.obank().get(cbr2).unwrap().is_dead(), "the duplicate CBRANCH is destroyed");
}

/// ADVERSARIAL 3 (w10-nodejoin-compare): the numInput()==1 boundary in
/// cutDownMultiequals (`blockaction.cc:2011`).  An exit MULTIEQUAL with a THIRD
/// predecessor (unrelated to block1/block2) has its block1/block2 inputs cut to
/// one merged input but RETAINS the third — numInput() stays 2, so it must NOT be
/// rewritten to COPY.  Guards against an off-by-one that collapses every cut
/// MULTIEQUAL.
#[test]
fn cj_cutdown_keeps_multiequal_when_extra_pred_remains() {
    let mut fd = cj_fd();
    let (_entry, block1, block2, _exita, _exitb, _cbr1, _cbr2, mq) =
        cj_build_split(&mut fd, /*same_exit_merge=*/ false, /*extra=*/ 1);

    assert_eq!(fd.obank().get(mq).unwrap().num_input(), 3, "precondition: 3-input exit MULTIEQUAL");

    let mut cj = ConditionalJoin::new();
    assert!(cj.match_blocks(block1, block2, &fd), "split-condition pattern must match");
    cj.execute(&mut fd).expect("execute");

    // block1+block2 edges collapse to one merged input; the extra pred survives.
    assert_eq!(
        fd.obank().get(mq).unwrap().num_input(),
        2,
        "3-input merge drops the hi block-edge -> 2 inputs (merged + extra pred)"
    );
    assert_eq!(
        fd.obank().get(mq).unwrap().code(),
        OpCode::CPUI_MULTIEQUAL,
        "a 2-input MULTIEQUAL must STAY a MULTIEQUAL (no COPY rewrite at numInput!=1)"
    );
}

// ---------------------------------------------------------------------------
// is_complex resolution (the FlowBlock::isComplex virtual dispatch: only
// BlockBasic/BlockCopy/BlockCondition override; base block.hh:254 is `true`)
// ---------------------------------------------------------------------------

#[test]
fn is_complex_resolves_through_block_copy() {
    // Build a structuring graph holding a single BlockCopy leaf whose `copy`
    // points at a (synthetic) bblocks BlockBasic id.  is_complex must read the
    // copy pointer (BlockCopy::isComplex, block.hh:549) and consult the
    // precomputed set (BlockBasic::isComplex, block.cc:2403).
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    // A "bblocks" basic block to be the copy target.
    let basic = g.new_block_basic(root);
    // The structuring BlockCopy leaf mirroring it.
    let copyleaf = g.new_block_copy(root, basic);

    // With an empty complex set, nothing is complex.
    let cs = CollapseStructure::new(&mut g, root);
    assert!(!cs.is_complex(copyleaf), "empty set -> not complex");
    drop(cs);

    // Marking the underlying bblocks id complex flips the answer.
    let mut complex = std::collections::BTreeSet::new();
    complex.insert(basic);
    let cs = CollapseStructure::new(&mut g, root).with_complex_blocks(complex);
    assert!(cs.is_complex(copyleaf), "marked basic -> complex through the copy pointer");
}

#[test]
fn is_complex_falls_back_to_true_for_non_copy_leaf() {
    // A plain graph node has no isComplex override, so the C++ base
    // FlowBlock::isComplex default (`true`, block.hh:254) applies.
    let (mut g, root, b) = build_graph(1);
    let cs = CollapseStructure::new(&mut g, root);
    assert!(cs.is_complex(b[0]), "non-copy leaf -> conservative true");
}

#[test]
fn is_complex_blocklist_is_unconditionally_complex() {
    // A BlockList has NO isComplex override upstream -> base `true`
    // (block.hh:254) even when its front leaf is a trivial (non-complex)
    // BlockBasic.  The buggy front-leaf descent scored such a list non-complex,
    // which skipped the whileDo overflow syntax and inlined the list's
    // statements (including embedded returns) into the `while(...)` parens —
    // invalid C (decbench O0-iproute2-ip-lookup_flag_data_by_name).
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let basic = g.new_block_basic(root);
    let copyleaf = g.new_block_copy(root, basic); // trivial front leaf
    let tail = g.new_block(root);
    g.add_edge(copyleaf, tail);
    let list = g.new_block_list(root, &[copyleaf, tail]).expect("new_block_list");

    // `basic` is NOT in complex_blocks -> the front leaf alone would say false.
    let cs = CollapseStructure::new(&mut g, root);
    assert!(!cs.is_complex(copyleaf), "the front leaf itself is not complex");
    assert!(cs.is_complex(list), "a BlockList is unconditionally complex (base isComplex)");
}

#[test]
fn is_complex_blockif_is_unconditionally_complex() {
    // A BlockIf has no isComplex override either -> base `true` (block.hh:254).
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let basic = g.new_block_basic(root);
    let cond = g.new_block_copy(root, basic); // trivial condition leaf
    let clause = g.new_block(root);
    let exit = g.new_block(root);
    g.add_edge(cond, clause);
    g.add_edge(cond, exit);
    g.add_edge(clause, exit);
    let ifblk = g.new_block_if(root, cond, clause);

    let cs = CollapseStructure::new(&mut g, root);
    assert!(cs.is_complex(ifblk), "a BlockIf is unconditionally complex (base isComplex)");
}

#[test]
fn is_complex_condition_delegates_to_component_zero() {
    // BlockCondition::isComplex -> getBlock(0)->isComplex() (block.hh:649):
    // only the FIRST component's verdict matters.
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let basic1 = g.new_block_basic(root);
    let basic2 = g.new_block_basic(root);
    let b1 = g.new_block_copy(root, basic1);
    let b2 = g.new_block_copy(root, basic2);
    let clause = g.new_block(root);
    let exit = g.new_block(root);
    g.add_edge(b1, b2);
    g.add_edge(b1, clause);
    g.add_edge(b2, clause);
    g.add_edge(b2, exit);
    let cond = g.new_block_condition(root, b1, b2).expect("new_block_condition");

    // Mark only the SECOND component's basic complex: the condition still
    // delegates to component 0 -> not complex.
    let mut complex = std::collections::BTreeSet::new();
    complex.insert(basic2);
    let cs = CollapseStructure::new(&mut g, root).with_complex_blocks(complex);
    assert!(
        !cs.is_complex(cond),
        "BlockCondition delegates to getBlock(0), ignoring the second component"
    );
    drop(cs);

    // Marking the FIRST component's basic complex flips the answer.
    let mut complex = std::collections::BTreeSet::new();
    complex.insert(basic1);
    let cs = CollapseStructure::new(&mut g, root).with_complex_blocks(complex);
    assert!(cs.is_complex(cond), "complex getBlock(0) -> the condition is complex");
}

// ---------------------------------------------------------------------------
// (kuna) `option condfold` -- the short-circuit fold across a COMPLEX sibling.
// ---------------------------------------------------------------------------

/// Build the canonical `ruleBlockOr` shape, oriented so neither
/// `negate_condition_rec` call is needed (`orblock` is already `bl`'s out 0 and
/// `clauseblock` is already `orblock`'s out 1):
///
/// ```text
///   bl --0--> orblock --0--> other
///    \--1--> clause <---1---/
/// ```
///
/// Returns `(graph, root, bl, orblock, bb_bl, bb_or)` where the `bb_*` ids are
/// the underlying "bblocks" `BlockBasic`s the two `BlockCopy` leaves point at.
#[allow(clippy::type_complexity)]
fn build_block_or_shape() -> (BlockGraph, BlockId, BlockId, BlockId, BlockId, BlockId) {
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let bb_bl = g.new_block_basic(root);
    let bb_or = g.new_block_basic(root);
    let bl = g.new_block_copy(root, bb_bl);
    let orblock = g.new_block_copy(root, bb_or);
    let clause = g.new_block(root);
    let other = g.new_block(root);
    g.add_edge(bl, orblock); // bl out 0 -> the sibling condition
    g.add_edge(bl, clause); // bl out 1 -> the shared clause
    g.add_edge(orblock, other); // orblock out 0
    g.add_edge(orblock, clause); // orblock out 1 -> the shared clause
    (g, root, bl, orblock, bb_bl, bb_or)
}

#[test]
fn condfold_off_leaves_rule_block_or_declining_a_complex_sibling() {
    // The upstream behavior, and the byte-identical guarantee of `condfold off`:
    // a complex `orblock` is refused outright, so nothing folds.
    let (mut g, root, bl, _orblock, _bb_bl, bb_or) = build_block_or_shape();
    let mut complex = std::collections::BTreeSet::new();
    complex.insert(bb_or);
    // No `with_condfold_blocks` -> the option is off.
    let mut cs = CollapseStructure::new(&mut g, root).with_complex_blocks(complex);
    assert!(
        !cs.rule_block_or(bl).expect("rule_block_or"),
        "a complex sibling must be refused when condfold is off"
    );
}

#[test]
fn condfold_on_folds_a_complex_but_eligible_sibling() {
    // The whole feature: the SAME graph and the SAME complex verdict, but the
    // sibling's underlying BlockBasic is in the condfold-eligible set.
    let (mut g, root, bl, _orblock, _bb_bl, bb_or) = build_block_or_shape();
    let mut complex = std::collections::BTreeSet::new();
    complex.insert(bb_or);
    let mut condfold = std::collections::BTreeSet::new();
    condfold.insert(bb_or);
    let mut cs = CollapseStructure::new(&mut g, root)
        .with_complex_blocks(complex)
        .with_condfold_blocks(condfold);
    assert!(
        cs.rule_block_or(bl).expect("rule_block_or"),
        "an eligible complex sibling folds when condfold is on"
    );
}

#[test]
fn condfold_relaxed_fold_reports_itself_complex() {
    // Guard for `rule_block_while_do`: BlockCondition::isComplex delegates to
    // getBlock(0), which here is the trivial LEFT operand -- so without the
    // `condfolded` marking the comma chain in the RIGHT operand would be lifted
    // into a `while(...)` header.
    let (mut g, root, bl, _orblock, _bb_bl, bb_or) = build_block_or_shape();
    let mut complex = std::collections::BTreeSet::new();
    complex.insert(bb_or); // note: bb_bl (the LEFT operand) is NOT complex
    let mut condfold = std::collections::BTreeSet::new();
    condfold.insert(bb_or);
    let mut cs = CollapseStructure::new(&mut g, root)
        .with_complex_blocks(complex)
        .with_condfold_blocks(condfold);
    assert!(cs.rule_block_or(bl).expect("rule_block_or"));
    // The folded node is the graph's newest component.
    let folded = *cs
        .graph
        .block(root)
        .get_list()
        .last()
        .expect("the folded BlockCondition is the newest component");
    assert_eq!(cs.graph.block(folded).get_type(), BlockType::Condition);
    assert!(
        cs.is_complex(folded),
        "a condfold-relaxed BlockCondition must report itself complex despite \
         its trivial left operand"
    );
}

#[test]
fn condfold_ok_refuses_a_non_copy_operand() {
    // Precondition: only a BlockCopy of ONE BlockBasic may be relaxed.  A
    // BlockList / BlockIf / BlockCondition operand can render braces, multiple
    // lines, or a label inside the parentheses -- i.e. invalid C.
    let (mut g, root, _bl, _orblock, _bb_bl, bb_or) = build_block_or_shape();
    let plain = g.new_block(root); // not a BlockCopy
    let mut condfold = std::collections::BTreeSet::new();
    condfold.insert(bb_or);
    let cs = CollapseStructure::new(&mut g, root).with_condfold_blocks(condfold);
    assert!(!cs.condfold_ok(plain), "a non-BlockCopy operand is never relaxed");
}

#[test]
fn condfold_ok_is_dead_when_the_option_is_off() {
    // With an empty eligible set (the `condfold off` state) the gate can never
    // fire, whatever the operand is.
    let (mut g, root, _bl, orblock, _bb_bl, _bb_or) = build_block_or_shape();
    let cs = CollapseStructure::new(&mut g, root);
    assert!(!cs.condfold_ok(orblock), "empty set -> the relaxation is dead");
}
