//! Port of `org/vineflower/kotlin/util/KTypes.java`.
//!
//! The `Map.of`/`Map.ofEntries` tables are slices in declaration order: the
//! code only looks them up, except `KotlinImportCollector`, which walks the
//! keys of `KOTLIN_TO_JAVA_LANG` and `KOTLIN_TO_JAVA_UTIL` (whose JDK order is
//! salted per JVM run).
//!
//! kuna fix `kt_mapped_java_type`: a JDK class that Kotlin maps to a
//! built-in type is written with the built-in's name
//! (`kuna_fix_kt_mapped_java_type`).
//!
//! kuna fix `kt_java_type_syntax`: `get_kotlin_type` writes a generic class
//! type's arguments in Kotlin's syntax (`kuna_fix_kt_java_type_syntax`).

use crate::expr_processor::ExprProcessor;
use crate::jthrow;
use crate::kotlin::kuna_fix_kt_mapped_java_type;
use crate::kotlin::kuna_fix_kt_java_type_syntax;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::{jnum, try_catch};
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
        let mut kotlin_type = kotlin_type.to_string();
        if kotlin_type.starts_with('L') && kotlin_type.ends_with(';') {
            kotlin_type = kotlin_type[1..kotlin_type.len() - 1].to_string();
        }

        if kotlin_type.contains('.') {
            kotlin_type = kotlin_type.replace('.', "$");
        }

        if kotlin_type.starts_with("kotlin/") {
            let primitive = KTypes::lookup(KTypes::KOTLIN_PRIMITIVE_TYPES, &kotlin_type);
            if primitive.is_some() && !is_nullable {
                return primitive.unwrap().to_string();
            } else if kotlin_type.ends_with("$Companion")
                && KTypes::lookup(
                    KTypes::KOTLIN_PRIMITIVE_TYPES,
                    &kotlin_type.replace("$Companion", ""),
                )
                .is_some()
            {
                let parts = java_split(&kotlin_type, '/');
                let simple = java_split(array_get(&parts, 1), '$');
                return format!(
                    "Lkotlin/jvm/internal/{}CompanionObject;",
                    array_get(&simple, 0)
                );
            } else if kotlin_type.ends_with("Array")
                && KTypes::lookup(
                    KTypes::KOTLIN_PRIMITIVE_TYPES,
                    &kotlin_type.replace("Array", ""),
                )
                .is_some()
            {
                let element = KTypes::lookup(
                    KTypes::KOTLIN_PRIMITIVE_TYPES,
                    &kotlin_type.replace("Array", ""),
                );
                return format!("[{}", element.unwrap());
            } else if let Some(java) = KTypes::lookup(KTypes::KOTLIN_TO_JAVA_LANG, &kotlin_type) {
                return format!("L{};", java);
            } else if let Some(java) = KTypes::lookup(KTypes::KOTLIN_TO_JAVA_UTIL, &kotlin_type) {
                return format!("L{};", java);
            } else if let Some(arity) = kotlin_type.strip_prefix("kotlin/Function") {
                match try_catch(|| jnum::parse_int(arity)) {
                    Ok(arity) => {
                        if arity > KTypes::MAX_KOTLIN_FUNCTION_ARITY {
                            return "Lkotlin/jvm/functions/FunctionN;".to_string();
                        }
                        return format!("Lkotlin/jvm/functions{};", &kotlin_type["kotlin".len()..]);
                    }
                    Err(e) if e.is("NumberFormatException") => {}
                    Err(e) => crate::rt::rethrow(e),
                }
            }
        }
        format!("L{};", kotlin_type)
    }

    /// Java: getKotlinType(VarType); `null` gives `*`.
    pub fn get_kotlin_type(type_: Option<&VarType>) -> String {
        KTypes::get_kotlin_type_outer(type_, true)
    }

    /// Java: getKotlinType(VarType, boolean)
    pub fn get_kotlin_type_outer(type_: Option<&VarType>, include_outer_classes: bool) -> String {
        let Some(type_) = type_ else {
            return "*".to_string();
        };

        let type_str;
        if KTypes::is_function_type(type_) {
            type_str = KTypes::function_type_to_kotlin(type_);
        } else if let Some(translated) =
            KTypes::lookup(KTypes::JAVA_CLASS_TRANSLATIONS, type_.value_str())
        {
            type_str = translated.to_string();
        } else if let Some(translated) = kuna_fix_kt_mapped_java_type::kotlin_name(type_)
            .filter(|_| kuna_fixes::enabled(KunaFix::KtMappedJavaType))
        {
            type_str = translated.to_string();
        } else if KTypes::lookup(KTypes::JAVA_GENERIC_TRANSLATIONS, type_.value_str()).is_some()
            && type_.is_generic()
        {
            let generic_type = type_.as_generic();
            let arguments = generic_type.get_arguments();
            let generic_name =
                KTypes::lookup(KTypes::JAVA_GENERIC_TRANSLATIONS, type_.value_str()).unwrap();
            if arguments.is_empty() {
                type_str = generic_name.to_string();
            } else {
                let mut builder = String::from(generic_name);
                builder.push('<');
                for (i, argument) in arguments.iter().enumerate() {
                    if i != 0 {
                        builder.push_str(", ");
                    }
                    if kuna_fixes::enabled(KunaFix::KtJavaTypeSyntax) {
                        builder.push_str(&kuna_fix_kt_java_type_syntax::type_argument(
                            argument.as_ref(),
                        ));
                    } else {
                        builder.push_str(&KTypes::get_kotlin_type(argument.as_ref()));
                    }
                }
                builder.push('>');
                type_str = builder;
            }
        } else {
            if kuna_fixes::enabled(KunaFix::KtJavaTypeSyntax) {
                if let Some(s) =
                    kuna_fix_kt_java_type_syntax::generic_class_type(type_, include_outer_classes)
                {
                    return s;
                }
            }
            let mut s = ExprProcessor::get_cast_type_name(type_);
            if ExprProcessor::UNDEFINED_TYPE_STRING == s {
                s = "Any".to_string();
            }
            if !include_outer_classes {
                s = match s.rfind('.') {
                    Some(dot) => s[dot + 1..].to_string(),
                    None => s,
                };
            }

            return KTypes::map_java_type_to_kotlin(&s);
        }
        if type_.array_dim < 0 {
            jthrow!(IllegalArgumentException; "count is negative: {}", type_.array_dim);
        }
        let array_dim = type_.array_dim as usize;
        format!(
            "{}{}{}",
            "Array<".repeat(array_dim),
            type_str,
            ">".repeat(array_dim)
        )
    }

    fn map_java_type_to_kotlin(type_: &str) -> String {
        if type_.starts_with('?') {
            if type_ == "?" {
                return "*".to_string();
            } else if let Some(bound) = type_.strip_prefix("? extends ") {
                return format!("out {}", bound);
            } else if let Some(bound) = type_.strip_prefix("? super ") {
                return format!("in {}", bound);
            }
        }

        if let Some(base_type) = type_.strip_suffix("[]") {
            return match base_type {
                "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double" => {
                    format!("{}Array", KTypes::map_java_type_to_kotlin(base_type))
                }
                _ => format!("Array<{}>", KTypes::map_java_type_to_kotlin(base_type)),
            };
        }

        match type_ {
            "boolean" => "Boolean".to_string(),
            "byte" => "Byte".to_string(),
            "char" => "Char".to_string(),
            "short" => "Short".to_string(),
            "int" => "Int".to_string(),
            "long" => "Long".to_string(),
            "float" => "Float".to_string(),
            "double" => "Double".to_string(),
            "void" => "Unit".to_string(),
            _ => {
                let parts = java_split(type_, '.');
                let mut sb = String::new();
                let mut append_dot = false;
                for part in parts {
                    let matcher = KTypes::generics_pattern_match(part);

                    if append_dot {
                        sb.push('.');
                    }
                    append_dot = true;

                    if let Some((group1, group2)) = matcher {
                        sb.push_str(&KTypes::map_java_type_to_kotlin(group1));
                        sb.push('<');
                        let mut append_comma = false;
                        for generic in java_split(group2, ',') {
                            if append_comma {
                                sb.push_str(", ");
                            }
                            sb.push_str(&KTypes::map_java_type_to_kotlin(java_trim(generic)));
                            append_comma = true;
                        }
                        sb.push('>');
                    } else {
                        sb.push_str(part);
                    }
                }
                sb
            }
        }
    }

    pub fn is_function_type(type_: &VarType) -> bool {
        type_
            .value_str()
            .starts_with("kotlin/jvm/functions/Function")
    }

    fn function_type_to_kotlin(type_: &VarType) -> String {
        if !KTypes::is_function_type(type_) {
            jthrow!(IllegalArgumentException; "Not a function type: {}", type_);
        }

        let param_count = &type_.value_str()["kotlin/jvm/functions/Function".len()..];
        let generic_type = match type_.try_as_generic() {
            Some(generic_type) if param_count != "N" => generic_type,
            _ => {
                let type_str = ExprProcessor::get_cast_type_name(type_);
                if ExprProcessor::UNDEFINED_TYPE_STRING == type_str {
                    return "Any".to_string();
                }
                return KTypes::map_java_type_to_kotlin(&type_str);
            }
        };

        let params = generic_type.get_arguments();

        let param_count_int = jnum::parse_int(param_count);
        let mut sb = String::new();
        sb.push('(');
        for i in 0..param_count_int {
            if i > 0 {
                sb.push_str(", ");
            }

            let param = list_get(params, i);
            let mut param_str = KTypes::get_kotlin_type(param.as_ref());
            if KTypes::is_function_type(non_null(param)) {
                param_str = format!("({})", param_str);
            }
            sb.push_str(&param_str);
            sb.push('?');
        }

        sb.push_str(") -> ");
        let return_type = list_get(params, param_count_int);
        let mut return_str = KTypes::get_kotlin_type(return_type.as_ref());
        if KTypes::is_function_type(non_null(return_type)) {
            return_str = format!("({})", return_str);
        }
        sb.push_str(&return_str);

        sb
    }
}

