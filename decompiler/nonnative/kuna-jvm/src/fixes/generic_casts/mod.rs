//! Fix family `generic_casts`: the unchecked casts and type arguments a
//! returned, assigned or passed generic value needs.

pub(crate) mod argument_generic_cast;
pub(crate) mod bounded_inference_cast;
pub(crate) mod disjoint_generic_cast;
pub(crate) mod external_supertype_arguments;
pub(crate) mod raw_receiver_erasure;
pub(crate) mod receiver_type_arguments;
pub(crate) mod unchecked_call_result_cast;
pub(crate) mod unchecked_generic_cast;

use crate::code_type::CodeType;
use crate::generic_type::GenericType;
use crate::var_type::VarType;

/// Whether the type variable `var` occurs in `t`.
fn mentions(t: &VarType, var: &str) -> bool {
    if t.type_ == CodeType::Genvar {
        return t.value_str() == var;
    }
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments().iter().flatten().any(|a| mentions(a, var))
            || g.get_parent().is_some_and(|p| mentions(&p, var))
    })
}

/// A type argument that is a type, not a wildcard.
fn is_exact(t: &VarType) -> bool {
    t.try_as_generic()
        .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
}
