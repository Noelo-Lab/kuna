//! Fix family `kt_delegation_calls` (Kotlin output): a constructor's
//! superclass or `this(..)` call: its supertype's type arguments, the empty
//! call `hideEmptySuper` removed, the body after it, the arguments kotlinc
//! computed before a null check (`kt_delegation_call_arguments`) and a call
//! through a default-arguments constructor (`kt_default_args_super_call`).

pub(crate) mod kt_default_args_super_call;
pub(crate) mod kt_delegation_call_arguments;

use crate::fixes::kt_types::kt_type_parameter_id;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_type::KType;
use crate::kt_attrs::KtAttrs;
use crate::method_wrapper::MethodWrapper;
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;

/// The type arguments, `<A, B>`, of the supertype of the Kotlin class `cl`
/// that names `classname` (a JVM internal name), when it has any.
pub fn type_arguments(cl: StructClass, classname: &str, indent: i32) -> Option<TextBuffer> {
    let Some(KElement::Class(cls)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    let resolver = cls.resolver.as_ref()?;
    let _scope = kt_type_parameter_id::class_scope(cl, &cls.proto, resolver);
    let supertype = cls
        .proto
        .get_supertype_list()
        .iter()
        .map(|t| KType::from(t, resolver))
        .find(|t| t.value.as_deref() == Some(classname))?;
    let arguments = supertype.k().type_arguments.clone()?;
    if arguments.is_empty() {
        return None;
    }
    let mut buf = TextBuffer::new();
    buf.append("<");
    for (i, argument) in arguments.iter().enumerate() {
        if i > 0 {
            buf.append(", ");
        }
        buf.append_buffer(&argument.stringify(indent + 1));
    }
    buf.append(">");
    Some(buf)
}

/// The superclass call `hideEmptySuper` removed from `method`, if any.
pub fn hidden_call(method: MethodWrapper) -> Option<InvocationExprent> {
    method.with(|d| d.hidden_super)
}

/// The constructor's rendered body, or `None` when it writes nothing.
pub fn nonempty_body(render: impl FnOnce() -> TextBuffer) -> Option<TextBuffer> {
    let body = render();
    let blank = body
        .convert_to_string_and_allow_data_discard()
        .chars()
        .all(char::is_whitespace);
    (!blank).then_some(body)
}
