//! Fix family `kt_properties` (Kotlin output): property declarations and
//! accessors: an accessor's default flags, a setter's parameter name in its
//! body, the annotations a backing field's flags stand for, a `@JvmField`
//! `var` without `private set`, `super.x` for a supertype property, an instance
//! field without a property written as one, and an accessor's backing field
//! read as `field` (`kt_accessor_backing_field`).

pub(crate) mod kt_accessor_backing_field;

use std::cell::RefCell;

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{
    ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC, ACC_SYNTHETIC,
    ACC_TRANSIENT, ACC_VOLATILE,
};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::{InvocationExprent, InvocationType};
use crate::k_types::KTypes;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_flags::Flags;
use crate::method_wrapper::MethodWrapper;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

/// The flags of an accessor: `value` when the metadata has the field
/// (`present`), else the default for a property with `property_flags`.
pub fn accessor_flags(present: bool, value: i32, property_flags: i32) -> i32 {
    if present {
        return value;
    }
    match (
        Flags::VISIBILITY.get(property_flags),
        Flags::MODALITY.get(property_flags),
    ) {
        (Some(visibility), Some(modality)) => Flags::get_accessor_flags(
            Flags::HAS_ANNOTATIONS.get(property_flags),
            visibility,
            modality,
            false,
            false,
            false,
        ),
        _ => value,
    }
}

