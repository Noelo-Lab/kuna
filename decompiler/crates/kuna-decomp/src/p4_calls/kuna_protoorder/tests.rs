//! Unit tests for the recovered-prototype converter and its acceptance policy.
//!
//! The end-to-end witness -- a callee's recovered prototype reaching its
//! caller's own signature, and a five-argument call to a one-parameter callee
//! surviving the park -- lives in `tests/cli/protoorder-*.json`, which drives
//! the whole `decompile-all` path the option actually changes.

use super::*;

use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::uintb;

use crate::dtype::{type_metatype, Datatype};
use crate::fspec::{parameter_pieces_flags, ParameterPieces, ProtoModel};

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 1, 0, 0, 0))
}

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 2, 0, 0, 0))
}

fn at(space: &Rc<AddrSpace>, off: uintb) -> Address {
    Address::new(Rc::clone(space), off)
}

fn int8() -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(8, 8, type_metatype::TYPE_INT))
}

fn voidt() -> Rc<Datatype> {
    Rc::new(Datatype::new(0, type_metatype::TYPE_VOID))
}

/// A proto with a model and an internal store: the shape recovery leaves behind
/// on a decompiled function.
fn recovered_proto(params: &[(Address, Rc<Datatype>)], out: Rc<Datatype>) -> FuncProto {
    let mgr = AddrSpaceManager::new();
    let model = Rc::new(ProtoModel::new(&mgr));
    let mut fp = FuncProto::new();
    fp.set_internal(model, voidt());
    for (i, (addr, ct)) in params.iter().enumerate() {
        let piece = ParameterPieces {
            addr: addr.clone(),
            type_: Some(Rc::clone(ct)),
            flags: 0,
        };
        fp.set_param(i as int4, &format!("a{i}"), &piece);
    }
    let out_piece = ParameterPieces {
        addr: Address::new_invalid(),
        type_: Some(out),
        flags: 0,
    };
    fp.set_output(&out_piece);
    fp
}

#[test]
fn converter_carries_types_names_and_storage() {
    let reg = reg_space();
    let fp = recovered_proto(&[(at(&reg, 0x38), int8()), (at(&reg, 0x30), int8())], int8());
    let (pieces, storage) = recovered_pieces(&fp, "sub_3700").expect("accepted");
    assert_eq!(pieces.name, "sub_3700");
    assert_eq!(pieces.intypes.len(), 2);
    assert_eq!(pieces.innames, vec!["a0".to_string(), "a1".to_string()]);
    // The parked list is OPEN past the recovered parameters: a caller that
    // passes more than the callee recovered still recovers the rest, because a
    // call spec whose prototype is locked AND variadic keeps active-input
    // recovery on (`ActionFuncLink::func_link_input`: `if !inputlocked ||
    // varargs`).  A closed list would delete those arguments outright.
    assert_eq!(pieces.first_var_arg_slot, 2);
    assert_eq!(storage, vec![(at(&reg, 0x38), 8), (at(&reg, 0x30), 8)]);
}

#[test]
fn void_in_void_out_is_declined() {
    let fp = recovered_proto(&[], voidt());
    assert_eq!(recovered_pieces(&fp, "sub_1000").err(), Some(Decline::VoidVoid));
}

#[test]
fn void_in_with_a_return_type_is_kept() {
    let fp = recovered_proto(&[], int8());
    let (pieces, storage) = recovered_pieces(&fp, "sub_1000").expect("accepted");
    assert!(pieces.intypes.is_empty());
    assert!(storage.is_empty());
}

#[test]
fn variadic_is_declined() {
    let reg = reg_space();
    let mut fp = recovered_proto(&[(at(&reg, 0x38), int8())], int8());
    fp.set_dotdotdot(true);
    assert_eq!(recovered_pieces(&fp, "sub_2000").err(), Some(Decline::Variadic));
}

#[test]
fn a_prototype_with_no_store_is_declined() {
    let fp = FuncProto::new();
    assert_eq!(recovered_pieces(&fp, "sub_3000").err(), Some(Decline::NoRecovery));
}

