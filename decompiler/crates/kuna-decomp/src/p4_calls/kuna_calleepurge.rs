//! (kuna) Read an x86-32 callee's stack purge off its own returns.
//!
//! Under a model whose extrapop is `unknown` (`x86win.cspec`'s `__stdcall`,
//! `__thiscall`, `__fastcall`), `StackSolver` has only a guess for a call that
//! no later merge pins, and the guess for an internal callee is "pops nothing".
//! A `__stdcall` or `__thiscall` callee pops its arguments, so every stack
//! offset after such a call is displaced by their size: stack arguments of
//! later calls are dropped and the caller's locals alias outgoing-argument
//! slots. Ghidra's analysis records each function's purge from its `RET n`;
//! this does the same for a direct call's target, at flow time.
//!
//! The callee's p-code is walked from its entry through fall-through and
//! direct branches, so an incremental-link `jmp` thunk reaches the body. The
//! extrapop of a returning instruction is the constant it adds to the stack
//! pointer (`RET 0x14` is `ESP += 4; ESP += 0x14; RETURN`). Every return the
//! walk reaches must agree; a walk that finds none, or two that disagree (a
//! fall-through past a call that does not return, into the next function),
//! yields nothing and leaves the guess in place.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::architecture::Architecture;
use crate::fspec::EXTRAPOP_UNKNOWN;

/// Instructions decoded per callee before the walk stops.
const BUDGET: usize = 2048;

/// The largest extrapop accepted.
const MAX_EXTRAPOP: int4 = 0x404;

/// The extrapop (`4 + purge`) of the direct call target `entry`, when the
/// language is x86-32 under a default model that leaves extrapop unknown and
/// the callee's returns state it.
pub fn callee_extra_pop(arch: &Architecture, entry: &Address) -> Option<int4> {
    if !arch.archid.starts_with("x86:LE:32:") || entry.is_invalid() {
        return None;
    }
    if arch.default_fp().map(|m| m.get_extra_pop()) != Some(EXTRAPOP_UNKNOWN) {
        return None;
    }
    let off = entry.get_offset();
    if let Some(hit) = arch.kuna_callee_purge.borrow().get(&off) {
        return *hit;
    }
    let found = walk(arch, entry);
    arch.kuna_callee_purge.borrow_mut().insert(off, found);
    found
}

#[derive(Default)]
struct Collect(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);

impl PcodeEmit for Collect {
    fn dump(&mut self, _at: &Address, opcode: OpCode, out: Option<&VarnodeData>, inputs: &[VarnodeData]) {
        self.0.push((opcode, out.cloned(), inputs.to_vec()));
    }
}

/// What one instruction's p-code does to the walk.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Effect {
    /// The extrapop of a returning instruction.
    pub returns: Option<int4>,
    /// Direct branch targets.
    pub targets: Vec<uintb>,
    /// Whether execution can continue at the next instruction.
    pub falls_through: bool,
}

fn same(a: &VarnodeData, b: &VarnodeData) -> bool {
    a.offset == b.offset
        && a.size == b.size
        && match (&a.space, &b.space) {
            (Some(x), Some(y)) => x.get_index() == y.get_index(),
            _ => false,
        }
}

fn is_const(v: &VarnodeData) -> bool {
    v.space.as_ref().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT)
}

/// Summarize one instruction's p-code, with `sp` the stack pointer register.
pub(crate) fn effect(ops: &[(OpCode, Option<VarnodeData>, Vec<VarnodeData>)], sp: &VarnodeData) -> Effect {
    let mut fx = Effect { falls_through: true, ..Effect::default() };
    let mut sp_delta: i64 = 0;
    for (opc, out, ins) in ops {
        match opc {
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB
                if out.as_ref().is_some_and(|o| same(o, sp))
                    && ins.len() == 2
                    && same(&ins[0], sp)
                    && is_const(&ins[1]) =>
            {
                let c = ins[1].offset as u32 as i32 as i64;
                sp_delta += if *opc == OpCode::CPUI_INT_ADD { c } else { -c };
            }
            OpCode::CPUI_RETURN => {
                fx.returns = Some(sp_delta as int4);
                fx.falls_through = false;
            }
            OpCode::CPUI_BRANCH if ins.first().is_some_and(|d| !is_const(d)) => {
                fx.targets.push(ins[0].offset);
                fx.falls_through = false;
            }
            OpCode::CPUI_CBRANCH if ins.first().is_some_and(|d| !is_const(d)) => {
                fx.targets.push(ins[0].offset);
            }
            OpCode::CPUI_BRANCHIND => fx.falls_through = false,
            _ => {}
        }
    }
    fx
}

fn stack_pointer(arch: &Architecture) -> Option<VarnodeData> {
    let base = arch.manage().get_stack_space()?.get_spacebase(0).ok()?;
    Some(VarnodeData { space: base.space.clone(), offset: base.offset, size: base.size as u32 })
}

/// The extrapop every reachable return of the function at `entry` agrees on.
fn walk(arch: &Architecture, entry: &Address) -> Option<int4> {
    let space: Rc<AddrSpace> = entry.get_space()?.clone();
    let sp = stack_pointer(arch)?;
    let translate = arch.translate();
    let mut work: Vec<uintb> = vec![entry.get_offset()];
    let mut seen = std::collections::HashSet::new();
    let mut extrapop: Option<int4> = None;
    while let Some(at) = work.pop() {
        if !seen.insert(at) {
            continue;
        }
        if seen.len() > BUDGET {
            break;
        }
        let addr = Address::new(space.clone(), at);
        let mut collect = Collect::default();
        let Ok(len) = translate.one_instruction(&mut collect, &addr) else { continue };
        let fx = effect(&collect.0, &sp);
        if let Some(ep) = fx.returns {
            if !(4..=MAX_EXTRAPOP).contains(&ep) || ep % 4 != 0 || extrapop.is_some_and(|q| q != ep) {
                return None;
            }
            extrapop = Some(ep);
        }
        if fx.falls_through {
            work.push(at.wrapping_add(len.max(1) as uintb));
        }
        work.extend(fx.targets);
    }
    extrapop
}

#[cfg(test)]
mod tests;
