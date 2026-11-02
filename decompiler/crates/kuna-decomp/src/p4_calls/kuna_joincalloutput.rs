//! Rebuild call flow when a late prototype needs missing joined-return storage.

use crate::fspec::{FuncCallSpecs, FuncProto};
use crate::funcdata::Funcdata;
use kuna_base::space::spacetype;

pub(crate) fn needs_reflow(data: &Funcdata, call: &FuncCallSpecs, proto: &FuncProto) -> bool {
    if !proto.is_output_locked() {
        return false;
    }
    let output = proto.get_output();
    let address = output.get_address();
    if !address
        .get_space()
        .is_some_and(|space| space.get_type() == spacetype::IPTR_JOIN)
    {
        return false;
    }
    !data
        .obank()
        .get(call.get_op())
        .and_then(|op| op.get_out())
        .and_then(|out| data.vbank().get(out))
        .is_some_and(|out| out.get_addr() == &address && out.get_size() == output.get_size())
}
