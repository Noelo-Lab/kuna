//! Tests for the cast-insertion strategy (`cast.rs`).
//!
//! The lattice tests port the `cast_basic`/`cast_pointer`/`cast_enum`/
//! `cast_compare`/`cast_integertoken` decision matrices from
//! `decompiler/unittests/testtypes.cc`.  Each `testtypes.cc` assertion runs
//! `inst->getInputCast(op, 0, strategy)` (typeop, W9) which internally calls the
//! `CastStrategy` methods ported here; the tests below feed those methods the
//! same data-type pairs `getInputCast` would, so they pre-validate the W9
//! `getInputCast` closure (`// pre-validates …` notes name the C++ call site).
//!
//! Promotion tests (`cast_integertoken`, plus the `int_promotion_type`/
//! `local_extension_type` walks) drive the IR-walking methods through a mock
//! [`CastContext`].

use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use crate::dtype::{flags, type_metatype, DatatypeKind, TypeFactoryImpl};

// -- Data-type builders -----------------------------------------------------
//
// These mirror `parse("int4")` etc. from testtypes.cc.  `parse` returns the
// canonical factory type; here we build standalone `Rc<Datatype>`s with the
// fields `castStandard`/`is*Cast` read (metatype, size, flags, typedefImm,
// pointer ptrto/space/wordsize, code proto).

fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, meta))
}

fn int_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_INT)
}

fn uint_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_UINT)
}

fn bool_t() -> Rc<Datatype> {
    base(1, type_metatype::TYPE_BOOL)
}

fn float_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_FLOAT)
}

fn unknown_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_UNKNOWN)
}

fn void_t() -> Rc<Datatype> {
    let mut t = Datatype::new(0, type_metatype::TYPE_VOID);
    t.kind = DatatypeKind::Void;
    Rc::new(t)
}

/// `parse("char")` — a 1-byte signed char (TYPE_INT metatype + chartype flag).
fn char_t() -> Rc<Datatype> {
    let mut t = Datatype::new(1, type_metatype::TYPE_INT);
    t.flags |= flags::chartype;
    Rc::new(t)
}

/// `types->getBase(s, meta, "name")` — a typedef whose `typedefImm` is `imm`.
fn typedef_of(imm: Rc<Datatype>) -> Rc<Datatype> {
    let mut t = Datatype::new(imm.get_size(), imm.get_metatype());
    t.typedef_imm = Some(imm);
    Rc::new(t)
}

/// An enum data-type (`enum {…}`) — TYPE_INT/UINT metatype + enumtype flag.
fn enum_t(size: int4, meta: type_metatype) -> Rc<Datatype> {
    let mut t = Datatype::new(size, meta);
    t.flags |= flags::enumtype;
    Rc::new(t)
}

/// A struct data-type (`struct {…}`) — TYPE_STRUCT, size from its fields.
fn struct_t(size: int4) -> Rc<Datatype> {
    let mut t = Datatype::new(size, type_metatype::TYPE_STRUCT);
    t.kind = DatatypeKind::Struct { field: Vec::new(), bitfield: Vec::new() };
    Rc::new(t)
}

/// `types->getTypePointer(8, ptrto, wordsize)` — an 8-byte pointer.
fn ptr_to(ptrto: Rc<Datatype>, wordsize: uint4) -> Rc<Datatype> {
    let mut t = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    t.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize };
    Rc::new(t)
}

/// An array data-type (`T var[n]`).
fn array_of(arrayof: Rc<Datatype>, arraysize: int4) -> Rc<Datatype> {
    let size = arrayof.get_size() * arraysize;
    let mut t = Datatype::new_with_align(size, -1, type_metatype::TYPE_ARRAY);
    t.kind = DatatypeKind::Array { arrayof, arraysize };
    Rc::new(t)
}

/// A code data-type with no prototype (a generic code pointer target).
fn code_noproto() -> Rc<Datatype> {
    let mut t = Datatype::new(1, type_metatype::TYPE_CODE);
    t.kind = DatatypeKind::Code { proto: None };
    Rc::new(t)
}

/// A `CastStrategyC` with `promoteSize == 4` (the x86:LE:64 `int` size the
/// testtypes environment yields).
fn strategy_c() -> CastStrategyC {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.setup_sizes(Some(4), 8, 8);
    CastStrategyC::new(Rc::new(f))
}

