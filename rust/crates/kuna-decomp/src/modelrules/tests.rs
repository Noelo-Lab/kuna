//! Tests for the declarative `<modelrule>` machinery (`modelrules.rs`, item
//! `w6-s4-modelrules`): PrimitiveExtractor layouts, the DatatypeFilter /
//! QualifierFilter predicates, and AssignAction decode + assignment over
//! real-shaped `.cspec` constructs.
//!
//! Address spaces are built directly with `AddrSpace::new` and parameter
//! entries with the `ParamEntry::seed` builder seam (the same approach as the
//! `fspec` tests).  The XML `<rule>`/`<datatype>`/`<consume>`/... constructs are
//! decoded with `XmlDecode` configured by an `IdRegistry` carrying the
//! modelrules ids (which the W4 decode wiring would otherwise register).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;

use crate::dtype::{type_class, type_metatype, Datatype, DatatypeKind, TypeField, TypeFactory};
use crate::fspec::{
    AssignActionResponse, ParamActive, ParamEntry, ParamListKind, ParamListStandard, ParameterPieces,
    PrototypePieces,
};

use super::*;

// =========================================================================
// Address-space + entry + datatype builders
// =========================================================================

/// A little-endian "register" space at index 3.
fn reg_space_le() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 0, 0))
}

/// A little-endian stack (spacebase) space at index 5.
fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 4, 1, 5, 0, 0, 0))
}

fn addr(spc: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(spc), off)
}

/// Build an exclusion register ParamEntry of the given storage class.
fn excl_entry_class(
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

/// Build a GENERAL exclusion register ParamEntry.
fn excl_entry(
    grp: int4,
    space: &Rc<AddrSpace>,
    base: u64,
    size: int4,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    excl_entry_class(grp, type_class::TYPECLASS_GENERAL, space, base, size, prev, mgr)
}

/// Build a stack resource ParamEntry with the given alignment.
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

fn dt_unknown(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, type_metatype::TYPE_UNKNOWN))
}

/// Build a struct Datatype from `(offset, field_type)` components.
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

/// Build an array Datatype.
fn dt_array(elem: Rc<Datatype>, nels: int4) -> Rc<Datatype> {
    let elsize = elem.get_align_size();
    let mut d = Datatype::new_with_align(elsize * nels, elem.get_alignment(), type_metatype::TYPE_ARRAY);
    d.kind = DatatypeKind::Array { arrayof: elem, arraysize: nels };
    Rc::new(d)
}

/// Build a union Datatype from field types (all at offset 0).
fn dt_union(size: int4, align: int4, fields: &[Rc<Datatype>]) -> Rc<Datatype> {
    let mut d = Datatype::new_with_align(size, align, type_metatype::TYPE_UNION);
    let field: Vec<TypeField> = fields
        .iter()
        .enumerate()
        .map(|(i, ft)| TypeField::new(i as int4, 0, format!("f{i}"), Rc::clone(ft)))
        .collect();
    d.kind = DatatypeKind::Union { field };
    Rc::new(d)
}

fn proto_empty() -> PrototypePieces {
    PrototypePieces::default()
}

// =========================================================================
// PrimitiveExtractor
// =========================================================================

#[test]
fn primitive_extractor_single() {
    let i4 = dt_int(4);
    let p = PrimitiveExtractor::new(&i4, false, 0, 16);
    assert!(p.is_valid());
    assert_eq!(p.size(), 1);
    assert_eq!(p.get(0).offset, 0);
    assert!(!p.contains_unknown());
    assert!(p.is_aligned());
    assert!(!p.contains_holes());
}

#[test]
fn primitive_extractor_unknown_flagged() {
    let u = dt_unknown(4);
    let p = PrimitiveExtractor::new(&u, false, 0, 16);
    assert!(p.is_valid());
    assert_eq!(p.size(), 1);
    assert!(p.contains_unknown());
}

#[test]
fn primitive_extractor_array_offsets() {
    // int[3] -> three int primitives at offsets 0, 4, 8.
    let arr = dt_array(dt_int(4), 3);
    let p = PrimitiveExtractor::new(&arr, false, 0, 16);
    assert!(p.is_valid());
    assert_eq!(p.size(), 3);
    assert_eq!(p.get(0).offset, 0);
    assert_eq!(p.get(1).offset, 4);
    assert_eq!(p.get(2).offset, 8);
    assert!(p.is_aligned());
    assert!(!p.contains_holes());
}

