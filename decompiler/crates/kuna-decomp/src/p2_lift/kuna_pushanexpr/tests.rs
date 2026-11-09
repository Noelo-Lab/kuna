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
