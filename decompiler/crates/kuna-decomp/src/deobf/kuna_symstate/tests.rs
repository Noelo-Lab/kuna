use super::*;

struct NoImage;
impl SymImage for NoImage {
    fn read_bytes(&self, _addr: u64, _buf: &mut [u8]) -> bool {
        false
    }
    fn is_writable(&self, _addr: u64) -> bool {
        true
    }
}

struct ConstImage;
impl SymImage for ConstImage {
    fn read_bytes(&self, addr: u64, buf: &mut [u8]) -> bool {
        if !(0x1000..0x2000).contains(&addr) {
            return false;
        }
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (addr as u8).wrapping_add(i as u8);
        }
        true
    }
    fn is_writable(&self, _addr: u64) -> bool {
        false
    }
}

#[test]
fn sub_registers_compose_and_slice() {
    let mut a = ExprArena::new();
    let mut s = State::new(false, (0x200, 0x216));
    let rax = s.read_reg(&mut a, 0, 8);
    let eax = s.read_reg(&mut a, 0, 4);
    assert_eq!(eax, a.mk_subpiece(rax, 0, 4));
    let ah = s.read_reg(&mut a, 1, 1);
    assert_eq!(ah, a.mk_subpiece(rax, 1, 1));
    // write AL, read back RAX: bytes 1..8 of the old value remain
    let k = a.cst(1, 0x77);
    s.write_reg(0, k, 1);
    let rax2 = s.read_reg(&mut a, 0, 8);
    let hi = a.mk_subpiece(rax, 1, 7);
    assert_eq!(rax2, a.mk_piece(hi, k));
    // a full write replaces everything
    let c = a.cst(8, 0x1234);
    s.write_reg(0, c, 8);
    assert_eq!(s.read_reg(&mut a, 0, 8), c);
    assert_eq!(s.read_reg(&mut a, 0, 2), a.cst(2, 0x1234));
    assert_eq!(s.read_reg(&mut a, 1, 1), a.cst(1, 0x12));
    // flags are boolean symbols
    let zf = s.read_reg(&mut a, 0x206, 1);
    assert!(a.is_bool(zf));
}

#[test]
fn memory_regions_and_image() {
    let mut a = ExprArena::new();
    let mut s = State::new(false, (0x200, 0x216));
    let img = ConstImage;
    let v = s.read_mem(&mut a, &img, None, 0x1000, 4);
    assert_eq!(a.is_const(v), Some(0x03020100));
    let unk = s.read_mem(&mut a, &img, None, 0x9000, 8);
    assert!(a.is_const(unk).is_none());
    let again = s.read_mem(&mut a, &img, None, 0x9000, 8);
    assert_eq!(unk, again);
    let rsp = a.var_of(VarKind::InitReg { offset: 0x20 }, 8, false);
    let base = a.as_var(rsp);
    let k = a.cst(8, 0xdead);
    s.write_mem(base, -8, k, 8);
    assert_eq!(s.read_mem(&mut a, &img, base, -8, 8), k);
    assert_eq!(s.read_mem(&mut a, &img, base, -8, 4), a.cst(4, 0xdead));
    assert_eq!(s.read_mem(&mut a, &img, base, -4, 4), a.cst(4, 0));
    // other regions are independent
    let other = s.read_mem(&mut a, &img, None, 0x9000, 8);
    assert_eq!(other, unk);
}

#[test]
fn diff_and_symbolize() {
    let mut a = ExprArena::new();
    let img = NoImage;
    let mut s1 = State::new(false, (0x200, 0x216));
    let mut s2 = s1.clone();
    let c1 = a.cst(8, 1);
    let c2 = a.cst(8, 2);
    s1.write_reg(0x30, c1, 8);
    s2.write_reg(0x30, c2, 8);
    s1.write_reg(0x38, c1, 8);
    s2.write_reg(0x38, c1, 8);
    let d = s1.diff_constants(&mut s2, &mut a, &img, 8);
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].loc, SymLoc { loc: Loc::Reg { off: 0x30 }, size: 8 });
    // a differing byte inside an otherwise symbolic register is found narrowly
    let mut s5 = s1.clone();
    let mut s6 = s1.clone();
    let b1 = a.cst(1, 1);
    let b2 = a.cst(1, 2);
    s5.write_reg(0x41, b1, 1);
    s6.write_reg(0x41, b2, 1);
    let d2 = s5.diff_constants(&mut s6, &mut a, &img, 8);
    assert!(
        d2.iter().any(|d| matches!(d.loc.loc, Loc::Reg { off } if off <= 0x41 && 0x41 < off + d.loc.size as u64)),
        "{d2:?}"
    );
    assert_eq!(d[0].values, Some((1, 2)));
    let locs = vec![d[0].loc];
    let made = s1.symbolize(&mut a, 7, &locs);
    let v = s1.read_reg(&mut a, 0x30, 8);
    assert!(a.is_const(v).is_none());
    assert_eq!(made[0].1, v);
    let mut s3 = s1.clone();
    s3.symbolize(&mut a, 7, &locs);
    assert_eq!(s3.read_reg(&mut a, 0x30, 8), v);
}

