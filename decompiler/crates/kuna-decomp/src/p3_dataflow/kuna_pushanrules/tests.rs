use super::*;
use crate::{context::ArchContext, context::TypeOp, dtype::type_metatype as Meta, dtype::Datatype};
use kuna_base::{
    address::Address,
    space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace},
};
use std::rc::Rc;

fn fixture(devirtualized: bool) -> (Funcdata, OpId) {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 2, addrspace_flags::hasphysical, 1, 1));
    manager.insert_space(ram.clone()).unwrap();
    let mut fd = Funcdata::new("vm", "vm", Rc::new(ArchContext::new(manager)), Address::new(ram.clone(), 0x1000), 0x10000000, 0x40).unwrap();
    if devirtualized {
        fd.kuna_set_devirtualized(true);
    }
    let root = fd.bblocks_ref().root.unwrap();
    let block = fd.bblocks_mut().new_block_basic(root);
    let anchor = fd.new_op(1, Address::new(ram, 0x1000));
    fd.op_set_opcode(anchor, TypeOp::new(CPUI_RETURN, 0, "RETURN"));
    fd.op_insert(anchor, block, None);
    (fd, anchor)
}

fn input(fd: &mut Funcdata, width: i32, offset: u64) -> VarnodeId {
    let ram = fd.get_arch().manage().get_space_by_name("ram").unwrap().clone();
    let vn = fd.vbank_mut().create(width, Address::new(ram, offset), Rc::new(Datatype::new(width, Meta::TYPE_UNKNOWN)));
    fd.vbank_mut().set_input(vn, &mut |_, _, _| Ok(())).unwrap()
}

fn k(fd: &mut Funcdata, width: i32, value: u64) -> VarnodeId {
    fd.new_constant(width, value & calc_mask(width))
}

struct Ops<'a> {
    fd: &'a mut Funcdata,
    anchor: OpId,
}

impl Ops<'_> {
    fn op(&mut self, code: OpCode, width: i32, inputs: &[VarnodeId]) -> VarnodeId {
        insert(self.fd, self.anchor, code, width, inputs)
    }
    fn not(&mut self, a: VarnodeId) -> VarnodeId {
        let w = size(self.fd, a);
        self.op(CPUI_INT_NEGATE, w, &[a])
    }
    fn and(&mut self, a: VarnodeId, b: VarnodeId) -> VarnodeId {
        let w = size(self.fd, a);
        self.op(CPUI_INT_AND, w, &[a, b])
    }
    fn or(&mut self, a: VarnodeId, b: VarnodeId) -> VarnodeId {
        let w = size(self.fd, a);
        self.op(CPUI_INT_OR, w, &[a, b])
    }
    fn add(&mut self, a: VarnodeId, b: VarnodeId) -> VarnodeId {
        let w = size(self.fd, a);
        self.op(CPUI_INT_ADD, w, &[a, b])
    }
    fn mul(&mut self, a: VarnodeId, c: u64) -> VarnodeId {
        let w = size(self.fd, a);
        let c = k(self.fd, w, c);
        self.op(CPUI_INT_MULT, w, &[a, c])
    }
    fn shr(&mut self, a: VarnodeId, c: u64) -> VarnodeId {
        let w = size(self.fd, a);
        let c = k(self.fd, 4, c);
        self.op(CPUI_INT_RIGHT, w, &[a, c])
    }
    fn sub(&mut self, a: VarnodeId, offset: u64, width: i32) -> VarnodeId {
        let c = k(self.fd, 4, offset);
        self.op(CPUI_SUBPIECE, width, &[a, c])
    }
    fn mask(&mut self, a: VarnodeId, c: u64) -> VarnodeId {
        let w = size(self.fd, a);
        let c = k(self.fd, w, c);
        self.and(a, c)
    }
}

