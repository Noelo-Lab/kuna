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
        Fx::with_index(cast_arith, true)
    }

    fn with_index(cast_arith: bool, cast_index: bool) -> Fx {
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
        arch.cast_index = cast_index;
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

// ---- castindex: a variable index and a byte-pointer difference ----

impl Fx {
    fn strat(&self) -> CastStrategyC {
        CastStrategyC::new(Rc::clone(&self.tf) as Rc<dyn TypeFactory>)
    }

    /// `out = <opc>(ins..)`, `out` an implied temporary typed `ty`.
    fn implied(&mut self, opc: OpCode, ins: &[VarnodeId], ty: Rc<Datatype>) -> (OpId, VarnodeId) {
        let op = self.op(ins.len() as int4, opc);
        for (k, &vn) in ins.iter().enumerate() {
            self.fd.op_set_input(op, vn, k as int4).unwrap();
        }
        let out = self.fd.new_unique_out(ty.get_size(), op).unwrap();
        self.fd.vn_update_type(out, ty);
        self.fd.vbank_mut().get_mut(out).unwrap().set_implied();
        (op, out)
    }

    fn konst(&mut self, size: int4, k: u64) -> VarnodeId {
        self.fd.new_constant(size, k)
    }

    /// `p + ext(i) * scale` read (or, with `store`, written) as a `t`; returns
    /// the add, the extension's output and the multiply.
    fn scaled(
        &mut self,
        p: VarnodeId,
        i: VarnodeId,
        ext: OpCode,
        scale: u64,
        t: Rc<Datatype>,
    ) -> (OpId, VarnodeId, OpId) {
        let long = self.base(8, type_metatype::TYPE_INT);
        let (_, x) = self.implied(ext, &[i], Rc::clone(&long));
        let k = self.konst(8, scale);
        let (mul, iv) = self.implied(OpCode::CPUI_INT_MULT, &[x, k], long);
        let tp = self.ptr(Rc::clone(&t));
        let (add, out) = self.implied(OpCode::CPUI_INT_ADD, &[p, iv], tp);
        self.load(out, t);
        (add, x, mul)
    }
}

fn index_rewritten(fx: &mut Fx, add: OpId, index: VarnodeId, elem: u64) -> VarnodeId {
    let strat = fx.strat();
    assert!(rewrite_index(&mut fx.fd, &strat, add), "the add was not rewritten");
    let o = fx.fd.obank().get(add).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_PTRADD);
    assert_eq!(fx.input(add, 1), index, "the index is the unscaled value");
    assert_eq!(fx.offset(fx.input(add, 2)), elem, "the element is the scale");
    fx.input(add, 0)
}

fn index_plan(fx: &mut Fx, add: OpId) -> Result<IndexPlan, Leave> {
    let strat = fx.strat();
    plan_index(&mut fx.fd, &strat, add)
}

/// `p + (long)i * sizeof(T)` read as a `T` is `((T *)p)[i]`: the multiply goes,
/// the extension stays as the index (and prints bare under the `PTRADD`, C's own
/// conversion of an `int` or `unsigned int` subscript), and the base is cast.
#[test]
fn a_scaled_index_becomes_a_subscript_of_that_element() {
    for (size, meta, ext, isize, imeta) in [
        (2, type_metatype::TYPE_INT, OpCode::CPUI_INT_SEXT, 4, type_metatype::TYPE_INT),
        (4, type_metatype::TYPE_UINT, OpCode::CPUI_INT_ZEXT, 4, type_metatype::TYPE_UINT),
        (8, type_metatype::TYPE_INT, OpCode::CPUI_INT_SEXT, 2, type_metatype::TYPE_INT),
        (8, type_metatype::TYPE_FLOAT, OpCode::CPUI_INT_ZEXT, 1, type_metatype::TYPE_UINT),
    ] {
        let mut fx = Fx::new();
        let vp = fx.void_ptr();
        let t = fx.base(size, meta);
        let it = fx.base(isize, imeta);
        let p = fx.var(8, vp);
        let i = fx.var(isize, it);
        let (add, x, mul) = fx.scaled(p, i, ext, size as u64, Rc::clone(&t));
        fx.high();
        let base = index_rewritten(&mut fx, add, x, size as u64);
        assert_eq!(cast_input_of(&fx, base), Some(p), "size {size}: the base is a cast of p");
        let tp = fx.ptr(t);
        assert!(Rc::ptr_eq(fx.fd.vbank().get(base).unwrap().get_type(), &tp));
        assert!(fx.fd.obank().get(mul).unwrap().is_dead(), "size {size}: the multiply is gone");
    }
}

