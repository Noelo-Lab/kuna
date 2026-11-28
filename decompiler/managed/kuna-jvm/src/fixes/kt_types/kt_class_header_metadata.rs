//! Fix `kt_class_header_metadata`: a class with metadata declares its type
//! parameters and supertypes as the metadata records them (variance,
//! nullability, Kotlin's collection names), not from its JVM `Signature`.

use std::rc::Rc;

use crate::code_constants::{
    ACC_ANNOTATION, OPC_ATHROW, OPC_DUP, OPC_INVOKESPECIAL, OPC_LDC, OPC_LDC_W, OPC_NEW,
};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::k_class::KClass;
use crate::k_element::KElement;
use crate::k_type::KType;
use crate::k_type_parameter::KTypeParameter;
use crate::fixes::kt_types::kt_type_parameter_id;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::kt_proto::{self, Projection, Variance};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

const FUNCTION: &str = "kotlin/Function";
const UNSUPPORTED: &str = "java/lang/UnsupportedOperationException";
const STUB_MESSAGE: &str = "Operation is not supported for read-only collection";

/// A class header's parts as the metadata declares them.
pub struct Header {
    /// `<out T, E : Enum<E>>`, empty without type parameters.
    pub type_parameters: String,
    /// ` where T : A, T : B`, empty without such constraints.
    pub where_clause: String,
    /// The supertypes other than `Any`: the JVM class each names and its text.
    pub supertypes: Vec<(Option<String>, TextBuffer)>,
}

/// The header of `cl` from its class metadata `kt_data`, `None` when the
/// fix does not apply.
pub fn header(cl: StructClass, kt_data: Option<&KElement>, indent: i32) -> Option<Header> {
    let cls = applicable(cl, kt_data)?;
    let resolver = cls.resolver.as_ref()?;
    let proto = &cls.proto;
    let _scope = kt_type_parameter_id::class_scope(cl, proto, resolver);
    if !all_resolved(proto, resolver) {
        return None;
    }
    let parameters: Vec<KTypeParameter> = proto
        .get_type_parameter_list()
        .iter()
        .map(|p| KTypeParameter::from(p, resolver))
        .collect();
    let (type_parameters, where_clause) = parameter_list(&parameters, indent);
    let mut supertypes = Vec::new();
    for supertype in proto.get_supertype_list() {
        let k_type = KType::from(&supertype, resolver);
        if VarType::VARTYPE_OBJECT().equals(&k_type) {
            continue;
        }
        let class = k_type.value.as_deref().map(str::to_string);
        supertypes.push((class, render(&k_type, indent)));
    }
    Some(Header {
        type_parameters,
        where_clause,
        supertypes,
    })
}

/// `cl`'s class metadata when the fix writes its header: a class (not an
/// annotation class) whose metadata has its supertypes and bounds inline.
fn applicable(cl: StructClass, kt_data: Option<&KElement>) -> Option<Rc<KClass>> {
    if cl.has_modifier(ACC_ANNOTATION) {
        return None;
    }
    let Some(KElement::Class(cls)) = kt_data else {
        return None;
    };
    cls.resolver.as_ref()?;
    let proto = &cls.proto;
    let tables = proto.get_supertype_count() == 0
        || !proto.message().scalars(2).is_empty()
        || proto
            .get_type_parameter_list()
            .iter()
            .any(|p| !p.message().scalars(6).is_empty());
    (!tables).then(|| cls.clone())
}

/// Whether the supertypes and bounds of `proto` name only type parameters
/// the open scopes declare, and `kotlin.Function` only at their outside.
fn all_resolved(proto: &kt_proto::Class, resolver: &MetadataNameResolver) -> bool {
    let bounds: Vec<kt_proto::Type> = proto
        .get_type_parameter_list()
        .iter()
        .flat_map(|p| p.get_upper_bound_list())
        .collect();
    proto
        .get_supertype_list()
        .iter()
        .chain(bounds.iter())
        .all(|t| {
            resolved(t)
                && t.get_argument_list()
                    .iter()
                    .all(|a| !names_function(a, resolver))
        })
}

/// Whether the type of `argument` is or holds `kotlin.Function`, the
/// supertype of function types, which `KType.stringify` takes for a
/// function type of no parameters (`() -> R` for `Function<R>`).
fn names_function(argument: &kt_proto::TypeArgument, resolver: &MetadataNameResolver) -> bool {
    if !argument.has_type() {
        return false;
    }
    let t = argument.get_type();
    (t.has_class_name() && resolver.resolve(t.get_class_name()) == FUNCTION)
        || t.get_argument_list()
            .iter()
            .any(|a| names_function(a, resolver))
}

