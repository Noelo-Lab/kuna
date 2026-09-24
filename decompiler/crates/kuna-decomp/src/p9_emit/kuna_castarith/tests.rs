use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::ArchContext;
use crate::dtype::{Datatype, TypeFactoryImpl};

struct Fx {
    fd: Funcdata,
    tf: Rc<TypeFactoryImpl>,
    ram: Rc<AddrSpace>,
    next: u64,
    bl: crate::context::BlockId,
}

impl Fx {
    fn new() -> Fx {
        Fx::with(true)
    }

    fn with(cast_arith: bool) -> Fx {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
        m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        let tf = TypeFactoryImpl::new();
        tf.setup_sizes(Some(4), 8, 8);
        tf.set_default_alignment_map();
        tf.set_max_basetype_size(8);
        let tf = Rc::new(tf);
        let mut arch = ArchContext::new(m);
        arch.types = Some(Rc::clone(&tf));
        arch.cast_arith = cast_arith;
        let glb = Rc::new(arch);
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let mut fd = Funcdata::new(
            "f",
            "f",
            glb,
            Address::new(Rc::clone(&ram), 0x1000),
            0x1000_0000,
            0x40,
        )
        .unwrap();
        fd.get_func_proto_mut()
            .attach_internal_store(tf.get_type_void().unwrap());
        let root = fd.bblocks_ref().root.expect("bblocks root");
        let bl = fd.bblocks_mut().new_block_basic(root);
        Fx {
            fd,
            tf,
            ram,
            next: 0x5000,
            bl,
        }
    }

    fn base(&self, size: int4, m: type_metatype) -> Rc<Datatype> {
        self.tf.get_base(size, m).unwrap()
    }

    fn ptr(&self, to: Rc<Datatype>) -> Rc<Datatype> {
        self.tf.get_type_pointer(8, to, 1).unwrap()
    }

    fn void_ptr(&self) -> Rc<Datatype> {
        self.ptr(self.tf.get_type_void().unwrap())
    }

    fn pc(&mut self) -> Address {
        self.next += 4;
        Address::new(Rc::clone(&self.ram), self.next)
    }

    fn op(&mut self, n: int4, opc: OpCode) -> OpId {
        let pc = self.pc();
        let op = self.fd.new_op(n, pc);
        self.fd.op_set_opcode_code(op, opc);
        self.fd.op_insert_end(op, self.bl);
        op
    }

    fn var(&mut self, size: int4, ty: Rc<Datatype>) -> VarnodeId {
        self.next += 0x10;
        let at = Address::new(Rc::clone(&self.ram), 0x10_0000 + self.next);
        let vn = self.fd.new_varnode(size, &at, Some(ty));
        self.fd.set_input_varnode(vn).unwrap()
    }

    /// `out = INT_ADD(p, #k)`, `out` a fresh implied temporary typed `outty`.
    fn add(&mut self, p: VarnodeId, k: i64, outty: Rc<Datatype>) -> (OpId, VarnodeId) {
        let op = self.op(2, OpCode::CPUI_INT_ADD);
        let c = self.fd.new_constant(8, k as uintb);
        self.fd.op_set_input(op, p, 0).unwrap();
        self.fd.op_set_input(op, c, 1).unwrap();
        let out = self.fd.new_unique_out(8, op).unwrap();
        self.fd.vn_update_type(out, outty);
        self.fd.vbank_mut().get_mut(out).unwrap().set_implied();
        (op, out)
    }

    /// `val = LOAD(ram, addr)`, `val` explicit and typed `ty`.
    fn load(&mut self, addr: VarnodeId, ty: Rc<Datatype>) -> OpId {
        let op = self.op(2, OpCode::CPUI_LOAD);
        let spc = self.fd.new_constant(8, 0);
        self.fd.op_set_input(op, spc, 0).unwrap();
        self.fd.op_set_input(op, addr, 1).unwrap();
        let size = ty.get_size();
        let out = self.fd.new_unique_out(size, op).unwrap();
        self.fd.vn_update_type(out, ty);
        self.fd.vbank_mut().get_mut(out).unwrap().set_explicit();
        op
    }

