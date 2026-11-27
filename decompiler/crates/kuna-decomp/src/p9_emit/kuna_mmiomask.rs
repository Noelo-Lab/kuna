//! Name a literal only when it masks the unchanged value of a known volatile register.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::userop::BUILTIN_VOLATILE_READ;
use kuna_num::opcodes::OpCode;

pub(super) fn operand<'a>(
    fd: &Funcdata,
    arch: &'a Architecture,
    op: OpId,
) -> Option<(VarnodeId, &'a str)> {
    let op = fd.obank().get(op)?;
    if op.code() != OpCode::CPUI_INT_AND {
        return None;
    }
    let (mut value, mut constant) = (op.get_in(0)?, op.get_in(1)?);
    if fd.vbank().get(value)?.is_constant() {
        std::mem::swap(&mut value, &mut constant);
    }
    let mask = fd.vbank().get(constant)?;
    if !mask.is_constant() || fd.vn_high_display_format(constant) != 0 {
        return None;
    }
    let size = mask.get_size();
    for _ in 0..16 {
        let v = fd.vbank().get(value)?;
        if v.get_size() != size {
            return None;
        }
        let def = fd.obank().get(v.get_def()?)?;
        match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => value = def.get_in(0)?,
            OpCode::CPUI_CALLOTHER => {
                let id = fd.vbank().get(def.get_in(0)?)?;
                if !id.is_constant() || id.get_offset() != u64::from(BUILTIN_VOLATILE_READ) {
                    return None;
                }
                let location = fd.vbank().get(def.get_in(1)?)?;
                let addr = location.get_addr();
                let data = arch.manage().get_default_data_space()?;
                if addr.get_space()?.get_index() != data.get_index() {
                    return None;
                }
                let name =
                    arch.mmio_field_masks
                        .get(&(addr.get_offset(), size, mask.get_offset()))?;
                return Some((constant, name));
            }
            _ => return None,
        }
    }
    None
}
