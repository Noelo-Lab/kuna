//! (kuna) The frame cookie MSVC pushes in an SEH prologue is no argument.
//!
//! An x86-32 MSVC function with an EH frame pushes a cookie below its saved
//! registers (`mov eax,[___security_cookie]; xor eax,esp; push eax`) and links
//! the registration record above it into `fs:[0]`. The slot belongs to the
//! frame: only the epilogue pops it. A call below it whose extent is not
//! stated was otherwise handed the slot, and the frame slots above it, as
//! trailing stack arguments.
//!
//! The trials are scored as before, so the slot still anchors the pushes below
//! it and is still claimed by the call that took it. Once every call's list is
//! final, the lowest argument that holds the cookie and every stack argument
//! above it leave the call: a call only ever loses frame slots. The argument
//! must hold the cookie on every path, stored into its slot directly (a value
//! copied in from another stack slot is a pushed argument, whatever it was
//! read as) by the prologue: in the block that links the registration record
//! into `fs:[0]`, before the link. A value of the same shape pushed in the
//! body is an argument. The callee's own code must neither read nor pop the
//! slot.

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use kuna_base::space::spacetype;

/// Ops [`holds_cookie`] follows back from the trial's value.
const DEPTH: u32 = 32;

/// Once no live call scores trials, drop the frame cookie and the stack
/// arguments above it from every call. Returns how many calls changed.
pub fn shed_frame_cookies(data: &mut Funcdata) -> int4 {
    let mut seh = None;
    let mut pending = Vec::new();
    for idx in 0..data.num_calls() {
        let fc = data.get_call_specs(idx);
        let Some(op) = data.obank().get(fc.get_op()).filter(|o| !o.is_dead()) else { continue };
        if !matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            continue;
        }
        if fc.is_input_active() {
            return 0;
        }
        if let Some(cut) = cookie_slot(fc, data, &mut seh) {
            pending.push((idx, cut));
        }
    }
    for &(idx, cut) in &pending {
        let op = data.get_call_specs(idx).get_op();
        let mut storage = data.get_call_specs(idx).final_input_storage().to_vec();
        for k in (0..storage.len()).rev() {
            let (addr, _) = &storage[k];
            if on_stack(addr) && addr.get_offset() as i64 >= cut {
                data.op_remove_input(op, k as int4 + 1);
                storage.remove(k);
            }
        }
        data.get_call_specs_mut(idx).set_final_input_storage(storage);
    }
    pending.len() as int4
}

/// The callee-relative offset of the lowest stack argument of `fc` that holds
/// the frame cookie, when the callee neither reads nor pops it.
fn cookie_slot(fc: &FuncCallSpecs, data: &Funcdata, seh: &mut Option<Vec<OpId>>) -> Option<i64> {
    let op = data.obank().get(fc.get_op())?;
    let storage = fc.final_input_storage();
    if storage.len() as int4 + 1 != op.num_input() {
        return None;
    }
    let read_end = fc.callee_stack_arg_bytes().map_or(0, |a| 4 + a.read as i64);
    let popped = crate::p4_calls::kuna_calleepurge::PopEvidence::of_call(fc).extra_pop().map_or(0, i64::from);
    let mut cut: Option<i64> = None;
    for (k, (addr, size)) in storage.iter().enumerate() {
        let start = addr.get_offset() as i64;
        let end = start + *size as i64;
        if !on_stack(addr) || end <= read_end || end <= popped || cut.is_some_and(|c| c <= start) {
            continue;
        }
        let Some(vn) = op.get_in(k as int4 + 1) else { continue };
        if holds_cookie(data, vn, seh) {
            cut = Some(start);
        }
    }
    cut
}

fn on_stack(addr: &kuna_base::address::Address) -> bool {
    addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
}

/// Does the stack value `vn` reaching a call hold the frame cookie on every
/// path: a value of the default data space XORed with the stack pointer,
/// stored into this slot before an SEH link of `links` and kept there through
/// calls and joins?
fn holds_cookie(data: &Funcdata, vn: VarnodeId, links: &mut Option<Vec<OpId>>) -> bool {
    let mut seen = Vec::new();
    cookie(data, vn, &mut seen, links, DEPTH)
}

fn cookie(
    data: &Funcdata,
    vn: VarnodeId,
    seen: &mut Vec<VarnodeId>,
    links: &mut Option<Vec<OpId>>,
    depth: u32,
) -> bool {
    if seen.contains(&vn) {
        return true;
    }
    seen.push(vn);
    let Some(op) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
        return false;
    };
    let input = |k| op.get_in(k);
    let in_frame =
        |v: VarnodeId| data.vbank().get(v).is_some_and(|v| v.get_space().get_type() == spacetype::IPTR_SPACEBASE);
    let joins = matches!(op.code(), OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL);
    let shaped = match op.code() {
        _ if depth == 0 => false,
        OpCode::CPUI_COPY => input(0).is_some_and(|v| !in_frame(v) && cookie(data, v, seen, links, depth - 1)),
        OpCode::CPUI_INDIRECT => input(0).is_some_and(|v| cookie(data, v, seen, links, depth - 1)),
        OpCode::CPUI_MULTIEQUAL => {
            op.num_input() > 0
                && (0..op.num_input()).all(|k| input(k).is_some_and(|v| cookie(data, v, seen, links, depth - 1)))
        }
        OpCode::CPUI_INT_XOR => match (input(0), input(1)) {
            (Some(a), Some(b)) => {
                (global(data, a, DEPTH) && stack(data, b, DEPTH)) || (global(data, b, DEPTH) && stack(data, a, DEPTH))
            }
            _ => false,
        },
        _ => false,
    };
    shaped && (joins || !in_frame(vn) || before_link(data, op, links))
}

/// Is `op` in the block of an SEH link of `links`, and before it?
fn before_link(data: &Funcdata, op: &crate::op::PcodeOp, links: &mut Option<Vec<OpId>>) -> bool {
    let links = links.get_or_insert_with(|| seh_links(data));
    links.iter().filter_map(|&l| data.obank().get(l)).any(|l| {
        op.get_parent().is_some()
            && l.get_parent() == op.get_parent()
            && op.get_seq_num().get_order() < l.get_seq_num().get_order()
    })
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

/// The stores that link an SEH registration record: a stack address stored to
/// `fs:[0]`.
fn seh_links(data: &Funcdata) -> Vec<OpId> {
    let Some(fs) = data.get_arch().manage().register_lookup().and_then(|l| l.probe_register("FS_OFFSET")) else {
        return Vec::new();
    };
    let Some(space) = fs.space else { return Vec::new() };
    let at_base = |mut vn: VarnodeId| {
        for _ in 0..4 {
            let Some(v) = data.vbank().get(vn) else { return false };
            if v.get_space().get_index() == space.get_index() && v.get_offset() == fs.offset {
                return true;
            }
            let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
            let zero = op.get_in(1).and_then(|i| data.vbank().get(i)).is_some_and(|i| i.is_constant() && i.get_offset() == 0);
            match (op.code(), op.get_in(0)) {
                (OpCode::CPUI_COPY, Some(i)) => vn = i,
                (OpCode::CPUI_INT_ADD, Some(i)) if zero => vn = i,
                _ => return false,
            }
        }
        false
    };
    data.obank()
        .iter_code(OpCode::CPUI_STORE)
        .filter(|&o| {
            data.obank().get(o).is_some_and(|o| {
                !o.is_dead() && o.get_in(1).is_some_and(at_base) && o.get_in(2).is_some_and(|v| stack(data, v, DEPTH))
            })
        })
        .collect()
}