/// Image bytes at 0x1000..0x2000, of which 0x1800.. is a writable data section.
struct DataImage;
impl SymImage for DataImage {
    fn read_bytes(&self, addr: u64, buf: &mut [u8]) -> bool {
        ConstImage.read_bytes(addr, buf)
    }
    fn is_writable(&self, addr: u64) -> bool {
        addr >= 0x1800
    }
}

#[test]
fn mutable_memory_is_not_the_file_and_calls_overwrite_it() {
    let mut a = ExprArena::new();
    let mut s = State::new(false, (0x200, 0x216));
    let img = DataImage;
    // read-only bytes are the file's; a writable global is its entry value
    let ro = s.read_mem(&mut a, &img, None, 0x1000, 4);
    assert!(a.is_const(ro).is_some());
    let g = s.read_mem(&mut a, &img, None, 0x1800, 4);
    assert!(a.is_const(g).is_none());
    assert_eq!(s.read_mem(&mut a, &img, None, 0x1800, 4), g);
    // a store sticks until an external call, which leaves an unknown
    let k = a.cst(4, 7);
    s.write_mem(None, 0x1804, k, 4);
    assert_eq!(s.read_mem(&mut a, &img, None, 0x1804, 4), k);
    s.clobber_mutable(&img, 1);
    let after = s.read_mem(&mut a, &img, None, 0x1804, 4);
    assert!(a.is_const(after).is_none());
    let g2 = s.read_mem(&mut a, &img, None, 0x1800, 4);
    assert_ne!(g2, g);
    // read-only memory survives
    let ro = s.read_mem(&mut a, &img, None, 0x1000, 4);
    assert!(a.is_const(ro).is_some());
}

#[test]
fn file_reads_see_writable_bytes_and_drop_cached_symbols() {
    let mut a = ExprArena::new();
    let mut s = State::new(false, (0x200, 0x216));
    let img = DataImage;
    // an ordinary read of writable memory caches the entry-value symbol
    let g = s.read_mem(&mut a, &img, None, 0x1810, 2);
    assert!(a.is_const(g).is_none());
    // a fetch through the VPC reads the file's bytes all the same
    let f = s.read_mem_file(&mut a, &img, None, 0x1810, 2);
    assert_eq!(a.is_const(f), Some(0x1110));
    // and a byte the path wrote reads what it wrote
    let k = a.cst(1, 0x55);
    s.write_mem(None, 0x1811, k, 1);
    let w = s.read_mem_file(&mut a, &img, None, 0x1810, 2);
    assert_eq!(a.is_const(w), Some(0x5510));
}

#[test]
fn a_copy_reads_its_source_lazily_until_overwritten_or_forgotten() {
    let mut a = ExprArena::new();
    let mut s = State::new(false, (0x200, 0x216));
    let img = DataImage;
    let rsp = a.var_of(VarKind::InitReg { offset: 0x20 }, 8, false);
    let sp = a.as_var(rsp);
    // a byte the path wrote to the source before the copy is copied too
    let k = a.cst(1, 0x99);
    s.write_mem(None, 0x1802, k, 1);
    assert!(s.copy_in(sp, -0x400, 0x1800, 0x100));
    assert_eq!(s.copies.len(), 1);
    assert_eq!(s.copy_source(sp, -0x3ff), Some(0x1801));
    assert_eq!(s.copy_source(sp, -0x300), None);
    // the unwritten destination reads the (writable) source's entry value,
    // and its file bytes through a fetch
    let src = s.read_mem(&mut a, &img, None, 0x1804, 4);
    assert_eq!(s.read_mem(&mut a, &img, sp, -0x3fc, 4), src);
    let f = s.read_mem_file(&mut a, &img, sp, -0x400, 4);
    assert_eq!(a.is_const(f), Some(0x0399_0100));
    // a store over the copy wins, and forgetting part of it keeps the rest
    let w = a.cst(4, 7);
    s.write_mem(sp, -0x3f0, w, 4);
    assert_eq!(s.read_mem(&mut a, &img, sp, -0x3f0, 4), w);
    s.forget_copies(sp, -0x380, -0x370);
    assert_eq!(s.copies.len(), 2);
    assert_eq!(s.copy_source(sp, -0x378), None);
    assert_eq!(s.copy_source(sp, -0x36f), Some(0x1891));
    // a copy out of another copy's concrete destination is refused
    assert!(s.copy_in(None, 0x1900, 0x1800, 0x10));
    assert!(!s.copy_in(sp, -0x800, 0x1900, 0x10));
    // an external call drops a concrete destination in mutable memory
    s.clobber_mutable(&img, 1);
    assert!(s.copies.iter().all(|c| c.base.is_some()));
}

