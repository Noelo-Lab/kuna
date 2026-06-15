//! INDEPENDENT VERIFIER, item w10-struct-return (round 1).
//!
//! Adversarial parity tests for `ParamListStandardOut::assignMap`'s hidden-return
//! path (C++ `decompiler/cpp/fspec.cc:1571-1614`).  The branch under review
//! un-seams the "too-big return value is passed back through a hidden pointer
//! parameter" path: when the output cannot be assigned a register/stack address,
//! the function (1) re-types the output as a pointer-to-return-value with the
//! `indirectstorage` flag, and (2) appends a hidden input parameter carrying the
//! same pointer type with an *invalid* address (filled in later by the input
//! list).  The pointer size/wordsize come from the model's `spacebase`, or — when
//! absent — from the architecture's default data space.
//!
//! These tests drive the **public** `ParamListStandard::assign_map` against
//! synthetic `ParamListStandardOut` models built from `ParamEntry::seed`, the
//! same builder seam the in-crate unit tests use, but from OUTSIDE the crate so
//! only the published surface is exercised.  Every assertion is keyed to a
//! generic structural invariant (sizes derived from the space, flag bits, piece
//! count) — never a function name, address, or magic type — so the test cannot
//! be satisfied by a special-cased decode of one file.
//!
//! Committed with the verdict so the measurement is reproducible.

use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_class, type_metatype, Datatype, TypeFactoryImpl};
use kuna_decomp::fspec::{
    parameter_pieces_flags, ParamEntry, ParamListKind, ParamListStandard, ParameterPieces,
    PrototypePieces,
};

/// A manager whose default data space is a `register`-like space of the given
/// `addr_size` (the size a hidden-return pointer must take).  Returns the manager
/// and that space.  Parametrizing `addr_size` lets a test prove the recovered
/// pointer size *tracks the space* and is not a hardcoded constant.
fn manager_with_data_space(addr_size: u32) -> (AddrSpaceManager, Rc<AddrSpace>) {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let reg = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false, // little endian
        addr_size,
        1, // word_size
        2, // index
        0,
        0,
        0,
    ));
    m.insert_space(Rc::clone(&reg)).unwrap();
    m.set_default_code_space(2).unwrap();
    m.set_default_data_space(2).unwrap();
    (m, reg)
}

/// A manager with NO default data space (and the model below has no spacebase),
/// so the hidden-return path has nowhere to source the pointer size.
fn manager_without_data_space() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m
}

/// A `ParamListStandardOut` with one exclusion register entry of `out_size`
/// bytes at register offset 0x10.  A return value larger than `out_size` cannot
/// be assigned and must fall through to the hidden-return path.
fn one_register_out_model(
    reg: &Rc<AddrSpace>,
    out_size: int4,
    mgr: &AddrSpaceManager,
) -> ParamListStandard {
    let mut model = ParamListStandard::new(ParamListKind::StandardOut);
    let e0 = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(reg),
        0x10,
        out_size,
        1,    // minsize
        0,    // alignment == size => exclusion entry
        0,    // flags
        true, // normalstack
        false,
        &[],
        mgr,
    )
    .expect("seed exclusion output entry");
    model.push_entry(e0);
    model.finish_decode();
    model
}

fn type_factory(default_data_addr_size: int4) -> TypeFactoryImpl {
    let tf = TypeFactoryImpl::new();
    tf.setup_sizes(Some(4), default_data_addr_size, default_data_addr_size);
    tf
}

fn proto_returning(outtype: Rc<Datatype>) -> PrototypePieces {
    PrototypePieces {
        name: "anon".to_string(),
        outtype: Some(outtype),
        intypes: Vec::new(),
        innames: Vec::new(),
        first_var_arg_slot: -1,
    }
}

/// Adversarial #1 — the core un-seam: a return value too big for the output
/// register becomes an indirect-storage output PLUS a hidden pointer param.
/// Mirrors C++ fspec.cc:1584-1612.  The struct is 24 bytes; the output register
/// is 8 bytes (cannot hold it) but the 8-byte pointer fits.
#[test]
fn hidden_return_emits_indirect_output_and_trailing_pointer_param() {
    let (mgr, reg) = manager_with_data_space(8);
    let model = one_register_out_model(&reg, 8, &mgr);
    let tf = type_factory(8);
    let outtype = Rc::new(Datatype::new_with_align(24, 8, type_metatype::TYPE_STRUCT));
    let proto = proto_returning(outtype);

    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).expect("assignMap must succeed");

    // C++ pushes exactly two pieces on the hidden-return path.
    assert_eq!(res.len(), 2, "expected indirect output + hidden pointer param");

    // Piece 0: the output, re-typed to a pointer-to-struct, flagged indirect.
    assert_eq!(res[0].flags, parameter_pieces_flags::INDIRECTSTORAGE);
    assert_eq!(res[0].type_.as_ref().unwrap().get_metatype(), type_metatype::TYPE_PTR);
    assert_eq!(res[0].type_.as_ref().unwrap().get_size(), 8, "pointer size = space addr_size");

    // Piece 1: the hidden input param carries the pointer, address still invalid.
    assert_eq!(res[1].type_.as_ref().unwrap().get_metatype(), type_metatype::TYPE_PTR);
    assert!(res[1].addr.is_invalid(), "hidden param address filled in by input list");
    // No model rules => plain hidden return (not special-register).
    assert_eq!(res[1].flags, 0);
}

