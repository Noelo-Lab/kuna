//! Tests for the uncomputed-return-trial rule.
//!
//! These pin the *classification* — which value shapes count as computed — on a
//! hand-built `Funcdata`. The end-to-end witness (a real binary whose `main`
//! stops returning a 16-byte phantom) lives in
//! `kuna-console/tests/verify_return_uncomputed.rs`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, JoinSpace, UniqueSpace, VarnodeStorage,
};

use crate::context::{ArchContext, TypeOp};

fn computes_a_value(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    computes_from(data, vn, depth, None)
}

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

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

/// A Varnode at a ram address with no defining op — a function input, or a
/// location the function only ever reads.
fn unwritten(fd: &mut Funcdata, off: u64, sz: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(sz, &Address::new(r, off), None)
}

/// Build `out = <opc>(inputs...)` at a fresh ram address and return `out`.
fn mk_def(fd: &mut Funcdata, opc: OpCode, inputs: &[VarnodeId], out_off: u64) -> VarnodeId {
    let r = ram(fd);
    let op = fd.new_op(inputs.len() as int4, Address::new(Rc::clone(&r), out_off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    for (i, &vn) in inputs.iter().enumerate() {
        fd.op_set_input(op, vn, i as int4).expect("wire input");
    }
    fd.new_varnode_out(8, &Address::new(r, out_off), op).expect("varnode out")
}

#[test]
fn a_constant_is_a_computed_return_value() {
    let mut fd = build_fd();
    let k = fd.new_constant(8, 7);
    assert!(
        computes_a_value(&fd, k, 0),
        "`return 0;` is a real return — a literal must never be dropped",
    );
}

#[test]
fn an_unwritten_varnode_is_not_a_computed_return_value() {
    let mut fd = build_fd();
    let vn = unwritten(&mut fd, 0x2000, 8);
    assert!(
        !computes_a_value(&fd, vn, 0),
        "an unwritten Varnode carries whatever the caller left there",
    );
}

#[test]
fn a_copy_chases_through_to_its_source() {
    let mut fd = build_fd();

    let leftover = unwritten(&mut fd, 0x2000, 8);
    let copied = mk_def(&mut fd, OpCode::CPUI_COPY, &[leftover], 0x2100);
    assert!(!computes_a_value(&fd, copied, 0), "a copy of leftover is still leftover");

    let k = fd.new_constant(8, 5);
    let copied_k = mk_def(&mut fd, OpCode::CPUI_COPY, &[k], 0x2200);
    assert!(computes_a_value(&fd, copied_k, 0), "a copy of a literal is a real value");
}

#[test]
fn arithmetic_is_a_computed_return_value() {
    let mut fd = build_fd();
    // Both inputs unwritten, but INT_ADD is not a move — the walk stops here.
    // This is the guard that keeps a genuine struct return whose halves are built
    // from parameters.
    let a = unwritten(&mut fd, 0x2000, 8);
    let b = unwritten(&mut fd, 0x2008, 8);
    let sum = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], 0x2100);
    assert!(computes_a_value(&fd, sum, 0), "arithmetic produces a value");
}

#[test]
fn a_load_is_a_computed_return_value() {
    let mut fd = build_fd();
    // `struct S get(struct S *p) { return *p; }` builds both halves with LOADs
    // through an (unwritten) parameter. A LOAD is not a move, so the walk stops
    // and the half is kept — this is the false positive this rule must not have.
    let spc = fd.new_constant(4, 0);
    let ptr = unwritten(&mut fd, 0x2000, 8);
    let loaded = mk_def(&mut fd, OpCode::CPUI_LOAD, &[spc, ptr], 0x2100);
    assert!(
        computes_a_value(&fd, loaded, 0),
        "a load through a pointer is a real value — a struct return reads this way",
    );
}

