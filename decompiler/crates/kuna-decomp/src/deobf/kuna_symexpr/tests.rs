use super::*;

fn reg(a: &mut ExprArena, off: u64, size: u8) -> ExprId {
    a.var(VarInfo::new(VarKind::InitReg { offset: off }, size, false))
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
    let cf = a.var(VarInfo::new(VarKind::InitReg { offset: 0x200 }, 1, true));
    let pf = a.var(VarInfo::new(VarKind::InitReg { offset: 0x202 }, 1, true));
    let zf = a.var(VarInfo::new(VarKind::InitReg { offset: 0x206 }, 1, true));
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

#[test]
fn direct_entry_points_fold_two_constants() {
    let mut a = ExprArena::new();
    let x = a.cst(1, 0xf0);
    let y = a.cst(1, 0x0f);
    // mk_bin dispatches to the direct builders; every pair must fold, not loop.
    for opc in [CPUI_INT_XOR, CPUI_INT_AND, CPUI_INT_OR, CPUI_INT_LEFT, CPUI_INT_RIGHT, CPUI_INT_EQUAL, CPUI_INT_LESS, CPUI_INT_SLESS] {
        let r = a.mk_bin(opc, x, y, if matches!(opc, CPUI_INT_EQUAL | CPUI_INT_LESS | CPUI_INT_SLESS) { 1 } else { 1 });
        assert!(a.is_const(r).is_some(), "{opc:?}");
    }
    // the route that hung: and-with-constant folding its operand to a constant, then xor
    let v = reg(&mut a, 0, 1);
    let k = a.cst(1, 0x0f);
    let t = a.mk_bin(CPUI_INT_XOR, v, k, 1); // v ^ 0x0f
    let m = a.cst(1, 0xf0);
    let r = a.mk_bin(CPUI_INT_AND, t, m, 1); // (v ^ 0x0f) & 0xf0 == v & 0xf0
    let expect = a.mk_bin(CPUI_INT_AND, v, m, 1);
    assert_eq!(r, expect);
}

#[test]
fn known_bits_fold_and_alignment_is_affine() {
    let mut a = ExprArena::new();
    // the entry stack pointer: low 16 bits known 0xfff8
    let sp = a.var_of_known(VarKind::InitReg { offset: 0x20 }, 8, false, (0xFFFF & !0xFFF8, 0xFFF8));
    let sp_var = a.as_var(sp);
    let c = a.cst(8, (-0x250i64) as u64 as u128);
    let x = a.mk_bin(CPUI_INT_ADD, sp, c, 8);
    let m = a.cst(8, (-0x10i64) as u64 as u128);
    let aligned = a.mk_bin(CPUI_INT_AND, x, m, 8);
    assert_eq!(a.affine(aligned), Some((sp_var, -0x258)), "{}", a.fmt(aligned));
    // a fully determined slice is a constant
    let low = a.mk_subpiece(x, 0, 2);
    assert_eq!(a.is_const(low), Some(0xFDA8));
    let zero = a.cst(2, 0);
    let eq = a.mk_bin(CPUI_INT_EQUAL, low, zero, 1);
    assert_eq!(a.is_const(eq), Some(0));
}

#[test]
fn stack_pointer_never_equals_a_small_constant() {
    let mut a = ExprArena::new();
    let sp = a.var_of_known(VarKind::InitReg { offset: 0x20 }, 8, false, (0xFFFF & !0xFFF8, 0xFFF8));
    a.mark_pointer(sp);
    let c = a.cst(8, (-0x218i64) as u64 as u128);
    let x = a.mk_bin(CPUI_INT_ADD, sp, c, 8);
    let zero = a.cst(8, 0);
    let eq = a.mk_bin(CPUI_INT_EQUAL, x, zero, 1);
    assert_eq!(a.is_const(eq), Some(0));
    let ne = a.mk_bin(CPUI_INT_NOTEQUAL, x, zero, 1);
    assert_eq!(a.is_const(ne), Some(1));
}

#[test]
fn pointer_relative_compares_fold() {
    let mut a = ExprArena::new();
    let sp = a.var_of_known(VarKind::InitReg { offset: 0x20 }, 8, false, (0xFFFF & !0xFFF8, 0xFFF8));
    a.mark_pointer(sp);
    let c1 = a.cst(8, (-0x40i64) as u64 as u128);
    let c2 = a.cst(8, (-0x18i64) as u64 as u128);
    let x = a.mk_bin(CPUI_INT_ADD, sp, c1, 8);
    let y = a.mk_bin(CPUI_INT_ADD, sp, c2, 8);
    let lt = a.mk_bin(CPUI_INT_LESS, x, y, 1);
    assert_eq!(a.is_const(lt), Some(1));
    let gt = a.mk_bin(CPUI_INT_LESS, y, x, 1);
    assert_eq!(a.is_const(gt), Some(0));
    let le = a.mk_bin(CPUI_INT_LESSEQUAL, y, x, 1);
    assert_eq!(a.is_const(le), Some(0));
}

#[test]
fn sleigh_pushfq_word_keeps_a_symbolic_zero_flag() {
    // The exact shape SLEIGH emits for PUSHFQ: (flag & 1) zext'ed, scaled by a
    // power of two, OR-ed together; the zero flag is a comparison of unknown data.
    let mut a = ExprArena::new();
    let v = a.var(VarInfo::new(VarKind::Unknown { tag: 1 }, 4, false));
    let five = a.cst(4, 5);
    let zf = a.mk_bin(CPUI_INT_EQUAL, v, five, 1);
    let mut word = a.cst(8, 0);
    let one1 = a.cst(1, 1);
    for (off, bit) in [(0x200u64, 0u32), (0x202, 2), (0x204, 4), (0x206, 6), (0x207, 7), (0x208, 8), (0x209, 9), (0x20a, 10), (0x20b, 11)] {
        let flag = if off == 0x206 { zf } else { a.var_of(VarKind::InitReg { offset: off }, 1, true) };
        let masked = a.mk_bin(CPUI_INT_AND, flag, one1, 1);
        let z = a.mk_un(CPUI_INT_ZEXT, masked, 8);
        let k = a.cst(8, 1u128 << bit);
        let scaled = a.mk_bin(CPUI_INT_MULT, k, z, 8);
        word = a.mk_bin(CPUI_INT_OR, word, scaled, 8);
    }
    // (word >> 6) & 1 == zext(zf)
    let six = a.cst(8, 6);
    let sh = a.mk_bin(CPUI_INT_RIGHT, word, six, 8);
    let one8 = a.cst(8, 1);
    let bit = a.mk_bin(CPUI_INT_AND, sh, one8, 8);
    assert_eq!(bit, a.mk_un(CPUI_INT_ZEXT, zf, 8), "{}", a.fmt(bit));
    // (word & 0x40) == 0  ==  !zf
    let m40 = a.cst(8, 0x40);
    let masked = a.mk_bin(CPUI_INT_AND, word, m40, 8);
    let zero8 = a.cst(8, 0);
    let test = a.mk_bin(CPUI_INT_EQUAL, masked, zero8, 1);
    let nzf = a.mk_un(CPUI_BOOL_NEGATE, zf, 1);
    assert_eq!(test, nzf, "{}", a.fmt(test));
    // (word << 57) >> 63 == zext(zf)
    let k57 = a.cst(8, 57);
    let k63 = a.cst(8, 63);
    let l = a.mk_bin(CPUI_INT_LEFT, word, k57, 8);
    let r = a.mk_bin(CPUI_INT_RIGHT, l, k63, 8);
    assert_eq!(r, a.mk_un(CPUI_INT_ZEXT, zf, 8), "{}", a.fmt(r));
    // NOR-style subtraction keeps the comparison symbolic: ~(~v + 5) == 0  <=>  v == 5
    let nv = a.mk_un(CPUI_INT_NEGATE, v, 4);
    let s = a.mk_bin(CPUI_INT_ADD, nv, five, 4);
    let ns = a.mk_un(CPUI_INT_NEGATE, s, 4);
    let zero4 = a.cst(4, 0);
    let eq = a.mk_bin(CPUI_INT_EQUAL, ns, zero4, 1);
    assert_eq!(eq, zf, "{}", a.fmt(eq));
    assert_eq!(a.possible_values(zf), Some(vec![0, 1]));
}

#[test]
fn de_morgan_over_a_flags_word_prunes_to_one_flag() {
    // ~(~flags | 0x815) & 0x40  == flags & 0x40 (VMProtect's NOR-style masking)
    let mut a = ExprArena::new();
    let v = a.var(VarInfo::new(VarKind::Unknown { tag: 1 }, 4, false));
    let five = a.cst(4, 5);
    let zf = a.mk_bin(CPUI_INT_EQUAL, v, five, 1);
    let mut word = a.cst(8, 0);
    for (off, bit) in [(0x200u64, 0u32), (0x202, 2), (0x204, 4), (0x206, 6), (0x207, 7), (0x20b, 11)] {
        let flag = if off == 0x206 { zf } else { a.var_of(VarKind::InitReg { offset: off }, 1, true) };
        let z = a.mk_un(CPUI_INT_ZEXT, flag, 8);
        let k = a.cst(8, 1u128 << bit);
        let scaled = a.mk_bin(CPUI_INT_MULT, k, z, 8);
        word = a.mk_bin(CPUI_INT_OR, word, scaled, 8);
    }
    let nw = a.mk_un(CPUI_INT_NEGATE, word, 8);
    let k815 = a.cst(8, 0x815);
    let or = a.mk_bin(CPUI_INT_OR, nw, k815, 8);
    let nor = a.mk_un(CPUI_INT_NEGATE, or, 8);
    let m40 = a.cst(8, 0x40);
    let bit = a.mk_bin(CPUI_INT_AND, nor, m40, 8);
    let zzf = a.mk_un(CPUI_INT_ZEXT, zf, 8);
    let expect = a.mk_bin(CPUI_INT_MULT, zzf, m40, 8);
    assert_eq!(bit, expect, "{}", a.fmt(bit));
    assert_eq!(a.bool_atoms(bit), vec![zf]);
    // the xor-with-constant form reduces the same way
    let kff = a.cst(8, 0x8d5);
    let x = a.mk_bin(CPUI_INT_XOR, word, kff, 8);
    let bit2 = a.mk_bin(CPUI_INT_AND, x, m40, 8);
    let nzf = a.mk_un(CPUI_BOOL_NEGATE, zf, 1);
    let znzf = a.mk_un(CPUI_INT_ZEXT, nzf, 8);
    let expect2 = a.mk_bin(CPUI_INT_MULT, znzf, m40, 8);
    assert_eq!(bit2, expect2, "{}", a.fmt(bit2));
}

#[test]
fn negated_flag_terms_do_not_hide_mask_pruning() {
    // word = 0x280 - 0x80 * !SF + other bits; (word & 0x4000) == 0 is decided.
    let mut a = ExprArena::new();
    let sf = a.var_of(VarKind::InitReg { offset: 0x207 }, 1, true);
    let cf = a.var_of(VarKind::InitReg { offset: 0x200 }, 1, true);
    let one = a.cst(1, 1);
    let nsf = a.mk_bin(CPUI_INT_XOR, sf, one, 1);
    let znsf = a.mk_un(CPUI_INT_ZEXT, nsf, 8);
    let neg80 = a.cst(8, (-0x80i64) as u64 as u128);
    let t1 = a.mk_bin(CPUI_INT_MULT, znsf, neg80, 8);
    let zcf = a.mk_un(CPUI_INT_ZEXT, cf, 8);
    let s = a.mk_bin(CPUI_INT_ADD, t1, zcf, 8);
    let k = a.cst(8, 0x280);
    let word = a.mk_bin(CPUI_INT_ADD, s, k, 8);
    let m = a.cst(8, 0x4000);
    let masked = a.mk_bin(CPUI_INT_AND, word, m, 8);
    assert_eq!(a.is_const(masked), Some(0), "{}", a.fmt(masked));
    let m80 = a.cst(8, 0x80);
    let sfbit = a.mk_bin(CPUI_INT_AND, word, m80, 8);
    let zsf = a.mk_un(CPUI_INT_ZEXT, sf, 8);
    assert_eq!(sfbit, a.mk_bin(CPUI_INT_MULT, zsf, m80, 8), "{}", a.fmt(sfbit));
}

#[test]
fn known_bits_decide_a_flag_word_compare() {
    use kuna_num::opcodes::OpCode::*;
    let mut a = ExprArena::new();
    let af = a.var(VarInfo::new(VarKind::InitReg { offset: 0x204 }, 1, true));
    let z = a.mk_un(CPUI_INT_ZEXT, af, 2);
    let k10 = a.cst(2, 0x10);
    let m = a.mk_bin(CPUI_INT_MULT, z, k10, 2);
    let one = a.cst(2, 1);
    let x = a.mk_bin(CPUI_INT_OR, m, one, 2);
    let x8 = a.mk_un(CPUI_INT_ZEXT, x, 8);
    let nx = a.mk_un(CPUI_INT_NEGATE, x8, 8);
    let k815 = a.cst(8, 0x815);
    let andv = a.mk_bin(CPUI_INT_AND, nx, k815, 8);
    let k280 = a.cst(8, 0x280);
    let v = a.mk_bin(CPUI_INT_ADD, andv, k280, 8);
    let nv = a.mk_un(CPUI_INT_NEGATE, v, 8);
    let kmask = a.cst(8, 0xffff_ffff_ffff_ff7f);
    let o = a.mk_bin(CPUI_INT_OR, nv, kmask, 8);
    let all = a.cst(8, u64::MAX as u128);
    let eq = a.mk_bin(CPUI_INT_EQUAL, o, all, 1);
    assert_eq!(a.is_const(eq), Some(0), "{}", a.fmt(eq));
}

#[test]
fn sign_of_not_plus_const() {
    use kuna_num::opcodes::OpCode::*;
    let mut a = ExprArena::new();
    let s = a.var(VarInfo::new(VarKind::InitReg { offset: 0x10 }, 4, false));
    let n = a.mk_un(CPUI_INT_NEGATE, s, 4);
    let k = a.cst(4, 5);
    let sum = a.mk_bin(CPUI_INT_ADD, n, k, 4);
    let z = a.cst(4, 0);
    let sf = a.mk_bin(CPUI_INT_SLESS, sum, z, 1);
    let of = a.mk_bin(CPUI_INT_SCARRY, k, n, 1);
    eprintln!("sum={} sf={} of={}", a.fmt(sum), a.fmt(sf), a.fmt(of));
}

/// A merged loop counter seen only as 0 and 1 carries bits 1..31 as known
/// zero: three probe points masked by them coincide, so a select between two
/// bytecode addresses over it must not pass for `counter + d`.
#[test]
fn a_narrow_merged_location_never_probes_affine() {
    let mut a = ExprArena::new();
    let k = a.var_of_known(VarKind::Merged { node: 1, tag: 2 }, 4, false, (0xffff_fffe, 0));
    let b = a.mk_subpiece(k, 0, 1);
    let (t, f) = (a.cst(4, 0x4744de), a.cst(4, 0x47444c));
    let vpc = a.mk_ite(b, t, f);
    assert_eq!(a.affine_probe(vpc), None, "{}", a.fmt(vpc));
    let one = a.cst(4, 1);
    let next = a.mk_bin(CPUI_INT_ADD, k, one, 4);
    assert_eq!(a.affine_probe(next), Some((a.as_var(k), 1)));
    let sq = a.mk_bin(CPUI_INT_MULT, k, k, 4);
    let zero = a.mk_bin(CPUI_INT_SUB, sq, k, 4);
    assert_eq!(a.affine_probe(zero), Some((None, 0)), "{}", a.fmt(zero));
}

#[test]
fn case_split_reaches_atoms_nested_in_a_comparison() {
    // `setl dl; push rdx; ...; test rax, rax; cmovz rsi, rcx`: the select's
    // guard is a comparison of the overflow and sign atoms, an atom whose own
    // operands hold the atoms that decide it.
    let mut a = ExprArena::new();
    let s = a.var_of_known(VarKind::Merged { node: 1, tag: 1 }, 8, false, (mask(8) & !1, 0));
    let ten = a.cst(8, 10);
    let zero = a.cst(8, 0);
    let of = a.mk_bin(CPUI_INT_SBORROW, s, ten, 1);
    let diff = a.mk_bin(CPUI_INT_SUB, s, ten, 8);
    let sf = a.mk_bin(CPUI_INT_SLESS, diff, zero, 1);
    let lt = a.mk_bin(CPUI_INT_NOTEQUAL, of, sf, 1);
    let wide = a.mk_un(CPUI_INT_ZEXT, lt, 8);
    let zf = a.mk_bin(CPUI_INT_EQUAL, wide, zero, 1);
    let (exit, body) = (a.cst(8, 0x40296b), a.cst(8, 0x402958));
    let vpc = a.mk_ite(zf, exit, body);
    assert_eq!(a.possible_values(vpc), Some(vec![0x402958, 0x40296b]), "{}", a.fmt(vpc));
}

#[test]
fn only_round_stable_variables_name_a_location() {
    let mut a = ExprArena::new();
    let sp = a.var_of(VarKind::InitReg { offset: 0x20 }, 8, false);
    let sp_var = a.as_var(sp).unwrap();
    let call = a.var(VarInfo::new(VarKind::CallResult { site: 0x1000 }, 8, false));
    let call_var = a.as_var(call).unwrap();
    let behind_sp = a.var_of(VarKind::InitMem { base: Some(sp_var), offset: 8 }, 8, false);
    let behind_call = a.var_of(VarKind::InitMem { base: Some(call_var), offset: 8 }, 8, false);
    assert!(a.is_stable_var(sp_var));
    assert!(a.is_stable_var(a.as_var(behind_sp).unwrap()));
    assert!(!a.is_stable_var(call_var));
    assert!(!a.is_stable_var(a.as_var(behind_call).unwrap()));
}

#[test]
fn a_stack_pointer_test_that_holds_only_high_up_is_no_constant() {
    let mut a = ExprArena::new();
    let sp = a.var_of_known(VarKind::InitReg { offset: 0x20 }, 8, false, (0x7, 0x8));
    a.mark_pointer(sp);
    let k32 = a.cst(1, 32);
    let hi = a.mk_bin(CPUI_INT_RIGHT, sp, k32, 8);
    let zero = a.cst(8, 0);
    let low = a.mk_bin(CPUI_INT_EQUAL, hi, zero, 1);
    let (x, y) = (a.cst(8, 0x401000), a.cst(8, 0x402000));
    let sel = a.mk_ite(low, x, y);
    assert_eq!(a.affine_probe(sel), None, "{}", a.fmt(sel));
    // the image base an XOR swap through the stack pointer leaves is one
    let base = a.cst(8, 0x1_4000_0000);
    let swapped = a.mk_bin(CPUI_INT_XOR, sp, base, 8);
    let back = a.mk_bin(CPUI_INT_XOR, swapped, sp, 8);
    assert_eq!(a.affine_probe(back), Some((None, 0x1_4000_0000)));
}

#[test]
fn constructors_remember_results_but_not_fresh_unknowns() {
    let mut a = ExprArena::new();
    let x = reg(&mut a, 0, 8);
    let y = reg(&mut a, 8, 8);
    let z = reg(&mut a, 0x10, 8);
    // Selects sharing their arms 40 levels deep: a mask distributes into each
    // level once, not once per path (2^40 of them).
    let w = reg(&mut a, 0x18, 8);
    let mut e = w;
    for i in 0..40u128 {
        let k = a.cst(8, i);
        let c = a.mk_bin(CPUI_INT_EQUAL, y, k, 1);
        let d = a.mk_bin(CPUI_INT_EQUAL, z, k, 1);
        let inner = a.mk_ite(d, e, x);
        e = a.mk_ite(c, e, inner);
    }
    let m = a.cst(8, 0xff);
    let masked = a.mk_bin(CPUI_INT_AND, e, m, 8);
    assert!(matches!(a.kind(masked), ExprKind::Ite(..)));
    assert_eq!(a.possible_bits(masked) & !0xff, 0);
    assert_eq!(a.mk_bin(CPUI_INT_AND, e, m, 8), masked);
    // A division by zero is a fresh unknown each time.
    let one = a.cst(4, 1);
    let zero = a.cst(4, 0);
    let u1 = a.mk_bin(CPUI_INT_DIV, one, zero, 4);
    let u2 = a.mk_bin(CPUI_INT_DIV, one, zero, 4);
    assert_ne!(u1, u2);
}

/// Themida's stack arithmetic: `y = sp - k` rebuilt as `(y & M) | (N - (~y & N))`
/// with `M | N` all ones, mixed with a second register that cancels.
#[test]
fn identity_probe_sees_through_two_variable_mba() {
    let mut a = ExprArena::new();
    let sp = a.var_of_known(VarKind::InitReg { offset: 0x10 }, 4, false, (0x3, 0xfffc));
    a.mark_pointer(sp);
    let ecx = reg(&mut a, 0x4, 4);
    let k = a.cst(4, 0xfffffc6c);
    let y = a.mk_bin(CPUI_INT_ADD, sp, k, 4);
    let (m, n) = (a.cst(4, 0xb46baf2b), a.cst(4, 0x4b9450d4));
    let ym = a.mk_bin(CPUI_INT_AND, y, m, 4);
    let ny = a.mk_un(CPUI_INT_NEGATE, y, 4);
    let nyn = a.mk_bin(CPUI_INT_AND, ny, n, 4);
    let rest = a.mk_bin(CPUI_INT_SUB, n, nyn, 4);
    let back = a.mk_bin(CPUI_INT_OR, ym, rest, 4);
    // ecx | K minus ecx & ~K is K, whatever ecx holds
    let kk = a.cst(4, 0x00ff_00f0);
    let nk = a.cst(4, !0x00ff_00f0u32 as u128);
    let hi = a.mk_bin(CPUI_INT_OR, ecx, kk, 4);
    let lo = a.mk_bin(CPUI_INT_AND, ecx, nk, 4);
    let junk = a.mk_bin(CPUI_INT_SUB, hi, lo, 4);
    let both = a.mk_bin(CPUI_INT_ADD, back, junk, 4);
    let sp_var = a.as_var(sp);
    assert_eq!(a.affine(back), None, "{}", a.fmt(back));
    assert_eq!(a.identity_probe(back), Some((sp_var, -0x394)), "{}", a.fmt(back));
    assert_eq!(a.identity_probe(junk), Some((None, 0x00ff_00f0)), "{}", a.fmt(junk));
    assert_eq!(a.affine_probe(both), Some((sp_var, -0x394 + 0x00ff_00f0)), "{}", a.fmt(both));
    // x + y == (x ^ y) + 2 * (x & y): two variables, the second subtracted again
    let x = reg(&mut a, 0x0, 4);
    let xy = a.mk_bin(CPUI_INT_XOR, x, ecx, 4);
    let and = a.mk_bin(CPUI_INT_AND, x, ecx, 4);
    let two = a.cst(4, 2);
    let and2 = a.mk_bin(CPUI_INT_MULT, and, two, 4);
    let sum = a.mk_bin(CPUI_INT_ADD, xy, and2, 4);
    let only_x = a.mk_bin(CPUI_INT_SUB, sum, ecx, 4);
    assert_eq!(a.affine(only_x), None, "{}", a.fmt(only_x));
    let zero = a.mk_bin(CPUI_INT_SUB, only_x, x, 4);
    assert_eq!(a.identity_probe(zero), Some((None, 0)), "{}", a.fmt(zero));
    assert_eq!(a.identity_probe(only_x), Some((a.as_var(x), 0)), "{}", a.fmt(only_x));
    // but the sum itself is no single base plus a constant
    assert_eq!(a.identity_probe(sum), None, "{}", a.fmt(sum));
}

#[test]
fn identity_probe_rejects_near_identities() {
    let mut a = ExprArena::new();
    let x = reg(&mut a, 0x0, 4);
    // an alignment mask is no offset
    let al = a.cst(4, 0xffff_fff0);
    let aligned = a.mk_bin(CPUI_INT_AND, x, al, 4);
    assert_eq!(a.identity_probe(aligned), None, "{}", a.fmt(aligned));
    // x + 0x10 everywhere but where the low 31 bits carry into bit 31 (2^-28 of all x)
    let low = a.cst(4, 0x7fff_ffff);
    let top = a.cst(4, 0x8000_0000);
    let k = a.cst(4, 0x10);
    let xl = a.mk_bin(CPUI_INT_AND, x, low, 4);
    let xl = a.mk_bin(CPUI_INT_ADD, xl, k, 4);
    let xt = a.mk_bin(CPUI_INT_AND, x, top, 4);
    let carry31 = a.mk_bin(CPUI_INT_OR, xl, xt, 4);
    assert_eq!(a.identity_probe(carry31), None, "{}", a.fmt(carry31));
    // a 64-bit pointer advanced in its low half only (wrong once the low 32 bits carry)
    let rsp = a.var_of_known(VarKind::InitReg { offset: 0x20 }, 8, false, (0x7, 0xfff8));
    let lo = a.mk_subpiece(rsp, 0, 4);
    let hi = a.mk_subpiece(rsp, 4, 4);
    let k8 = a.cst(4, 0x18);
    let lo2 = a.mk_bin(CPUI_INT_ADD, lo, k8, 4);
    let halves = a.mk_piece(hi, lo2);
    assert_eq!(a.identity_probe(halves), None, "{}", a.fmt(halves));
    // the same advance within the known low bits never carries: an identity
    let k4 = a.cst(4, 0x4);
    let lo3 = a.mk_bin(CPUI_INT_ADD, lo, k4, 4);
    let halves2 = a.mk_piece(hi, lo3);
    assert_eq!(a.affine(halves2), None, "{}", a.fmt(halves2));
    assert_eq!(a.identity_probe(halves2), Some((a.as_var(rsp), 4)), "{}", a.fmt(halves2));
    // a select on program data is never sampled: it may hold at one value only
    let five = a.cst(4, 5);
    let eq = a.mk_bin(CPUI_INT_EQUAL, x, five, 1);
    let x4 = a.mk_bin(CPUI_INT_ADD, x, k4, 4);
    let sel = a.mk_ite(eq, x4, x);
    assert_eq!(a.identity_probe(sel), None, "{}", a.fmt(sel));
}

#[test]
fn identity_probe_is_exhaustive_over_few_free_bits() {
    let mut a = ExprArena::new();
    // two 2-bit counters: c1 * c2 - c2 * c1 is 0, c1 & c2 is not constant
    let c1 = a.var_of_known(VarKind::Merged { node: 1, tag: 1 }, 4, false, (0xffff_fffc, 0));
    let c2 = a.var_of_known(VarKind::Merged { node: 1, tag: 2 }, 4, false, (0xffff_fffc, 0));
    let both = a.mk_bin(CPUI_INT_AND, c1, c2, 4);
    assert_eq!(a.identity_probe(both), None, "{}", a.fmt(both));
    let three = a.cst(4, 3);
    let or3 = a.mk_bin(CPUI_INT_OR, both, three, 4);
    assert_eq!(a.identity_probe(or3), Some((None, 3)), "{}", a.fmt(or3));
    // deterministic: a second arena built the same way answers the same
    let mut b = ExprArena::new();
    let d1 = b.var_of_known(VarKind::Merged { node: 1, tag: 1 }, 4, false, (0xffff_fffc, 0));
    let d2 = b.var_of_known(VarKind::Merged { node: 1, tag: 2 }, 4, false, (0xffff_fffc, 0));
    let bb = b.mk_bin(CPUI_INT_AND, d1, d2, 4);
    assert_eq!(b.identity_probe(bb), None);
    assert_eq!((0..ID_SAMPLES).map(|i| a.sample_value(0, i)).collect::<Vec<_>>(), (0..ID_SAMPLES).map(|i| b.sample_value(0, i)).collect::<Vec<_>>());
}

/// A flags word pushed by `pushf` and mixed back out through
/// `(w ^ K) + 2 * (w & K) - w == K`: the value is constant whatever the
/// flags are, which only treating the atoms as free inputs shows.
#[test]
fn identity_probe_cancels_flag_words_over_free_atoms() {
    let mut a = ExprArena::new();
    let sp = a.var_of_known(VarKind::InitReg { offset: 0x10 }, 4, false, (0x3, 0xfffc));
    a.mark_pointer(sp);
    let ret = a.var(VarInfo::new(VarKind::CallResult { site: 0x401376 }, 4, false));
    let k = a.cst(4, 0x1c);
    let zero = a.cst(4, 0);
    let x = a.mk_bin(CPUI_INT_ADD, sp, ret, 4);
    let sum = a.mk_bin(CPUI_INT_ADD, x, k, 4);
    let flags = [
        (a.mk_bin(CPUI_INT_CARRY, x, k, 1), 0x1u128),
        (a.mk_bin(CPUI_INT_EQUAL, sum, zero, 1), 0x40),
        (a.mk_bin(CPUI_INT_SLESS, sum, zero, 1), 0x80),
        (a.mk_bin(CPUI_INT_SCARRY, x, k, 1), 0x800),
    ];
    let rr = a.mk_bin(CPUI_INT_ADD, ret, ret, 4);
    let flags2 = [
        (a.mk_bin(CPUI_INT_CARRY, ret, ret, 1), 0x1u128),
        (a.mk_bin(CPUI_INT_EQUAL, rr, zero, 1), 0x40),
        (a.mk_bin(CPUI_INT_SLESS, rr, zero, 1), 0x80),
    ];
    // two flags words added: their bits overlap, so no disjoint-sum rule applies
    let mut word = a.cst(4, 0x202);
    for (f, w) in flags.into_iter().chain(flags2) {
        let z = a.mk_un(CPUI_INT_ZEXT, f, 4);
        let wc = a.cst(4, w);
        let t = a.mk_bin(CPUI_INT_MULT, z, wc, 4);
        word = a.mk_bin(CPUI_INT_ADD, word, t, 4);
    }
    let key = a.cst(4, 0xef7f_e5be);
    let xk = a.mk_bin(CPUI_INT_XOR, word, key, 4);
    let ak = a.mk_bin(CPUI_INT_AND, word, key, 4);
    let two = a.cst(4, 2);
    let ak2 = a.mk_bin(CPUI_INT_MULT, ak, two, 4);
    let s1 = a.mk_bin(CPUI_INT_ADD, xk, ak2, 4);
    let back = a.mk_bin(CPUI_INT_SUB, s1, word, 4);
    assert_eq!(a.affine(back), None, "{}", a.fmt(back));
    assert_eq!(a.identity_probe(back), Some((None, 0xef7f_e5beu32 as i32 as i128)), "{}", a.fmt(back));
    // the word itself depends on the flags
    assert_eq!(a.identity_probe(word), None, "{}", a.fmt(word));
    let addr = a.mk_bin(CPUI_INT_ADD, back, sp, 4);
    assert_eq!(a.affine_probe(addr), Some((a.as_var(sp), 0xef7f_e5beu32 as i32 as i128)), "{}", a.fmt(addr));
}

/// Themida decodes a context offset `k` through a key, `x ^ ((x & ~k) +
/// (~x & k))` with `x` the key plus a constant, which is `k` whatever the key
/// holds.  Over one merge symbol (no pointer) the three probe points never
/// take a constant, so the identity test decides it; a near miss stays open.
#[test]
fn a_key_decoded_offset_over_one_symbol_is_its_constant() {
    let mut a = ExprArena::new();
    let s = a.var_of(VarKind::Merged { node: 3, tag: 4 }, 4, false);
    let c = a.cst(4, 0xccaf_056c);
    let x = a.mk_bin(CPUI_INT_ADD, s, c, 4);
    let nk = a.cst(4, 0xffff_ffb3);
    let lo = a.mk_bin(CPUI_INT_AND, x, nk, 4);
    let ns = a.mk_un(CPUI_INT_2COMP, s, 4);
    let k = a.cst(4, 0x4c);
    let decode = |a: &mut ExprArena, d: u128| {
        let cd = a.cst(4, d);
        let nx = a.mk_bin(CPUI_INT_ADD, ns, cd, 4);
        let hi = a.mk_bin(CPUI_INT_AND, nx, k, 4);
        let sum = a.mk_bin(CPUI_INT_ADD, lo, hi, 4);
        a.mk_bin(CPUI_INT_XOR, x, sum, 4)
    };
    let off = decode(&mut a, 0x3350_fa93);
    assert_eq!(a.affine_probe(off), Some((None, 0x4c)), "{}", a.fmt(off));
    let near = decode(&mut a, 0x3350_fa94);
    assert_eq!(a.affine_probe(near), None, "{}", a.fmt(near));
}
