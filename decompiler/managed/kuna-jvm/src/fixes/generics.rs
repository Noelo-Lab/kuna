//! Fix family `generics`: `diamond_argument_conflict` writes a generic `new`
//! raw when its arguments bind a type argument the expected type contradicts;
//! `external_class_diamond` gives `new` of a class outside the input the
//! diamond its expected type proves generic; `static_type_variable_scope`
//! stops counting class type variables in scope past a static method;
//! `thrown_type_variable` casts a thrown checked exception to the method's
//! thrown type variable, and types a raw `new C()` receiver of a call that
//! throws `C`'s parameters with `RuntimeException`.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::fixes::constructor_calls::synthetic_null_check_shape::static_type;
use crate::generic_type::GenericType;
use crate::invocation_exprent::InvocationExprent;
use crate::method_wrapper::MethodWrapper;
use crate::new_exprent::NewExprent;
use crate::rt::{JHashMap, JHashSet};
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_type::{get_class, str_equals, VarType};

/// Whether `inferred`, the type a generic `new` of a class with type
/// parameters `fparams` gets from its arguments, binds a type argument to a
/// type variable in scope (`named`) where `expected` has a different one.
pub fn contradicts(
    inferred: Option<&VarType>,
    expected: Option<&VarType>,
    fparams: &[String],
    named: &JHashMap<VarType, Vec<VarType>>,
) -> bool {
    let (Some(inferred), Some(expected)) = (inferred, expected) else {
        return false;
    };
    if !inferred.is_generic() || !expected.is_generic() || inferred.type_ != CodeType::Object {
        return false;
    }
    let lifted = if str_equals(&inferred.value, &expected.value) {
        Some(inferred.clone())
    } else {
        GenericType::get_generic_super_type(inferred, expected)
            .filter(|l| str_equals(&l.value, &expected.value))
            .or_else(|| direct_super_type(inferred, expected))
    };
    match lifted {
        Some(lifted) if lifted.is_generic() => args_clash(&lifted, expected, fparams, named),
        _ => false,
    }
}

/// `inferred` as its direct supertype of `expected`'s class, read from its
/// class's own signature (the supertype's class may be outside the input).
fn direct_super_type(inferred: &VarType, expected: &VarType) -> Option<VarType> {
    let sig = get_class(&inferred.value)?.get_signature()?;
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    sig.generic_type().map_gen_vars_to(&inferred.as_generic(), &mut map);
    let mut supers = vec![sig.superclass()];
    supers.extend(sig.superinterfaces().iter().cloned());
    supers
        .into_iter()
        .find(|t| str_equals(&t.value, &expected.value))
        .and_then(|t| t.remap(&map))
}

fn args_clash(
    a: &VarType,
    b: &VarType,
    fparams: &[String],
    named: &JHashMap<VarType, Vec<VarType>>,
) -> bool {
    if a.type_ == CodeType::Genvar && b.type_ == CodeType::Genvar {
        let own = fparams
            .iter()
            .any(|p| a.value.as_deref() == Some(p.as_str()));
        return a.array_dim == b.array_dim
            && !own
            && named.contains_key(a)
            && named.contains_key(b)
            && !a.equals(b);
    }
    if a.type_ != CodeType::Object
        || b.type_ != CodeType::Object
        || !a.is_generic_type()
        || !b.is_generic_type()
        || !str_equals(&a.value, &b.value)
        || a.array_dim != b.array_dim
    {
        return false;
    }
    let (ga, gb) = (a.as_generic(), b.as_generic());
    if ga.get_wildcard() != GenericType::WILDCARD_NO
        || gb.get_wildcard() != GenericType::WILDCARD_NO
    {
        return false;
    }
    let (xs, ys) = (ga.get_arguments(), gb.get_arguments());
    xs.len() == ys.len()
        && xs.iter().zip(ys.iter()).any(|(x, y)| match (x, y) {
            (Some(x), Some(y)) => {
                let (gx, gy) = (x.try_as_generic(), y.try_as_generic());
                let wild = gx.is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO)
                    || gy.is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO);
                !wild && args_clash(x, y, fparams, named)
            }
            _ => false,
        })
}

/// The inferred type of `new`, whose class is outside the class universe,
/// when `upper_bound` makes it a generic instance creation; `None` leaves the
/// expression to Vineflower's inference.
pub fn external_diamond_type(new: NewExprent, upper_bound: Option<&VarType>) -> Option<VarType> {
    let (new_type, anonymous, lambda, constructor) =
        new.d(|d| (d.new_type.clone(), d.anonymous, d.lambda, d.constructor));
    let constructor = constructor?;
    let ub = upper_bound?;
    if lambda
        || anonymous
        || new.0.is_kotlin()
        || new_type.array_dim != 0
        || ub.type_ != CodeType::Object
        || ub.array_dim != 0
        || !ub.is_generic()
        || !str_equals(&ub.value, &new_type.value)
    {
        return None;
    }
    let generic = ub.as_generic();
    let arguments = generic.get_arguments();
    if generic.get_wildcard() != GenericType::WILDCARD_NO || arguments.is_empty() {
        return None;
    }

    let named = new.0.get_named_generics();
    let mapped = arguments
        .iter()
        .flatten()
        .filter(|a| a.type_ != CodeType::Genvar || named.contains_key(a))
        .count();
    if mapped == 0 {
        return None;
    }

    constructor.d_mut(|d| {
        d.generic_args.clear();
        d.generic_args.push(GenericType::DUMMY_VAR().0);
    });
    let named_keys = JHashSet::from_collection(named.key_vec());
    if mapped == arguments.len() && !generic.has_unknown_generic_type(&named_keys) {
        Some(ub.clone())
    } else {
        Some(new.get_expr_type())
    }
}

