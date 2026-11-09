//! kuna fix `unchecked_call_result_cast` (`kuna_fixes`): the unchecked cast
//! of a call's result, `(Class<T>) type.resolve()` for a `Class<?>`,
//! `(Constructor<Exception>) exceptionClass.getConstructor(..)` on a
//! `Class<? extends Exception>`, compiles to no instruction, like the casts
//! of the fix `unchecked_generic_cast`, which leaves calls alone because
//! their types are Vineflower's inference. A call's result type is declared,
//! though, when its method has no type parameters of its own in its return
//! type: the generic return type with the class's type arguments taken from
//! the receiver's declared type (JLS 4.5.2). With the fix that type takes
//! part in `unchecked_generic_cast`'s test, so a returned or assigned call
//! whose declared result the target's type arguments do not accept gets the
//! cast javac needs; where the result's type argument is a type variable and
//! the target's a type, the cast is to the raw class (`(Iterator)
//! buckets.iterator()` for an `Iterator<B>` returned as an
//! `Iterator<Bucket>`), since javac rejects the parameterized one when the
//! variable's bound is not the type.

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type};
use crate::kuna_static_types::{formals, is_raw_type, supertype_args};
use crate::rt::{JHashMap, JHashSet};
use crate::var_type::VarType;

/// The declared type of `inv`'s result, `declared` giving its receiver's.
pub fn declared_result(
    inv: InvocationExprent,
    declared: fn(Exprent) -> Option<VarType>,
) -> Option<VarType> {
    if inv.get_functype() != Type::General
        || matches!(
            inv.get_invocation_type(),
            InvocationType::Dynamic | InvocationType::ConstantDynamic
        )
    {
        return None;
    }
    let m = inv.d(|d| d.desc).or_else(|| {
        DecompilerContext::get_struct_context()
            .get_class(&inv.get_classname())?
            .get_method_recursive(&inv.get_name(), &inv.get_string_descriptor())
    })?;
    let ret = match m.get_signature() {
        Some(sig) => {
            let r = sig.return_type()?;
            if sig.type_parameters().iter().any(|p| mentions(&r, p)) {
                return None;
            }
            r
        }
        None => {
            let r = inv.get_descriptor().ret.clone();
            return (r.type_ == CodeType::Object && !is_raw_type(&r)).then_some(r);
        }
    };
    if ret.type_ != CodeType::Object || ret.array_dim != 0 {
        return None;
    }
    let owner = m.get_class_qualified_name();
    let class_vars = formals(&owner)?;
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    if class_vars.iter().any(|v| mentions(&ret, v.value_str())) {
        if m.has_modifier(CodeConstants::ACC_STATIC) {
            return None;
        }
        let receiver = declared(inv.get_instance()?)?;
        if receiver.type_ != CodeType::Object || receiver.array_dim != 0 || is_raw_type(&receiver) {
            return None;
        }
        let view = supertype_args(&receiver, &owner)?;
        if view.len() != class_vars.len() {
            return None;
        }
        for (v, a) in class_vars.iter().zip(view) {
            let wild = a.as_ref().is_none_or(|a| {
                a.try_as_generic()
                    .is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO)
            });
            if wild && !only_top_level(&ret, v.value_str()) {
                return None;
            }
            map.put(v.clone(), a);
        }
    }
    let result = ret.remap(&map)?;
    let named = JHashSet::from_collection(inv.0.get_named_generics().key_vec());
    (!(result.is_generic() && result.as_generic().has_unknown_generic_type(&named)))
        .then_some(result)
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

/// Whether `var` occurs in `t` only as one of its own type arguments, where
/// a wildcard can stand for it.
fn only_top_level(t: &VarType, var: &str) -> bool {
    let Some(g) = t.try_as_generic() else {
        return true;
    };
    !g.get_parent().is_some_and(|p| mentions(&p, var))
        && g.get_arguments().iter().flatten().all(|a| {
            (a.type_ == CodeType::Genvar
                && a.try_as_generic()
                    .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO))
                || !mentions(a, var)
        })
}
