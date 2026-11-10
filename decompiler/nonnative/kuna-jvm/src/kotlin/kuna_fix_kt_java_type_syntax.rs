//! kuna fix `kt_java_type_syntax`: in Kotlin output, a type read from a Java
//! generic signature is written in Kotlin's syntax.
//!
//! `KTypes.getKotlinType` converts the type arguments of the `java.util`
//! collection classes it translates but drops their wildcards
//! (`MutableList<R>` for `List<? super R>`, a different type); any other
//! generic class falls back to Java's `getCastTypeName` and a string rewrite
//! that handles one wildcard or array at the outside of the name only, so
//! nested arguments keep Java's syntax: `Continuation<? super
//! java.lang.Boolean>`, `Flow<Flow<? extends R>>`. The writers the Kotlin plugin shares with Java's class writer, for
//! a class header and for a method the metadata does not describe (an
//! interface's `DefaultImpls`, a synthetic class, a `$$forInline` copy, the
//! object of fix `kt_synthetic_anonymous_object`), print supertypes with
//! `getCastTypeName` (`object : Function0<T[]>`, `KClass<Object>`) and type
//! parameters as Java does (`<T extends Comparable<? super T>>`), and a
//! class literal of an array type is `Object[]::class.java`. kotlinc
//! rejects all of these. With the fix a type argument is converted
//! (`? extends X` is `out X`, `? super X` is `in X`, `?` is `*`, `T[]` is
//! `Array<T>`, `int[]` `IntArray`, `Object` `Any`), a generic class outside
//! `getKotlinType`'s table keeps its name with its arguments converted, a
//! header's supertype or bound keeps the class name Vineflower writes
//! (`Function0<Array<T>>`, `java.lang.Iterable<Int>`: valid Kotlin, and not
//! the function-type form with the nullable parameters `getKotlinType`
//! gives `FunctionN`), a type parameter's bound is written `T : B`, and a
//! type parameter with more than one bound is constrained in a `where`
//! clause, as Kotlin requires.

use crate::code_type::CodeType;
use crate::expr_processor::ExprProcessor;
use crate::generic_type::GenericType;
use crate::k_types::KTypes;
use crate::var_type::VarType;

/// A generic class type with type arguments in Kotlin's syntax
/// (`Flow<out R>`, `Array<Pair<K, in V>>`); `None` for any other type.
pub fn generic_class_type(type_: &VarType, include_outer_classes: bool) -> Option<String> {
    let generic = type_.try_as_generic()?;
    if type_.type_ != CodeType::Object
        || generic.get_arguments().is_empty()
        || generic.get_parent().is_some()
    {
        return None;
    }
    let erasure = VarType::new_value(CodeType::Object, 0, type_.value_str());
    let mut name = KTypes::get_kotlin_type_outer(Some(&erasure), include_outer_classes);
    name.push('<');
    for (i, argument) in generic.get_arguments().iter().enumerate() {
        if i > 0 {
            name.push_str(", ");
        }
        name.push_str(&type_argument(argument.as_ref()));
    }
    name.push('>');
    let dims = type_.array_dim.max(0) as usize;
    Some(format!(
        "{}{}{}",
        "Array<".repeat(dims),
        name,
        ">".repeat(dims)
    ))
}

/// A header's supertype or a type parameter's bound: the class's name as
/// Java's writer gives it, with its type arguments in Kotlin's syntax.
pub fn supertype(type_: &VarType) -> String {
    match type_.try_as_generic() {
        Some(generic)
            if type_.type_ == CodeType::Object
                && type_.array_dim == 0
                && !generic.get_arguments().is_empty()
                && generic.get_parent().is_none() =>
        {
            let erasure = VarType::new_value(CodeType::Object, 0, type_.value_str());
            let mut name = ExprProcessor::get_cast_type_name(&erasure);
            name.push('<');
            for (i, argument) in generic.get_arguments().iter().enumerate() {
                if i > 0 {
                    name.push_str(", ");
                }
                name.push_str(&type_argument(argument.as_ref()));
            }
            name.push('>');
            name
        }
        _ => ExprProcessor::get_cast_type_name(type_),
    }
}

/// A type argument in Kotlin's syntax: a wildcard as a projection.
pub fn type_argument(argument: Option<&VarType>) -> String {
    let Some(argument) = argument else {
        return "*".to_string();
    };
    let wildcard = argument
        .try_as_generic()
        .map_or(GenericType::WILDCARD_NO, |generic| generic.get_wildcard());
    match wildcard {
        GenericType::WILDCARD_UNBOUND => "*".to_string(),
        GenericType::WILDCARD_EXTENDS | GenericType::WILDCARD_SUPER => {
            let bound = GenericType::with_wildcard(argument, GenericType::WILDCARD_NO);
            let variance = if wildcard == GenericType::WILDCARD_EXTENDS {
                "out"
            } else {
                "in"
            };
            format!("{variance} {}", KTypes::get_kotlin_type(Some(&bound)))
        }
        _ => KTypes::get_kotlin_type(Some(argument)),
    }
}

/// Kotlin type parameters (`<T : B, U>`) for Java's parameters and their
/// bounds, and the `where` clause (` where T : A, T : B`, else empty) that
/// constrains a parameter with more than one bound.
pub fn type_parameters(parameters: &[String], bounds: &[Vec<VarType>]) -> (String, String) {
    let mut list = String::from("<");
    let mut constraints: Vec<String> = Vec::new();
    for (i, parameter) in parameters.iter().enumerate() {
        if i > 0 {
            list.push_str(", ");
        }
        list.push_str(parameter);
        let parameter_bounds = &bounds[i];
        if parameter_bounds.len() > 1 {
            for bound in parameter_bounds {
                constraints.push(format!("{parameter} : {}", bound_type(bound)));
            }
        } else if parameter_bounds[0].value.as_deref() != Some("java/lang/Object") {
            list.push_str(" : ");
            list.push_str(&bound_type(&parameter_bounds[0]));
        }
    }
    list.push('>');
    let clause = if constraints.is_empty() {
        String::new()
    } else {
        format!(" where {}", constraints.join(", "))
    };
    (list, clause)
}

/// A type parameter's bound: [`supertype`], except that a JDK class Kotlin
/// maps to a built-in type has the built-in's name when fix
/// `kt_mapped_java_type` is on (`T : Throwable`, as Kotlin source writes it;
/// an explicit `java.lang.Throwable` bound rejects Kotlin's `Throwable`).
fn bound_type(type_: &VarType) -> String {
    use crate::kuna_fixes::{self, KunaFix};
    match crate::kotlin::kuna_fix_kt_mapped_java_type::kotlin_name(type_)
        .filter(|_| type_.array_dim == 0 && kuna_fixes::enabled(KunaFix::KtMappedJavaType))
    {
        Some(name) => name.to_string(),
        None => supertype(type_),
    }
}
