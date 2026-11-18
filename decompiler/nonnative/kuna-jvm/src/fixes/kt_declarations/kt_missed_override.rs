//! kuna fix `kt_missed_override`: a function or property that overrides a
//! supertype member is written `override` when its class file shows it.
//!
//! Kotlin metadata does not record `override`; the plugin decides it by
//! looking for a supertype method with exactly the same descriptor
//! (`KotlinWriter::search_for_method`), and never for properties. An
//! override that narrows the return type (`fun buffer(): Buffer` for
//! `BufferedSink.buffer(): BufferedSink`) or whose parameter is a type
//! parameter of the supertype (`compareTo(other: ByteString)` for
//! `Comparable<ByteString>`) has another descriptor, and an overriding
//! property (`val connection: RealConnection` of an interface) is written
//! `open val`: Kotlin rejects both ("hides member of supertype and needs an
//! 'override' modifier"). kotlinc compiles such an override with a bridge
//! method in the same class, of the supertype's descriptor, that calls it.
//! With the fix a function is an override when its class has a bridge method
//! that calls it, and a property when its getter is such a function or has
//! a supertype method of the same descriptor.

use crate::code_constants::{
    ACC_BRIDGE, OPC_INVOKEINTERFACE, OPC_INVOKESPECIAL, OPC_INVOKEVIRTUAL,
};
use crate::kotlin_writer::KotlinWriter;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

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
