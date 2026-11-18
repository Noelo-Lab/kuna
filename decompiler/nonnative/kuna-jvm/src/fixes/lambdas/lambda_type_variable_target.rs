//! kuna fix `lambda_type_variable_target` (`kuna_fixes`): a lambda or method
//! reference passed where a generic method takes a bare type variable, as
//! `single(T element)`, gives javac no constraint on that variable (JLS
//! 18.5.2): the variable is resolved from its other bounds first and the
//! lambda is then checked against the result. When the call's result is in
//! turn an argument of a generic method, as `concat(single((b, p) -> ...),
//! chunks)` for `<T> It<T> concat(It<? extends T>... parts)`, the other
//! arguments give that method's variable proper lower bounds, and unless
//! the target type fixes it, it resolves to their least upper bound
//! (`Leaf`, or the capture of `? extends Chunk`), which is not a functional
//! interface: javac rejects the lambda. `InvocationExprent.getInferredExprType`
//! takes the lambda's own interface for the variable, so Vineflower writes
//! the lambda bare, without the cast the source needed. With the fix the
//! outer call works out the variable's bounds from its other arguments and
//! its declared target (a method's generic return type, or the function
//! type's return type in a lambda body) and, when they resolve it to a type
//! other than the lambda's interface, the lambda is cast to its interface,
//! `single((Chunk)((b, p) -> ...))`; the cast compiles to no instruction.

use crate::classes_processor::ClassNodeType;
use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::InvocationExprent;
use crate::kuna_static_types::{arg_equal, supertype_args};
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

/// The method `inv` calls, with its generic signature.
fn method(inv: InvocationExprent) -> Option<StructMethod> {
    let m = inv.d(|d| d.desc).or_else(|| {
        DecompilerContext::get_struct_context()
            .get_class(&inv.get_classname())?
            .get_method_recursive(&inv.get_name(), &inv.get_string_descriptor())
    })?;
    m.get_signature()
        .is_some_and(|s| !s.type_parameters().is_empty())
        .then_some(m)
}

fn type_params(m: StructMethod) -> Vec<String> {
    m.get_signature()
        .map(|s| s.type_parameters().clone())
        .unwrap_or_default()
}

fn is_param(t: &VarType, params: &[String]) -> bool {
    t.type_ == CodeType::Genvar
        && t.array_dim == 0
        && t.try_as_generic()
            .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
        && t.value
            .as_deref()
            .is_some_and(|v| params.iter().any(|p| p == v))
}

fn is_lambda(e: Exprent) -> bool {
    e.try_as_new().is_some_and(|n| n.is_lambda())
}

/// The arguments of `inv` with their formal types; a variable-arity call's
/// array of trailing arguments is spread into its elements.
fn arguments(inv: InvocationExprent, m: StructMethod) -> Option<Vec<(VarType, Exprent)>> {
    let sig = m.get_signature()?;
    let formals = sig.parameter_types().clone();
    let args = inv.get_lst_parameters().to_vec();
    if formals.len() != args.len() || inv.get_functype() != crate::invocation_exprent::Type::General
    {
        return None;
    }
    let mut out = Vec::new();
    for (i, (f, a)) in formals.into_iter().zip(args).enumerate() {
        let spread = i + 1 == inv.get_lst_parameters().len()
            && m.has_modifier(CodeConstants::ACC_VARARGS)
            && f.array_dim == 1
            && a.try_as_new().is_some_and(|n| n.is_var_arg_param());
        if spread {
            let element = f.resize_array_dim(0);
            for e in a.as_new().get_lst_array_elements().to_vec() {
                out.push((element.clone(), e));
            }
        } else {
            out.push((f, a));
        }
    }
    Some(out)
}

/// A call whose result type has a bare type variable of its method at one
/// type-argument place, that variable fixed by nothing but lambda and
/// method-reference arguments passed for it bare: `single(lambda)`. Its
/// result class and that place.
fn lambda_only(e: Exprent) -> Option<(String, usize, InvocationExprent)> {
    let n = e.try_as_invocation()?;
    if !n.get_generic_args().is_empty() {
        return None;
    }
    let m = method(n)?;
    let params = type_params(m);
    let sig = m.get_signature()?;
    let ret = sig.return_type()?;
    let ret_args = ret.try_as_generic()?.get_arguments().to_vec();
    let mut place = None;
    for (k, a) in ret_args.iter().enumerate() {
        if a.as_ref().is_some_and(|a| is_param(a, &params)) {
            if place.is_some() {
                return None;
            }
            place = Some(k);
        }
    }
    let k = place?;
    let alpha = ret_args[k].clone()?;
    if sig.type_parameters().len() != sig.type_parameter_bounds().len() {
        return None;
    }
    let ai = params
        .iter()
        .position(|p| Some(p.as_str()) == alpha.value.as_deref())?;
    if sig.type_parameter_bounds()[ai]
        .iter()
        .any(|b| !b.equals(&VarType::VARTYPE_OBJECT()))
    {
        return None;
    }
    let mut lambdas = 0;
    for (f, a) in arguments(n, m)? {
        let mentions = mentions(&f, &alpha);
        if !mentions {
            continue;
        }
        if !is_lambda(a) {
            return None;
        }
        if f.equals(&alpha) {
            lambdas += 1;
        }
    }
    (lambdas > 0).then(|| (ret.value_str().to_string(), k, n))
}