fn strategy_java() -> CastStrategyJava {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.setup_sizes(Some(4), 8, 8);
    CastStrategyJava::new(Rc::new(f))
}

/// `castPrinted(opc, t1, t2)` for the non-comparison ops drives
/// `TypeOpCopy::getInputCast` (typeop.cc:398-404):
/// `castStandard(reqtype=t1, curtype=t2, care_uint_int=false, care_ptr_uint=true)`.
fn copy_cast_printed(s: &CastStrategyC, t1: &Rc<Datatype>, t2: &Rc<Datatype>) -> bool {
    s.cast_standard(t1, t2, false, true).is_some()
}

// -- cast_basic (testtypes.cc:121-135) --------------------------------------

#[test]
fn cast_basic() {
    let s = strategy_c();
    assert_eq!(s.promote_size(), 4);

    // pre-validates TypeOpCopy::getInputCast over castStandard(t1,t2,false,true)
    assert!(copy_cast_printed(&s, &int_t(4), &int_t(2))); // size differs -> cast
    assert!(!copy_cast_printed(&s, &int_t(4), &uint_t(4))); // !care_uint_int -> no cast
    assert!(copy_cast_printed(&s, &ptr_to(int_t(4), 1), &uint_t(8))); // ptr vs uint
    assert!(!copy_cast_printed(&s, &int_t(1), &bool_t())); // INT<-BOOL same size -> no cast
    assert!(!copy_cast_printed(&s, &unknown_t(4), &uint_t(4))); // reqbase UNKNOWN -> no cast
    assert!(!copy_cast_printed(&s, &int_t(4), &unknown_t(4))); // curbase UNKNOWN, INT, !care -> no cast
    assert!(copy_cast_printed(&s, &int_t(4), &float_t(4))); // INT<-FLOAT -> cast
    assert!(copy_cast_printed(&s, &array_of(int_t(1), 4), &uint_t(4))); // ARRAY size4 vs UINT4 -> cast

    // typedef int4 vs int4: stripping the typedef yields the shared canonical
    // int4 -> ptr-eq -> no cast.
    let canonical_int4 = int_t(4);
    let typedef_int = typedef_of(Rc::clone(&canonical_int4));
    assert!(!copy_cast_printed(&s, &typedef_int, &canonical_int4));

    // char vs int1: both TYPE_INT size 1 -> no cast.
    assert!(!copy_cast_printed(&s, &char_t(), &int_t(1)));
    // uint1 vs char: req UINT1, cur INT(char) size1, !care_uint_int -> meta INT
    // in the no-cast set -> no cast.
    assert!(!copy_cast_printed(&s, &uint_t(1), &char_t()));
}

// -- cast_pointer (testtypes.cc:137-155) ------------------------------------

