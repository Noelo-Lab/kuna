//! Tests for the exact lowered-switch gate: the routing replay of a compare tree
//! against the switch the install builds, and the check that the install deletes
//! no statement.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::{ArchContext, TypeOp};
use crate::p2_lift::kuna_loweredswitch::{ActionLowerSwitchDetect, KunaLoweredSwitchRecord};

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

fn build_fd_with(exact: bool) -> Funcdata {
    let mut ctx = ArchContext::new(build_manager());
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

/// A `size`-byte switch variable written by a load in its own block.
fn switch_var(fd: &mut Funcdata, size: int4) -> VarnodeId {
    let bl = new_block(fd, 0x0f00);
    let load = fd.new_op(2, ram(fd, 0x0f00));
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

/// `out = code(v)` in a block of its own at `off`.
fn unary(fd: &mut Funcdata, code: OpCode, v: VarnodeId, out_size: int4, off: u64) -> VarnodeId {
    let bl = new_block(fd, off);
    let op = fd.new_op(if code == OpCode::CPUI_SUBPIECE { 2 } else { 1 }, ram(fd, off));
    fd.op_set_opcode(op, TypeOp::new(code, 0, "unary"));
    fd.op_set_input(op, v, 0).unwrap();
    if code == OpCode::CPUI_SUBPIECE {
        let zero = fd.new_constant(4, 0);
        fd.op_set_input(op, zero, 1).unwrap();
    }
    let out = fd.new_unique_out(out_size, op).unwrap();
    fd.op_insert_end(op, bl);
    out
}

/// End `bl` with `CBRANCH(code(v, c))`; the caller adds the false edge then the
/// true edge.
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

fn record(fd: &Funcdata, cases: &[(u64, u64)], default: u64, size: int4) -> KunaLoweredSwitchRecord {
    KunaLoweredSwitchRecord {
        branch_addr: ram(fd, 0x1001),
        var_addr: ram(fd, 0x4000),
        var_size: size,
        case_vals: cases.iter().map(|c| c.0).collect(),
        case_targets: cases.iter().map(|c| ram(fd, c.1)).collect(),
        default_target: ram(fd, default),
        signed_labels: false,
        value: crate::p2_lift::kuna_loweredswitchvalue::ValueCheck::default(),
        later_head: false,
    }
}

/// The shared-body tree:
///
/// ```text
///   B0: if (V < 2)   true -> B1   false -> B2
///   B1: if (V == 0)  true -> A    false -> D
///   B2: if (V == 2)  true -> B    false -> B3
///   B3: if (V == 3)  true -> A    false -> D
/// ```
struct Tree {
    head: BlockId,
    spine: BTreeSet<BlockId>,
    v: VarnodeId,
}

fn shared_body_tree(fd: &mut Funcdata) -> Tree {
    let v = switch_var(fd, 4);
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let b3 = new_block(fd, 0x1300);
    let a = new_block(fd, 0x2000);
    let b = new_block(fd, 0x2100);
    let d = new_block(fd, 0x3000);
    cmp_branch(fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 2);
    edges(fd, b0, b2, b1);
    cmp_branch(fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 0);
    edges(fd, b1, d, a);
    cmp_branch(fd, b2, 0x1200, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(fd, b2, b3, b);
    cmp_branch(fd, b3, 0x1300, OpCode::CPUI_INT_EQUAL, v, 3);
    edges(fd, b3, d, a);
    Tree { head: b0, spine: [b0, b1, b2, b3].into_iter().collect(), v }
}

#[test]
fn a_shared_body_routes_exactly_only_with_every_label() {
    let mut fd = build_fd_with(true);
    let t = shared_body_tree(&mut fd);
    let rec = record(&fd, &[(0, 0x2000), (2, 0x2100), (3, 0x2000)], 0x3000, 4);
    assert!(switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true));
    assert!(
        !switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, false),
        "with one label per body, 3 falls into the default"
    );
}

#[test]
fn a_moved_arm_a_missing_case_or_an_extra_case_is_refused() {
    let mut fd = build_fd_with(true);
    let t = shared_body_tree(&mut fd);
    let moved = record(&fd, &[(0, 0x2000), (2, 0x2000), (3, 0x2100)], 0x3000, 4);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &moved, true));
    let missing = record(&fd, &[(0, 0x2000), (2, 0x2100)], 0x3000, 4);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &missing, true));
    let extra = record(&fd, &[(0, 0x2000), (1, 0x2100), (2, 0x2100), (3, 0x2000)], 0x3000, 4);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &extra, true));
}

