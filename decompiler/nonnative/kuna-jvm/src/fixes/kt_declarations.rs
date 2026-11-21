//! Fix family `kt_declarations` (Kotlin output): what a function header needs
//! that the plugin leaves out: Kotlin names in `@Throws`, the import and opt-in
//! of a `contract`, and the `override` a bridge method shows.

use crate::code_constants::{
    ACC_BRIDGE, OPC_INVOKEINTERFACE, OPC_INVOKESPECIAL, OPC_INVOKEVIRTUAL,
};
use crate::decompiler_context::DecompilerContext;
use crate::fixes::kt_calls::kt_facade_call;
use crate::k_types::KTypes;
use crate::kotlin_writer::KotlinWriter;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// The Kotlin name of the exception class with internal name `name`.
pub fn kotlin_name(name: &str) -> String {
    KTypes::get_kotlin_type(Some(&VarType::new_signature_cl(name, true)))
}

/// How `contract` is written: bare when imported, else qualified.
pub fn contract_call() -> String {
    let prefix = kt_facade_call::qualifier("kotlin.contracts", "contract")
        .unwrap_or_default();
    format!("{prefix}contract")
}

/// Appends the line `@OptIn(ExperimentalContracts::class)`.
pub fn append_opt_in(buf: &mut TextBuffer, indent: i32) {
    let name = DecompilerContext::get_import_collector()
        .get_short_name("kotlin.contracts.ExperimentalContracts")
        .unwrap_or_else(|| "kotlin.contracts.ExperimentalContracts".to_string());
    buf.append_indent(indent)
        .append(&format!("@OptIn({name}::class)"))
        .append_line_separator();
}

/// Whether a bridge method of `cl` calls `method`.
pub fn bridged(cl: StructClass, method: StructMethod) -> bool {
    let methods = cl.get_methods();
    let mut bridges = methods
        .as_slice()
        .iter()
        .filter(|bridge| bridge.has_modifier(ACC_BRIDGE))
        .peekable();
    if bridges.peek().is_none() {
        return false;
    }
    let name = method.get_name();
    let descriptor = method.get_descriptor();
    bridges.any(|bridge| {
        bridge.get_name() == name
            && bridge.get_descriptor() != descriptor
            && calls(cl, *bridge, &name, &descriptor)
    })
}

/// Whether the getter `getter` of a property of `cl` overrides a supertype member.
pub fn property_overrides(cl: StructClass, getter: StructMethod) -> bool {
    KotlinWriter::search_for_method(
        Some(cl),
        &getter.get_name(),
        &getter.method_descriptor(),
        false,
    ) || bridged(cl, getter)
}

fn calls(cl: StructClass, mt: StructMethod, name: &str, descriptor: &str) -> bool {
    if !mt.contains_code() {
        return false;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return false;
    }
    let found = mt.get_instruction_sequence().is_some_and(|seq| {
        (0..seq.length()).any(|i| {
            let instr = seq.get_instr(i);
            matches!(
                instr.opcode(),
                OPC_INVOKEVIRTUAL | OPC_INVOKEINTERFACE | OPC_INVOKESPECIAL
            ) && cl
                .get_pool()
                .get_link_constant(instr.operand(0))
                .is_some_and(|link| {
                    link.classname.as_deref() == Some(cl.qualified_name().as_str())
                        && link.elementname == name
                        && link.descriptor == descriptor
                })
        })
    });
    if !expanded {
        mt.release_resources();
    }
    found
}
