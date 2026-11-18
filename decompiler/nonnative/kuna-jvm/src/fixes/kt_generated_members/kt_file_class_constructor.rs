//! kuna fix `kt_file_class_constructor` (`kuna_fixes`): the constructor of a
//! Kotlin file class is not written.
//!
//! A Kotlin source file has no constructor, but kotlinc gives the parts of a
//! `@JvmMultifileClass` facade compiled with `-Xmultifile-parts-inherit` (as
//! kotlin-stdlib is) a public `<init>()V` so that each part, and finally the
//! facade, can extend the one before. The file's Kotlin metadata does not
//! name it, and `KotlinWriter.writeClass` writes a member without metadata
//! through `writeMethod`, which prints it among the file's top-level
//! declarations as `open fun StringsKt__StringsKt() {}`: not Kotlin, and not
//! in the source. Such a constructor is recognised by its code: `aload 0`,
//! one `invokespecial` of the superclass's `<init>()V`, and `return`.

use crate::code_constants::{OPC_ALOAD, OPC_INVOKESPECIAL, OPC_RETURN};
use crate::k_element::KElement;
use crate::kt_attrs::KtAttrs;
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

/// Whether `mt`, a member of `cl`, is the constructor of a Kotlin file class
/// that only calls its superclass's constructor (see the module header).
pub fn is_file_class_constructor(cl: StructClass, mt: StructMethod) -> bool {
    if mt.get_name() != "<init>"
        || mt.get_descriptor() != "()V"
        || KtAttrs::get(mt, &KElement::KEY()).is_some()
        || !matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::File(_)))
    {
        return false;
    }
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return false;
    }
    let only_super_call = calls_only_super(cl, mt);
    if !expanded {
        mt.release_resources();
    }
    only_super_call
}

fn calls_only_super(cl: StructClass, mt: StructMethod) -> bool {
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    let Some(super_class) = cl.super_class() else {
        return false;
    };
    if seq.length() != 3
        || seq.get_instr(0).opcode() != OPC_ALOAD
        || seq.get_instr(0).operand(0) != 0
        || seq.get_instr(1).opcode() != OPC_INVOKESPECIAL
        || seq.get_instr(2).opcode() != OPC_RETURN
    {
        return false;
    }
    cl.get_pool()
        .get_link_constant(seq.get_instr(1).operand(0))
        .is_some_and(|link| {
            link.classname.as_deref() == Some(&*super_class.get_string())
                && link.elementname == "<init>"
                && link.descriptor == "()V"
        })
}