    /// `STORE(ram, addr, val)`.
    fn store(&mut self, addr: VarnodeId, val: VarnodeId) -> OpId {
        let op = self.op(3, OpCode::CPUI_STORE);
        let spc = self.fd.new_constant(8, 0);
        self.fd.op_set_input(op, spc, 0).unwrap();
        self.fd.op_set_input(op, addr, 1).unwrap();
        self.fd.op_set_input(op, val, 2).unwrap();
        op
    }

    /// `out = <opc>(a, b)`, `out` explicit and typed `ty`.
    fn binary(&mut self, opc: OpCode, a: VarnodeId, b: VarnodeId, ty: Rc<Datatype>) -> OpId {
        let op = self.op(2, opc);
        self.fd.op_set_input(op, a, 0).unwrap();
        self.fd.op_set_input(op, b, 1).unwrap();
        let size = ty.get_size();
        let out = self.fd.new_unique_out(size, op).unwrap();
        self.fd.vn_update_type(out, ty);
        self.fd.vbank_mut().get_mut(out).unwrap().set_explicit();
        op
    }

    fn high(&mut self) {
        self.fd.set_high_level();
    }

    fn input(&self, op: OpId, slot: int4) -> VarnodeId {
        self.fd.obank().get(op).unwrap().get_in(slot).unwrap()
    }

    fn offset(&self, vn: VarnodeId) -> uintb {
        self.fd.vbank().get(vn).unwrap().get_offset()
    }
}

/// The address a rewritten `PTRADD` computes from a base address `p`.
fn ptradd_address(fx: &Fx, op: OpId, p: u64) -> u64 {
    let idx = fx.offset(fx.input(op, 1));
    let elem = fx.offset(fx.input(op, 2));
    p.wrapping_add(idx.wrapping_mul(elem))
}

/// Rewrite the add and check it is a `PTRADD` whose address equals `p + k` for
/// several base addresses, with a base of type `T *`.
fn rewritten(fx: &mut Fx, op: OpId, k: i64, want_elem: u64, want_index: i64) -> VarnodeId {
    assert!(rewrite(&mut fx.fd, op), "the add was not rewritten");
    let o = fx.fd.obank().get(op).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_PTRADD);
    assert_eq!(o.num_input(), 3);
    assert_eq!(fx.offset(fx.input(op, 2)), want_elem);
    assert_eq!(fx.offset(fx.input(op, 1)) as i64, want_index);
    for p in [
        0u64,
        0x1000,
        0x7fff_ffff_fff0,
        0xffff_ffff_ffff_fff8,
        0x5555_5555_5555_5550,
    ] {
        assert_eq!(
            ptradd_address(fx, op, p),
            p.wrapping_add(k as u64),
            "k={k} p={p:#x}"
        );
    }
    fx.input(op, 0)
}

fn cast_input_of(fx: &Fx, vn: VarnodeId) -> Option<VarnodeId> {
    let def = fx.fd.vbank().get(vn)?.get_def()?;
    let o = fx.fd.obank().get(def)?;
    (o.code() == OpCode::CPUI_CAST)
        .then(|| o.get_in(0))
        .flatten()
}

#[test]
fn every_width_and_sign_becomes_an_element_of_that_width() {
    for (size, meta) in [
        (1, type_metatype::TYPE_INT),
        (1, type_metatype::TYPE_UINT),
        (2, type_metatype::TYPE_INT),
        (2, type_metatype::TYPE_UINT),
        (4, type_metatype::TYPE_INT),
        (4, type_metatype::TYPE_UINT),
        (8, type_metatype::TYPE_INT),
        (8, type_metatype::TYPE_UINT),
        (4, type_metatype::TYPE_FLOAT),
        (8, type_metatype::TYPE_FLOAT),
    ] {
        let mut fx = Fx::new();
        let vp = fx.void_ptr();
        let t = fx.base(size, meta);
        let tp = fx.ptr(Rc::clone(&t));
        let p = fx.var(8, vp);
        let k = 0xa8;
        let (add, out) = fx.add(p, k, tp.clone());
        fx.load(out, t);
        fx.high();
        let base = rewritten(&mut fx, add, k, size as u64, k / size as i64);
        assert_eq!(
            cast_input_of(&fx, base),
            Some(p),
            "size {size}: the base is a cast of p"
        );
        assert!(Rc::ptr_eq(fx.fd.vbank().get(base).unwrap().get_type(), &tp));
    }
}

