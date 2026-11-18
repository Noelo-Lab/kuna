//! Fix `unchecked_call_result_cast`: a call's declared result type (its generic
//! return type with the receiver's type arguments) takes part in
//! `unchecked_generic_cast`'s test.

use super::mentions;
use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type};
use crate::kuna_static_types::{formals, is_raw_type, supertype_args};
use crate::rt::{JHashMap, JHashSet};
use crate::var_type::VarType;

/// The declared type of `inv`'s result, `declared` giving its receiver's.
pub fn declared_result(
    inv: InvocationExprent,
    declared: fn(Exprent) -> Option<VarType>,
) -> Option<VarType> {
    if inv.get_functype() != Type::General
        || matches!(
            inv.get_invocation_type(),
            InvocationType::Dynamic | InvocationType::ConstantDynamic
        )
    {
        return None;
    }
    let Some(m) = inv.d(|d| d.desc).or_else(|| {
        DecompilerContext::get_struct_context()
            .get_class(&inv.get_classname())?
            .get_method_recursive(&inv.get_name(), &inv.get_string_descriptor())
    }) else {
        let r = inv.get_descriptor().ret.clone();
        return (r.type_ == CodeType::Object
            && r.array_dim == 0
            && formals(r.value_str()).is_some_and(|f| f.is_empty()))
        .then_some(r);
    };
    let ret = match m.get_signature() {
        Some(sig) => {
            let r = sig.return_type()?;
            if sig.type_parameters().iter().any(|p| mentions(&r, p)) {
                return None;
            }
            r
        }
        None => {
            let r = inv.get_descriptor().ret.clone();
            return (r.type_ == CodeType::Object && !is_raw_type(&r)).then_some(r);
        }
    };
    if ret.type_ != CodeType::Object || ret.array_dim != 0 {
        return None;
    }
    let owner = m.get_class_qualified_name();
    let class_vars = formals(&owner)?;
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    if class_vars.iter().any(|v| mentions(&ret, v.value_str())) {
        if m.has_modifier(CodeConstants::ACC_STATIC) {
            return None;
        }
        let receiver = declared(inv.get_instance()?)?;
        if receiver.type_ != CodeType::Object || receiver.array_dim != 0 || is_raw_type(&receiver) {
            return None;
        }
        let view = supertype_args(&receiver, &owner)?;
        if view.len() != class_vars.len() {
            return None;
        }
        for (v, a) in class_vars.iter().zip(view) {
            let wild = a.as_ref().is_none_or(|a| {
                a.try_as_generic()
                    .is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO)
            });
            if wild && !only_top_level(&ret, v.value_str()) {
                return None;
            }
            map.put(v.clone(), a);
        }
    }
    let result = ret.remap(&map)?;
    let named = JHashSet::from_collection(inv.0.get_named_generics().key_vec());
    (!(result.is_generic() && result.as_generic().has_unknown_generic_type(&named)))
        .then_some(result)
}

/// Whether `var` occurs in `t` only as one of its own type arguments, where
/// a wildcard can stand for it.
fn only_top_level(t: &VarType, var: &str) -> bool {
    let Some(g) = t.try_as_generic() else {
        return true;
    };
    !g.get_parent().is_some_and(|p| mentions(&p, var))
        && g.get_arguments().iter().flatten().all(|a| {
            (a.type_ == CodeType::Genvar
                && a.try_as_generic()
                    .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO))
                || !mentions(a, var)
        })
}
