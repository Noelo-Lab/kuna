//! Fix `kt_mapped_java_type`: a JDK class Kotlin maps to a built-in type
//! (`java.lang.String`, `Throwable`, the boxes) is written by the built-in's
//! name, except a box's constructor call.

use crate::code_type::CodeType;
use crate::var_type::VarType;

const MAPPED: &[(&str, &str)] = &[
    ("java/lang/String", "String"),
    ("java/lang/CharSequence", "CharSequence"),
    ("java/lang/Throwable", "Throwable"),
    ("java/lang/Number", "Number"),
    ("java/lang/Long", "Long"),
    ("java/lang/Short", "Short"),
    ("java/lang/Byte", "Byte"),
    ("java/lang/Character", "Char"),
    ("java/lang/Float", "Float"),
    ("java/lang/Double", "Double"),
    ("java/lang/Boolean", "Boolean"),
];

/// The Kotlin name of `type_` (its class, arrays aside) when Kotlin maps it.
pub fn kotlin_name(type_: &VarType) -> Option<&'static str> {
    if type_.type_ != CodeType::Object {
        return None;
    }
    let value = type_.value.as_deref()?;
    MAPPED
        .iter()
        .find(|(java, _)| *java == value)
        .map(|(_, kotlin)| *kotlin)
}

/// Whether a constructor call of `type_` keeps its Java name.
pub fn java_constructor(type_: &VarType) -> bool {
    kotlin_name(type_).is_some_and(|name| !matches!(name, "String" | "Throwable"))
}