fn evaluate(fd: &Funcdata, vn: VarnodeId, inputs: &[(VarnodeId, u64)]) -> u64 {
    let node = fd.vbank().get(vn).unwrap();
    let mask = calc_mask(node.get_size());
    if node.is_constant() {
        return node.get_offset() & mask;
    }
    if let Some((_, v)) = inputs.iter().find(|(id, _)| *id == vn) {
        return v & mask;
    }
    let op = fd.obank().get(node.get_def().unwrap()).unwrap();
    let a = evaluate(fd, op.get_in(0).unwrap(), inputs);
    let b = op.get_in(1).map(|v| evaluate(fd, v, inputs)).unwrap_or(0);
    let value = match op.code() {
        CPUI_COPY | CPUI_INT_ZEXT => a,
        CPUI_PIECE => (a << (8 * size(fd, op.get_in(1).unwrap()))) | b,
        CPUI_SUBPIECE => a.checked_shr(b as u32 * 8).unwrap_or(0),
        CPUI_INT_RIGHT => a.checked_shr(b as u32).unwrap_or(0),
        CPUI_INT_AND => a & b,
        CPUI_INT_OR => a | b,
        CPUI_INT_XOR => a ^ b,
        CPUI_INT_NEGATE => !a,
        CPUI_INT_2COMP => a.wrapping_neg(),
        CPUI_INT_ADD => a.wrapping_add(b),
        CPUI_INT_SUB => a.wrapping_sub(b),
        CPUI_INT_MULT => a.wrapping_mul(b),
        CPUI_INT_CARRY | CPUI_INT_SCARRY => {
            let in_mask = calc_mask(size(fd, op.get_in(0).unwrap()));
            let sum = a.wrapping_add(b) & in_mask;
            let sign = (in_mask >> 1) + 1;
            if op.code() == CPUI_INT_CARRY {
                u64::from(sum < a)
            } else {
                u64::from((a & sign) == (b & sign) && (sum & sign) != (a & sign))
            }
        }
        code => panic!("unexpected {code:?}"),
    };
    value & mask
}

fn samples(n: usize) -> Vec<u64> {
    let mut x = 0x9e3779b97f4a7c15u64;
    let mut out = vec![0, u64::MAX, 1, 0x8000000000000000, 0x7fffffffffffffff];
    while out.len() < n {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        out.push(x);
    }
    out
}

/// Apply `rule` to the definition of `root` and check the value is unchanged on
/// many assignments of `vars`; returns the rewritten op's code.
fn check(fd: &mut Funcdata, rule: &mut dyn Rule, root: VarnodeId, vars: &[VarnodeId]) -> Option<OpCode> {
    let values = samples(64);
    let assignments: Vec<Vec<(VarnodeId, u64)>> = (0..values.len())
        .map(|i| {
            vars.iter()
                .enumerate()
                .map(|(j, &v)| (v, values[(i * 7 + j * 13) % values.len()] & fd.vbank().get(v).unwrap().get_nz_mask()))
                .collect()
        })
        .collect();
    let before: Vec<u64> = assignments.iter().map(|a| evaluate(fd, root, a)).collect();
    let op = fd.vbank().get(root).unwrap().get_def().unwrap();
    if rule.apply_op(op, fd) == 0 {
        return None;
    }
    let after: Vec<u64> = assignments.iter().map(|a| evaluate(fd, root, a)).collect();
    assert_eq!(before, after);
    Some(fd.obank().get(op).unwrap().code())
}

fn root_inputs(fd: &Funcdata, root: VarnodeId) -> Vec<VarnodeId> {
    let op = fd.obank().get(fd.vbank().get(root).unwrap().get_def().unwrap()).unwrap();
    (0..op.num_input()).map(|i| op.get_in(i).unwrap()).collect()
}

fn logic() -> RulePushanLogic {
    RulePushanLogic::new("analysis")
}

