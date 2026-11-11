//! kuna fix `kt_java_reference_cast`: in Kotlin output, the casts to a class
//! or array type that Vineflower writes in Java's syntax, `(Sink)delegate`,
//! `(byte[])this.data`, `((Http2Stream)this).id`, which are not Kotlin, are
//! written `delegate as Sink`, `this.data as ByteArray`,
//! `(this as Http2Stream).id`.
//!
//! Java's cast and Kotlin's `as` to a non-null type compile to the same
//! `checkcast` when the operand's Kotlin type is not nullable. Where it may
//! be `null` (the `null` literal, a value of a type variable's type, which
//! Kotlin bounds by `Any?`) the cast is written `as T?`, which lets `null`
//! through as Java's cast does, and so is the upcast to `Object`, `as Any?`,
//! which never fails; so is the cast of a parameter the function's Kotlin
//! metadata declares nullable. A cast to a primitive type from
//! `Object` or a type variable (Java's checked unboxing) is written
//! `x as Int`, Kotlin's. The type is written in Kotlin: `java.lang.String` is
//! `String`, `byte[]` is `ByteArray`, `? extends T` is `out T`, a raw generic
//! class takes star projections, and a collection interface is Kotlin's
//! read-only one (`java.util.List<T>` is `List<T>`): `as List<T>` is a
//! `checkcast`, where `as MutableList<T>` also rejects Kotlin's read-only
//! collections.

use std::cell::RefCell;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::k_types::KTypes;
use crate::kotlin::kuna_fix_kt_nullable_local;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

const KOTLIN_NAMES: &[(&str, &str)] = &[
    ("java/lang/Object", "Any"),
    ("java/lang/String", "String"),
    ("java/lang/CharSequence", "CharSequence"),
    ("java/lang/Throwable", "Throwable"),
    ("java/lang/Number", "Number"),
    ("java/lang/Comparable", "Comparable"),
    ("java/lang/Enum", "Enum"),
    ("java/lang/Cloneable", "Cloneable"),
    ("java/lang/annotation/Annotation", "Annotation"),
    ("java/lang/Boolean", "Boolean"),
    ("java/lang/Byte", "Byte"),
    ("java/lang/Character", "Char"),
    ("java/lang/Short", "Short"),
    ("java/lang/Integer", "Int"),
    ("java/lang/Long", "Long"),
    ("java/lang/Float", "Float"),
    ("java/lang/Double", "Double"),
    ("java/lang/Iterable", "Iterable"),
    ("java/util/Iterator", "Iterator"),
    ("java/util/ListIterator", "ListIterator"),
    ("java/util/Collection", "Collection"),
    ("java/util/List", "List"),
    ("java/util/Set", "Set"),
    ("java/util/Map", "Map"),
    ("java/util/Map$Entry", "Map.Entry"),
];

fn primitive(type_: CodeType) -> bool {
    matches!(
        type_,
        CodeType::Boolean
            | CodeType::Byte
            | CodeType::Short
            | CodeType::Char
            | CodeType::Int
            | CodeType::Long
            | CodeType::Float
            | CodeType::Double
    )
}

/// Whether Java's cast of a value of type `from` to `to` is written `as`.
pub fn applies(to: &VarType, from: &VarType) -> bool {
    if !kuna_fixes::enabled(KunaFix::KtJavaReferenceCast) {
        return false;
    }
    if to.array_dim > 0 || to.type_ == CodeType::Object {
        return true;
    }
    primitive(to.type_)
        && from.array_dim == 0
        && (from.type_ == CodeType::Genvar
            || (from.type_ == CodeType::Object && from.value_str() == "java/lang/Object"))
}

fn class_name(type_: &VarType) -> String {
    let value = type_.value_str();
    if let Some((_, kotlin)) = KOTLIN_NAMES.iter().find(|(java, _)| *java == value) {
        return kotlin.to_string();
    }
    ExprProcessor::get_cast_type_name(&VarType::new_value(CodeType::Object, 0, value))
}

