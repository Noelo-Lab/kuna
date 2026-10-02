//! The convention a declared function-pointer prototype is forced under at an
//! indirect call.  A declarator that names no convention gets the default model,
//! which passes floating-point values in floating-point registers.  An image may
//! pass them in integer registers (soft-float) or keep doubles out of the
//! floating-point registers (single-float); forcing the default storage there
//! reads registers the caller never set and drops the arguments it did set.
//! A variadic prototype is forced only when every fixed parameter and the return
//! value sit in one non-floating-point location: ARM passes every variadic value
//! in core registers, and a locked variadic prototype with a floating-point
//! return or a register-pair parameter misreads the arguments past it.  A return
//! value narrower than its register is forced only where the model extends it by
//! its type or the caller extends it: a MIPS model states no extension, a RISC-V
//! one zero-extends signed values, and an Apple arm64 callee sign-extends a
//! signed value narrower than 32 bits that the model zero-extends.
use crate::{
    context::ArchContext,
    dtype::{type_class, type_metatype, Datatype, TypeFactory},
    fspec::{FuncProto, ProtoModel, PrototypePieces},
    funcdata::Funcdata,
    infra::architecture::Architecture,
};
use kuna_base::{
    address::Address,
    space::{AddrSpace, AddrSpaceManager},
};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use std::rc::Rc;

/// The ARM model for the base (core-register) procedure-call standard.
const SOFT_FLOAT_MODEL: &str = "__stdcall_softfp";

/// Whether the image passes floating-point arguments, doubles included, in
/// floating-point registers: x86 and AArch64 always do; elsewhere only the
/// convention the container states decides.
pub fn image_evidence(arch: &Architecture) -> Option<bool> {
    if x86_or_aarch64(arch) {
        return Some(true);
    }
    arch.translate()
        .loader_rc()
        .try_borrow()
        .ok()
        .and_then(|loader| loader.float_arg_registers())
}

fn x86_or_aarch64(arch: &Architecture) -> bool {
    arch.archid.starts_with("x86:") || arch.archid.starts_with("AARCH64:")
}

/// The narrowest return value, in bytes, whose caller extends it itself: any on
/// x86, and on AArch64 where the container states a platform other than Apple's;
/// otherwise 32 bits on AArch64, since an Apple arm64 callee extends a narrower
/// one and its caller reads it as it is; none elsewhere.
pub fn caller_extends_from(arch: &Architecture) -> i32 {
    if arch.archid.starts_with("x86:") {
        return 1;
    }
    if !arch.archid.starts_with("AARCH64:") {
        return i32::MAX;
    }
    let stated_other = !arch.archid.starts_with("AARCH64:LE:64:AppleSilicon:")
        && arch
            .translate()
            .loader_rc()
            .try_borrow()
            .is_ok_and(|loader| loader.callee_extends_returns() == Some(false));
    if stated_other {
        1
    } else {
        4
    }
}

/// The spec's soft-float model, on an image that states the soft-float convention.
pub fn soft_model(arch: &Architecture, evidence: Option<bool>) -> Option<Rc<ProtoModel>> {
    (evidence == Some(false))
        .then(|| arch.get_model(SOFT_FLOAT_MODEL).cloned())
        .flatten()
}

/// The model a declared prototype that names no convention is laid out under,
/// or `None` for the default one: the soft-float model, on an image that states
/// the soft-float convention, when the default model would put a parameter or
/// the return value of `pieces` in its floating-point registers.
pub fn undeclared_model(arch: &ArchContext, pieces: &PrototypePieces) -> Option<Rc<ProtoModel>> {
    let soft = arch.soft_float_model.as_ref()?;
    soft_layout(soft, arch.default_fp()?, pieces, arch.types()?, arch.manage())
}

/// [`undeclared_model`] for the architecture a declaration is parked on.
pub fn undeclared_model_for(arch: &Architecture, pieces: &PrototypePieces) -> Option<Rc<ProtoModel>> {
    let soft = soft_model(arch, image_evidence(arch))?;
    soft_layout(&soft, arch.default_fp()?, pieces, arch.types(), arch.manage())
}

fn soft_layout(
    soft: &Rc<ProtoModel>,
    default: &Rc<ProtoModel>,
    pieces: &PrototypePieces,
    types: &dyn TypeFactory,
    manage: &AddrSpaceManager,
) -> Option<Rc<ProtoModel>> {
    let candidate = |ty: &Rc<Datatype>| {
        matches!(
            ty.get_metatype(),
            type_metatype::TYPE_FLOAT
                | type_metatype::TYPE_STRUCT
                | type_metatype::TYPE_UNION
                | type_metatype::TYPE_ARRAY
        )
    };
    if !pieces.outtype.iter().chain(&pieces.intypes).any(candidate) {
        return None;
    }
    let types_only = PrototypePieces {
        input_storage: Vec::new(),
        output_storage: None,
        ..pieces.clone()
    };
    let mut proto = FuncProto::new();
    let void = types.get_type_void().ok()?;
    proto
        .seed_locked_from_pieces(&types_only, Rc::clone(default), void, types, manage)
        .ok()?;
    uses_float_storage(&storage(&proto), &float_entries(default)).then(|| Rc::clone(soft))
}

