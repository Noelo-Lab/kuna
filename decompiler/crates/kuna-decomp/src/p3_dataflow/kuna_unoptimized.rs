//! (kuna) Was this function compiled without optimization?
//!
//! An unoptimizing compiler gives every parameter a home in its frame: the
//! prologue stores each incoming argument register to a slot of the function's
//! own frame and the body loads the value back from there whenever it needs
//! it, and no value is kept in a callee-saved register. An optimizing one
//! computes with the argument registers directly and keeps values live across
//! calls in callee-saved registers. [`detect`] decides once per decompile,
//! right after the first heritage pass, when the registers are in SSA form but
//! the stack is not yet: a spill is still the raw `STORE` of an incoming
//! register to the entry stack pointer plus a constant, and a reload the raw
//! `LOAD` from the same offset.
//!
//! The verdict reads no symbol and no type; a function with no parameter
//! evidence and no frame pointer is not called unoptimized.

use std::collections::HashMap;

use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::varnode::Varnode;

/// Longest chain of copies and constant offsets followed back to the stack pointer.
const WALK_DEPTH: u32 = 12;

/// What the first heritage saw of a function's parameter registers and frame.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Evidence {
    /// Parameter registers whose incoming value is only stored to a frame slot
    /// in the entry block, a slot the body loads into another register.
    pub spilled: u32,
    /// Parameter registers stored to the frame but never loaded back.
    pub stored: u32,
    /// Parameter registers the body computes with directly.
    pub live: u32,
    /// Frame accesses addressed through a callee-saved register that holds a
    /// frame address (a frame pointer).
    pub frame_pointer: u32,
    /// Callee-saved registers whose incoming value is saved to the frame and
    /// that the body then reuses for values of its own.
    pub callee_saved: u32,
}

impl Evidence {
    /// The verdict. A parameter computed with in its register means optimized
    /// code; one that lives in the frame means unoptimized code, unless more
    /// than one callee-saved register carries values (an unoptimizing compiler
    /// uses at most one, as a temporary). A function with no parameter evidence
    /// is unoptimized only when it addresses its frame through a frame pointer
    /// and reuses no callee-saved register.
    pub fn unoptimized(&self) -> bool {
        if self.live > 0 {
            return false;
        }
        if self.spilled > 0 {
            return self.callee_saved <= 1;
        }
        self.stored == 0 && self.frame_pointer > 0 && self.callee_saved == 0
    }
}

/// Compute and cache the verdict once, right after the first heritage pass.
pub(crate) fn detect(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || fd.kuna_unoptimized.is_some() {
        return;
    }
    let ev = evidence(fd);
    fd.kuna_unoptimized = Some(ev.is_some_and(|e| e.unoptimized()));
    fd.kuna_frame_pointer = ev.is_some_and(|e| e.frame_pointer > 0);
}

/// The cached verdict; `false` before the first heritage pass.
pub fn is_unoptimized(fd: &Funcdata) -> bool {
    fd.kuna_unoptimized == Some(true)
}

/// Whether the first heritage pass saw the function address its frame through
/// a frame pointer, which optimizing compilers rarely keep; `false` before it.
pub fn frame_pointer(fd: &Funcdata) -> bool {
    fd.kuna_frame_pointer
}

fn sext(value: u64, size: int4) -> i64 {
    if !(1..8).contains(&size) {
        return value as i64;
    }
    let shift = 64 - 8 * size as u32;
    ((value << shift) as i64) >> shift
}

/// The value of a constant, or of a temporary copied from one (`sub sp,#8`
/// lowers to a copy of 8 and a subtraction of that copy).
fn constant_value(fd: &Funcdata, vn: VarnodeId) -> Option<i64> {
    let v = fd.vbank().get(vn)?;
    if v.is_constant() {
        return Some(sext(v.get_offset(), v.get_size()));
    }
    let def = fd.obank().get(v.get_def()?)?;
    if def.code() != OpCode::CPUI_COPY {
        return None;
    }
    let c = fd.vbank().get(def.get_in(0)?)?;
    c.is_constant().then(|| sext(c.get_offset(), c.get_size()))
}

/// Is `v` a register the calling convention preserves across calls?
fn callee_saved(fd: &Funcdata, v: &Varnode) -> bool {
    v.get_space().get_type() == spacetype::IPTR_PROCESSOR
        && fd.get_func_proto().has_effect(&v.get_addr(), v.get_size())
            == crate::fspec::effect_type::UNAFFECTED
}