#[test]
fn a_default_that_is_a_case_body_is_refused() {
    let mut fd = build_fd_with(true);
    let t = shared_body_tree(&mut fd);
    let rec = record(&fd, &[(0, 0x2000), (2, 0x2100), (3, 0x2000), (4, 0x3000)], 0x2000, 4);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true));
}

#[test]
fn a_range_edge_pinned_to_one_value_must_be_a_case() {
    // B0: if (V u< 3)  true -> B1  false -> D
    // B1: if (V u< 2)  true -> B2  false -> C   (C is reached for V == 2 only)
    // B2: if (V == 0)  true -> A   false -> B
    let mut fd = build_fd_with(true);
    let v = switch_var(&mut fd, 4);
    let b0 = new_block(&mut fd, 0x1000);
    let b1 = new_block(&mut fd, 0x1100);
    let b2 = new_block(&mut fd, 0x1200);
    let a = new_block(&mut fd, 0x2000);
    let b = new_block(&mut fd, 0x2100);
    let c = new_block(&mut fd, 0x2200);
    let d = new_block(&mut fd, 0x3000);
    cmp_branch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 3);
    edges(&mut fd, b0, d, b1);
    cmp_branch(&mut fd, b1, 0x1100, OpCode::CPUI_INT_LESS, v, 2);
    edges(&mut fd, b1, c, b2);
    cmp_branch(&mut fd, b2, 0x1200, OpCode::CPUI_INT_EQUAL, v, 0);
    edges(&mut fd, b2, b, a);
    let spine: BTreeSet<BlockId> = [b0, b1, b2].into_iter().collect();
    let dropped = record(&fd, &[(0, 0x2000), (1, 0x2100)], 0x3000, 4);
    assert!(!switch_routes_like_cascade(&fd, b0, &spine, v, &dropped, true));
    let full = record(&fd, &[(0, 0x2000), (1, 0x2100), (2, 0x2200)], 0x3000, 4);
    assert!(switch_routes_like_cascade(&fd, b0, &spine, v, &full, true));
}

/// B0: if (V s< 0) true -> NEG false -> B1;  B1: if (V == 1) true -> ONE false -> D
fn signed_tree(fd: &mut Funcdata, code: OpCode, operand: impl Fn(&mut Funcdata, VarnodeId) -> VarnodeId, size: int4) -> Tree {
    let v = switch_var(fd, size);
    let x = operand(fd, v);
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let neg = new_block(fd, 0x2000);
    let one = new_block(fd, 0x2100);
    let d = new_block(fd, 0x3000);
    cmp_branch(fd, b0, 0x1000, code, x, 0);
    edges(fd, b0, b1, neg);
    cmp_branch(fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, x, 1);
    edges(fd, b1, d, one);
    Tree { head: b0, spine: [b0, b1].into_iter().collect(), v }
}

#[test]
fn signed_and_unsigned_thresholds_are_evaluated_as_written() {
    // Values 0x80..0xff of a signed byte take the negative arm: the record that
    // gives them no label is right only for the signed compare.
    let mut fd = build_fd_with(true);
    let t = signed_tree(&mut fd, OpCode::CPUI_INT_SLESS, |_, v| v, 1);
    let rec = record(&fd, &[(1, 0x2100)], 0x3000, 1);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true), "0x80 reaches NEG, not the default");
    let mut fd = build_fd_with(true);
    let t = signed_tree(&mut fd, OpCode::CPUI_INT_LESS, |_, v| v, 1);
    let rec = record(&fd, &[(1, 0x2100)], 0x3000, 1);
    assert!(switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true), "unsigned V < 0 never holds");
}

#[test]
fn a_sign_extended_operand_turns_negative_at_the_narrow_sign_bit() {
    let mut fd = build_fd_with(true);
    let t = signed_tree(&mut fd, OpCode::CPUI_INT_SLESS, |fd, v| unary(fd, OpCode::CPUI_INT_SEXT, v, 4, 0x0f10), 1);
    let rec = record(&fd, &[(1, 0x2100)], 0x3000, 1);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true));
    let mut fd = build_fd_with(true);
    let t = signed_tree(&mut fd, OpCode::CPUI_INT_SLESS, |fd, v| unary(fd, OpCode::CPUI_INT_ZEXT, v, 4, 0x0f10), 1);
    let rec = record(&fd, &[(1, 0x2100)], 0x3000, 1);
    assert!(switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true), "a zero-extended byte is never negative");
}