#[test]
fn every_three_variable_function_has_a_correct_minimal_formula() {
    for n in 1..=3usize {
        let rows = 1usize << n;
        let table = formulas(n);
        assert_eq!(table.len(), 1 << rows);
        for (tt, _) in table.iter().enumerate() {
            let e = Expr::from_form(n, tt as u16, &(0..n).collect::<Vec<_>>());
            for s in 0..rows {
                let leaves: Vec<u64> = (0..n).map(|j| if s >> j & 1 == 1 { 1 } else { 0 }).collect();
                assert_eq!(e.eval(&leaves, 1), (tt >> s & 1) as u64, "n={n} tt={tt:#x}");
            }
        }
    }
    let xor = Expr::from_form(2, 0b0110, &[0, 1]);
    assert!(matches!(xor, Expr::Bin(CPUI_INT_XOR, _, _)));
    assert_eq!(xor.cost(&[0, 0]), 2);
}

#[test]
fn nor_gate_xor_of_a_value_and_its_shift_collapses_to_one_xor() {
    let (mut fd, anchor) = fixture(true);
    let v = input(&mut fd, 8, 0x100);
    let mut o = Ops { fd: &mut fd, anchor };
    let c1 = o.shr(v, 3);
    let c2 = o.shr(v, 3);
    let c3 = o.shr(v, 3);
    for c in [c1, c2, c3] {
        o.fd.vbank_mut().get_mut(c).unwrap().set_nz_mask(u64::MAX >> 3);
    }
    let (nv, nc1) = (o.not(v), o.not(c1));
    let nor = o.and(nv, nc1);
    let left = o.not(nor);
    let (nc2, nc3) = (o.not(c2), o.not(c3));
    let both = o.or(nc2, nc3);
    let c = o.not(both);
    let and = o.and(v, c);
    let right = o.not(and);
    let root = o.and(left, right);
    assert_eq!(check(&mut fd, &mut logic(), root, &[v]), Some(CPUI_INT_XOR));
    let inputs = root_inputs(&fd, root);
    assert!(inputs.contains(&v));
    assert!(inputs.iter().any(|&i| same_value(&fd, i, c1, SAME_DEPTH)));
}

#[test]
fn de_morgan_and_idempotence_collapse() {
    let (mut fd, anchor) = fixture(true);
    let a = input(&mut fd, 4, 0x100);
    let b = input(&mut fd, 4, 0x200);
    let mut o = Ops { fd: &mut fd, anchor };
    let (na, nb) = (o.not(a), o.not(b));
    let nor = o.and(na, nb);
    let or = o.not(nor);
    let (na2, nb2) = (o.not(a), o.not(b));
    let nand = o.or(na2, nb2);
    let and = o.not(nand);
    let a1 = o.shr(a, 3);
    let a2 = o.shr(a, 3);
    let same = o.and(a1, a2);
    let m1 = o.mask(a, 0xff);
    let m2 = o.mask(a, 0xff);
    let masked = o.or(m1, m2);
    assert_eq!(check(&mut fd, &mut logic(), or, &[a, b]), Some(CPUI_INT_OR));
    assert_eq!(root_inputs(&fd, or), vec![a, b]);
    assert_eq!(check(&mut fd, &mut logic(), and, &[a, b]), Some(CPUI_INT_AND));
    assert_eq!(check(&mut fd, &mut logic(), same, &[a]), Some(CPUI_COPY));
    assert_eq!(root_inputs(&fd, same), vec![a1]);
    assert_eq!(check(&mut fd, &mut logic(), masked, &[a]), Some(CPUI_INT_AND));
    assert_eq!(root_inputs(&fd, masked)[0], a);
}

#[test]
fn or_and_not_and_is_xor() {
    let (mut fd, anchor) = fixture(true);
    let a = input(&mut fd, 8, 0x100);
    let b = input(&mut fd, 8, 0x200);
    let mut o = Ops { fd: &mut fd, anchor };
    let or = o.or(a, b);
    let and = o.and(a, b);
    let nand = o.not(and);
    let root = o.and(or, nand);
    assert_eq!(check(&mut fd, &mut logic(), root, &[a, b]), Some(CPUI_INT_XOR));
}