#[test]
fn a_merged_decision_splits_over_its_recorded_constants() {
    use super::super::kuna_symemu::{split_over_symbols, symbol_pin, SymValues};
    use kuna_num::opcodes::OpCode::*;
    let mut a = ExprArena::new();
    let mut s = State::new(false, (0x200, 0x216));
    let locs = vec![(SymLoc { loc: Loc::Reg { off: 0x30 }, size: 4 }, (0xffff_fffe, 0))];
    let made = s.symbolize_known(&mut a, 9, &locs);
    let flag = made[0].1;
    let fv = a.as_var(flag).unwrap();
    let x = s.read_reg(&mut a, 0, 4);
    let z = a.cst(4, 0);
    let c = a.mk_bin(CPUI_INT_EQUAL, x, z, 1);
    let zc = a.mk_un(CPUI_INT_ZEXT, c, 4);
    let one = a.cst(4, 1);
    // vpc = 0x1000 + 0x40 * (flag ? (x == 0) : 1): a virtual branch whose
    // decision a join merged into `flag`
    let sel = a.mk_subpiece(flag, 0, 1);
    let pick = a.mk_ite(sel, zc, one);
    let k = a.cst(4, 0x40);
    let scaled = a.mk_bin(CPUI_INT_MULT, pick, k, 4);
    let base = a.cst(4, 0x1000);
    let vpc = a.mk_bin(CPUI_INT_ADD, base, scaled, 4);
    assert!(a.case_split(vpc).is_none_or(|cs| cs.iter().any(|(_, e)| a.is_const(*e).is_none())));
    let values: SymValues = Default::default();
    assert!(split_over_symbols(&mut a, &values, vpc).is_none(), "no recorded constants: no split");
    values.borrow_mut().insert(fv, [0u128, 1].into_iter().collect());
    let cases = split_over_symbols(&mut a, &values, vpc).expect("split");
    let mut got: Vec<(u128, u128, Option<bool>)> = cases
        .iter()
        .map(|(assign, e)| {
            let (sym, kc) = symbol_pin(&mut a, &values, assign[0].0).expect("pin first");
            assert_eq!(sym, flag);
            (a.is_const(kc).unwrap(), a.is_const(*e).unwrap(), assign.get(1).map(|(atom, v)| {
                assert_eq!(*atom, c);
                *v
            }))
        })
        .collect();
    got.sort();
    assert_eq!(got, vec![(0, 0x1040, None), (1, 0x1000, Some(false)), (1, 0x1040, Some(true))]);
    // a value set over the limit is left unsplit
    let many: std::collections::BTreeSet<u128> = (0..17).collect();
    values.borrow_mut().insert(fv, many);
    assert!(split_over_symbols(&mut a, &values, vpc).is_none());
}

/// Two paths rebuild a register through different identities: no difference.
#[test]
fn a_value_rebuilt_through_an_identity_is_no_difference() {
    use kuna_num::opcodes::OpCode::*;
    let mut a = ExprArena::new();
    let img = NoImage;
    let sp = a.var_of_known(VarKind::InitReg { offset: 0x10 }, 4, false, (0x3, 0xfffc));
    let k = a.cst(4, 0xffff_fffc);
    let y = a.mk_bin(CPUI_INT_ADD, sp, k, 4);
    let (m, n) = (a.cst(4, 0xb46b_af2b), a.cst(4, 0x4b94_50d4));
    let ym = a.mk_bin(CPUI_INT_AND, y, m, 4);
    let ny = a.mk_un(CPUI_INT_NEGATE, y, 4);
    let nyn = a.mk_bin(CPUI_INT_AND, ny, n, 4);
    let rest = a.mk_bin(CPUI_INT_SUB, n, nyn, 4);
    let back = a.mk_bin(CPUI_INT_OR, ym, rest, 4);
    assert_ne!(y, back);
    let mut s1 = State::new(false, (0x200, 0x216));
    let mut s2 = s1.clone();
    s1.write_reg(0x30, y, 4);
    s2.write_reg(0x30, back, 4);
    assert!(s1.diff_constants(&mut s2, &mut a, &img, 4).is_empty());
    // a value that differs stays a difference
    let one = a.cst(4, 1);
    let off = a.mk_bin(CPUI_INT_ADD, back, one, 4);
    let mut s3 = s1.clone();
    s3.write_reg(0x30, off, 4);
    assert_eq!(s1.diff_constants(&mut s3, &mut a, &img, 4).len(), 1);
}

