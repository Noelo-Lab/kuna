//! Static types as kuna's fixes of Vineflower's type inference need them:
//! the declared type of a method parameter, a class's type parameters, the
//! type arguments a type gives one of its supertypes, raw types.

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::field_exprent::FieldExprent;
use crate::generic_type::GenericType;
use crate::method_descriptor::MethodDescriptor;
use crate::var_exprent::VarExprent;
use crate::var_type::{get_class, VarType};

/// The declared type of `var` when it is a parameter of its method:
/// `Some(None)` when it is one but the type is not known (a signature that
/// leaves out synthetic parameters).
pub fn param_type(var: VarExprent) -> Option<Option<VarType>> {
    let varproc = var
        .get_processor()
        .unwrap_or_else(DecompilerContext::get_var_processor);
    let mt = varproc.get_method();
    let index = var.get_index();
    let slot = varproc.get_var_original_index(index).unwrap_or(index);
    let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
    let sig = mt.get_signature();
    let mut at = if mt.has_modifier(CodeConstants::ACC_STATIC) {
        0
    } else {
        1
    };
    for (i, p) in md.params.iter().enumerate() {
        if at == slot {
            return Some(match &sig {
                None => Some(p.clone()),
                Some(s) => {
                    let generic = s.parameter_types();
                    (generic.len() == md.params.len()).then(|| generic[i].clone())
                }
            });
        }
        at += p.stack_size;
    }
    None
}

/// The type variables a class declares, `None` when the class is unknown.
pub fn formals(class: &str) -> Option<Vec<VarType>> {
    let cls = get_class(&Some(class.into()))?;
    Some(match cls.get_signature() {
        Some(sig) => sig
            .fparameters()
            .iter()
            .map(|p| GenericType::parse(&format!("T{};", p)))
            .collect(),
        None => Vec::new(),
    })
}

pub fn args_of(t: &VarType) -> Vec<Option<VarType>> {
    t.try_as_generic()
        .map(|g| g.get_arguments().to_vec())
        .unwrap_or_default()
}

/// The type arguments `sub` gives the class `sup` it extends or implements,
/// or `None` when they are not known exactly.
pub fn supertype_args(sub: &VarType, sup: &str) -> Option<Vec<Option<VarType>>> {
    let sub_args = args_of(sub);
    if sub.value_str() == sup {
        return Some(sub_args);
    }
    let sub_formals = formals(sub.value_str())?;
    if sub_args.len() != sub_formals.len() || sub_args.iter().any(Option::is_none) {
        return None;
    }
    let sup_formals = formals(sup)?;
    let cls = get_class(&sub.value)?;
    let hierarchy = cls.get_all_generics();
    let map = hierarchy.get(&sup.to_string())?;
    let mut subst = crate::rt::JHashMap::new();
    for (p, a) in sub_formals.iter().zip(&sub_args) {
        subst.put(p.clone(), a.clone().expect("checked above"));
    }
    sup_formals
        .iter()
        .map(|q| map.get(q).map(|e| e.remap(&subst)))
        .collect::<Option<Vec<_>>>()
}

pub fn is_bare_var(t: &VarType) -> bool {
    t.type_ == CodeType::Genvar
        && t.array_dim == 0
        && t.try_as_generic()
            .is_none_or(|g| g.get_wildcard() == GenericType::WILDCARD_NO)
}

pub fn args_equal(a: &[Option<VarType>], b: &[Option<VarType>]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| arg_equal(x, y))
}

pub fn arg_equal(a: &Option<VarType>, b: &Option<VarType>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => match (a.try_as_generic(), b.try_as_generic()) {
            (Some(g), _) => g.equals_exact(b),
            (None, Some(g)) => g.equals_exact(a),
            (None, None) => a.equals(b),
        },
        _ => false,
    }
}

/// A raw type: a class type without type arguments whose class declares
/// type parameters.
pub fn is_raw_type(t: &VarType) -> bool {
    t.type_ == CodeType::Object
        && t.array_dim == 0
        && !t.is_generic()
        && formals(t.value_str()).is_some_and(|f| !f.is_empty())
}

/// Whether the field `field` reads is found in the classes kuna knows, so
/// that its declared type is known.
pub fn field_declared(field: FieldExprent) -> bool {
    let context = DecompilerContext::get_struct_context();
    let name = field.get_name();
    let descriptor = field.get_descriptor().descriptor_string.clone();
    let mut cls = context.get_class(&field.get_classname());
    while let Some(c) = cls {
        if c.get_field(&name, &descriptor).is_some() {
            return true;
        }
        cls = c
            .super_class()
            .and_then(|s| s.get_string_opt())
            .and_then(|s| context.get_class(&s));
    }
    false
}