#[test]
fn a_hidden_return_parameter_is_declined() {
    let reg = reg_space();
    let mgr = AddrSpaceManager::new();
    let model = Rc::new(ProtoModel::new(&mgr));
    let mut fp = FuncProto::new();
    fp.set_internal(model, voidt());
    let piece = ParameterPieces {
        addr: at(&reg, 0x38),
        type_: Some(int8()),
        flags: parameter_pieces_flags::HIDDENRETPARM,
    };
    fp.set_param(0, "a0", &piece);
    assert_eq!(recovered_pieces(&fp, "sub_5000").err(), Some(Decline::HiddenParam));
}

/// The recovered storage is reported alongside the types so the park can
/// compare it with the model's own assignment; a skipped argument register is
/// the shape that comparison exists to catch.
#[test]
fn a_skipped_register_is_visible_in_the_recovered_storage() {
    let reg = reg_space();
    let fp = recovered_proto(&[(at(&reg, 0x38), int8()), (at(&reg, 0x10), int8())], int8());
    let (_pieces, storage) = recovered_pieces(&fp, "sub_3700").expect("accepted");
    // What the model would say from the types alone is the first two argument
    // registers in canonical order; the second parameter is not there.
    let model = vec![(at(&reg, 0x38), 8), (at(&reg, 0x30), 8)];
    assert_ne!(storage, model);
}

/// A stack parameter the model would have put in a register: same comparison.
#[test]
fn a_stack_parameter_is_visible_in_the_recovered_storage() {
    let reg = reg_space();
    let stack =
        Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 2, 0, 1, 1));
    let fp = recovered_proto(&[(at(&stack, 0x8), int8())], int8());
    let (_pieces, storage) = recovered_pieces(&fp, "sub_6000").expect("accepted");
    assert_eq!(storage, vec![(at(&stack, 0x8), 8)]);
    assert_ne!(storage, vec![(at(&reg, 0x38), 8)]);
}

/// The over-recovery trim: a trailing parameter the body never reads is not
/// stated, so no caller has to materialise it.  Here the third parameter's
/// register is written before every path terminator and never read -- the
/// register an earlier call left live.
#[test]
fn a_provably_dead_trailing_parameter_is_trimmed() {
    let reg = reg_space();
    let fp = recovered_proto(
        &[(at(&reg, 0x38), int8()), (at(&reg, 0x30), int8()), (at(&reg, 0x10), int8())],
        int8(),
    );
    let (mut pieces, mut storage) = recovered_pieces(&fp, "sub_7000").expect("accepted");
    let cuts = vec![(0x10..0x18).map(|b| (reg.get_index(), b)).collect::<Vec<_>>()];
    let facts = crate::kuna_calleedeadarg::CalleeEntryDead::from_parts(
        reg.get_index(),
        // The first two parameters' registers are read before they are written.
        vec![(reg.get_index(), 0x38, 8), (reg.get_index(), 0x30, 8)],
        cuts,
        true,
    );
    assert_eq!(trim_unjustified_tail(&mut pieces, &mut storage, &facts), 1);
    assert_eq!(pieces.intypes.len(), 2);
    assert_eq!(pieces.innames, vec!["a0".to_string(), "a1".to_string()]);
    assert_eq!(storage, vec![(at(&reg, 0x38), 8), (at(&reg, 0x30), 8)]);
}

/// The shape that deleted an argument at a caller before the tail carried the
/// burden of proof: `save_cwd` (`findutils/find -O2`, one parameter in the
/// DWARF) is recovered with three because a nested call reads `rdx` the body
/// never writes.  `rdx` is neither read nor provably dead, so it is only
/// "possible"; `rsi` is written before every terminator.  Both go.
#[test]
fn a_trailing_parameter_the_body_never_reads_is_trimmed() {
    let reg = reg_space();
    let fp = recovered_proto(
        &[(at(&reg, 0x38), int8()), (at(&reg, 0x30), int8()), (at(&reg, 0x10), int8())],
        int8(),
    );
    let (mut pieces, mut storage) = recovered_pieces(&fp, "sub_24230").expect("accepted");
    // One terminator, reached with only `rsi` written: `rdx` is untouched.
    let cuts = vec![(0x30..0x38).map(|b| (reg.get_index(), b)).collect::<Vec<_>>()];
    let facts = crate::kuna_calleedeadarg::CalleeEntryDead::from_parts(
        reg.get_index(),
        vec![(reg.get_index(), 0x38, 8)],
        cuts,
        true,
    );
    assert_eq!(trim_unjustified_tail(&mut pieces, &mut storage, &facts), 2);
    assert_eq!(pieces.intypes.len(), 1);
    assert_eq!(storage, vec![(at(&reg, 0x38), 8)]);
}

