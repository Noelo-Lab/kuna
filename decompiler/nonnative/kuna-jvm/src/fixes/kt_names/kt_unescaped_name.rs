//! Fix `kt_unescaped_name`: the names the writers Kotlin shares with Java print
//! (captured variables, lambda parameters, fields, accessor calls) go through
//! `KotlinWriter::to_valid_kotlin_identifier`; an anonymous lambda parameter is
//! `_`.

use crate::kotlin_writer::KotlinWriter;
use crate::fixes::kt_util::writing_kotlin;
use crate::var_exprent::VarExprent;

/// A variable's name as the Kotlin writer prints it, when the class being
/// written is Kotlin; else `name`.
pub fn variable(name: &str) -> String {
    if !writing_kotlin()
        || name == "this"
        || name.starts_with("this@")
        || name == VarExprent::VAR_NAMELESS_ENCLOSURE
    {
        return name.to_string();
    }
    KotlinWriter::to_valid_kotlin_identifier(name)
}

/// A field's or method's name as the Kotlin writer prints it, when the
/// class being written is Kotlin; else `name`.
pub fn member(name: &str) -> String {
    if !writing_kotlin() {
        return name.to_string();
    }
    KotlinWriter::to_valid_kotlin_identifier(name)
}

/// A lambda parameter's name in the lambda's header: `_` for kotlinc's
/// `<anonymous parameter N>`, else as [`variable`] writes it.
pub fn lambda_parameter(name: &str) -> String {
    let anonymous = name
        .strip_prefix("<anonymous parameter ")
        .and_then(|rest| rest.strip_suffix('>'))
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|c| c.is_ascii_digit()));
    if anonymous && writing_kotlin() {
        return "_".to_string();
    }
    variable(name)
}
