use super::*;

fn leaves(n: usize) -> Vec<VarnodeId> {
    let mut map: slotmap::SlotMap<VarnodeId, ()> = slotmap::SlotMap::with_key();
    (0..n).map(|_| map.insert(())).collect()
}

fn leaf(id: VarnodeId, size: i32) -> Expr {
    Expr::Leaf(id, size)
}

fn op(code: OpCode, size: i32, args: Vec<Expr>) -> Expr {
    simplify(Expr::Op { code, size, args })
}

#[test]
fn comparison_forms_normalize() {
    let v = leaves(2);
    let n = leaf(v[0], 4);
    let zero = konst(4, 0);
    let one = konst(4, 1);
    let ne = cond_of(&op(OpCode::CPUI_INT_NOTEQUAL, 1, vec![n.clone(), zero.clone()]));
    let eqz = cond_of(&op(OpCode::CPUI_INT_EQUAL, 1, vec![zero.clone(), n.clone()]));
    assert_eq!(ne, eqz.clone().negate());
    let lt0 = cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![zero.clone(), n.clone()]));
    assert_eq!(lt0, ne);
    let lt1 = cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![n.clone(), one.clone()]));
    assert_eq!(lt1, eqz);
    let gt = cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![zero.clone(), n.clone()]));
    let le = cond_of(&op(OpCode::CPUI_INT_SLESSEQUAL, 1, vec![n.clone(), zero.clone()]));
    assert_eq!(gt, le.negate());
    let bare = cond_of(&n);
    assert_eq!(bare, ne);
}

#[test]
fn signed_and_unsigned_tests_stay_apart() {
    let v = leaves(1);
    let n = leaf(v[0], 4);
    let zero = konst(4, 0);
    let s = cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![zero.clone(), n.clone()]));
    let ne = cond_of(&op(OpCode::CPUI_INT_NOTEQUAL, 1, vec![zero, n]));
    assert_ne!(s, ne);
}

#[test]
fn pointer_end_test_reduces_to_count() {
    let v = leaves(2);
    let base = leaf(v[0], 8);
    let n = leaf(v[1], 8);
    let end = op(OpCode::CPUI_PTRADD, 8, vec![base.clone(), n.clone(), konst(8, 8)]);
    let entry = cond_of(&op(OpCode::CPUI_INT_NOTEQUAL, 1, vec![base.clone(), end]));
    let guard = cond_of(&op(OpCode::CPUI_INT_EQUAL, 1, vec![n.clone(), konst(8, 0)])).negate();
    assert_eq!(entry, guard);
    let plain = op(OpCode::CPUI_INT_ADD, 8, vec![base.clone(), op(OpCode::CPUI_INT_MULT, 8, vec![n.clone(), konst(8, 8)])]);
    let wraps = cond_of(&op(OpCode::CPUI_INT_NOTEQUAL, 1, vec![base.clone(), plain]));
    assert_ne!(wraps, guard);
}

#[test]
fn widened_count_cannot_wrap() {
    let v = leaves(2);
    let base = leaf(v[0], 8);
    let n = leaf(v[1], 4);
    let scaled = op(OpCode::CPUI_INT_MULT, 8, vec![op(OpCode::CPUI_INT_ZEXT, 8, vec![n.clone()]), konst(8, 0x10)]);
    let end = op(OpCode::CPUI_INT_ADD, 8, vec![base.clone(), scaled]);
    let entry = cond_of(&op(OpCode::CPUI_INT_NOTEQUAL, 1, vec![end, base]));
    let guard = cond_of(&op(OpCode::CPUI_INT_NOTEQUAL, 1, vec![n, konst(4, 0)]));
    assert_eq!(entry, guard);
}

#[test]
fn constants_fold_after_substitution() {
    let v = leaves(1);
    let n = leaf(v[0], 4);
    let sub = op(OpCode::CPUI_SUBPIECE, 4, vec![konst(8, 0), konst(4, 0)]);
    let entry = cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![sub, n.clone()]));
    let guard = cond_of(&op(OpCode::CPUI_INT_SLESSEQUAL, 1, vec![n, konst(4, 0)])).negate();
    assert_eq!(entry, guard);
    let always = cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![konst(4, 0), konst(4, 16)]));
    assert_eq!(always, Cond::of_bool(true));
    let negative = cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![konst(4, 0xffff_ffff), konst(4, 0)]));
    assert_eq!(negative, Cond::of_bool(true));
}

