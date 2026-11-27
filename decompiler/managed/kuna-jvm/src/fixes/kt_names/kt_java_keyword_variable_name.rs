//! Fix `kt_java_keyword_variable_name`: in Kotlin output a debug variable name
//! that is a Java keyword but not reserved in Kotlin (`new`, `default`) is taken
//! from the local variable table.

use crate::fixes::kt_util::writing_kotlin;
use crate::text_util::TextUtil;

/// Whether `name`, a debug name `TextUtil.isValidIdentifier` refused, names
/// the variable in the class being decompiled.
pub fn accepts(name: &str) -> bool {
    name != "this" && TextUtil::is_java_identifier(name) && writing_kotlin()
}
