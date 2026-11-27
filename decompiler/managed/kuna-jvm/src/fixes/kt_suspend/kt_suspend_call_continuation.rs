//! Fix `kt_suspend_call_continuation`: in a suspend function, a call of a
//! suspend function that passes the function's own continuation is marked to
//! be written without it, and the casts of its result to the declared type are
//! left out.

use std::rc::Rc;

use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::method_descriptor::MethodDescriptor;
use crate::decompiler_context::DecompilerContext;
use crate::fixes::kt_suspend::kt_suspend_lambda;
use crate::fixes::kt_calls::kt_synthetic_accessor_call;
use crate::fixes::kt_suspend::kt_suspend_state_machine::{completion_slot, is_function_invoke};
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::fixes::kt_util::SUSPEND_DESCRIPTOR_END;
use super::CONTINUATION;
use crate::fixes::util::all_exprents;

/// Whether `mt`'s body is a suspend function's whose calls this fix marks, for
/// fix `kt_nullable_local`'s typing.
pub fn is_suspend_body(mt: StructMethod) -> bool {
    continuation_slot(mt).is_some()
}

/// The slot of the continuation a suspend function's calls pass on: its
/// `$completion`, or `this` in a suspend lambda's body that fix
/// `kt_suspend_lambda` rebuilt.
fn continuation_slot(mt: StructMethod) -> Option<i32> {
    completion_slot(mt).or_else(|| kt_suspend_lambda::parameters(mt).is_some().then_some(0))
}

/// Marks the suspend calls under `root`, the body of `mt`, that pass the
/// function's own continuation; whether it marked one.
pub fn mark(root: RootStatement, mt: StructMethod) -> bool {
    let Some(completion) = continuation_slot(mt) else {
        return false;
    };
    let exprents = all_exprents(root.0);
    let mut res = false;
    for e in exprents {
        let Some(inv) = e.try_as_invocation() else {
            continue;
        };
        if (passes_continuation(inv, completion)
            && (is_function_invoke(&inv.get_classname(), &inv.get_name(), &inv.get_string_descriptor())
                || !declared_not_suspend(inv)))
            || reads_context(inv, completion)
        {
            inv.set_kuna_continuation_arg(true);
            res = true;
        }
    }
    if res {
        uncast_results(root.0);
    }
    res
}

/// Leaves out, under `stat`, the casts of marked calls' results to their
/// declared return types.
fn uncast_results(stat: Statement) {
    match stat.get_exprents() {
        Some(exprents) => {
            for e in exprents.to_vec() {
                uncast_under(e);
            }
        }
        None => {
            for st in stat.get_stats().borrow().to_vec() {
                uncast_results(st);
            }
            for e in stat.get_stat_exprents() {
                uncast_under(e);
            }
        }
    }
}

fn uncast_under(parent: Exprent) {
    for child in parent.get_all_exprents() {
        match result_of_cast(child).or_else(|| result_of_unboxing(child)) {
            Some((call, declared)) => {
                let inv = call.as_invocation();
                let jvm = inv.get_string_descriptor();
                let params = &jvm[..=jvm.rfind(')').unwrap_or(0)];
                inv.set_descriptor(MethodDescriptor::parse_descriptor(&format!("{params}{declared}")));
                parent.replace_exprent(child, call);
                uncast_under(call);
            }
            None => uncast_under(child),
        }
    }
}

const UNBOXING: [(&str, &str); 8] = [
    ("booleanValue", "()Z"),
    ("byteValue", "()B"),
    ("shortValue", "()S"),
    ("intValue", "()I"),
    ("longValue", "()J"),
    ("floatValue", "()F"),
    ("doubleValue", "()D"),
    ("charValue", "()C"),
];

/// The marked call `e` unboxes (through kotlinc's cast to `Number` or the
/// wrapper) to the primitive its callee's metadata declares it returns, and
/// that type.
fn result_of_unboxing(e: Exprent) -> Option<(Exprent, VarType)> {
    let unbox = e.try_as_invocation()?;
    let name = unbox.get_name();
    let descriptor = unbox.get_string_descriptor();
    if unbox.is_static() || !UNBOXING.iter().any(|(n, d)| *n == name && *d == descriptor) {
        return None;
    }
    let mut call = unbox.get_instance()?;
    if let Some(cast) = call.try_as_function().filter(|f| f.get_func_type() == FunctionType::Cast) {
        call = cast.get_lst_operands().get(0);
    }
    let function = declared_function(call)?;
    let declared = &function.return_type;
    let primitive = &descriptor[2..];
    let fits = declared.k().type_parameter_name.is_none()
        && !declared.k().is_nullable
        && declared.0.array_dim == 0
        && declared.0.to_string() == primitive;
    fits.then(|| (call, declared.0.clone()))
}