/// A parameter the entry walk cannot speak about -- one on the stack -- is not
/// evidence of itself, and the floor keeps the caller free to pass it anyway.
#[test]
fn a_stack_tail_parameter_is_trimmed() {
    let reg = reg_space();
    let stack =
        Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 2, 0, 1, 1));
    let fp = recovered_proto(&[(at(&reg, 0x38), int8()), (at(&stack, 0x8), int8())], int8());
    let (mut pieces, mut storage) = recovered_pieces(&fp, "sub_7300").expect("accepted");
    let facts = crate::kuna_calleedeadarg::CalleeEntryDead::from_parts(
        reg.get_index(),
        vec![(reg.get_index(), 0x38, 8)],
        vec![Vec::new()],
        true,
    );
    assert_eq!(trim_unjustified_tail(&mut pieces, &mut storage, &facts), 1);
    assert_eq!(storage, vec![(at(&reg, 0x38), 8)]);
}

/// Only the tail is trimmed: removing an interior parameter would leave a hole
/// the convention re-packs, which silently re-binds every later type.
#[test]
fn an_interior_dead_parameter_is_left_alone() {
    let reg = reg_space();
    let fp = recovered_proto(&[(at(&reg, 0x38), int8()), (at(&reg, 0x30), int8())], int8());
    let (mut pieces, mut storage) = recovered_pieces(&fp, "sub_7100").expect("accepted");
    // The FIRST parameter's register is the provably dead one; the LAST is read.
    let cuts = vec![(0x38..0x40).map(|b| (reg.get_index(), b)).collect::<Vec<_>>()];
    let facts = crate::kuna_calleedeadarg::CalleeEntryDead::from_parts(
        reg.get_index(),
        vec![(reg.get_index(), 0x30, 8)],
        cuts,
        true,
    );
    assert_eq!(trim_unjustified_tail(&mut pieces, &mut storage, &facts), 0);
    assert_eq!(pieces.intypes.len(), 2);
}

/// An incomplete walk proves nothing, so nothing is trimmed and the recovered
/// list stands or falls on the rest of the policy.
#[test]
fn an_incomplete_walk_trims_nothing() {
    let reg = reg_space();
    let fp = recovered_proto(&[(at(&reg, 0x38), int8())], int8());
    let (mut pieces, mut storage) = recovered_pieces(&fp, "sub_7200").expect("accepted");
    let cuts = vec![(0x38..0x40).map(|b| (reg.get_index(), b)).collect::<Vec<_>>()];
    let facts = crate::kuna_calleedeadarg::CalleeEntryDead::from_parts(
        reg.get_index(),
        Vec::new(),
        cuts,
        false,
    );
    assert_eq!(trim_unjustified_tail(&mut pieces, &mut storage, &facts), 0);
    assert_eq!(pieces.intypes.len(), 1);
}

/// The under-recovery rule: the callee reads the register its NEXT argument
/// would arrive in, so its recovered list is short and must not be stated.
/// This is what a variadic's register-save prologue looks like from outside.
#[test]
fn a_read_of_the_next_argument_register_is_under_recovery() {
    let reg = reg_space();
    let facts = crate::kuna_calleedeadarg::CalleeEntryDead::from_parts(
        reg.get_index(),
        vec![(reg.get_index(), 0x30, 8)],
        vec![Vec::new()],
        true,
    );
    assert!(slot_is_read(Some((at(&reg, 0x30), 8)), &facts));
    assert!(!slot_is_read(Some((at(&reg, 0x10), 8)), &facts));
    // A storage the model cannot name says nothing either way.
    assert!(!slot_is_read(None, &facts));
}

