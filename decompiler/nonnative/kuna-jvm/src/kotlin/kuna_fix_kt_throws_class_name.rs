//! kuna fix `kt_throws_class_name`: the `@Throws` written for a function's
//! `throws` clause names each exception class as Kotlin does.
//!
//! kotlinc records `@Throws(IOException::class)` as the method's
//! `Exceptions` attribute, which lists internal class names. The plugin
//! writes them as they are, `@Throws(java/io/IOException::class)`, which
//! Kotlin reads as a division of unresolved names. With the fix each name is
//! written as the class's Kotlin type name, imported like any other type
//! (`IOException::class`, `Outer.Failure::class`, `Throwable::class`).

use crate::k_types::KTypes;
use crate::var_type::VarType;

/// The Kotlin name of the exception class with internal name `name`.
pub fn kotlin_name(name: &str) -> String {
    KTypes::get_kotlin_type(Some(&VarType::new_signature_cl(name, true)))
}
