//! Fix `disjoint_generic_cast`: a cast between provably distinct
//! parameterizations of one class is written to the raw class where the value
//! is returned, assigned or initialises a field, and so is a variable
//! written without a cast whose type gives the target's class another
//! parameterization (`return (Entry) nullEntry` for an `Entry<String, V>`
//! result, where `NullEntry implements Entry<Object, Object>`).

use super::is_exact;
use std::cell::Cell;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::fixes::generic_casts::raw_receiver_erasure;
use crate::kuna_static_types::{arg_equal, supertype_args};
use crate::rt::JHashMap;
use crate::var_type::{get_class, instance_of, VarType};

thread_local! {
    static DECLARED_TARGET: Cell<bool> = const { Cell::new(false) };
}

/// Marks the next value `ExprProcessor.getCastedExprent` writes as returned,
/// assigned or a field's initializer, until the returned guard drops (a
/// throw before that call leaves no mark for the next cast on the thread).
#[must_use]
pub fn mark_declared_target() -> DeclaredTarget {
    DECLARED_TARGET.with(|c| c.set(true));
    DeclaredTarget
}

/// Clears the mark [`mark_declared_target`] set when dropped.
pub struct DeclaredTarget;

impl Drop for DeclaredTarget {
    fn drop(&mut self) {
        DECLARED_TARGET.with(|c| c.set(false));
    }
}

/// Whether the value being written was marked; clears the mark, so that the
/// values nested in it are not.
pub fn take_declared_target() -> bool {
    DECLARED_TARGET.with(|c| c.replace(false))
}

/// Whether the declared target of the value being written is fixed by its
/// declaration: anywhere but in the body of a lambda, whose return type can
/// be an inference variable of the call the lambda is passed to.
pub fn fixed_target() -> bool {
    DecompilerContext::get_current_method_wrapper()
        .is_none_or(|m| !m.with(|d| d.method_struct).get_name().starts_with("lambda$"))
}

/// The raw class to cast to instead of `left`, when `right` (the value's
/// type) and `left` are parameterizations with a pair of provably distinct
/// type arguments; `named` maps the type variables in scope to their bounds.
pub fn raw_target(
    named: &JHashMap<VarType, Vec<VarType>>,
    left: &VarType,
    right: &VarType,
) -> Option<VarType> {
    let target = left.try_as_generic()?;
    if left.type_ != CodeType::Object
        || left.array_dim != 0
        || right.type_ != CodeType::Object
        || right.array_dim != 0
        || target.get_parent().is_some()
        || target.get_arguments().is_empty()
        || !instance_of(&right.value, &left.value)
    {
        return None;
    }
    let view = supertype_args(right, left.value_str())?;
    let wanted = target.get_arguments();
    if view.len() != wanted.len() {
        return None;
    }
    let disjoint = wanted.iter().zip(&view).any(|(w, v)| match (w, v) {
        (Some(w), Some(v)) => disjoint(named, w, v),
        _ => false,
    });
    disjoint.then(|| VarType::new_value(CodeType::Object, 0, left.value_str()))
}

/// The raw class `left` to cast `value`, a variable written without a cast
/// for the target `left`, to, when the type it is declared with is not
/// assignable to `left`: it gives `left`'s class a parameterization (it is
/// not raw, which would convert unchecked) with a type argument other than
/// the target's where both are types, not wildcards, which no subtyping
/// allows (JLS 4.10.2). The raw class is a supertype of the variable's type
/// and converts to `left` unchecked, and javac emits nothing for the cast.
pub fn unassignable_target(left: &VarType, value: Exprent) -> Option<VarType> {
    let right = &raw_receiver_erasure::declared_var_type(value.try_as_var()?)?;
    let target = left.try_as_generic()?;
    if left.type_ != CodeType::Object
        || left.array_dim != 0
        || right.type_ != CodeType::Object
        || right.array_dim != 0
        || target.get_parent().is_some()
        || target.get_arguments().is_empty()
        || !instance_of(&right.value, &left.value)
    {
        return None;
    }
    let view = supertype_args(right, left.value_str())?;
    let wanted = target.get_arguments();
    if view.len() != wanted.len() {
        return None;
    }
    let differs = wanted.iter().zip(&view).any(|(w, v)| match (w, v) {
        (Some(w), Some(v)) => is_exact(w) && is_exact(v) && !arg_equal(&Some(w.clone()), &Some(v.clone())),
        _ => false,
    });
    differs.then(|| VarType::new_value(CodeType::Object, 0, left.value_str()))
}

/// javac's `disjointType` for two type arguments that are not wildcards: two
/// types other than type variables unless they are the same type; a type
/// variable and a type that is not a subtype of the variable's bound with its
/// type variables made wildcards (`Comparable` raw and `C extends
/// Comparable<?>`). Wildcards and two type variables are left alone.
fn disjoint(named: &JHashMap<VarType, Vec<VarType>>, a: &VarType, b: &VarType) -> bool {
    if !is_exact(a) || !is_exact(b) {
        return false;
    }
    match (a.type_ == CodeType::Genvar, b.type_ == CodeType::Genvar) {
        (false, false) => !arg_equal(&Some(a.clone()), &Some(b.clone())),
        (true, false) => !below_bound(named, b, a),
        (false, true) => !below_bound(named, a, b),
        (true, true) => false,
    }
}

/// Whether the class type `t` is a subtype of the bound of the type variable
/// `var` with the bound's type arguments read as wildcards: of its class, and
/// not raw where the bound is parameterized. A class outside the input is
/// taken for one.
fn below_bound(named: &JHashMap<VarType, Vec<VarType>>, t: &VarType, var: &VarType) -> bool {
    let key = var.resize_array_dim(0);
    let Some(bounds) = named.get(&key) else {
        return true;
    };
    bounds.iter().all(|bound| {
        if bound.type_ != CodeType::Object || bound.value_str() == "java/lang/Object" {
            return true;
        }
        if t.type_ != CodeType::Object || t.array_dim != 0 || get_class(&t.value).is_none() {
            return true;
        }
        if !instance_of(&t.value, &bound.value) {
            return false;
        }
        let bound_args = bound
            .try_as_generic()
            .is_some_and(|g| !g.get_arguments().is_empty());
        let raw = t
            .try_as_generic()
            .is_none_or(|g| g.get_arguments().is_empty());
        !(bound_args && raw && t.value_str() == bound.value_str())
    })
}