#[test]
fn cast_pointer() {
    let s = strategy_c();

    // uint4* vs int4*: descend into ptrto (care_uint_int becomes true);
    // UINT vs INT with care -> cast.
    assert!(copy_cast_printed(&s, &ptr_to(uint_t(4), 1), &ptr_to(int_t(4), 1)));
    // void* vs float4*: descend; reqbase VOID -> no cast.
    assert!(!copy_cast_printed(&s, &ptr_to(void_t(), 1), &ptr_to(float_t(4), 1)));
    // int2* vs void*: descend; curbase VOID -> no cast.
    assert!(!copy_cast_printed(&s, &ptr_to(int_t(2), 1), &ptr_to(void_t(), 1)));

    // pointer-to-typedef (myint4) vs int4*: descend into the pointers, then strip
    // the typedef on the req side -> shared int4 -> ptr-eq -> no cast.
    // (testtypes: `typedefPtr = getTypePointer(8, typedefInt, 1)`.)
    let canonical_int4 = int_t(4);
    let typedef_int = typedef_of(Rc::clone(&canonical_int4));
    let ptr_to_typedef = ptr_to(typedef_int, 1);
    let plain_int_ptr = ptr_to(Rc::clone(&canonical_int4), 1);
    assert!(!copy_cast_printed(&s, &ptr_to_typedef, &plain_int_ptr));

    // bool** vs int1**: descend twice; BOOL vs INT same size with care -> cast.
    assert!(copy_cast_printed(
        &s,
        &ptr_to(ptr_to(bool_t(), 1), 1),
        &ptr_to(ptr_to(int_t(1), 1), 1)
    ));

    // structone* vs structtwo*: descend; two distinct structs same size ->
    // default arm -> cast.
    assert!(copy_cast_printed(&s, &ptr_to(struct_t(8), 1), &ptr_to(struct_t(8), 1)));

    // xunknown4* vs int4*: descend; reqbase UNKNOWN -> no cast.
    assert!(!copy_cast_printed(&s, &ptr_to(unknown_t(4), 1), &ptr_to(int_t(4), 1)));
    // uint4* vs xunknown4*: descend; UINT with care, curbase UNKNOWN + isptr ->
    // no cast.
    assert!(!copy_cast_printed(&s, &ptr_to(uint_t(4), 1), &ptr_to(unknown_t(4), 1)));
    // char* vs int1*: descend; both TYPE_INT (char is INT) size1 -> no cast.
    assert!(!copy_cast_printed(&s, &ptr_to(char_t(), 1), &ptr_to(int_t(1), 1)));
    // uint1* vs char*: descend; req UINT1, cur INT(char), care -> default -> cast.
    assert!(copy_cast_printed(&s, &ptr_to(uint_t(1), 1), &ptr_to(char_t(), 1)));

    // int4* vs named-pointer(myptrint4 -> int4): a named pointer is still a real
    // TypePointer to the same int4 (`getTypePointer(8, int4, 1, "myptrint4")`),
    // so descending yields shared int4 -> ptr-eq -> no cast.  The name only
    // affects printing, not the cast lattice.
    let inner2 = int_t(4);
    let req_ptr = ptr_to(Rc::clone(&inner2), 1);
    let named = ptr_to(Rc::clone(&inner2), 1);
    assert!(!copy_cast_printed(&s, &req_ptr, &named));
}

// -- cast_enum (testtypes.cc:157-163) ---------------------------------------

#[test]
fn cast_enum() {
    let s = strategy_c();
    // `enum enumone { ONE=1, TWO=2 }` is an *unsigned* enum in Ghidra
    // (TYPE_ENUM_UINT -> base metatype TYPE_UINT), size 8.
    let enum1 = enum_t(8, type_metatype::TYPE_UINT);

    // int8 vs enumone: req INT8, cur UINT(enum) size8, !care_uint_int -> meta
    // UINT in INT's no-cast set -> no cast.
    assert!(!copy_cast_printed(&s, &int_t(8), &enum1));
    // uint8* vs enumone*: descend (care becomes true); req UINT8, cur UINT(enum)
    // -> UINT no-cast arm -> no cast.
    assert!(!copy_cast_printed(&s, &ptr_to(uint_t(8), 1), &ptr_to(Rc::clone(&enum1), 1)));
    // enumone vs uint8: req UINT(enum), cur UINT8, !care -> UINT in UINT's
    // no-cast set -> no cast.
    assert!(!copy_cast_printed(&s, &enum1, &uint_t(8)));
}

// -- cast_compare (testtypes.cc:165-177) ------------------------------------
//
// These run TypeOpIntLess/Sless/Equal/NotEqual::getInputCast (typeop, W9).
// `getInputCast` for a comparison is (schematically, e.g. TypeOpIntLess,
// typeop.cc:1077-): pick reqtype = the lower-typeOrder of the two inputs, then
//   if checkIntPromotionForCompare(op,slot) return reqtype (cast)
//   else castStandard(reqtype, in[slot], false, false)
// where INT_LESS additionally pre-checks: if both inputs metatype != TYPE_UINT,
// the comparison itself forces unsignedness (a cast).  We port the *castStandard*
// half here (the inputs are explicit non-promoting variables, so
// checkIntPromotionForCompare is false — verified by the promotion tests).

