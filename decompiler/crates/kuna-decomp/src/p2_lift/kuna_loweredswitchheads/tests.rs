//! Tests for trying a lowered-switch cascade behind every head on the switch
//! variable.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::{ArchContext, TypeOp, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_loweredswitch::ActionLowerSwitchDetect;

fn build_manager() -> AddrSpaceManager {
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

fn build_fd(every_head: bool, exact: bool) -> Funcdata {
    let mut ctx = ArchContext::new(build_manager());
    ctx.lowered_switch_every_head = every_head;
    ctx.lowered_switch_exact = exact;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
}

fn new_block(fd: &mut Funcdata, off: u64) -> BlockId {
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let a = ram(fd, off);
    fd.set_basic_block_range(bl, &a, &a);
    bl
}

fn switch_var(fd: &mut Funcdata, size: int4) -> VarnodeId {
    let bl = new_block(fd, 0x0100);
    let load = fd.new_op(2, ram(fd, 0x0100));
    fd.op_set_opcode(load, TypeOp::new(OpCode::CPUI_LOAD, 0, "LOAD"));
    let spc = fd.new_constant(4, 0);
    let ptr = fd.new_constant(8, 0x5000);
    fd.op_set_input(load, spc, 0).unwrap();
    fd.op_set_input(load, ptr, 1).unwrap();
    let a = ram(fd, 0x4000);
    let v = fd.new_varnode_out(size, &a, load).unwrap();
    fd.op_insert_end(load, bl);
    v
}

fn cmp_branch(fd: &mut Funcdata, bl: BlockId, off: u64, code: OpCode, v: VarnodeId, c: u64) {
    let cmp = fd.new_op(2, ram(fd, off));
    fd.op_set_opcode(cmp, TypeOp::new(code, 0, "cmp"));
    fd.op_set_input(cmp, v, 0).unwrap();
    let size = fd.vbank().get(v).unwrap().get_size();
    let cvn = fd.new_constant(size, c);
    fd.op_set_input(cmp, cvn, 1).unwrap();
    let b = fd.new_unique_out(1, cmp).unwrap();
    fd.op_insert_end(cmp, bl);
    let cb = fd.new_op(2, ram(fd, off + 1));
    fd.op_set_opcode(cb, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CBRANCH"));
    let dest = fd.new_constant(8, 0);
    fd.op_set_input(cb, dest, 0).unwrap();
    fd.op_set_input(cb, b, 1).unwrap();
    fd.op_insert_end(cb, bl);
}

fn edges(fd: &mut Funcdata, bl: BlockId, on_false: BlockId, on_true: BlockId) {
    fd.bblocks_mut().add_edge(bl, on_false);
    fd.bblocks_mut().add_edge(bl, on_true);
}

/// A compare on `v` in front of the cascade whose edges leave for blocks that
/// compare nothing: a head no cascade stands behind.
fn decoy(fd: &mut Funcdata, v: VarnodeId) {
    let bl = new_block(fd, 0x0800);
    let x = new_block(fd, 0x0900);
    let y = new_block(fd, 0x0a00);
    cmp_branch(fd, bl, 0x0800, OpCode::CPUI_INT_SLESS, v, 1);
    edges(fd, bl, x, y);
}

/// ```text
///   B0: if (V < 2)   true -> B1   false -> B2
///   B1: if (V == 0)  true -> A    false -> D
///   B2: if (V == 2)  true -> B    false -> B3
///   B3: if (V == 3)  true -> C3   false -> D
/// ```
/// `C3` is `A` when `shared` is set.
fn cascade(fd: &mut Funcdata, v: VarnodeId, shared: bool) -> BlockId {
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let b3 = new_block(fd, 0x1300);
    let a = new_block(fd, 0x2000);
    let b = new_block(fd, 0x2100);
    let c = if shared { a } else { new_block(fd, 0x2200) };
    let d = new_block(fd, 0x3000);
    cmp_branch(fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 2);
    edges(fd, b0, b2, b1);
    cmp_branch(fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 0);
    edges(fd, b1, d, a);
    cmp_branch(fd, b2, 0x1200, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(fd, b2, b3, b);
    cmp_branch(fd, b3, 0x1300, OpCode::CPUI_INT_EQUAL, v, 3);
    edges(fd, b3, d, c);
    b0
}

fn detect(fd: &mut Funcdata) -> (int4, Option<u64>) {
    let mut act = ActionLowerSwitchDetect::new(true, "base");
    let n = act.detect(fd);
    let at = act.store().borrow().records(fd).first().map(|r| r.branch_addr.get_offset());
    (n, at)
}

#[test]
fn heads_are_listed_in_block_order_and_only_the_first_when_off() {
    let mut fd = build_fd(true, true);
    let blocks: Vec<BlockId> = (0..3).map(|i| new_block(&mut fd, 0x1000 + i)).collect();
    let spine: BTreeSet<BlockId> = blocks.iter().copied().collect();
    let children: BTreeSet<BlockId> = [blocks[1]].into_iter().collect();
    assert_eq!(cascade_heads(&spine, &children, true), vec![blocks[0], blocks[2]]);
    assert_eq!(cascade_heads(&spine, &children, false), vec![blocks[0]]);
}

#[test]
fn the_cascade_behind_an_unrelated_compare_is_found_only_with_every_head() {
    let mut fd = build_fd(false, true);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    cascade(&mut fd, v, false);
    assert_eq!(detect(&mut fd), (0, None));

    let mut fd = build_fd(true, true);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    cascade(&mut fd, v, false);
    assert_eq!(detect(&mut fd), (1, Some(0x1001)), "the switch replaces the cascade head, not the decoy");
}

#[test]
fn a_later_head_is_checked_even_with_the_exact_gate_off() {
    // With one label per body the shared body loses case 3, so the later head is
    // refused; with every label it is exact and recorded.
    let mut fd = build_fd(true, false);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    cascade(&mut fd, v, true);
    assert_eq!(detect(&mut fd), (0, None));

    let mut fd = build_fd(true, true);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    cascade(&mut fd, v, true);
    assert_eq!(detect(&mut fd), (1, Some(0x1001)));

    let mut fd = build_fd(true, false);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    cascade(&mut fd, v, false);
    assert_eq!(detect(&mut fd), (1, Some(0x1001)), "distinct bodies are exact under either install");
}

#[test]
fn the_first_head_keeps_its_unchecked_acceptance_with_the_exact_gate_off() {
    let mut fd = build_fd(true, false);
    let v = switch_var(&mut fd, 4);
    cascade(&mut fd, v, true);
    assert_eq!(detect(&mut fd), (1, Some(0x1001)));
}

#[test]
fn a_later_head_entered_twice_or_with_a_body_entered_from_outside_is_refused() {
    let mut fd = build_fd(true, true);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    let head = cascade(&mut fd, v, false);
    let one = new_block(&mut fd, 0x0c00);
    fd.bblocks_mut().add_edge(one, head);
    assert_eq!(detect(&mut fd), (1, Some(0x1001)), "a head block with one predecessor");

    let mut fd = build_fd(true, true);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    let head = cascade(&mut fd, v, false);
    let one = new_block(&mut fd, 0x0c00);
    let two = new_block(&mut fd, 0x0d00);
    fd.bblocks_mut().add_edge(one, head);
    fd.bblocks_mut().add_edge(two, head);
    assert_eq!(detect(&mut fd), (0, None), "a head block with two predecessors");

    let mut fd = build_fd(true, true);
    let v = switch_var(&mut fd, 4);
    decoy(&mut fd, v);
    let head = cascade(&mut fd, v, false);
    let body = fd.bblocks_ref().block(head).get_out(1);
    let body = fd.bblocks_ref().block(body).get_out(1);
    let other = new_block(&mut fd, 0x0c00);
    fd.bblocks_mut().add_edge(other, body);
    assert_eq!(detect(&mut fd), (0, None), "a case body entered from outside the tree");
}
