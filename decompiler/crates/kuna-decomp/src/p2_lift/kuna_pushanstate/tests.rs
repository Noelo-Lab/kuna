use super::*;

struct NoImage;
impl PushanImage for NoImage {
    fn read_bytes(&self, _addr: u64, _buf: &mut [u8]) -> bool {
        false
    }
    fn is_writable(&self, _addr: u64) -> bool {
        true
    }
}

struct ConstImage;
impl PushanImage for ConstImage {
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
impl PushanImage for DataImage {
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
