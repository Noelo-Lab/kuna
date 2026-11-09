use super::*;

fn reg(a: &mut ExprArena, off: u64, size: u8) -> ExprId {
    a.var(VarInfo { kind: VarKind::InitReg { offset: off }, size, boolean: false })
}

#[test]
fn constants_fold_and_intern() {
    let mut a = ExprArena::new();
    let x = a.cst(8, 5);
    let y = a.cst(8, 7);
    let s = a.mk_bin(CPUI_INT_ADD, x, y, 8);
    assert_eq!(a.is_const(s), Some(12));
    let s2 = a.mk_bin(CPUI_INT_ADD, y, x, 8);
    assert_eq!(s, s2);
    let m = a.mk_bin(CPUI_INT_MULT, x, y, 1);
    assert_eq!(a.is_const(m), Some(35));
    let neg = a.mk_un(CPUI_INT_2COMP, x, 8);
    assert_eq!(a.is_const(neg), Some(u64::MAX as u128 - 4));
}

#[test]
fn linear_normal_form_cancels() {
    let mut a = ExprArena::new();
    let x = reg(&mut a, 0, 8);
    let y = reg(&mut a, 8, 8);
    // (x + 3) - (x + 3) == 0
    let three = a.cst(8, 3);
    let p = a.mk_bin(CPUI_INT_ADD, x, three, 8);
    let d = a.mk_bin(CPUI_INT_SUB, p, p, 8);
    assert_eq!(a.is_const(d), Some(0));
    // ~(~x + ~y) == x + y + 1
    let nx = a.mk_un(CPUI_INT_NEGATE, x, 8);
    let ny = a.mk_un(CPUI_INT_NEGATE, y, 8);
    let s = a.mk_bin(CPUI_INT_ADD, nx, ny, 8);
    let r = a.mk_un(CPUI_INT_NEGATE, s, 8);
    let one = a.cst(8, 1);
    let xy = a.mk_bin(CPUI_INT_ADD, x, y, 8);
    let expect = a.mk_bin(CPUI_INT_ADD, xy, one, 8);
    assert_eq!(r, expect, "{} vs {}", a.fmt(r), a.fmt(expect));
    // x + y - y == x, and commutativity
    let t = a.mk_bin(CPUI_INT_SUB, xy, y, 8);
    assert_eq!(t, x);
    let yx = a.mk_bin(CPUI_INT_ADD, y, x, 8);
    assert_eq!(yx, xy);
    // -(-x) == x ; ~~x == x
    let n2 = a.mk_un(CPUI_INT_2COMP, x, 8);
    let n3 = a.mk_un(CPUI_INT_2COMP, n2, 8);
    assert_eq!(n3, x);
    let nn = a.mk_un(CPUI_INT_NEGATE, nx, 8);
    assert_eq!(nn, x);
    // ~(~x + 0x18) == x - 0x18  (VMProtect stack arithmetic)
    let k = a.cst(8, 0x18);
    let inner = a.mk_bin(CPUI_INT_ADD, nx, k, 8);
    let outer = a.mk_un(CPUI_INT_NEGATE, inner, 8);
    let expect2 = a.mk_bin(CPUI_INT_SUB, x, k, 8);
    assert_eq!(outer, expect2);
    assert_eq!(a.affine(outer), Some((a.as_var(x), -0x18)));
}

