//! Fix `explicit_lambda_parameters`: a lambda passed where its target does
//! not give its parameters the types they have, because the target's type
//! argument for one is a wildcard or a type variable of the called method
//! that nothing else fixes, declares its parameters with the types its
//! `invokedynamic` instantiates: `g((String s) -> s.length())` for
//! `g(Function<?, ?>)`, where `s -> s.length()` types `s` as `Object`.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationKind};
use crate::kuna_static_types::is_bare_var;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::var_type::VarType;

/// Marks the lambda arguments of `inv` the module header describes.
pub fn mark(inv: InvocationExprent) {
    if inv.get_functype() != InvocationKind::General {
        return;
    }
    let args = inv.get_lst_parameters().to_vec();
    if !args.iter().any(|a| a.try_as_new().is_some_and(|n| n.is_lambda() && !n.is_method_reference())) {
        return;
    }
    let Some(sig) = inv.find_desc().and_then(|m| m.get_signature()) else {
        return;
    };
    let formals = sig.parameter_types().clone();
    if formals.len() != args.len() {
        return;
    }
    let method_vars = sig.type_parameters().clone();
    let bounds = sig.type_parameter_bounds().clone();
    let ret = sig.return_type();
    for (i, (formal, arg)) in formals.iter().zip(&args).enumerate() {
        let Some(lambda) = arg.try_as_new().filter(|n| n.is_lambda() && !n.is_method_reference()) else {
            continue;
        };
        let lonely = |name: &str| {
            let index = method_vars.iter().position(|v| v == name)?;
            if ret.as_ref().is_some_and(|r| mentions(r, name))
                || formals.iter().enumerate().any(|(k, f)| k != i && mentions(f, name))
            {
                return None;
            }
            match bounds.get(index).and_then(|b| b.first()) {
                None => Some(VarType::VARTYPE_OBJECT()),
                Some(b) if b.type_ == CodeType::Object && !has_args(b) => Some(erasure(b)),
                Some(_) => None,
            }
        };
        if needs_explicit_types(lambda, formal, &lonely) {
            lambda.set_explicit_lambda_params();
        }
    }
}

/// Marks `value`, assigned to a variable or field of type `target`, when it
/// is a lambda the module header describes.
pub fn mark_assigned(target: &VarType, value: Exprent) {
    let Some(lambda) = value.try_as_new().filter(|n| n.is_lambda() && !n.is_method_reference()) else {
        return;
    };
    if super::raw_target_cast::erases_parameters(target, lambda) {
        lambda.set_kuna_erased_params();
    }
    if needs_explicit_types(lambda, target, &|_| None) {
        lambda.set_explicit_lambda_params();
    }
}

/// Whether `lambda`, written for the target type `target`, needs its
/// parameters declared, and can have them declared with the instantiated
/// types: each parameter of the interface method that is a type variable of
/// the interface takes its type from `target`'s argument, which for a
/// wildcard is its bound (JLS 9.9) and for a type variable `lonely` accepts
/// is that variable's bound, what inference gives it; the declared types are written when one of these differs
/// from the instantiated type and every other parameter's type is exactly
/// the instantiated one.
fn needs_explicit_types(lambda: NewExprent, target: &VarType, lonely: &dyn Fn(&str) -> Option<VarType>) -> bool {
    let Some((iface, instantiated, sam)) = super::functional_method(lambda) else {
        return false;
    };
    if target.array_dim != 0 || target.value != iface.value {
        return false;
    }
    let Some(target_args) = target.try_as_generic().map(|g| g.get_arguments().to_vec()) else {
        return false;
    };
    let Some(cls) = DecompilerContext::get_struct_context().get_class(iface.value_str()) else {
        return false;
    };
    let Some((fparams, fbounds)) = cls.get_signature().map(|s| (s.fparameters().clone(), s.fbounds().clone())) else {
        return false;
    };
    let Some(sam_sig) = sam.get_signature() else {
        return false;
    };
    let generic_params = sam_sig.parameter_types().clone();
    let md = MethodDescriptor::parse_descriptor(&instantiated);
    if fparams.len() != target_args.len() || generic_params.len() != md.params.len() || md.params.is_empty() {
        return false;
    }
    let mut needed = false;
    for (generic, erased) in generic_params.iter().zip(&md.params) {
        if !is_bare_var(generic) {
            if generic.type_ == CodeType::Genvar || has_args(generic) {
                return false;
            }
            continue;
        }
        let Some(j) = fparams.iter().position(|p| p == generic.value_str()) else {
            return false;
        };
        let implicit = match target_args[j].as_ref() {
            None => fbounds
                .get(j)
                .and_then(|b| b.first())
                .filter(|b| b.type_ == CodeType::Object && !has_args(b))
                .map(erasure)
                .unwrap_or_else(VarType::VARTYPE_OBJECT),
            Some(arg) => {
                let wildcard = arg.try_as_generic().map_or(GenericType::WILDCARD_NO, |g| g.get_wildcard());
                if wildcard != GenericType::WILDCARD_NO {
                    if arg.type_ != CodeType::Object || has_args(arg) {
                        return false;
                    }
                    erasure(arg)
                } else if let Some(bound) = is_bare_var(arg).then(|| lonely(arg.value_str())).flatten() {
                    bound
                } else if arg.type_ == CodeType::Object && !has_args(arg) && erasure(arg).equals(erased) {
                    continue;
                } else {
                    return false;
                }
            }
        };
        needed |= !implicit.equals(erased);
    }
    needed
}

fn has_args(t: &VarType) -> bool {
    t.try_as_generic().is_some_and(|g| !g.get_arguments().is_empty())
}

fn erasure(t: &VarType) -> VarType {
    if t.try_as_generic().is_some_and(|g| g.get_wildcard() == GenericType::WILDCARD_UNBOUND) {
        return VarType::VARTYPE_OBJECT();
    }
    VarType::new_value(t.type_, t.array_dim, t.value.clone())
}

fn mentions(t: &VarType, name: &str) -> bool {
    if t.type_ == CodeType::Genvar {
        return t.value_str() == name;
    }
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments().iter().flatten().any(|a| mentions(a, name))
            || g.get_parent().is_some_and(|p| mentions(&p, name))
    })
}
