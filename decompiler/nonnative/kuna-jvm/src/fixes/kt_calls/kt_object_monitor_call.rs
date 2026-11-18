//! Fix `kt_object_monitor_call`: a call of `Object.wait`, `notify` or
//! `notifyAll` is made on the receiver cast to `java.lang.Object`, as Kotlin
//! source writes it.

use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::text_buffer::TextBuffer;

const MONITOR_METHODS: &[(&str, &str)] = &[
    ("wait", "()V"),
    ("wait", "(J)V"),
    ("wait", "(JI)V"),
    ("notify", "()V"),
    ("notifyAll", "()V"),
];

/// The call written on its receiver cast to `java.lang.Object`, when it
/// calls a monitor method.
pub fn write(inv: InvocationExprent, indent: i32) -> Option<TextBuffer> {
    if inv.is_static() {
        return None;
    }
    let instance = inv.get_instance()?;
    let name = inv.get_name();
    let descriptor = inv.get_string_descriptor();
    if !MONITOR_METHODS
        .iter()
        .any(|(n, d)| *n == name && *d == descriptor)
    {
        return None;
    }
    let instance = uncast(instance);
    let mut buf = TextBuffer::new();
    buf.add_bytecode_mapping(inv.get_bytecode().as_ref());
    let mut receiver = instance.to_java(indent);
    if instance.get_precedence() > 0 {
        receiver.enclose("(", ")");
    }
    buf.append_char('(')
        .append_buffer(&receiver)
        .append(" as java.lang.Object).")
        .append(&name)
        .append_char('(')
        .append_buffer(&inv.append_param_list(indent))
        .append_char(')');
    Some(buf)
}

/// `value` without the casts to `java.lang.Object` around it.
fn uncast(value: Exprent) -> Exprent {
    match value.try_as_function() {
        Some(func)
            if func.get_func_type() == FunctionType::Cast
                && func
                    .get_lst_operands()
                    .get(1)
                    .get_expr_type()
                    .value
                    .as_deref()
                    == Some("java/lang/Object")
                && func.get_lst_operands().get(1).get_expr_type().array_dim == 0 =>
        {
            uncast(func.get_lst_operands().get(0))
        }
        _ => value,
    }
}
