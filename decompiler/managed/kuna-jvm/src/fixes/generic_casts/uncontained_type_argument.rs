//! Fix `uncontained_type_argument`: a variable (or `this`) whose declared
//! type is a parameterization of the target's class is not assignable to
//! the target when the target has `? extends B` where the variable's type
//! has a type variable that is not a subtype of `B` (JLS 4.5.1, 4.10.2): no
//! bound of it, through the type variables that bound it, has `B`'s class
//! or a subclass. javac compiles the cast the source needs there to no
//! instruction, since both erase alike, so it is written from the types:
//! `return (Node<? extends Node<T>>) this` for a `Node<T>` with `T` bounded
//! by `Object`.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::rt::JHashMap;
use crate::var_type::{get_class, instance_of, VarType};

/// Whether `value`, written with the type `right`, needs a cast to `left`
/// by the module header's rule.
pub fn needs_cast(value: Exprent, left: &VarType, right: &VarType) -> bool {
    if value.try_as_var().is_none()
        || left.type_ != CodeType::Object
        || right.type_ != CodeType::Object
        || left.array_dim != 0
        || right.array_dim != 0
        || left.value != right.value
    {
        return false;
    }
    let (Some(l), Some(r)) = (left.try_as_generic(), right.try_as_generic()) else {
        return false;
    };
    let (la, ra) = (l.get_arguments(), r.get_arguments());
    if la.len() != ra.len() || la.is_empty() {
        return false;
    }
    let named = value.get_named_generics();
    la.iter().zip(ra.iter()).any(|(a, b)| match (a, b) {
        (Some(a), Some(b)) => outside_upper_bound(&named, a, b),
        _ => false,
    })
}

/// Whether `arg` is `? extends B` for a class `B` other than `Object` and
/// `var` a type variable in scope, without wildcard, none of whose bounds,
/// through the type variables among them, is of `B`'s class (each bound
/// that is a class known, or `Object`).
fn outside_upper_bound(named: &JHashMap<VarType, Vec<VarType>>, arg: &VarType, var: &VarType) -> bool {
    let Some(g) = arg.try_as_generic() else {
        return false;
    };
    if g.get_wildcard() != GenericType::WILDCARD_EXTENDS
        || arg.type_ != CodeType::Object
        || arg.array_dim != 0
        || arg.value_str() == "java/lang/Object"
    {
        return false;
    }
    let bare = |t: &VarType| {
        t.type_ == CodeType::Genvar
            && t.array_dim == 0
            && t.try_as_generic().is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
    };
    if !bare(var) || !named.contains_key(var) {
        return false;
    }
    let mut pending = vec![var.clone()];
    let mut seen: Vec<VarType> = Vec::new();
    while let Some(v) = pending.pop() {
        if seen.iter().any(|s| s.value == v.value) {
            continue;
        }
        let Some(bounds) = named.get(&v) else {
            return false;
        };
        for b in bounds {
            if bare(b) {
                pending.push(b.clone());
            } else if b.type_ != CodeType::Object
                || b.array_dim != 0
                || (b.value_str() != "java/lang/Object" && get_class(&b.value).is_none())
                || instance_of(&b.value, &arg.value)
            {
                return false;
            }
        }
        seen.push(v);
    }
    true
}