/// A shift by `log2 sizeof(T)` scales the same way.
#[test]
fn a_shifted_index_becomes_a_subscript() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let t = fx.base(8, type_metatype::TYPE_UINT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, vp);
    let i = fx.var(8, Rc::clone(&long));
    let three = fx.konst(4, 3);
    let (_, iv) = fx.implied(OpCode::CPUI_INT_LEFT, &[i, three], long);
    let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, iv], tp);
    fx.load(out, t);
    fx.high();
    index_rewritten(&mut fx, add, i, 8);
}

/// A one-byte element takes the variable itself as the index, whatever it is.
#[test]
fn a_byte_element_is_indexed_by_the_variable_itself() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(Rc::clone(&c));
    let p = fx.var(8, vp);
    let i = fx.var(8, Rc::clone(&long));
    let six = fx.konst(8, 6);
    let (_, iv) = fx.implied(OpCode::CPUI_INT_MULT, &[i, six], long);
    let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, iv], cp);
    fx.load(out, c);
    fx.high();
    index_rewritten(&mut fx, add, iv, 1);
}

/// A scale that is not the element's size -- a 16-byte record read at 8 bytes,
/// a 12-byte stride read at 4 -- keeps the integer form, as does a byte offset
/// read at a wider element.
#[test]
fn a_scale_that_is_not_the_element_keeps_the_integer_form() {
    for (scale, size) in [(16u64, 8), (12, 4), (6, 2)] {
        let mut fx = Fx::new();
        let vp = fx.void_ptr();
        let int = fx.base(4, type_metatype::TYPE_INT);
        let t = fx.base(size, type_metatype::TYPE_INT);
        let p = fx.var(8, vp);
        let i = fx.var(4, int);
        let (add, _, _) = fx.scaled(p, i, OpCode::CPUI_INT_SEXT, scale, t);
        fx.high();
        assert!(matches!(index_plan(&mut fx, add), Err(Leave::ScaleMismatch)), "scale {scale}");
    }
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let lp = fx.ptr(Rc::clone(&long));
    let p = fx.var(8, vp);
    let off = fx.var(8, Rc::clone(&long));
    let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, off], lp);
    fx.load(out, long);
    fx.high();
    assert!(matches!(index_plan(&mut fx, add), Err(Leave::ScaleMismatch)));
}

/// The index must convert the way the integer form adds it: a pointer (the sum
/// of two pointers), a float and a value the pass already cast keep the form.
#[test]
fn an_index_that_is_not_a_plain_integer_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(Rc::clone(&c));
    let p = fx.var(8, Rc::clone(&vp));
    let q = fx.var(8, vp);
    let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, q], Rc::clone(&cp));
    fx.load(out, Rc::clone(&c));
    fx.high();
    assert!(matches!(index_plan(&mut fx, add), Err(Leave::TwoPointers)));

    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let dbl = fx.base(8, type_metatype::TYPE_FLOAT);
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(Rc::clone(&c));
    let p = fx.var(8, vp);
    let f = fx.var(8, dbl);
    let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, f], cp);
    fx.load(out, c);
    fx.high();
    assert!(matches!(index_plan(&mut fx, add), Err(Leave::IndexNotInteger)));

    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(Rc::clone(&c));
    let p = fx.var(8, Rc::clone(&vp));
    let q = fx.var(8, vp);
    let (_, cast) = fx.implied(OpCode::CPUI_CAST, &[q], long);
    let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, cast], cp);
    fx.load(out, c);
    fx.high();
    assert!(matches!(index_plan(&mut fx, add), Err(Leave::IndexNotInteger)));
}

