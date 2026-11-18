//! kuna fix `kt_companion_static_copy`: only a static method of a class is a
//! copy of its companion object's member.
//!
//! Writing a class with a companion object reads the companion's functions
//! and properties again with the class as parent and marks every method of
//! the class with the JVM name and descriptor of a companion function or
//! accessor `HiddenElement(CompanionItem)`, for the static copy kotlinc emits
//! for a `@JvmStatic` member. An instance method of the class with that
//! signature is the class's own member, which the companion overrides
//! (`companion object Default : Random()` overrides every function of
//! `kotlin.random.Random`; an interface's companion implements the
//! interface) or shadows (a private function of the same name): the mark
//! replaced its element and the class was written without it. With the fix
//! only a static method is marked; a class cannot hold a static and an
//! instance method of one signature, and kotlinc makes a member of a class
//! with a companion static only for `@JvmStatic`.

use crate::code_constants::ACC_STATIC;
use crate::struct_method::StructMethod;

/// Whether `in_parent`, a method of a companion's class with a companion
/// member's JVM signature, is that member's static copy.
pub fn is_copy(in_parent: StructMethod) -> bool {
    in_parent.has_modifier(ACC_STATIC)
}