#[test]
fn cast_compare_signedness() {
    let s = strategy_c();
    // INT_LESS wants unsigned operands: comparing two int4 needs a cast, two
    // uint4 does not.  This is castStandard(uint-required, curtype, false, false)
    // with the requirement that LESS forces UINT.  Model the decision directly:
    // for INT_LESS the required type is the UINT view; an INT input differs in
    // sign (care_uint_int is false here, so signedness alone does NOT force a
    // cast) — the actual cast in testtypes comes from getInputCast forcing the
    // unsigned requirement.  We assert the castStandard building block: a UINT
    // requirement against an INT current with !care_uint_int does NOT cast,
    // confirming the cast must originate in getInputCast's promotion forcing.
    assert!(s.cast_standard(&uint_t(4), &int_t(4), false, false).is_none());

    // INT_SLESS wants signed operands.  castStandard(INT, UINT, false, false):
    // !care_uint_int -> UINT in INT no-cast set -> no cast at the lattice level.
    assert!(s.cast_standard(&int_t(4), &uint_t(4), false, false).is_none());

    // INT_EQUAL uint8 vs int4*: same size 8?  int4* is size 8, uint8 size 8.
    // reqtype = lower typeOrder; castStandard(uint8, int4*, false, false): req
    // UINT, cur PTR, care_ptr_uint=false -> the (!care_ptr_uint && PTR) arm ->
    // no cast.  But testtypes asserts a cast for slot... the asymmetry is in
    // which slot getInputCast queries.  At the lattice level:
    //   castStandard(int4*, uint8, false, false): req PTR, cur UINT8 same size,
    //   PTR default arm -> cast.
    assert!(s.cast_standard(&ptr_to(int_t(4), 1), &uint_t(8), false, false).is_some());
    //   castStandard(uint8, int4*, false, false): req UINT, !care_ptr_uint & PTR
    //   -> no cast.
    assert!(s.cast_standard(&uint_t(8), &ptr_to(int_t(4), 1), false, false).is_none());

    // INT_EQUAL int4 vs float4: different metatype same size -> reqtype lower
    // typeOrder is float; castStandard(int4, float4, false, false): req INT, cur
    // FLOAT not in no-cast set -> default -> cast.
    assert!(s.cast_standard(&int_t(4), &float_t(4), false, false).is_some());
}

// -- cast_integertoken (testtypes.cc:204-213) -------------------------------
//
// `longPrinted(opc, t1, val)` builds `op = opc(const(t1,val), unique(int4))` and
// returns `markExplicitLongSize(op, 0)`.  Drive it through the mock context.

#[test]
fn cast_integertoken() {
    let s = strategy_c();
    // INT_LEFT, int8, 10 -> long (msb of 10 < 32 bits) -> true
    assert!(long_printed(&s, OpCode::CPUI_INT_LEFT, type_metatype::TYPE_INT, 8, 10));
    // INT_LEFT, int8, 0x100000000 -> bit 32 >= 32 -> naturally long -> false
    assert!(!long_printed(&s, OpCode::CPUI_INT_LEFT, type_metatype::TYPE_INT, 8, 0x100000000));
    // INT_SRIGHT, int8, -3 -> negative INT: negate -> 3, msb < 31 -> true
    assert!(long_printed(
        &s,
        OpCode::CPUI_INT_SRIGHT,
        type_metatype::TYPE_INT,
        8,
        (-3i64) as u64
    ));
    // INT_SRIGHT, int8, 0xffffffff7fffffff -> signed-negative INT.  uintb_negate
    // is ONE'S complement (~in & mask, address.cc:894): ~0xffffffff7fffffff & m8
    // = 0x80000000, mostsigbit_set = 31, guard `31 >= promoteSize*8-1 == 31` ->
    // return false (NOT printed long).
    assert!(!long_printed(
        &s,
        OpCode::CPUI_INT_SRIGHT,
        type_metatype::TYPE_INT,
        8,
        0xffffffff7fffffff
    ));
    // INT_SRIGHT, int8, 0xffffffff80000000 -> signed-negative INT.  One's
    // complement: ~0xffffffff80000000 & m8 = 0x7fffffff, mostsigbit_set = 30,
    // guard `30 >= 31` is false -> printed long (true).
    assert!(long_printed(
        &s,
        OpCode::CPUI_INT_SRIGHT,
        type_metatype::TYPE_INT,
        8,
        0xffffffff80000000
    ));
    // INT_RIGHT (uint8), 0xffffffff -> UINT path, msb 31 < 32 -> true
    assert!(long_printed(
        &s,
        OpCode::CPUI_INT_RIGHT,
        type_metatype::TYPE_UINT,
        8,
        0xffffffff
    ));
    // INT_RIGHT (uint8), 0x100000000 -> msb 32 >= 32 -> false
    assert!(!long_printed(
        &s,
        OpCode::CPUI_INT_RIGHT,
        type_metatype::TYPE_UINT,
        8,
        0x100000000
    ));
}

