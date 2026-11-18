//! Fix `external_supertype_arguments`: the type arguments a class gives a
//! supertype outside the input are read from the generic supertypes the known
//! classes' signatures declare.

use crate::generic_type::GenericType;
use crate::kuna_static_types::args_of;
use crate::rt::JHashMap;
use crate::var_type::{get_class, VarType};

/// The type arguments `sub` gives `sup`, a class kuna does not know, read
/// from the signatures of `sub`'s known superclasses and interfaces.
pub fn supertype_args(sub: &VarType, sup: &str) -> Option<Vec<Option<VarType>>> {
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
