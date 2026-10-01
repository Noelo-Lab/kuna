//! The convention a declared function-pointer prototype is forced under at an
//! indirect call.  A declarator that names no convention gets the default model,
//! which passes floating-point values in floating-point registers.  An image may
//! pass them in integer registers (soft-float) or keep doubles out of the
//! floating-point registers (single-float); forcing the default storage there
//! reads registers the caller never set and drops the arguments it did set.
use crate::{
    context::ArchContext,
    dtype::type_class,
    fspec::{FuncProto, ProtoModel, PrototypePieces},
    funcdata::Funcdata,
    infra::architecture::Architecture,
};
use kuna_base::{address::Address, space::AddrSpace};
use std::rc::Rc;

/// The ARM model for the base (core-register) procedure-call standard.
const SOFT_FLOAT_MODEL: &str = "__stdcall_softfp";

/// Whether the image passes floating-point arguments, doubles included, in
/// floating-point registers: x86 and AArch64 always do; elsewhere only the
/// convention the container states decides.
pub fn image_evidence(arch: &Architecture) -> Option<bool> {
    if arch.archid.starts_with("x86:") || arch.archid.starts_with("AARCH64:") {
        return Some(true);
    }
    arch.translate()
        .loader_rc()
        .try_borrow()
        .ok()
        .and_then(|loader| loader.float_arg_registers())
}

/// The spec's soft-float model, on an image that states the soft-float convention.
pub fn soft_model(arch: &Architecture, evidence: Option<bool>) -> Option<Rc<ProtoModel>> {
    (evidence == Some(false))
        .then(|| arch.get_model(SOFT_FLOAT_MODEL).cloned())
        .flatten()
}

/// The prototype to force at a CALLIND typed with `proto`, or `None` to leave the
/// call to model recovery.  A prototype that puts no parameter or return value in
/// the model's floating-point registers, or names its own convention, is forced as
/// declared.  Otherwise the stated convention decides: hard-float keeps it;
/// soft-float rebuilds it under the soft-float model, or declines when the spec
/// has none; single-float or no stated convention declines.
pub fn admit(data: &Funcdata, proto: Rc<FuncProto>) -> Option<Rc<FuncProto>> {
    if !proto.has_model() {
        return Some(proto);
    }
    let entries = float_entries(proto.model());
    if !uses_float_storage(&proto, &entries) {
        return Some(proto);
    }
    let arch = data.get_arch();
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

/// Some parameter or the return value lives in a floating-point entry, directly or
/// as a piece of a join.
fn uses_float_storage(proto: &FuncProto, entries: &[(i32, u64, u64)]) -> bool {
    let in_entry = |addr: Address, size: i32| {
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
        overlaps(entries, space, addr.get_offset(), size.max(0) as u64)
    };
    let output = proto.get_output();
    (0..proto.num_params())
        .filter_map(|i| proto.get_param(i))
        .any(|p| in_entry(p.get_address(), p.get_size()))
        || (output.get_size() > 0 && in_entry(output.get_address(), output.get_size()))
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