// -- castStandard TYPE_CODE branch (cast.cc:378-386) ------------------------

#[test]
fn cast_code_generic_no_cast() {
    let s = strategy_c();
    // Two generic code pointers (no prototype) of the same size -> the TYPE_CODE
    // arm returns no cast (don't cast between function/generic code pointers).
    let a = code_noproto();
    let b = code_noproto();
    assert!(s.cast_standard(&a, &b, false, false).is_none());
}

// -- mark_explicit_unsigned (cast.cc:38-71) ---------------------------------

#[test]
fn mark_explicit_unsigned_basic() {
    let s = strategy_c();
    // INT_OR(const uint4, var int4): INT_OR inherits sign, not first-param-only,
    // const slot 0 is UINT (markable), other side INT (won't force) -> marked.
    let mut ctx = MockCtx::new();
    let cst = ctx.add_const(4, 5, uint_t(4));
    let other = ctx.add_var(4, int_t(4));
    let out = ctx.add_var(4, int_t(4));
    let op = ctx.add_op(OpCode::CPUI_INT_OR, vec![cst, other], Some(out));
    assert!(s.mark_explicit_unsigned(&mut ctx, op, 0));
    assert!(ctx.unsigned_marked(cst));

    // If the other side is UINT, the operation forces unsigned anyway -> not marked.
    let mut ctx2 = MockCtx::new();
    let cst2 = ctx2.add_const(4, 5, uint_t(4));
    let other2 = ctx2.add_var(4, uint_t(4));
    let op2 = ctx2.add_op(OpCode::CPUI_INT_OR, vec![cst2, other2], None);
    assert!(!s.mark_explicit_unsigned(&mut ctx2, op2, 0));
}

// -- int_promotion_type / local_extension_type (cast.cc:140-247) ------------

#[test]
fn int_promotion_no_promotion_for_full_size() {
    let s = strategy_c();
    let mut ctx = MockCtx::new();
    // size >= promoteSize(4) -> NO_PROMOTION regardless of contents.
    let v = ctx.add_var(4, int_t(4));
    assert_eq!(s.int_promotion_type(&ctx, v), IntPromotionCode::NO_PROMOTION);
}

#[test]
fn int_promotion_explicit_small_is_no_promotion() {
    let s = strategy_c();
    let mut ctx = MockCtx::new();
    let v = ctx.add_var(2, int_t(2));
    ctx.set_explicit(v, true);
    // explicit small non-constant -> NO_PROMOTION
    assert_eq!(s.int_promotion_type(&ctx, v), IntPromotionCode::NO_PROMOTION);
}

#[test]
fn local_extension_constant_high_bit_zero_is_either() {
    let s = strategy_c();
    let mut ctx = MockCtx::new();
    // small UINT constant, high bit clear -> EITHER_EXTENSION
    let c = ctx.add_const(2, 5, uint_t(2));
    assert_eq!(
        s.local_extension_type(&ctx, c, None),
        IntPromotionCode::EITHER_EXTENSION
    );
    // small INT constant, high bit set -> natural (SIGNED)
    let c2 = ctx.add_const(2, 0x8000, int_t(2));
    assert_eq!(
        s.local_extension_type(&ctx, c2, None),
        IntPromotionCode::SIGNED_EXTENSION
    );
}

// -- Java cast surface (cast.cc:471-544) ------------------------------------

#[test]
fn java_cast_standard_pointers_never_cast() {
    let s = strategy_java();
    // Any pointer involved -> no cast (object refs use explicit cast ops).
    assert!(s
        .cast_standard(&ptr_to(int_t(4), 1), &ptr_to(uint_t(4), 1), true, true)
        .is_none());
    assert!(s.cast_standard(&int_t(4), &ptr_to(int_t(4), 1), true, true).is_none());
    // Size change -> always cast.
    assert!(s.cast_standard(&int_t(4), &int_t(2), false, true).is_some());
}