#[test]
fn a_negative_offset_is_a_negative_index() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let t = fx.base(2, type_metatype::TYPE_INT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, vp);
    let (add, out) = fx.add(p, -0x1a, tp);
    fx.load(out, t);
    fx.high();
    rewritten(&mut fx, add, -0x1a, 2, -0xd);
}

/// C types the literal `0x80000000` through `0xffffffff` as `unsigned int`, so
/// `((long *)a0)[-0x80000000]` indexes 2^31 elements FORWARD.  A negative index
/// that wide keeps the integer form, whose byte offset is a `long` literal; one
/// element short of it is still pointer arithmetic, as is a wide positive index.
#[test]
fn a_negative_index_c_would_read_as_unsigned_keeps_the_integer_form() {
    for (width, k) in [
        (8, -0x4_0000_0000i64),
        (8, -0x7_ffff_fff8),
        (8, -0x40_0000_0000),
        (4, -0x2_0000_0000),
        (4, -0x3_ffff_fffc),
        (2, -0x1_0000_0000),
        (1, -0x8000_0000),
    ] {
        let mut fx = Fx::new();
        let vp = fx.void_ptr();
        let t = fx.base(width, type_metatype::TYPE_INT);
        let tp = fx.ptr(Rc::clone(&t));
        let p = fx.var(8, vp);
        let (add, out) = fx.add(p, k, tp);
        fx.load(out, t);
        fx.high();
        assert!(
            matches!(plan(&mut fx.fd, add), Err(Leave::WideNegativeIndex)),
            "width {width} k {k:#x}"
        );
        assert!(!rewrite(&mut fx.fd, add));
        assert_eq!(fx.fd.obank().get(add).unwrap().code(), OpCode::CPUI_INT_ADD);
    }
    for (width, k) in [(8, -0x3_ffff_fff8i64), (4, -0x1_ffff_fffc), (8, 0x4_0000_0000)] {
        let mut fx = Fx::new();
        let vp = fx.void_ptr();
        let t = fx.base(width, type_metatype::TYPE_INT);
        let tp = fx.ptr(Rc::clone(&t));
        let p = fx.var(8, vp);
        let (add, out) = fx.add(p, k, tp);
        fx.load(out, t);
        fx.high();
        rewritten(&mut fx, add, k, width as u64, k / width as i64);
    }
}

#[test]
fn an_offset_that_is_not_whole_elements_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let t = fx.base(4, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, vp);
    let (add, out) = fx.add(p, 0x6a, tp);
    fx.load(out, t);
    fx.high();
    assert!(matches!(plan(&mut fx.fd, add), Err(Leave::NonDividing)));
    assert!(!rewrite(&mut fx.fd, add));
    assert_eq!(fx.fd.obank().get(add).unwrap().code(), OpCode::CPUI_INT_ADD);
}

#[test]
fn a_base_that_already_points_at_the_element_needs_no_cast() {
    let mut fx = Fx::new();
    let t = fx.base(4, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, Rc::clone(&tp));
    let (add, out) = fx.add(p, 0x3c, Rc::clone(&tp));
    fx.load(out, t);
    fx.high();
    let base = rewritten(&mut fx, add, 0x3c, 4, 0xf);
    assert_eq!(
        base, p,
        "a base declared unsigned int * is indexed directly"
    );
}

#[test]
fn a_base_of_another_width_is_cast_to_the_element_pointer() {
    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let lp = fx.ptr(long);
    let t = fx.base(4, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, lp);
    let (add, out) = fx.add(p, 0x3c, tp);
    fx.load(out, t);
    fx.high();
    let base = rewritten(&mut fx, add, 0x3c, 4, 0xf);
    assert_eq!(cast_input_of(&fx, base), Some(p));
}

#[test]
fn a_base_that_is_itself_a_cast_read_only_here_is_retargeted() {
    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let vp = fx.void_ptr();
    let t = fx.base(4, type_metatype::TYPE_INT);
    let tp = fx.ptr(Rc::clone(&t));
    let x = fx.var(8, long);
    let cast = fx.op(1, OpCode::CPUI_CAST);
    fx.fd.op_set_input(cast, x, 0).unwrap();
    let p = fx.fd.new_unique_out(8, cast).unwrap();
    fx.fd.vn_update_type(p, vp);
    fx.fd.vbank_mut().get_mut(p).unwrap().set_implied();
    let (add, out) = fx.add(p, 0x14, Rc::clone(&tp));
    fx.load(out, t);
    fx.high();
    assert!(matches!(
        plan(&mut fx.fd, add),
        Ok(Plan {
            base: Base::Retype,
            ..
        })
    ));
    let base = rewritten(&mut fx, add, 0x14, 4, 5);
    assert_eq!(base, p, "no second cast is stacked on the first");
    assert!(Rc::ptr_eq(fx.fd.vbank().get(p).unwrap().get_type(), &tp));
    assert_eq!(cast_input_of(&fx, p), Some(x));
}

