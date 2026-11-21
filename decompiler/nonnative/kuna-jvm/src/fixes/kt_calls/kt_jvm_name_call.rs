//! Fix `kt_jvm_name_call`: a call is written with the Kotlin name its callee's
//! metadata (or that of the class declaring it) gives, where the JVM name is
//! an `internal` member's mangled name or a `@JvmName`; a name an inline-class
//! parameter mangled keeps the JVM name.

use crate::decompiler_context::DecompilerContext;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::fixes::kt_calls::kt_facade_call;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

/// The Kotlin name to write the call of `function` with, when it is not its
/// JVM name.
pub fn kotlin_name(invocation: InvocationExprent, function: &KFunction) -> Option<String> {
    let jvm = invocation.get_name();
    if function.name == jvm
        || function.name.is_empty()
        || function.name.starts_with('<')
        || function.class_struct.is_none_or(synthetic_class)
    {
        return None;
    }
    if kt_facade_call::mangled(&jvm, &function.name)
        || kt_facade_call::value_class_argument(invocation)
    {
        return None;
    }
    Some(function.name.clone())
}

/// Whether `cl` is a lambda's or local function's class, whose one function
/// (`invoke`) the metadata names after the source's local function.
pub fn synthetic_class(cl: StructClass) -> bool {
    matches!(
        KtAttrs::get(cl, &KElement::KEY()),
        Some(KElement::Function(_)) | Some(KElement::FunctionReference(_))
    )
}

/// The method a call of `cl`'s `name` and `descriptor` reaches in a
/// supertype, with its declaring class's metadata parsed.
pub fn inherited(cl: StructClass, name: &str, descriptor: &str) -> Option<StructMethod> {
    let method = cl.get_method_recursive(name, descriptor)?;
    let owner = DecompilerContext::get_struct_context().get_class(&method.get_class_qualified_name())?;
    KotlinChooser::parse_metadata_for(owner);
    Some(method)
}

/// `inv` written by `InvocationExprent::to_java` with the name `name`, which
/// is taken off the invocation again afterwards, also on an unwinding throw.
pub fn to_java_named(inv: InvocationExprent, name: String, indent: i32) -> TextBuffer {
    struct Clear(InvocationExprent);
    impl Drop for Clear {
        fn drop(&mut self) {
            self.0.d_mut(|d| d.kuna_kotlin_name = None);
        }
    }
    inv.d_mut(|d| d.kuna_kotlin_name = Some(name));
    let _clear = Clear(inv);
    InvocationExprent::to_java(inv, indent)
}