fn mentions(t: &VarType, var: &VarType) -> bool {
    if t.type_ == CodeType::Genvar {
        return t.value == var.value;
    }
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments().iter().flatten().any(|a| mentions(a, var))
            || g.get_parent().is_some_and(|p| mentions(&p, var))
    })
}

#[derive(Default)]
struct Bounds {
    equal: Vec<VarType>,
    lower: Vec<VarType>,
    captures: usize,
    upper: Vec<VarType>,
}

/// Marks the calls among `m`'s arguments whose lambda arguments javac
/// cannot type without a cast (`InvocationExprent::kuna_cast_lambdas`).
pub fn mark(m: InvocationExprent) {
    let Some(method) = method(m) else {
        return;
    };
    let params = type_params(method);
    let Some(args) = arguments(m, method) else {
        return;
    };
    for (f, a) in &args {
        let Some((class, k, n)) = lambda_only(*a) else {
            continue;
        };
        let cast = place_var(f, &class, k, &params)
            .and_then(|beta| resolve(m, method, &args, &beta))
            .is_some_and(|resolved| resolves_elsewhere(n, &resolved));
        n.d_mut(|d| d.kuna_cast_lambdas = cast);
    }
}

/// `β` when `formal` is `C<β>` or `C<? extends β>` with `β` a type
/// parameter of the outer method at place `k` of class `class`.
fn place_var(formal: &VarType, class: &str, k: usize, params: &[String]) -> Option<VarType> {
    if formal.value_str() != class || formal.array_dim != 0 {
        return None;
    }
    let a = formal.try_as_generic()?.get_arguments().get(k)?.clone()?;
    let w = a
        .try_as_generic()
        .map_or(GenericType::WILDCARD_NO, |g| g.get_wildcard());
    if w != GenericType::WILDCARD_NO && w != GenericType::WILDCARD_EXTENDS {
        return None;
    }
    let bare = strip(&a);
    (bare.type_ == CodeType::Genvar && params.iter().any(|p| p == bare.value_str())).then_some(bare)
}

/// What `β` resolves to (JLS 18.4), from the other arguments of `m` and its
/// declared target; `None` when a bound cannot be told.
fn resolve(
    m: InvocationExprent,
    method: StructMethod,
    args: &[(VarType, Exprent)],
    beta: &VarType,
) -> Option<Resolved> {
    let mut b = Bounds::default();
    let ret = method.get_signature()?.return_type()?;
    if mentions(&ret, beta) {
        let target = m.d(|d| d.kuna_target.clone())?;
        target_bound(&ret, &target, beta, &mut b)?;
    }
    for (f, a) in args {
        if !mentions(f, beta) {
            continue;
        }
        if lambda_only(*a).is_some() {
            continue;
        }
        if is_lambda(*a) {
            return None;
        }
        if f.type_ == CodeType::Genvar {
            if f.array_dim != 0 || !f.value.eq(&beta.value) {
                return None;
            }
            b.lower.push(declared(*a)?);
            continue;
        }
        let k = place_of(f, beta)?;
        let at = declared(*a)?;
        let view = supertype_args(&at, f.value_str())?;
        let x = view.get(k)?.clone()?;
        let wf = wildcard(&f.try_as_generic()?.get_arguments()[k].clone()?);
        match wildcard(&x) {
            GenericType::WILDCARD_NO if wf == GenericType::WILDCARD_EXTENDS => b.lower.push(x),
            GenericType::WILDCARD_NO => b.equal.push(x),
            GenericType::WILDCARD_EXTENDS if wf == GenericType::WILDCARD_EXTENDS => b.captures += 1,
            _ => return None,
        }
    }
    Some(if let Some(e) = b.equal.first() {
        if b.equal
            .iter()
            .any(|x| !arg_equal(&Some(x.clone()), &Some(e.clone())))
        {
            return None;
        }
        Resolved::Type(e.clone())
    } else if b.captures > 0 {
        if b.captures + b.lower.len() != 1 {
            return None;
        }
        Resolved::Capture
    } else if let Some(l) = b.lower.first() {
        if b.lower
            .iter()
            .any(|x| !arg_equal(&Some(x.clone()), &Some(l.clone())))
        {
            return None;
        }
        Resolved::Type(l.clone())
    } else if let Some(u) = b.upper.first() {
        if b.upper.len() != 1 {
            return None;
        }
        Resolved::Type(u.clone())
    } else {
        Resolved::Type(VarType::VARTYPE_OBJECT())
    })
}

enum Resolved {
    Type(VarType),
    Capture,
}

/// Whether the lambdas `n` takes bare resolve to something other than
/// their own interface.
fn resolves_elsewhere(n: InvocationExprent, resolved: &Resolved) -> bool {
    let Resolved::Type(t) = resolved else {
        return true;
    };
    n.get_lst_parameters()
        .to_vec()
        .into_iter()
        .filter(|a| is_lambda(*a))
        .all(|a| {
            t.type_ != CodeType::Object
                || t.array_dim != 0
                || t.value != a.as_new().0.get_expr_type().value
        })
}