/// A sum that is itself another pointer's subscript is an integer: the integer
/// form spells it with one cast, a subscript would need two.
#[test]
fn a_sum_used_as_a_subscript_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let lp = fx.ptr(Rc::clone(&long));
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(c);
    let p = fx.var(8, vp);
    let i = fx.var(8, Rc::clone(&long));
    let arr = fx.var(8, Rc::clone(&lp));
    let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, i], cp);
    let eight = fx.konst(8, 8);
    let (_, elt) = fx.implied(OpCode::CPUI_PTRADD, &[arr, out, eight], lp);
    fx.load(elt, long);
    fx.high();
    assert!(matches!(index_plan(&mut fx, add), Err(Leave::IntegerUse)));
}

/// A multiply the pass has not reached yet could still be retyped there.
#[test]
fn a_scale_after_the_add_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let vp = fx.void_ptr();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let t = fx.base(8, type_metatype::TYPE_INT);
    let tp = fx.ptr(Rc::clone(&t));
    let p = fx.var(8, vp);
    let i = fx.var(8, Rc::clone(&long));
    let add = fx.op(2, OpCode::CPUI_INT_ADD);
    let out = fx.fd.new_unique_out(8, add).unwrap();
    fx.fd.vn_update_type(out, tp);
    fx.fd.vbank_mut().get_mut(out).unwrap().set_implied();
    let eight = fx.konst(8, 8);
    let (_, iv) = fx.implied(OpCode::CPUI_INT_MULT, &[i, eight], long);
    fx.fd.op_set_input(add, p, 0).unwrap();
    fx.fd.op_set_input(add, iv, 1).unwrap();
    fx.load(out, t);
    fx.high();
    assert!(matches!(index_plan(&mut fx, add), Err(Leave::ScaleUnsettled)));
}

/// A value whose consumer needs its own cast anyway saves nothing: a sum typed
/// `unsigned int *` copied into a `long *` variable keeps the integer form,
/// and copied into an `unsigned int *` one is rewritten.
#[test]
fn a_value_use_is_rewritten_only_where_it_saves_a_cast() {
    for (same, want) in [(false, false), (true, true)] {
        let mut fx = Fx::new();
        let vp = fx.void_ptr();
        let long = fx.base(8, type_metatype::TYPE_INT);
        let u = fx.base(4, type_metatype::TYPE_UINT);
        let up = fx.ptr(Rc::clone(&u));
        let lp = fx.ptr(Rc::clone(&long));
        let p = fx.var(8, vp);
        let i = fx.var(8, Rc::clone(&long));
        let four = fx.konst(8, 4);
        let (_, iv) = fx.implied(OpCode::CPUI_INT_MULT, &[i, four], long);
        let (add, out) = fx.implied(OpCode::CPUI_INT_ADD, &[p, iv], Rc::clone(&up));
        fx.binary(OpCode::CPUI_COPY, out, out, if same { up } else { lp });
        fx.high();
        let strat = fx.strat();
        assert_eq!(rewrite_index(&mut fx.fd, &strat, add), want, "same type {same}");
    }
}

#[test]
fn the_cast_pass_rewrites_a_scaled_index_only_when_the_option_is_on() {
    for (on, want) in [(true, OpCode::CPUI_PTRADD), (false, OpCode::CPUI_INT_ADD)] {
        let mut fx = Fx::with_index(true, on);
        let vp = fx.void_ptr();
        let int = fx.base(4, type_metatype::TYPE_INT);
        let t = fx.base(4, type_metatype::TYPE_UINT);
        let p = fx.var(8, vp);
        let i = fx.var(4, int);
        let (add, _, _) = fx.scaled(p, i, OpCode::CPUI_INT_SEXT, 4, t);
        fx.high();
        let _ = fx.fd.action_set_casts();
        assert_eq!(fx.fd.obank().get(add).unwrap().code(), want, "castindex {on}");
    }
}

/// `sub = a - b` of two pointers typed `pa` and `pb`, `sub` an explicit `long`.
fn difference(fx: &mut Fx, pa: Rc<Datatype>, pb: Rc<Datatype>, out: Rc<Datatype>) -> (OpId, VarnodeId, VarnodeId) {
    let a = fx.var(8, pa);
    let b = fx.var(8, pb);
    let op = fx.binary(OpCode::CPUI_INT_SUB, a, b, out);
    (op, a, b)
}

