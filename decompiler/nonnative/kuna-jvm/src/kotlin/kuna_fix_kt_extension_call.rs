//! kuna fix `kt_extension_call`: an extension function or property is
//! called on its receiver, `s.shout()`, `s.firstChar`, `s.tag = 4`.
//!
//! kotlinc compiles an extension to a method whose first parameter is the
//! receiver. Vineflower writes a call of a top-level extension function as
//! Java does, the receiver as the first argument (`Util.closeQuietly(x)`),
//! a member extension the same way on its dispatch receiver
//! (`ByteString.Companion.encodeUtf8(s)`, which Kotlin resolves to another,
//! deprecated function), and the accessors of an extension property with
//! the property path that has no receiver: `firstChar` for `s.firstChar` and
//! `tag = s` for `s.tag = 4`, which read and assign another property (or
//! none) and lose the value assigned.
//!
//! With the fix, when the callee's metadata says it has a receiver, the call
//! is written on the receiver, by the callee's Kotlin name, the other
//! arguments as Vineflower writes them. The callee must be in scope there: a
//! top-level one by the import of its package's declaration of that name (no
//! other import of the file holds the name), a member of an object (`object`
//! or companion) as a member of a class written around the call or by the
//! import of the object's member (`import okio.ByteString.Companion.encodeUtf8`),
//! a member of another class only on `this`; and no member extension of that
//! name may be declared by a class around the call. It must also be what
//! Kotlin picks: the receiver is cast to the parameter's type (`(s as
//! CharSequence).trim()`) where Vineflower casts the argument to pick the
//! method, and where its own type would pick another extension of that name
//! the callee's class or package declares for a narrower receiver, or a
//! member the parameter's type does not have; the call keeps Vineflower's
//! form when a member of the type the receiver is seen as could take the
//! arguments (by name and count: Kotlin's members of a mapped type such as
//! `String`, the non-private methods, properties and fields of a loaded
//! class and its supertypes, private ones too inside that class; a type
//! that is not loaded counts as having one), and for a receiver of an inline
//! value class type and a callee whose name a value class mangled, whose
//! calls Kotlin resolves by other types than the JVM's. A property whose
//! call keeps its form is written as the accessor's call, not without its
//! receiver. An infix call Vineflower already writes on its receiver gets
//! the import of its top-level callee.

use std::rc::Rc;

use crate::code_constants::{ACC_PRIVATE, ACC_STATIC, ACC_SYNTHETIC, ACC_VARARGS};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::k_property::{KProperty, PropertyMethod};
use crate::k_type::KType;
use crate::kotlin::kuna_fix_kt_facade_call::{self as facade, java_property};
use crate::kotlin::kuna_fix_kt_jvm_name_call;
use crate::kotlin::kuna_fix_kt_unescaped_name;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::ClassKind;
use crate::kuna_fix_kt_explicit_type_arguments;
use crate::kuna_fix_kt_java_reference_cast;
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// The callee of an extension call, as the call reaches it.
enum Dispatch {
    /// A top-level extension of the package.
    TopLevel(String),
    /// A member of the enclosing class (`this`), or of an object around
    /// the call.
    InScope,
    /// A member of the object of this Kotlin name, which the file imports.
    Object(String),
}

/// How the dispatch receiver of `invocation`, a call of a member of
/// `owner`, is in scope where the call is written.
fn dispatch(invocation: InvocationExprent) -> Option<Dispatch> {
    let classname = invocation.get_classname();
    if invocation.is_static() {
        if let Some(package) = facade::top_level_package(&classname) {
            return Some(Dispatch::TopLevel(package));
        }
        return object_dispatch(&classname);
    }
    let instance = invocation.get_instance()?;
    if let Some(this) = facade::this_class(uncast(instance)) {
        let owns = this == classname
            || context_class(&this).is_some_and(|c| {
                facade::supertypes(c)
                    .iter()
                    .any(|s| s.qualified_name() == classname)
            });
        return owns.then_some(Dispatch::InScope);
    }
    let field = instance.try_as_field()?;
    if !field.is_static() || field.get_instance().is_some() {
        return None;
    }
    let object = field.get_classname();
    let field_type = field.get_descriptor().type_.clone();
    let reaches = field_type.value.as_deref() == Some(classname.as_str())
        && (field.get_name() == "INSTANCE" && object == classname
            || facade::companion(context_class(&object)?)
                .is_some_and(|c| c.qualified_name() == classname));
    if !reaches {
        return None;
    }
    object_dispatch(&classname)
}

