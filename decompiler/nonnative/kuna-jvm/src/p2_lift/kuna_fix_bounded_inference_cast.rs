//! kuna fix `bounded_inference_cast` (`kuna_fixes`): a generic method whose
//! result's type variables no parameter mentions, `<C extends Comparable>
//! Ordering<C> natural()`, gets them from its target alone (JLS 18.5.2), and
//! javac rejects the call when the target's type argument is outside the
//! variable's bound: `Ordering<E> o = Ordering.natural();` for an unbounded
//! `E` (`inference variable C has incompatible bounds`). The source casts,
//! `(Ordering<E>) Ordering.natural()`, which compiles to no instruction, and
//! Vineflower writes no cast. With the fix, a returned or assigned call whose
//! target forces such a variable to a type variable or class outside its
//! bound's class is cast to the target, or to the target's raw class when
//! the type argument is a class (provably distinct from the call's own).

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type};
use crate::kuna_static_types::supertype_args;
use crate::rt::JHashMap;
use crate::var_type::{get_class, instance_of, VarType};

/// The cast the call `exprent` needs to be assigned to `left`.
pub fn cast_for(exprent: Exprent, left: &VarType) -> Option<VarType> {
    if exprent.is_kotlin() {
        return None;
    }
    let inv = exprent.try_as_invocation()?;
    if inv.get_functype() != Type::General
        || matches!(
            inv.get_invocation_type(),
            InvocationType::Dynamic | InvocationType::ConstantDynamic
        )
        || !inv.get_generic_args().is_empty()
    {
        return None;
    }
    let target = left.try_as_generic()?;
    if left.type_ != CodeType::Object
        || left.array_dim != 0
        || target.get_parent().is_some()
        || target.get_arguments().is_empty()
    {
        return None;
    }
    let m = inv.get_desc().or_else(|| {
        DecompilerContext::get_struct_context()
            .get_class(&inv.get_classname())?
            .get_method_recursive(&inv.get_name(), &inv.get_string_descriptor())
    })?;
    let sig = m.get_signature()?;
    let own: Vec<String> = sig.type_parameters().clone();
    if own.is_empty() {
        return None;
    }
    let ret = sig.return_type()?;
    if ret.type_ != CodeType::Object || ret.array_dim != 0 || !ret.is_generic() {
        return None;
    }
    if sig
        .parameter_types()
        .iter()
        .any(|p| own.iter().any(|o| mentions(p, o)))
    {
        return None;
    }
    let view = supertype_args(&ret, left.value_str())?;
    let wanted = target.get_arguments();
    if view.len() != wanted.len() {
        return None;
    }
    let named = exprent.get_named_generics();
    let receiver = receiver_args(inv, &m.get_class_qualified_name());
    let bounds = sig.type_parameter_bounds().clone();
    let mut violated = false;
    let mut distinct = false;
    for (v, w) in view.iter().zip(wanted.iter()) {
        let (Some(v), Some(w)) = (v, w) else {
            continue;
        };
        let Some(idx) = bare_own(v, &own) else {
            continue;
        };
        let Some(x) = forced(w) else {
            continue;
        };
        let Some(var_bounds) = bounds.get(idx) else {
            continue;
        };
        if var_bounds
            .iter()
            .any(|b| !satisfies(&named, &x, b, &receiver, &own))
        {
            violated = true;
            distinct |= x.type_ != CodeType::Genvar;
        }
    }
    violated.then(|| {
        if distinct {
            VarType::new_value(CodeType::Object, 0, left.value_str())
        } else {
            left.clone()
        }
    })
}

/// The type arguments the call's receiver gives the method's class, by the
/// class's type variables; empty when unknown or static.
fn receiver_args(inv: InvocationExprent, owner: &str) -> JHashMap<String, VarType> {
    let mut out = JHashMap::new();
    let Some(instance) = inv.get_instance() else {
        return out;
    };
    let Some(cls) = get_class(&Some(owner.into())) else {
        return out;
    };
    let Some(cls_sig) = cls.get_signature() else {
        return out;
    };
    let t = instance
        .try_as_invocation()
        .and_then(standalone_type)
        .unwrap_or_else(|| instance.get_inferred_expr_type(None));
    if t.type_ != CodeType::Object || t.array_dim != 0 {
        return out;
    }
    let Some(view) = supertype_args(&t, owner) else {
        return out;
    };
    for (p, a) in cls_sig.fparameters().iter().zip(view) {
        if let Some(a) = a {
            out.put(p.clone(), a);
        }
    }
    out
}