#[test]
fn primitive_extractor_struct_packed() {
    // struct { int @0; float @4; } -> aligned, no holes.
    let s = dt_struct(8, 4, &[(0, dt_int(4)), (4, dt_float(4))]);
    let p = PrimitiveExtractor::new(&s, false, 0, 16);
    assert!(p.is_valid());
    assert_eq!(p.size(), 2);
    assert_eq!(p.get(0).offset, 0);
    assert_eq!(p.get(1).offset, 4);
    assert!(p.is_aligned());
    assert!(!p.contains_holes());
    assert_eq!(p.get(1).dt.get_metatype(), type_metatype::TYPE_FLOAT);
}

#[test]
fn primitive_extractor_struct_with_hole() {
    // struct { int @0; int @8; } with size 12: the gap at offset 4..8 is a hole.
    let s = dt_struct(12, 4, &[(0, dt_int(4)), (8, dt_int(4))]);
    let p = PrimitiveExtractor::new(&s, false, 0, 16);
    assert!(p.is_valid());
    assert_eq!(p.size(), 2);
    assert!(p.contains_holes());
}

#[test]
fn primitive_extractor_max_exceeded_invalid() {
    // An array of 5 ints with max 4 -> invalid (extraction gives up).
    let arr = dt_array(dt_int(4), 5);
    let p = PrimitiveExtractor::new(&arr, false, 0, 4);
    assert!(!p.is_valid());
}

#[test]
fn primitive_extractor_union_illegal() {
    // Union flagged illegal -> extraction of a union fails (invalid).
    let u = dt_union(4, 4, &[dt_int(4), dt_float(4)]);
    let p = PrimitiveExtractor::new(&u, true, 0, 16);
    assert!(!p.is_valid());
}

#[test]
fn primitive_extractor_union_common_refinement() {
    // Union of {int4} and {float4}: integer is preferred over float at the same
    // offset, so the common refinement is a single int primitive.
    let u = dt_union(4, 4, &[dt_int(4), dt_float(4)]);
    let p = PrimitiveExtractor::new(&u, false, 0, 16);
    assert!(p.is_valid());
    assert_eq!(p.size(), 1);
    // Integer preferred (checkOverlap: float big is overridden by the int).
    assert_eq!(p.get(0).dt.get_metatype(), type_metatype::TYPE_INT);
}

// =========================================================================
// SizeRestriction / parse_size_list
// =========================================================================

#[test]
fn size_restriction_bounds() {
    let sz = SizeRestriction::new(2, 8);
    assert!(sz.filter_on_size(&dt_int(4)));
    assert!(sz.filter_on_size(&dt_int(2)));
    assert!(sz.filter_on_size(&dt_int(8)));
    assert!(!sz.filter_on_size(&dt_int(1)));
    assert!(!sz.filter_on_size(&dt_int(16)));
}

#[test]
fn size_restriction_no_filter_when_maxsize_zero() {
    let sz = SizeRestriction::new_for_decode();
    // maxSize == 0 -> no filtering, everything passes.
    assert!(sz.filter_on_size(&dt_int(99)));
}

#[test]
fn parse_size_list_comma_and_space() {
    let v = parse_size_list(b"1, 2 4,8").unwrap();
    assert_eq!(v, vec![1, 2, 4, 8]);
}

#[test]
fn parse_size_list_rejects_nonpositive() {
    assert!(parse_size_list(b"4, 0").is_err());
    assert!(parse_size_list(b"-3").is_err());
}

// =========================================================================
// DatatypeFilter predicates
// =========================================================================

#[test]
fn filter_any_passes_everything() {
    let f = DatatypeFilter::SizeRestricted(SizeRestriction::new_for_decode());
    assert!(f.filter(&dt_int(4)));
    assert!(f.filter(&dt_float(8)));
    assert!(f.filter(&dt_struct(8, 4, &[(0, dt_int(4)), (4, dt_int(4))])));
}

#[test]
fn filter_metatype_matches_only_float() {
    let f = DatatypeFilter::MetaType {
        meta_type: type_metatype::TYPE_FLOAT,
        size: SizeRestriction::new_for_decode(),
    };
    assert!(f.filter(&dt_float(8)));
    assert!(!f.filter(&dt_int(8)));
}

