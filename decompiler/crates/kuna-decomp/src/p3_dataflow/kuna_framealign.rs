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

use std::rc::Rc;

use kuna_base::types::{int4, uintb};

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// The low bits a realigned frame may assume clear in the stack-pointer input.
const FRAME_ALIGN_BITS: uintb = 0xff;

/// The alignment of the stack pointer at entry: the ABI's where the stack slots
/// misstate it ([`abi_stack_align`]), else the widest stack-slot alignment of
/// the default model's inputs (8 on x86-64, 4 on i386 and SuperH, 1 on AVR,
/// none on 6502); never more than 8 or twice the pointer width.
pub(crate) fn entry_alignment(data: &Funcdata, size: int4) -> uintb {
    let arch = data.get_arch();
    let align = if arch.stack_abi_align != 0 {
        arch.stack_abi_align
    } else {
        match (arch.default_fp(), arch.manage().get_stack_space()) {
            (Some(model), Some(stack)) => model.input_opt().map_or(1, |input| {
                input
                    .get_entry()
                    .iter()
                    .filter(|e| Rc::ptr_eq(e.get_space(), stack))
                    .map(|e| e.get_align().max(1) as uintb)
                    .max()
                    .unwrap_or(1)
            }),
            _ => 1,
        }
    };
    let align = align.min(8).min(2 * size.max(1) as uintb);
    if align.is_power_of_two() { align } else { 1 << align.ilog2() }
}

/// The stack alignment an ABI keeps where its 4-byte stack slots misstate it,
/// or 0 to take the slots: 8 on ARM (AAPCS keeps 8 at every public interface),
/// AArch64, MIPS, PowerPC, RISC-V, SPARC and LoongArch, whose ABIs keep at
/// least 8; 2 on the 68000, whose slots say 4 while its stack is word-aligned.
pub(crate) fn abi_stack_align(archid: &str) -> uintb {
    match archid.split(':').next().unwrap_or("") {
        "ARM" | "AARCH64" | "MIPS" | "PowerPC" | "RISCV" | "sparc" | "Loongarch" => 8,
        "68000" => 2,
        _ => 0,
    }
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