#[test]
fn constant_masks_reduce_to_one_mask() {
    let (mut fd, anchor) = fixture(true);
    let x = input(&mut fd, 4, 0x100);
    let mut o = Ops { fd: &mut fd, anchor };
    let (n1, n2) = (o.not(x), o.not(x));
    let both = o.and(n1, n2);
    let low = o.mask(both, 0xffff);
    let inv = o.not(low);
    let root = o.mask(inv, 0x815);
    assert_eq!(check(&mut fd, &mut logic(), root, &[x]), Some(CPUI_INT_AND));
    let inputs = root_inputs(&fd, root);
    assert_eq!(inputs[0], x);
    assert_eq!(constant_value(&fd, inputs[1]), Some(0x815));
}

#[test]
fn masked_flag_word_keeps_the_tested_flag() {
    let (mut fd, anchor) = fixture(true);
    let zf = input(&mut fd, 4, 0x100);
    let cf = input(&mut fd, 4, 0x200);
    fd.vbank_mut().get_mut(zf).unwrap().set_nz_mask(0x40);
    fd.vbank_mut().get_mut(cf).unwrap().set_nz_mask(0x1);
    let mut o = Ops { fd: &mut fd, anchor };
    let reserved = k(o.fd, 4, 0x202);
    let word = o.or(zf, reserved);
    let word = o.or(word, cf);
    let root = o.mask(word, 0x40);
    assert_eq!(check(&mut fd, &mut logic(), root, &[zf, cf]), Some(CPUI_COPY));
    assert_eq!(root_inputs(&fd, root), vec![zf]);
}

#[test]
fn disjoint_masked_halves_merge_with_or() {
    let (mut fd, anchor) = fixture(true);
    let a = input(&mut fd, 4, 0x100);
    let b = input(&mut fd, 4, 0x200);
    let mut o = Ops { fd: &mut fd, anchor };
    let hi = o.mask(a, 0xff00);
    let lo = o.mask(b, 0x00ff);
    o.fd.vbank_mut().get_mut(hi).unwrap().set_nz_mask(0xff00);
    o.fd.vbank_mut().get_mut(lo).unwrap().set_nz_mask(0x00ff);
    let root = o.add(hi, lo);
    assert_eq!(check(&mut fd, &mut logic(), root, &[a, b]), Some(CPUI_INT_OR));
}

#[test]
fn ordinary_expressions_and_other_functions_are_left_alone() {
    let (mut fd, anchor) = fixture(true);
    let a = input(&mut fd, 8, 0x100);
    let b = input(&mut fd, 8, 0x200);
    let mut o = Ops { fd: &mut fd, anchor };
    let plain = o.and(a, b);
    let nb = o.not(b);
    let andnot = o.and(a, nb);
    let sum = o.add(a, b);
    assert_eq!(check(&mut fd, &mut logic(), plain, &[a, b]), None);
    assert_eq!(check(&mut fd, &mut logic(), andnot, &[a, b]), None);
    assert_eq!(check(&mut fd, &mut logic(), sum, &[a, b]), None);

    let (mut fd, anchor) = fixture(false);
    let a = input(&mut fd, 8, 0x100);
    let b = input(&mut fd, 8, 0x200);
    let mut o = Ops { fd: &mut fd, anchor };
    let (na, nb) = (o.not(a), o.not(b));
    let nor = o.and(na, nb);
    let or = o.not(nor);
    let pieces = [o.sub(a, 0, 4), o.shr(a, 32)];
    let hi = o.sub(pieces[1], 0, 4);
    let piece = o.op(CPUI_PIECE, 8, &[hi, pieces[0]]);
    let na2 = o.not(a);
    let nb2 = o.add(na2, b);
    let sub = o.not(nb2);
    assert_eq!(check(&mut fd, &mut logic(), or, &[a, b]), None);
    assert_eq!(check(&mut fd, &mut RulePushanPiece::new("analysis"), piece, &[a]), None);
    assert_eq!(check(&mut fd, &mut RulePushanArith::new("analysis"), sub, &[a, b]), None);
}