/// Adversarial #2 — faithfulness of the void short-circuit (C++ fspec.cc:1577).
/// A void return must produce exactly ONE piece (the void type) with no flags and
/// no hidden parameter; the hidden-return machinery must NOT fire.
#[test]
fn void_return_is_single_unflagged_piece() {
    let (mgr, reg) = manager_with_data_space(8);
    let model = one_register_out_model(&reg, 8, &mgr);
    let tf = type_factory(8);
    let voidtype = tf_void(&tf);
    let proto = proto_returning(voidtype);

    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).expect("assignMap must succeed");

    assert_eq!(res.len(), 1, "void return must not synthesize a hidden param");
    assert_eq!(res[0].flags, 0);
    assert_eq!(res[0].type_.as_ref().unwrap().get_metatype(), type_metatype::TYPE_VOID);
    // Void leaves the address invalid (C++: "Leave the address as invalid").
    assert!(res[0].addr.is_invalid());
}

/// Adversarial #3 — negative control: the seam must NOT over-fire.  A return
/// value that FITS the output register is assigned directly (one piece, no
/// indirect flag, no hidden param).  This guards against a shortcut that always
/// routes through the hidden path.
#[test]
fn fitting_return_assigns_directly_with_no_hidden_param() {
    let (mgr, reg) = manager_with_data_space(8);
    let model = one_register_out_model(&reg, 8, &mgr);
    let tf = type_factory(8);
    // A 4-byte int fits the 8-byte output register.
    let outtype = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_INT));
    let proto = proto_returning(outtype);

    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).expect("assignMap must succeed");

    assert_eq!(res.len(), 1, "an assignable return must not gain a hidden param");
    assert_eq!(res[0].flags & parameter_pieces_flags::INDIRECTSTORAGE, 0);
    assert_eq!(res[0].type_.as_ref().unwrap().get_metatype(), type_metatype::TYPE_INT);
    // It got a concrete address (the output register), not an invalid one.
    assert!(!res[0].addr.is_invalid());
}

/// Adversarial #4 — the pointer size is sourced from the default data space, not
/// hardcoded.  Same too-big struct, but the default data space is 4 bytes wide:
/// the recovered hidden-return pointer must be 4 bytes, proving the value tracks
/// `getDefaultDataSpace()->getAddrSize()` (C++ fspec.cc:1590-1592).
#[test]
fn hidden_return_pointer_size_tracks_default_data_space() {
    let (mgr, reg) = manager_with_data_space(4);
    // An output register big enough for nothing useful here; make it tiny so the
    // 24-byte struct cannot be assigned and the 4-byte pointer can.
    let model = one_register_out_model(&reg, 4, &mgr);
    let tf = type_factory(4);
    let outtype = Rc::new(Datatype::new_with_align(24, 8, type_metatype::TYPE_STRUCT));
    let proto = proto_returning(outtype);

    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).expect("assignMap must succeed");

    assert_eq!(res.len(), 2);
    // Both the indirect output and the hidden param carry a 4-byte pointer.
    assert_eq!(res[0].type_.as_ref().unwrap().get_size(), 4, "pointer follows 4-byte data space");
    assert_eq!(res[1].type_.as_ref().unwrap().get_size(), 4);
    assert_eq!(res[0].type_.as_ref().unwrap().get_metatype(), type_metatype::TYPE_PTR);
}

/// Adversarial #5 — error faithfulness, not a silent shortcut.  When the
/// hidden-return path is reached but there is no spacebase AND no default data
/// space, the C++ `spc = typefactory.getArch()->getDefaultDataSpace()` would
/// return null; the port must surface a real error rather than fabricating a
/// pointer or silently degrading.
#[test]
fn hidden_return_without_data_space_errors_cleanly() {
    let mgr = manager_without_data_space();
    // Build a register space NOT registered as the default data space, used only
    // to seed the (too-small) output entry.
    let reg = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        2,
        0,
        0,
        0,
    ));
    let model = one_register_out_model(&reg, 8, &mgr);
    let tf = type_factory(8);
    let outtype = Rc::new(Datatype::new_with_align(24, 8, type_metatype::TYPE_STRUCT));
    let proto = proto_returning(outtype);

    let mut res: Vec<ParameterPieces> = Vec::new();
    let err = model.assign_map(&proto, &tf, &mut res, &mgr).unwrap_err();
    let msg = err.explain().to_string();
    assert!(
        msg.contains("default data space") || msg.contains("hidden return"),
        "expected a clear no-default-data-space error, got: {msg}"
    );
}

/// `TypeFactory::get_type_void` is on the trait; reach it through the impl.
fn tf_void(tf: &TypeFactoryImpl) -> Rc<Datatype> {
    use kuna_decomp::dtype::TypeFactory;
    tf.get_type_void().expect("void type")
}
