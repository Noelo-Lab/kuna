//! Which of a probed body's direct calls it sets up with a stack argument.
//!
//! The convention passes an argument on the stack only once its argument
//! registers are full, so a call whose first stack-argument slot the body
//! stores into hands its target every argument register.
//! [`super::reads_through_calls`] reads that as the target taking the whole
//! register file, which its body read alone is not: the same run is what a
//! variadic register save looks like. The walk follows the stack pointer from
//! the start of the path and records each store it can place against it; at a
//! direct call, the slot the default model passes its first stack parameter in
//! says whether the call has one. The stores are told apart on the terms the
//! decompiled function's own call is (`kuna_passthrough`'s
//! `stores_first_stack_arg`): a constant or a pushed value is an argument, a
//! register stored as it stands (`mov %rbx,(%rsp)`, a spill) is not, and
//! neither is a pushed register the path never wrote that is no argument
//! register (gcc's `push %rbx`, clang's aligning `push %rax`).

use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::byteset::ByteSet;

/// Where a call's first stack argument lies, for the walk to look.
#[derive(Clone, Debug)]
pub(super) struct FirstStackArg {
    /// The stack-pointer register, `(offset, size)` in the register space.
    pub(super) sp: (u64, u32),
    /// The slot's offset above the stack pointer as the CALL op sees it, after
    /// the call instruction's own return-address push: the model's first
    /// stack parameter, as the callee's entry sees it.
    pub(super) offset: i64,
    /// The bytes of the model's general-purpose argument registers.
    pub(super) args: ByteSet,
}

/// The stack slots a path has stored into, as `(offset from the current stack
/// pointer, width, whether the store passes an argument)`.
#[derive(Clone, Debug, Default)]
pub(super) struct Stored {
    slots: Vec<(i64, i64, bool)>,
}

/// Per-instruction state for [`Stored::track`], by temporary offset: the stack
/// offset each temporary computed from the stack pointer holds, and the
/// temporaries holding a copy of a register that is a save or a pad.
#[derive(Default)]
pub(super) struct Temps {
    at: Vec<(u64, i64)>,
    saves: Vec<u64>,
}

impl Temps {
    fn at(&self, t: u64) -> Option<i64> {
        self.at.iter().find(|(o, _)| *o == t).map(|(_, d)| *d)
    }

    fn set(&mut self, t: u64, at: Option<i64>, save: bool) {
        self.at.retain(|(o, _)| *o != t);
        self.saves.retain(|o| *o != t);
        if let Some(d) = at {
            self.at.push((t, d));
        }
        if save {
            self.saves.push(t);
        }
    }
}

impl Stored {
    /// Follow one op. `written` is the set the instruction's reads are scored
    /// against.
    pub(super) fn track(&mut self, op: &super::RawOp, first: &FirstStackArg, reg_idx: int4, written: &ByteSet, temps: &mut Temps) {
        let (opc, out, ins) = (op.opc, op.out.as_ref(), op.ins.as_slice());
        let is_sp = |v: &VarnodeData| {
            v.offset == first.sp.0 && v.size == first.sp.1 && v.space.as_ref().is_some_and(|s| s.get_index() == reg_idx)
        };
        let unique = |v: &VarnodeData| v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL);
        let in_regs = |v: &VarnodeData| v.space.as_ref().is_some_and(|s| s.get_index() == reg_idx);
        let constant = |v: &VarnodeData| {
            v.space.as_ref().filter(|s| s.get_type() == spacetype::IPTR_CONSTANT).map(|_| {
                let bits = 64 - 8 * v.size.min(8);
                ((v.offset << bits) as i64) >> bits
            })
        };
        let at = |v: &VarnodeData| if is_sp(v) { Some(0) } else if unique(v) { temps.at(v.offset) } else { None };
        let displaced = || match (opc, ins) {
            (OpCode::CPUI_COPY, [a]) => at(a),
            (OpCode::CPUI_INT_ADD, [a, b]) => at(a).zip(constant(b)).or_else(|| at(b).zip(constant(a))).map(|(d, c)| d + c),
            (OpCode::CPUI_INT_SUB, [a, b]) => at(a).zip(constant(b)).map(|(d, c)| d - c),
            _ => None,
        };
        if opc == OpCode::CPUI_STORE {
            let (Some(off), Some(value)) = (ins.get(1).and_then(at), ins.get(2)) else { return };
            let size = value.size as i64;
            let passes = !in_regs(value) && !(unique(value) && temps.saves.contains(&value.offset));
            self.slots.retain(|(o, w, _)| !(*o < off + size && off < *o + *w));
            self.slots.push((off, size, passes));
            return;
        }
        let Some(o) = out else { return };
        if is_sp(o) {
            match displaced() {
                Some(d) => {
                    for slot in &mut self.slots {
                        slot.0 -= d;
                    }
                    self.slots.retain(|(off, _, _)| *off >= 0);
                }
                None => self.slots.clear(),
            }
        } else if unique(o) {
            let save = match (opc, ins) {
                (OpCode::CPUI_COPY, [src]) if in_regs(src) => (src.offset..src.offset + src.size as u64)
                    .map(|b| (reg_idx, b))
                    .any(|b| !written.contains(&b) && !first.args.contains(&b)),
                (OpCode::CPUI_COPY, [src]) if unique(src) => temps.saves.contains(&src.offset),
                _ => false,
            };
            temps.set(o.offset, displaced(), save);
        }
    }

    /// Does the path hand the call it is at a stack argument, a store that
    /// passes one at the first stack-argument slot?
    pub(super) fn passes_stack_argument(&self, first: &FirstStackArg) -> bool {
        self.slots.iter().any(|(off, _, passes)| *off == first.offset && *passes)
    }
}