#[test]
fn not_of_not_plus_is_a_subtraction() {
    let (mut fd, anchor) = fixture(true);
    let x = input(&mut fd, 8, 0x100);
    let y = input(&mut fd, 8, 0x200);
    let mut o = Ops { fd: &mut fd, anchor };
    let nx = o.not(x);
    let sum = o.add(nx, y);
    let root = o.not(sum);
    let mut arith = RulePushanArith::new("analysis");
    assert_eq!(check(&mut fd, &mut arith, root, &[x, y]), Some(CPUI_INT_ADD));
    let inputs = root_inputs(&fd, root);
    assert_eq!(inputs[0], x);
    let scaled = fd.vbank().get(inputs[1]).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(scaled).unwrap().code(), CPUI_INT_MULT);
}

#[test]
fn negations_around_a_rotate_fold_into_one_constant() {
    let (mut fd, anchor) = fixture(true);
    let r = input(&mut fd, 4, 0x100);
    let mut o = Ops { fd: &mut fd, anchor };
    let nr = o.not(r);
    let neg = o.mul(nr, u64::MAX);
    let nn = o.not(neg);
    let c = k(o.fd, 4, 0xc65cfe5f);
    let root = o.add(nn, c);
    let one_minus = {
        let nx = o.not(r);
        let one = k(o.fd, 4, 1);
        o.add(nx, one)
    };
    let mut arith = RulePushanArith::new("analysis");
    assert_eq!(check(&mut fd, &mut arith, root, &[r]), Some(CPUI_INT_ADD));
    let inputs = root_inputs(&fd, root);
    assert_eq!(constant_value(&fd, inputs[1]), Some(0xc65cfe5d));
    assert_eq!(check(&mut fd, &mut arith, one_minus, &[r]), Some(CPUI_INT_MULT));
    let plain = {
        let mut o = Ops { fd: &mut fd, anchor };
        let nx = o.not(r);
        let c = k(o.fd, 4, 5);
        o.add(nx, c)
    };
    assert_eq!(check(&mut fd, &mut arith, plain, &[r]), None);
}

#[test]
fn pieces_of_one_shifted_value_rejoin() {
    let (mut fd, anchor) = fixture(true);
    let v = input(&mut fd, 8, 0x100);
    let mut o = Ops { fd: &mut fd, anchor };
    let shifted = o.shr(v, 3);
    let lo = o.sub(shifted, 0, 6);
    let top = o.shr(v, 0x33);
    let hi = o.sub(top, 0, 2);
    let root = o.op(CPUI_PIECE, 8, &[hi, lo]);
    let mut piece = RulePushanPiece::new("analysis");
    assert_eq!(check(&mut fd, &mut piece, root, &[v]), Some(CPUI_COPY));
    assert_eq!(root_inputs(&fd, root), vec![shifted]);
}

#[test]
fn piece_rejoin_preserves_every_bit() {
    let mut fired = 0;
    for base in [2, 4, 8] {
        for lo in 1..base {
            for hi in 1..=(8 - lo).min(base) {
                for shift in [0u64, 3, 8, 13, 16, 24] {
                    if shift >= base as u64 * 8 {
                        continue;
                    }
                    let (mut fd, anchor) = fixture(true);
                    let v = input(&mut fd, base, 0x100);
                    let mut o = Ops { fd: &mut fd, anchor };
                    let s1 = if shift == 0 { v } else { o.shr(v, shift) };
                    let lo_vn = o.sub(s1, 0, lo);
                    let hi_shift = shift + lo as u64 * 8;
                    let hi_vn = if hi_shift >= base as u64 * 8 {
                        continue;
                    } else {
                        let s2 = o.shr(v, hi_shift);
                        if hi > base {
                            continue;
                        }
                        o.sub(s2, 0, hi)
                    };
                    let root = o.op(CPUI_PIECE, lo + hi, &[hi_vn, lo_vn]);
                    if check(&mut fd, &mut RulePushanPiece::new("analysis"), root, &[v]).is_some() {
                        fired += 1;
                    }
                }
            }
        }
    }
    assert!(fired > 20, "fired {fired}");
}

