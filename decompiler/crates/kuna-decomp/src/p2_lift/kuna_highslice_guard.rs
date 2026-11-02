//! Match a high-byte slice with its equivalent zero-extended logical shift.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) fn matches(fd: &Funcdata, first: OpId, second: OpId) -> bool {
    proof(fd, first, second).is_some()
}

fn proof(fd: &Funcdata, first: OpId, second: OpId) -> Option<()> {
    let first = fd.obank().get(first)?;
    let second = fd.obank().get(second)?;
    let (shift, slice) = match (first.code(), second.code()) {
        (OpCode::CPUI_INT_RIGHT, OpCode::CPUI_SUBPIECE) => (first, second),
        (OpCode::CPUI_SUBPIECE, OpCode::CPUI_INT_RIGHT) => (second, first),
        _ => return None,
    };
    if shift.num_input() != 2 || slice.num_input() != 2 {
        return None;
    }
    let input = shift.get_in(0)?;
    if slice.get_in(0)? != input {
        return None;
    }
    let size = fd.vbank().get(input)?.get_size();
    if !(1..=8).contains(&size) || fd.vbank().get(shift.get_out()?)?.get_size() != size {
        return None;
    }
    let offset = fd.vbank().get(slice.get_in(1)?)?;
    let amount = fd.vbank().get(shift.get_in(1)?)?;
    let output_size = fd.vbank().get(slice.get_out()?)?.get_size();
    if !offset.is_constant()
        || !amount.is_constant()
        || output_size <= 0
        || offset.get_offset() == 0
        || offset.get_offset() >= size as u64
        || offset.get_offset() + output_size as u64 != size as u64
        || amount.get_offset() != offset.get_offset() * 8
    {
        return None;
    }
    Some(())
}