#[test]
fn a_phi_is_computed_when_any_arm_is() {
    let mut fd = build_fd();
    let leftover = unwritten(&mut fd, 0x2000, 8);
    let k = fd.new_constant(8, 3);
    let phi = mk_def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[leftover, k], 0x2100);
    assert!(
        computes_a_value(&fd, phi, 0),
        "one real arm is enough — the function returns a value on that path",
    );
}

#[test]
fn a_phi_of_only_leftover_is_not_computed() {
    let mut fd = build_fd();
    let a = unwritten(&mut fd, 0x2000, 8);
    let b = unwritten(&mut fd, 0x2008, 8);
    let phi = mk_def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[a, b], 0x2100);
    assert!(
        !computes_a_value(&fd, phi, 0),
        "leftover on every path is still leftover",
    );
}

#[test]
fn the_strict_walk_refuses_a_phi_one_arm_of_which_is_leftover() {
    let mut fd = build_fd();
    let leftover = unwritten(&mut fd, 0x2000, 8);
    let k = fd.new_constant(8, 3);
    let phi = mk_def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[leftover, k], 0x2100);
    assert!(
        computes_a_value(&fd, phi, 0),
        "the pair repair's question: one real arm is enough",
    );
    assert!(
        !computes_everywhere(&fd, phi, None, false),
        "a caller adopting the whole value needs every path to produce one",
    );
}

#[test]
fn the_strict_walk_refuses_a_piece_whose_high_half_is_leftover() {
    let mut fd = build_fd();
    let leftover = unwritten(&mut fd, 0x2000, 4);
    let k = fd.new_constant(4, 7);
    let joined = mk_def(&mut fd, OpCode::CPUI_PIECE, &[leftover, k], 0x2100);
    assert!(
        !computes_everywhere(&fd, joined, None, false),
        "version_etc_arn's CONCAT44(<leftover>, call result) is not a return value",
    );
}

#[test]
fn the_strict_walk_keeps_a_value_every_byte_of_which_is_computed() {
    let mut fd = build_fd();
    let k1 = fd.new_constant(4, 1);
    let k2 = fd.new_constant(4, 2);
    let joined = mk_def(&mut fd, OpCode::CPUI_PIECE, &[k1, k2], 0x2100);
    let copy = mk_def(&mut fd, OpCode::CPUI_COPY, &[joined], 0x2200);
    assert!(
        computes_everywhere(&fd, copy, None, false),
        "a value built from constants is a return value in every byte",
    );
}

#[test]
fn the_strict_walk_stops_at_an_operation_that_produces_a_value() {
    let mut fd = build_fd();
    let leftover = unwritten(&mut fd, 0x2000, 8);
    let leftover = fd.set_input_varnode(leftover).expect("function input");
    let sum = mk_def(&mut fd, OpCode::CPUI_INT_ADD, &[leftover, leftover], 0x2100);
    assert!(
        computes_everywhere(&fd, sum, None, false),
        "arithmetic over leftover is still a value the function computed",
    );
}

// --- the pair repair on a value held in ONE return register -----------------

fn build_fd_with_blocks() -> (Funcdata, crate::context::BlockId) {
    let mut m = build_manager();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "breg",
        true,
        8,
        1,
        3,
        0,
        1,
        1,
    )))
    .unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let mut fd = Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x1000_0000, 0x40).unwrap();
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let bl = fd.bblocks_mut().new_block_basic(root);
    (fd, bl)
}

/// A live `out = <opc>(inputs...)` in `bl`, `out` sized `size` at ram `out_off`.
fn live_def(fd: &mut Funcdata, bl: crate::context::BlockId, opc: OpCode, inputs: &[VarnodeId], out_off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let op = fd.new_op(inputs.len() as int4, Address::new(Rc::clone(&r), out_off));
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    for (i, &vn) in inputs.iter().enumerate() {
        fd.op_set_input(op, vn, i as int4).expect("wire input");
    }
    fd.op_insert(op, bl, None);
    fd.new_varnode_out(size, &Address::new(r, out_off), op).expect("varnode out")
}

