//! Separate post-call resumption from branches that can bypass a call in a terminal lift.

use kuna_num::opcodes::OpCode;

use crate::listing::model::{FlowKind, FlowType, RawOp};

pub(super) fn candidate(flow: FlowType, fall_through: Option<u64>, has_branches: bool) -> bool {
    flow.is_call && flow.kind == FlowKind::Return && fall_through.is_none() && has_branches
}

/// Reject any instruction exit before the call, or a relative edge crossing it.
/// Edges confined to the prefix cannot make a later exit reachable without the call.
pub(super) fn all_branches_after_call(ops: &[RawOp]) -> bool {
    let Some(call) = ops.iter().position(|op| matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)) else {
        return false;
    };
    ops[..call].iter().enumerate().all(|(index, op)| match op.opcode {
        OpCode::CPUI_RETURN | OpCode::CPUI_BRANCHIND => false,
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => op.in0.as_ref().is_some_and(|dest| {
            let addr = dest.get_addr();
            addr.is_constant() && index.wrapping_add(addr.get_offset() as usize) <= call
        }),
        _ => true,
    })
}