fn argument_name(arg: &Option<VarType>) -> String {
    let Some(arg) = arg else {
        return "*".to_string();
    };
    if arg.is_generic() {
        match arg.as_generic().get_wildcard() {
            GenericType::WILDCARD_EXTENDS => return format!("out {}", type_name(arg)),
            GenericType::WILDCARD_SUPER => return format!("in {}", type_name(arg)),
            GenericType::WILDCARD_UNBOUND => return "*".to_string(),
            _ => {}
        }
    }
    let name = type_name(arg);
    if name.is_empty() {
        return "*".to_string();
    }
    name
}

fn star_projections(type_: &VarType) -> String {
    let count = DecompilerContext::get_struct_context()
        .get_class(type_.value_str())
        .and_then(|cl| cl.get_signature())
        .map_or(0, |sig| sig.fparameters().len());
    if count == 0 {
        return String::new();
    }
    format!("<{}>", vec!["*"; count].join(", "))
}

fn bound_name(arg: &Option<VarType>) -> Option<String> {
    let arg = arg.as_ref()?;
    if arg.is_generic() && arg.as_generic().get_wildcard() == GenericType::WILDCARD_UNBOUND {
        return None;
    }
    Some(type_name(arg)).filter(|name| !name.is_empty())
}

/// `(A, B) -> R` for `FunctionN<? super A, ? super B, ? extends R>`.
fn function_type_name(type_: &VarType) -> Option<String> {
    let generic = type_.as_generic();
    let arguments = generic.get_arguments();
    let (ret, params) = arguments.split_last()?;
    let params: Option<Vec<String>> = params.iter().map(bound_name).collect();
    Some(format!("({}) -> {}", params?.join(", "), bound_name(ret)?))
}

/// Whether the type `type_` has a type argument Vineflower does not know,
/// which it writes as the empty `<>` (written `*` here).
pub fn has_unknown_argument(type_: &VarType) -> bool {
    type_.type_ == CodeType::Object
        && type_.is_generic()
        && type_
            .as_generic()
            .get_arguments()
            .iter()
            .flatten()
            .any(|arg| {
                (arg.type_ != CodeType::Object && ExprProcessor::get_cast_type_name(arg).is_empty())
                    || has_unknown_argument(arg)
            })
}

/// The Kotlin name of the type `type_`: `ByteArray` for `byte[]`,
/// `List<out T>` for `List<? extends T>`, `Iterable<*>` for the raw
/// `Iterable`.
pub fn type_name(type_: &VarType) -> String {
    if type_.array_dim > 0 {
        let element = if type_.is_generic() {
            type_.as_generic().decrease_array_dim().0.clone()
        } else {
            type_.resize_array_dim(type_.array_dim - 1)
        };
        if element.array_dim == 0 && primitive(element.type_) {
            return format!("{}Array", type_name(&element));
        }
        return format!("Array<{}>", type_name(&element));
    }
    if primitive(type_.type_) {
        return KTypes::get_kotlin_type(Some(type_));
    }
    if KTypes::is_function_type(type_) && type_.is_generic() {
        if let Some(name) = function_type_name(type_) {
            return name;
        }
    }
    if type_.type_ != CodeType::Object {
        return ExprProcessor::get_cast_type_name(type_);
    }
    if type_.is_generic() {
        let generic = type_.as_generic();
        if generic.get_parent().is_some() {
            return KTypes::get_kotlin_type(Some(type_));
        }
        if !generic.get_arguments().is_empty() {
            let arguments: Vec<String> =
                generic.get_arguments().iter().map(argument_name).collect();
            return format!("{}<{}>", class_name(type_), arguments.join(", "));
        }
    }
    class_name(type_) + &star_projections(type_)
}

/// Whether the cast Vineflower writes to pick a Java overload, of a value of
/// type `from` to the parameter's type `to`, is left out: when the value's
/// own type is the parameter's, Kotlin picks the same overload without it,
/// and `as` would only add a null check.
pub fn overload_cast_redundant(to: &VarType, from: &VarType) -> bool {
    (to.array_dim > 0 || to.type_ == CodeType::Object)
        && from.type_ != CodeType::Null
        && from.equals(to)
        && to.is_generic() == from.is_generic()
        && (!to.is_generic() || to.as_generic().equals_exact(from))
        && active()
}

