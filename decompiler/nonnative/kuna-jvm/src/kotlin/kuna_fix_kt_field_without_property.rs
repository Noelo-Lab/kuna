//! kuna fix `kt_field_without_property`: an instance field that has no
//! Kotlin property is written as a Kotlin property.
//!
//! `KotlinWriter` writes a field without a `KProperty` (every field of a
//! class with no Class metadata, such as the spill fields `label` and `L$0`
//! of the continuations kotlinc copies into `$$inlined$` classes and
//! object expressions) with the Java writer's declaration: `open int
//! label;`, which kotlinc rejects. With the fix it is a property declared
//! from the class file alone: `val` when the field is final, else `var`;
//! the Kotlin form of its (generic) type, nullable when it is a reference,
//! since nothing in the class file says it is not; its initializer when
//! `InitializerProcessor` extracted one, else for a `var` its JVM default
//! (`0`, `false`, `null`), which Kotlin requires and kotlinc does not store;
//! a `val` without one is assigned in the constructors. A Kotlin property
//! other than a private one has accessors, and a field does not, so
//! non-private fields are `@JvmField`. The visibility is the field's; a
//! package-private field, which Kotlin cannot declare, is `internal`, which
//! keeps it visible to the package's other classes. Static fields, which a
//! Kotlin class cannot declare, keep Vineflower's form.

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{
    ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC,
};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::interpreter_util::InterpreterUtil;
use crate::k_types::KTypes;
use crate::kotlin_options::KotlinOptions;
use crate::kotlin_writer::KotlinWriter;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// Whether `fd` of `cl` is written by [`write`]: an instance field of a
/// class, not an enum constant.
pub fn applies(cl: StructClass, fd: StructField, is_enum: bool) -> bool {
    !is_enum && !fd.has_modifier(ACC_STATIC) && !cl.has_modifier(ACC_INTERFACE)
}

/// Writes `fd` (named `name`) as a property declaration on its own line,
/// after its annotations: `@JvmField internal var label: Int = 0`.
pub fn write(
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
        if DecompilerContext::get_option(KotlinOptions::SHOW_PUBLIC_VISIBILITY) {
            "public "
        } else {
            ""
        }
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
