//! Adversarial verification tests for item `w6-s4-modelrules`
//! (`decompiler/cpp/modelrules.{cc,hh}` — `PrimitiveExtractor`, the
//! `DatatypeFilter` / `QualifierFilter` predicates, the `AssignAction` family,
//! and `ModelRule`).
//!
//! Written by the INDEPENDENT verifier.  These target the spots the hunt list
//! flagged as most fragile for this item:
//!
//!   - **`SizeRestrictedFilter::initFromSizeList`** (`istringstream` loop,
//!     modelrules.cc:277-301): comma/whitespace skipping, the `val <= 0` throw
//!     ("Bad filter size"), and the derived `minSize = *sizes.begin()` /
//!     `maxSize = *--sizes.end()` from the `set<int4>` order.  (parse_size_list)
//!   - **`AssignAction::justifyPieces`** (modelrules.cc:683-694): the
//!     `addOffset = isBigEndian ^ consumeMostSig ^ justifyRight` parity and the
//!     `size -= offset` truncation, reached for an odd-sized data-type in
//!     `MultiSlotAssign::assignAddress`.  Wrap-prone (`uint4 -= int4`).
//!   - **`HomogeneousAggregate::filter`** (modelrules.cc:412-430): the
//!     `primitives.get(i).dt != base` pointer-identity comparison, ported as
//!     `Rc::ptr_eq` — must accept an array of identical primitives and reject a
//!     struct of distinct primitives.  TypeFactory cache determinism.
//!   - **`VarargsFilter`** (modelrules.cc:502-508, .hh:224): the
//!     `firstPos = 0x80000000` (INT_MIN) decode default, the
//!     `firstVarArgSlot < 0` short-circuit, and inclusive `[firstPos,lastPos]`.
//!
//! Everything is built through the public crate API (no porter internals).

use std::rc::Rc;

use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_class, type_metatype, Datatype, DatatypeKind, TypeField, TypeFactory};
use kuna_decomp::fspec::{
    AssignActionResponse, ParamEntry, ParamListKind, ParamListStandard, ParameterPieces,
    PrototypePieces,
};
use kuna_decomp::modelrules::{
    DatatypeFilter, ModelRule, QualifierFilter,
    ELEM_CONSUME, ELEM_CONVERT_TO_PTR, ELEM_DATATYPE, ELEM_DATATYPE_AT, ELEM_EXTRA_STACK,
    ELEM_GOTO_STACK, ELEM_HIDDEN_RETURN, ELEM_JOIN, ELEM_JOIN_DUAL_CLASS, ELEM_JOIN_PER_PRIMITIVE,
    ELEM_POSITION, ELEM_RULE, ELEM_VARARGS, ELEM_CONSUME_EXTRA, ELEM_CONSUME_REMAINING,
    ATTRIB_A, ATTRIB_AFTER_BYTES, ATTRIB_AFTER_STORAGE, ATTRIB_B, ATTRIB_FILL_ALTERNATE,
    ATTRIB_FIRST, ATTRIB_LAST, ATTRIB_MATCHSIZE, ATTRIB_MAXSIZE, ATTRIB_MAX_PRIMITIVES,
    ATTRIB_MINSIZE, ATTRIB_REVERSEJUSTIFY, ATTRIB_REVERSESIGNIF, ATTRIB_SIZES, ATTRIB_STRATEGY,
    ATTRIB_VOIDLOCK,
};

// ---------------------------------------------------------------------------
// builders (mirroring the in-crate test helpers, public-API only)
// ---------------------------------------------------------------------------

fn reg_space_le() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 0, 0))
}
fn reg_space_be() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", true, 4, 1, 3, 0, 0, 0))
}
fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 4, 1, 5, 0, 0, 0))
}

fn excl_entry(
    grp: int4,
    class: type_class,
    space: &Rc<AddrSpace>,
    base: u64,
    size: int4,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    ParamEntry::seed(grp, class, Rc::clone(space), base, size, 1, 0, 0, true, false, prev, mgr)
        .expect("seed exclusion entry")
}

fn stack_entry(
    grp: int4,
    space: &Rc<AddrSpace>,
    base: u64,
    size: int4,
    align: int4,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        base,
        size,
        1,
        align,
        0,
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed stack entry")
}

fn dt_int(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, type_metatype::TYPE_INT))
}
fn dt_float(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, type_metatype::TYPE_FLOAT))
}

