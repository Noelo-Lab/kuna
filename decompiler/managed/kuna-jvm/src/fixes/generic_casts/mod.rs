//! Fix family `generic_casts`: the unchecked casts and type arguments a
//! returned, assigned or passed generic value needs. In this file,
//! `external_supertype_arguments` reads the type arguments a class gives a
//! supertype outside the input from the generic supertypes the known classes'
//! signatures declare.

pub(crate) mod argument_checkcast;
pub(crate) mod argument_generic_cast;
pub(crate) mod bounded_inference_cast;
pub(crate) mod disjoint_generic_cast;
pub(crate) mod initializer_local_type;
pub(crate) mod raw_receiver_erasure;
pub(crate) mod receiver_type_arguments;
pub(crate) mod unchecked_call_result_cast;
pub(crate) mod unchecked_generic_cast;
pub(crate) mod uncontained_type_argument;

use crate::code_type::CodeType;
use crate::generic_type::GenericType;
use crate::kuna_static_types::args_of;
use crate::rt::JHashMap;
use crate::var_type::{get_class, VarType};

/// Whether the type variable `var` occurs in `t`.
pub(crate) fn mentions(t: &VarType, var: &str) -> bool {
    if t.type_ == CodeType::Genvar {
        return t.value_str() == var;
    }
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments().iter().flatten().any(|a| mentions(a, var))
            || g.get_parent().is_some_and(|p| mentions(&p, var))
    })
}

/// Whether `sub` and `sup` are type variables (no array, no wildcard) and
/// `sup` is a bound of `sub`, or of a type variable that bounds it, by the
/// bounds `named` gives the variables in scope: `sub` is then a subtype of
/// `sup` (JLS 4.10.2) and converts to it without a cast.
pub(crate) fn variable_below(named: &JHashMap<VarType, Vec<VarType>>, sub: &VarType, sup: &VarType) -> bool {
    let bare = |t: &VarType| t.type_ == CodeType::Genvar && t.array_dim == 0 && is_exact(t);
    if !bare(sub) || !bare(sup) {
        return false;
    }
    let mut pending = vec![sub.clone()];
    let mut seen: Vec<VarType> = Vec::new();
    while let Some(v) = pending.pop() {
        if seen.iter().any(|s| s.value == v.value) {
            continue;
        }
        let Some(bounds) = named.get(&v) else {
            continue;
        };
        for b in bounds.iter().filter(|b| bare(b)) {
            if b.value == sup.value {
                return true;
            }
            pending.push(b.clone());
        }
        seen.push(v);
    }
    false
}

/// Whether the cast to `cast` of `value`'s operand, whose type is `right`,
/// is javac's cast of an array of a type variable to its erasure: `right`
/// is an array of a type variable in scope with a bound of the cast's
/// class, and `cast` that class's array of the same dimensions. The
/// operand already has the type the cast would erase, so the cast is not
/// written (`for (K e : c.getEnumConstants())` for a `Class<K> c` and `K
/// extends Enum<K>`, which `(Enum[])` turned into a loop over `Enum`).
pub(crate) fn variable_array_erasure(value: crate::exprent::Exprent, right: &VarType, cast: &[VarType]) -> bool {
    if crate::fixes::kt_util::writing_kotlin()
        || right.type_ != CodeType::Genvar
        || right.array_dim == 0
        || !is_exact(right)
        || cast.len() != 1
        || cast[0].type_ != CodeType::Object
        || cast[0].array_dim != right.array_dim
    {
        return false;
    }
    value
        .get_named_generics()
        .get(&right.resize_array_dim(0))
        .is_some_and(|bounds| bounds.iter().any(|b| b.type_ == CodeType::Object && b.array_dim == 0 && b.value == cast[0].value))
}

/// A type argument that is a type, not a wildcard.
fn is_exact(t: &VarType) -> bool {
    t.try_as_generic()
        .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
}

/// The type arguments `sub` gives `sup`, a class kuna does not know, read
/// from the signatures of `sub`'s known superclasses and interfaces.
pub fn external_supertype_args(sub: &VarType, sup: &str) -> Option<Vec<Option<VarType>>> {
    walk(sub, sup, 0)
}

fn walk(sub: &VarType, sup: &str, depth: usize) -> Option<Vec<Option<VarType>>> {
    if depth > 16 {
        return None;
    }
    let cls = get_class(&sub.value)?;
    let sig = cls.get_signature()?;
    let formals: Vec<VarType> = sig
        .fparameters()
        .iter()
        .map(|p| GenericType::parse(&format!("T{};", p)))
        .collect();
    let args = args_of(sub);
    if args.len() != formals.len() || args.iter().any(Option::is_none) {
        return None;
    }
    let mut subst: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    for (p, a) in formals.iter().zip(&args) {
        subst.put(p.clone(), a.clone());
    }
    let supers: Vec<VarType> = std::iter::once(sig.superclass())
        .chain(sig.superinterfaces().iter().cloned())
        .collect();
    for t in supers {
        let Some(t) = t.remap(&subst) else {
            continue;
        };
        if t.value.as_deref() == Some(sup) {
            let found = args_of(&t);
            return (!found.is_empty()).then_some(found);
        }
        if get_class(&t.value).is_some() {
            if let Some(found) = walk(&t, sup, depth + 1) {
                return Some(found);
            }
        }
    }
    None
}
