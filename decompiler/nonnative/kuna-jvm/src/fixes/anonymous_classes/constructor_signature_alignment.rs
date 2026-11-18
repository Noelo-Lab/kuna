//! kuna fix `constructor_signature_alignment` (`kuna_fixes`): a method's
//! `Signature` attribute may describe fewer parameters than its descriptor.
//! `MethodDescriptor.parseDescriptor` knows two conventions: javac's, whose
//! signature omits every synthetic parameter (outer instance, captured
//! variables), and one that keeps them all (kotlinc's IR backend), which it
//! trims to the real parameters. kotlinc's pre-IR backend (Kotlin 1.3 and
//! older) writes a third: a local or anonymous class constructor
//! `(Outer this$0, captured..., params...)` gets the signature
//! `(captured..., params...)`, without the outer instance only. Vineflower
//! keeps that signature as it is, `ClassWriter` then strips every synthetic
//! parameter from the descriptor and indexes the shorter list by the
//! signature's positions: `IndexOutOfBoundsException`, and the method that
//! creates the anonymous object fails. With the fix a signature that matches
//! the descriptor once its leading run of synthetic parameters is dropped,
//! each type erasing to the descriptor's, is trimmed to the real parameters,
//! as `parseDescriptor` already does for the IR convention.

use crate::code_type::CodeType;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::var_type::VarType;

/// The signature restricted to the real parameters, when `sig` describes the
/// descriptor parameters `params` after a leading run of synthetic ones
/// (`real[i]` is false for a synthetic parameter).
pub fn align(
    sig: &GenericMethodDescriptor,
    params: &[VarType],
    real: &[bool],
) -> Option<GenericMethodDescriptor> {
    let sig_params = sig.parameter_types().clone();
    if real.len() != params.len() || sig_params.len() >= params.len() {
        return None;
    }
    let skipped = params.len() - sig_params.len();
    if real[..skipped].iter().any(|&r| r) {
        return None;
    }
    let described = &params[skipped..];
    if !described
        .iter()
        .zip(sig_params.iter())
        .all(|(d, s)| erases_to(s, d))
    {
        return None;
    }
    let kept = sig_params
        .iter()
        .zip(&real[skipped..])
        .filter(|(_, &r)| r)
        .map(|(s, _)| s.clone())
        .collect();
    let d = sig.borrow();
    Some(GenericMethodDescriptor::new(
        d.type_parameters.clone(),
        d.type_parameter_bounds.clone(),
        kept,
        d.return_type.clone(),
        d.exception_types.clone(),
    ))
}

/// Whether the signature type `sig` can be the generic form of the
/// descriptor type `desc`.
fn erases_to(sig: &VarType, desc: &VarType) -> bool {
    if sig.array_dim != desc.array_dim {
        return false;
    }
    match sig.type_ {
        CodeType::Genvar => desc.type_ == CodeType::Object,
        CodeType::Object => desc.type_ == CodeType::Object && sig.value == desc.value,
        t => desc.type_ == t,
    }
}