/// Frame offsets of values, followed back to the entry stack pointer.
struct Frame<'a> {
    fd: &'a Funcdata,
    sp: VarnodeId,
    sp_offset: u64,
    memo: HashMap<VarnodeId, Option<(i64, bool)>>,
}

impl Frame<'_> {
    /// The entry-stack-pointer offset `vn` holds, and whether the chain passes
    /// through a callee-saved register other than the stack pointer.
    fn offset(&mut self, vn: VarnodeId) -> Option<(i64, bool)> {
        self.walk(vn, WALK_DEPTH)
    }

    fn walk(&mut self, vn: VarnodeId, depth: u32) -> Option<(i64, bool)> {
        if vn == self.sp {
            return Some((0, false));
        }
        if let Some(hit) = self.memo.get(&vn) {
            return *hit;
        }
        let res = if depth == 0 { None } else { self.step(vn, depth) };
        self.memo.insert(vn, res);
        res
    }

    fn step(&mut self, vn: VarnodeId, depth: u32) -> Option<(i64, bool)> {
        let fd = self.fd;
        let v = fd.vbank().get(vn)?;
        let via = v.get_offset() != self.sp_offset && callee_saved(fd, v);
        let op = fd.obank().get(v.get_def()?)?;
        let constant = |i: int4| op.get_in(i).and_then(|c| constant_value(fd, c));
        let (base, delta) = match op.code() {
            OpCode::CPUI_COPY => (op.get_in(0)?, 0),
            OpCode::CPUI_INT_ADD => match (constant(0), constant(1)) {
                (None, Some(c)) => (op.get_in(0)?, c),
                (Some(c), None) => (op.get_in(1)?, c),
                _ => return None,
            },
            OpCode::CPUI_INT_SUB => (op.get_in(0)?, constant(1)?.wrapping_neg()),
            _ => return None,
        };
        let (off, fp) = self.walk(base, depth - 1)?;
        Some((off.wrapping_add(delta), fp || via))
    }
}

fn is_move(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT
    )
}

/// The value family of an incoming register: itself and what copies, pieces
/// and extensions of it compute.
fn family(fd: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let mut out = vec![vn];
    let mut i = 0;
    while i < out.len() && out.len() < 32 {
        let cur = out[i];
        i += 1;
        let Some(v) = fd.vbank().get(cur) else { continue };
        for op in v.descend_iter() {
            let Some(o) = fd.obank().get(op) else { continue };
            if is_move(o.code()) && o.get_in(0) == Some(cur) {
                if let Some(next) = o.get_out().filter(|n| !out.contains(n)) {
                    out.push(next);
                }
            }
        }
    }
    out
}

/// How an incoming register is used.
enum Use {
    /// Computed with: an operand of anything but a move, a call argument, a
    /// merge, a piece of a partial write or a frame store.
    Live,
    /// Only stored, to these slots of the function's own frame in the entry
    /// block.
    Spilled(Vec<i64>),
    /// Neither.
    Idle,
}

fn classify(fd: &Funcdata, frame: &mut Frame, vn: VarnodeId, entry: Option<BlockId>, own: &dyn Fn(i64) -> bool) -> Use {
    let mut spills = Vec::new();
    let members = family(fd, vn);
    for &m in &members {
        let Some(mv) = fd.vbank().get(m) else { continue };
        for op in mv.descend_iter() {
            let Some(o) = fd.obank().get(op) else { continue };
            match o.code() {
                c if is_move(c) && o.get_out().is_some_and(|x| members.contains(&x)) => {}
                OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_PIECE | OpCode::CPUI_RETURN => {}
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if o.get_in(0) != Some(m) => {}
                OpCode::CPUI_STORE if o.get_in(2) == Some(m) && o.get_in(1) != Some(m) => {
                    match o.get_in(1).and_then(|a| frame.offset(a)) {
                        Some((off, _)) if !own(off) => {}
                        Some((off, _)) if o.get_parent() == entry => spills.push(off),
                        _ => return Use::Live,
                    }
                }
                _ => return Use::Live,
            }
        }
    }
    if spills.is_empty() {
        Use::Idle
    } else {
        Use::Spilled(spills)
    }
}

/// Is the incoming value of `vn` stored to the frame anywhere?
fn saved_to_frame(fd: &Funcdata, frame: &mut Frame, vn: VarnodeId) -> bool {
    for m in family(fd, vn) {
        let Some(mv) = fd.vbank().get(m) else { continue };
        let stores: Vec<OpId> = mv.descend_iter().collect();
        for op in stores {
            let Some(o) = fd.obank().get(op) else { continue };
            if o.code() == OpCode::CPUI_STORE
                && o.get_in(2) == Some(m)
                && o.get_in(1).is_some_and(|a| frame.offset(a).is_some())
            {
                return true;
            }
        }
    }
    false
}