/// Two `char *` (or two `unsigned char *`) subtract to their byte difference:
/// the difference keeps both operands, and the result is cast from `long`.
#[test]
fn a_difference_of_two_byte_pointers_keeps_its_operands() {
    for meta in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT] {
        for out_meta in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT] {
            let mut fx = Fx::new();
            let c = fx.base(1, meta);
            let cp = fx.ptr(c);
            let out = fx.base(8, out_meta);
            let (op, _, _) = difference(&mut fx, Rc::clone(&cp), cp, out);
            fx.high();
            let long = fx.base(8, type_metatype::TYPE_INT);
            let tok = pointer_difference(&mut fx.fd, op).expect("a byte-pointer difference");
            assert!(Rc::ptr_eq(&tok, &long), "the result is a ptrdiff_t");
        }
    }
}

/// Wider pointees subtract to an element count, not the byte count the integer
/// form computes; two different byte types, or `void *`, do not subtract in C;
/// and a result that is not a pointer-width integer is not `ptrdiff_t`.
#[test]
fn other_differences_keep_the_integer_form() {
    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let lp = fx.ptr(Rc::clone(&long));
    let (op, _, _) = difference(&mut fx, Rc::clone(&lp), lp, Rc::clone(&long));
    fx.high();
    assert!(matches!(pointer_difference(&mut fx.fd, op), Err(Leave::DiffPointee)));

    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let c = fx.base(1, type_metatype::TYPE_INT);
    let uc = fx.base(1, type_metatype::TYPE_UINT);
    let (pc, puc) = (fx.ptr(c), fx.ptr(uc));
    let (op, _, _) = difference(&mut fx, pc, puc, Rc::clone(&long));
    fx.high();
    assert!(matches!(pointer_difference(&mut fx.fd, op), Err(Leave::DiffPointee)));

    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let vp = fx.void_ptr();
    let (op, _, _) = difference(&mut fx, Rc::clone(&vp), vp, long);
    fx.high();
    assert!(matches!(pointer_difference(&mut fx.fd, op), Err(Leave::DiffPointee)));

    let mut fx = Fx::new();
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(c);
    let (op, _, _) = difference(&mut fx, Rc::clone(&cp), Rc::clone(&cp), cp);
    fx.high();
    assert!(matches!(pointer_difference(&mut fx.fd, op), Err(Leave::DiffResult)));

    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(c);
    let b = fx.var(8, Rc::clone(&cp));
    let k = fx.konst(8, 0x1000);
    let op = fx.binary(OpCode::CPUI_INT_SUB, b, k, long);
    fx.high();
    assert!(matches!(pointer_difference(&mut fx.fd, op), Err(Leave::DiffNotPointers)));
}

/// An operand that is an integer the pass cast to a pointer costs the integer
/// form nothing when its `long` input is read directly, so the pointer form
/// would only move the cast: kept.
#[test]
fn a_difference_that_saves_no_cast_keeps_the_integer_form() {
    let mut fx = Fx::new();
    let long = fx.base(8, type_metatype::TYPE_INT);
    let c = fx.base(1, type_metatype::TYPE_INT);
    let cp = fx.ptr(c);
    let n = fx.var(8, Rc::clone(&long));
    let (_, a) = fx.implied(OpCode::CPUI_CAST, &[n], Rc::clone(&cp));
    fx.binary(OpCode::CPUI_COPY, a, a, Rc::clone(&cp));
    let (_, b2) = fx.implied(OpCode::CPUI_CAST, &[n], Rc::clone(&cp));
    fx.binary(OpCode::CPUI_COPY, b2, b2, Rc::clone(&cp));
    let op = fx.binary(OpCode::CPUI_INT_SUB, a, b2, Rc::clone(&long));
    fx.high();
    assert!(matches!(pointer_difference(&mut fx.fd, op), Err(Leave::NoCastSaved)));
}

#[test]
fn the_cast_pass_leaves_a_byte_pointer_difference_uncast_only_when_the_option_is_on() {
    for on in [true, false] {
        let mut fx = Fx::with_index(true, on);
        let long = fx.base(8, type_metatype::TYPE_INT);
        let c = fx.base(1, type_metatype::TYPE_INT);
        let cp = fx.ptr(c);
        let (op, a, b) = difference(&mut fx, Rc::clone(&cp), cp, long);
        fx.high();
        let _ = fx.fd.action_set_casts();
        let uncast = fx.input(op, 0) == a && fx.input(op, 1) == b;
        assert_eq!(uncast, on, "castindex {on}");
    }
}
