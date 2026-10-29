//! A for-loop header must preserve reads of its printed iterator expression.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;

use kuna_decomp::context::{ArchContext, BlockId, OpId};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
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

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

/// The `ram` space as a clonable handle (captured once per test so it does
/// not re-borrow `fd` while building ops).
fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn addr(rs: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(rs), off)
}

/// A loop latch with an explicit increment, a visible COPY, and a hidden
/// backedge COPY. The visible COPY can either consume the increment or be
/// independent of it.
fn terminal_copy_fixture(dependent: bool) -> (Funcdata, BlockId, OpId, OpId) {
    let mut fd = build_fd();
    let rs = ramspace(&fd);
    let root = fd.bblocks_ref().root.unwrap();
    let entry = fd.bblocks_mut().new_block_basic(root);
    let head = fd.bblocks_mut().new_block_basic(root);
    let tail = fd.bblocks_mut().new_block_basic(root);
    let exit = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(entry, head);
    fd.bblocks_mut().add_edge(head, tail);
    fd.bblocks_mut().add_edge(head, exit);
    fd.bblocks_mut().add_edge(tail, head);

    let mut variable = |offset| {
        let v = fd.new_varnode(4, &addr(&rs, offset), None);
        let high = fd.high_bank_mut().new_high(v);
        let node = fd.vbank_mut().get_mut(v).unwrap();
        node.set_high(high, 0);
        node.set_flags_pub(kuna_decomp::varnode::varnode_flags::explict);
        v
    };
    let current = variable(0x20);
    let next = variable(0x24);
    let visible = variable(0x28);
    let backedge = variable(0x2c);
    let zero = fd.new_constant(4, 0);
    let one = fd.new_constant(4, 1);
    let phi = fd.new_op(2, addr(&rs, 0x1000));
    fd.op_set_opcode(
        phi,
        kuna_decomp::typeop::type_op_for(OpCode::CPUI_MULTIEQUAL),
    );
    fd.op_set_output(phi, current).unwrap();
    fd.op_set_input(phi, zero, 0).unwrap();
    fd.op_set_input(phi, backedge, 1).unwrap();
    fd.op_insert_end(phi, head);
    let increment = fd.new_op(2, addr(&rs, 0x1010));
    fd.op_set_opcode(
        increment,
        kuna_decomp::typeop::type_op_for(OpCode::CPUI_INT_ADD),
    );
    fd.op_set_output(increment, next).unwrap();
    fd.op_set_input(increment, current, 0).unwrap();
    fd.op_set_input(increment, one, 1).unwrap();
    fd.op_insert_end(increment, tail);
    let copy = fd.new_op(1, addr(&rs, 0x1014));
    fd.op_set_opcode(copy, kuna_decomp::typeop::type_op_for(OpCode::CPUI_COPY));
    fd.op_set_output(copy, visible).unwrap();
    fd.op_set_input(copy, if dependent { next } else { zero }, 0)
        .unwrap();
    fd.op_insert_end(copy, tail);
    let hidden = fd.new_op(1, addr(&rs, 0x1018));
    fd.op_set_opcode(hidden, kuna_decomp::typeop::type_op_for(OpCode::CPUI_COPY));
    fd.op_set_output(hidden, backedge).unwrap();
    fd.op_set_input(hidden, next, 0).unwrap();
    fd.op_insert_end(hidden, tail);
    fd.obank_mut()
        .get_mut(hidden)
        .unwrap()
        .set_flag(pcodeop_flags::nonprinting);

    let mut graph = kuna_decomp::block::BlockGraph::new();
    let sr = graph.arena.insert(kuna_decomp::block::FlowBlock::new_kind(
        kuna_decomp::block::BlockKind::Graph,
    ));
    graph.root = Some(sr);
    let copies = graph.build_copy_from(sr, fd.bblocks_ref(), root);
    let hc = copies[&head];
    let tc = copies[&tail];
    *fd.sblocks_mut() = graph;
    let wd = fd.sblocks_mut().new_block_while_do(sr, hc, tc);
    fd.sblocks_mut().block_mut(wd).set_loop_def(Some(phi));
    fd.sblocks_mut()
        .block_mut(wd)
        .set_iterate_op(Some(increment));
    (fd, wd, increment, copy)
}

#[test]
fn for_iterator_cannot_pass_a_visible_copy_that_reads_its_result() {
    let (mut fd, wd, increment, copy) = terminal_copy_fixture(true);
    fd.finalize_forloop_printing();
    assert!(fd.sblocks_ref().block(wd).get_iterate_op().is_none());
    assert!(!fd.obank().get(increment).unwrap().not_printed());
    assert_eq!(fd.op_next_op(increment), Some(copy));
}

#[test]
fn for_iterator_can_pass_an_independent_visible_copy() {
    let (mut fd, wd, increment, copy) = terminal_copy_fixture(false);
    fd.finalize_forloop_printing();
    assert_eq!(fd.sblocks_ref().block(wd).get_iterate_op(), Some(increment));
    assert!(fd.obank().get(increment).unwrap().not_printed());
    assert_eq!(fd.op_next_op(copy), Some(increment));
}
