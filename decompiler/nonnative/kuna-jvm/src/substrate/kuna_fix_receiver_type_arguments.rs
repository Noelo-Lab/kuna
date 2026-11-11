//! kuna fix `receiver_type_arguments` (`kuna_fixes`): a generic call or
//! diamond creation that is the qualifier of another call has no target type
//! (JLS 15.12), so javac infers its type arguments from its own arguments
//! alone: `List.of(MODE).iterator()` returned as an `Iterator<Setting<?>>`
//! infers `E` as `Setting<IndexMode>`, `new Builder<>(comparator).build()`
//! returned as an `ImmutableSortedSet<E>` infers a capture of
//! `? super E`, and javac rejects the result (`incompatible types`). The
//! source writes the type arguments, `List.<Setting<?>>of(MODE)`, which
//! compile to no instruction; Vineflower writes none, or a raw `new`. With
//! the fix, a returned or assigned chain of calls whose results are typed by
//! their classes' type variables is traced back from its target to the
//! generic call or creation it starts with, and that one gets the explicit
//! type arguments the target needs, unless its arguments already give
//! javac exactly those, or one of them could not take them.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type};
use crate::kuna_fix_lambda_pertinence;
use crate::kuna_static_types::{arg_equal, formals, supertype_args};
use crate::new_exprent::NewExprent;
use crate::rt::{JHashMap, JHashSet};
use crate::struct_method::StructMethod;
use crate::var_type::{get_class, instance_of, VarType};

/// Traces the chain `exprent` is the outermost call of back from `left`,
/// its declared target, and gives the call or creation it starts with the
/// type arguments the target needs.
pub fn apply(exprent: Exprent, left: &VarType) {
    if !is_class_type(left) || !left.is_generic() {
        return;
    }
    let mut needed = left.clone();
    let mut cur = exprent;
    for depth in 0..32 {
        let Some(inv) = cur.try_as_invocation() else {
            return;
        };
        let Some((m, ret, own)) = plain_method(inv) else {
            return;
        };
        if own.iter().any(|o| mentions(&ret, o)) {
            if depth > 0 {
                bottom_call(inv, &m, &ret, &own, &needed);
            }
            return;
        }
        let Some(instance) = inv.get_instance() else {
            return;
        };
        let owner = m.get_class_qualified_name();
        let Some(next) = receiver_needed(&owner, &ret, &needed) else {
            return;
        };
        if let Some(new) = instance.try_as_new() {
            bottom_new(new, &next);
            return;
        }
        needed = next;
        cur = instance;
    }
}

/// The type the receiver of a method of `owner` returning `ret` must have
/// for the call to produce `needed`.
fn receiver_needed(owner: &str, ret: &VarType, needed: &VarType) -> Option<VarType> {
    let fs = formals(owner)?;
    if fs.is_empty() {
        return None;
    }
    let names: Vec<String> = fs.iter().map(|f| f.value_str().to_string()).collect();
    let map = unify_view(ret, needed, &names)?;
    let args = names
        .iter()
        .map(|n| map.get(n).cloned())
        .collect::<Option<Vec<VarType>>>()?;
    Some(GenericType::new(CodeType::Object, 0, owner, None, args.into_iter().map(Some).collect(), GenericType::WILDCARD_NO).0)
}

/// The method `inv` calls, its generic result and its own type parameters,
/// for an ordinary call of a method kuna knows the signature of.
fn plain_method(inv: InvocationExprent) -> Option<(StructMethod, VarType, Vec<String>)> {
    if inv.get_functype() != Type::General
        || matches!(
            inv.get_invocation_type(),
            InvocationType::Dynamic | InvocationType::ConstantDynamic
        )
    {
        return None;
    }
    let m = inv.get_desc().or_else(|| {
        DecompilerContext::get_struct_context()
            .get_class(&inv.get_classname())?
            .get_method_recursive(&inv.get_name(), &inv.get_string_descriptor())
    })?;
    let sig = m.get_signature()?;
    let ret = sig.return_type()?;
    if !is_class_type(&ret) || !ret.is_generic() {
        return None;
    }
    let own = sig.type_parameters().clone();
    Some((m, ret, own))
}

