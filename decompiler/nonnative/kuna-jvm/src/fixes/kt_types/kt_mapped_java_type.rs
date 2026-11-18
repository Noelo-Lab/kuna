//! kuna fix `kt_mapped_java_type`: a type that is a JDK class Kotlin maps
//! to one of its built-in types is written with the built-in's name.
//!
//! Kotlin reads `java.lang.String`, `CharSequence`, `Throwable`, `Number`
//! and the boxed primitives in Java signatures as its own `String`,
//! `CharSequence`, `Throwable`, `Number`, `Long`, ...: the same JVM classes.
//! Named explicitly in Kotlin source, `java.lang.String` is another type
//! (the Java class seen without the mapping), so a local declared
//! `val tag: java.lang.String = loggerTag(name)`, a cast `as
//! java.lang.CharSequence` or a `catch (e: java.lang.Throwable)` does not
//! accept the `String` or `Throwable` it is given. Vineflower writes every
//! class without a Kotlin translation by its Java name, except
//! `java.lang.Integer` (`Int`) and `java.lang.Object` (`Any`). With the fix
//! the other mapped classes without type parameters are written as Kotlin
//! names them; the class file, and what kotlinc compiles the type to, is
//! the same. A constructor call keeps the Java name of a boxed type (Kotlin's
//! `Char`, `Long`, ... have no such constructors); `String(..)` and
//! `Throwable(..)` are Kotlin's own functions and constructors.

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
