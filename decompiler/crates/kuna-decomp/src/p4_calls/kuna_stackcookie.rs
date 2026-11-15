//! (kuna) The frame cookie MSVC stores on the stack is no argument.
//!
//! `/GS` and the EH frame store a global XORed with the stack or frame pointer
//! (`mov eax,[___security_cookie]; xor eax,esp; push eax`). The slot belongs to
//! the frame: only the epilogue's check reads it back. A call below it would
//! otherwise be offered the slot as a stack argument nothing else claims.

use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;

/// Ops [`holds_cookie`] follows back from the trial's value.
const DEPTH: u32 = 32;

/// Does the stack value `vn` reaching a call hold the frame cookie on every
/// path: a value of the default data space XORed with the stack pointer,
/// reached through copies, the indirect effects of calls and joins?
pub fn holds_cookie(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut seen = Vec::new();
    cookie(data, vn, &mut seen, DEPTH)
}

fn cookie(data: &Funcdata, vn: VarnodeId, seen: &mut Vec<VarnodeId>, depth: u32) -> bool {
    if seen.contains(&vn) {
        return true;
    }
    seen.push(vn);
    let Some(op) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
        return false;
    };
    let input = |k| op.get_in(k);
    match op.code() {
        _ if depth == 0 => false,
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => input(0).is_some_and(|v| cookie(data, v, seen, depth - 1)),
        OpCode::CPUI_MULTIEQUAL => {
            op.num_input() > 0 && (0..op.num_input()).all(|k| input(k).is_some_and(|v| cookie(data, v, seen, depth - 1)))
        }
        OpCode::CPUI_INT_XOR => match (input(0), input(1)) {
            (Some(a), Some(b)) => {
                (global(data, a, DEPTH) && stack(data, b, DEPTH)) || (global(data, b, DEPTH) && stack(data, a, DEPTH))
            }
            _ => false,
        },
        _ => false,
    }
}

/// Is `vn` a value read from a fixed address of the default data space?
fn global(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    let ram = data.get_arch().manage().get_default_data_space().map(|s| s.get_index());
    if !v.is_constant() && Some(v.get_space().get_index()) == ram {
        return true;
    }
    let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
    match op.code() {
        _ if depth == 0 => false,
        OpCode::CPUI_COPY => op.get_in(0).is_some_and(|i| global(data, i, depth - 1)),
        OpCode::CPUI_LOAD => {
            op.get_in(1).and_then(|p| data.vbank().get(p)).is_some_and(|p| p.is_constant())
        }
        _ => false,
    }
}

/// Is `vn` the stack pointer, or an offset from it?
fn stack(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_spacebase() {
        return true;
    }
    let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
    let constant = |k| op.get_in(k).and_then(|i| data.vbank().get(i)).is_some_and(|i| i.is_constant());
    match op.code() {
        _ if depth == 0 => false,
        OpCode::CPUI_COPY => op.get_in(0).is_some_and(|i| stack(data, i, depth - 1)),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => {
            constant(1) && op.get_in(0).is_some_and(|i| stack(data, i, depth - 1))
        }
        _ => false,
    }
}