/// The operand of a cast, else the expression.
fn uncast(expr: Exprent) -> Exprent {
    match expr.try_as_function() {
        Some(f) if f.get_func_type() == FunctionType::Cast => f.get_lst_operands().get(0),
        _ => expr,
    }
}

fn context_class(name: &str) -> Option<StructClass> {
    DecompilerContext::get_struct_context().get_class(name)
}

/// The dispatch of a member of the object `classname`.
fn object_dispatch(classname: &str) -> Option<Dispatch> {
    let cl = context_class(classname)?;
    KotlinChooser::parse_metadata_for(cl);
    let Some(KElement::Class(kclass)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    match Flags::CLASS_KIND.get(kclass.flags()) {
        Some(ClassKind::Object) | Some(ClassKind::CompanionObject) => {}
        _ => return None,
    }
    if facade::scope_classes().contains(&cl) {
        return Some(Dispatch::InScope);
    }
    Some(Dispatch::Object(classname.replace(['/', '$'], ".")))
}

/// Whether `name` of `dispatch` is what a call written on a receiver
/// resolves to where it is written, claiming its import.
fn reachable(dispatch: &Dispatch, name: &str) -> bool {
    if facade::scope_classes()
        .into_iter()
        .any(|cl| declares_member_extension(cl, name))
        && !matches!(dispatch, Dispatch::InScope)
    {
        return false;
    }
    match dispatch {
        Dispatch::TopLevel(package) | Dispatch::Object(package) => {
            facade::claim_name(package, name)
        }
        Dispatch::InScope => true,
    }
}

/// Whether `cl` or a supertype declares a member extension named `name`.
fn declares_member_extension(cl: StructClass, name: &str) -> bool {
    facade::supertypes(cl).into_iter().any(|c| {
        KotlinChooser::parse_metadata_for(c);
        if !matches!(KtAttrs::get(c, &KElement::KEY()), Some(KElement::Class(_))) {
            return false;
        }
        c.get_methods().to_vec().into_iter().any(|mt| {
            match KtAttrs::get(mt, &KElement::KEY()) {
                Some(KElement::Function(f)) => f.name == name && f.receiver_type.is_some(),
                Some(KElement::Property(p)) => {
                    let params = mt.method_descriptor().params.len();
                    p.name == name
                        && match KtAttrs::get(mt, &PropertyMethod::KEY()) {
                            Some(PropertyMethod::Getter) => params == 1,
                            Some(PropertyMethod::Setter) => params == 2,
                            None => false,
                        }
                }
                _ => false,
            }
        })
    })
}

/// Whether `type_` is an inline value class (`Result`, `UInt`, `@JvmInline`),
/// whose JVM type is another.
fn value_class(type_: &KType) -> bool {
    type_.k().is_unsigned_type || value_class_type(type_)
}

/// Whether the class of the JVM type `type_` is an inline value class, whose
/// functions Kotlin resolves by another type than the JVM's.
fn value_class_type(type_: &VarType) -> bool {
    if type_.array_dim > 0 || type_.type_ != CodeType::Object {
        return false;
    }
    if type_.value.as_deref() == Some("kotlin/Result") {
        return true;
    }
    let Some(cl) = type_.value.as_deref().and_then(context_class) else {
        return false;
    };
    KotlinChooser::parse_metadata_for(cl);
    matches!(KtAttrs::get(cl, &KElement::KEY()),
        Some(KElement::Class(k)) if Flags::IS_VALUE_CLASS.get(k.flags()))
}

/// The receiver of a call written on it: its value (an upcast the writer
/// may leave out unwrapped), and whether it is cast to the parameter's type
/// because its own type would select another overload (`narrower`) or a
/// member (`member`) the parameter's type does not have. `None` for a
/// `null` receiver, and where a member of the type it is seen as may win.
struct Receiver {
    value: Exprent,
    force: bool,
    ambiguous: bool,
}

impl Receiver {
    fn of(
        recv: Exprent,
        param: &VarType,
        ambiguous: bool,
        narrower: impl Fn(&VarType) -> bool,
        member: impl Fn(&VarType) -> bool,
    ) -> Option<Receiver> {
        if recv.try_as_const().is_some_and(|c| c.get_const_type().type_ == CodeType::Null) {
            return None;
        }
        let operand = upcast_operand(recv);
        let natural = operand.get_expr_type();
        let natural = if natural.type_ == CodeType::Genvar || natural.type_ == CodeType::Null {
            param.clone()
        } else {
            natural
        };
        if value_class_type(&natural) {
            return None;
        }
        let force = !ambiguous
            && !natural.equals(param)
            && jvm_subtype(&natural, param)
            && (narrower(&natural) || member(&natural) && !member(param));
        if member(param) || !force && !ambiguous && member(&natural) {
            return None;
        }
        Some(Receiver {
            value: if force { operand } else { recv },
            force,
            ambiguous,
        })
    }

    /// The receiver written as the qualifier of the call: cast to `param`
    /// (`T?` where the extension's receiver is `nullable`) where `force`
    /// says, else as Vineflower writes the argument (as the type `written`).
    fn write(
        &self,
        param: &VarType,
        written: &VarType,
        nullable: bool,
        parent_precedence: i32,
        indent: i32,
    ) -> TextBuffer {
        if self.force {
            let mut buf = self.value.to_java(indent);
            if self.value.get_precedence() > 0 {
                buf.enclose("(", ")");
            }
            buf.append(" as ").append(&kuna_fix_kt_java_reference_cast::type_name(param));
            if nullable {
                buf.append("?");
            }
            buf.enclose("(", ")");
            return buf;
        }
        let mut buf = TextBuffer::new();
        let cast = ExprProcessor::get_casted_exprent_full(
            self.value,
            written,
            &mut buf,
            indent,
            if self.ambiguous {
                NullCastType::Cast
            } else {
                NullCastType::DontCastAtAll
            },
            self.ambiguous,
            true,
            true,
        );
        let text = buf.convert_to_string_and_allow_data_discard();
        if cast || self.value.get_precedence() > parent_precedence || text.starts_with('-') {
            buf.enclose("(", ")");
        }
        buf
    }
}

/// The value an upcast (which the writer may leave out) casts, else `expr`.
fn upcast_operand(expr: Exprent) -> Exprent {
    let mut expr = expr;
    while let Some(f) = expr.try_as_function() {
        if f.get_func_type() != FunctionType::Cast {
            break;
        }
        let operand = f.get_lst_operands().get(0);
        if !jvm_subtype(&operand.get_expr_type(), &expr.get_expr_type()) {
            break;
        }
        expr = operand;
    }
    expr
}

/// Whether a value of the JVM type `a` is one of `b`.
fn jvm_subtype(a: &VarType, b: &VarType) -> bool {
    if a.equals(b) {
        return true;
    }
    if b.array_dim == 0 && b.value.as_deref() == Some("java/lang/Object") {
        return a.type_ == CodeType::Object || a.array_dim > 0;
    }
    if a.array_dim > 0 || b.array_dim > 0 || a.type_ != CodeType::Object || b.type_ != CodeType::Object
    {
        return false;
    }
    match (a.value.as_deref(), b.value.as_deref()) {
        (Some(x), Some(y)) => {
            let context = DecompilerContext::get_struct_context();
            context.get_class(x).is_some() && context.instance_of(x, y)
        }
        _ => false,
    }
}

/// Whether another extension of `name` that the call's import or scope
/// brings (in the callee's class, a multifile facade's parts, or the
/// callee's package of the input) takes a receiver of a type between
/// `seen` and the callee's `declared` one, which Kotlin would choose for a
/// receiver of the type `seen`.
fn narrower_overload(
    invocation: InvocationExprent,
    name: &str,
    seen: &VarType,
    declared: &VarType,
) -> bool {
    let classname = invocation.get_classname();
    let Some(cl) = context_class(&classname) else {
        return true;
    };
    KotlinChooser::parse_metadata_for(cl);
    let mut classes = vec![cl];
    if let Some(KElement::MultifileFacade(f)) = KtAttrs::get(cl, &KElement::KEY()) {
        let context = DecompilerContext::get_struct_context();
        classes.extend(f.files.iter().filter_map(|p| context.get_class(p)));
    }
    if let Some(package) = facade::top_level_package(&classname) {
        classes.extend(facade::package_facades(&package));
    } else {
        classes = facade::supertypes(cl);
    }
    classes.into_iter().any(|c| {
        KotlinChooser::parse_metadata_for(c);
        c.get_methods().to_vec().into_iter().any(|mt| {
            let Some(KElement::Function(f)) = KtAttrs::get(mt, &KElement::KEY()) else {
                return false;
            };
            if f.name != name || f.receiver_type.is_none() {
                return false;
            }
            let Some(r) = mt.method_descriptor().params.first().cloned() else {
                return false;
            };
            !r.equals(declared) && jvm_subtype(seen, &r) && jvm_subtype(&r, declared)
        })
    })
}

/// The written call of an extension function; `None` to leave it to
/// Vineflower.
pub fn write_function(
    invocation: InvocationExprent,
    function: &Rc<KFunction>,
    precedence: i32,
    indent: i32,
) -> Option<TextBuffer> {
    if function.receiver_type.is_none()
        || !function.context_receiver_types.is_empty()
        || Flags::IS_INFIX.get(function.flags)
    {
        return None;
    }
    let args = invocation.get_lst_parameters();
    let descriptor = invocation.get_descriptor();
    if args.is_empty()
        || descriptor.params.len() != args.len()
        || function
            .class_struct
            .is_none_or(kuna_fix_kt_jvm_name_call::synthetic_class)
        || facade::mangled(&invocation.get_name(), &function.name)
        || function.receiver_type.as_ref().is_some_and(value_class)
    {
        return None;
    }
    let dispatch = dispatch(invocation)?;
    let declared = &descriptor.params[0];
    let narrower = |seen: &VarType| narrower_overload(invocation, &function.name, seen, declared);
    let member = |seen: &VarType| member_may_win(seen, &function.name, Some(args.len() - 1));
    let ambiguous = invocation.kuna_ambiguous_argument(0);
    let recv = Receiver::of(args.get(0), declared, ambiguous, narrower, member)?;
    if !reachable(&dispatch, &function.name) {
        return None;
    }
    let mut buf = TextBuffer::new();
    let nullable = function
        .receiver_type
        .as_ref()
        .is_some_and(|r| r.k().is_nullable);
    let written = invocation.kuna_parameter_type(0);
    buf.append_buffer(&recv.write(declared, &written, nullable, precedence, indent))
        .append(".");
    buf.append_method_descriptor(
        &kuna_fix_kt_unescaped_name::member(&function.name),
        false,
        &invocation.get_classname(),
        &invocation.get_name(),
        &descriptor,
    );
    if kuna_fix_kt_explicit_type_arguments::active() {
        kuna_fix_kt_explicit_type_arguments::append(&mut buf, &invocation.get_generic_args());
    }
    buf.append("(");
    let skipped = invocation.d(|d| d.kuna_skip_arg);
    invocation.set_kuna_skip_arg(Some(0));
    let params = invocation.append_param_list(indent);
    invocation.set_kuna_skip_arg(skipped);
    buf.append_buffer(&params).append(")");
    buf.add_bytecode_mapping(invocation.get_bytecode().as_ref());
    Some(buf)
}

/// Whether the property accessor `invocation` calls (getter or setter as
/// `method`) takes an extension receiver.
pub fn is_extension_accessor(invocation: InvocationExprent, method: PropertyMethod) -> bool {
    let receiver_arity = if method == PropertyMethod::Setter { 2 } else { 1 };
    !invocation.get_name().contains('-')
        && invocation.get_descriptor().params.len() == receiver_arity
        && invocation.get_lst_parameters().len() == receiver_arity
}

/// The written read or assignment of an extension property through its
/// accessor; `None` to keep the accessor's call, which Vineflower would
/// write without the receiver.
pub fn write_property(
    invocation: InvocationExprent,
    property: &Rc<KProperty>,
    method: PropertyMethod,
    precedence: i32,
    indent: i32,
) -> Option<TextBuffer> {
    let args = invocation.get_lst_parameters();
    let descriptor = invocation.get_descriptor();
    if !is_extension_accessor(invocation, method) {
        return None;
    }
    let dispatch = dispatch(invocation)?;
    let declared = &descriptor.params[0];
    let member = |seen: &VarType| member_may_win(seen, &property.name, None);
    let recv = Receiver::of(args.get(0), declared, false, |_| false, member)?;
    if recv.force {
        return None;
    }
    if !reachable(&dispatch, &property.name) {
        return None;
    }
    let mut buf = TextBuffer::new();
    buf.add_bytecode_mapping(invocation.get_bytecode().as_ref());
    let written = invocation.kuna_parameter_type(0);
    buf.append_buffer(&recv.write(declared, &written, false, precedence, indent))
        .append(".");
    buf.append(&kuna_fix_kt_unescaped_name::member(&property.name));
    if method == PropertyMethod::Setter {
        buf.append(" = ");
        buf.append_buffer(&args.get(1).to_java(indent));
    }
    Some(buf)
}

/// Claims the import of a top-level infix function Vineflower writes on its
/// receiver.
pub fn import_infix(invocation: InvocationExprent, function: &KFunction) {
    if !invocation.is_static() || function.receiver_type.is_none() {
        return;
    }
    if let Some(package) = facade::top_level_package(&invocation.get_classname()) {
        facade::claim_name(&package, &function.name);
    }
}

/// Whether a member of a receiver of the type `seen` may be what Kotlin
/// calls for `name` with `arity` arguments (`None`: a property read), so
/// that an extension of that name is not.
fn member_may_win(seen: &VarType, name: &str, arity: Option<usize>) -> bool {
    if seen.array_dim > 0 {
        return builtin_has(ARRAY, name, arity);
    }
    if let Some(members) = primitive_members(seen.type_) {
        return builtin_has(members, name, arity) || builtin_has(ANY, name, arity);
    }
    if seen.type_ != CodeType::Object {
        return true;
    }
    let Some(class) = seen.value.as_deref() else {
        return true;
    };
    if let Some(members) = mapped_members(class) {
        return builtin_has(members, name, arity) || builtin_has(ANY, name, arity);
    }
    let Some(cl) = context_class(class) else {
        return true;
    };
    builtin_has(ANY, name, arity)
        || facade::supertypes(cl).into_iter().any(|c| {
            let qualified = c.qualified_name();
            if let Some(members) = mapped_members(&qualified) {
                return builtin_has(members, name, arity);
            }
            class_has(c, name, arity)
        })
}

/// Whether the class `c` itself declares a member Kotlin could call so
/// where the class is written (a private one only inside `c`).
fn class_has(c: StructClass, name: &str, arity: Option<usize>) -> bool {
    KotlinChooser::parse_metadata_for(c);
    let kotlin = matches!(KtAttrs::get(c, &KElement::KEY()), Some(KElement::Class(_)));
    let inside = enclosing(c);
    let methods = c.get_methods().to_vec();
    let method_hit = methods.into_iter().any(|mt| {
        if mt.has_modifier(ACC_STATIC)
            || mt.has_modifier(ACC_SYNTHETIC)
            || mt.has_modifier(ACC_PRIVATE) && !inside
        {
            return false;
        }
        match KtAttrs::get(mt, &KElement::KEY()) {
            Some(KElement::Function(f)) => arity.is_some_and(|n| {
                let vararg = f.parameters.iter().any(|p| p.vararg_type.is_some());
                let required = f
                    .parameters
                    .iter()
                    .filter(|p| {
                        p.vararg_type.is_none() && !Flags::DECLARES_DEFAULT_VALUE.get(p.flags)
                    })
                    .count();
                f.name == name
                    && f.receiver_type.is_none()
                    && required <= n
                    && (n <= f.parameters.len() || vararg)
            }),
            Some(KElement::Property(p)) => p.name == name,
            _ if kotlin => false,
            _ => match arity {
                Some(n) => {
                    let k = mt.method_descriptor().params.len();
                    mt.get_name() == name
                        && (k == n || mt.has_modifier(ACC_VARARGS) && k <= n + 1)
                }
                None => java_property(&mt.get_name()).as_deref() == Some(name),
            },
        }
    });
    method_hit
        || arity.is_none()
            && c.get_fields()
                .to_vec()
                .into_iter()
                .any(|fd| {
                    !fd.has_modifier(ACC_STATIC)
                        && (inside || !fd.has_modifier(ACC_PRIVATE))
                        && fd.get_name() == name
                })
}

/// Whether `c` is the class written or one around it.
fn enclosing(c: StructClass) -> bool {
    let mut node = DecompilerContext::get_current_class_node();
    while let Some(n) = node {
        if n.with(|d| d.class_struct) == c {
            return true;
        }
        node = n.with(|d| d.parent);
    }
    false
}

/// Whether a built-in member list has `name` (`name/arity` for a
/// function, `name` for a property).
fn builtin_has(members: &[&str], name: &str, arity: Option<usize>) -> bool {
    members.iter().any(|m| match (m.split_once('/'), arity) {
        (Some((n, k)), Some(a)) => n == name && k.parse::<usize>().is_ok_and(|k| k == a),
        (None, None) => *m == name,
        _ => false,
    })
}

const ANY: &[&str] = &["equals/1", "hashCode/0", "toString/0"];
const ARRAY: &[&str] = &[
    "size", "get/1", "set/2", "iterator/0", "clone/0", "equals/1", "hashCode/0", "toString/0",
];
const NUMBER: &[&str] = &[
    "toByte/0", "toShort/0", "toInt/0", "toLong/0", "toFloat/0", "toDouble/0", "toChar/0",
];
const INTEGRAL: &[&str] = &[
    "plus/1", "minus/1", "times/1", "div/1", "rem/1", "inc/0", "dec/0", "unaryPlus/0",
    "unaryMinus/0", "rangeTo/1", "rangeUntil/1", "compareTo/1", "toByte/0", "toShort/0",
    "toInt/0", "toLong/0", "toFloat/0", "toDouble/0", "toChar/0", "shl/1", "shr/1", "ushr/1",
    "and/1", "or/1", "xor/1", "inv/0",
];
const NARROW: &[&str] = &[
    "plus/1", "minus/1", "times/1", "div/1", "rem/1", "inc/0", "dec/0", "unaryPlus/0",
    "unaryMinus/0", "rangeTo/1", "rangeUntil/1", "compareTo/1", "toByte/0", "toShort/0",
    "toInt/0", "toLong/0", "toFloat/0", "toDouble/0", "toChar/0",
];
const CHAR: &[&str] = &[
    "plus/1", "minus/1", "inc/0", "dec/0", "rangeTo/1", "rangeUntil/1", "compareTo/1",
    "toByte/0", "toShort/0", "toInt/0", "toLong/0", "toFloat/0", "toDouble/0", "toChar/0",
    "code",
];
const BOOLEAN: &[&str] = &["not/0", "and/1", "or/1", "xor/1", "compareTo/1"];
const STRING: &[&str] = &[
    "length", "get/1", "subSequence/2", "compareTo/1", "plus/1", "chars/0", "codePoints/0",
];
const CHAR_SEQUENCE: &[&str] = &["length", "get/1", "subSequence/2", "chars/0", "codePoints/0"];
const ENUM: &[&str] = &["name", "ordinal", "compareTo/1", "declaringClass"];
const THROWABLE: &[&str] = &[
    "message", "cause", "stackTrace", "localizedMessage", "suppressed", "setStackTrace/1",
    "fillInStackTrace/0", "getLocalizedMessage/0", "printStackTrace/0", "printStackTrace/1",
    "getStackTrace/0", "initCause/1", "getSuppressed/0", "addSuppressed/1",
];
const ITERATOR: &[&str] = &[
    "hasNext/0", "next/0", "remove/0", "forEachRemaining/1", "hasPrevious/0", "previous/0",
    "nextIndex/0", "previousIndex/0", "set/1", "add/1",
];
const ITERABLE: &[&str] = &["iterator/0", "forEach/1", "spliterator/0"];
const COLLECTION: &[&str] = &[
    "iterator/0", "forEach/1", "spliterator/0", "size", "isEmpty/0", "contains/1",
    "containsAll/1", "add/1", "remove/1", "addAll/1", "removeAll/1", "retainAll/1", "clear/0",
    "stream/0", "parallelStream/0", "removeIf/1", "toArray/0", "toArray/1",
];
const LIST: &[&str] = &[
    "iterator/0", "forEach/1", "spliterator/0", "size", "isEmpty/0", "contains/1",
    "containsAll/1", "add/1", "remove/1", "addAll/1", "removeAll/1", "retainAll/1", "clear/0",
    "stream/0", "parallelStream/0", "removeIf/1", "toArray/0", "toArray/1", "get/1",
    "indexOf/1", "lastIndexOf/1", "listIterator/0", "listIterator/1", "subList/2", "set/2",
    "add/2", "addAll/2", "removeAt/1", "replaceAll/1", "sort/1", "getFirst/0", "getLast/0",
    "removeFirst/0", "removeLast/0", "addFirst/1", "addLast/1", "reversed/0",
];
const MAP: &[&str] = &[
    "size", "isEmpty/0", "containsKey/1", "containsValue/1", "get/1", "keys", "values",
    "entries", "put/2", "remove/1", "remove/2", "putAll/1", "clear/0", "getOrDefault/2",
    "forEach/1", "replaceAll/1", "merge/3", "computeIfPresent/2", "putIfAbsent/2", "replace/2",
    "replace/3", "computeIfAbsent/2", "compute/2",
];
const MAP_ENTRY: &[&str] = &["key", "value", "setValue/1"];

/// Kotlin's members of a primitive type.
fn primitive_members(type_: CodeType) -> Option<&'static [&'static str]> {
    Some(match type_ {
        CodeType::Int | CodeType::Long => INTEGRAL,
        CodeType::Byte | CodeType::Short | CodeType::Float | CodeType::Double => NARROW,
        CodeType::Char => CHAR,
        CodeType::Boolean => BOOLEAN,
        _ => return None,
    })
}

/// Kotlin's members of a JDK class Kotlin maps to a built-in type.
fn mapped_members(class: &str) -> Option<&'static [&'static str]> {
    Some(match class {
        "java/lang/Object" | "java/lang/Cloneable" | "java/lang/annotation/Annotation" => ANY,
        "java/lang/String" => STRING,
        "java/lang/CharSequence" => CHAR_SEQUENCE,
        "java/lang/Number" => NUMBER,
        "java/lang/Comparable" => &["compareTo/1"],
        "java/lang/Enum" => ENUM,
        "java/lang/Throwable" => THROWABLE,
        "java/lang/Integer" | "java/lang/Long" => INTEGRAL,
        "java/lang/Byte" | "java/lang/Short" | "java/lang/Float" | "java/lang/Double" => NARROW,
        "java/lang/Character" => CHAR,
        "java/lang/Boolean" => BOOLEAN,
        "java/lang/Iterable" => ITERABLE,
        "java/util/Iterator" | "java/util/ListIterator" => ITERATOR,
        "java/util/Collection" | "java/util/Set" => COLLECTION,
        "java/util/List" => LIST,
        "java/util/Map" => MAP,
        "java/util/Map$Entry" => MAP_ENTRY,
        _ => return None,
    })
}
