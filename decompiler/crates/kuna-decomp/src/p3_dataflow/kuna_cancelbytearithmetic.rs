//! `cancelbytearithmetic` folds an exact modular cancellation that an
//! uncommuted low-byte left shift otherwise hides.
//!
//! For `1 <= s < 8`, the identity is
//! `low8(x) * (-2^s) + low8(x << s) == 0 (mod 256)`.  The matcher requires
//! that exact one-byte SSA tree, with an independent preserved addend, and
//! rewrites the outer add directly to the preserved addend.  It never creates
//! a narrowed left shift, so a signed `char << s` cannot reach emitted C.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshalling element id for the option (kuna 4000+ range).
pub const ELEM_CANCELBYTEARITHMETIC: ElementId = ElementId::new("cancelbytearithmetic", 4166);

/// The `cancelbytearithmetic on|off` option.
pub struct OptionCancelByteArithmetic;

impl OptionCancelByteArithmetic {
    /// Parse the option and return its confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((
            val,
            format!("Cancelling byte arithmetic fold turned {prop}"),
        ))
    }
}

/// A proven cancellation, represented by the one operation that must change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ByteArithmeticCancellation {
    root: OpId,
    preserved: VarnodeId,
}

fn varnode_size(data: &Funcdata, vn: VarnodeId) -> Option<int4> {
    data.vbank().get(vn).map(|vn| vn.get_size())
}

fn written_op(data: &Funcdata, vn: VarnodeId, opcode: OpCode) -> Option<OpId> {
    data.vbank()
        .get(vn)
        .and_then(|vn| vn.get_def())
        .filter(|&op| data.obank().get(op).is_some_and(|op| op.code() == opcode))
}

fn constant_value(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    data.vbank()
        .get(vn)
        .filter(|vn| vn.is_constant())
        .map(|vn| vn.get_offset())
}

fn other_binary_input(data: &Funcdata, op: OpId, known: VarnodeId) -> Option<VarnodeId> {
    let op = data.obank().get(op)?;
    if op.num_input() != 2 {
        return None;
    }
    let left = op.get_in(0)?;
    let right = op.get_in(1)?;
    if left == known {
        Some(right)
    } else if right == known {
        Some(left)
    } else {
        None
    }
}

fn is_matching_low_piece(
    data: &Funcdata,
    candidate: VarnodeId,
    source: VarnodeId,
    consumer: OpId,
) -> bool {
    if varnode_size(data, candidate) != Some(1) || data.lone_descend(candidate) != Some(consumer) {
        return false;
    }
    let Some(subpiece) = written_op(data, candidate, OpCode::CPUI_SUBPIECE) else {
        return false;
    };
    let Some(op) = data.obank().get(subpiece) else {
        return false;
    };
    op.num_input() == 2
        && op.get_in(0) == Some(source)
        && op.get_in(1).and_then(|vn| constant_value(data, vn)) == Some(0)
}

fn matching_negative_term(
    data: &Funcdata,
    candidate: VarnodeId,
    source: VarnodeId,
    coefficient: u64,
    consumer: OpId,
) -> bool {
    if varnode_size(data, candidate) != Some(1) || data.lone_descend(candidate) != Some(consumer) {
        return false;
    }
    let Some(mult) = written_op(data, candidate, OpCode::CPUI_INT_MULT) else {
        return false;
    };
    let Some(op) = data.obank().get(mult) else {
        return false;
    };
    if op.num_input() != 2 {
        return false;
    }
    for constant_slot in 0..2 {
        let value_slot = 1 - constant_slot;
        let Some(constant) = op.get_in(constant_slot) else {
            continue;
        };
        let Some(value) = op.get_in(value_slot) else {
            continue;
        };
        if varnode_size(data, constant) == Some(1)
            && constant_value(data, constant) == Some(coefficient)
            && is_matching_low_piece(data, value, source, mult)
        {
            return true;
        }
    }
    false
}

/// Recognize the exact cancellation tree rooted above `subpiece`.
///
/// The input is deliberately required to be the filed witness's pure
/// `INT_AND` of two run-time values.  Constant masks are bit-field packing, and
/// direct CALL/LOAD producers may carry effects; all of those decline before
/// any graph mutation.
pub(crate) fn find_cancellation(
    data: &Funcdata,
    subpiece: OpId,
    shift: OpId,
    outsize: int4,
) -> Option<ByteArithmeticCancellation> {
    if !data.get_arch().cancel_byte_arithmetic || outsize != 1 {
        return None;
    }
    let shift_op = data.obank().get(shift)?;
    if shift_op.code() != OpCode::CPUI_INT_LEFT || shift_op.num_input() != 2 {
        return None;
    }
    let source = shift_op.get_in(0)?;
    let amount = shift_op.get_in(1).and_then(|vn| constant_value(data, vn))?;
    if !(1..8).contains(&amount) {
        return None;
    }

    let producer = written_op(data, source, OpCode::CPUI_INT_AND)?;
    let producer = data.obank().get(producer)?;
    if producer.num_input() != 2
        || (0..2).any(|slot| {
            producer
                .get_in(slot)
                .and_then(|vn| data.vbank().get(vn))
                .is_none_or(|vn| vn.is_constant())
        })
    {
        return None;
    }

    let subpiece_op = data.obank().get(subpiece)?;
    if subpiece_op.code() != OpCode::CPUI_SUBPIECE
        || subpiece_op.num_input() != 2
        || subpiece_op.get_in(0) != shift_op.get_out()
        || subpiece_op
            .get_in(1)
            .and_then(|vn| constant_value(data, vn))
            != Some(0)
    {
        return None;
    }
    let shifted_low = subpiece_op.get_out()?;
    if varnode_size(data, shifted_low) != Some(1) {
        return None;
    }

    let root = data.lone_descend(shifted_low)?;
    let root_op = data.obank().get(root)?;
    if root_op.code() != OpCode::CPUI_INT_ADD
        || root_op.num_input() != 2
        || root_op.get_out().and_then(|vn| varnode_size(data, vn)) != Some(1)
    {
        return None;
    }
    let inner_out = other_binary_input(data, root, shifted_low)?;
    if varnode_size(data, inner_out) != Some(1) || data.lone_descend(inner_out) != Some(root) {
        return None;
    }
    let inner = written_op(data, inner_out, OpCode::CPUI_INT_ADD)?;
    let inner_op = data.obank().get(inner)?;
    if inner_op.num_input() != 2 {
        return None;
    }

    let coefficient = 0u64.wrapping_sub(1u64 << amount) & 0xff;
    for negative_slot in 0..2 {
        let negative = inner_op.get_in(negative_slot)?;
        let preserved = inner_op.get_in(1 - negative_slot)?;
        if matching_negative_term(data, negative, source, coefficient, inner) {
            return Some(ByteArithmeticCancellation { root, preserved });
        }
    }
    None
}

/// Apply a previously validated cancellation without touching the dead input
/// tree.  Normal dead-code elimination can then retain any effectful ancestors
/// according to their existing op flags.
pub(crate) fn apply_cancellation(data: &mut Funcdata, plan: ByteArithmeticCancellation) {
    data.op_set_opcode(plan.root, crate::typeop::type_op_for(OpCode::CPUI_COPY));
    data.op_set_input(plan.root, plan.preserved, 0)
        .expect("cancelbytearithmetic: root input");
    data.op_remove_input(plan.root, 1);
}