/// The prototype to force at a CALLIND typed with `proto`, or `None` to leave the
/// call to model recovery.  A prototype whose return value is narrower than its
/// register and extended by neither the model by type nor the caller is
/// declined.  A variadic prototype is forced only when no fixed parameter or
/// return value lives in a join or in the model's floating-point registers.  Any
/// other prototype that puts nothing in those registers, or names its own
/// convention, is forced as declared.  Otherwise the stated convention decides:
/// hard-float keeps it; soft-float rebuilds it under the soft-float model, or
/// declines when the spec has none; single-float or no stated convention
/// declines.
pub fn admit(data: &Funcdata, proto: Rc<FuncProto>) -> Option<Rc<FuncProto>> {
    if !proto.has_model() {
        return Some(proto);
    }
    let arch = data.get_arch();
    if proto.get_output().get_size() < arch.caller_extends_returns_from
        && unextended_return(&proto)
    {
        return None;
    }
    let storage = storage(&proto);
    let in_float = uses_float_storage(&storage, &float_entries(proto.model()));
    if proto.is_dotdotdot() {
        let in_join = storage.iter().any(|(addr, _)| addr.is_join());
        return (!in_float && !in_join).then_some(proto);
    }
    if !in_float {
        return Some(proto);
    }
    let default = arch.defaultfp.as_ref().map(|model| model.get_name());
    if default.is_some_and(|name| name != proto.get_model_name()) {
        return Some(proto);
    }
    match arch.float_arg_registers {
        Some(true) => Some(proto),
        Some(false) => rebuild(arch, &proto),
        None => None,
    }
}

/// The (space index, offset, size) of each floating-point entry of the model.
fn float_entries(model: &ProtoModel) -> Vec<(i32, u64, u64)> {
    model
        .input()
        .get_entry()
        .iter()
        .chain(model.output().get_entry())
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT)
        .map(|e| {
            (
                e.get_space().get_index(),
                e.get_base(),
                e.get_size().max(0) as u64,
            )
        })
        .collect()
}

fn overlaps(entries: &[(i32, u64, u64)], space: &AddrSpace, offset: u64, size: u64) -> bool {
    entries.iter().any(|&(index, base, len)| {
        index == space.get_index() && base < offset + size && offset < base + len
    })
}

/// A non-floating-point return value sits in the low part of a register entry
/// for which the model states no extension by type: the callee's extension of
/// the rest goes unmodelled (MIPS) or is modelled with the wrong sign (RISC-V).
fn unextended_return(proto: &FuncProto) -> bool {
    let output = proto.get_output();
    let size = output.get_size();
    let float = output
        .get_type()
        .is_some_and(|ty| ty.get_metatype() == type_metatype::TYPE_FLOAT);
    if size <= 0 || float {
        return false;
    }
    let addr = output.get_address();
    let mut container = VarnodeData::default();
    match proto.model().assumed_output_extension(&addr, size, &mut container) {
        OpCode::CPUI_PIECE => false,
        OpCode::CPUI_COPY => {
            let Some(space) = addr.get_space() else {
                return false;
            };
            let offset = addr.get_offset();
            proto.model().output().get_entry().iter().any(|e| {
                let len = e.get_size().max(0) as u64;
                e.get_type() != type_class::TYPECLASS_FLOAT
                    && e.get_space().get_index() == space.get_index()
                    && e.get_base() <= offset
                    && offset + size as u64 <= e.get_base() + len
                    && (size as u64) < len
            })
        }
        _ => true,
    }
}

/// The storage of each parameter and of a non-void return value.
fn storage(proto: &FuncProto) -> Vec<(Address, i32)> {
    let output = proto.get_output();
    (0..proto.num_params())
        .filter_map(|i| proto.get_param(i))
        .map(|p| (p.get_address(), p.get_size()))
        .chain((output.get_size() > 0).then(|| (output.get_address(), output.get_size())))
        .collect()
}

/// Some storage lies in a floating-point entry, directly or as a piece of a join.
fn uses_float_storage(storage: &[(Address, i32)], entries: &[(i32, u64, u64)]) -> bool {
    storage.iter().any(|(addr, size)| {
        let Some(space) = addr.get_space() else {
            return false;
        };
        if addr.is_join() {
            return space.find_join(addr.get_offset()).is_ok_and(|rec| {
                (0..rec.num_pieces()).any(|i| {
                    let piece = rec.get_piece(i);
                    piece
                        .space
                        .as_ref()
                        .is_some_and(|s| overlaps(entries, s, piece.offset, u64::from(piece.size)))
                })
            });
        }
        overlaps(entries, space, addr.get_offset(), (*size).max(0) as u64)
    })
}

fn rebuild(arch: &ArchContext, proto: &FuncProto) -> Option<Rc<FuncProto>> {
    let model = arch.soft_float_model.clone()?;
    let types = arch.types()?;
    let mut pieces = PrototypePieces::default();
    proto.get_pieces(&mut pieces);
    let mut soft = FuncProto::new();
    soft.set_internal(model, types.get_type_void().ok()?);
    soft.update_all_types(&pieces, types, &arch.manage).ok()?;
    soft.set_input_lock(true);
    soft.set_output_lock(true);
    Some(Rc::new(soft))
}
