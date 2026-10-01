//! The convention a declared function-pointer prototype is forced under at an
//! indirect call.  A declarator that names no convention gets the default model,
//! which passes floating-point values in floating-point registers; a soft-float
//! image passes them in integer registers, so forcing that storage there reads
//! registers the caller never set and drops the arguments it did set.
use crate::{
    context::{ArchContext, VarnodeId},
    dtype::{type_class, type_metatype, Datatype},
    fspec::{FuncProto, ProtoModel, PrototypePieces},
    funcdata::Funcdata,
    infra::architecture::Architecture,
};
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

/// The ARM model for the base (core-register) procedure-call standard.
const SOFT_FLOAT_MODEL: &str = "__stdcall_softfp";

/// Whether the image passes floating-point arguments in floating-point
/// registers: x86 and AArch64 always do; elsewhere the container decides.
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
/// call to model recovery.  A prototype that carries no floating-point value, or
/// names its own convention, is forced as declared.  Otherwise the image decides:
/// hard-float keeps it; soft-float rebuilds it under the soft-float model, or
/// declines when the spec has none; an image that states nothing keeps it only
/// when this function computes with the model's floating-point registers.
pub fn admit(data: &Funcdata, proto: Rc<FuncProto>) -> Option<Rc<FuncProto>> {
    if !proto.has_model() || !carries_float(&proto) {
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
        None => uses_float_registers(data, proto.model()).then_some(proto),
    }
}

fn carries_float(proto: &FuncProto) -> bool {
    let output = proto.get_output().get_type().cloned();
    (0..proto.num_params())
        .filter_map(|i| proto.get_param(i).and_then(|p| p.get_type().cloned()))
        .chain(output)
        .any(|ty| holds_float(&ty, 8))
}

fn holds_float(ty: &Datatype, depth: u32) -> bool {
    match ty.get_metatype() {
        type_metatype::TYPE_FLOAT => true,
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION | type_metatype::TYPE_ARRAY
            if depth > 0 =>
        {
            (0..ty.num_depend())
                .filter_map(|i| ty.get_depend(i))
                .any(|field| holds_float(&field, depth - 1))
        }
        _ => false,
    }
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

/// Some op other than a call, return or SSA join reads or writes a register the
/// model passes floating-point values in.  Soft-float code never touches them.
fn uses_float_registers(data: &Funcdata, model: &ProtoModel) -> bool {
    let entries: Vec<(i32, u64, u64)> = model
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
        .collect();
    if entries.is_empty() {
        return false;
    }
    let float_register = |vn: VarnodeId| {
        data.vbank().get(vn).is_some_and(|v| {
            let addr: &Address = v.get_addr();
            let (Some(space), size) = (addr.get_space(), v.get_size().max(0) as u64) else {
                return false;
            };
            let offset = addr.get_offset();
            entries.iter().any(|&(index, base, len)| {
                index == space.get_index() && base < offset + size && offset < base + len
            })
        })
    };
    data.obank().iter_alive().any(|id| {
        data.obank().get(id).is_some_and(|op| {
            !matches!(
                op.code(),
                OpCode::CPUI_MULTIEQUAL
                    | OpCode::CPUI_INDIRECT
                    | OpCode::CPUI_CALL
                    | OpCode::CPUI_CALLIND
                    | OpCode::CPUI_RETURN
            ) && (op.get_out().is_some_and(float_register)
                || (0..op.num_input())
                    .filter_map(|i| op.get_in(i))
                    .any(float_register))
        })
    })
}
