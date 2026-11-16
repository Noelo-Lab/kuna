//! (kuna) A direct access at a known stack offset aliases only the bytes it
//! touches.
//!
//! `checkInputTrialUse` drops a stack trial when the frame's alias boundary
//! (`AliasChecker::hasLocalAlias`) reaches it: some pointer formed from the
//! stack pointer sits at or below the slot, so anything above it may be
//! reached. A `LOAD`/`STORE` through such a pointer counts until heritage turns
//! it into a stack variable, and a pointer whose offset only becomes known late
//! — a stack pointer restored from the frame slot it was saved in, after an
//! `alloca` — is still one when call inputs are scored. The call after the
//! restore stores its own argument through it, below the earlier calls'
//! arguments, and those were dropped (`reg(1)` printed for `reg(1, n)`).
//!
//! A pointer that is the stack-pointer input plus constants, through copies
//! and frame slots only, and that is used solely as the address of `LOAD`s and
//! `STORE`s, touches exactly `[offset, offset + size)`. Such a pointer aliases a
//! trial only when those bytes overlap it; every other pointer keeps the
//! upstream boundary. Always on.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::varmap::{AddBase, AliasChecker};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

/// How many definitions [`fixed_offset`] follows back to the stack pointer.
const MAX_STEPS: usize = 16;

/// Does a gathered pointer reach the trial at `[offset, offset + size)`?
/// Call after `has_local_alias` answered `true` for it.
pub fn aliases(data: &Funcdata, aliascheck: &mut AliasChecker, offset: uintb, size: int4) -> bool {
    let (boundary, direct) = aliascheck.split_direct(|base: &AddBase| direct_bytes(data, base.base));
    let end = offset as u128 + size.max(1) as u128;
    offset >= *boundary
        || direct.iter().any(|&(start, bytes)| (start as u128) < end && (offset as u128) < start as u128 + bytes as u128)
}

/// `vn` stopped or started being a call input: redraw the cached split if
/// that changed whether it is direct.
pub fn refresh(data: &Funcdata, aliascheck: &mut AliasChecker, vn: VarnodeId) {
    aliascheck.refresh_split_for(vn, || direct_bytes(data, vn));
}

/// The inputs of call `op`, taken before its argument list is rewritten and
/// handed to [`refresh_rewritten`] after.
pub fn call_inputs(data: &Funcdata, op: OpId) -> Vec<VarnodeId> {
    data.obank().get(op).map(|o| (0..o.num_input()).filter_map(|i| o.get_in(i)).collect()).unwrap_or_default()
}

/// [`refresh`] every input the rewrite of call `op` from `before` dropped or added.
pub fn refresh_rewritten(data: &Funcdata, aliascheck: &mut AliasChecker, op: OpId, before: &[VarnodeId]) {
    let after = call_inputs(data, op);
    for &vn in before.iter().filter(|v| !after.contains(v)).chain(after.iter().filter(|v| !before.contains(v))) {
        refresh(data, aliascheck, vn);
    }
}

/// The widest access through `vn` when it is only ever the address of a
/// `LOAD`/`STORE` and a fixed offset from the stack pointer.
fn direct_bytes(data: &Funcdata, vn: VarnodeId) -> Option<uintb> {
    let mut widest = 0;
    for id in data.vbank().get(vn)?.descend_iter() {
        let op = data.obank().get(id)?;
        let accessed = match op.code() {
            OpCode::CPUI_LOAD if op.get_in(1) == Some(vn) => op.get_out()?,
            OpCode::CPUI_STORE if op.get_in(1) == Some(vn) && op.get_in(2) != Some(vn) => op.get_in(2)?,
            _ => return None,
        };
        widest = widest.max(data.vbank().get(accessed)?.get_size().max(0) as uintb);
    }
    (widest > 0 && fixed_offset(data, vn)).then_some(widest)
}

/// Is `vn` the stack-pointer input plus constants, through copies only?
fn fixed_offset(data: &Funcdata, mut vn: VarnodeId) -> bool {
    for _ in 0..MAX_STEPS {
        let Some(node) = data.vbank().get(vn) else { return false };
        if node.is_input() {
            return node.get_space().get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR
                && data.get_arch().manage().get_stack_space().is_some_and(|stack| {
                    stack.get_spacebase(0).is_ok_and(|sp| sp.offset == node.get_offset() && sp.size as int4 == node.get_size())
                });
        }
        let Some(op) = node.get_def().and_then(|id| data.obank().get(id)) else { return false };
        let constant = |slot| op.get_in(slot).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_constant());
        let next = match op.code() {
            OpCode::CPUI_COPY => op.get_in(0),
            OpCode::CPUI_INT_ADD if constant(1) => op.get_in(0),
            OpCode::CPUI_INT_ADD if constant(0) => op.get_in(1),
            OpCode::CPUI_INT_SUB if constant(1) => op.get_in(0),
            _ => None,
        };
        let Some(next) = next else { return false };
        vn = next;
    }
    false
}
