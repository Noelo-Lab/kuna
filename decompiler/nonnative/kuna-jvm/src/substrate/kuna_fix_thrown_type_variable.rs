//! kuna fix `thrown_type_variable` (`kuna_fixes`): a method may throw a type
//! variable, `<T extends Throwable> Error throwIt(Throwable t) throws T`. Its
//! `throw (T) t` is an unchecked cast to `T`'s erasure, `Throwable`, which
//! compiles to no instruction, and Vineflower writes `throw t`: javac then
//! sees a `Throwable` thrown where only `T` is declared (`unreported
//! exception`). A call of such a method on `new C<Error>()`, the sneaky throw
//! (Guava's `SneakyThrows`), is written on a raw `new C()`, whose erased
//! `throws` clause names `Throwable` too. With the fix a `throw` of a variable
//! whose declared (not generic) type is a checked exception no declared class covers, and
//! that a thrown type variable's erasure admits, is cast to that variable;
//! and a raw `new C()` that is the receiver of a call throwing only `C`'s type
//! parameters is given `RuntimeException` for each of them (where its bound
//! admits it), the type JLS 18.4 infers for a thrown inference variable.
//! Kotlin output, which has no checked exceptions, is left alone.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::p2_lift::kuna_fix_synthetic_null_check_shape::static_type;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_type::VarType;

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
