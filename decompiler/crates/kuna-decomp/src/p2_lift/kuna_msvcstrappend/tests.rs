use super::*;

fn load(syms: &mut Syms, off: u64) -> Val {
    let base = syms.atom(Atom::Leaf { space: 1, off: 0x20, size: 8, epoch: 0 }, 8);
    syms.atom(Atom::Load { addr: base.plus(off), size: 8, ver: 0 }, 8)
}

fn id(v: &Val) -> u32 {
    v.terms[0].0
}

/// `CMP a,b` flags as SLEIGH lowers them: CF = a < b, ZF = (a - b) == 0.
fn cmp_flags(syms: &mut Syms, a: &Val, b: &Val) -> (Val, Val) {
    let cf = syms.eval(OpCode::CPUI_INT_LESS, vec![a.clone(), b.clone()], 1);
    let diff = syms.eval(OpCode::CPUI_INT_SUB, vec![a.clone(), b.clone()], 8);
    let zf = syms.eval(OpCode::CPUI_INT_EQUAL, vec![diff, Val::konst(0, 8)], 1);
    (cf, zf)
}

#[test]
fn jnc_over_size_and_capacity_is_size_less_than_capacity() {
    let mut s = Syms::default();
    let size = load(&mut s, 0x10);
    let res = load(&mut s, 0x18);
    let (cf, _) = cmp_flags(&mut s, &size, &res);
    let jnc = s.eval(OpCode::CPUI_BOOL_NEGATE, vec![cf], 1);
    // JNC to the grow arm: the fast arm is the not-taken edge.
    assert!(s.decide_two(&jnc, false, id(&size), id(&res), |a, b| a < b));
    assert!(!s.decide_two(&jnc, true, id(&size), id(&res), |a, b| a < b));
}

#[test]
fn jbe_with_operands_swapped_is_the_same_decision() {
    let mut s = Syms::default();
    let size = load(&mut s, 0x10);
    let res = load(&mut s, 0x18);
    let (cf, zf) = cmp_flags(&mut s, &res, &size);
    let be = s.eval(OpCode::CPUI_BOOL_OR, vec![cf, zf], 1);
    assert!(s.decide_two(&be, false, id(&size), id(&res), |a, b| a < b));
}

#[test]
fn a_non_boolean_constant_is_not_a_condition() {
    let mut s = Syms::default();
    let size = load(&mut s, 0x10);
    let res = load(&mut s, 0x18);
    let (cf, _) = cmp_flags(&mut s, &size, &res);
    // `CF | 2` is never zero, so a branch on it is always taken.
    let never_zero = s.eval(OpCode::CPUI_INT_OR, vec![cf, Val::konst(2, 1)], 1);
    assert!(!s.decide_two(&never_zero, true, id(&size), id(&res), |a, b| a < b));
}

#[test]
fn a_signed_compare_is_not_decided() {
    let mut s = Syms::default();
    let size = load(&mut s, 0x10);
    let res = load(&mut s, 0x18);
    let lt = s.eval(OpCode::CPUI_INT_SLESS, vec![size.clone(), res.clone()], 1);
    assert!(!s.decide_two(&lt, true, id(&size), id(&res), |a, b| a < b));
}

#[test]
fn cmova_against_fifteen_splits_small_from_heap() {
    let mut s = Syms::default();
    let res = load(&mut s, 0x18);
    let (cf, zf) = cmp_flags(&mut s, &res, &Val::konst(15, 8));
    let ncf = s.eval(OpCode::CPUI_BOOL_NEGATE, vec![cf], 1);
    let nzf = s.eval(OpCode::CPUI_BOOL_NEGATE, vec![zf], 1);
    let above = s.eval(OpCode::CPUI_BOOL_AND, vec![ncf, nzf], 1);
    let skip = s.eval(OpCode::CPUI_BOOL_NEGATE, vec![above], 1);
    // The CMOV skips its assignment (small-string path) when the branch is taken.
    let pts = s.classify_one(&[(skip.clone(), true)], &res, &[15, 16]).unwrap();
    assert!(pts.iter().all(|&(v, t)| t == (v <= 15)));
    let pts = s.classify_one(&[(skip, false)], &res, &[15, 16]).unwrap();
    assert!(pts.iter().all(|&(v, t)| t == (v > 15)));
}

#[test]
fn append_room_check_is_decided_over_the_difference() {
    let mut s = Syms::default();
    let size = load(&mut s, 0x10);
    let res = load(&mut s, 0x18);
    let room = res.sub(&size);
    let (cf, _) = cmp_flags(&mut s, &room, &Val::konst(2, 8));
    let pts = s.classify_one(&[(cf, false)], &room, &[2]).unwrap();
    assert!(pts.iter().all(|&(d, t)| t == (d >= 2)));
}

#[test]
fn an_offset_ordered_compare_is_rejected() {
    let mut s = Syms::default();
    let res = load(&mut s, 0x18);
    let shifted = res.plus(5);
    let lt = s.eval(OpCode::CPUI_INT_LESS, vec![shifted, Val::konst(20, 8)], 1);
    assert!(s.classify_one(&[(lt, true)], &res, &[]).is_none());
}

#[test]
fn a_zero_extended_byte_reads_back_as_itself() {
    let mut s = Syms::default();
    let b = s.atom(Atom::Leaf { space: 1, off: 0x88, size: 1, epoch: 0 }, 1);
    let d = s.eval(OpCode::CPUI_INT_ZEXT, vec![b.clone()], 4);
    let q = s.eval(OpCode::CPUI_INT_ZEXT, vec![d], 8);
    assert_eq!(s.subpiece(q, 0, 1), b);
}

#[test]
fn linear_values_cancel() {
    let mut s = Syms::default();
    let base = load(&mut s, 0x10);
    let v = base.plus(1).sub(&base);
    assert_eq!(v.as_const(), Some(1));
}
