//! How aligned the stack pointer is at entry, and where a frame realigns it.
//!
//! Upstream gave the stack-pointer input a non-zero mask with its low byte
//! clear, so any rounding of a stack address to 256 bytes or less folded into
//! a frame offset. That is false for a buffer rounded up inside the frame,
//! `(sp + 0x3f) & -0x40`, while the stack pointer keeps its own value. The input
//! now has the alignment the call guarantees ([`entry_alignment`]), and the AND
//! rules take the 256-byte reading ([`and_operand_nz_mask`]) only for an AND
//! that writes the stack pointer itself: a prologue's `and rsp,-0x40`, after
//! which every frame offset is measured from the realigned pointer.

use kuna_base::types::{int4, uintb};

use crate::context::{OpId, VarnodeId};
use crate::fspec::EXTRAPOP_UNKNOWN;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// The low bits a realigned frame may assume clear in the stack-pointer input.
const FRAME_ALIGN_BITS: uintb = 0xff;

/// The alignment of a `size`-byte stack pointer at entry. A call that pushes its
/// return address (the default model's extrapop) leaves it aligned to that push:
/// 8 on x86-64, 4 on i386. A call that pushes nothing leaves the ABI's alignment,
/// at least 8 on every such target kuna specs (SPARC, MIPS, ARM, AArch64, PPC,
/// RISC-V). Neither is more than twice the pointer width.
pub(crate) fn entry_alignment(data: &Funcdata, size: int4) -> uintb {
    let extrapop = data.get_arch().default_fp().map(|m| m.get_extra_pop());
    let align: uintb = match extrapop {
        Some(0) => 8,
        Some(ep) if ep > 0 && ep != EXTRAPOP_UNKNOWN => 1 << (ep as uintb).trailing_zeros(),
        _ => size.max(1) as uintb,
    };
    align.min(8).min(2 * size.max(1) as uintb)
}

/// The non-zero mask of `vn` as an operand of the `INT_AND` `andop`: the
/// stack-pointer input counts as 256-aligned when `andop` realigns the frame.
pub(crate) fn and_operand_nz_mask(data: &Funcdata, vn: VarnodeId, andop: OpId) -> uintb {
    let Some(v) = data.vbank().get(vn) else {
        return 0;
    };
    let nzm = v.get_nz_mask();
    if v.is_input() && v.is_spacebase() && realigns_frame(data, vn, andop) {
        nzm & !FRAME_ALIGN_BITS
    } else {
        nzm
    }
}

/// Does `andop` write the stack pointer `spin` is the input of, directly or
/// through a `COPY`?
fn realigns_frame(data: &Funcdata, spin: VarnodeId, andop: OpId) -> bool {
    let Some(sp) = data.vbank().get(spin) else {
        return false;
    };
    let is_sp = |vn: VarnodeId| {
        data.vbank()
            .get(vn)
            .is_some_and(|v| v.get_size() == sp.get_size() && v.get_addr() == sp.get_addr())
    };
    let Some(out) = data.obank().get(andop).and_then(|o| o.get_out()) else {
        return false;
    };
    if is_sp(out) {
        return true;
    }
    data.vbank().get(out).is_some_and(|o| {
        o.descend_iter().any(|d| {
            data.obank().get(d).is_some_and(|op| {
                op.code() == OpCode::CPUI_COPY && op.get_out().is_some_and(is_sp)
            })
        })
    })
}