#[test]
fn a_narrowed_operand_is_refused() {
    let mut fd = build_fd_with(true);
    let t = signed_tree(&mut fd, OpCode::CPUI_INT_LESS, |fd, v| unary(fd, OpCode::CPUI_SUBPIECE, v, 1, 0x0f10), 4);
    let rec = record(&fd, &[(1, 0x2100)], 0x3000, 4);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true));
}

#[test]
fn values_at_the_wrap_points_are_replayed() {
    // B0: if (V == 0xffffffff) true -> A false -> B1;  B1: if (V == 0) -> B, else D
    let mut fd = build_fd_with(true);
    let v = switch_var(&mut fd, 4);
    let b0 = new_block(&mut fd, 0x1000);
    let b1 = new_block(&mut fd, 0x1100);
    let a = new_block(&mut fd, 0x2000);
    let b = new_block(&mut fd, 0x2100);
    let d = new_block(&mut fd, 0x3000);
    cmp_branch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 0xffff_ffff);
    edges(&mut fd, b0, b1, a);
    cmp_branch(&mut fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 0);
    edges(&mut fd, b1, d, b);
    let spine: BTreeSet<BlockId> = [b0, b1].into_iter().collect();
    assert!(!switch_routes_like_cascade(&fd, b0, &spine, v, &record(&fd, &[(0, 0x2100)], 0x3000, 4), true));
    assert!(switch_routes_like_cascade(&fd, b0, &spine, v, &record(&fd, &[(0, 0x2100), (0xffff_ffff, 0x2000)], 0x3000, 4), true));
    assert!(
        switch_routes_like_cascade(&fd, b0, &spine, v, &record(&fd, &[(0, 0x2100), (u64::MAX, 0x2000)], 0x3000, 4), true),
        "a sign-extended label is the same bit pattern"
    );
}

#[test]
fn a_label_wider_than_the_switch_variable_is_refused() {
    let mut fd = build_fd_with(true);
    let t = signed_tree(&mut fd, OpCode::CPUI_INT_LESS, |_, v| v, 1);
    let rec = record(&fd, &[(0x101, 0x2100)], 0x3000, 1);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true));
}

#[test]
fn a_spine_that_loops_is_refused() {
    let mut fd = build_fd_with(true);
    let v = switch_var(&mut fd, 4);
    let b0 = new_block(&mut fd, 0x1000);
    let b1 = new_block(&mut fd, 0x1100);
    let d = new_block(&mut fd, 0x3000);
    cmp_branch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 2);
    edges(&mut fd, b0, d, b1);
    cmp_branch(&mut fd, b1, 0x1100, OpCode::CPUI_INT_LESS, v, 5);
    edges(&mut fd, b1, d, b0);
    let spine: BTreeSet<BlockId> = [b0, b1].into_iter().collect();
    assert!(!switch_routes_like_cascade(&fd, b0, &spine, v, &record(&fd, &[], 0x3000, 4), true));
}

// -----------------------------------------------------------------------------
// install_keeps_statements
// -----------------------------------------------------------------------------

