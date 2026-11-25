use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

use super::*;
use crate::action::Rule;
use crate::context::{ArchContext, TypeOp};
use crate::op::pcodeop_flags;

fn fd() -> Funcdata {
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
    Funcdata::new("sd", "sd", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata, at: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), at)
}

fn op(fd: &mut Funcdata, bl: BlockId, at: u64, opc: OpCode, ins: &[VarnodeId], size: int4) -> (OpId, VarnodeId) {
    let o = fd.new_op(ins.len() as int4, ram(fd, at));
    let flags = match opc {
        OpCode::CPUI_CBRANCH => pcodeop_flags::branch,
        OpCode::CPUI_MULTIEQUAL => pcodeop_flags::marker,
        _ => 0,
    };
    fd.obank_mut().change_opcode(o, TypeOp::new(opc, flags, format!("{opc:?}")));
    for (i, &v) in ins.iter().enumerate() {
        fd.op_set_input(o, v, i as int4).unwrap();
    }
    let out = fd.new_unique_out(size, o).unwrap();
    fd.op_insert(o, bl, None);
    (o, out)
}

/// The shape a test builds: which edge of the sign branch carries the add, how the
/// branch reads the sign, the two offsets of the root that meet at the join (the
/// dividend's first), and the shift.
struct Shape {
    size: int4,
    cond: OpCode,
    cond_x_first: bool,
    flip: bool,
    add_on_true_edge: bool,
    diamond: bool,
    x_in_arm: bool,
    base: u64,
    addend: u64,
    shift: u64,
}

const DIV8: Shape = Shape {
    size: 4,
    cond: OpCode::CPUI_INT_SLESS,
    cond_x_first: true,
    flip: false,
    add_on_true_edge: true,
    diamond: false,
    x_in_arm: false,
    base: 0,
    addend: 7,
    shift: 3,
};

struct Built {
    fd: Funcdata,
    join: BlockId,
    m: VarnodeId,
    x: VarnodeId,
}

/// `B: x = r + base; t = r + base + addend; c = <sign test of x>; CBRANCH c`
/// with `t` reaching the join `J: m = MULTIEQUAL(..)` over B's true or false
/// edge (through `A`), and `x` over the other (directly, or through `D`).
fn build(s: &Shape) -> Built {
    let mut fd = fd();
    let root = fd.bblocks_root_pub();
    let [b, a, d, j] = [(); 4].map(|_| fd.bblocks_mut().new_block_basic(root));
    let x_side = if s.diamond { d } else { j };
    let (false_to, true_to) = if s.add_on_true_edge { (x_side, a) } else { (a, x_side) };
    fd.bblocks_mut().add_edge(b, false_to);
    fd.bblocks_mut().add_edge(b, true_to);
    fd.bblocks_mut().add_edge(a, j);
    if s.diamond {
        fd.bblocks_mut().add_edge(d, j);
    }
    fd.bblocks_mut().set_start_block(root, b);
    let rv = fd.new_varnode(s.size, &ram(&fd, 0x100), None);
    let r = fd.set_input_varnode(rv).unwrap();
    let x = if s.base == 0 {
        r
    } else {
        let k = fd.new_constant(s.size, s.base);
        op(&mut fd, b, 0x1000, OpCode::CPUI_INT_ADD, &[r, k], s.size).1
    };
    let tested = x;
    let x = if s.x_in_arm {
        let k = fd.new_constant(s.size, s.base);
        op(&mut fd, d, 0x1010, OpCode::CPUI_INT_ADD, &[r, k], s.size).1
    } else {
        x
    };
    let k = fd.new_constant(s.size, (s.base + s.addend) & calc_mask(s.size));
    let (_, t) = op(&mut fd, b, 0x1002, OpCode::CPUI_INT_ADD, &[r, k], s.size);
    let k = match s.cond {
        OpCode::CPUI_INT_SLESS if s.cond_x_first => 0,
        OpCode::CPUI_INT_SLESSEQUAL if !s.cond_x_first => 0,
        _ => calc_mask(s.size),
    };
    let k = fd.new_constant(s.size, k);
    let ins = if s.cond_x_first { [tested, k] } else { [k, tested] };
    let (_, c) = op(&mut fd, b, 0x1004, s.cond, &ins, 1);
    let target = fd.new_constant(8, 0x1100);
    let (cb, _) = op(&mut fd, b, 0x1008, OpCode::CPUI_CBRANCH, &[target, c], 1);
    if s.flip {
        fd.op_flip_condition(cb);
    }
    let slot_a = fd.bblocks_ref().block(j).get_in_index(a);
    let mut ins = [x, x];
    ins[slot_a as usize] = t;
    let (_, m) = op(&mut fd, j, 0x1100, OpCode::CPUI_MULTIEQUAL, &ins, s.size);
    Built { fd, join: j, m, x }
}

