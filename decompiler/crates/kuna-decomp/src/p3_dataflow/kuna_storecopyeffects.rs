//! Resolve a STORE's existing INDIRECT effects when it becomes a direct COPY.
//! Waiting for the marker's next rule visit lets dead-code removal delete the
//! newly written value before those effects read it.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

pub(crate) fn collapse(data: &mut Funcdata, copy: OpId) {
    let mut effects = Vec::new();
    let mut prev = data.op_previous_op(copy);
    while let Some(op) = prev {
        let marker = data.obank().get(op).expect("storecopyeffects: marker");
        if marker.code() != OpCode::CPUI_INDIRECT {
            break;
        }
        prev = data.op_previous_op(op);
        let Some(iop) = marker.get_in(1).and_then(|id| data.vbank().get(id)) else {
            continue;
        };
        if iop.get_space().get_type() == spacetype::IPTR_IOP
            && crate::funcdata_varnode::op_iop_decode(iop.get_offset()) == copy
        {
            effects.push(op);
        }
    }
    for effect in effects {
        resolve(data, copy, effect);
    }
}

fn resolve(data: &mut Funcdata, copy: OpId, effect: OpId) {
    let written = data
        .obank()
        .get(copy)
        .expect("storecopyeffects: copy")
        .get_out()
        .unwrap();
    let marker = data.obank().get(effect).expect("storecopyeffects: effect");
    let out = marker.get_out().unwrap();
    let old = marker.get_in(0).unwrap();
    let w = data.vbank().get(written).unwrap();
    let v = data.vbank().get(out).unwrap();
    let overlap = w.characterize_overlap(v);
    if overlap == 0 {
        data.total_replace(out, old)
            .expect("storecopyeffects: disjoint effect");
        data.op_destroy(effect);
        return;
    }
    if overlap == 2 {
        data.op_uninsert(effect);
        data.op_set_input(effect, written, 0)
            .expect("storecopyeffects: exact input");
        data.op_remove_input(effect, 1);
        data.op_set_opcode(effect, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        data.op_insert_after(effect, copy);
        return;
    }
    let big = w.get_space().is_big_endian();
    let wlo = w.get_offset();
    let vlo = v.get_offset();
    let ws = w.get_size();
    let vs = v.get_size();
    let start = u128::from(wlo).max(u128::from(vlo));
    let end = (u128::from(wlo) + ws as u128).min(u128::from(vlo) + vs as u128);
    let before = (start - u128::from(vlo)) as i32;
    let middle = (end - start) as i32;
    let after = vs - before - middle;
    let mut anchor = copy;
    let mut parts = Vec::new();
    if before != 0 {
        parts.push(slice(data, old, 0, before, big, &mut anchor));
    }
    parts.push(slice(
        data,
        written,
        (start - u128::from(wlo)) as i32,
        middle,
        big,
        &mut anchor,
    ));
    if after != 0 {
        parts.push(slice(data, old, vs - after, after, big, &mut anchor));
    }
    data.op_uninsert(effect);
    if parts.len() == 1 {
        data.op_remove_input(effect, 1);
        data.op_set_input(effect, parts[0], 0)
            .expect("storecopyeffects: slice input");
        data.op_set_opcode(effect, crate::typeop::type_op_for(OpCode::CPUI_COPY));
    } else {
        let mut joined = parts[0];
        for (index, &part) in parts.iter().enumerate().skip(1) {
            let last = index + 1 == parts.len();
            let op = if last {
                effect
            } else {
                let addr = data.obank().get(effect).unwrap().get_addr().clone();
                let op = data.new_op(2, addr);
                let size = data.vbank().get(joined).unwrap().get_size()
                    + data.vbank().get(part).unwrap().get_size();
                data.new_unique_out(size, op)
                    .expect("storecopyeffects: joined output");
                op
            };
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_PIECE));
            data.op_set_input(op, if big { joined } else { part }, 0)
                .expect("storecopyeffects: high piece");
            data.op_set_input(op, if big { part } else { joined }, 1)
                .expect("storecopyeffects: low piece");
            if !last {
                data.op_insert_after(op, anchor);
                anchor = op;
                joined = data.obank().get(op).unwrap().get_out().unwrap();
            }
        }
    }
    data.op_insert_after(effect, anchor);
}

fn slice(
    data: &mut Funcdata,
    vn: VarnodeId,
    offset: i32,
    size: i32,
    big: bool,
    anchor: &mut OpId,
) -> VarnodeId {
    let whole = data.vbank().get(vn).unwrap().get_size();
    if size == whole {
        return vn;
    }
    let trunc = if big { whole - offset - size } else { offset };
    let addr = data.obank().get(*anchor).unwrap().get_addr().clone();
    let op = data.new_op(2, addr);
    data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_SUBPIECE));
    let out = data
        .new_unique_out(size, op)
        .expect("storecopyeffects: sliced output");
    data.op_set_input(op, vn, 0)
        .expect("storecopyeffects: slice source");
    let count = data.new_constant(4, trunc as u64);
    data.op_set_input(op, count, 1)
        .expect("storecopyeffects: slice offset");
    data.op_insert_after(op, *anchor);
    *anchor = op;
    out
}

#[cfg(test)]
#[path = "kuna_storecopyeffects/tests.rs"]
mod tests;