#[test]
fn filter_metatype_with_size_bound() {
    let f = DatatypeFilter::MetaType {
        meta_type: type_metatype::TYPE_INT,
        size: SizeRestriction::new(4, 4),
    };
    assert!(f.filter(&dt_int(4)));
    assert!(!f.filter(&dt_int(8))); // wrong size
    assert!(!f.filter(&dt_float(4))); // wrong metatype
}

#[test]
fn filter_homogeneous_float_aggregate() {
    let f = DatatypeFilter::Homogeneous {
        meta_type: type_metatype::TYPE_FLOAT,
        max_primitives: 4,
        size: SizeRestriction::new(0, 0),
    };
    // struct of two identical floats -> homogeneous float aggregate.
    let flt = dt_float(4);
    let s = dt_struct(8, 4, &[(0, Rc::clone(&flt)), (4, Rc::clone(&flt))]);
    assert!(f.filter(&s));
    // struct of int+float -> not homogeneous.
    let mixed = dt_struct(8, 4, &[(0, dt_int(4)), (4, dt_float(4))]);
    assert!(!f.filter(&mixed));
    // A scalar float is not an aggregate (must be ARRAY or STRUCT).
    assert!(!f.filter(&dt_float(4)));
}

// =========================================================================
// QualifierFilter predicates
// =========================================================================

#[test]
fn qualifier_position_match() {
    let q = QualifierFilter::Position(2);
    let proto = proto_empty();
    assert!(q.filter(&proto, 2));
    assert!(!q.filter(&proto, 1));
}

#[test]
fn qualifier_varargs_range() {
    // <varargs first="0"/> matches optional args (pos >= firstVarArgSlot).
    let q = QualifierFilter::Varargs { first_pos: 0, last_pos: 0x7fffffff };
    let mut proto = proto_empty();
    proto.first_var_arg_slot = 2;
    assert!(!q.filter(&proto, 1)); // non-optional (pos-2 = -1 < 0)
    assert!(q.filter(&proto, 2)); // first optional (pos-2 = 0)
    assert!(q.filter(&proto, 5));
    // Not varargs at all -> never matches.
    proto.first_var_arg_slot = -1;
    assert!(!q.filter(&proto, 2));
}

#[test]
fn qualifier_and_all_must_pass() {
    let q = QualifierFilter::And(vec![
        QualifierFilter::Position(1),
        QualifierFilter::Varargs { first_pos: 0, last_pos: 0x7fffffff },
    ]);
    let mut proto = proto_empty();
    proto.first_var_arg_slot = 0;
    assert!(q.filter(&proto, 1)); // position 1 AND optional
    assert!(!q.filter(&proto, 2)); // wrong position
}

#[test]
fn qualifier_datatype_at_position() {
    let q = QualifierFilter::DatatypeMatch {
        position: 0,
        type_filter: Box::new(DatatypeFilter::MetaType {
            meta_type: type_metatype::TYPE_FLOAT,
            size: SizeRestriction::new_for_decode(),
        }),
    };
    let mut proto = proto_empty();
    proto.intypes = vec![dt_float(8)];
    assert!(q.filter(&proto, 99)); // pos arg ignored; checks intypes[0]
    proto.intypes = vec![dt_int(8)];
    assert!(!q.filter(&proto, 99));
    // Position out of range -> false.
    proto.intypes = vec![];
    assert!(!q.filter(&proto, 0));
}

// =========================================================================
// AssignAction assignment (built directly, single-register paths)
// =========================================================================

/// Build a single-GENERAL-register input model: one 4-byte register at group 0.
fn single_reg_model(mgr: &AddrSpaceManager, reg: &Rc<AddrSpace>) -> ParamListStandard {
    let mut m = ParamListStandard::new(ParamListKind::Standard);
    m.push_entry(excl_entry(0, reg, 0x10, 4, &[], mgr));
    m.finish_decode();
    m
}

/// Build a model with one 4-byte register (group 0) and a stack resource
/// (group 1) so goto_stack / fall-through can find a stack entry.
fn reg_and_stack_model(
    mgr: &AddrSpaceManager,
    reg: &Rc<AddrSpace>,
    stk: &Rc<AddrSpace>,
) -> ParamListStandard {
    let mut m = ParamListStandard::new(ParamListKind::Standard);
    m.push_entry(excl_entry(0, reg, 0x10, 4, &[], mgr));
    let e1 = stack_entry(1, stk, 0x0, 0x40, 4, m.get_entry(), mgr);
    m.push_entry(e1);
    m.finish_decode();
    m
}