#[test]
fn a_base_that_is_a_cast_other_ops_read_is_recast_from_its_input() {
    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let ulong = fx.base(8, type_metatype::TYPE_UINT);
    let up = fx.ptr(ulong);
    let t = fx.base(1, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let x = fx.var(8, long);
    let cast = fx.op(1, OpCode::CPUI_CAST);
    fx.fd.op_set_input(cast, x, 0).unwrap();
    let p = fx.fd.new_unique_out(8, cast).unwrap();
    fx.fd.vn_update_type(p, Rc::clone(&up));
    fx.fd.vbank_mut().get_mut(p).unwrap().set_implied();
    let (add, out) = fx.add(p, 10, Rc::clone(&tp));
    fx.load(out, t);
    fx.load(p, fx.base(8, type_metatype::TYPE_UINT));
    fx.high();
    assert!(matches!(
        plan(&mut fx.fd, add),
        Ok(Plan {
            base: Base::Recast(src),
            ..
        }) if src == x
    ));
    let base = rewritten(&mut fx, add, 10, 1, 10);
    assert_ne!(base, p, "the shared cast is not retyped under its other reader");
    assert_eq!(cast_input_of(&fx, base), Some(x), "no cast is stacked on the shared one");
    assert!(Rc::ptr_eq(fx.fd.vbank().get(base).unwrap().get_type(), &tp));
    assert!(Rc::ptr_eq(fx.fd.vbank().get(p).unwrap().get_type(), &up));
}

#[test]
fn a_stored_address_indexes_the_stored_value_type() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let t = fx.base(2, type_metatype::TYPE_UINT);
    let p = fx.var(8, vp.clone());
    let v = fx.var(2, t);
    let (add, out) = fx.add(p, 0x82, vp);
    fx.store(out, v);
    fx.high();
    rewritten(&mut fx, add, 0x82, 2, 0x41);
}

#[test]
fn a_pointer_that_is_stored_or_compared_keeps_its_type_and_a_subtracted_one_stays_integer() {
    for consumer in ["store", "compare", "subtract"] {
        let mut fx = Fx::new();
        let vp = fx.void_ptr();
        let t = fx.base(4, type_metatype::TYPE_UINT);
        let tp = fx.ptr(Rc::clone(&t));
        let long = fx.base(8, type_metatype::TYPE_INT);
        let boolt = fx.base(1, type_metatype::TYPE_BOOL);
        let p = fx.var(8, vp);
        let q = fx.var(8, Rc::clone(&tp));
        let (add, out) = fx.add(p, 0x28, Rc::clone(&tp));
        match consumer {
            "store" => {
                let slot = fx.var(8, fx.ptr(Rc::clone(&tp)));
                fx.store(slot, out);
            }
            "compare" => {
                fx.binary(OpCode::CPUI_INT_EQUAL, out, q, boolt);
            }
            _ => {
                fx.binary(OpCode::CPUI_INT_SUB, q, out, long);
            }
        }
        if consumer == "subtract" {
            fx.high();
            assert!(
                matches!(plan(&mut fx.fd, add), Err(Leave::IntegerUse)),
                "a subtracted sum stays integer"
            );
            continue;
        }
        fx.high();
        let base = rewritten(&mut fx, add, 0x28, 4, 0xa);
        assert!(
            Rc::ptr_eq(fx.fd.vbank().get(base).unwrap().get_type(), &tp),
            "{consumer}"
        );
    }
}

#[test]
fn a_void_variable_is_stepped_in_bytes() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let p = fx.var(8, vp.clone());
    let (add, out) = fx.add(p, 0x28, vp);
    let v = fx.fd.vbank_mut().get_mut(out).unwrap();
    v.clear_implied();
    v.set_explicit();
    fx.high();
    let base = rewritten(&mut fx, add, 0x28, 1, 0x28);
    let bt = fx.fd.vbank().get(base).unwrap().get_type().clone();
    assert_eq!(bt.get_ptr_to().unwrap().get_size(), 1);
}