/// `return <value>;` as a live RETURN in `bl`.
fn live_return(fd: &mut Funcdata, bl: crate::context::BlockId, value: VarnodeId) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(2, Address::new(r, 0x1ff0));
    fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_RETURN, 0, "RETURN"));
    let k = fd.new_constant(8, 0);
    fd.op_set_input(op, k, 0).expect("wire input");
    fd.op_set_input(op, value, 1).expect("wire input");
    fd.op_insert(op, bl, None);
    op
}

#[test]
fn each_half_of_one_register_sits_in_its_own_bytes() {
    let (mut fd, _) = build_fd_with_blocks();
    let r = ram(&fd);
    let whole = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x3000), None);
    assert_eq!(slot_storage(&fd, whole, 0, 4), Some(Address::new(Rc::clone(&r), 0x3000)), "little-endian low half");
    assert_eq!(slot_storage(&fd, whole, 4, 4), Some(Address::new(Rc::clone(&r), 0x3004)), "little-endian high half");
    assert!(!spans_two_locations(&fd, whole), "one register is one location");

    let be = Rc::clone(fd.get_arch().manage().get_space_by_name("breg").unwrap());
    let whole_be = fd.new_varnode(8, &Address::new(Rc::clone(&be), 0x100), None);
    assert_eq!(slot_storage(&fd, whole_be, 0, 4), Some(Address::new(Rc::clone(&be), 0x104)), "big-endian low half");
    assert_eq!(slot_storage(&fd, whole_be, 4, 4), Some(Address::new(be, 0x100)), "big-endian high half");
}

#[test]
fn a_never_written_high_half_of_one_register_is_still_dropped() {
    // `RAX = PIECE(<RAX's own high bytes, never written>, EAX = a + b)`: the
    // function computed only the low half into its return register.
    let (mut fd, bl) = build_fd_with_blocks();
    let r = ram(&fd);
    let leftover = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x3004), None);
    let a = unwritten(&mut fd, 0x2000, 4);
    let b = unwritten(&mut fd, 0x2008, 4);
    let sum = live_def(&mut fd, bl, OpCode::CPUI_INT_ADD, &[a, b], 0x3000, 4);
    let whole = live_def(&mut fd, bl, OpCode::CPUI_PIECE, &[leftover, sum], 0x3000, 8);
    let ret = live_return(&mut fd, bl, whole);
    assert!(strip_uncomputed_return_piece(&mut fd));
    assert_eq!(fd.obank().get(ret).unwrap().get_in(1), Some(sum), "the return narrows to the computed low half");
}

#[test]
fn the_high_half_of_one_register_is_never_returned_alone() {
    // `RAX = PIECE(a + b, <EAX, never written>)`: returning the sum by itself
    // would hand back the high 32 bits as the whole value.
    let (mut fd, bl) = build_fd_with_blocks();
    let r = ram(&fd);
    let a = unwritten(&mut fd, 0x2000, 4);
    let b = unwritten(&mut fd, 0x2008, 4);
    let sum = live_def(&mut fd, bl, OpCode::CPUI_INT_ADD, &[a, b], 0x3100, 4);
    let leftover = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x3000), None);
    let whole = live_def(&mut fd, bl, OpCode::CPUI_PIECE, &[sum, leftover], 0x3000, 8);
    let ret = live_return(&mut fd, bl, whole);
    assert!(!strip_uncomputed_return_piece(&mut fd), "nothing is dropped from a value built in one register");
    assert_eq!(fd.obank().get(ret).unwrap().get_in(1), Some(whole), "the return keeps all eight bytes");
}

// --- a literal zero low half the rules folded into the pair ------------------