fn wildcard(t: &VarType) -> i32 {
    t.try_as_generic()
        .map_or(GenericType::WILDCARD_NO, |g| g.get_wildcard())
}

fn place_of(f: &VarType, beta: &VarType) -> Option<usize> {
    let g = f.try_as_generic()?;
    let args = g.get_arguments();
    let mut place = None;
    for (k, a) in args.iter().enumerate() {
        let a = a.as_ref()?;
        if a.type_ == CodeType::Genvar && a.value == beta.value && a.array_dim == 0 {
            if place.is_some() {
                return None;
            }
            place = Some(k);
        } else if mentions(a, beta) {
            return None;
        }
    }
    place
}

/// The bound the declared target `target` puts on `beta` through the
/// method's return type `ret`.
fn target_bound(ret: &VarType, target: &VarType, beta: &VarType, b: &mut Bounds) -> Option<()> {
    let k = place_of(ret, beta)?;
    if target.type_ != CodeType::Object || target.array_dim != 0 {
        return None;
    }
    if target.value_str() != ret.value_str() {
        return None;
    }
    let t = target.try_as_generic()?.get_arguments().get(k)?.clone()?;
    if wildcard(ret.try_as_generic()?.get_arguments()[k].as_ref()?) != GenericType::WILDCARD_NO {
        return None;
    }
    match wildcard(&t) {
        GenericType::WILDCARD_NO => b.equal.push(t),
        GenericType::WILDCARD_EXTENDS => b.upper.push(strip(&t)),
        _ => return None,
    }
    Some(())
}

/// `t` without its wildcard: `X` for `? extends X`.
fn strip(t: &VarType) -> VarType {
    match t.try_as_generic() {
        Some(g) if g.get_wildcard() != GenericType::WILDCARD_NO => {
            GenericType::new(
                t.type_,
                t.array_dim,
                t.value_str(),
                g.get_parent(),
                g.get_arguments().to_vec(),
                GenericType::WILDCARD_NO,
            )
            .0
        }
        _ => t.clone(),
    }
}

/// The static type the program gives an argument: a variable or field by
/// its declaration, a call by its inferred type.
fn declared(e: Exprent) -> Option<VarType> {
    if e.try_as_var().is_some() || e.try_as_field().is_some() || e.try_as_invocation().is_some() {
        let t = e.get_inferred_expr_type(None);
        return (t.type_ == CodeType::Object).then_some(t);
    }
    None
}

/// The target of the value a `return` in the method being written returns:
/// its generic return type, or in a lambda's body the return type of the
/// function type, when it names no type variable of the interface.
pub fn return_target(declared: &VarType, generic: bool) -> Option<VarType> {
    if generic {
        return Some(declared.clone());
    }
    let node = DecompilerContext::get_current_class_node()?;
    if node.with(|d| d.type_) != ClassNodeType::Lambda {
        return None;
    }
    let info = node.with(|d| d.lambda_information.clone())?;
    if info.is_method_reference {
        return None;
    }
    let interface = node.with(|d| d.anonymous_class_type.clone())?;
    let cls = DecompilerContext::get_struct_context().get_class(interface.value.as_deref()?)?;
    let sam = cls.get_method(&info.method_name, &info.method_descriptor)?;
    let ret = sam.get_signature()?.return_type()?;
    let vars = cls
        .get_signature()
        .map(|s| s.fparameters().clone())
        .unwrap_or_default();
    (!vars
        .iter()
        .any(|v| mentions(&ret, &GenericType::parse(&format!("T{v};")))))
    .then_some(ret)
}

/// The arguments of `n`, marked by [`mark`], that are lambdas or method
/// references passed for a bare type variable: written with a cast.
pub fn cast_arguments(n: InvocationExprent) -> crate::rt::BitSet {
    let mut out = crate::rt::BitSet::new();
    if !n.d(|d| d.kuna_cast_lambdas) {
        return out;
    }
    let Some(m) = method(n) else {
        return out;
    };
    let params = type_params(m);
    let Some(sig) = m.get_signature() else {
        return out;
    };
    let formals = sig.parameter_types().clone();
    let args = n.get_lst_parameters();
    if formals.len() != args.len() {
        return out;
    }
    let named = crate::rt::JHashSet::from_collection(n.0.get_named_generics().key_vec());
    for (i, f) in formals.iter().enumerate() {
        let arg = args.get(i);
        if !is_param(f, &params) || !is_lambda(arg) {
            continue;
        }
        let own = arg.as_new().0.get_expr_type();
        let mapped = n.d(|d| d.generics_map.get(f).cloned().flatten());
        let fits = mapped.is_some_and(|t| {
            t.type_ == CodeType::Object
                && t.array_dim == 0
                && t.value == own.value
                && !(t.is_generic() && t.as_generic().has_unknown_generic_type(&named))
                && !crate::kuna_static_types::is_raw_type(&t)
        });
        if fits {
            out.set(i);
        }
    }
    out
}