/// Does `op` restore a register from the frame (a load, or a copy of one)?
fn restores(fd: &Funcdata, op: &crate::op::PcodeOp) -> bool {
    match op.code() {
        OpCode::CPUI_LOAD => true,
        OpCode::CPUI_COPY => op
            .get_in(0)
            .and_then(|i| fd.vbank().get(i))
            .and_then(|i| i.get_def())
            .and_then(|d| fd.obank().get(d))
            .is_some_and(|d| d.code() == OpCode::CPUI_LOAD),
        _ => false,
    }
}

/// Collect the [`Evidence`] for `fd`; `None` without a stack pointer input.
pub fn evidence(fd: &Funcdata) -> Option<Evidence> {
    let stack = fd.get_arch().manage().get_stack_space()?.clone();
    let sp = fd.find_spacebase_input(&stack)?;
    let sp_offset = fd.vbank().get(sp)?.get_offset();
    let mut frame = Frame { fd, sp, sp_offset, memo: HashMap::new() };
    let entry = fd.bblocks_ref().get_start_block(fd.bblocks_root_pub()).ok();
    // A slot in the function's own frame; the caller's side (a register
    // parameter's home or save area) is used by optimized code too.
    let grows_down = stack.stack_grows_negative();
    let own = move |off: i64| if grows_down { off < 0 } else { off > 0 };
    let mut ev = Evidence::default();
    // Frame loads as (instruction, frame offset, register filled).
    let mut loads: Vec<(u64, i64, Option<u64>)> = Vec::new();
    // Callee-saved registers written with a value of the body's own.
    let mut writes: Vec<(u64, u64)> = Vec::new();
    let mut calls: Vec<u64> = Vec::new();
    let ops: Vec<OpId> = fd.obank().iter_alive().collect();
    for op in ops {
        let Some(o) = fd.obank().get(op) else { continue };
        let at = o.get_addr().get_offset();
        match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => calls.push(at),
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                if let Some((offset, fp)) = o.get_in(1).and_then(|a| frame.offset(a)) {
                    if fp {
                        ev.frame_pointer += 1;
                    }
                    if o.code() == OpCode::CPUI_LOAD {
                        let reg = o
                            .get_out()
                            .and_then(|v| fd.vbank().get(v))
                            .filter(|v| v.get_space().get_type() == spacetype::IPTR_PROCESSOR)
                            .map(|v| v.get_offset());
                        loads.push((at, offset, reg));
                    }
                }
            }
            OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL => continue,
            _ => {}
        }
        let Some(out_id) = o.get_out() else { continue };
        let Some(out) = fd.vbank().get(out_id) else { continue };
        if out.get_offset() == sp_offset || out.get_size() < 4 || !callee_saved(fd, out) {
            continue;
        }
        if !restores(fd, o) && frame.offset(out_id).is_none() {
            writes.push((out.get_offset(), at));
        }
    }
    // A load at a call reads the callee's stack argument, and a register a
    // call writes is its return address; neither is the body's own.
    loads.retain(|(at, _, _)| !calls.contains(at));
    writes.retain(|(_, at)| !calls.contains(at));
    let inputs: Vec<VarnodeId> = fd.vbank().iter_def_flag(crate::varnode::varnode_flags::input).collect();
    let mut saved: Vec<u64> = Vec::new();
    for vn in inputs {
        let Some(v) = fd.vbank().get(vn) else { continue };
        if vn == sp || v.get_space().get_type() != spacetype::IPTR_PROCESSOR {
            continue;
        }
        let reg = v.get_offset();
        if callee_saved(fd, v) && saved_to_frame(fd, &mut frame, vn) {
            saved.push(reg);
        }
        if !fd.get_func_proto().possible_input_param(&v.get_addr(), v.get_size()) {
            continue;
        }
        match classify(fd, &mut frame, vn, entry, &own) {
            Use::Live => ev.live += 1,
            Use::Idle => {}
            Use::Spilled(offsets) => {
                if offsets.iter().any(|&off| loads.iter().any(|&(_, o, r)| o == off && r != Some(reg))) {
                    ev.spilled += 1;
                } else {
                    ev.stored += 1;
                }
            }
        }
    }
    let mut reused: Vec<u64> = writes.into_iter().map(|(r, _)| r).filter(|r| saved.contains(r)).collect();
    reused.sort_unstable();
    reused.dedup();
    ev.callee_saved = reused.len() as u32;
    Some(ev)
}

#[cfg(test)]
mod tests;
