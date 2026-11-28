//! The calls a revisit of a heritaged range must guard again.
//!
//! A later heritage pass revisits a range when a new read lands inside one an
//! earlier pass heritaged. `collect` sets aside every MULTIEQUAL and INDIRECT
//! marker narrower than the range, and `removeRevisitedMarkers` turns each into
//! a SUBPIECE of the range as it stands after the marker's op, trusting the
//! guards of the same pass to put a replacement INDIRECT there. Those guards
//! run only for a range with new addresses, so in a revisit of old addresses
//! alone the call's INDIRECT is never rebuilt and the SUBPIECE reads the value
//! from before the call.
//!
//! For an INDIRECT creation that is false about the program: the call killed
//! those bytes or returned its result in them, and the caller now reads what it
//! had there before. `RulePullsubIndirect` narrows a creation to the bytes read
//! after the call (the four of `movss %xmm0,sink` inside the eight-byte
//! `XMM0_Qa`), and a call argument added on a later pass (`XMM0_Qa` handed to
//! the next call) revisits the eight bytes: gcc -O2 -mavx2 `sink = avxsum(f,
//! 40)` printed `avxsum(v23,..); sink = (int)v23;` with `v23` the result of the
//! call before. [`retired_creation_calls`] names the calls such a revisit
//! strips of a creation and no full-size marker of the range still guards;
//! heritage guards each again over the whole range for its effect only, taking
//! no new parameter or return trial, so the creation is rebuilt where the first
//! pass made it and an output trial it carried still finds it.

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// The live calls whose INDIRECT creation among `remove`, the markers a revisit
/// retires, no INDIRECT of `write`, the range's full-size writes, replaces.
pub(crate) fn retired_creation_calls(fd: &Funcdata, remove: &[VarnodeId], write: &[VarnodeId]) -> Vec<OpId> {
    let target = |vn: VarnodeId, creation: bool| -> Option<OpId> {
        let op = fd.obank().get(fd.vbank().get(vn)?.get_def()?)?;
        if op.code() != OpCode::CPUI_INDIRECT || op.is_indirect_creation() != creation {
            return None;
        }
        let iop = fd.vbank().get(op.get_in(1)?)?.get_addr().get_offset();
        Some(crate::funcdata_varnode::op_iop_decode(iop))
    };
    let guarded: Vec<OpId> = write.iter().filter_map(|&vn| target(vn, true).or_else(|| target(vn, false))).collect();
    let mut calls: Vec<OpId> = Vec::new();
    for call in remove.iter().filter_map(|&vn| target(vn, true)) {
        let live = fd
            .obank()
            .get(call)
            .is_some_and(|o| !o.is_dead() && matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND));
        if live && !guarded.contains(&call) && !calls.contains(&call) {
            calls.push(call);
        }
    }
    calls
}