/// `B1: tmp = COPY 0x8a; if (V == 2) -> A else D`, with `reader` reading `tmp`.
fn statement_in_spine(fd: &mut Funcdata, read_in_join: bool) -> (BlockId, Vec<BlockId>) {
    let v = switch_var(fd, 4);
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let a = new_block(fd, 0x2000);
    let b = new_block(fd, 0x2100);
    let d = new_block(fd, 0x3000);
    cmp_branch(fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 1);
    edges(fd, b0, b1, b);
    let copy = fd.new_op(1, ram(fd, 0x1100));
    fd.op_set_opcode(copy, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    let k = fd.new_constant(4, 0x8a);
    fd.op_set_input(copy, k, 0).unwrap();
    let tmp_addr = ram(fd, 0x10);
    let tmp = fd.new_varnode_out(4, &tmp_addr, copy).unwrap();
    fd.op_insert_end(copy, b1);
    cmp_branch(fd, b1, 0x1101, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(fd, b1, d, a);
    let reader_block = if read_in_join { d } else { b1 };
    let reader = fd.new_op(2, ram(fd, 0x3000));
    fd.op_set_opcode(reader, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "ADD"));
    fd.op_set_input(reader, tmp, 0).unwrap();
    let one = fd.new_constant(4, 1);
    fd.op_set_input(reader, one, 1).unwrap();
    fd.new_unique_out(4, reader).unwrap();
    if read_in_join {
        fd.op_insert_end(reader, reader_block);
    } else {
        let tail = fd.bb_op_tail(b1).unwrap();
        fd.op_insert_before(reader, tail);
    }
    (b0, vec![a, b, d])
}

#[test]
fn a_value_a_deleted_block_computes_for_the_join_is_a_lost_statement() {
    let mut fd = build_fd_with(true);
    let (head, targets) = statement_in_spine(&mut fd, true);
    assert!(!install_keeps_statements(&fd, head, &targets));
}

#[test]
fn a_value_used_only_inside_the_deleted_blocks_is_not_a_statement() {
    let mut fd = build_fd_with(true);
    let (head, targets) = statement_in_spine(&mut fd, false);
    assert!(install_keeps_statements(&fd, head, &targets));
}

#[test]
fn a_store_or_call_in_a_deleted_block_is_a_lost_statement() {
    for code in [OpCode::CPUI_STORE, OpCode::CPUI_CALL] {
        let mut fd = build_fd_with(true);
        let v = switch_var(&mut fd, 4);
        let b0 = new_block(&mut fd, 0x1000);
        let b1 = new_block(&mut fd, 0x1100);
        let a = new_block(&mut fd, 0x2000);
        let d = new_block(&mut fd, 0x3000);
        cmp_branch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 1);
        edges(&mut fd, b0, b1, a);
        let eff = fd.new_op(1, ram(&fd, 0x1100));
        fd.op_set_opcode(eff, TypeOp::new(code, 0, "effect"));
        let k = fd.new_constant(8, 0x5000);
        fd.op_set_input(eff, k, 0).unwrap();
        fd.op_insert_end(eff, b1);
        cmp_branch(&mut fd, b1, 0x1101, OpCode::CPUI_INT_EQUAL, v, 2);
        edges(&mut fd, b1, d, a);
        assert!(!install_keeps_statements(&fd, b0, &[a, d]), "{code:?}");
    }
}

#[test]
fn a_block_still_reached_from_outside_the_cascade_is_not_deleted() {
    let mut fd = build_fd_with(true);
    let (head, targets) = statement_in_spine(&mut fd, true);
    let outside = new_block(&mut fd, 0x0e00);
    let b1 = fd.bblocks_ref().block(head).get_out(0);
    fd.bblocks_mut().add_edge(outside, b1);
    assert!(install_keeps_statements(&fd, head, &targets));
}

// -----------------------------------------------------------------------------
// detection
// -----------------------------------------------------------------------------

/// The shared-body tree with three distinct bodies, where B2 also computes
/// `tmp = 0x8a` for the default block D to read.
fn cascade_with_join_statement(fd: &mut Funcdata) {
    let v = switch_var(fd, 4);
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let b2 = new_block(fd, 0x1200);
    let b3 = new_block(fd, 0x1300);
    let a = new_block(fd, 0x2000);
    let b = new_block(fd, 0x2100);
    let c = new_block(fd, 0x2200);
    let d = new_block(fd, 0x3000);
    cmp_branch(fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 2);
    edges(fd, b0, b2, b1);
    cmp_branch(fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 0);
    edges(fd, b1, d, a);
    let copy = fd.new_op(1, ram(fd, 0x1200));
    fd.op_set_opcode(copy, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    let k = fd.new_constant(4, 0x8a);
    fd.op_set_input(copy, k, 0).unwrap();
    let tmp_addr = ram(fd, 0x10);
    let tmp = fd.new_varnode_out(4, &tmp_addr, copy).unwrap();
    fd.op_insert_end(copy, b2);
    cmp_branch(fd, b2, 0x1201, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(fd, b2, b3, b);
    cmp_branch(fd, b3, 0x1300, OpCode::CPUI_INT_EQUAL, v, 3);
    edges(fd, b3, d, c);
    let reader = fd.new_op(2, ram(fd, 0x3000));
    fd.op_set_opcode(reader, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "ADD"));
    fd.op_set_input(reader, tmp, 0).unwrap();
    let one = fd.new_constant(4, 1);
    fd.op_set_input(reader, one, 1).unwrap();
    fd.new_unique_out(4, reader).unwrap();
    fd.op_insert_end(reader, d);
}

#[test]
fn detection_keeps_a_shared_body_cascade() {
    let mut fd = build_fd_with(true);
    let _ = shared_body_tree(&mut fd);
    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1);
}