/// The register-file rule: a recovered list whose last parameter sits in the
/// last argument register the convention has is declined, because the next slot
/// is a stack offset the entry walk cannot speak about -- and the population on
/// that boundary is dominated by the register-save prologue.
#[test]
fn a_list_that_fills_the_argument_registers_is_declined() {
    let reg = reg_space();
    let stack = stack_space();
    // Six integer registers recovered, the seventh slot on the stack: saturated.
    assert!(ends_at_the_register_boundary(Some(&(at(&reg, 0x48), 8)), Some((at(&stack, 8), 8))));
    // Room left in the register file: the body can still be asked about it.
    assert!(!ends_at_the_register_boundary(Some(&(at(&reg, 0x10), 8)), Some((at(&reg, 0x18), 8))));
    // A convention that passes everything on the stack never trips this.
    assert!(!ends_at_the_register_boundary(Some(&(at(&stack, 4), 4)), Some((at(&stack, 8), 4))));
    // Nothing recovered, or a slot the model cannot name: no decision.
    assert!(!ends_at_the_register_boundary(None, Some((at(&reg, 0x18), 8))));
    assert!(!ends_at_the_register_boundary(Some(&(at(&reg, 0x48), 8)), None));
}

#[test]
fn decline_tokens_are_stable() {
    assert_eq!(Decline::Scc.as_str(), "scc");
    assert_eq!(Decline::Declared.as_str(), "declared");
    assert_eq!(Decline::Variadic.as_str(), "variadic");
    assert_eq!(Decline::DecompileError.as_str(), "decompile-error");
    assert_eq!(Decline::NonCanonicalStorage.as_str(), "non-canonical-storage");
    assert_eq!(Decline::StorageUnderivable.as_str(), "storage-underivable");
    assert_eq!(Decline::UnderRecovered.as_str(), "under-recovered");
    assert_eq!(Decline::RegisterFileFull.as_str(), "register-file-full");
}

// --- `types`: the storage-and-position agreement the vote rests on -----------

fn ptr_to(ptrto: Rc<Datatype>) -> Rc<Datatype> {
    let mut p = Datatype::new(8, type_metatype::TYPE_PTR);
    p.submeta = crate::dtype::sub_metatype::SUB_PTR;
    p.kind = crate::dtype::DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(p)
}

fn stated(params: &[(Address, int4, Rc<Datatype>)]) -> RecoveredTypes {
    RecoveredTypes { inputs: params.to_vec() }
}

#[test]
fn option_parses_the_four_values() {
    assert_eq!(OptionProtoOrder.apply("off").unwrap().0, ProtoOrderMode::Off);
    assert_eq!(OptionProtoOrder.apply("types").unwrap().0, ProtoOrderMode::Types);
    assert_eq!(OptionProtoOrder.apply("cycles").unwrap().0, ProtoOrderMode::Cycles);
    assert_eq!(OptionProtoOrder.apply("lock").unwrap().0, ProtoOrderMode::Lock);
    // `on` was the round-3 spelling of what is now `lock`; accepting it as an
    // alias for either value would silently change what an old command means.
    assert!(OptionProtoOrder.apply("on").is_err());
    assert!(OptionProtoOrder.apply("").is_err());
    assert!(!ProtoOrderMode::Off.is_on());
    assert!(ProtoOrderMode::Types.is_on() && ProtoOrderMode::Lock.is_on());
    assert!(ProtoOrderMode::Cycles.is_on());
}

#[test]
fn only_cycles_states_inside_a_cycle_and_it_never_locks() {
    for mode in [ProtoOrderMode::Off, ProtoOrderMode::Types, ProtoOrderMode::Cycles, ProtoOrderMode::Lock] {
        assert_eq!(ProtoOrderMode::from_u8(mode.as_u8()), mode);
        assert_eq!(OptionProtoOrder.apply(mode.as_str()).unwrap().0, mode);
    }
    assert!(ProtoOrderMode::Cycles.states_in_cycles());
    assert!(!ProtoOrderMode::Types.states_in_cycles() && !ProtoOrderMode::Lock.states_in_cycles());
    assert!(ProtoOrderMode::Cycles.states_types_only() && ProtoOrderMode::Types.states_types_only());
    assert!(!ProtoOrderMode::Lock.states_types_only());
}

#[test]
fn a_matching_slot_takes_the_callees_type() {
    let reg = reg_space();
    let ct = ptr_to(int8());
    let st = stated(&[(at(&reg, 0x38), 8, Rc::clone(&ct))]);
    let got = st.param_type_at(0, &at(&reg, 0x38), 8).expect("stated");
    assert_eq!(got.get_metatype(), type_metatype::TYPE_PTR);
}