/// Whether `method` is a static method of its class's source. A synthetic
/// method, such as the static implementation method of a lambda, says
/// nothing about the context the lambda is written in.
pub fn is_static_method(node: Option<ClassNode>, method: MethodWrapper) -> bool {
    if node.is_some_and(|n| n.with(|d| d.type_) == ClassNodeType::Lambda) {
        return false;
    }
    let mt = method.with(|d| d.method_struct);
    mt.has_modifier(CodeConstants::ACC_STATIC) && !mt.has_modifier(CodeConstants::ACC_SYNTHETIC)
}

const THROWABLE: &str = "java/lang/Throwable";
const EXCEPTION: &str = "java/lang/Exception";
const RUNTIME_EXCEPTION: &str = "java/lang/RuntimeException";

/// The type variable to cast the value of a `throw` in the current method to.
pub fn throw_cast(value: Exprent) -> Option<VarType> {
    if value.is_kotlin() {
        return None;
    }
    let method = DecompilerContext::get_current_method_wrapper()?.with(|d| d.method_struct);
    let sig = method.get_signature()?;
    let thrown = sig.exception_types().clone();
    if !thrown.iter().any(|t| t.type_ == CodeType::Genvar) {
        return None;
    }
    let var = value.try_as_var()?;
    if var
        .get_lvt()
        .is_some_and(|lvt| lvt.get_signature().is_some())
    {
        return None;
    }
    let v = static_type(value);
    if v.type_ != CodeType::Object || v.array_dim != 0 || !is_checked(v.value_str()) {
        return None;
    }
    if thrown
        .iter()
        .any(|t| t.type_ != CodeType::Genvar && admits(t.value_str(), v.value_str()))
    {
        return None;
    }
    let erased: Vec<String> = method
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS)
        .map(|attr| {
            let pool = DecompilerContext::get_current_class_node()
                .expect("NullPointerException: class node")
                .with(|d| d.class_struct)
                .get_pool();
            (0..attr.get_throws_exceptions().len())
                .map(|i| attr.get_exc_classname(i as i32, pool))
                .collect()
        })
        .unwrap_or_default();
    if erased.len() != thrown.len() {
        return None;
    }
    thrown
        .iter()
        .zip(&erased)
        .find(|(t, e)| t.type_ == CodeType::Genvar && t.array_dim == 0 && admits(e, v.value_str()))
        .map(|(t, _)| t.clone())
}

/// Gives a raw `new C()` that is `call`'s receiver the type arguments that
/// make `call`'s thrown type variables unchecked.
pub fn type_receiver(call: InvocationExprent, instance: Exprent) {
    if instance.is_kotlin() {
        return;
    }
    let Some(new) = instance.try_as_new() else {
        return;
    };
    let Some(ctor) = new.get_constructor() else {
        return;
    };
    let new_type = new.get_new_type();
    if new.is_lambda()
        || new.is_anonymous()
        || new_type.array_dim != 0
        || !ctor.get_generic_args().is_empty()
    {
        return;
    }
    let context = DecompilerContext::get_struct_context();
    let Some(class_sig) = context
        .get_class(new_type.value_str())
        .and_then(|c| c.get_signature())
    else {
        return;
    };
    let Some(method_sig) = context
        .get_class(&call.get_classname())
        .and_then(|c| c.get_method_recursive(&call.get_name(), &call.get_string_descriptor()))
        .and_then(|m| m.get_signature())
    else {
        return;
    };
    let params = class_sig.fparameters().clone();
    let thrown: Vec<String> = method_sig
        .exception_types()
        .iter()
        .filter(|t| t.type_ == CodeType::Genvar)
        .filter_map(|t| t.value.as_deref().map(str::to_string))
        .collect();
    if params.is_empty() || thrown.is_empty() || !params.iter().all(|p| thrown.contains(p)) {
        return;
    }
    let bounds_admit = class_sig.fbounds().iter().all(|bounds| {
        bounds.iter().all(|b| {
            b.type_ == CodeType::Object
                && b.array_dim == 0
                && ["java/lang/Object", THROWABLE, EXCEPTION, RUNTIME_EXCEPTION]
                    .contains(&b.value_str())
        })
    });
    if !bounds_admit {
        return;
    }
    let arg = VarType::new_value(CodeType::Object, 0, RUNTIME_EXCEPTION);
    ctor.d_mut(|d| d.generic_args = vec![arg; params.len()]);
}

/// Whether `class` is a checked exception class: `Throwable`, `Exception`, or
/// a known subclass of either that is not a `RuntimeException` or an `Error`.
fn is_checked(class: &str) -> bool {
    if class == THROWABLE || class == EXCEPTION {
        return true;
    }
    let context = DecompilerContext::get_struct_context();
    context.get_class(class).is_some()
        && context.instance_of(class, THROWABLE)
        && !context.instance_of(class, RUNTIME_EXCEPTION)
        && !context.instance_of(class, "java/lang/Error")
}

/// Whether a value of the exception class `value` is one of `declared`.
fn admits(declared: &str, value: &str) -> bool {
    declared == value
        || declared == THROWABLE
        || (declared == EXCEPTION && value != THROWABLE)
        || DecompilerContext::get_struct_context().instance_of(value, declared)
}
