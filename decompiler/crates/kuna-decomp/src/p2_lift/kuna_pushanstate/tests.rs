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
    let d = s1.diff_constants(&mut s2, &mut a, &img);
    assert_eq!(d, vec![SymLoc { loc: Loc::Reg { off: 0x30 }, size: 8 }]);
    s1.symbolize(&mut a, 7, &d);
    let v = s1.read_reg(&mut a, 0x30, 8);
    assert!(a.is_const(v).is_none());
    let mut s3 = s1.clone();
    s3.symbolize(&mut a, 7, &d);
    assert_eq!(s3.read_reg(&mut a, 0x30, 8), v);
}