#[test]
fn a_truncated_flags_word_keeps_only_the_masked_flags() {
    let (mut fd, anchor) = fixture(true);
    let af = input(&mut fd, 4, 0x100);
    let cf = input(&mut fd, 4, 0x200);
    fd.vbank_mut().get_mut(af).unwrap().set_nz_mask(0x10);
    fd.vbank_mut().get_mut(cf).unwrap().set_nz_mask(0x1);
    let mut o = Ops { fd: &mut fd, anchor };
    let image = k(o.fd, 4, 0x284);
    let word = o.or(af, image);
    let word = o.or(word, cf);
    let low = o.sub(word, 0, 2);
    let root = o.mask(low, 0x815);
    assert!(check(&mut fd, &mut logic(), root, &[af, cf]).is_some());
}

#[test]
fn an_addition_of_disjoint_values_never_carries() {
    let (mut fd, anchor) = fixture(true);
    let x = input(&mut fd, 4, 0x100);
    let y = input(&mut fd, 4, 0x200);
    let hi = input(&mut fd, 4, 0x300);
    fd.vbank_mut().get_mut(hi).unwrap().set_nz_mask(0xff00);
    let mut o = Ops { fd: &mut fd, anchor };
    let nx = o.not(x);
    let carry = o.op(CPUI_INT_CARRY, 1, &[nx, x]);
    let nx2 = o.not(x);
    let overflow = o.op(CPUI_INT_SCARRY, 1, &[x, nx2]);
    let low = o.mask(y, 0xff);
    let disjoint = o.op(CPUI_INT_CARRY, 1, &[hi, low]);
    let plain = o.op(CPUI_INT_CARRY, 1, &[x, y]);
    let mut rule = RulePushanCarry::new("analysis");
    assert_eq!(check(&mut fd, &mut rule, carry, &[x]), Some(CPUI_COPY));
    assert_eq!(check(&mut fd, &mut rule, overflow, &[x]), Some(CPUI_COPY));
    fd.vbank_mut().get_mut(low).unwrap().set_nz_mask(0xff);
    assert_eq!(check(&mut fd, &mut rule, disjoint, &[hi, y]), Some(CPUI_COPY));
    assert_eq!(check(&mut fd, &mut rule, plain, &[x, y]), None);
}

#[test]
fn a_zero_extended_pushf_image_keeps_only_the_masked_flags() {
    let (mut fd, anchor) = fixture(true);
    let af = input(&mut fd, 2, 0x100);
    let other = input(&mut fd, 2, 0x200);
    fd.vbank_mut().get_mut(af).unwrap().set_nz_mask(0x10);
    let mut o = Ops { fd: &mut fd, anchor };
    let image = k(o.fd, 2, 0x284);
    let word = o.or(af, image);
    let wide = o.op(CPUI_INT_ZEXT, 4, &[word]);
    let root = o.mask(wide, 0x815);
    assert!(check(&mut fd, &mut logic(), root, &[af]).is_some());

    let mut o = Ops { fd: &mut fd, anchor };
    let inverted = o.not(other);
    let wide = o.op(CPUI_INT_ZEXT, 4, &[inverted]);
    let root = o.mask(wide, 0xffff_0fff);
    let masked = o.mask(root, 0x1_0000);
    assert_eq!(check(&mut fd, &mut logic(), masked, &[other]), Some(CPUI_COPY));
    assert_eq!(constant_value(&fd, root_inputs(&fd, masked)[0]), Some(0));
}
