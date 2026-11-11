//! kuna fix `disjoint_generic_cast` (`kuna_fixes`): where a value's generic
//! type differs from its target's in a way `doGenericTypesCast` flags,
//! `ExprProcessor.getCastedExprent` writes a cast to the target's
//! parameterization. javac rejects a cast between two parameterizations of one
//! class whose type arguments are provably distinct (JLS 4.5, javac's
//! `disjointType`): `(ImmutableSet<Class<? super T>>) builder.build()` of an
//! `ImmutableSet<Class<?>>`, `(Collector<E, ?, ImmutableList<E>>) TO_LIST` of a
//! `Collector<Object, ?, ImmutableList<Object>>`, `(Range<C>) ALL` of a
//! `Range<Comparable>`; the source casts to the raw class, which compiles to
//! the same (no) instruction. With the fix such a cast is written raw where
//! the value is returned, assigned or initialises a field, whose declared type
//! takes it by unchecked conversion; an argument stays as it is (a raw
//! argument would erase the call's result type, JLS 15.12.2.6).

use std::cell::Cell;

use crate::code_type::CodeType;
use crate::generic_type::GenericType;
use crate::kuna_static_types::{arg_equal, supertype_args};
use crate::rt::JHashMap;
use crate::var_type::{get_class, instance_of, VarType};

thread_local! {
    static DECLARED_TARGET: Cell<bool> = const { Cell::new(false) };
}

/// Marks the next value `ExprProcessor.getCastedExprent` writes as returned,
/// assigned or a field's initializer.
pub fn mark_declared_target() {
    DECLARED_TARGET.with(|c| c.set(true));
}

/// Whether the value being written was marked; clears the mark, so that the
/// values nested in it are not.
pub fn take_declared_target() -> bool {
    DECLARED_TARGET.with(|c| c.replace(false))
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

fn is_exact(t: &VarType) -> bool {
    t.try_as_generic()
        .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
}