#[test]
fn detection_drops_a_cascade_that_would_lose_a_statement_only_when_exact() {
    let mut fd = build_fd_with(true);
    cascade_with_join_statement(&mut fd);
    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 0);
    assert!(!act.store().borrow().has_record(&fd));

    let mut fd = build_fd_with(false);
    cascade_with_join_statement(&mut fd);
    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1, "the unchecked install is kept with the option off");
}

/// `out = code(inputs)` appended to `bl` at `off`, writing `size` bytes at `ram:out`.
fn write_ram(fd: &mut Funcdata, bl: BlockId, off: u64, code: OpCode, inputs: &[VarnodeId], out: u64, size: int4) {
    let op = fd.new_op(inputs.len() as int4, ram(fd, off));
    fd.op_set_opcode(op, TypeOp::new(code, 0, "op"));
    for (i, &v) in inputs.iter().enumerate() {
        fd.op_set_input(op, v, i as int4).unwrap();
    }
    let a = ram(fd, out);
    fd.new_varnode_out(size, &a, op).unwrap();
    fd.op_insert_end(op, bl);
}

/// A raw two-compare cascade whose second compare block loads 7 into `ram:0x40`
/// (`mov esi,7 ; cmp edi,2 ; je T`) and sets a flag, with `T` reading `ram:0x40`.
/// With `body_writes_first` every target writes both locations before anything
/// else; the fixture has no prototype model, so leaving the function counts as
/// reading every location.
fn raw_load_in_spine(fd: &mut Funcdata, body_writes_first: bool) -> (BlockId, Vec<BlockId>) {
    let v = switch_var(fd, 4);
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let a = new_block(fd, 0x2000);
    let t = new_block(fd, 0x2100);
    let d = new_block(fd, 0x3000);
    cmp_branch(fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 1);
    edges(fd, b0, b1, a);
    let seven = fd.new_constant(4, 7);
    write_ram(fd, b1, 0x1100, OpCode::CPUI_COPY, &[seven], 0x40, 4);
    let flag_in = fd.new_constant(4, 0);
    write_ram(fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, &[flag_in, flag_in], 0x200, 1);
    cmp_branch(fd, b1, 0x1101, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(fd, b1, d, t);
    if body_writes_first {
        for (bl, off) in [(a, 0x2000), (t, 0x2100), (d, 0x3000)] {
            let one = fd.new_constant(4, 1);
            write_ram(fd, bl, off, OpCode::CPUI_COPY, &[one], 0x40, 4);
            let zero = fd.new_constant(1, 0);
            write_ram(fd, bl, off, OpCode::CPUI_COPY, &[zero], 0x200, 1);
        }
    }
    let at = ram(fd, 0x40);
    let esi = fd.new_varnode(4, &at, None);
    write_ram(fd, t, 0x2104, OpCode::CPUI_COPY, &[esi], 0x38, 4);
    (b0, vec![a, t, d])
}

#[test]
fn a_register_a_deleted_block_loads_for_a_body_is_a_lost_value() {
    let mut fd = build_fd_with(true);
    let (head, targets) = raw_load_in_spine(&mut fd, false);
    assert!(!raw_install_keeps_values(&fd, head, &targets));
}

#[test]
fn a_register_every_body_writes_before_reading_is_not_a_lost_value() {
    let mut fd = build_fd_with(true);
    let (head, targets) = raw_load_in_spine(&mut fd, true);
    assert!(raw_install_keeps_values(&fd, head, &targets));
}

#[test]
fn two_labels_for_one_value_are_refused() {
    let mut fd = build_fd_with(true);
    let t = shared_body_tree(&mut fd);
    let rec = record(&fd, &[(0, 0x2000), (0, 0x2100), (2, 0x2100), (3, 0x2000)], 0x3000, 4);
    assert!(!switch_routes_like_cascade(&fd, t.head, &t.spine, t.v, &rec, true));
}

#[test]
fn a_later_head_is_refused_when_a_body_is_entered_from_outside_the_tree() {
    use crate::p2_lift::kuna_loweredswitchheads::case_body_entered_elsewhere;
    let mut fd = build_fd_with(true);
    let v = switch_var(&mut fd, 4);
    let b0 = new_block(&mut fd, 0x1000);
    let b1 = new_block(&mut fd, 0x1100);
    let a = new_block(&mut fd, 0x2000);
    let c = new_block(&mut fd, 0x2100);
    let d = new_block(&mut fd, 0x3000);
    cmp_branch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 1);
    edges(&mut fd, b0, b1, a);
    cmp_branch(&mut fd, b1, 0x1101, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(&mut fd, b1, d, c);
    let rec = record(&fd, &[(1, 0x2000), (2, 0x2100)], 0x3000, 4);
    assert!(!case_body_entered_elsewhere(&fd, b0, &rec));
    fd.bblocks_mut().add_edge(a, c);
    assert!(case_body_entered_elsewhere(&fd, b0, &rec), "a body falling into another body");
    let mut fd = build_fd_with(true);
    let v = switch_var(&mut fd, 4);
    let b0 = new_block(&mut fd, 0x1000);
    let b1 = new_block(&mut fd, 0x1100);
    let a = new_block(&mut fd, 0x2000);
    let c = new_block(&mut fd, 0x2100);
    let d = new_block(&mut fd, 0x3000);
    let other = new_block(&mut fd, 0x0e00);
    cmp_branch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 1);
    edges(&mut fd, b0, b1, a);
    cmp_branch(&mut fd, b1, 0x1101, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(&mut fd, b1, d, c);
    fd.bblocks_mut().add_edge(other, d);
    let rec = record(&fd, &[(1, 0x2000), (2, 0x2100)], 0x3000, 4);
    assert!(case_body_entered_elsewhere(&fd, b0, &rec), "a default also entered after the switch");
}

/// A raw two-compare cascade whose second compare block loads 7 into `ram:0x40`;
/// every target writes `ram:0x40` first, except that `T` starts with a call when
/// `call_first` is set and writes it in its successor.
fn raw_call_in_body(fd: &mut Funcdata, call_first: bool) -> (BlockId, Vec<BlockId>) {
    let v = switch_var(fd, 4);
    let b0 = new_block(fd, 0x1000);
    let b1 = new_block(fd, 0x1100);
    let a = new_block(fd, 0x2000);
    let t = new_block(fd, 0x2100);
    let u = new_block(fd, 0x2200);
    let d = new_block(fd, 0x3000);
    cmp_branch(fd, b0, 0x1000, OpCode::CPUI_INT_EQUAL, v, 1);
    edges(fd, b0, b1, a);
    let seven = fd.new_constant(4, 7);
    write_ram(fd, b1, 0x1100, OpCode::CPUI_COPY, &[seven], 0x40, 4);
    cmp_branch(fd, b1, 0x1101, OpCode::CPUI_INT_EQUAL, v, 2);
    edges(fd, b1, d, t);
    for (bl, off) in [(a, 0x2000), (d, 0x3000), (u, 0x2200)] {
        let one = fd.new_constant(4, 1);
        write_ram(fd, bl, off, OpCode::CPUI_COPY, &[one], 0x40, 4);
    }
    if call_first {
        let call = fd.new_op(1, ram(fd, 0x2100));
        fd.op_set_opcode(call, TypeOp::new(OpCode::CPUI_CALL, 0, "CALL"));
        let dest = fd.new_constant(8, 0x5000);
        fd.op_set_input(call, dest, 0).unwrap();
        fd.op_insert_end(call, t);
    }
    fd.bblocks_mut().add_edge(t, u);
    (b0, vec![a, t, d])
}

#[test]
fn a_call_that_may_take_the_register_as_a_parameter_reads_it() {
    let mut fd = build_fd_with(true);
    let (head, targets) = raw_call_in_body(&mut fd, false);
    assert!(raw_install_keeps_values(&fd, head, &targets));
    let mut fd = build_fd_with(true);
    let (head, targets) = raw_call_in_body(&mut fd, true);
    assert!(!raw_install_keeps_values(&fd, head, &targets), "the call may pass the loaded register");
}
