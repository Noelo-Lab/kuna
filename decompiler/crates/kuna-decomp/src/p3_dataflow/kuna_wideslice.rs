//! Reduce demanded byte ranges through wide SSA expressions — the `wideslice`
//! decision point.
//!
//! Heritage widens each partial lane write of a vector register by joining it
//! with the bytes below it (`Heritage::normalizeWriteSize`, `heritage.cc:417`),
//! so a run of byte writes leaves intermediates 2..15 bytes wide, and
//! `RuleConcatZext`/`RuleConcatZero` keep manufacturing such widths while they
//! fold zero lanes.  Nothing upstream takes a 9..15-byte value apart again:
//! `SubvariableFlow` stops at a 64-bit mask, `RuleDumptyHump`, `RuleSubCommute`
//! and `RuleShiftSub` do not cross a piece boundary, and `ActionLaneDivide`
//! accepts only lane-aligned `PIECE`s.  This rule pushes the demanded
//! `SUBPIECE` through `PIECE`, byte-aligned logical shifts and
//! `INT_AND`/`INT_OR`/`INT_XOR` as bitvector identities: a slice straddling a
//! `PIECE` becomes a `PIECE` of two slices, a slice reaching into a shift's
//! zero fill becomes a zero-extended shorter slice.  It is independent of
//! source types, function signatures, loop shapes and the vector instruction
//! that produced the SSA.
//!
//! Power-of-two widths (8, 16, 32, 64) are refused on purpose.  Those are the
//! laned-register widths `ActionLaneDivide`, `simdlane` and `constspaceload`
//! own, and slicing through them before lane division runs breaks their lane
//! view: without the gate seven stage assertions regress and the byte-sum loop
//! itself comes out as `CONCAT12(...) & 0xffffffffffff00ff`.  Rebuilding the
//! heritage ladder as an aligned tree instead was tried and does not reach the
//! clean output, because the rule pool recreates the odd widths.
//! `option wideslice off` leaves the wide intermediates to upstream's rules.
//!
//! Offsets are significance offsets, independent of memory endianness.

use crate::action::{ActionGroupList, Rule};
use crate::context::{OpId, TypeOp, VarnodeId};
use crate::dtype::type_metatype as Meta;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode::{self, *};

pub struct RuleWideSlice {
    group: String,
}

impl RuleWideSlice {
    pub fn new(group: impl Into<String>) -> Self {
        Self { group: group.into() }
    }
}

fn size(data: &Funcdata, vn: VarnodeId) -> i32 {
    data.vbank().get(vn).unwrap().get_size()
}

fn set_op(data: &mut Funcdata, op: OpId, code: OpCode, inputs: &[VarnodeId]) {
    data.op_set_all_input(op, inputs).expect("wide-slice inputs");
    data.op_set_opcode(op, TypeOp::new(code, 0, format!("{code:?}")));
}

fn insert(data: &mut Funcdata, before: OpId, code: OpCode, width: i32, inputs: &[VarnodeId]) -> VarnodeId {
    let address = data.obank().get(before).unwrap().get_addr().clone();
    let op = data.new_op(inputs.len() as i32, address);
    set_op(data, op, code, inputs);
    let out = data.new_unique_out(width, op).expect("wide-slice output");
    data.op_insert_before(op, before);
    out
}

fn slice(data: &mut Funcdata, before: OpId, vn: VarnodeId, offset: i32, width: i32) -> VarnodeId {
    if offset == 0 && width == size(data, vn) {
        return vn;
    }
    let node = data.vbank().get(vn).unwrap();
    if node.is_constant() && node.get_size() <= 8 {
        let mask = u64::MAX >> ((8 - width) * 8);
        let value = (node.get_offset() >> (offset * 8)) & mask;
        return data.new_constant(width, value);
    }
    let off = data.new_constant(4, offset as u64);
    insert(data, before, CPUI_SUBPIECE, width, &[vn, off])
}