#[test]
fn consume_as_assigns_general_register() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let action = AssignAction::ConsumeAs { resource_type: type_class::TYPECLASS_GENERAL };
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_int(4);
    let r = action
        .assign_address(&dt, &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    assert_eq!(res.addr.get_offset(), 0x10);
    assert_eq!(status[0], -1); // register consumed
}

#[test]
fn goto_stack_assigns_stack_slot() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stk = stack_space();
    let model = reg_and_stack_model(&mgr, &reg, &stk);
    let mut action = AssignAction::GotoStack { stack_entry: None };
    action.initialize(&model).unwrap();
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_int(4);
    let r = action
        .assign_address(&dt, &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    // Stack slot at offset 0 (group 1 consumed one slot).
    assert!(res.addr.get_space().map(|s| Rc::ptr_eq(s, &stk)).unwrap_or(false));
    assert_eq!(res.addr.get_offset(), 0x0);
    assert_eq!(status[1], 1); // one stack slot consumed
}

#[test]
fn hidden_return_signals_code() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let action = AssignAction::HiddenReturnAssign {
        ret_code: AssignActionResponse::hiddenret_specialreg,
    };
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_int(8);
    let r = action
        .assign_address(&dt, &proto_empty(), -1, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::hiddenret_specialreg);
}

#[test]
fn multi_slot_single_register_success() {
    // A single 4-byte register exactly fills a 4-byte type -> one piece, success.
    // The default MultiSlotAssign for an input list consumes from the stack, so
    // the model needs a stack entry for initialize() to succeed.
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stk = stack_space();
    let model = reg_and_stack_model(&mgr, &reg, &stk);
    let mut action = new_multi_slot_assign_default(&model);
    action.initialize(&model).unwrap();
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_int(4);
    let r = action
        .assign_address(&dt, &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    assert_eq!(res.addr.get_offset(), 0x10);
    assert_eq!(status[0], -1);
}

#[test]
fn multi_slot_out_of_registers_fails_without_stack() {
    // A type bigger than the single register, no stack -> fail.  Build the
    // action with consume_from_stack = false up front so initialize() does not
    // require a stack <pentry> (mirrors a `<join stackspill="false"/>`).
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let mut action = new_multi_slot_assign_default(&model);
    if let AssignAction::MultiSlotAssign { consume_from_stack, .. } = &mut action {
        *consume_from_stack = false;
    }
    action.initialize(&model).unwrap();
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_int(8); // needs two registers; only one available
    let r = action
        .assign_address(&dt, &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::fail);
}

#[test]
fn consume_extra_consumes_register_as_sideeffect() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let mut action = AssignAction::ConsumeExtra {
        resource_type: type_class::TYPECLASS_GENERAL,
        match_size: false,
        tiles: Vec::new(),
    };
    action.initialize(&model).unwrap();
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_int(4);
    let r = action
        .assign_address(&dt, &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    assert_eq!(status[0], -1); // the register was consumed as a side effect
}

#[test]
fn consume_remaining_consumes_all() {
    // Two general registers; consume_remaining takes all unconsumed.
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    model.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
    let e1 = excl_entry(1, &reg, 0x20, 4, model.get_entry(), &mgr);
    model.push_entry(e1);
    model.finish_decode();
    let mut action = AssignAction::ConsumeRemaining {
        resource_type: type_class::TYPECLASS_GENERAL,
        tiles: Vec::new(),
    };
    action.initialize(&model).unwrap();
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_int(4);
    action
        .assign_address(&dt, &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(status[0], -1);
    assert_eq!(status[1], -1);
}

#[test]
fn extra_stack_noop_when_already_on_stack() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stk = stack_space();
    let model = reg_and_stack_model(&mgr, &reg, &stk);
    let mut action = new_extra_stack_decode();
    action.initialize(&model).unwrap();
    let mut status = vec![0i32; 2];
    // res.addr already on the stack -> success, no extra consumption.
    let mut res = ParameterPieces { addr: addr(&stk, 0x0), ..Default::default() };
    let dt = dt_int(4);
    let r = action
        .assign_address(&dt, &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    assert_eq!(status[1], 0); // stack not further consumed
}

#[test]
fn convert_to_pointer_recurses_and_marks_indirect() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    // space=Some(reg) so we don't need a default data space in the manager.
    let action = AssignAction::ConvertToPointer { space: Some(Rc::clone(&reg)) };
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let dt = dt_struct(64, 4, &[(0, dt_int(4))]); // big aggregate passed by pointer
    let factory = PointerFactory;
    let r = action
        .assign_address(&dt, &proto_empty(), 0, &factory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    // The pointer (4 bytes) was assigned to the register.
    assert_eq!(res.addr.get_offset(), 0x10);
    assert_eq!(res.flags, parameter_pieces_flags::INDIRECTSTORAGE);
}

// =========================================================================
// fillinOutputMap
// =========================================================================

#[test]
fn consume_as_fillin_output_single_trial() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let action = AssignAction::ConsumeAs { resource_type: type_class::TYPECLASS_GENERAL };
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&reg, 0x10), 4);
    active.get_trial_mut(0).set_entry(Some(0), 0);
    assert!(action.fillin_output_map(&mut active, &model));
}

