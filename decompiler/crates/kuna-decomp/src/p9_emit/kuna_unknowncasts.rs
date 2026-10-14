//! Unknown call argument storage does not declare an integer C contract.

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
            proto.has_store()
                && proto
                    .get_param(slot - 1)
                    .is_some_and(|param| param.is_type_locked())
        }
        Some(OpCode::CPUI_CALLOTHER) if slot > 0 => false,
        _ => true,
    }
}