/// The type `operand` is declared with, where it is known for certain: a
/// field's, or a variable's from the local variable table.
pub fn declared_type(operand: Exprent) -> Option<VarType> {
    if let Some(field) = operand.try_as_field() {
        return Some(field.get_expr_type());
    }
    let var = operand.try_as_var()?;
    var.get_lvt()?;
    Some(var.get_definition_var_type())
}

/// Whether Java's cast of a value of type `from` to the reference type `to`
/// must let `null` through where Kotlin's `as T` would not: the operand is
/// the `null` literal or of a type variable's type, or the cast is the
/// upcast to `Object`.
pub fn nullable(to: &VarType, from: &VarType) -> bool {
    if to.array_dim != 0 || to.type_ != CodeType::Object {
        return false;
    }
    from.type_ == CodeType::Null
        || (from.type_ == CodeType::Genvar && from.array_dim == 0)
        || to.value_str() == "java/lang/Object"
}

thread_local! {
    static NULLABLE_PARAMETERS: RefCell<Vec<Vec<String>>> = const { RefCell::new(Vec::new()) };
}

struct Scope;

impl Drop for Scope {
    fn drop(&mut self) {
        NULLABLE_PARAMETERS.with(|p| p.borrow_mut().pop());
    }
}

/// Runs `f`, which writes the body of a function whose parameters `names`
/// are declared nullable.
pub fn with_nullable_parameters<R>(names: Vec<String>, f: impl FnOnce() -> R) -> R {
    NULLABLE_PARAMETERS.with(|p| p.borrow_mut().push(names));
    let _scope = Scope;
    f()
}

fn nullable_parameter(operand: Exprent) -> bool {
    let Some(var) = operand.try_as_var() else {
        return false;
    };
    let name = var.get_name();
    NULLABLE_PARAMETERS.with(|p| p.borrow().last().is_some_and(|names| names.contains(&name)))
}

fn non_null(operand: Exprent) -> bool {
    operand.try_as_new().is_some() || operand.try_as_const().is_some_and(|c| !c.is_null())
}

/// `operand` written with the cast to `to`, `as T?` when `nullable` says the
/// cast may see `null`, or the operand is a parameter declared nullable or a
/// local that may hold `null` there (fix `kt_nullable_cast`), and it is not a
/// literal or `new` object.
pub fn write(operand: Exprent, to: &VarType, nullable: bool, indent: i32) -> TextBuffer {
    let mut buf = operand.to_java(indent);
    if operand.get_precedence() > 1 {
        buf.enclose("(", ")");
    }
    let mut name = type_name(to);
    if (nullable || nullable_parameter(operand) || kuna_fix_kt_nullable_local::may_read_null(operand))
        && !non_null(operand)
    {
        if name.contains("->") {
            name = format!("({})", name);
        }
        name.push('?');
    }
    buf.append(" as ").append(&name);
    buf
}

/// The instance qualifier `res` of a call, cast to `to`: `(res as T)`.
pub fn qualifier(buf: &mut TextBuffer, to: &VarType, instance: Exprent, mut res: TextBuffer) {
    if instance.get_precedence() > 1 {
        res.enclose_with_parens();
    }
    buf.append("(")
        .append_buffer(&res)
        .append(" as ")
        .append(&type_name(to))
        .append(")");
}

/// Whether the fix writes the casts of the Kotlin class being written.
pub fn active() -> bool {
    kuna_fixes::enabled(KunaFix::KtJavaReferenceCast)
        && crate::kuna_fix_kt_java_cast_syntax::writing_kotlin()
}

/// `res`, the array operand of an element access or `length` whose type is
/// not an array, cast to `Array<Any>`.
pub fn object_array(res: &mut TextBuffer) {
    res.enclose("(", " as Array<Any>)");
}
