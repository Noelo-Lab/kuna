//! Port of `org/vineflower/kotlin/util/KTypes.java`.
//!
//! The `Map.of`/`Map.ofEntries` tables are slices in declaration order: the
//! code only looks them up, except `KotlinImportCollector`, which walks the
//! keys of `KOTLIN_TO_JAVA_LANG` and `KOTLIN_TO_JAVA_UTIL` (whose JDK order is
//! salted per JVM run).

#![allow(dead_code, unused_variables)]

use crate::unported;
use crate::var_type::VarType;

pub struct KTypes;

impl KTypes {
    const MAX_KOTLIN_FUNCTION_ARITY: i32 = 22;

    pub const UNIT: &'static str = "kotlin/Unit";

    const JAVA_CLASS_TRANSLATIONS: &'static [(&'static str, &'static str)] =
        &[("java/lang/Integer", "Int"), ("java/lang/Object", "Any")];

    const JAVA_GENERIC_TRANSLATIONS: &'static [(&'static str, &'static str)] = &[
        ("java/util/Map", "MutableMap"),
        ("java/util/HashMap", "HashMap"),
        ("java/util/List", "MutableList"),
        ("java/util/ArrayList", "ArrayList"),
        ("java/util/Set", "MutableSet"),
        ("java/util/HashSet", "HashSet"),
        ("java/util/Collection", "MutableCollection"),
        ("java/util/Iterator", "MutableIterator"),
        ("java/util/Map$Entry", "MutableMap.MutableEntry"),
        ("java/lang/Iterable", "MutableIterable"),
    ];

    pub const KOTLIN_TO_JAVA_LANG: &'static [(&'static str, &'static str)] = &[
        ("kotlin/Any", "java/lang/Object"),
        ("kotlin/Boolean", "java/lang/Boolean"),
        ("kotlin/Byte", "java/lang/Byte"),
        ("kotlin/Char", "java/lang/Character"),
        ("kotlin/CharSequence", "java/lang/CharSequence"),
        ("kotlin/Comparable", "java/lang/Comparable"),
        ("kotlin/Double", "java/lang/Double"),
        ("kotlin/Enum", "java/lang/Enum"),
        ("kotlin/Float", "java/lang/Float"),
        ("kotlin/Int", "java/lang/Integer"),
        ("kotlin/Long", "java/lang/Long"),
        ("kotlin/Nothing", "java/lang/Void"),
        ("kotlin/Number", "java/lang/Number"),
        ("kotlin/Short", "java/lang/Short"),
        ("kotlin/String", "java/lang/String"),
        ("kotlin/Throwable", "java/lang/Throwable"),
        ("kotlin/collections/MutableIterable", "java/lang/Iterable"),
        ("kotlin/collections/Iterable", "java/lang/Iterable"),
    ];

    pub const KOTLIN_TO_JAVA_UTIL: &'static [(&'static str, &'static str)] = &[
        (
            "kotlin/collections/MutableCollection",
            "java/util/Collection",
        ),
        ("kotlin/collections/MutableMap", "java/util/Map"),
        ("kotlin/collections/MutableList", "java/util/List"),
        ("kotlin/collections/MutableSet", "java/util/Set"),
        ("kotlin/collections/MutableIterator", "java/util/Iterator"),
        (
            "kotlin/collections/MutableListIterator",
            "java/util/ListIterator",
        ),
        (
            "kotlin/collections/MutableMap$MutableEntry",
            "java/util/Map$Entry",
        ),
        ("kotlin/collections/Collection", "java/util/Collection"),
        ("kotlin/collections/Map", "java/util/Map"),
        ("kotlin/collections/List", "java/util/List"),
        ("kotlin/collections/ListIterator", "java/util/ListIterator"),
        ("kotlin/collections/Set", "java/util/Set"),
        ("kotlin/collections/Iterator", "java/util/Iterator"),
        ("kotlin/collections/Map$Entry", "java/util/Map$Entry"),
    ];

    const KOTLIN_PRIMITIVE_TYPES: &'static [(&'static str, &'static str)] = &[
        ("kotlin/Int", "I"),
        ("kotlin/Long", "J"),
        ("kotlin/Short", "S"),
        ("kotlin/Byte", "B"),
        ("kotlin/Boolean", "Z"),
        ("kotlin/Char", "C"),
        ("kotlin/Float", "F"),
        ("kotlin/Double", "D"),
        ("kotlin/Unit", "V"),
    ];

    /// `table.get(key)` of one of the tables above.
    pub fn lookup(table: &[(&'static str, &'static str)], key: &str) -> Option<&'static str> {
        table.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
    }

    /// `GENERICS_PATTERN.matcher(input).matches()` (`(.+)<(.+)>`), with groups 1 and 2.
    fn generics_pattern_match(input: &str) -> Option<(&str, &str)> {
        if input
            .chars()
            .any(|c| matches!(c, '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}'))
        {
            return None;
        }
        let body = input.strip_suffix('>')?;
        let (last_char, _) = body.char_indices().last()?;
        let lt = body[..last_char].rfind('<')?;
        if lt == 0 {
            return None;
        }
        Some((&body[..lt], &body[lt + 1..]))
    }

    pub fn get_java_signature(kotlin_type: &str, is_nullable: bool) -> String {
        unported!("KTypes.getJavaSignature")
    }

    /// Java: getKotlinType(VarType); `null` gives `*`.
    pub fn get_kotlin_type(type_: Option<&VarType>) -> String {
        KTypes::get_kotlin_type_outer(type_, true)
    }

    /// Java: getKotlinType(VarType, boolean)
    pub fn get_kotlin_type_outer(type_: Option<&VarType>, include_outer_classes: bool) -> String {
        unported!("KTypes.getKotlinType")
    }

    fn map_java_type_to_kotlin(type_: &str) -> String {
        unported!("KTypes.mapJavaTypeToKotlin")
    }

    pub fn is_function_type(type_: &VarType) -> bool {
        type_
            .value
            .as_deref()
            .expect("NullPointerException: type.value")
            .starts_with("kotlin/jvm/functions/Function")
    }

    fn function_type_to_kotlin(type_: &VarType) -> String {
        unported!("KTypes.functionTypeToKotlin")
    }
}
