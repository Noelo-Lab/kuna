//! Tests for deciding a lowered switch's install before restarting for it: the
//! raw snapshot reads as the function it copied, the install's checks give the
//! same verdict on both, and detection takes a declined cascade without a
//! restart.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::{ArchContext, TypeOp};
use crate::p2_lift::kuna_loweredswitch::{new_shared_store, ActionLowerSwitchDetect, SharedLoweredSwitchStore};
use crate::p2_lift::kuna_loweredswitchvalue::ValueState;

fn build_fd() -> Funcdata {
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
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
}

fn new_block(fd: &mut Funcdata, off: u64) -> BlockId {
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let at = ram(fd, off);
    fd.set_basic_block_range(bl, &at, &at);
    bl
}

fn cmp_cbranch(fd: &mut Funcdata, bl: BlockId, off: u64, code: OpCode, v: VarnodeId, c: u64) {
    let cmp = fd.new_op(2, ram(fd, off));
    fd.op_set_opcode(cmp, TypeOp::new(code, 0, "cmp"));
    fd.op_set_input(cmp, v, 0).unwrap();
    let size = fd.vbank().get(v).unwrap().get_size();
    let cvn = fd.new_constant(size, c);
    fd.op_set_input(cmp, cvn, 1).unwrap();
    let b = fd.new_unique_out(1, cmp).unwrap();
    fd.op_insert_end(cmp, bl);
    let cb = fd.new_op(2, ram(fd, off + 1));
    fd.op_set_opcode(cb, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CPUI_CBRANCH"));
    let dest = fd.new_constant(8, 0);
    fd.op_set_input(cb, dest, 0).unwrap();
    fd.op_set_input(cb, b, 1).unwrap();
    fd.op_insert_end(cb, bl);
}

/// A call with five inputs in `bl`, so some op keeps inputs past the inline ones.
fn wide_call(fd: &mut Funcdata, bl: BlockId, off: u64) {
    let call = fd.new_op(5, ram(fd, off));
    fd.op_set_opcode(call, TypeOp::new(OpCode::CPUI_CALL, 0, "CALL"));
    for slot in 0..5 {
        let c = fd.new_constant(8, 0x7000 + slot as u64);
        fd.op_set_input(call, c, slot).unwrap();
    }
    fd.op_insert_end(call, bl);
}

/// The binary-search cascade of `kuna_loweredswitch`'s tests over `V = LOAD`:
/// `V < 2` at 0x1000, then `V == 0`, `V == 2`, `V == 3`, case bodies at 0x2000
/// and 0x2100, the default at 0x3000. Without `default` the default block is
/// left out, so the install cannot find it.
fn cascade(fd: &mut Funcdata, default: bool) {
    let def_bl = new_block(fd, 0x0F00);
    let load = fd.new_op(2, ram(fd, 0x0F00));
    fd.op_set_opcode(load, TypeOp::new(OpCode::CPUI_LOAD, 0, "LOAD"));
    let spaceid = fd.new_constant(4, 0);
    let ptr = fd.new_constant(8, 0x5000);
    fd.op_set_input(load, spaceid, 0).unwrap();
    fd.op_set_input(load, ptr, 1).unwrap();
    let vaddr = ram(fd, 0x4000);
    let v = fd.new_varnode_out(4, &vaddr, load).unwrap();
    fd.op_insert_end(load, def_bl);

    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let b3 = new_block(fd, 0x1300);
    let case_a = new_block(fd, 0x2000);
    let case_b = new_block(fd, 0x2100);
    let dflt = if default { new_block(fd, 0x3000) } else { new_block(fd, 0x3800) };
    fd.bblocks_mut().add_edge(def_bl, b0);
    cmp_cbranch(fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 2);
    fd.bblocks_mut().add_edge(b0, b2);
    fd.bblocks_mut().add_edge(b0, b1);
    cmp_cbranch(fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 0);
    fd.bblocks_mut().add_edge(b1, dflt);
    fd.bblocks_mut().add_edge(b1, case_a);
    cmp_cbranch(fd, b2, 0x1200, OpCode::CPUI_INT_EQUAL, v, 2);
    fd.bblocks_mut().add_edge(b2, b3);
    fd.bblocks_mut().add_edge(b2, case_b);
    cmp_cbranch(fd, b3, 0x1300, OpCode::CPUI_INT_EQUAL, v, 3);
    fd.bblocks_mut().add_edge(b3, dflt);
    fd.bblocks_mut().add_edge(b3, case_a);
    wide_call(fd, case_b, 0x2100);
}

fn recorded(fd: &mut Funcdata, store: &SharedLoweredSwitchStore) -> KunaLoweredSwitchRecord {
    let mut act = ActionLowerSwitchDetect::with_store(true, "base", Rc::clone(store));
    assert_eq!(act.detect(fd), 1, "the cascade is recorded");
    store.borrow().records(fd)[0].clone()
}

#[test]
fn the_snapshot_reads_as_the_function() {
    let mut fd = build_fd();
    cascade(&mut fd, true);
    let snap = RawSnapshot::capture(&fd);
    assert_eq!(snap.raw_num_blocks(), fd.raw_num_blocks());
    for i in 0..fd.raw_num_blocks() {
        let b = fd.raw_block(i);
        assert_eq!(snap.raw_block(i), b);
        assert_eq!(snap.raw_is_basic(b), fd.raw_is_basic(b));
        assert_eq!(snap.raw_is_entry(b), fd.raw_is_entry(b));
        assert_eq!(snap.raw_start(b), fd.raw_start(b));
        assert_eq!(snap.raw_size_in(b), fd.raw_size_in(b));
        assert_eq!(snap.raw_size_out(b), fd.raw_size_out(b));
        for k in 0..fd.raw_size_in(b) {
            assert_eq!(snap.raw_in(b, k), fd.raw_in(b, k));
        }
        for k in 0..fd.raw_size_out(b) {
            assert_eq!(snap.raw_out(b, k), fd.raw_out(b, k));
        }
        assert_eq!(snap.raw_ops(b), fd.raw_ops(b));
        assert_eq!(snap.raw_tail(b), fd.raw_tail(b));
        for op in fd.raw_ops(b) {
            let (a, o) = (snap.raw_op(op).unwrap(), fd.raw_op(op).unwrap());
            assert_eq!(RawOp::code(a), RawOp::code(o));
            assert_eq!(RawOp::get_addr(a), RawOp::get_addr(o));
            assert_eq!(RawOp::get_out(a), RawOp::get_out(o));
            assert_eq!(RawOp::num_input(a), RawOp::num_input(o));
            for slot in -1..RawOp::num_input(o) + 2 {
                assert_eq!(RawOp::get_in(a, slot), RawOp::get_in(o, slot));
            }
            for vn in (0..RawOp::num_input(o)).filter_map(|k| RawOp::get_in(o, k)).chain(RawOp::get_out(o)) {
                let (x, y) = (snap.raw_vn(vn).unwrap(), fd.raw_vn(vn).unwrap());
                assert_eq!(RawVn::get_addr(x), RawVn::get_addr(y));
                assert_eq!(RawVn::get_offset(x), RawVn::get_offset(y));
                assert_eq!(RawVn::get_size(x), RawVn::get_size(y));
                assert_eq!(RawVn::is_constant(x), RawVn::is_constant(y));
                assert_eq!(RawVn::is_written(x), RawVn::is_written(y));
                assert_eq!(RawVn::get_def(x), RawVn::get_def(y));
            }
        }
    }
    assert!(fd.raw_ops(fd.raw_block(6)).iter().any(|&op| RawOp::num_input(fd.raw_op(op).unwrap()) == 5));
    let at = ram(&fd, 0x1001);
    assert_eq!(snap.raw_table_at(&at), fd.raw_table_at(&at));
}

#[test]
fn the_install_checks_agree_on_the_snapshot() {
    let mut fd = build_fd();
    cascade(&mut fd, true);
    let store = new_shared_store();
    let rec = recorded(&mut fd, &store);
    let snap = RawSnapshot::capture(&fd);
    assert_eq!(install_declines(&snap, &rec), install_declines(&fd, &rec));
    let mut missing = rec.clone();
    missing.default_target = ram(&fd, 0x3333);
    assert!(install_declines(&fd, &missing));
    assert!(install_declines(&snap, &missing));
    let mut headless = rec.clone();
    headless.branch_addr = ram(&fd, 0x1002);
    assert!(install_declines(&fd, &headless) && install_declines(&snap, &headless));
    let mut path = rec;
    path.spine = vec![ram(&fd, 0x1001)];
    assert!(!install_declines(&fd, &path) && !install_declines(&snap, &path));
}

#[test]
fn a_cascade_the_install_declines_takes_no_restart() {
    let mut raw = build_fd();
    cascade(&mut raw, false);
    let store = new_shared_store();
    store.borrow_mut().begin_pass(&raw);
    let mut fd = build_fd();
    cascade(&mut fd, true);
    let rec = recorded(&mut fd, &store);
    assert!(!fd.has_restart_pending(), "a declined install needs no restart");
    assert_eq!(store.borrow().decided(&fd), Some(0));
    assert_eq!(rec.value.state, ValueState::Unchecked);
}

#[test]
fn an_install_that_may_succeed_restarts() {
    let mut fd = build_fd();
    cascade(&mut fd, true);
    let store = new_shared_store();
    store.borrow_mut().begin_pass(&fd);
    let rec = recorded(&mut fd, &store);
    assert!(!install_declines(&fd, &rec), "the fixture installs");
    assert!(fd.has_restart_pending());
    assert_eq!(store.borrow().decided(&fd), None);
}

#[test]
fn a_restarted_pass_ends_the_decision() {
    let mut raw = build_fd();
    cascade(&mut raw, false);
    let store = new_shared_store();
    store.borrow_mut().begin_pass(&raw);
    let mut fd = build_fd();
    cascade(&mut fd, true);
    let _ = recorded(&mut fd, &store);
    assert_eq!(store.borrow().decided(&fd), Some(0));
    store.borrow_mut().begin_pass(&fd);
    assert_eq!(store.borrow().decided(&fd), None, "a restarted pass installs for itself");
    assert!(store.borrow_mut().take_raw(&fd).is_none(), "a pass with a record keeps no snapshot");
}

#[test]
fn the_pass_counts_its_detection_calls() {
    let mut fd = build_fd();
    cascade(&mut fd, true);
    let store = new_shared_store();
    store.borrow_mut().begin_pass(&fd);
    store.borrow_mut().count_detect(&fd);
    store.borrow_mut().count_detect(&fd);
    let (calls, _) = store.borrow_mut().take_raw(&fd).expect("kept");
    assert_eq!(calls, 2);
    assert!(store.borrow_mut().take_raw(&fd).is_none(), "taken once");
}

#[test]
fn a_function_without_three_compares_of_one_location_keeps_no_snapshot() {
    let mut fd = build_fd();
    let b = new_block(&mut fd, 0x1000);
    wide_call(&mut fd, b, 0x1000);
    assert!(!worth_capturing(&fd));
    let mut cascade_fd = build_fd();
    cascade(&mut cascade_fd, true);
    assert!(worth_capturing(&cascade_fd), "four compares of the selector");
    let store = new_shared_store();
    store.borrow_mut().begin_pass(&fd);
    assert!(store.borrow_mut().take_raw(&fd).is_none());
}

#[test]
fn the_end_of_the_pass_settles_the_record() {
    for (base_shape, restarting, state) in [
        (false, false, ValueState::Withdrawn),
        (true, false, ValueState::Unchecked),
        (false, true, ValueState::Unchecked),
    ] {
        let mut raw = build_fd();
        cascade(&mut raw, false);
        let store = new_shared_store();
        store.borrow_mut().begin_pass(&raw);
        let mut fd = build_fd();
        cascade(&mut fd, true);
        let _ = recorded(&mut fd, &store);
        store.borrow_mut().record_mut(&fd, 0).unwrap().base_shape = base_shape;
        store.borrow_mut().settle_decided(&fd, restarting);
        assert_eq!(store.borrow().decided(&fd), None);
        assert_eq!(store.borrow().records(&fd)[0].value.state, state);
    }
}