#[test]
fn a_merge_leaves_the_byte_an_unaligned_neighbour_shares_with_the_slot() {
    use kuna_num::opcodes::OpCode::*;
    let mut a = ExprArena::new();
    let img = NoImage;
    let sp = a.var_of(VarKind::InitReg { offset: 0x20 }, 8, false);
    let k = a.cst(8, (-0x58i64) as u64 as u128);
    let fp = a.mk_bin(CPUI_INT_ADD, sp, k, 8);
    let x = a.var_of(VarKind::InitReg { offset: 0x8 }, 8, false);
    let y = a.var_of(VarKind::InitReg { offset: 0x10 }, 8, false);
    let mut s1 = State::new(false, (0x200, 0x216));
    s1.write_mem(None, 0x1001, fp, 8);
    let mut s2 = s1.clone();
    s1.write_mem(None, 0x1009, x, 8);
    s2.write_mem(None, 0x1009, y, 8);
    let d = s1.diff_constants(&mut s2, &mut a, &img, 8);
    assert!(d.iter().all(|d| !matches!(d.loc.loc, Loc::Mem { off, .. } if off <= 0x1008 && 0x1008 < off + d.loc.size as i128)), "{d:?}");
    assert!(d.iter().any(|d| d.loc == SymLoc { loc: Loc::Mem { base: None, off: 0x1009 }, size: 7 }), "{d:?}");
    let locs: Vec<SymLoc> = d.iter().map(|d| d.loc).collect();
    s1.symbolize(&mut a, 9, &locs);
    assert_eq!(s1.read_mem(&mut a, &img, None, 0x1001, 8), fp);
}

#[test]
fn a_call_overwrites_symbolic_regions_and_the_escaped_frame() {
    let mut a = ExprArena::new();
    let mut s = State::new(false, (0x200, 0x216));
    let img = DataImage;
    let sp = a.var_of(VarKind::InitReg { offset: 0x20 }, 8, false);
    let frame = a.as_var(sp).unwrap();
    let p = a.var_of(VarKind::InitReg { offset: 0x08 }, 8, false);
    let pv = a.as_var(p).unwrap();
    // a field behind a pointer parameter, read and written before the call
    let field = s.read_mem(&mut a, &img, Some(pv), 0x400, 4);
    let k = a.cst(4, 7);
    s.write_mem(Some(pv), 0x10, k, 4);
    // a local whose address escaped, one below it, and the return address
    let five = a.cst(8, 5);
    s.write_mem(Some(frame), -0x90, five, 8);
    s.write_mem(Some(frame), -0x140, five, 8);
    let ret = s.read_mem(&mut a, &img, Some(frame), 0, 8);
    s.clobber_call(&img, 3, frame, vec![(-0x90, 0)]);
    let field2 = s.read_mem(&mut a, &img, Some(pv), 0x400, 4);
    assert_ne!(field2, field, "an unwritten field is the call's output");
    let stored = s.read_mem(&mut a, &img, Some(pv), 0x10, 4);
    assert!(a.is_const(stored).is_none(), "a stored field is dropped");
    let local = s.read_mem(&mut a, &img, Some(frame), -0x90, 8);
    assert!(a.is_const(local).is_none(), "the escaped local may be written");
    assert_eq!(s.read_mem(&mut a, &img, Some(frame), -0x140, 8), five, "the frame below every escaped offset stays");
    assert_eq!(s.read_mem(&mut a, &img, Some(frame), 0, 8), ret, "the return address stays");
    // a read straddling the range's lower end is pieced together
    let low = s.read_mem(&mut a, &img, Some(frame), -0x94, 8);
    assert!(a.is_const(low).is_none());
    assert_eq!(s.read_mem(&mut a, &img, Some(pv), 0x400, 4), field2, "one call's output is one value");
}
