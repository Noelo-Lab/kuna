//! Fix `kt_escaped_class_name`: each segment of a class name that is no Kotlin
//! identifier is written in backticks in types, expressions and imports, as its
//! declaration is.

use crate::kotlin_writer::KotlinWriter;

/// `name`, a simple class name, in backticks when Kotlin needs them.
pub fn escape(name: &str) -> String {
    if name.is_empty()
        || name.contains(['$', '`', '/', '<', '>', ';', '[', ']', ':', '\\'])
        || name.bytes().all(|c| c.is_ascii_digit())
    {
        return name.to_string();
    }
    KotlinWriter::to_valid_kotlin_identifier(name)
}

/// `name`, a dotted class name, with each segment as [`escape`] writes it.
pub fn escape_qualified(name: &str) -> String {
    if !name.split('.').any(|segment| escape(segment) != segment) {
        return name.to_string();
    }
    name.split('.').map(escape).collect::<Vec<_>>().join(".")
}
