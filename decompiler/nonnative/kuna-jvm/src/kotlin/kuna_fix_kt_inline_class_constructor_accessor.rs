//! kuna fix `kt_inline_class_constructor_accessor` (`kuna_fixes`): a Kotlin
//! constructor with an inline-class parameter is read from the constructor
//! kotlinc compiles it to, not from the accessor its metadata names.
//!
//! kotlinc compiles `class UIntRange(start: UInt, endInclusive: UInt)` to a
//! private `<init>(II)V` holding the code and a public synthetic accessor
//! `<init>(IILkotlin/jvm/internal/DefaultConstructorMarker;)V` that only
//! passes its parameters on, which Kotlin callers call; the metadata's JVM
//! signature names the accessor. `KConstructor.parse` takes that accessor as
//! the constructor, so the header shows the accessor's `this(..)` call as the
//! supertype call (`: UIntRange(start, endInclusive)`) and the real
//! constructor, without metadata, is written as `fun UIntRange(..)`. The
//! accessor is recognised by its code: `aload 0`, a load of each parameter
//! but the marker in order, one `invokespecial` of this class's `<init>` with
//! the same parameters minus the marker, and `return`.

use crate::code_constants::{OPC_ALOAD, OPC_ILOAD, OPC_INVOKESPECIAL, OPC_RETURN};
use crate::rt::try_catch;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

const MARKER: &str = "Lkotlin/jvm/internal/DefaultConstructorMarker;";

/// The constructor `mt` only forwards to when it is such an accessor.
pub fn real_constructor(cl: StructClass, mt: StructMethod) -> Option<StructMethod> {
    let desc = mt.get_descriptor();
    let target_desc = desc
        .strip_suffix(&format!("{MARKER})V"))
        .map(|head| format!("{head})V"))?;
    if mt.get_name() != "<init>" || !mt.is_synthetic() {
        return None;
    }
    let target = cl.get_method("<init>", &target_desc)?;
    let descriptor = mt.method_descriptor();
    let params = &descriptor.params;
    let mut slots = vec![0];
    let mut slot = 1;
    for param in &params[..params.len() - 1] {
        slots.push(slot);
        slot += param.stack_size;
    }

    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return None;
    }
    let forwards = forwards_to(cl, mt, &slots, &target_desc);
    if !expanded {
        mt.release_resources();
    }
    forwards.then_some(target)
}

/// Whether `mt`'s code loads `slots` in order, calls this class's
/// `<init>target_desc` and returns.
fn forwards_to(cl: StructClass, mt: StructMethod, slots: &[i32], target_desc: &str) -> bool {
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    if seq.length() as usize != slots.len() + 2 {
        return false;
    }
    for (i, &slot) in slots.iter().enumerate() {
        let instr = seq.get_instr(i as i32);
        if !(OPC_ILOAD..=OPC_ALOAD).contains(&instr.opcode()) || instr.operand(0) != slot {
            return false;
        }
    }
    let call = seq.get_instr(slots.len() as i32);
    if call.opcode() != OPC_INVOKESPECIAL
        || seq.get_instr(slots.len() as i32 + 1).opcode() != OPC_RETURN
    {
        return false;
    }
    cl.get_pool()
        .get_link_constant(call.operand(0))
        .is_some_and(|link| {
            link.classname.as_deref() == Some(cl.qualified_name().as_str())
                && link.elementname == "<init>"
                && link.descriptor == target_desc
        })
}
