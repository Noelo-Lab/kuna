//! kuna fix `kt_generated_constructor_hidden` (`kuna_fixes`): the
//! constructors kotlinc generates for a constructor's default arguments are
//! not written.
//!
//! For `class C(a: Int = 0)` kotlinc adds a no-argument constructor, and for
//! `@JvmOverloads constructor(..)` one constructor per trailing default.
//! Neither is in the Kotlin metadata; each only passes its parameters and
//! placeholders to the default-arguments constructor `<init>(.., int mask..,
//! DefaultConstructorMarker)`. `KotlinWriter.writeMethod` writes a
//! constructor without metadata as a function named after the class, `fun
//! C() { this(0, 1, null) }`, which the source does not have and Kotlin does
//! not accept. Such a constructor is recognised by its code: loads of its
//! parameters and constants, one `invokespecial` of this class's
//! default-arguments constructor, and `return`.

use crate::code_constants::{OPC_INVOKESPECIAL, OPC_RETURN};
use crate::k_element::KElement;
use crate::kt_attrs::KtAttrs;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

const MARKER_TAIL: &str = "ILkotlin/jvm/internal/DefaultConstructorMarker;)V";

/// Whether `mt`, a member of the Kotlin class `cl`, is a constructor kotlinc
/// generated for default arguments (see the module header).
pub fn is_generated(cl: StructClass, mt: StructMethod) -> bool {
    if mt.get_name() != "<init>"
        || KtAttrs::get(mt, &KElement::KEY()).is_some()
        || !matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::Class(_)))
    {
        return false;
    }
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    let pool = cl.get_pool();
    let name = cl.qualified_name();
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
        if opcode == OPC_RETURN {
            returned = true;
            continue;
        }
        if opcode != OPC_INVOKESPECIAL || calls > 0 {
            return false;
        }
        let Some(link) = pool.get_link_constant(instr.operand(0)) else {
            return false;
        };
        if link.classname.as_deref() != Some(name.as_str())
            || link.elementname != "<init>"
            || !link.descriptor.ends_with(MARKER_TAIL)
        {
            return false;
        }
        calls += 1;
    }
    calls == 1 && returned
}
