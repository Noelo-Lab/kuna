//! kuna fix `argument_generic_cast` (`kuna_fixes`): the unchecked casts the
//! fixes `unchecked_generic_cast`, `unchecked_call_result_cast` and
//! `bounded_inference_cast` write for a returned or assigned value compile
//! to no instruction wherever they are written, and javac needs them just as
//! much on a method or constructor argument: `out.writeGenericMap(innerMap)`
//! of a `Map<String, String>` for a `Map<String, Object>` parameter,
//! `super(name, source)` of a `Properties` for a `Map<String, Object>`,
//! `this(Ordering.natural())` for a `Comparator<? super E>` (`incompatible
//! types`). Vineflower writes them bare. With the fix, an argument whose
//! parameter's declared type does not depend on what javac infers for the
//! call (no type variable of a generic method, nor of the class a diamond
//! creates; the class's variables taken from the receiver's declared type,
//! or from the superclass a `super(..)` call names) gets the cast those fixes
//! derive for it, written through the raw class, `(Map<String, Object>)
//! (Map) innerMap`, where the two parameterizations are provably distinct:
//! a raw argument would erase the call's result (JLS 15.12.2.6).

use std::cell::RefCell;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, Type};
use crate::kuna_fix_unchecked_generic_cast;
use crate::kuna_static_types::{formals, supertype_args};
use crate::p2_lift::kuna_fix_bounded_inference_cast;
use crate::rt::JHashMap;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

thread_local! {
    static ARGUMENT_TARGET: RefCell<Option<VarType>> = const { RefCell::new(None) };
}

/// Marks the next value `ExprProcessor.getCastedExprent` writes as an
/// argument whose parameter is declared `formal`.
pub fn mark_argument_target(formal: VarType) {
    ARGUMENT_TARGET.with(|c| *c.borrow_mut() = Some(formal));
}

/// The declared parameter type of the value being written, when it was
/// marked; clears the mark, so that the values nested in it are not.
pub fn take_argument_target() -> Option<VarType> {
    ARGUMENT_TARGET.with(|c| c.borrow_mut().take())
}

/// The cast `exprent` needs as an argument for a parameter declared
/// `formal`, and the raw class to cast through first when the two are
/// provably distinct.
pub fn cast_for(exprent: Exprent, formal: &VarType) -> Option<(VarType, Option<VarType>)> {
    let cast = kuna_fix_unchecked_generic_cast::cast_for(exprent, formal)
        .or_else(|| kuna_fix_bounded_inference_cast::cast_for(exprent, formal))?;
    Some(if cast.is_generic() || !formal.is_generic() {
        (cast, None)
    } else {
        (formal.clone(), Some(cast))
    })
}

/// The declared type of the parameter `param` (the signature's) of `m`,
/// called by `inv`, with the class's type variables substituted, when it
/// does not depend on what javac infers for the call.
pub fn fixed_formal(inv: InvocationExprent, m: StructMethod, param: &VarType) -> Option<VarType> {
    let generic = param.type_ == CodeType::Object && param.is_generic();
    if param.array_dim != 0 || !(generic || param.type_ == CodeType::Genvar) {
        return None;
    }
    let sig = m.get_signature()?;
    if sig
        .type_parameters()
        .iter()
        .any(|o| mentions(param, o))
    {
        return None;
    }
    let owner = m.get_class_qualified_name();
    let class_vars = formals(&owner)?;
    if !class_vars.iter().any(|v| mentions(param, v.value_str())) {
        return generic.then(|| param.clone());
    }
    let receiver = if inv.get_functype() == Type::Init {
        let current = DecompilerContext::get_current_class_node()?
            .with(|d| d.class_struct);
        let instance = inv.get_instance()?.try_as_var()?;
        let pair = instance.get_var_version_pair();
        if !instance
            .get_processor()?
            .get_this_vars()
            .borrow()
            .contains_key(&pair)
        {
            return None;
        }
        let csig = current.get_signature()?;
        if current.qualified_name() == owner {
            csig.generic_type().0
        } else {
            let sup = csig.superclass();
            if sup.value_str() != owner {
                return None;
            }
            sup
        }
    } else {
        if inv.is_static() {
            return None;
        }
        kuna_fix_unchecked_generic_cast::declared_type(inv.get_instance()?)?
    };
    if receiver.type_ != CodeType::Object || receiver.array_dim != 0 {
        return None;
    }
    let view = supertype_args(&receiver, &owner)?;
    if view.len() != class_vars.len() {
        return None;
    }
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    for (v, a) in class_vars.iter().zip(view) {
        let a = a?;
        if a.try_as_generic()
            .is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO)
        {
            return None;
        }
        map.put(v.clone(), Some(a));
    }
    param
        .remap(&map)
        .filter(|f| f.type_ == CodeType::Object && f.array_dim == 0 && f.is_generic())
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