/// `t` in Kotlin's syntax, as `KType.stringify` writes it, but for
/// `kotlin.Function` (see [`names_function`]), written by its name, and a
/// nested class that is no mapped type, written as the class header
/// Vineflower writes names it (`Outer.Inner`, not `pkg.Outer.Inner`).
fn render(t: &KType, indent: i32) -> TextBuffer {
    let k = t.k();
    let function_type = k.kotlin_type.starts_with("kotlin/Function") && k.kotlin_type != FUNCTION;
    if k.type_parameter_name.is_some() || function_type {
        return t.stringify(indent);
    }
    let class = k.kotlin_type.rsplit('/').next().unwrap_or_default();
    let jvm = t.value.as_deref().unwrap_or_default();
    let nested = class.contains('.') && k.kotlin_type.replace('.', "$") == jvm;
    let name = if nested {
        ExprProcessor::get_cast_type_name(&VarType::new_value(CodeType::Object, 0, jvm))
    } else {
        let dotted = k.kotlin_type.replace('/', ".");
        DecompilerContext::get_import_collector()
            .get_short_name(&dotted)
            .unwrap_or(dotted)
    };
    let mut buf = TextBuffer::new();
    buf.append(&name);
    if let Some(arguments) = k.type_arguments.as_ref().filter(|a| !a.is_empty()) {
        buf.append("<");
        for (i, argument) in arguments.iter().enumerate() {
            if i > 0 {
                buf.append(", ");
            }
            match argument.type_projection {
                Projection::Star => {
                    buf.append("*");
                    continue;
                }
                Projection::In => {
                    buf.append("in ");
                }
                Projection::Out => {
                    buf.append("out ");
                }
                Projection::Inv => {}
            }
            buf.append_buffer(&render(&argument.type_, indent + 1));
        }
        buf.append(">");
    }
    if k.is_nullable {
        buf.append("?");
    }
    buf
}

/// Whether the header of `cl` (an object expression when `anonymous`) is
/// written from its class metadata `kt_data`: Vineflower writes an object
/// expression's so, the fix a named class's.
fn metadata_header(cl: StructClass, kt_data: Option<&KElement>, anonymous: bool) -> bool {
    if anonymous {
        return matches!(kt_data, Some(KElement::Class(_)));
    }
    let Some(cls) = applicable(cl, kt_data) else {
        return false;
    };
    let Some(resolver) = cls.resolver.as_ref() else {
        return false;
    };
    let _scope = kt_type_parameter_id::class_scope(cl, &cls.proto, resolver);
    all_resolved(&cls.proto, resolver)
}

/// Whether `mt`, a method of `cl` (class metadata `kt_data`, an object
/// expression when `anonymous`) the metadata does not describe, is a stub
/// kotlinc adds to a class that implements a read-only collection interface
/// for a member only Java's interface has (`remove` of an `Iterator`): its
/// code throws `UnsupportedOperationException("Operation is not supported
/// for read-only collection")`. A header written from the metadata names
/// Kotlin's interface, which the stub overrides nothing of.
pub fn is_collection_stub(
    cl: StructClass,
    mt: StructMethod,
    kt_data: Option<&KElement>,
    anonymous: bool,
) -> bool {
    if mt.get_name().starts_with('<')
        || KtAttrs::get(mt, &KElement::KEY()).is_some()
        || !metadata_header(cl, kt_data, anonymous)
    {
        return false;
    }
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    if seq.length() != 5 {
        return false;
    }
    let pool = cl.get_pool();
    let op = |i: i32| seq.get_instr(i).opcode();
    let operand = |i: i32| seq.get_instr(i).operand(0);
    op(0) == OPC_NEW
        && op(1) == OPC_DUP
        && matches!(op(2), OPC_LDC | OPC_LDC_W)
        && op(3) == OPC_INVOKESPECIAL
        && op(4) == OPC_ATHROW
        && pool
            .get_primitive_constant(operand(0))
            .is_some_and(|c| &*c.get_string() == UNSUPPORTED)
        && pool
            .get_primitive_constant(operand(2))
            .is_some_and(|c| &*c.get_string() == STUB_MESSAGE)
        && pool.get_link_constant(operand(3)).is_some_and(|l| {
            l.classname.as_deref() == Some(UNSUPPORTED)
                && l.elementname == "<init>"
                && l.descriptor == "(Ljava/lang/String;)V"
        })
}

/// Whether every type parameter `t` refers to by id is in the open scopes
/// (a class's metadata copied into an inlined copy of an object names the
/// inline function's).
fn resolved(t: &kt_proto::Type) -> bool {
    if t.has_type_parameter() && kt_type_parameter_id::name(t.get_type_parameter()).is_none() {
        return false;
    }
    t.get_argument_list()
        .iter()
        .all(|a| !a.has_type() || resolved(&a.get_type()))
}

/// The type parameter list and the `where` clause of `parameters`.
fn parameter_list(parameters: &[KTypeParameter], indent: i32) -> (String, String) {
    if parameters.is_empty() {
        return (String::new(), String::new());
    }
    let mut list = String::from("<");
    let mut constraints: Vec<String> = Vec::new();
    for (i, parameter) in parameters.iter().enumerate() {
        if i > 0 {
            list.push_str(", ");
        }
        match parameter.variance {
            Variance::In => list.push_str("in "),
            Variance::Out => list.push_str("out "),
            Variance::Inv => {}
        }
        let name = KotlinWriter::to_valid_kotlin_identifier(&parameter.name);
        list.push_str(&name);
        match parameter.upper_bounds.as_slice() {
            [] => {}
            [bound] => {
                list.push_str(" : ");
                list.push_str(
                    &render(bound, indent + 1).convert_to_string_and_allow_data_discard(),
                );
            }
            bounds => {
                for bound in bounds {
                    constraints.push(format!(
                        "{name} : {}",
                        render(bound, indent + 1).convert_to_string_and_allow_data_discard()
                    ));
                }
            }
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