thread_local! {
    static SETTERS: RefCell<Vec<(VarProcessor, i32, String)>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the setter whose body is being written.
pub struct SetterScope;

impl Drop for SetterScope {
    fn drop(&mut self) {
        SETTERS.with(|s| {
            s.borrow_mut().pop();
        });
    }
}

/// The name the setter's parameter is written with, from the metadata's `name`.
pub fn setter_parameter_name(name: Option<&str>) -> String {
    match name {
        Some(name)
            if !name.is_empty()
                && !name.starts_with('<')
                && KotlinWriter::to_valid_kotlin_identifier(name) == name =>
        {
            name.to_string()
        }
        _ => "value".to_string(),
    }
}

/// Enters the body of the setter `setter`, whose parameter (the last one,
/// after an extension property's receiver) is written `name`.
pub fn enter_setter(setter: MethodWrapper, name: String) -> SetterScope {
    let (varproc, method) = setter.with(|d| (d.varproc, d.method_struct));
    let params = method.method_descriptor().params.clone();
    let before: i32 = params
        .iter()
        .take(params.len().saturating_sub(1))
        .map(|p| p.stack_size)
        .sum();
    let slot = before
        + if method.has_modifier(ACC_STATIC) {
            0
        } else {
            1
        };
    SETTERS.with(|s| s.borrow_mut().push((varproc, slot, name)));
    SetterScope
}

/// The name `var` is written with in the setter being written.
pub fn name_in_setter(var: VarExprent) -> Option<String> {
    let (varproc, slot, name) = SETTERS.with(|s| s.borrow().last().cloned())?;
    let processor = var.get_processor()?;
    if processor != varproc {
        return None;
    }
    let index = var.get_index();
    (processor.get_var_original_index(index).unwrap_or(index) == slot).then_some(name)
}

/// The annotations, in source form, that `field`'s flags stand for on its
/// property; `delegated` when `field` holds the property's delegate.
pub fn field_flag_annotations(field: StructField, delegated: bool) -> Vec<String> {
    let target = if delegated { "delegate:" } else { "" };
    [
        (ACC_VOLATILE, "Volatile"),
        (ACC_TRANSIENT, "Transient"),
        (ACC_SYNTHETIC, "JvmSynthetic"),
    ]
    .into_iter()
    .filter(|&(flag, _)| field.has_modifier(flag))
    .map(|(_, name)| format!("@{target}{name}"))
    .collect()
}

/// Whether a `var` without a setter method, with the backing field `field`
/// (`None` when it has none, or `delegated` when it holds the delegate),
/// has no setter at all rather than a private default one.
pub fn has_no_setter(field: Option<StructField>, delegated: bool) -> bool {
    !delegated && field.is_some_and(|f| !f.has_modifier(ACC_PRIVATE))
}

/// `super` (or `super<Iface>`) when `call` is a super call of an accessor.
pub fn super_qualifier(call: InvocationExprent) -> Option<String> {
    if call.get_invocation_type() != InvocationType::Special {
        return None;
    }
    let this = call.get_instance()?.try_as_var()?;
    let processor = this.get_processor().or_else(|| {
        DecompilerContext::get_current_method_wrapper().map(|m| m.with(|d| d.varproc))
    })?;
    let this_class = processor
        .get_this_vars()
        .borrow()
        .get(&VarVersionPair::from_var(this))
        .cloned()?;
    let classname = call.get_classname();
    if classname == this_class {
        return None;
    }
    let interface = DecompilerContext::get_struct_context()
        .get_class(&classname)
        .is_some_and(|cl| cl.has_modifier(ACC_INTERFACE));
    Some(if interface {
        let name = DecompilerContext::get_import_collector()
            .get_short_name_in_class_context(&ExprProcessor::build_java_class_name(&classname));
        format!("super<{name}>")
    } else {
        "super".to_string()
    })
}

/// Whether `fd` of `cl` is written by [`write`]: an instance field of a
/// class, not an enum constant.
pub fn writes_field_as_property(cl: StructClass, fd: StructField, is_enum: bool) -> bool {
    !is_enum && !fd.has_modifier(ACC_STATIC) && !cl.has_modifier(ACC_INTERFACE)
}

/// Writes `fd` (named `name`) as a property declaration on its own line,
/// after its annotations: `@JvmField internal var label: Int = 0`.
pub fn write_field_as_property(
    wrapper: ClassWrapper,
    cl: StructClass,
    fd: StructField,
    name: &str,
    buffer: &mut TextBuffer,
    indent: i32,
) {
    buffer.append_indent(indent);
    if !fd.has_modifier(ACC_PRIVATE) {
        buffer.append("@JvmField ");
    }
    buffer.append(visibility(fd.get_access_flags()));
    let is_final = fd.has_modifier(ACC_FINAL);
    buffer.append(if is_final { "val " } else { "var " });
    buffer.append(&KotlinWriter::to_valid_kotlin_identifier(name));
    buffer.append(": ");

    let field_type = VarType::new_signature_cl(&fd.get_descriptor(), false);
    let descriptor = fd.get_signature();
    if let Some(descriptor) = &descriptor {
        descriptor.verify_type(cl.get_signature().as_ref(), &field_type);
    }
    let declared = match &descriptor {
        None => field_type.clone(),
        Some(descriptor) => descriptor.type_(),
    };
    let nullable = !is_primitive(&declared);
    let mut type_name = KTypes::get_kotlin_type(Some(&declared));
    if nullable && KTypes::is_function_type(&declared) {
        type_name = format!("({type_name})");
    }
    buffer.append(&type_name);
    if nullable {
        buffer.append("?");
    }

    let key = InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor());
    let initializer = wrapper
        .get_dynamic_field_initializers()
        .borrow()
        .get_with_key(&key);
    if let Some(initializer) = initializer {
        buffer.append(" = ");
        if let Some(const_exprent) = initializer.try_as_const() {
            const_exprent.adjust_const_type(&field_type);
        }
        ExprProcessor::get_casted_exprent(initializer, &declared, buffer, indent, false);
    } else if !is_final {
        buffer.append(" = ").append(default_value(&declared));
    }
    buffer.append_line_separator();
}

pub fn visibility(access_flags: i32) -> &'static str {
    if access_flags & ACC_PUBLIC != 0 {
        "public "
    } else if access_flags & ACC_PROTECTED != 0 {
        "protected "
    } else if access_flags & ACC_PRIVATE != 0 {
        "private "
    } else {
        "internal "
    }
}

fn is_primitive(type_: &VarType) -> bool {
    type_.array_dim == 0
        && matches!(
            type_.type_,
            CodeType::Byte
                | CodeType::Char
                | CodeType::Double
                | CodeType::Float
                | CodeType::Int
                | CodeType::Long
                | CodeType::Short
                | CodeType::Boolean
        )
}

/// The Kotlin literal of the JVM default value of a field of `type_`.
pub fn default_value(type_: &VarType) -> &'static str {
    if type_.array_dim != 0 {
        return "null";
    }
    match type_.type_ {
        CodeType::Byte | CodeType::Short | CodeType::Int => "0",
        CodeType::Long => "0L",
        CodeType::Float => "0.0F",
        CodeType::Double => "0.0",
        CodeType::Char => "'\\u0000'",
        CodeType::Boolean => "false",
        _ => "null",
    }
}