/// The type javac gives a call of a generic method that takes no arguments
/// where nothing is expected of it (a qualifier): its result with each of
/// the method's type variables replaced by its bound's class.
fn standalone_type(inv: InvocationExprent) -> Option<VarType> {
    if !inv.get_lst_parameters().is_empty() || !inv.get_generic_args().is_empty() {
        return None;
    }
    let sig = inv.get_desc()?.get_signature()?;
    let own = sig.type_parameters().clone();
    if own.is_empty() {
        return None;
    }
    let bounds = sig.type_parameter_bounds().clone();
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    for (i, o) in own.iter().enumerate() {
        let b = bounds
            .get(i)
            .and_then(|b| b.first().cloned())
            .filter(|b| b.type_ == CodeType::Object)
            .map(|b| VarType::new_value(CodeType::Object, 0, b.value_str()))
            .unwrap_or_else(VarType::VARTYPE_OBJECT);
        map.put(GenericType::parse(&format!("T{o};")), Some(b));
    }
    sig.return_type()?.remap(&map)
}

/// The index of `v` among `own` when `v` is one of them, bare.
fn bare_own(v: &VarType, own: &[String]) -> Option<usize> {
    if v.type_ != CodeType::Genvar || v.array_dim != 0 || !is_exact(v) {
        return None;
    }
    own.iter().position(|o| o == v.value_str())
}

/// The type a target argument forces its variable to contain: itself, or
/// the lower bound of `? super X`.
fn forced(w: &VarType) -> Option<VarType> {
    match w.try_as_generic().map(|g| g.get_wildcard()) {
        None | Some(GenericType::WILDCARD_NO) => Some(w.clone()),
        Some(GenericType::WILDCARD_SUPER) => {
            Some(GenericType::with_wildcard(w, GenericType::WILDCARD_NO))
        }
        _ => None,
    }
}

/// Whether `x` lies within the class of `bound` (erasures only); a bound or
/// type kuna cannot resolve counts as satisfied.
fn satisfies(
    named: &JHashMap<VarType, Vec<VarType>>,
    x: &VarType,
    bound: &VarType,
    receiver: &JHashMap<String, VarType>,
    own: &[String],
) -> bool {
    let mut bound = bound.clone();
    if bound.type_ == CodeType::Genvar {
        if own.iter().any(|o| o == bound.value_str()) {
            return true;
        }
        match receiver.get(&bound.value_str().to_string()) {
            Some(b) => bound = b.clone(),
            None => return true,
        }
    }
    if bound.type_ != CodeType::Object
        || bound.array_dim != 0
        || bound.value_str() == "java/lang/Object"
        || get_class(&bound.value).is_none()
    {
        return true;
    }
    if x.array_dim != 0 {
        return true;
    }
    match x.type_ {
        CodeType::Genvar => {
            let Some(var_bounds) = named.get(&x.resize_array_dim(0)) else {
                return true;
            };
            var_bounds.iter().any(|b| {
                b.type_ != CodeType::Object
                    || b.value_str() != "java/lang/Object"
                        && (get_class(&b.value).is_none() || instance_of(&b.value, &bound.value))
            })
        }
        CodeType::Object => {
            x.value_str() != "java/lang/Object"
                && (get_class(&x.value).is_none() || instance_of(&x.value, &bound.value))
        }
        _ => true,
    }
}

fn mentions(t: &VarType, var: &str) -> bool {
    if t.type_ == CodeType::Genvar {
        return t.value_str() == var;
    }
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments().iter().flatten().any(|a| mentions(a, var))
            || g.get_parent().is_some_and(|p| mentions(&p, var))
    })
}

fn is_exact(t: &VarType) -> bool {
    t.try_as_generic()
        .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
}
