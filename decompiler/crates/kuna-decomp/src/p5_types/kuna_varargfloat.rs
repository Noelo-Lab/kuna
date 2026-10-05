//! A value a variadic call passes in a floating-point register is a `double`.
//! The variadic part of a declared prototype has no parameter to type the
//! argument, so a constant moved straight into `d0` on AArch64 (`fmov d0,#0.5`)
//! printed as the integer `0x3fe0000000000000`, and compiled back as one, in an
//! integer register. The register the model's storage assignment chose for the
//! argument already says what it is: [`argument_vote`] turns a floating-point
//! entry into a `double` vote on the argument, the way a locked parameter's type
//! is one.
use crate::{
    context::{OpId, VarnodeId},
    dtype::{type_class, type_metatype, Datatype},
    fspec::FuncCallSpecs,
    funcdata::Funcdata,
};
use std::rc::Rc;

/// The `double` vote on argument `slot` of `op`, when it lies past the declared
/// parameters of the variadic call `fc` and its storage is a floating-point
/// entry of the model.  A constant whose bits a float literal cannot spell (a
/// NaN payload) gets none.
pub fn argument_vote(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    op: OpId,
    slot: i32,
) -> Option<Rc<Datatype>> {
    if !fc.is_dotdotdot() || !fc.proto().has_model() || fc.proto().get_param(slot - 1).is_some() {
        return None;
    }
    let (addr, size) = fc.final_input_storage().get((slot - 1) as usize)?;
    if *size != 8 {
        return None;
    }
    let float = fc.proto().model().input().get_entry().iter().any(|e| {
        e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, *size) == 0
    });
    let vn = data.obank().get(op)?.get_in(slot)?;
    if !float || data.vbank().get(vn)?.get_size() != 8 || !spells_as_a_float(data, vn) {
        return None;
    }
    data.get_arch()
        .types()?
        .get_base(8, type_metatype::TYPE_FLOAT)
        .ok()
}

/// The type C requires of that argument: `double`, when [`argument_vote`] gives
/// one and the value is an integer. No integer travels in a floating-point
/// register, so its bits got there by a reinterpretation, and the cast this
/// requirement adds prints as one. A struct or a pointer there gets none.
pub fn argument_requirement(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    op: OpId,
    slot: i32,
) -> Option<Rc<Datatype>> {
    let ct = argument_vote(data, fc, op, slot)?;
    let vn = data.obank().get(op)?.get_in(slot)?;
    crate::kuna_bitcast::reinterprets_to_float(data, data.get_arch().decl_high_type, vn).then_some(ct)
}

fn spells_as_a_float(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(node) = data.vbank().get(vn).filter(|n| n.is_constant()) else {
        return true;
    };
    data.get_arch()
        .get_float_format(node.get_size())
        .is_some_and(|f| f.get_host_float(node.get_offset()).1 != kuna_num::float::floatclass::nan)
}
