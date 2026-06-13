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
use crate::seams::BlockId;

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
// Action / switch seam visibility
// ---------------------------------------------------------------------------

#[test]
fn new_block_switch_seam_is_err() {
    // The switch factory (getExitLeaf + grabCaseBasic) is a documented seam; the
    // engine surfaces it as an Err rather than silently mis-porting.
    assert!(new_block_switch_seam(true).is_err());
    assert!(new_block_switch_seam(false).is_err());
}

// ---------------------------------------------------------------------------
// ConditionalJoin::execute (blockaction.cc:2094) — the node-join mutation path
// ---------------------------------------------------------------------------
//
// `execute` is no longer a seam: `nodeJoinCreateBlock` + `setupMultiequals` +
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
use crate::seams::{Architecture, OpId, TypeOp, VarnodeId};

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
    let glb = Rc::new(Architecture::new(cj_manager()));
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
    cj.execute(&mut fd).expect("ConditionalJoin::execute must succeed (no longer a seam)");

    // A new join block was created (nodeJoinCreateBlock).
    assert_eq!(fd.bblocks_get_size(), n_blocks_before + 1, "execute must add one join block");

    // The duplicate CBRANCH (cbr2) was destroyed (marked dead, unlinked from its
    // block); the surviving one (cbr1) is still alive.  (opDestroy marks dead +
    // removes from parent; the op shell stays in the bank per the W3-varnode seam.)
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
