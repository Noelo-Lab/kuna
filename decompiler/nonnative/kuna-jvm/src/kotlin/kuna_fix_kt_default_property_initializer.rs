//! kuna fix `kt_default_property_initializer`: a property whose field no
//! constructor stores is written with its type's default value.
//!
//! kotlinc does not store a property initializer that is the field's JVM
//! default (`var count: Int = 0`, `val closed = false`, `var next: Node? =
//! null`): the field starts at that value. The decompiler finds no store,
//! so `KProperty` writes `var count: Int`, which Kotlin rejects ("property
//! must be initialized"). With the fix a property backed by a field that no
//! constructor (`<init>`, or `<clinit>` of the class or its outer class for a
//! static field) of its class stores, and that has no other initializer,
//! is written `= 0`, `= 0L`, `= false`, `= null`, ...: the value the field
//! holds when construction ends, and what kotlinc compiles to the same
//! class file. A secondary constructor that delegates to another of its
//! class (`this(..)`) runs after that one has initialised the property, so
//! its stores do not count.

use crate::code_constants::{ACC_STATIC, OPC_INVOKESPECIAL, OPC_PUTFIELD, OPC_PUTSTATIC};
use crate::decompiler_context::DecompilerContext;
use crate::k_type::KType;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_method::StructMethod;

/// The default value of `type_` written as a Kotlin literal, for the types
/// that have one.
pub fn default_literal(type_: &KType) -> Option<&'static str> {
    if type_.k().is_nullable {
        return Some("null");
    }
    Some(match type_.k().kotlin_type.as_str() {
        "kotlin/Int" | "kotlin/Short" | "kotlin/Byte" => "0",
        "kotlin/Long" => "0L",
        "kotlin/Float" => "0.0F",
        "kotlin/Double" => "0.0",
        "kotlin/Boolean" => "false",
        "kotlin/Char" => "'\\u0000'",
        _ => return None,
    })
}

/// Whether no constructor of `class_struct` (or, for a static field, of the
/// classes that may hold it) stores `field`.
pub fn never_stored(class_struct: StructClass, field: StructField) -> bool {
    let is_static = field.has_modifier(ACC_STATIC);
    let mut owners = vec![class_struct];
    if is_static {
        let name = class_struct.qualified_name();
        if let Some(outer) = name
            .rfind('$')
            .and_then(|i| DecompilerContext::get_struct_context().get_class(&name[..i]))
        {
            owners.push(outer);
        }
    }
    let constructor = if is_static { "<clinit>" } else { "<init>" };
    owners.into_iter().all(|owner| {
        owner
            .get_methods()
            .as_slice()
            .iter()
            .filter(|mt| mt.get_name() == constructor)
            .all(|mt| !stores(owner, *mt, field, is_static))
    })
}

/// Whether `mt`'s code stores `field` (conservatively true when it cannot be read).
fn stores(cl: StructClass, mt: StructMethod, field: StructField, is_static: bool) -> bool {
    if !mt.contains_code() {
        return false;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return true;
    }
    let opcode = if is_static {
        OPC_PUTSTATIC
    } else {
        OPC_PUTFIELD
    };
    let found = mt.get_instruction_sequence().is_none_or(|seq| {
        let delegating = !is_static
            && (0..seq.length())
                .map(|i| seq.get_instr(i))
                .find(|instr| {
                    instr.opcode() == OPC_INVOKESPECIAL
                        && cl
                            .get_pool()
                            .get_link_constant(instr.operand(0))
                            .is_some_and(|link| link.elementname == "<init>")
                })
                .and_then(|instr| cl.get_pool().get_link_constant(instr.operand(0)))
                .is_some_and(|link| {
                    link.classname.as_deref() == Some(cl.qualified_name().as_str())
                });
        !delegating
            && (0..seq.length()).any(|i| {
                let instr = seq.get_instr(i);
                instr.opcode() == opcode
                    && cl
                        .get_pool()
                        .get_link_constant(instr.operand(0))
                        .is_none_or(|link| {
                            link.elementname == field.get_name()
                                && link.descriptor == field.get_descriptor()
                        })
            })
    });
    if !expanded {
        mt.release_resources();
    }
    found
}