#[test]
fn java_is_zext_cast() {
    let s = strategy_java();
    // int1 -> int4 (byte->int): INT size1 -> not zext (byte) -> false
    assert!(!s.is_zext_cast(&int_t(4), &int_t(1)));
    // uint1 -> uint4: UINT size1 (<4, not the short/byte exclusions) -> zext
    assert!(s.is_zext_cast(&uint_t(4), &uint_t(1)));
    // size 2 non-char -> not zext (short) -> false
    assert!(!s.is_zext_cast(&int_t(4), &uint_t(2)));
    // size >=4 -> not zext -> false
    assert!(!s.is_zext_cast(&int_t(8), &uint_t(4)));
    // non-integer input -> false
    assert!(!s.is_zext_cast(&int_t(4), &float_t(2)));
}

#[test]
fn c_is_sext_zext_cast() {
    let s = strategy_c();
    // SEXT cast: input must be signed (INT or BOOL), output INT/UINT.
    assert!(s.is_sext_cast(&int_t(4), &int_t(2)));
    assert!(s.is_sext_cast(&uint_t(4), &bool_t()));
    assert!(!s.is_sext_cast(&int_t(4), &uint_t(2))); // input UINT -> not sext
    assert!(!s.is_sext_cast(&float_t(4), &int_t(2))); // output FLOAT -> not cast
    // ZEXT cast: input must be unsigned (UINT or BOOL).
    assert!(s.is_zext_cast(&int_t(4), &uint_t(2)));
    assert!(s.is_zext_cast(&uint_t(4), &bool_t()));
    assert!(!s.is_zext_cast(&int_t(4), &int_t(2))); // input INT -> not zext
}

#[test]
fn c_is_subpiece_cast() {
    let s = strategy_c();
    // offset != 0 -> never a cast.
    assert!(!s.is_subpiece_cast(&int_t(2), &int_t(4), 1));
    // INT<-INT, offset 0 -> cast (truncation shows up).
    assert!(s.is_subpiece_cast(&int_t(2), &int_t(4), 0));
    // float output from int input -> cast.
    assert!(s.is_subpiece_cast(&float_t(2), &int_t(4), 0));
    // far pointer (size 8) -> near pointer (size 4): out smaller -> cast.
    let near = {
        let mut t = Datatype::new_with_align(4, -1, type_metatype::TYPE_PTR);
        t.kind = DatatypeKind::Pointer {
            ptrto: int_t(4),
            spaceid: None,
            truncate: None,
            wordsize: 1,
        };
        Rc::new(t)
    };
    assert!(s.is_subpiece_cast(&near, &ptr_to(int_t(4), 1), 0));
    // ptr input, non-int/uint output -> not a cast.
    assert!(!s.is_subpiece_cast(&float_t(8), &ptr_to(int_t(4), 1), 0));
}

// -- Mock CastContext -------------------------------------------------------

struct MockVn {
    size: int4,
    offset: uintb,
    ty: Rc<Datatype>,
    constant: bool,
    explicit: bool,
    written: bool,
    def: Option<OpRef>,
    lone: Option<OpRef>,
    bool_output_def: bool,
}

struct MockOp {
    code: OpCode,
    ins: Vec<VnRef>,
    out: Option<VnRef>,
}

struct MockCtx {
    vns: Vec<MockVn>,
    ops: Vec<MockOp>,
    unsigned: RefCell<Vec<bool>>,
    long: RefCell<Vec<bool>>,
}

impl MockCtx {
    fn new() -> MockCtx {
        MockCtx {
            vns: Vec::new(),
            ops: Vec::new(),
            unsigned: RefCell::new(Vec::new()),
            long: RefCell::new(Vec::new()),
        }
    }

    fn add_var(&mut self, size: int4, ty: Rc<Datatype>) -> VnRef {
        let id = self.vns.len();
        self.vns.push(MockVn {
            size,
            offset: 0,
            ty,
            constant: false,
            explicit: false,
            written: false,
            def: None,
            lone: None,
            bool_output_def: false,
        });
        self.unsigned.borrow_mut().push(false);
        self.long.borrow_mut().push(false);
        VnRef(id)
    }

    fn add_const(&mut self, size: int4, offset: uintb, ty: Rc<Datatype>) -> VnRef {
        let v = self.add_var(size, ty);
        self.vns[v.0].constant = true;
        self.vns[v.0].offset = offset;
        v
    }

    fn set_explicit(&mut self, v: VnRef, val: bool) {
        self.vns[v.0].explicit = val;
    }

    fn add_op(&mut self, code: OpCode, ins: Vec<VnRef>, out: Option<VnRef>) -> OpRef {
        let id = self.ops.len();
        let op = OpRef(id);
        if let Some(o) = out {
            self.vns[o.0].written = true;
            self.vns[o.0].def = Some(op);
        }
        self.ops.push(MockOp { code, ins, out });
        op
    }