#[test]
fn a_void_sum_passed_to_a_call_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let p = fx.var(8, vp.clone());
    let (add, out) = fx.add(p, 0x28, vp);
    let call = fx.op(2, OpCode::CPUI_CALL);
    let target = fx
        .fd
        .new_code_ref(&Address::new(Rc::clone(&fx.ram), 0x9000));
    fx.fd.op_set_input(call, target, 0).unwrap();
    fx.fd.op_set_input(call, out, 1).unwrap();
    fx.high();
    assert!(matches!(plan(&mut fx.fd, add), Err(Leave::VoidTarget)));
}

#[test]
fn a_void_sum_compared_in_place_is_stepped_in_bytes() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let p = fx.var(8, vp.clone());
    let q = fx.var(8, vp.clone());
    let boolt = fx.base(1, type_metatype::TYPE_BOOL);
    let (add, out) = fx.add(p, 0x28, vp);
    fx.binary(OpCode::CPUI_INT_EQUAL, out, q, boolt);
    fx.high();
    rewritten(&mut fx, add, 0x28, 1, 0x28);
}

#[test]
fn a_store_through_a_sum_of_the_other_signedness_keeps_the_sum_type() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let short = fx.base(2, type_metatype::TYPE_INT);
    let sp = fx.ptr(short);
    let ushort = fx.base(2, type_metatype::TYPE_UINT);
    let p = fx.var(8, vp);
    let v = fx.var(2, ushort);
    let (add, out) = fx.add(p, 0x6c, Rc::clone(&sp));
    fx.store(out, v);
    fx.high();
    let base = rewritten(&mut fx, add, 0x6c, 2, 0x36);
    assert!(Rc::ptr_eq(fx.fd.vbank().get(base).unwrap().get_type(), &sp));
}

#[test]
fn an_aggregate_element_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let u = fx.base(4, type_metatype::TYPE_UINT);
    let arr = fx.tf.get_type_array(4, u).unwrap();
    let ap = fx.ptr(arr);
    let q = fx.var(8, ap.clone());
    let p = fx.var(8, vp);
    let boolt = fx.base(1, type_metatype::TYPE_BOOL);
    let (add, out) = fx.add(p, 0x20, ap);
    fx.binary(OpCode::CPUI_INT_EQUAL, out, q, boolt);
    fx.high();
    assert!(matches!(plan(&mut fx.fd, add), Err(Leave::AggregateTarget)));
}

#[test]
fn an_integer_sum_or_an_integer_base_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let p = fx.var(8, vp);
    let n = fx.var(8, Rc::clone(&long));
    let (add, out) = fx.add(p, 0x10, Rc::clone(&long));
    fx.binary(OpCode::CPUI_INT_SUB, n, out, Rc::clone(&long));
    let x = fx.var(8, Rc::clone(&long));
    let t = fx.base(4, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let (add2, out2) = fx.add(x, 0x10, tp);
    fx.load(out2, t);
    fx.high();
    assert!(matches!(plan(&mut fx.fd, add), Err(Leave::OutNotPointer)));
    assert!(matches!(plan(&mut fx.fd, add2), Err(Leave::NoPointer)));
}

#[test]
fn a_variable_index_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let t = fx.base(4, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, vp);
    let i = fx.var(8, long);
    let add = fx.op(2, OpCode::CPUI_INT_ADD);
    fx.fd.op_set_input(add, p, 0).unwrap();
    fx.fd.op_set_input(add, i, 1).unwrap();
    let out = fx.fd.new_unique_out(8, add).unwrap();
    fx.fd.vn_update_type(out, tp);
    fx.fd.vbank_mut().get_mut(out).unwrap().set_implied();
    fx.load(out, t);
    fx.high();
    assert!(matches!(plan(&mut fx.fd, add), Err(Leave::NonConstant)));
}

#[test]
fn the_option_off_leaves_the_add_alone() {
    let mut fx = Fx::with(false);
    let vp = fx.void_ptr();
    let t = fx.base(4, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, vp);
    let (add, out) = fx.add(p, 8, tp);
    fx.load(out, t);
    fx.high();
    assert!(plan(&mut fx.fd, add).is_ok());
    let _ = fx.fd.action_set_casts();
    assert_eq!(fx.fd.obank().get(add).unwrap().code(), OpCode::CPUI_INT_ADD);
}