#[test]
fn bitwise_identities_and_known_bits() {
    let mut a = ExprArena::new();
    let x = reg(&mut a, 0, 8);
    let xx = a.mk_bin(CPUI_INT_XOR, x, x, 8);
    assert_eq!(a.is_const(xx), Some(0));
    let ax = a.mk_bin(CPUI_INT_AND, x, x, 8);
    assert_eq!(ax, x);
    let nx = a.mk_un(CPUI_INT_NEGATE, x, 8);
    let z = a.mk_bin(CPUI_INT_AND, x, nx, 8);
    assert_eq!(a.is_const(z), Some(0));
    let o = a.mk_bin(CPUI_INT_OR, x, nx, 8);
    assert_eq!(a.is_const(o), Some(u64::MAX as u128));
    // zext(byte) & 0xff == zext(byte); (zext(b) << 8) & 0xff == 0
    let b = reg(&mut a, 0x10, 1);
    let zb = a.mk_un(CPUI_INT_ZEXT, b, 8);
    let ff = a.cst(8, 0xff);
    assert_eq!(a.mk_bin(CPUI_INT_AND, zb, ff, 8), zb);
    let eight = a.cst(4, 8);
    let sh = a.mk_bin(CPUI_INT_LEFT, zb, eight, 8);
    let masked = a.mk_bin(CPUI_INT_AND, sh, ff, 8);
    assert_eq!(a.is_const(masked), Some(0));
    // (x << 4) >> 4 == x & 0x0fff..
    let four = a.cst(4, 4);
    let l = a.mk_bin(CPUI_INT_LEFT, x, four, 8);
    let r = a.mk_bin(CPUI_INT_RIGHT, l, four, 8);
    let m = a.cst(8, u64::MAX as u128 >> 4);
    assert_eq!(r, a.mk_bin(CPUI_INT_AND, x, m, 8));
}

#[test]
fn flag_word_bit_extraction() {
    // flags = zext(CF) | zext(PF)<<2 | zext(ZF)<<6 ; (flags >> 6) & 1 == zext(ZF)
    let mut a = ExprArena::new();
    let cf = a.var(VarInfo { kind: VarKind::InitReg { offset: 0x200 }, size: 1, boolean: true });
    let pf = a.var(VarInfo { kind: VarKind::InitReg { offset: 0x202 }, size: 1, boolean: true });
    let zf = a.var(VarInfo { kind: VarKind::InitReg { offset: 0x206 }, size: 1, boolean: true });
    let zcf = a.mk_un(CPUI_INT_ZEXT, cf, 8);
    let zpf = a.mk_un(CPUI_INT_ZEXT, pf, 8);
    let zzf = a.mk_un(CPUI_INT_ZEXT, zf, 8);
    let two = a.cst(4, 2);
    let six = a.cst(4, 6);
    let spf = a.mk_bin(CPUI_INT_LEFT, zpf, two, 8);
    let szf = a.mk_bin(CPUI_INT_LEFT, zzf, six, 8);
    let f1 = a.mk_bin(CPUI_INT_OR, zcf, spf, 8);
    let flags = a.mk_bin(CPUI_INT_OR, f1, szf, 8);
    let sh = a.mk_bin(CPUI_INT_RIGHT, flags, six, 8);
    let one = a.cst(8, 1);
    let bit = a.mk_bin(CPUI_INT_AND, sh, one, 8);
    assert_eq!(bit, zzf, "{}", a.fmt(bit));
    let b1 = a.mk_subpiece(bit, 0, 1);
    assert_eq!(b1, zf);
    // the same through a two-byte flags word pushed and reloaded as pieces
    let lo = a.mk_subpiece(flags, 0, 1);
    let hi = a.mk_subpiece(flags, 1, 7);
    let re = a.mk_piece(hi, lo);
    assert_eq!(re, flags);
}

#[test]
fn comparisons_normalize() {
    let mut a = ExprArena::new();
    let x = reg(&mut a, 0, 8);
    let five = a.cst(8, 5);
    let zero = a.cst(8, 0);
    let d = a.mk_bin(CPUI_INT_SUB, x, five, 8);
    let e1 = a.mk_bin(CPUI_INT_EQUAL, d, zero, 1);
    let e2 = a.mk_bin(CPUI_INT_EQUAL, x, five, 1);
    assert_eq!(e1, e2);
    let ne = a.mk_bin(CPUI_INT_NOTEQUAL, x, five, 1);
    let nne = a.mk_un(CPUI_BOOL_NEGATE, ne, 1);
    assert_eq!(nne, e2);
    assert!(a.is_bool(e2));
    // zext(bool) == 1  ->  bool
    let z = a.mk_un(CPUI_INT_ZEXT, e2, 8);
    let one = a.cst(8, 1);
    assert_eq!(a.mk_bin(CPUI_INT_EQUAL, z, one, 1), e2);
    // x < 0 is false; x == x is true
    let lt0 = a.mk_bin(CPUI_INT_LESS, x, zero, 1);
    assert_eq!(a.is_const(lt0), Some(0));
    let eqxx = a.mk_bin(CPUI_INT_EQUAL, x, x, 1);
    assert_eq!(a.is_const(eqxx), Some(1));
}

