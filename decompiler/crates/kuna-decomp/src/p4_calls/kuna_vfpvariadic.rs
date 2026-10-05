//! A declared variadic function's return value on an ARM image that states the
//! VFP procedure-call standard.  AAPCS-VFP lays a variadic function out under the
//! base standard, its result included: a `float` comes back in `r0`, a `double`
//! in `r0:r1`, a homogeneous floating-point aggregate of four bytes in `r0` and a
//! larger one in memory through a hidden pointer.  The spec's default model sends
//! a variadic function's arguments to core registers (its `<varargs/>` input
//! rules) but its output list has no such rule, so `double vr(int n, ...)` was
//! read from `d0`.  A declaration that names no convention, has a variadic tail
//! and returns a floating-point or aggregate value is laid out under the default
//! model with those output rules in front.
//!
//! An image that states the base standard (`softfp`) or no convention keeps the
//! default layout: there a function's own declared floating-point return still
//! sits in a VFP register, and moving only its variadic callee's would break
//! `return vr(...)`.
use crate::{
    dtype::type_metatype,
    fspec::{FuncProto, ProtoModel, PrototypePieces},
    infra::architecture::Architecture,
    modelrules::{register_ids, ModelRule},
};
use kuna_base::address::Address;
use kuna_base::marshal::{IdRegistry, XmlDecode};
use kuna_base::space::AddrSpaceManager;
use kuna_base::types::int4;
use std::rc::Rc;

const OUTPUT_RULES: &str = concat!(
    "<output>",
    "<rule><datatype name=\"float\"/><varargs/><join/></rule>",
    "<rule><datatype name=\"struct\" minsize=\"5\"/><varargs/><hidden_return/></rule>",
    "<rule><datatype name=\"homogeneous-float-aggregate\"/><varargs/><join/></rule>",
    "</output>"
);

/// The image is ARM and states the VFP variant.
pub fn applies(arch: &Architecture) -> bool {
    arch.archid.starts_with("ARM:") && crate::kuna_typedcallabi::image_evidence(arch) == Some(true)
}

/// A variadic declaration whose return value the base standard may place
/// outside the default model's storage for it.
pub fn moves_return(pieces: &PrototypePieces) -> bool {
    pieces.first_var_arg_slot >= 0
        && pieces.outtype.as_ref().is_some_and(|ty| {
            matches!(
                ty.get_metatype(),
                type_metatype::TYPE_FLOAT | type_metatype::TYPE_STRUCT | type_metatype::TYPE_ARRAY
            )
        })
}

/// The architecture's default model with the variadic return rules, derived
/// once per default model.
pub fn declarations(arch: &Architecture) -> Option<Rc<ProtoModel>> {
    let base = arch.default_fp()?;
    let mut cache = arch.kuna_vfp_variadic.borrow_mut();
    if let Some((key, derived)) = cache.as_ref() {
        if Rc::ptr_eq(key, base) {
            return derived.clone();
        }
    }
    let derived = model(base, arch.manage());
    *cache = Some((Rc::clone(base), derived.clone()));
    derived
}

/// `base` with the variadic return rules ahead of its own.
pub fn model(base: &Rc<ProtoModel>, manager: &AddrSpaceManager) -> Option<Rc<ProtoModel>> {
    if base.is_merged() {
        return None;
    }
    let doc = kuna_base::xml::xml_tree(OUTPUT_RULES.as_bytes()).ok()?;
    let mut registry = IdRegistry::with_base_ids();
    register_ids(&mut registry);
    let mut result = (**base).clone();
    let rules = doc
        .get_root()
        .get_children()
        .iter()
        .map(|rule| {
            let mut decoder = XmlDecode::new_with_root(manager, &registry, rule, 0);
            ModelRule::decode(&mut decoder, result.output())
        })
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    result.output_mut().kuna_prepend_model_rules(rules);
    Some(Rc::new(result))
}

/// Would a recovered prototype parked with an open tail have its return value
/// moved by the variadic rules?  Its callers lay a parked list out as a
/// declaration, so the open tail `protoorder` gives every parked list would
/// re-bind a recovered `float` return to `r0`.
pub fn moves_parked_return(arch: &Architecture, pieces: &PrototypePieces) -> bool {
    if !moves_return(pieces) || !applies(arch) {
        return false;
    }
    let (Some(base), Some(variadic)) = (arch.default_fp(), declarations(arch)) else { return false };
    output(arch, pieces, base) != output(arch, pieces, &variadic)
}

fn output(arch: &Architecture, pieces: &PrototypePieces, model: &Rc<ProtoModel>) -> Option<(Address, int4)> {
    let void_ty = arch.types().get_type_void().ok()?;
    let types_only = PrototypePieces { input_storage: Vec::new(), output_storage: None, ..pieces.clone() };
    let mut fp = FuncProto::new();
    fp.seed_locked_from_pieces(&types_only, Rc::clone(model), void_ty, arch.types(), arch.manage()).ok()?;
    let out = fp.get_output();
    Some((out.get_address(), out.get_size()))
}