#[test]
fn a_different_storage_states_nothing() {
    let reg = reg_space();
    let st = stated(&[(at(&reg, 0x38), 8, ptr_to(int8()))]);
    // Same position, a different register: the two recoveries are not talking
    // about the same argument.
    assert!(st.param_type_at(0, &at(&reg, 0x30), 8).is_none());
    // Same register, a different width.
    assert!(st.param_type_at(0, &at(&reg, 0x38), 4).is_none());
    // The right storage at the WRONG position: a caller whose own recovery
    // ordered its arguments differently is not told anything either.
    assert!(st.param_type_at(1, &at(&reg, 0x38), 8).is_none());
    // Past the recovered list entirely.
    assert!(st.param_type_at(3, &at(&reg, 0x38), 8).is_none());
}

#[test]
fn a_stack_slot_is_matched_like_a_register() {
    let stack = stack_space();
    let st = stated(&[(at(&stack, 8), 8, int8())]);
    assert!(st.param_type_at(0, &at(&stack, 8), 8).is_some());
    assert!(st.param_type_at(0, &at(&stack, 0x10), 8).is_none());
}

#[test]
fn a_recovered_void_return_is_never_stated() {
    // `recovered_pieces` drops a void return before anything is stated, so the
    // types table can never carry one: an output lock would DELETE the caller's
    // use of the return register.
    let reg = reg_space();
    let fp = recovered_proto(&[(at(&reg, 0x38), int8())], voidt());
    let (pieces, _) = recovered_pieces(&fp, "sub_1000").expect("accepted");
    assert!(pieces.outtype.is_none());
}

#[test]
fn a_vote_is_classed_as_pointer_integer_or_float() {
    assert!(class_of(&ptr_to(int8())) == Some(Class::Pointer));
    assert!(class_of(&int8()) == Some(Class::Integer));
    assert!(class_of(&Datatype::new(4, type_metatype::TYPE_FLOAT)) == Some(Class::Float));
    // An unknown or void type says nothing a call argument does not already
    // default to, so it is never offered as a vote.
    assert!(class_of(&Datatype::new(8, type_metatype::TYPE_UNKNOWN)).is_none());
    assert!(class_of(&voidt()).is_none());
}

#[test]
fn float_and_integer_evidence_come_from_the_ops_that_touch_the_value() {
    use kuna_num::opcodes::OpCode;
    assert!(produced_class(OpCode::CPUI_FLOAT_MULT) == Some(Class::Float));
    assert!(produced_class(OpCode::CPUI_FLOAT_TRUNC) == Some(Class::Integer));
    assert!(read_class(OpCode::CPUI_FLOAT_TRUNC) == Some(Class::Float));
    assert!(read_class(OpCode::CPUI_FLOAT_INT2FLOAT) == Some(Class::Integer));
    assert!(produced_class(OpCode::CPUI_INT_SREM) == Some(Class::Integer));
    // Addition is pointer arithmetic as often as it is integer arithmetic, and a
    // copy says nothing at all.
    assert!(read_class(OpCode::CPUI_INT_ADD).is_none());
    assert!(produced_class(OpCode::CPUI_COPY).is_none());
}

#[test]
fn integer_arithmetic_comparison_and_bit_extraction_are_neutral_to_a_pointer_but_not_to_a_float() {
    use kuna_num::opcodes::OpCode;
    for code in [
        OpCode::CPUI_INT_ADD,
        OpCode::CPUI_INT_SUB,
        OpCode::CPUI_INT_EQUAL,
        OpCode::CPUI_INT_NOTEQUAL,
        OpCode::CPUI_INT_LESS,
        OpCode::CPUI_INT_SLESS,
        OpCode::CPUI_INT_LESSEQUAL,
        OpCode::CPUI_INT_SLESSEQUAL,
        OpCode::CPUI_INT_CARRY,
        OpCode::CPUI_INT_SCARRY,
        OpCode::CPUI_INT_SBORROW,
        OpCode::CPUI_INT_ZEXT,
        OpCode::CPUI_INT_SEXT,
        OpCode::CPUI_SUBPIECE,
        OpCode::CPUI_PIECE,
    ] {
        assert!(read_class(code).is_none() && produced_class(code).is_none(), "{code:?}");
        assert!(integer_but_pointer_neutral(code), "{code:?}");
    }
    for code in [OpCode::CPUI_COPY, OpCode::CPUI_LOAD, OpCode::CPUI_FLOAT_ADD, OpCode::CPUI_INT_MULT] {
        assert!(!integer_but_pointer_neutral(code), "{code:?}");
    }
}

