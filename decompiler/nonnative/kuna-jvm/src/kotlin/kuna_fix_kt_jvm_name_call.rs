//! kuna fix `kt_jvm_name_call`: a call of a Kotlin function is written with
//! the function's Kotlin name, not its JVM name.
//!
//! kotlinc names a function's method differently from the function in two
//! cases: an `internal` member gets its module's name appended
//! (`bump$main`, `canonicalize$okhttp`), and `@JvmName` renames it
//! (`@JvmName("url") fun toUrl()`, okio's `@JvmName("getByte") operator fun
//! get`). Vineflower writes the call with the method's name, which Kotlin
//! cannot resolve (`` b.`bump$main`() ``) or resolves to the deprecated
//! alias the library keeps under the JVM name (`url.url()`, `s.getByte(i)`).
//! With the fix a call whose callee's metadata names the function otherwise
//! is written with the metadata's name, `b.bump()`, `url.toUrl()`,
//! `s.get(i)`; the call's instance and arguments are Vineflower's. A name
//! that an inline-class parameter mangled (`f-QeA7wkU`), which another
//! overload of the Kotlin name may take, keeps the JVM name. Vineflower
//! reads the callee's metadata from the class the call names only, so a
//! member the class inherits (`this.responseBodyComplete$okhttp()` in a
//! subclass, an inherited property's `getClosed()`) kept its JVM name; with
//! the fix the metadata of the class that declares it is read too. The
//! `invoke` of a lambda's or local function's class keeps its name (its
//! metadata names the local function, which is no member).

use crate::decompiler_context::DecompilerContext;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::kotlin::kuna_fix_kt_facade_call;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

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
    if kuna_fix_kt_facade_call::mangled(&jvm, &function.name) {
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
