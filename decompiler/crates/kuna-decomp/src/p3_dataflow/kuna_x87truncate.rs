//! An x87 integer conversion whose control word provably selects round-toward-zero
//! is the C conversion `(intN)x`: rewrite `x87_float_to_intN(x, cw)` to
//! `FLOAT_TRUNC(x)` when bits 10-11 of `cw` are known to be set, and drop it when
//! its result is unused.

use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

pub const USEROP_NAMES: [&[u8]; 3] = [b"x87_float_to_int16", b"x87_float_to_int32", b"x87_float_to_int64"];

const ROUND_TOWARD_ZERO: u64 = 0x0c00;

/// Bits of `vn` that are 1 on every path through bit operations, copies and joins;
/// `budget` bounds the definitions visited.
fn known_ones(data: &Funcdata, vn: VarnodeId, budget: &mut u32) -> u64 {
    let Some(v) = data.vbank().get(vn) else { return 0 };
    let size = v.get_size();
    if !(1..=8).contains(&size) {
        return 0;
    }
    let mask = kuna_base::address::calc_mask(size);
    if v.is_constant() {
        return v.get_offset() & mask;
    }
    if *budget == 0 {
        return 0;
    }
    *budget -= 1;
    let Some(op) = v.get_def().and_then(|id| data.obank().get(id)) else { return 0 };
    let mut input = |slot: int4| op.get_in(slot).map_or(0, |i| known_ones(data, i, budget));
    let size_of = |slot: int4| op.get_in(slot).and_then(|i| data.vbank().get(i)).map_or(0, |i| i.get_size());
    let ones = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => input(0),
        OpCode::CPUI_INT_OR => input(0) | input(1),
        OpCode::CPUI_INT_AND => input(0) & input(1),
        OpCode::CPUI_PIECE => {
            let low = size_of(1);
            if !(1..8).contains(&low) {
                return 0;
            }
            input(0) << (low * 8) | input(1)
        }
        OpCode::CPUI_SUBPIECE => {
            let shift = op.get_in(1).and_then(|i| data.vbank().get(i)).map_or(8, |i| i.get_offset());
            if shift >= 8 {
                return 0;
            }
            input(0) >> (shift * 8)
        }
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).fold(mask, |acc, slot| acc & input(slot)),
        _ => 0,
    };
    ones & mask
}

/// Rewrite a truncating `x87_float_to_intN` (see the module docs).
pub struct RuleX87Truncate {
    group: String,
}

impl RuleX87Truncate {
    pub fn new(group: &str) -> RuleX87Truncate {
        RuleX87Truncate { group: group.to_string() }
    }
}

impl Rule for RuleX87Truncate {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_CALLOTHER]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleX87Truncate::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if data.get_arch().x87_float_to_int_userops.is_empty() {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        if o.num_input() != 3 {
            return 0;
        }
        let used = o.get_out().is_some();
        let (Some(id), Some(control)) = (o.get_in(0), o.get_in(2)) else { return 0 };
        let selected = data.vbank().get(id).is_some_and(|v| {
            v.is_constant() && data.get_arch().x87_float_to_int_userops.contains(&(v.get_offset() as uint4))
        });
        if !selected || known_ones(data, control, &mut 32) & ROUND_TOWARD_ZERO != ROUND_TOWARD_ZERO {
            return 0;
        }
        if !used {
            data.op_destroy(op);
            return 1;
        }
        data.op_remove_input(op, 2);
        data.op_remove_input(op, 0);
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_FLOAT_TRUNC));
        1
    }
}
