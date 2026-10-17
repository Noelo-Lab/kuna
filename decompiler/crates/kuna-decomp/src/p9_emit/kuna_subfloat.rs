//! Extracting a float-sized word from integer storage preserves the word's bits.
use crate::context::OpId;
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::kuna_bitcast::Transfer;
use kuna_num::opcodes::OpCode;

pub(crate) fn transfer(fd: &Funcdata, decl_high_type: bool, op: OpId) -> Option<Transfer> {
    let operation = fd.obank().get(op)?;
    if operation.code() != OpCode::CPUI_SUBPIECE { return None; }
    let input = operation.get_in(0)?;
    let source = fd.vbank().get(input)?;
    let output = operation.get_out()?;
    let result = fd.vn_type_def_facing(output);
    let offset = fd.vbank().get(operation.get_in(1)?)?;
    if !offset.is_constant() || offset.get_offset() != 0
        || source.get_size() != 8 || fd.vbank().get(output)?.get_size() != 4
        || result.get_metatype() != type_metatype::TYPE_FLOAT || result.get_size() != 4
        || !crate::kuna_bitcast::reinterprets_to_float(fd, decl_high_type, input)
    { return None; }
    let types = fd.get_arch().types()?;
    if types.get_size_of_int() != 4 || types.get_size_of_float() != 4 { return None; }
    Some(Transfer { input, target: result, size: 4, to_float: true })
}