#[test]
fn a_base_of_the_other_signedness_and_the_same_width_needs_no_cast() {
    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let lp = fx.ptr(long);
    let ulong = fx.base(8, type_metatype::TYPE_UINT);
    let p = fx.var(8, Rc::clone(&lp));
    let v = fx.var(8, ulong);
    let (add, out) = fx.add(p, 0x20, lp);
    fx.store(out, v);
    fx.high();
    let base = rewritten(&mut fx, add, 0x20, 8, 4);
    assert_eq!(
        base, p,
        "a long * base stores an unsigned long element directly"
    );
}

#[test]
fn a_float_element_through_an_integer_base_is_still_cast() {
    let mut fx = Fx::new();
    let int = fx.base(4, type_metatype::TYPE_INT);
    let ip = fx.ptr(int);
    let fl = fx.base(4, type_metatype::TYPE_FLOAT);
    let p = fx.var(8, Rc::clone(&ip));
    let (add, out) = fx.add(p, 8, ip);
    fx.load(out, fl);
    fx.high();
    let base = rewritten(&mut fx, add, 8, 4, 2);
    assert_eq!(
        cast_input_of(&fx, base),
        Some(p),
        "an int * base is reinterpreted, never converted"
    );
}

#[test]
fn the_cast_pass_rewrites_the_add_when_the_option_is_on() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let t = fx.base(4, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, vp);
    let (add, out) = fx.add(p, 8, tp);
    fx.load(out, t);
    fx.high();
    let _ = fx.fd.action_set_casts();
    assert_eq!(fx.fd.obank().get(add).unwrap().code(), OpCode::CPUI_PTRADD);
}

#[test]
fn a_store_of_a_value_the_pass_has_not_reached_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let u = fx.base(4, type_metatype::TYPE_UINT);
    let up = fx.ptr(Rc::clone(&u));
    let f = fx.base(4, type_metatype::TYPE_FLOAT);
    let p = fx.var(8, vp);
    let a = fx.var(4, Rc::clone(&f));
    let b = fx.var(4, f);
    let (add, out) = fx.add(p, 8, up);
    let mul = fx.op(2, OpCode::CPUI_FLOAT_MULT);
    fx.fd.op_set_input(mul, a, 0).unwrap();
    fx.fd.op_set_input(mul, b, 1).unwrap();
    let prod = fx.fd.new_unique_out(4, mul).unwrap();
    fx.fd.vn_update_type(prod, u);
    fx.fd.vbank_mut().get_mut(prod).unwrap().set_implied();
    fx.store(out, prod);
    fx.high();
    assert!(matches!(plan(&mut fx.fd, add), Err(Leave::UnsettledStore)));
}

#[test]
fn a_store_of_a_value_computed_before_the_address_is_rewritten() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let f = fx.base(4, type_metatype::TYPE_FLOAT);
    let fp = fx.ptr(Rc::clone(&f));
    let p = fx.var(8, vp);
    let a = fx.var(4, Rc::clone(&f));
    let b = fx.var(4, Rc::clone(&f));
    let mul = fx.op(2, OpCode::CPUI_FLOAT_MULT);
    fx.fd.op_set_input(mul, a, 0).unwrap();
    fx.fd.op_set_input(mul, b, 1).unwrap();
    let prod = fx.fd.new_unique_out(4, mul).unwrap();
    fx.fd.vn_update_type(prod, f);
    fx.fd.vbank_mut().get_mut(prod).unwrap().set_implied();
    let (add, out) = fx.add(p, 8, fp);
    fx.store(out, prod);
    fx.high();
    rewritten(&mut fx, add, 8, 4, 2);
}

