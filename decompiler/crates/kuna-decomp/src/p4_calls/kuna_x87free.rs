//! FFREE identifies a stack register without consuming its floating payload.
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub fn annotate(data: &mut Funcdata) {
    let Some(storage) = data.get_arch().x87_empty_storage.clone() else {
        return;
    };
    let index = storage.free;
    let bank = storage.bank;
    let tags = storage.tags;
    if bank.iter().any(|r| r.size != 10) {
        return;
    }
    let operands: Vec<_> = data
        .obank()
        .iter_code(OpCode::CPUI_CALLOTHER)
        .filter_map(|id| {
            let op = data.obank().get(id)?;
            if op.num_input() != 2 {
                return None;
            }
            if let Some(out) = op.get_out() {
                let out = data.vbank().get(out)?;
                if out.get_size() != tags.size as i32
                    || out.get_offset() != tags.offset
                    || !out
                        .get_addr()
                        .get_space()
                        .zip(tags.space.as_ref())
                        .is_some_and(|(a, b)| a.get_index() == b.get_index())
                {
                    return None;
                }
            }
            let selector = data.vbank().get(op.get_in(0)?)?;
            if !selector.is_constant() || selector.get_offset() != index {
                return None;
            }
            let operand = data.vbank().get(op.get_in(1)?)?;
            if operand.is_annotation() {
                return None;
            }
            bank.iter()
                .any(|reg| {
                    operand.get_size() == reg.size as i32
                        && operand.get_offset() == reg.offset
                        && operand
                            .get_addr()
                            .get_space()
                            .zip(reg.space.as_ref())
                            .is_some_and(|(a, b)| a.get_index() == b.get_index())
                })
                .then(|| (id, operand.get_addr().clone(), operand.get_size()))
        })
        .collect();
    for (op, address, size) in operands {
        let reference = data.new_varnode(size, &address, None);
        data.vbank_mut()
            .get_mut(reference)
            .unwrap()
            .set_annotation();
        data.op_set_input(op, reference, 1)
            .expect("existing FFREE operand");
    }
}