fn dt_struct(size: int4, align: int4, fields: &[(int4, Rc<Datatype>)]) -> Rc<Datatype> {
    let mut d = Datatype::new_with_align(size, align, type_metatype::TYPE_STRUCT);
    let field: Vec<TypeField> = fields
        .iter()
        .enumerate()
        .map(|(i, (off, ft))| TypeField::new(i as int4, *off, format!("f{i}"), Rc::clone(ft)))
        .collect();
    d.kind = DatatypeKind::Struct { field, bitfield: Vec::new() };
    Rc::new(d)
}

fn dt_array(elem: Rc<Datatype>, nels: int4) -> Rc<Datatype> {
    let elsize = elem.get_align_size();
    let mut d =
        Datatype::new_with_align(elsize * nels, elem.get_alignment(), type_metatype::TYPE_ARRAY);
    d.kind = DatatypeKind::Array { arrayof: elem, arraysize: nels };
    Rc::new(d)
}

fn modelrules_registry() -> IdRegistry {
    let mut r = IdRegistry::with_base_ids();
    for e in [
        &ELEM_DATATYPE,
        &ELEM_CONSUME,
        &ELEM_CONSUME_EXTRA,
        &ELEM_CONVERT_TO_PTR,
        &ELEM_GOTO_STACK,
        &ELEM_JOIN,
        &ELEM_DATATYPE_AT,
        &ELEM_POSITION,
        &ELEM_VARARGS,
        &ELEM_HIDDEN_RETURN,
        &ELEM_JOIN_PER_PRIMITIVE,
        &ELEM_JOIN_DUAL_CLASS,
        &ELEM_EXTRA_STACK,
        &ELEM_CONSUME_REMAINING,
        &ELEM_RULE,
    ] {
        r.register_element(e);
    }
    for a in [
        &ATTRIB_SIZES,
        &ATTRIB_MAX_PRIMITIVES,
        &ATTRIB_REVERSESIGNIF,
        &ATTRIB_MATCHSIZE,
        &ATTRIB_AFTER_BYTES,
        &ATTRIB_AFTER_STORAGE,
        &ATTRIB_FILL_ALTERNATE,
        &ATTRIB_MAXSIZE,
        &ATTRIB_MINSIZE,
        &ATTRIB_STRATEGY,
        &ATTRIB_VOIDLOCK,
        &ATTRIB_REVERSEJUSTIFY,
        &ATTRIB_FIRST,
        &ATTRIB_LAST,
        &ATTRIB_A,
        &ATTRIB_B,
    ] {
        r.register_attribute(a);
    }
    r
}

fn decode_rule(
    xml: &str,
    resource: &ParamListStandard,
    mgr: &AddrSpaceManager,
) -> kuna_base::error::KunaResult<ModelRule> {
    let registry = modelrules_registry();
    let mut decoder = XmlDecode::new(mgr, &registry);
    decoder.ingest_stream(xml.as_bytes())?;
    ModelRule::decode(&mut decoder, resource)
}

/// Decode just a `<datatype .../>` filter (so we can probe initFromSizeList).
fn decode_filter(xml: &str, mgr: &AddrSpaceManager) -> kuna_base::error::KunaResult<DatatypeFilter> {
    let registry = modelrules_registry();
    let mut decoder = XmlDecode::new(mgr, &registry);
    decoder.ingest_stream(xml.as_bytes())?;
    DatatypeFilter::decode_filter(&mut decoder)
}