/// kuna prints every enum as a plain `enum`, which C sizes by its own rules,
/// while a packed enum, `-fshort-enums` or a C++ `enum class : uint8_t` is 1 or
/// 2 bytes in the binary: `((color *)p)[3]` would scale by C's `sizeof`, not by
/// the width the offset was measured in.  An enum element keeps the integer
/// form at every width, whether the access or the sum's own pointee names it.
#[test]
fn an_enum_element_keeps_the_integer_form() {
    for (size, k) in [(1, 3i64), (2, 6), (4, 0xc), (8, 0x10)] {
        for via in ["load-void-sum", "load-enum-sum", "compare", "call"] {
            let mut fx = Fx::new();
            let e = fx
                .tf
                .get_type_enum_sized(&format!("e{size}"), size, type_metatype::TYPE_ENUM_UINT)
                .unwrap();
            let ep = fx.ptr(Rc::clone(&e));
            let vp = fx.void_ptr();
            let p = fx.var(8, Rc::clone(&vp));
            match via {
                "load-void-sum" => {
                    let (add, out) = fx.add(p, k, vp);
                    fx.load(out, e);
                    fx.high();
                    assert_refused(&mut fx, add, Leave::EnumTarget, size, via);
                }
                "load-enum-sum" => {
                    let (add, out) = fx.add(p, k, ep);
                    fx.load(out, e);
                    fx.high();
                    assert_refused(&mut fx, add, Leave::EnumTarget, size, via);
                }
                "compare" => {
                    let q = fx.var(8, Rc::clone(&ep));
                    let boolt = fx.base(1, type_metatype::TYPE_BOOL);
                    let (add, out) = fx.add(p, k, ep);
                    fx.binary(OpCode::CPUI_INT_EQUAL, out, q, boolt);
                    fx.high();
                    assert_refused(&mut fx, add, Leave::EnumTarget, size, via);
                }
                _ => {
                    let (add, out) = fx.add(p, k, ep);
                    let call = fx.op(2, OpCode::CPUI_CALL);
                    let target = fx
                        .fd
                        .new_code_ref(&Address::new(Rc::clone(&fx.ram), 0x9000));
                    fx.fd.op_set_input(call, target, 0).unwrap();
                    fx.fd.op_set_input(call, out, 1).unwrap();
                    fx.high();
                    assert_refused(&mut fx, add, Leave::EnumTarget, size, via);
                }
            }
        }
    }
}

fn assert_refused(fx: &mut Fx, add: OpId, why: Leave, size: int4, via: &str) {
    let got = plan(&mut fx.fd, add).map(|_| ());
    assert_eq!(got, Err(why), "size {size} via {via}");
    assert!(!rewrite(&mut fx.fd, add));
    assert_eq!(fx.fd.obank().get(add).unwrap().code(), OpCode::CPUI_INT_ADD);
}

/// The printed index is scaled by C's `sizeof` of the printed element, so an
/// element C sizes differently keeps the integer form: a pointer narrower than
/// the target's pointers (C's `T *` is 8 bytes here), and a 16-byte float
/// (spelled `long double`, whose `sizeof` is 16, 12 or 8 by target).  A
/// one-byte `bool` and a pointer of the model's size are elements C sizes
/// exactly.
#[test]
fn an_element_c_sizes_differently_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let u = fx.base(4, type_metatype::TYPE_UINT);
    let narrow = fx.tf.get_type_pointer(4, u, 1).unwrap();
    let vp = fx.void_ptr();
    let p = fx.var(8, Rc::clone(&vp));
    let (add, out) = fx.add(p, 0x10, vp);
    fx.load(out, narrow);
    fx.high();
    assert_refused(&mut fx, add, Leave::Unsized, 4, "4-byte pointer");

    let mut fx = Fx::new();
    fx.tf.set_max_basetype_size(16);
    let f16 = fx.base(16, type_metatype::TYPE_FLOAT);
    assert_eq!(f16.get_size(), 16);
    let vp = fx.void_ptr();
    let p = fx.var(8, Rc::clone(&vp));
    let (add, out) = fx.add(p, 0x20, vp);
    fx.load(out, f16);
    fx.high();
    assert_refused(&mut fx, add, Leave::Unsized, 16, "16-byte float");

    for (size, meta) in [(1, type_metatype::TYPE_BOOL), (8, type_metatype::TYPE_PTR)] {
        let mut fx = Fx::new();
        let t = if meta == type_metatype::TYPE_PTR {
            let u = fx.base(4, type_metatype::TYPE_UINT);
            fx.ptr(u)
        } else {
            fx.base(size, meta)
        };
        let vp = fx.void_ptr();
        let p = fx.var(8, Rc::clone(&vp));
        let (add, out) = fx.add(p, 0x18, vp);
        fx.load(out, t);
        fx.high();
        rewritten(&mut fx, add, 0x18, size as u64, 0x18 / size as i64);
    }
}
