//! Unknown call argument storage does not declare an integer C contract, and
//! neither does an untyped word a format-call override places before the format
//! (`kuna_fmtcallee`).

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) fn declared_argument(fd: &Funcdata, op: OpId, slot: i32) -> bool {
    match fd.obank().get(op).map(|op| op.code()) {
        Some(OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) if slot > 0 => {
            let Some(index) = fd.get_call_specs_index(op) else {
                return false;
            };
            let proto = fd.get_call_specs(index).proto();
            let format_call = fd.obank().get(op).is_some_and(|o| fd.get_override().is_format_call(o.get_addr()));
            proto.has_store()
                && proto.get_param(slot - 1).is_some_and(|param| {
                    param.is_type_locked()
                        && !(format_call
                            && param.get_type().is_some_and(|t| t.get_metatype() == crate::dtype::type_metatype::TYPE_UNKNOWN))
                })
        }
        Some(OpCode::CPUI_CALLOTHER) if slot > 0 => false,
        _ => true,
    }
}