/// The `INT_SDIV(x, 2^shift)` that `vn` reads, if it does.
fn quotient_of(fd: &Funcdata, vn: VarnodeId, x: VarnodeId, shift: u64) -> bool {
    let Some(o) = fd.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d)) else { return false };
    o.code() == OpCode::CPUI_INT_SDIV && o.get_in(0) == Some(x) && constant(fd, o.get_in(1).unwrap()) == Some(1 << shift)
}

fn divides(s: &Shape) -> bool {
    let mut b = build(s);
    let shift = b.fd.new_constant(s.size, s.shift);
    let (r, out) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_INT_SRIGHT, &[b.m, shift], s.size);
    if !recover(r, &mut b.fd) {
        assert_eq!(b.fd.obank().get(r).unwrap().code(), OpCode::CPUI_INT_SRIGHT);
        return false;
    }
    assert!(quotient_of(&b.fd, out, b.x, s.shift));
    true
}

#[test]
fn an_add_on_the_negative_edge_is_a_division() {
    assert!(divides(&DIV8));
    assert!(divides(&Shape { cond: OpCode::CPUI_INT_SLESSEQUAL, ..DIV8 }));
    assert!(divides(&Shape { size: 8, addend: 0x3ff, shift: 10, ..DIV8 }));
    assert!(divides(&Shape { size: 2, addend: 1, shift: 1, ..DIV8 }));
    assert!(divides(&Shape { diamond: true, ..DIV8 }));
}

#[test]
fn an_offset_dividend_is_divided_whole() {
    assert!(divides(&Shape { base: 0x200, addend: 0x3ff, shift: 10, ..DIV8 }));
    assert!(divides(&Shape { base: 0xffff_fff0, cond: OpCode::CPUI_INT_SLESSEQUAL, ..DIV8 }));
    assert!(!divides(&Shape { base: 0x200, addend: 0x5ff, shift: 10, ..DIV8 }));
}

#[test]
fn a_dividend_computed_in_its_own_arm_does_not_reach_the_join() {
    assert!(divides(&Shape { diamond: true, base: 0x200, addend: 0x3ff, shift: 10, ..DIV8 }));
    assert!(!divides(&Shape { diamond: true, x_in_arm: true, base: 0x200, addend: 0x3ff, shift: 10, ..DIV8 }));
}

#[test]
fn the_tested_sense_and_the_flip_pick_the_negative_edge() {
    assert!(divides(&Shape { cond_x_first: false, add_on_true_edge: false, ..DIV8 }));
    assert!(divides(&Shape { cond: OpCode::CPUI_INT_SLESSEQUAL, cond_x_first: false, add_on_true_edge: false, ..DIV8 }));
    assert!(divides(&Shape { flip: true, add_on_true_edge: false, ..DIV8 }));
    assert!(divides(&Shape { flip: true, cond_x_first: false, ..DIV8 }));
}

#[test]
fn an_add_on_the_non_negative_edge_is_not_a_division() {
    assert!(!divides(&Shape { add_on_true_edge: false, ..DIV8 }));
    assert!(!divides(&Shape { flip: true, ..DIV8 }));
    assert!(!divides(&Shape { cond_x_first: false, ..DIV8 }));
    assert!(!divides(&Shape { diamond: true, flip: true, ..DIV8 }));
}

