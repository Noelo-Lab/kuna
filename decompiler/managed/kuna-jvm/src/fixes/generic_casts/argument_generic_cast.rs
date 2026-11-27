//! Fix `argument_generic_cast`: an argument whose parameter's declared type
//! does not depend on the call's inference gets the unchecked cast the other
//! cast fixes derive, through the raw class where the two are provably
//! distinct.

use super::mentions;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::invocation_exprent::{InvocationExprent, Type};
use crate::fixes::generic_casts::unchecked_generic_cast;
use crate::kuna_static_types::{formals, supertype_args};
use crate::fixes::generic_casts::bounded_inference_cast;
use crate::rt::JHashMap;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

/// The cast `exprent` needs as an argument for a parameter declared
/// `formal`, and the raw class to cast through first when the two are
/// provably distinct.
pub fn cast_for(exprent: Exprent, formal: &VarType) -> Option<(VarType, Option<VarType>)> {
    let cast = unchecked_generic_cast::cast_for(exprent, formal)
        .or_else(|| bounded_inference_cast::cast_for(exprent, formal))?;
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
        unchecked_generic_cast::declared_type(inv.get_instance()?)?
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