#[test]
fn goto_stack_fillin_output_requires_exactly_one() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stk = stack_space();
    let model = reg_and_stack_model(&mgr, &reg, &stk);
    let mut action = AssignAction::GotoStack { stack_entry: None };
    action.initialize(&model).unwrap();
    let stack_idx = model.get_stack_entry().unwrap();
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&stk, 0x0), 4);
    active.get_trial_mut(0).set_entry(Some(stack_idx), 0);
    assert!(action.fillin_output_map(&mut active, &model));
    // A second trial -> not exactly one -> false.
    active.register_trial(&addr(&stk, 0x4), 4);
    active.get_trial_mut(1).set_entry(Some(stack_idx), 0);
    assert!(!action.fillin_output_map(&mut active, &model));
}

#[test]
fn can_affect_fillin_output_flags() {
    assert!(AssignAction::ConsumeAs { resource_type: type_class::TYPECLASS_GENERAL }
        .can_affect_fillin_output());
    assert!(AssignAction::GotoStack { stack_entry: None }.can_affect_fillin_output());
    assert!(!AssignAction::HiddenReturnAssign {
        ret_code: AssignActionResponse::hiddenret_specialreg
    }
    .can_affect_fillin_output());
    assert!(!new_extra_stack_decode().can_affect_fillin_output());
}

// =========================================================================
// Decode (XML <rule> and component elements)
// =========================================================================

/// Build an IdRegistry carrying the modelrules ids needed by the decoders.
fn modelrules_registry() -> IdRegistry {
    let mut r = IdRegistry::with_base_ids();
    r.register_element(&ELEM_DATATYPE);
    r.register_element(&ELEM_CONSUME);
    r.register_element(&ELEM_CONSUME_EXTRA);
    r.register_element(&ELEM_CONVERT_TO_PTR);
    r.register_element(&ELEM_GOTO_STACK);
    r.register_element(&ELEM_JOIN);
    r.register_element(&ELEM_DATATYPE_AT);
    r.register_element(&ELEM_POSITION);
    r.register_element(&ELEM_VARARGS);
    r.register_element(&ELEM_HIDDEN_RETURN);
    r.register_element(&ELEM_JOIN_PER_PRIMITIVE);
    r.register_element(&ELEM_JOIN_DUAL_CLASS);
    r.register_element(&ELEM_EXTRA_STACK);
    r.register_element(&ELEM_CONSUME_REMAINING);
    r.register_element(&ELEM_RULE);
    r.register_attribute(&ATTRIB_SIZES);
    r.register_attribute(&ATTRIB_MAX_PRIMITIVES);
    r.register_attribute(&ATTRIB_REVERSESIGNIF);
    r.register_attribute(&ATTRIB_MATCHSIZE);
    r.register_attribute(&ATTRIB_AFTER_BYTES);
    r.register_attribute(&ATTRIB_AFTER_STORAGE);
    r.register_attribute(&ATTRIB_FILL_ALTERNATE);
    r.register_attribute(&ATTRIB_MAXSIZE);
    r.register_attribute(&ATTRIB_MINSIZE);
    r.register_attribute(&ATTRIB_STRATEGY);
    r.register_attribute(&ATTRIB_VOIDLOCK);
    r.register_attribute(&ATTRIB_REVERSEJUSTIFY);
    r.register_attribute(&ATTRIB_FIRST);
    r.register_attribute(&ATTRIB_LAST);
    r.register_attribute(&ATTRIB_A);
    r.register_attribute(&ATTRIB_B);
    r
}

