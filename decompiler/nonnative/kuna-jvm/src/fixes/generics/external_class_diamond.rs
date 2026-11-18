//! The kuna fix `external_class_diamond` (`kuna_fixes`), called from
//! `NewExprent::get_inferred_expr_type`.
//!
//! Vineflower learns that a constructed class is generic only from that
//! class's own `Signature`, so `new C(..)` of a class outside the class
//! universe (a library not given to the decompiler) is always written raw.
//! The expected type can prove the class generic: a parameterization
//! `C<A, ..>` of the constructed class itself. This applies to such a class
//! the step `InvocationExprent::get_inferred_expr_type` takes for a known one,
//! with the class's formal parameters taken from that type: each argument that
//! names a type usable here (not `?`, not a type variable out of scope) maps
//! its parameter; with at least one mapped, the constructor gets the diamond,
//! and the expression's type is the expected type when every argument mapped.

use crate::code_type::CodeType;
use crate::generic_type::GenericType;
use crate::new_exprent::NewExprent;
use crate::rt::JHashSet;
use crate::var_type::{str_equals, VarType};

/// The inferred type of `new`, whose class is outside the class universe,
/// when `upper_bound` makes it a generic instance creation; `None` leaves the
/// expression to Vineflower's inference.
pub fn infer(new: NewExprent, upper_bound: Option<&VarType>) -> Option<VarType> {
    let (new_type, anonymous, lambda, constructor) =
        new.d(|d| (d.new_type.clone(), d.anonymous, d.lambda, d.constructor));
    let constructor = constructor?;
    let ub = upper_bound?;
    if lambda
        || anonymous
        || new.0.is_kotlin()
        || new_type.array_dim != 0
        || ub.type_ != CodeType::Object
        || ub.array_dim != 0
        || !ub.is_generic()
        || !str_equals(&ub.value, &new_type.value)
    {
        return None;
    }
    let generic = ub.as_generic();
    let arguments = generic.get_arguments();
    if generic.get_wildcard() != GenericType::WILDCARD_NO || arguments.is_empty() {
        return None;
    }

    let named = new.0.get_named_generics();
    let mapped = arguments
        .iter()
        .flatten()
        .filter(|a| a.type_ != CodeType::Genvar || named.contains_key(a))
        .count();
    if mapped == 0 {
        return None;
    }

    constructor.d_mut(|d| {
        d.generic_args.clear();
        d.generic_args.push(GenericType::DUMMY_VAR().0);
    });
    let named_keys = JHashSet::from_collection(named.key_vec());
    if mapped == arguments.len() && !generic.has_unknown_generic_type(&named_keys) {
        Some(ub.clone())
    } else {
        Some(new.get_expr_type())
    }
}