/// A function with a big-endian register pair `breg:0x100` (first, high per
/// the ABI) and `breg:0x104`, the return value joined first register low as
/// `0x104:0x100`, and `return ZEXT(hi) << 32` at that join.
fn folded_zero_pair(hi_computed: bool) -> (Funcdata, OpId, VarnodeId) {
    let mut m = build_manager();
    m.insert_space(Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "breg", true, 8, 1, 3, 0, 1, 1))).unwrap();
    m.insert_space(Rc::new(JoinSpace::new(4, false))).unwrap();
    let breg = Rc::clone(m.get_space_by_name("breg").unwrap());
    let piece = |off| VarnodeStorage { space: Some(Rc::clone(&breg)), offset: off, size: 4 };
    let join = m.find_add_join(&[piece(0x104), piece(0x100)], 0).unwrap().get_unified().get_addr();
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let mut fd = Funcdata::new("func", "func", glb, Address::new(Rc::clone(&ram), 0x1000), 0x1000_0000, 0x40).unwrap();
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let bl = fd.bblocks_mut().new_block_basic(root);
    let arg = fd.new_varnode(4, &Address::new(Rc::clone(&breg), 0x104), None);
    let hi = if hi_computed {
        let k = fd.new_constant(4, 3);
        live_def(&mut fd, bl, OpCode::CPUI_INT_MULT, &[arg, k], 0x3200, 4)
    } else {
        arg
    };
    let ext = live_def(&mut fd, bl, OpCode::CPUI_INT_ZEXT, &[hi], 0x3100, 8);
    let op = fd.new_op(2, Address::new(Rc::clone(&ram), 0x1fe0));
    fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_INT_LEFT, 0, "INT_LEFT"));
    let sa = fd.new_constant(4, 32);
    fd.op_set_input(op, ext, 0).expect("wire input");
    fd.op_set_input(op, sa, 1).expect("wire input");
    fd.op_insert(op, bl, None);
    let whole = fd.new_varnode_out(8, &join, op).expect("varnode out");
    let ret = live_return(&mut fd, bl, whole);
    (fd, ret, whole)
}

#[test]
fn the_folded_form_of_a_zero_low_half_is_read_as_the_pair() {
    let (fd, _, whole) = folded_zero_pair(false);
    let def = fd.vbank().get(whole).and_then(|v| v.get_def()).unwrap();
    let (hi, lo_size, out) = zero_low_pair(&fd, def).expect("ZEXT(hi) << 32 is PIECE(hi, 0:4)");
    assert_eq!((fd.vbank().get(hi).unwrap().get_size(), lo_size, out), (4, 4, whole));
}

#[test]
fn a_leftover_beside_a_folded_zero_is_dropped_from_a_pair_joined_against_the_abi() {
    let (mut fd, ret, whole) = folded_zero_pair(false);
    fd.kuna_set_pairs_first_low(true);
    assert!(strip_uncomputed_return_piece(&mut fd));
    let kept = fd.obank().get(ret).unwrap().get_in(1).unwrap();
    assert_ne!(kept, whole);
    let v = fd.vbank().get(kept).unwrap();
    assert!(v.is_constant() && v.get_offset() == 0 && v.get_size() == 4, "the RETURN returns the 4-byte zero");
}

#[test]
fn a_folded_zero_is_left_alone_where_the_pair_joins_in_the_abis_order() {
    let (mut fd, ret, whole) = folded_zero_pair(false);
    assert!(!strip_uncomputed_return_piece(&mut fd), "the folded form can be the value");
    assert_eq!(fd.obank().get(ret).unwrap().get_in(1), Some(whole));
}

#[test]
fn a_computed_high_half_beside_a_folded_zero_keeps_the_pair() {
    let (mut fd, ret, whole) = folded_zero_pair(true);
    fd.kuna_set_pairs_first_low(true);
    assert!(!strip_uncomputed_return_piece(&mut fd));
    assert_eq!(fd.obank().get(ret).unwrap().get_in(1), Some(whole));
}