#[test]
fn loads_compare_by_time() {
    let v = leaves(1);
    let addr = leaf(v[0], 8);
    let ops: Vec<OpId> = {
        let mut map: slotmap::SlotMap<OpId, ()> = slotmap::SlotMap::with_key();
        (0..2).map(|_| map.insert(())).collect()
    };
    let at_header = |at| Expr::Load { addr: Box::new(addr.clone()), size: 4, at };
    assert_eq!(at_header(None), at_header(None));
    assert_ne!(at_header(Some(ops[0])), at_header(None));
    assert_ne!(at_header(Some(ops[0])), at_header(Some(ops[1])));
}

#[test]
fn parse_values() {
    assert_eq!(parse_loop_test("proven").unwrap().0, LoopTestMode::Proven);
    assert_eq!(parse_loop_test("binary").unwrap().0, LoopTestMode::Binary);
    assert_eq!(parse_loop_test("assume").unwrap().0, LoopTestMode::Assume);
    assert!(parse_loop_test("on").is_err());
}

#[test]
fn pointer_order_follows_the_index() {
    let v = leaves(2);
    let base = leaf(v[0], 8);
    let n = leaf(v[1], 4);
    let end = op(OpCode::CPUI_PTRADD, 8, vec![base.clone(), op(OpCode::CPUI_INT_SEXT, 8, vec![n.clone()]), konst(8, 1)]);
    let below = cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![base.clone(), end.clone()]));
    let positive = cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![konst(4, 0), n.clone()]));
    assert_eq!(below, positive);
    let above = cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![end, base]));
    let negative = cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![n, konst(4, 0)]));
    assert_eq!(above, negative);
}

#[test]
fn extensions_keep_the_order_against_a_constant() {
    let v = leaves(1);
    let n = leaf(v[0], 4);
    let wide = op(OpCode::CPUI_INT_SEXT, 8, vec![n.clone()]);
    assert_eq!(
        cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![konst(8, 0), wide])),
        cond_of(&op(OpCode::CPUI_INT_SLESS, 1, vec![konst(4, 0), n.clone()]))
    );
    let zwide = op(OpCode::CPUI_INT_ZEXT, 8, vec![n.clone()]);
    assert_eq!(
        cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![zwide.clone(), konst(8, 5)])),
        cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![n.clone(), konst(4, 5)]))
    );
    assert_ne!(
        cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![zwide, konst(8, 0x1_0000_0000)])),
        cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![n, konst(4, 0)]))
    );
}

#[test]
fn bound_is_a_multiple_of_the_step() {
    let v = leaves(2);
    let base = leaf(v[0], 8);
    let n = leaf(v[1], 8);
    let ptr = op(OpCode::CPUI_PTRADD, 8, vec![base.clone(), n.clone(), konst(8, 12)]);
    assert!(multiple_of(&ptr, &base, 12));
    assert!(!multiple_of(&ptr, &base, 24));
    let scaled = op(OpCode::CPUI_INT_ADD, 8, vec![base.clone(), op(OpCode::CPUI_INT_MULT, 8, vec![n.clone(), konst(8, 16)])]);
    assert!(multiple_of(&scaled, &base, 8));
    let odd = op(OpCode::CPUI_INT_ADD, 8, vec![base.clone(), op(OpCode::CPUI_INT_MULT, 8, vec![n.clone(), konst(8, 12)])]);
    assert!(!multiple_of(&odd, &base, 12));
    let wrapped = op(OpCode::CPUI_INT_ADD, 8, vec![base.clone(), op(OpCode::CPUI_INT_ADD, 8, vec![n, konst(8, 1)])]);
    assert!(!multiple_of(&wrapped, &base, 2));
}

#[test]
fn element_pointers_compare_as_indices() {
    let v = leaves(1);
    let base = leaf(v[0], 8);
    let at = |i: u64| op(OpCode::CPUI_PTRADD, 8, vec![base.clone(), konst(8, i), konst(8, 4)]);
    let entry = cond_of(&op(OpCode::CPUI_INT_NOTEQUAL, 1, vec![at(16), at(32)]));
    assert_eq!(entry, Cond::of_bool(true));
    assert_eq!(cond_of(&op(OpCode::CPUI_INT_LESS, 1, vec![at(16), at(32)])), Cond::of_bool(true));
    assert!(multiple_of(&at(32), &at(16), 4));
    assert!(!multiple_of(&at(32), &at(16), 8));
}
