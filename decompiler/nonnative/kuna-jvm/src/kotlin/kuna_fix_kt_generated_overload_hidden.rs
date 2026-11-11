//! kuna fix `kt_generated_overload_hidden`: the overloads kotlinc generates
//! for an `@JvmOverloads` function are not written.
//!
//! For `@JvmOverloads fun f(a: Int, b: Int = 0)` kotlinc adds `f(a)`, one
//! method per trailing default. They are not in the Kotlin metadata; each
//! passes its parameters, placeholders, the mask and `null` to the
//! default-arguments bridge `f$default`. `KotlinWriter.writeMethod` wrote
//! them as functions, `@JvmOverloads fun f(a: Int): Int { return
//! f$default(this, a, 0, 2, null) }`, which the source does not have and Kotlin
//! does not accept (`` `f$default` `` names nothing, and the source's `f`
//! already declares these overloads). The constructor counterpart is
//! `kt_generated_constructor_hidden`. Such a method is recognised by its
//! `@JvmOverloads` annotation (kotlinc copies it from `f`, whose own copy has
//! metadata) and its code: loads of its parameters and constants, the null
//! checks of its parameters (`Intrinsics.checkNotNullParameter`), one
//! `invokestatic` of `f$default` (in this class, or in the facade of a
//! multifile part), and a return.

use crate::code_constants::{OPC_ARETURN, OPC_INVOKESTATIC, OPC_IRETURN, OPC_RETURN};
use crate::k_element::KElement;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

const JVM_OVERLOADS: &str = "kotlin/jvm/JvmOverloads";
const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const PARAMETER_CHECKS: [&str; 2] = ["checkNotNullParameter", "checkParameterIsNotNull"];

/// Whether `mt`, a member of `cl` without Kotlin metadata, is an overload
/// kotlinc generated for an `@JvmOverloads` function (see the module header).
pub fn is_generated(cl: StructClass, mt: StructMethod) -> bool {
    if mt.get_name().starts_with('<') || KtAttrs::get(mt, &KElement::KEY()).is_some() {
        return false;
    }
    let annotated = KotlinWriter::ANNOTATION_ATTRIBUTES.iter().any(|key| {
        mt.get_attribute(*key)
            .is_some_and(|a| a.get_annotations().iter().any(|an| an.get_class_name() == JVM_OVERLOADS))
    });
    if !annotated {
        return false;
    }
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    let pool = cl.get_pool();
    let bridge = format!("{}$default", mt.get_name());
    let mut calls = 0;
    let mut returned = false;
    for i in 0..seq.length() {
        let instr = seq.get_instr(i);
        let opcode = instr.opcode();
        if returned {
            return false;
        }
        if (1..=45).contains(&opcode) {
            continue;
        }
        if (OPC_IRETURN..=OPC_ARETURN).contains(&opcode) || opcode == OPC_RETURN {
            returned = true;
            continue;
        }
        if opcode != OPC_INVOKESTATIC || calls > 0 {
            return false;
        }
        let Some(link) = pool.get_link_constant(instr.operand(0)) else {
            return false;
        };
        if link.classname.as_deref() == Some(INTRINSICS) && PARAMETER_CHECKS.contains(&link.elementname.as_str()) {
            continue;
        }
        if link.elementname != bridge {
            return false;
        }
        calls += 1;
    }
    calls == 1 && returned
}
