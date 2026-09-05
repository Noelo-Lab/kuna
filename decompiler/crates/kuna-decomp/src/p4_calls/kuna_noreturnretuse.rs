//! P4 return-trial tolerance for a terminating no-return call — the
//! `noreturnretuse` decision point.
//!
//! # The gap
//!
//! `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, `funcdata_varnode.cc:1851`)
//! decides whether a Varnode reaching `opmatch` is used *only* there. When the
//! match is the function's RETURN, any other CALL that consumes the same Varnode
//! rejects the output trial, and `ActionOutputPrototype` then types the function
//! `void`.
//!
//! A status helper hits that rule for free on an ABI where the return register is
//! also the first argument register (ARM/AArch64 `r0`/`w0`, and any convention
//! that overlaps them). The value is returned on the normal path and handed to a
//! failure handler on the error path:
//!
//! ```text
//!   csetm  w0, ne            ; the status
//!   cmp    w2, w1
//!   b.ne   .Lfail
//!   ret                      ; <- opmatch
//! .Lfail:
//!   bl     __stack_chk_fail  ; same w0, and this block never returns
//! ```
//!
//! The failure block ends in an artificial `noreturn` halt, so no RETURN is
//! reachable from it and the two uses never compete. Rejecting the trial anyway
//! costs the whole signature: `int guarded_status(int)` comes back as
//! `void guarded_status(int)`.
//!
//! # The rule
//!
//! Skip a CALL/CALLIND use when all three hold: the matched op is a RETURN, the
//! call is the second-to-last op of its block, and the block's last op is a halt
//! flagged no-return. A call that may return, a halt further down the block, and
//! every non-RETURN match keep the upstream rejection.

use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;

/// Whether the CALL/CALLIND at `op` may be skipped while scoring the trial
/// matched at `opmatch`. Off restores the upstream blanket rejection.
pub fn call_cannot_reach_return(data: &Funcdata, opmatch: OpId, op: OpId) -> bool {
    if !data.get_arch().noreturn_ret_use {
        return false;
    }
    if data.obank().get(opmatch).map(|m| m.code()) != Some(OpCode::CPUI_RETURN) {
        return false;
    }
    call_ends_in_noreturn_halt(data, op)
}

/// Whether `call` is immediately followed by its block's final op and that op is
/// an artificial halt marked no-return.
pub fn call_ends_in_noreturn_halt(data: &Funcdata, call: OpId) -> bool {
    let Some(parent) = data.obank().get(call).and_then(|op| op.get_parent()) else {
        return false;
    };
    let Some(last) = data.bblocks_ref().struct_last_op(parent) else {
        return false;
    };
    if data.op_next_op(call) != Some(last) {
        return false;
    }
    data.obank()
        .get(last)
        .is_some_and(|op| (op.get_halt_type() & crate::op::pcodeop_flags::noreturn) != 0)
}