/// The generic call a chain starts with: its own type variables from the
/// target, when its arguments do not give javac exactly those.
fn bottom_call(inv: InvocationExprent, m: &StructMethod, ret: &VarType, own: &[String], needed: &VarType) {
    let Some(map) = unify_view(ret, needed, own) else {
        return;
    };
    let Some(args) = own
        .iter()
        .map(|o| map.get(o).cloned())
        .collect::<Option<Vec<VarType>>>()
    else {
        return;
    };
    let Some(sig) = m.get_signature() else {
        return;
    };
    let params = sig.parameter_types().clone();
    let values: Vec<Exprent> = inv.get_lst_parameters().iter().collect();
    if !settle(&args, own, &params, &values, inv.0) {
        return;
    }
    let current = inv.get_generic_args();
    if current.len() == args.len() && current.iter().zip(&args).all(|(c, a)| arg_equal(&Some(c.clone()), &Some(a.clone()))) {
        return;
    }
    inv.d_mut(|d| d.kuna_chain_type_args = Some(args));
}

/// The creation a chain starts with: its class's type arguments from the
/// target, for a diamond or raw `new` of a generic class.
fn bottom_new(new: NewExprent, needed: &VarType) {
    if new.is_lambda() || new.is_anonymous() {
        return;
    }
    let Some(ctor) = new.get_constructor() else {
        return;
    };
    let new_type = new.get_new_type();
    if new_type.array_dim != 0 {
        return;
    }
    let Some(cls) = get_class(&new_type.value) else {
        return;
    };
    let Some(cls_sig) = cls.get_signature() else {
        return;
    };
    let own: Vec<String> = cls_sig.fparameters().clone();
    if own.is_empty() {
        return;
    }
    let Some(map) = unify_view(&cls_sig.generic_type().0, needed, &own) else {
        return;
    };
    let Some(args) = own
        .iter()
        .map(|o| map.get(o).cloned())
        .collect::<Option<Vec<VarType>>>()
    else {
        return;
    };
    let params = ctor
        .get_desc()
        .and_then(|d| d.get_signature())
        .map(|s| s.parameter_types().clone())
        .unwrap_or_default();
    let values: Vec<Exprent> = ctor.get_lst_parameters().iter().collect();
    if !settle(&args, &own, &params, &values, ctor.0) {
        return;
    }
    ctor.d_mut(|d| d.kuna_chain_type_args = Some(args));
}

fn named(e: Exprent) -> JHashSet<VarType> {
    JHashSet::from_collection(e.get_named_generics().key_vec())
}

/// Whether the type arguments `args` for `own` can be written at a call
/// whose formals are `params` and argument values `values`, and are needed:
/// every argument whose formal mentions one of `own` is a variable, field or
/// constant (whose type javac reads as kuna does), one with a bare variable
/// for its formal is compatible with that variable's type argument, and the
/// arguments do not bind every variable to exactly its type argument.
fn settle(args: &[VarType], own: &[String], params: &[VarType], values: &[Exprent], at: Exprent) -> bool {
    if !kuna_fix_lambda_pertinence::writable(args, &named(at)) || params.len() != values.len() {
        return false;
    }
    let mut bindings: Vec<Vec<VarType>> = vec![Vec::new(); own.len()];
    for (p, v) in params.iter().zip(values) {
        if !own.iter().any(|o| mentions(p, o)) {
            continue;
        }
        let Some(t) = plain_value_type(*v) else {
            return false;
        };
        let bare = p.type_ == CodeType::Genvar && is_exact(p);
        if let Some(i) = own.iter().position(|o| bare && o == p.value_str()) {
            let want = args[i].resize_array_dim(args[i].array_dim + p.array_dim);
            if !compatible(&t, &want) {
                return false;
            }
        }
        let mut map = JHashMap::new();
        if bounds_of(p, &t, own, &mut map) {
            for (i, o) in own.iter().enumerate() {
                if let Some(x) = map.get(o) {
                    bindings[i].push(x.clone());
                }
            }
        }
    }
    !bindings.iter().zip(args).all(|(b, a)| {
        !b.is_empty() && b.iter().all(|x| arg_equal(&Some(x.clone()), &Some(a.clone())))
    })
}

/// The types an argument of type `a` passed for the formal `p` binds the
/// variables `vars` to, as javac's inference resolves a single bound: a
/// variable in a type argument position, bare or under a wildcard, takes the
/// argument's corresponding type when that one is not a wildcard (whose
/// capture javac would bind instead).
fn bounds_of(p: &VarType, a: &VarType, vars: &[String], map: &mut JHashMap<String, VarType>) -> bool {
    if p.type_ == CodeType::Genvar && vars.iter().any(|v| v == p.value_str()) {
        if !is_exact(a) || a.array_dim < p.array_dim {
            return false;
        }
        map.put(p.value_str().to_string(), a.resize_array_dim(a.array_dim - p.array_dim));
        return true;
    }
    let Some(gp) = p.try_as_generic() else {
        return false;
    };
    if p.type_ != CodeType::Object || p.array_dim != 0 || a.type_ != CodeType::Object || a.array_dim != 0 {
        return false;
    }
    let Some(view) = supertype_args(a, p.value_str()) else {
        return false;
    };
    let pa = gp.get_arguments();
    if pa.len() != view.len() {
        return false;
    }
    for (x, y) in pa.iter().zip(&view) {
        let (Some(x), Some(y)) = (x, y) else {
            continue;
        };
        if !vars.iter().any(|v| mentions(x, v)) || !is_exact(y) {
            continue;
        }
        let x = if is_exact(x) {
            x.clone()
        } else {
            GenericType::with_wildcard(x, GenericType::WILDCARD_NO)
        };
        bounds_of(&x, y, vars, map);
    }
    true
}