// --- pointer votes: what the caller does through the pointer -----------------

fn prim(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

/// `struct M { int i; float f; double d; char s[4]; }`, 20 bytes.
fn struct_m() -> Rc<Datatype> {
    let mut chars = Datatype::new(4, type_metatype::TYPE_ARRAY);
    chars.kind = crate::dtype::DatatypeKind::Array { arrayof: prim(1, type_metatype::TYPE_INT), arraysize: 4 };
    let field = vec![
        crate::dtype::TypeField::new(0, 0, "i", prim(4, type_metatype::TYPE_INT)),
        crate::dtype::TypeField::new(1, 4, "f", prim(4, type_metatype::TYPE_FLOAT)),
        crate::dtype::TypeField::new(2, 8, "d", prim(8, type_metatype::TYPE_FLOAT)),
        crate::dtype::TypeField::new(3, 16, "s", Rc::new(chars)),
    ];
    let mut s = Datatype::new(20, type_metatype::TYPE_STRUCT);
    s.kind = crate::dtype::DatatypeKind::Struct { field, bitfield: Vec::new() };
    Rc::new(s)
}

#[test]
fn an_access_lands_on_one_member_or_is_split() {
    let m = struct_m();
    let meta = |r: Member| match r {
        Member::Exact(t) => Some((t.get_metatype(), t.get_size())),
        _ => None,
    };
    assert_eq!(meta(member_at(&m, 4, 4)), Some((type_metatype::TYPE_FLOAT, 4)));
    assert_eq!(meta(member_at(&m, 8, 8)), Some((type_metatype::TYPE_FLOAT, 8)));
    assert_eq!(meta(member_at(&m, 17, 1)), Some((type_metatype::TYPE_INT, 1)));
    // An 8-byte copy over `i` and `f`, the low half of `d`, and a word over the
    // byte array are what the split rules print piece by piece.
    assert!(matches!(member_at(&m, 0, 8), Member::Split));
    assert!(matches!(member_at(&m, 8, 4), Member::Split));
    assert!(matches!(member_at(&m, 16, 4), Member::Split));
    assert!(matches!(member_at(&prim(8, type_metatype::TYPE_FLOAT), 0, 4), Member::Split));
    let mut sparse = Datatype::new(24, type_metatype::TYPE_STRUCT);
    sparse.kind = crate::dtype::DatatypeKind::Struct {
        field: vec![
            crate::dtype::TypeField::new(0, 0, "a", prim(8, type_metatype::TYPE_INT)),
            crate::dtype::TypeField::new(1, 16, "c", prim(8, type_metatype::TYPE_INT)),
        ],
        bitfield: Vec::new(),
    };
    assert!(matches!(member_at(&Rc::new(sparse), 8, 4), Member::Gap));
}

#[test]
fn strides_combine_by_their_common_divisor() {
    assert_eq!(gcd(0, 16), 16);
    assert_eq!(gcd(24, 16), 8);
    assert_eq!(gcd(-8, 12), 4);
    assert_eq!(gcd(0, 0), 0);
}

fn named_struct(name: &str) -> Rc<Datatype> {
    let mut t = Datatype::new(0x28, type_metatype::TYPE_STRUCT);
    t.name = name.to_string();
    Rc::new(t)
}

fn ptr_to(elem: Rc<Datatype>) -> Rc<Datatype> {
    let mut t = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    t.kind = crate::dtype::DatatypeKind::Pointer { ptrto: elem, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(t)
}

/// The convergence sweep forgets a statement that names a superseded structure
/// at any pointer depth, and nothing else: `struct_1` does not answer for
/// `struct_10`.
#[test]
fn a_statement_naming_a_superseded_structure_is_recognised_through_pointers() {
    let stale = vec!["struct_1".to_string()];
    assert!(names_type(&ptr_to(named_struct("struct_1")), &stale));
    assert!(names_type(&ptr_to(ptr_to(named_struct("struct_1"))), &stale));
    assert!(!names_type(&ptr_to(named_struct("struct_10")), &stale));
    assert!(!names_type(&int8(), &stale));
    assert!(!names_type(&ptr_to(named_struct("struct_1")), &[]));
}