impl Rule for RuleWideSlice {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![CPUI_SUBPIECE]
    }

    fn clone_rule(&self, groups: &ActionGroupList) -> Option<Box<dyn Rule>> {
        groups
            .contains(&self.group)
            .then(|| Box::new(Self::new(self.group.clone())) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> i32 {
        if !data.get_arch().wide_slice_reduce {
            return 0;
        }
        let Some(operation) = data.obank().get(op) else {
            return 0;
        };
        let (Some(input), Some(offset), Some(output)) = (operation.get_in(0), operation.get_in(1), operation.get_out())
        else {
            return 0;
        };
        let off = data.vbank().get(offset).unwrap();
        if !off.is_constant() || off.get_offset() > i32::MAX as u64 {
            return 0;
        }
        let offset = off.get_offset() as i32;
        let node = data.vbank().get(input).unwrap();
        let out = data.vbank().get(output).unwrap();
        let input_size = node.get_size();
        let width = out.get_size();
        if input_size <= 8
            || input_size & (input_size - 1) == 0
            || width <= 0
            || width >= input_size
            || offset.checked_add(width).is_none_or(|end| end > input_size)
            || out.is_precis_lo()
            || out.is_precis_hi()
            || node.is_type_lock()
            || !matches!(
                node.get_type().get_metatype(),
                Meta::TYPE_UNKNOWN | Meta::TYPE_INT | Meta::TYPE_UINT
            )
        {
            return 0;
        }
        let Some(def) = node.get_def().and_then(|id| data.obank().get(id)) else {
            return 0;
        };
        let code = def.code();
        let Some(left) = def.get_in(0) else { return 0 };
        let Some(right) = def.get_in(1) else { return 0 };
        // Do not move unheritaged register reads to a different instruction.
        if [left, right].iter().any(|&v| {
            let n = data.vbank().get(v).unwrap();
            n.is_free() && !n.is_constant()
        }) {
            return 0;
        }
        if code == CPUI_PIECE {
            let low_size = size(data, right);
            if size(data, left).checked_add(low_size) != Some(input_size) {
                return 0;
            }
            let result = if offset >= low_size {
                slice(data, op, left, offset - low_size, width)
            } else if offset + width <= low_size {
                slice(data, op, right, offset, width)
            } else {
                let low_width = low_size - offset;
                let lo = slice(data, op, right, offset, low_width);
                let hi = slice(data, op, left, 0, width - low_width);
                set_op(data, op, CPUI_PIECE, &[hi, lo]);
                return 1;
            };
            set_op(data, op, CPUI_COPY, &[result]);
            return 1;
        }
        if matches!(code, CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR)
            && size(data, left) == input_size
            && size(data, right) == input_size
        {
            let lhs = slice(data, op, left, offset, width);
            let rhs = slice(data, op, right, offset, width);
            set_op(data, op, code, &[lhs, rhs]);
            return 1;
        }
        if !matches!(code, CPUI_INT_LEFT | CPUI_INT_RIGHT) || size(data, left) != input_size {
            return 0;
        }
        let shift = data.vbank().get(right).unwrap();
        if !shift.is_constant() || shift.get_offset() % 8 != 0 {
            return 0;
        }
        let shift = shift.get_offset() / 8;
        if shift >= input_size as u64 {
            let zero = data.new_constant(width, 0);
            set_op(data, op, CPUI_COPY, &[zero]);
            return 1;
        }
        let start = if code == CPUI_INT_LEFT {
            offset as i64 - shift as i64
        } else {
            offset as i64 + shift as i64
        };
        let first = start.max(0);
        let end = (start + width as i64).min(input_size as i64);
        if first >= end {
            let zero = data.new_constant(width, 0);
            set_op(data, op, CPUI_COPY, &[zero]);
            return 1;
        }
        let mut result = slice(data, op, left, first as i32, (end - first) as i32);
        if end - first < width as i64 {
            result = insert(data, op, CPUI_INT_ZEXT, width, &[result]);
        }
        if start < 0 {
            let amount = data.new_constant(4, (-start * 8) as u64);
            set_op(data, op, CPUI_INT_LEFT, &[result, amount]);
        } else {
            set_op(data, op, CPUI_COPY, &[result]);
        }
        1
    }
}

#[cfg(test)]
mod tests;