#[test]
fn ite_case_split_enumerates_targets() {
    let mut a = ExprArena::new();
    let x = reg(&mut a, 0, 8);
    let five = a.cst(8, 5);
    let c = a.mk_bin(CPUI_INT_EQUAL, x, five, 1);
    let zc = a.mk_un(CPUI_INT_ZEXT, c, 8);
    // target = 0x1000 + zext(c) * 0x40   (select between two handlers)
    let k = a.cst(8, 0x40);
    let scaled = a.mk_bin(CPUI_INT_MULT, zc, k, 8);
    let base = a.cst(8, 0x1000);
    let t = a.mk_bin(CPUI_INT_ADD, base, scaled, 8);
    let vals = a.possible_values(t).unwrap();
    assert_eq!(vals, vec![0x1000, 0x1040]);
    // the listing-1 shape: ~(~(0xfffffffff + zext(c)) | K1) + ~((0xfffffffff + zext(c)) | K2)
    let big = a.cst(8, 0xfffffffff);
    let inner = a.mk_bin(CPUI_INT_ADD, big, zc, 8);
    let k1 = a.cst(8, (-5369364481i64) as u64 as u128);
    let k2 = a.cst(8, (-5369367216i64) as u64 as u128);
    let ninner = a.mk_un(CPUI_INT_NEGATE, inner, 8);
    let o1 = a.mk_bin(CPUI_INT_OR, ninner, k1, 8);
    let p1 = a.mk_un(CPUI_INT_NEGATE, o1, 8);
    let o2 = a.mk_bin(CPUI_INT_OR, inner, k2, 8);
    let p2 = a.mk_un(CPUI_INT_NEGATE, o2, 8);
    let tgt = a.mk_bin(CPUI_INT_ADD, p1, p2, 8);
    let vals = a.possible_values(tgt).expect("enumerable");
    assert_eq!(vals.len(), 2, "{:?}", vals);
    // a guard that is a comparison stays two-valued; one over constants is fixed
    assert_eq!(a.possible_values(c).unwrap(), vec![0, 1]);
    let ite = a.mk_ite(c, base, k);
    let w = a.mk_bin(CPUI_INT_ADD, ite, five, 8);
    assert_eq!(a.possible_values(w).unwrap(), vec![0x45, 0x1005]);
}

#[test]
fn subpiece_distributes_over_low_arithmetic() {
    let mut a = ExprArena::new();
    let x = reg(&mut a, 0, 8);
    let y = reg(&mut a, 8, 8);
    let s = a.mk_bin(CPUI_INT_ADD, x, y, 8);
    let low = a.mk_subpiece(s, 0, 4);
    let lx = a.mk_subpiece(x, 0, 4);
    let ly = a.mk_subpiece(y, 0, 4);
    assert_eq!(low, a.mk_bin(CPUI_INT_ADD, lx, ly, 4));
    // zext then truncate round-trips
    let z = a.mk_un(CPUI_INT_ZEXT, lx, 8);
    assert_eq!(a.mk_subpiece(z, 0, 4), lx);
    assert_eq!(a.mk_subpiece(z, 4, 4), a.cst(4, 0));
    // piece of the two halves of x is x
    let hi = a.mk_subpiece(x, 4, 4);
    assert_eq!(a.mk_piece(hi, lx), x);
}
