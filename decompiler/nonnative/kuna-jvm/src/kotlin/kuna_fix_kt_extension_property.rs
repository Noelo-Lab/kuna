//! kuna fix `kt_extension_property`: an extension property is declared with
//! its receiver and type parameters, and its accessors' bodies write the
//! receiver `this`.
//!
//! The metadata of a property holds its receiver type and type parameters
//! as a function's does, but `KProperty` never read them: `val
//! CharSequence.lastIndex: Int` was written `val lastIndex: Int`, another
//! declaration (a property without receiver, which callers' `s.lastIndex`
//! does not name), and its getter read the receiver parameter by its debug
//! name, `` `$this$lastIndex` ``. With the fix the header is `val <T : B>
//! Receiver.name: Type` (a type parameter with several bounds goes to a
//! `where` clause after the type) and each accessor body is written in the
//! receiver scope of `kt_extension_receiver` (`this`, and `this@Class` for
//! the class's own instance in a member extension).

use crate::k_type::KType;
use crate::k_type_parameter::KTypeParameter;
use crate::kotlin::kuna_fix_kt_extension_receiver::{self, ReceiverScope};
use crate::kotlin_writer::KotlinWriter;
use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::method_wrapper::MethodWrapper;
use crate::code_constants::ACC_STATIC;
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;

/// An extension property's receiver and type parameters.
#[derive(Clone)]
pub struct Extension {
    pub receiver: KType,
    pub type_parameters: Vec<KTypeParameter>,
}

/// The receiver of `property`, read in its type parameters' scope; `None` for a property without one.
pub fn parse(property: &kt_proto::Property, resolver: &MetadataNameResolver) -> Option<Extension> {
    if !property.has_receiver_type() {
        return None;
    }
    Some(Extension {
        receiver: KType::from(&property.get_receiver_type(), resolver),
        type_parameters: property
            .get_type_parameter_list()
            .iter()
            .map(|p| KTypeParameter::from(p, resolver))
            .collect(),
    })
}

/// Writes `<T : B> Receiver.` before the property's name.
pub fn write_receiver(buf: &mut TextBuffer, extension: &Extension, indent: i32) {
    let parameters = &extension.type_parameters;
    if !parameters.is_empty() {
        buf.append_char('<');
        for (i, parameter) in parameters.iter().enumerate() {
            if parameter.reified {
                buf.append("reified ");
            }
            buf.append(&KotlinWriter::to_valid_kotlin_identifier(&parameter.name));
            if parameter.upper_bounds.len() == 1 {
                buf.append(" : ")
                    .append_buffer(&parameter.upper_bounds[0].stringify(indent + 1));
            }
            if i + 1 < parameters.len() {
                buf.append(", ");
            }
        }
        buf.append("> ");
    }
    let receiver = &extension.receiver;
    let wrap = !receiver.k().is_nullable && receiver.k().kotlin_type.starts_with("kotlin/Function");
    if wrap {
        buf.append("(");
    }
    buf.append_buffer(&receiver.stringify(indent + 1));
    if wrap {
        buf.append(")");
    }
    buf.append(".");
}

/// Writes ` where T : A, T : B` after the property's type for type parameters with several bounds.
pub fn write_where(buf: &mut TextBuffer, extension: &Extension, indent: i32) {
    let mut first = true;
    for parameter in extension.type_parameters.iter().filter(|p| p.upper_bounds.len() > 1) {
        for bound in &parameter.upper_bounds {
            buf.append(if first { " where " } else { ", " });
            buf.append(&KotlinWriter::to_valid_kotlin_identifier(&parameter.name))
                .append(" : ")
                .append_buffer(&bound.stringify(indent + 1));
            first = false;
        }
    }
}

/// Enters the body of `accessor`, an accessor of an extension property of `cl`.
pub fn enter(cl: StructClass, accessor: MethodWrapper) -> Option<ReceiverScope> {
    let is_static = accessor.with(|d| d.method_struct).has_modifier(ACC_STATIC);
    kuna_fix_kt_extension_receiver::enter(cl, accessor, is_static)
}