// A minimal TypeFactory that only services get_type_pointer (for ConvertToPointer),
// panicking on everything else.  Not used by these tests, but assign_address needs
// the trait object — so build a never-reached panic factory.
struct PanicFactory;
#[allow(unused_variables)]
impl TypeFactory for PanicFactory {
    fn get_size_of_int(&self) -> int4 { panic!() }
    fn get_size_of_long(&self) -> int4 { panic!() }
    fn get_size_of_char(&self) -> int4 { panic!() }
    fn get_size_of_wchar(&self) -> int4 { panic!() }
    fn get_size_of_pointer(&self) -> int4 { panic!() }
    fn get_size_of_alt_pointer(&self) -> int4 { panic!() }
    fn get_alignment(&self, size: u32) -> kuna_base::error::KunaResult<int4> { panic!() }
    fn get_primitive_align_size(&self, size: u32) -> kuna_base::error::KunaResult<int4> { panic!() }
    fn get_type_void(&self) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_base_no_char(&self, s: int4, m: type_metatype) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_base(&self, s: int4, m: type_metatype) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_base_named(&self, s: int4, m: type_metatype, n: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_char(&self, s: int4) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_code(&self) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_pointer_strip_array(&self, s: int4, pt: Rc<Datatype>, ws: u32) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_pointer(&self, s: int4, pt: Rc<Datatype>, ws: u32) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_pointer_named(&self, s: int4, pt: Rc<Datatype>, ws: u32, n: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn resize_pointer(&self, ptr: Rc<Datatype>, new_size: int4) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_pointer_rel(&self, parent_ptr: Rc<Datatype>, ptr_to: Rc<Datatype>, off: int4) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_pointer_rel_full(&self, sz: int4, parent: Rc<Datatype>, ptr_to: Rc<Datatype>, ws: int4, off: int4, nm: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_pointer_with_space(&self, ptr_to: Rc<Datatype>, spc: Rc<AddrSpace>, nm: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_array(&self, as_: int4, ao: Rc<Datatype>) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_struct(&self, n: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_partial_struct(&self, contain: Rc<Datatype>, off: int4, sz: int4) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_union(&self, n: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_partial_union(&self, contain: Rc<Datatype>, off: int4, sz: int4) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_partial_enum(&self, contain: Rc<Datatype>, off: int4, sz: int4) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_enum(&self, n: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_type_spacebase(&self, id: Rc<AddrSpace>, addr: &kuna_base::address::Address) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn resize_integer(&self, ct: Rc<Datatype>, new_size: int4) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
    fn get_exact_piece(&self, ct: Rc<Datatype>, offset: int4, size: int4) -> kuna_base::error::KunaResult<Option<Rc<Datatype>>> { panic!() }
    fn find_by_name(&self, n: &str) -> kuna_base::error::KunaResult<Option<Rc<Datatype>>> { panic!() }
    fn concretize(&self, ct: Rc<Datatype>) -> kuna_base::error::KunaResult<Rc<Datatype>> { panic!() }
}

// ===========================================================================
// 1. SizeRestrictedFilter::initFromSizeList (parse_size_list) parity
// ===========================================================================

#[test]
fn v_w6_s4_modelrules_size_list_parse_parity() {
    let mgr = AddrSpaceManager::new();

    // "8, 2, 4" -> sizes {2,4,8} (set order), minSize=2, maxSize=8.
    // The filter then accepts exactly 2/4/8-byte types and nothing else.
    let f = decode_filter(r#"<datatype name="any" sizes="8, 2, 4"/>"#, &mgr).expect("sizes decode");
    assert!(f.filter(&dt_int(2)), "2 in {{2,4,8}}");
    assert!(f.filter(&dt_int(4)), "4 in {{2,4,8}}");
    assert!(f.filter(&dt_int(8)), "8 in {{2,4,8}}");
    assert!(!f.filter(&dt_int(1)), "1 below min");
    assert!(!f.filter(&dt_int(3)), "3 not enumerated even though min<=3<=max");
    assert!(!f.filter(&dt_int(16)), "16 above max");

    // Space-separated (no commas) parses identically (C++ `s>>ws` between ints).
    let f2 = decode_filter(r#"<datatype name="any" sizes="4 8"/>"#, &mgr).expect("space decode");
    assert!(f2.filter(&dt_int(4)) && f2.filter(&dt_int(8)) && !f2.filter(&dt_int(6)));

    // A non-positive value throws "Bad filter size" (val <= 0 in the C++ loop).
    assert!(decode_filter(r#"<datatype name="any" sizes="4 0 8"/>"#, &mgr).is_err());
    assert!(decode_filter(r#"<datatype name="any" sizes="-1"/>"#, &mgr).is_err());

    // Mixing "sizes" with "minsize"/"maxsize" is rejected (the decode guard).
    assert!(decode_filter(r#"<datatype name="any" sizes="4" minsize="2"/>"#, &mgr).is_err());
}

// ===========================================================================
// 2. justifyPieces parity via MultiSlotAssign on an odd-sized data-type
// ===========================================================================
//
// Two 4-byte GENERAL registers + a stack fallback.  A 3-byte type consumes one
// 4-byte register; sizeLeft = 3 - 4 = -1 (< 0), so justifyPieces truncates the
// single piece by 1 byte.  We exercise BOTH endianness/justify parities and
// confirm the piece's size/offset land where addOffset = LE^mostSig^justRight
// dictates (and that the uint4 `size -= 1` does not panic / wrap).

fn two_reg_stack_model(
    mgr: &AddrSpaceManager,
    reg: &Rc<AddrSpace>,
    stk: &Rc<AddrSpace>,
) -> ParamListStandard {
    let mut m = ParamListStandard::new(ParamListKind::Standard);
    m.push_entry(excl_entry(0, type_class::TYPECLASS_GENERAL, reg, 0x10, 4, &[], mgr));
    let e1 = excl_entry(1, type_class::TYPECLASS_GENERAL, reg, 0x20, 4, m.get_entry(), mgr);
    m.push_entry(e1);
    let se = stack_entry(2, stk, 0, 0x1000, 4, m.get_entry(), mgr);
    m.push_entry(se);
    m.finish_decode();
    m
}

#[test]
fn v_w6_s4_modelrules_justify_pieces_little_endian() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stk = stack_space();
    let model = two_reg_stack_model(&mgr, &reg, &stk);
    // Little-endian <join> default: consumeMostSig=false, justifyRight=false.
    let rule = decode_rule(r#"<rule><datatype name="any"/><join/></rule>"#, &model, &mgr)
        .expect("decode join");
    let mut status = vec![0i32; 3];
    let mut res = ParameterPieces::default();
    let r = rule
        .assign_address(&dt_int(3), &PrototypePieces::default().with_dummy(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .expect("assign");
    assert_eq!(r, AssignActionResponse::success);
    // One 4-byte register consumed; the single piece truncated to 3 bytes.
    // addOffset = LE(false) ^ mostSig(false) ^ justRight(false) = false -> offset stays 0x10.
    assert_eq!(status[0], -1);
    assert_eq!(res.addr.get_offset(), 0x10, "LE/justify-left keeps the low offset");
}

#[test]
fn v_w6_s4_modelrules_justify_pieces_big_endian() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_be();
    let stk = stack_space();
    let model = two_reg_stack_model(&mgr, &reg, &stk);
    // Big-endian <join>: ctor sets consumeMostSig=true, justifyRight=true.
    let rule = decode_rule(r#"<rule><datatype name="any"/><join/></rule>"#, &model, &mgr)
        .expect("decode join");
    let mut status = vec![0i32; 3];
    let mut res = ParameterPieces::default();
    let r = rule
        .assign_address(&dt_int(3), &PrototypePieces::default().with_dummy(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .expect("assign");
    assert_eq!(r, AssignActionResponse::success);
    // addOffset = BE(true) ^ mostSig(true) ^ justRight(true) = true -> offset += 1.
    // justifyRight=true picks pieces[0]; offset 0x10 + 1 = 0x11, size 4 -> 3.
    assert_eq!(status[0], -1);
    assert_eq!(res.addr.get_offset(), 0x11, "BE/justify-right shifts the offset up by the padding");
}

// ===========================================================================
// 3. HomogeneousAggregate::filter pointer-identity (Rc::ptr_eq)
// ===========================================================================

#[test]
fn v_w6_s4_modelrules_homogeneous_aggregate_ptr_identity() {
    let mgr = AddrSpaceManager::new();
    // <datatype name="homogeneous-float-aggregate"/> -> meta=FLOAT, maxPrim=4.
    let f = decode_filter(r#"<datatype name="homogeneous-float-aggregate"/>"#, &mgr)
        .expect("homo decode");

    // Array of 4 identical floats: every primitive shares the same Rc (array base)
    // -> Rc::ptr_eq true for all, base meta == FLOAT -> ACCEPT.
    let f4 = dt_float(4);
    let arr = dt_array(Rc::clone(&f4), 4);
    assert!(f.filter(&arr), "array of 4 identical floats is a homogeneous float aggregate");

    // Array of 5 identical floats exceeds maxPrimitives=4 -> extractor invalid -> REJECT.
    let arr5 = dt_array(Rc::clone(&f4), 5);
    assert!(!f.filter(&arr5), "5 primitives exceeds the maxprimitives=4 default");

    // Struct {float, int}: primitives differ (and one is non-float) -> REJECT.
    let mixed = dt_struct(8, 4, &[(0, dt_float(4)), (4, dt_int(4))]);
    assert!(!f.filter(&mixed), "mixed float/int struct is not a homogeneous float aggregate");

    // Struct {float@0, float@4} built from DISTINCT Rc allocations of the same
    // logical float: C++ would dedup these to one Datatype* (ptr-eq true); the
    // kuna harness does NOT dedup, so Rc::ptr_eq is false on field 1 -> REJECT.
    // This documents the TypeFactory-cache dependency of the ptr-identity port.
    let s_distinct = dt_struct(8, 4, &[(0, dt_float(4)), (4, dt_float(4))]);
    assert!(
        !f.filter(&s_distinct),
        "distinct-Rc fields are NOT ptr-eq in the no-dedup harness (cache-determinism dependency)"
    );

    // Struct {float@0, float@4} built from a SHARED Rc: ptr-eq true -> ACCEPT,
    // matching C++ when the TypeFactory has deduplicated the field type.
    let shared = dt_float(4);
    let s_shared = dt_struct(8, 4, &[(0, Rc::clone(&shared)), (4, Rc::clone(&shared))]);
    assert!(f.filter(&s_shared), "shared-Rc identical float fields ARE a homogeneous float aggregate");

    // An int aggregate is rejected because the base meta (INT) != FLOAT.
    let i4 = dt_int(4);
    let iarr = dt_array(Rc::clone(&i4), 2);
    assert!(!f.filter(&iarr), "homogeneous INT aggregate fails a FLOAT filter");
}

// ===========================================================================
// 4. VarargsFilter default boundary (INT_MIN/INT_MAX) + firstVarArgSlot<0
// ===========================================================================

#[test]
fn v_w6_s4_modelrules_varargs_default_and_boundary() {
    // The decode default (<varargs/> with no first/last) is firstPos=INT_MIN,
    // lastPos=INT_MAX: matches ANY parameter once the prototype is varargs.
    let q_default = QualifierFilter::Varargs { first_pos: i32::MIN, last_pos: i32::MAX };
    let mut proto = PrototypePieces::default();

    // firstVarArgSlot < 0 -> never matches, regardless of pos.
    proto.first_var_arg_slot = -1;
    assert!(!q_default.filter(&proto, 0));
    assert!(!q_default.filter(&proto, 5));

    // firstVarArgSlot = 0 -> pos-0 in [INT_MIN,INT_MAX] is always true for any
    // representable pos, including the non-optional pos that maps to a negative.
    proto.first_var_arg_slot = 0;
    assert!(q_default.filter(&proto, 0));
    assert!(q_default.filter(&proto, 100));

    // <varargs first="0"/> : matches pos >= firstVarArgSlot (optional args only).
    let q_first0 = QualifierFilter::Varargs { first_pos: 0, last_pos: i32::MAX };
    proto.first_var_arg_slot = 3;
    assert!(!q_first0.filter(&proto, 2), "pos 2 is non-optional (2-3 = -1 < 0)");
    assert!(q_first0.filter(&proto, 3), "pos 3 is the first optional (3-3 = 0)");
    assert!(q_first0.filter(&proto, 9));

    // <varargs first="-1"/> : matches the last NON-optional arg and all optional.
    let q_firstm1 = QualifierFilter::Varargs { first_pos: -1, last_pos: i32::MAX };
    assert!(q_firstm1.filter(&proto, 2), "pos 2 (2-3=-1) is the last non-optional");
    assert!(!q_firstm1.filter(&proto, 1), "pos 1 (1-3=-2) is before the matched range");

    // <varargs first="0" last="0"/> : matches ONLY the first optional argument.
    let q_first0last0 = QualifierFilter::Varargs { first_pos: 0, last_pos: 0 };
    assert!(q_first0last0.filter(&proto, 3), "first optional only");
    assert!(!q_first0last0.filter(&proto, 4), "second optional excluded by last=0");
}

// A tiny extension trait so the justify tests can pass a non-default proto
// without depending on PrototypePieces internals (the dummy is unused by
// MultiSlotAssign::assignAddress, which ignores proto/pos entirely).
trait DummyProto {
    fn with_dummy(self) -> Self;
}
impl DummyProto for PrototypePieces {
    fn with_dummy(mut self) -> Self {
        self.first_var_arg_slot = 0;
        self
    }
}