/// Decode a `<rule>` blob against the given resource.
fn decode_rule(xml: &str, resource: &ParamListStandard, mgr: &AddrSpaceManager) -> KunaResult<ModelRule> {
    let registry = modelrules_registry();
    let mut decoder = XmlDecode::new(mgr, &registry);
    decoder.ingest_stream(xml.as_bytes())?;
    ModelRule::decode(&mut decoder, resource)
}

#[test]
fn decode_rule_consume_general() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let rule = decode_rule(
        r#"<rule><datatype name="float"/><consume storage="general"/></rule>"#,
        &model,
        &mgr,
    )
    .expect("decode rule");
    // The rule's filter accepts a float and rejects an int.
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let r = rule
        .assign_address(&dt_int(4), &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::fail); // datatype filter (float) rejects int
    let mut status2 = vec![0i32; 2];
    let mut res2 = ParameterPieces::default();
    let r2 = rule
        .assign_address(&dt_float(4), &proto_empty(), 0, &PanicFactory, &mut status2, &mut res2, &model, &mgr)
        .unwrap();
    assert_eq!(r2, AssignActionResponse::success);
    assert_eq!(res2.addr.get_offset(), 0x10);
}

#[test]
fn decode_rule_goto_stack() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stk = stack_space();
    let model = reg_and_stack_model(&mgr, &reg, &stk);
    let rule = decode_rule(
        r#"<rule><datatype name="any"/><goto_stack/></rule>"#,
        &model,
        &mgr,
    )
    .expect("decode rule");
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let r = rule
        .assign_address(&dt_int(4), &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    assert!(res.addr.get_space().map(|s| Rc::ptr_eq(s, &stk)).unwrap_or(false));
}

#[test]
fn decode_rule_with_position_qualifier() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let rule = decode_rule(
        r#"<rule><datatype name="any"/><position index="0"/><consume storage="general"/></rule>"#,
        &model,
        &mgr,
    )
    .expect("decode rule");
    // Qualifier passes at pos 0, fails elsewhere.
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    assert_eq!(
        rule.assign_address(&dt_int(4), &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
            .unwrap(),
        AssignActionResponse::success
    );
    let mut status2 = vec![0i32; 2];
    let mut res2 = ParameterPieces::default();
    assert_eq!(
        rule.assign_address(&dt_int(4), &proto_empty(), 1, &PanicFactory, &mut status2, &mut res2, &model, &mgr)
            .unwrap(),
        AssignActionResponse::fail
    );
}

#[test]
fn decode_rule_hidden_return_strategy() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let rule = decode_rule(
        r#"<rule><datatype name="any"/><hidden_return strategy="normalparam"/></rule>"#,
        &model,
        &mgr,
    )
    .expect("decode rule");
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let r = rule
        .assign_address(&dt_int(8), &proto_empty(), -1, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::hiddenret_ptrparam);
}

#[test]
fn decode_rule_size_restricted_filter() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = single_reg_model(&mgr, &reg);
    let rule = decode_rule(
        r#"<rule><datatype name="any" minsize="4" maxsize="4"/><consume storage="general"/></rule>"#,
        &model,
        &mgr,
    )
    .expect("decode rule");
    // 4-byte type passes the size filter; an 8-byte one is rejected by the filter.
    let mut s1 = vec![0i32; 2];
    let mut r1 = ParameterPieces::default();
    assert_eq!(
        rule.assign_address(&dt_int(4), &proto_empty(), 0, &PanicFactory, &mut s1, &mut r1, &model, &mgr)
            .unwrap(),
        AssignActionResponse::success
    );
    let mut s2 = vec![0i32; 2];
    let mut r2 = ParameterPieces::default();
    assert_eq!(
        rule.assign_address(&dt_int(8), &proto_empty(), 0, &PanicFactory, &mut s2, &mut r2, &model, &mgr)
            .unwrap(),
        AssignActionResponse::fail
    );
}