/// The type of a variable, field or constant argument; other values are
/// not typed here, since typing a call or creation recomputes the type
/// arguments it is written with.
fn plain_value_type(v: Exprent) -> Option<VarType> {
    (v.try_as_var().is_some() || v.try_as_field().is_some() || v.try_as_const().is_some())
        .then(|| v.get_inferred_expr_type(None))
}

/// Whether a value of type `t` can be passed where `want` is expected, as far
/// as kuna can tell.
fn compatible(t: &VarType, want: &VarType) -> bool {
    if t.type_ == CodeType::Null {
        return true;
    }
    if t.type_ != CodeType::Object || want.type_ != CodeType::Object || t.array_dim != want.array_dim {
        return false;
    }
    if get_class(&t.value).is_none() || !instance_of(&t.value, &want.value) {
        return false;
    }
    let Some(g) = want.try_as_generic().filter(|g| !g.get_arguments().is_empty()) else {
        return true;
    };
    let Some(view) = supertype_args(&t.resize_array_dim(0), want.value_str()) else {
        return false;
    };
    let wanted = g.get_arguments();
    view.len() == wanted.len()
        && wanted.iter().zip(&view).all(|(w, v)| match w {
            None => true,
            Some(w) => match w.try_as_generic().map(|g| g.get_wildcard()) {
                Some(GenericType::WILDCARD_UNBOUND) => true,
                Some(GenericType::WILDCARD_EXTENDS) | Some(GenericType::WILDCARD_SUPER) => {
                    w.value_str() == "java/lang/Object" && w.type_ == CodeType::Object
                }
                _ => arg_equal(&Some(w.clone()), v),
            },
        })
}

/// The bindings of `vars` under which `ret`, seen as `needed`'s class, is
/// `needed`.
fn unify_view(ret: &VarType, needed: &VarType, vars: &[String]) -> Option<JHashMap<String, VarType>> {
    let view = supertype_args(ret, needed.value_str())?;
    let wanted = needed.try_as_generic()?.get_arguments().to_vec();
    if view.len() != wanted.len() {
        return None;
    }
    let mut map = JHashMap::new();
    for (v, w) in view.iter().zip(&wanted) {
        match (v, w) {
            (None, None) => {}
            (Some(v), Some(w)) if unify(v, w, vars, &mut map) => {}
            _ => return None,
        }
    }
    Some(map)
}

/// Matches the pattern `p`, whose variables in `vars` are unknowns, with the
/// type `a`, binding the unknowns.
fn unify(p: &VarType, a: &VarType, vars: &[String], map: &mut JHashMap<String, VarType>) -> bool {
    if p.type_ == CodeType::Genvar && vars.iter().any(|v| v == p.value_str()) {
        if !is_exact(p) || !is_exact(a) || a.array_dim < p.array_dim {
            return false;
        }
        let bound = a.resize_array_dim(a.array_dim - p.array_dim);
        let key = p.value_str().to_string();
        if let Some(prev) = map.get(&key) {
            return arg_equal(&Some(prev.clone()), &Some(bound));
        }
        map.put(key, bound);
        return true;
    }
    if !vars.iter().any(|v| mentions(p, v)) {
        return arg_equal(&Some(p.clone()), &Some(a.clone()));
    }
    let (Some(gp), Some(ga)) = (p.try_as_generic(), a.try_as_generic()) else {
        return false;
    };
    if p.type_ != a.type_
        || p.array_dim != a.array_dim
        || p.value_str() != a.value_str()
        || gp.get_wildcard() != ga.get_wildcard()
        || gp.get_parent().is_some()
        || ga.get_parent().is_some()
    {
        return false;
    }
    let (pa, aa) = (gp.get_arguments(), ga.get_arguments());
    pa.len() == aa.len()
        && pa.iter().zip(aa.iter()).all(|(x, y)| match (x, y) {
            (Some(x), Some(y)) => unify(x, y, vars, map),
            (None, None) => true,
            _ => false,
        })
}

fn is_class_type(t: &VarType) -> bool {
    t.type_ == CodeType::Object && t.array_dim == 0
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