    fn unsigned_marked(&self, v: VnRef) -> bool {
        self.unsigned.borrow()[v.0]
    }
}

impl CastContext for MockCtx {
    fn op_code(&self, op: OpRef) -> OpCode {
        self.ops[op.0].code
    }
    fn op_num_input(&self, op: OpRef) -> int4 {
        self.ops[op.0].ins.len() as int4
    }
    fn op_in(&self, op: OpRef, slot: int4) -> VnRef {
        self.ops[op.0].ins[slot as usize]
    }
    fn op_out(&self, op: OpRef) -> Option<VnRef> {
        self.ops[op.0].out
    }
    fn op_slot(&self, op: OpRef, vn: VnRef) -> int4 {
        self.ops[op.0].ins.iter().position(|v| *v == vn).unwrap() as int4
    }
    fn vn_is_constant(&self, vn: VnRef) -> bool {
        self.vns[vn.0].constant
    }
    fn vn_is_explicit(&self, vn: VnRef) -> bool {
        self.vns[vn.0].explicit
    }
    fn vn_is_written(&self, vn: VnRef) -> bool {
        self.vns[vn.0].written
    }
    fn vn_size(&self, vn: VnRef) -> int4 {
        self.vns[vn.0].size
    }
    fn vn_offset(&self, vn: VnRef) -> uintb {
        self.vns[vn.0].offset
    }
    fn vn_def(&self, vn: VnRef) -> Option<OpRef> {
        self.vns[vn.0].def
    }
    fn vn_lone_descend(&self, vn: VnRef) -> Option<OpRef> {
        self.vns[vn.0].lone
    }
    fn vn_high_type(&self, vn: VnRef) -> Rc<Datatype> {
        Rc::clone(&self.vns[vn.0].ty)
    }
    fn vn_high_type_read_facing(&self, vn: VnRef, _op: OpRef) -> Rc<Datatype> {
        Rc::clone(&self.vns[vn.0].ty)
    }
    fn op_inherits_sign(&self, op: OpRef) -> bool {
        crate::typeop::type_op_info(self.ops[op.0].code).inherits_sign()
    }
    fn op_inherits_sign_first_param_only(&self, op: OpRef) -> bool {
        crate::typeop::type_op_info(self.ops[op.0].code).inherits_sign_first_param_only()
    }
    fn op_is_shift_op(&self, op: OpRef) -> bool {
        crate::typeop::type_op_info(self.ops[op.0].code).is_shift_op()
    }
    fn op_is_bool_output(&self, op: OpRef) -> bool {
        // Mirror the def's bool-output flag if set, else use the op-code's
        // booloutput property (PcodeOp::booloutput, op.rs `pcodeop_flags`).
        if let Some(out) = self.ops[op.0].out {
            if self.vns[out.0].bool_output_def {
                return true;
            }
        }
        (crate::typeop::type_op_info(self.ops[op.0].code).get_flags()
            & crate::op::pcodeop_flags::booloutput)
            != 0
    }
    fn op_is_call(&self, op: OpRef) -> bool {
        matches!(
            self.ops[op.0].code,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
        )
    }
    fn vn_set_unsigned_print(&mut self, vn: VnRef) {
        self.unsigned.borrow_mut()[vn.0] = true;
    }
    fn vn_set_long_print(&mut self, vn: VnRef) {
        self.long.borrow_mut()[vn.0] = true;
    }
}

/// `longPrinted(opc, meta, size, val)` (testtypes.cc:104-119): build
/// `op = opc(const(meta-typed, size, val), unique(int4))` and run
/// `markExplicitLongSize(op, 0)`.
fn long_printed(
    s: &CastStrategyC,
    opc: OpCode,
    meta: type_metatype,
    size: int4,
    val: uintb,
) -> bool {
    let mut ctx = MockCtx::new();
    let ct = base(size, meta);
    let cst = ctx.add_const(size, val, ct);
    let sa = ctx.add_var(4, int_t(4));
    let out = ctx.add_var(size, int_t(size));
    let op = ctx.add_op(opc, vec![cst, sa], Some(out));
    s.mark_explicit_long_size(&mut ctx, op, 0)
}
