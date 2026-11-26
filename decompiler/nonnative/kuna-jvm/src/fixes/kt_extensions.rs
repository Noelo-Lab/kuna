//! Fix family `kt_extensions` (Kotlin output): an extension's receiver: the
//! receiver and type parameters an extension property declares, and `this` for
//! the receiver in an extension's body, kept on a stack of the bodies being written.

use std::cell::RefCell;

use crate::code_constants::{ACC_STATIC, OPC_ASTORE, OPC_ASTORE_3, OPC_IINC, OPC_ISTORE, OPC_ISTORE_0};
use crate::fixes::kt_suspend::kt_suspend_state_machine;
use crate::k_type::KType;
use crate::k_type_parameter::KTypeParameter;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::method_wrapper::MethodWrapper;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;

#[derive(Clone)]
struct Receiver {
    varproc: VarProcessor,
    slot: i32,
    dispatch: Option<String>,
}

thread_local! {
    static RECEIVERS: RefCell<Vec<Receiver>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the extension function whose body is being written.
pub struct ReceiverScope;

impl Drop for ReceiverScope {
    fn drop(&mut self) {
        RECEIVERS.with(|r| {
            r.borrow_mut().pop();
        });
    }
}

/// Enters the body of the extension function `method` of `cl`; `None` when
/// its class's own instance could not be named, or when the code stores to a
/// receiver's slot (a suspend function's state machine reloads it there).
pub fn enter(cl: StructClass, method: MethodWrapper, is_static: bool) -> Option<ReceiverScope> {
    let dispatch = if is_static {
        None
    } else {
        Some(format!("this@{}", class_label(cl)?))
    };
    let slot = if is_static { 0 } else { 1 };
    if stores_to(cl, method.with(|d| d.method_struct), slot) {
        return None;
    }
    let varproc = method.with(|d| d.varproc);
    RECEIVERS.with(|r| {
        r.borrow_mut().push(Receiver {
            varproc,
            slot,
            dispatch,
        })
    });
    Some(ReceiverScope)
}

/// The name `var` is written with in the extension function being written.
pub fn name(var: VarExprent) -> Option<String> {
    let receiver = RECEIVERS.with(|r| r.borrow().last().cloned())?;
    let processor = var.get_processor()?;
    if processor != receiver.varproc {
        return None;
    }
    let index = var.get_index();
    let slot = processor.get_var_original_index(index).unwrap_or(index);
    if slot == receiver.slot {
        Some("this".to_string())
    } else if slot == 0 {
        receiver.dispatch
    } else {
        None
    }
}

/// The label of `cl`'s `this`: its simple name, when it has one.
pub(crate) fn class_label(cl: StructClass) -> Option<String> {
    let name = cl.qualified_name();
    let simple = name.rsplit('/').next()?.rsplit('$').next()?.to_string();
    let first = simple.chars().next()?;
    (first.is_alphabetic() || first == '_')
        .then(|| KotlinWriter::to_valid_kotlin_identifier(&simple))
}

/// Whether `mt`'s code stores to a local slot up to `last` (true when it cannot be read).
fn stores_to(cl: StructClass, mt: StructMethod, last: i32) -> bool {
    if !mt.contains_code() {
        return false;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return true;
    }
    let resumed = kt_suspend_state_machine::resume_path(mt);
    let found = mt.get_instruction_sequence().is_none_or(|seq| {
        (0..seq.length()).any(|i| {
            if resumed.as_ref().is_some_and(|r| r.contains(&(i as usize))) {
                return false;
            }
            let instr = seq.get_instr(i);
            let op = instr.opcode();
            let slot = if (OPC_ISTORE..=OPC_ASTORE).contains(&op) || op == OPC_IINC {
                instr.operand(0)
            } else if (OPC_ISTORE_0..=OPC_ASTORE_3).contains(&op) {
                (op - OPC_ISTORE_0) % 4
            } else {
                return false;
            };
            slot <= last
        })
    });
    if !expanded {
        mt.release_resources();
    }
    found
}

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
pub fn enter_accessor(cl: StructClass, accessor: MethodWrapper) -> Option<ReceiverScope> {
    let is_static = accessor.with(|d| d.method_struct).has_modifier(ACC_STATIC);
    enter(cl, accessor, is_static)
}
