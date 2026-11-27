//! Fix family `kt_compiler_idioms` (Kotlin output): kotlinc's inlined library
//! code and `Nothing` throws, written as the source.

pub(crate) mod kt_inlined_stdlib_call;
pub(crate) mod kt_inlined_to_typed_array;
pub(crate) mod kt_nothing_value_throw;