/// The function a marked call with arguments calls, from its metadata
/// (through the synthetic accessor it calls, if it calls one).
fn declared_function(call: Exprent) -> Option<Rc<KFunction>> {
    let inv = call.try_as_invocation()?;
    if !inv.d(|d| d.kuna_continuation_arg) || inv.get_lst_parameters().is_empty() {
        return None;
    }
    let cl = DecompilerContext::get_struct_context().get_class(&inv.get_classname())?;
    let method = cl.get_method(&inv.get_name(), &inv.get_string_descriptor())?;
    let element = KtAttrs::get(method, &KElement::KEY()).or_else(|| {
        kt_synthetic_accessor_call::resugar(inv).map(|access| access.calling_data.clone())
    });
    match element {
        Some(KElement::Function(function)) => Some(function),
        _ => None,
    }
}

/// The marked call `e` casts to the class of its callee's declared return
/// type, and that type.
fn result_of_cast(e: Exprent) -> Option<(Exprent, VarType)> {
    let cast = e.try_as_function()?;
    if cast.get_func_type() != FunctionType::Cast {
        return None;
    }
    let operands = cast.get_lst_operands();
    let call = operands.get(0);
    let function = declared_function(call)?;
    let target = operands.get(1).get_expr_type();
    let declared = &function.return_type;
    let fits = declared.k().type_parameter_name.is_none()
        && target.array_dim == declared.0.array_dim
        && target.value.is_some()
        && target.value == declared.0.value
        && target.value.as_deref() != Some("java/lang/Object");
    fits.then_some((call, target))
}

/// Whether `inv` calls a method in kotlinc's suspend convention with the
/// variable in slot `completion` as its last argument.
fn passes_continuation(inv: InvocationExprent, completion: i32) -> bool {
    let descriptor = inv.get_string_descriptor();
    (descriptor.ends_with(SUSPEND_DESCRIPTOR_END)
        || is_function_invoke(&inv.get_classname(), &inv.get_name(), &descriptor))
        && inv
            .get_lst_parameters()
            .last()
            .is_some_and(|last| is_completion(last, completion))
}

/// Whether `inv` is `getContext()` on the variable in slot `completion`.
fn reads_context(inv: InvocationExprent, completion: i32) -> bool {
    inv.get_classname() == CONTINUATION
        && inv.get_name() == "getContext"
        && inv.get_string_descriptor() == "()Lkotlin/coroutines/CoroutineContext;"
        && inv
            .get_instance()
            .is_some_and(|instance| is_completion(instance, completion))
}

fn is_completion(e: Exprent, completion: i32) -> bool {
    let Some(var) = e.try_as_var() else {
        return false;
    };
    let index = var.get_index();
    let slot = var
        .get_processor()
        .and_then(|p| p.get_var_original_index(index))
        .unwrap_or(index);
    slot == completion
}

/// `kotlin.coroutines.coroutineContext`, for the context read `inv` marks:
/// qualified, since in a `CoroutineScope`'s scope the simple name is the
/// scope's property.
pub fn write_context(inv: InvocationExprent) -> Option<TextBuffer> {
    if !inv.d(|d| d.kuna_continuation_arg) || !inv.get_lst_parameters().is_empty() {
        return None;
    }
    let mut buf = TextBuffer::new();
    buf.add_bytecode_mapping(inv.get_bytecode().as_ref());
    buf.append("kotlin.coroutines.coroutineContext");
    Some(buf)
}

/// Whether the metadata of the method `inv` calls declares it a function
/// without `suspend`.
fn declared_not_suspend(inv: InvocationExprent) -> bool {
    let Some(cl) = DecompilerContext::get_struct_context().get_class(&inv.get_classname()) else {
        return false;
    };
    let Some(method) = cl.get_method(&inv.get_name(), &inv.get_string_descriptor()) else {
        return false;
    };
    matches!(
        KtAttrs::get(method, &KElement::KEY()),
        Some(KElement::Function(function)) if !Flags::IS_SUSPEND.get(function.flags)
    )
}