/// `s.split(regex)` for a one-character literal regex: trailing empty strings
/// are dropped, and a string without the separator is the only element.
fn java_split(s: &str, separator: char) -> Vec<&str> {
    if !s.contains(separator) {
        return vec![s];
    }
    let mut parts: Vec<&str> = s.split(separator).collect();
    while parts.last() == Some(&"") {
        parts.pop();
    }
    parts
}

/// `array[index]` on the result of `split`.
fn array_get<'a>(array: &[&'a str], index: usize) -> &'a str {
    match array.get(index) {
        Some(s) => s,
        None => {
            jthrow!(ArrayIndexOutOfBoundsException; "Index {} out of bounds for length {}", index, array.len())
        }
    }
}

/// `String.trim()`.
fn java_trim(s: &str) -> &str {
    s.trim_matches(|c: char| c <= ' ')
}

/// `list.get(index)` on a `GenericType`'s argument list.
fn list_get(list: &[Option<VarType>], index: i32) -> &Option<VarType> {
    if index < 0 || index as usize >= list.len() {
        jthrow!(IndexOutOfBoundsException; "Index {} out of bounds for length {}", index, list.len());
    }
    &list[index as usize]
}

/// A dereference of a possibly-null type.
fn non_null(type_: &Option<VarType>) -> &VarType {
    match type_ {
        Some(type_) => type_,
        None => jthrow!(NullPointerException; "type is null"),
    }
}
