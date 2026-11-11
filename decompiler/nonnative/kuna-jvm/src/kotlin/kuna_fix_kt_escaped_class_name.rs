//! kuna fix `kt_escaped_class_name`: in Kotlin output, a class whose Kotlin
//! name is no identifier is named in backticks in types, expressions and
//! imports.
//!
//! A Kotlin class may have any name in backticks (okio's
//! ``object `-DeprecatedOkio` ``, a nested ``class `in` ``). `KotlinWriter`
//! puts the name in backticks where it declares the class, but every other
//! place takes the name from `ImportCollector.getShortName` (types, the
//! object or class a member is read from, `import` lines) or from the class
//! node (`outer.Inner(..)`), unescaped: `val Okio: -DeprecatedOkio =
//! -DeprecatedOkio` is no Kotlin. With the fix each segment of such a name
//! that is no Kotlin identifier is written in backticks, as the declaration
//! is. A segment with `$` (a synthetic class's binary name, which Kotlin
//! cannot name at all), an anonymous class's number (`fn.1.Local`), a marker
//! (`<unrepresentable>`) or a character no JVM simple name has
//! (`kotlin/RequiresOptIn`) is left as it is.

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