#[test]
fn decode_rule_consume_extra_sideeffect() {
    // A rule with a consume_as action plus a consume_extra side-effect.
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    // Two general registers so the side-effect has something to consume.
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    model.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
    let e1 = excl_entry(1, &reg, 0x20, 4, model.get_entry(), &mgr);
    model.push_entry(e1);
    model.finish_decode();
    let rule = decode_rule(
        r#"<rule><datatype name="any"/><consume storage="general"/><consume_extra storage="general" matchsize="false"/></rule>"#,
        &model,
        &mgr,
    )
    .expect("decode rule");
    let mut status = vec![0i32; 2];
    let mut res = ParameterPieces::default();
    let r = rule
        .assign_address(&dt_int(4), &proto_empty(), 0, &PanicFactory, &mut status, &mut res, &model, &mgr)
        .unwrap();
    assert_eq!(r, AssignActionResponse::success);
    // The consume took group 0; the side-effect consumed group 1.
    assert_eq!(status[0], -1);
    assert_eq!(status[1], -1);
}

// =========================================================================
// Test TypeFactory stubs
// =========================================================================

/// Panics on any reach; the actions that don't touch the type factory use this.
struct PanicFactory;

/// Returns a 4-byte pointer for `get_type_pointer`; everything else panics.
struct PointerFactory;

macro_rules! impl_type_factory {
    ($name:ident, $ptr_body:item) => {
        #[allow(unused_variables)]
        impl TypeFactory for $name {
            fn get_size_of_int(&self) -> int4 { panic!("type factory reached") }
            fn get_size_of_long(&self) -> int4 { panic!("type factory reached") }
            fn get_size_of_char(&self) -> int4 { panic!("type factory reached") }
            fn get_size_of_wchar(&self) -> int4 { panic!("type factory reached") }
            fn get_size_of_pointer(&self) -> int4 { panic!("type factory reached") }
            fn get_size_of_alt_pointer(&self) -> int4 { panic!("type factory reached") }
            fn get_alignment(&self, size: u32) -> KunaResult<int4> { panic!("type factory reached") }
            fn get_primitive_align_size(&self, size: u32) -> KunaResult<int4> { panic!("type factory reached") }
            fn get_type_void(&self) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_base_no_char(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_base(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_base_named(&self, s: int4, m: type_metatype, n: &str) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_char(&self, s: int4) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_code(&self) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_pointer_strip_array(&self, s: int4, pt: Rc<Datatype>, ws: u32) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            $ptr_body
            fn get_type_pointer_named(&self, s: int4, pt: Rc<Datatype>, ws: u32, n: &str) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn resize_pointer(&self, ptr: Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_pointer_rel(&self, parent_ptr: Rc<Datatype>, ptr_to: Rc<Datatype>, off: int4) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_pointer_rel_full(&self, sz: int4, parent: Rc<Datatype>, ptr_to: Rc<Datatype>, ws: int4, off: int4, nm: &str) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_pointer_with_space(&self, ptr_to: Rc<Datatype>, spc: Rc<AddrSpace>, nm: &str) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_array(&self, as_: int4, ao: Rc<Datatype>) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_struct(&self, n: &str) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_partial_struct(&self, contain: Rc<Datatype>, off: int4, sz: int4) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_union(&self, n: &str) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_partial_union(&self, contain: Rc<Datatype>, off: int4, sz: int4) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_partial_enum(&self, contain: Rc<Datatype>, off: int4, sz: int4) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_enum(&self, n: &str) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_type_spacebase(&self, id: Rc<AddrSpace>, addr: &Address) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn resize_integer(&self, ct: Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
            fn get_exact_piece(&self, ct: Rc<Datatype>, offset: int4, size: int4) -> KunaResult<Option<Rc<Datatype>>> { panic!("type factory reached") }
            fn find_by_name(&self, n: &str) -> KunaResult<Option<Rc<Datatype>>> { panic!("type factory reached") }
            fn concretize(&self, ct: Rc<Datatype>) -> KunaResult<Rc<Datatype>> { panic!("type factory reached") }
        }
    };
}

impl_type_factory!(
    PanicFactory,
    fn get_type_pointer(&self, s: int4, pt: Rc<Datatype>, ws: u32) -> KunaResult<Rc<Datatype>> {
        panic!("type factory reached")
    }
);

impl_type_factory!(
    PointerFactory,
    fn get_type_pointer(&self, s: int4, pt: Rc<Datatype>, ws: u32) -> KunaResult<Rc<Datatype>> {
        let mut d = Datatype::new_with_align(s, s, type_metatype::TYPE_PTR);
        d.kind = DatatypeKind::Pointer { ptrto: pt, spaceid: None, truncate: None, wordsize: ws };
        Ok(Rc::new(d))
    }
);