#[test]
fn the_addend_must_be_one_less_than_the_divisor() {
    assert!(!divides(&Shape { addend: 8, ..DIV8 }));
    assert!(!divides(&Shape { addend: 3, ..DIV8 }));
    assert!(!divides(&Shape { shift: 2, ..DIV8 }));
    assert!(!divides(&Shape { cond: OpCode::CPUI_INT_LESS, ..DIV8 }));
}

#[test]
fn a_shift_into_the_sign_bit_is_not_a_division() {
    assert!(!divides(&Shape { addend: 0x7fff_ffff, shift: 31, ..DIV8 }));
    assert!(divides(&Shape { addend: 0x3fff_ffff, shift: 30, ..DIV8 }));
}

#[test]
fn a_logical_shift_divides_when_only_its_low_bits_are_read() {
    let mut b = build(&Shape { addend: 0xff, shift: 8, ..DIV8 });
    let eight = b.fd.new_constant(4, 8);
    let (r, out) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_INT_RIGHT, &[b.m, eight], 4);
    let zero = b.fd.new_constant(4, 0);
    let (low, _) = op(&mut b.fd, b.join, 0x1108, OpCode::CPUI_SUBPIECE, &[out, zero], 2);
    assert!(recover(r, &mut b.fd));
    assert!(quotient_of(&b.fd, out, b.x, 8));
    let _ = low;

    let mut b = build(&Shape { addend: 0xff, shift: 8, ..DIV8 });
    let eight = b.fd.new_constant(4, 8);
    let (r, out) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_INT_RIGHT, &[b.m, eight], 4);
    let zero = b.fd.new_constant(4, 0);
    op(&mut b.fd, b.join, 0x1108, OpCode::CPUI_SUBPIECE, &[out, zero], 1);
    op(&mut b.fd, b.join, 0x110c, OpCode::CPUI_SUBPIECE, &[out, zero], 4);
    assert!(!recover(r, &mut b.fd), "a read of the shifted-in high bits");
}

#[test]
fn a_byte_of_the_join_is_a_byte_of_the_quotient() {
    let mut b = build(&Shape { addend: 0xff, shift: 8, ..DIV8 });
    let one = b.fd.new_constant(4, 1);
    let (r, out) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_SUBPIECE, &[b.m, one], 1);
    assert!(recover(r, &mut b.fd));
    let o = b.fd.obank().get(r).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_SUBPIECE);
    assert_eq!(constant(&b.fd, o.get_in(1).unwrap()), Some(0));
    assert!(quotient_of(&b.fd, o.get_in(0).unwrap(), b.x, 8));
    let _ = out;

    let mut b = build(&Shape { addend: 0xff, shift: 8, ..DIV8 });
    let two = b.fd.new_constant(4, 2);
    let (r, _) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_SUBPIECE, &[b.m, two], 1);
    assert!(!recover(r, &mut b.fd), "byte 2 is not the quotient's low byte");
}

#[test]
fn the_join_rounded_down_is_the_quotient_scaled_back() {
    let mut b = build(&DIV8);
    let mask = b.fd.new_constant(4, 0xffff_fff8);
    let (r, _) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_INT_AND, &[b.m, mask], 4);
    assert!(recover(r, &mut b.fd));
    let o = b.fd.obank().get(r).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_MULT);
    assert_eq!(constant(&b.fd, o.get_in(1).unwrap()), Some(8));
    assert!(quotient_of(&b.fd, o.get_in(0).unwrap(), b.x, 3));

    let mut b = build(&DIV8);
    let mask = b.fd.new_constant(4, 0xffff_fff0);
    let (r, _) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_INT_AND, &[b.m, mask], 4);
    assert!(!recover(r, &mut b.fd));
}

#[test]
fn divopt_recovers_the_conditional_form() {
    let mut b = build(&DIV8);
    let shift = b.fd.new_constant(4, 3);
    let (r, _) = op(&mut b.fd, b.join, 0x1104, OpCode::CPUI_INT_SRIGHT, &[b.m, shift], 4);
    assert_eq!(crate::ruleaction_6::RuleDivOpt::new("analysis").apply_op(r, &mut b.fd), 1);
    assert_eq!(b.fd.obank().get(r).unwrap().code(), OpCode::CPUI_INT_SDIV);
}
