//! (kuna `calleeargs`) Measure how many stack-argument bytes an x86-32 callee
//! reads.
//!
//! An argument pushed for an outer call before an inner call sits on the stack
//! across the inner call, and nothing at the inner call site tells the two
//! apart: a `__cdecl` callee also returns with `ret`, and MSVC may defer its
//! cleanup past the next call. The callee does tell: it never touches the stack
//! above its last argument.
//!
//! The callee's p-code is walked from its entry along every path, following
//! direct branches, tracking the stack pointer relative to its value at entry
//! (where the return address sits; arguments start at +4) and the frame
//! pointer a `mov ebp,esp` sets. A direct call inside the callee moves the
//! stack pointer by that callee's own purge
//! ([`crate::p4_calls::kuna_calleepurge`]). The extent is the highest byte at
//! +4 or above that a load or store touches, rounded up to a 4-byte slot, less
//! the return address. It is stated only when every path ends in a return and
//! no access went untracked; an argument address handed on (a `va_list`, a
//! `lea` of a parameter), an indirect jump, a call whose purge is unknown
//! followed by a stack access, an aligned stack or a budget overrun makes no
//! claim.
//!
//! At the call site (`check_input_trial_use`), a stack trial past the extent
//! (or past what the callee pops, if that is more) is no argument, and one
//! within it skips the flow-insensitive local-alias test: the callee reads it.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::architecture::Architecture;
use crate::fspec::EXTRAPOP_UNKNOWN;

/// `option calleeargs on|off` (default on).
pub struct OptionCalleeArgs;

impl OptionCalleeArgs {
    /// The option name.
    pub const NAME: &'static str = "calleeargs";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::kuna_calleeargs`.
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(bool, String)> {
        let val = crate::p0_knowledge::options::on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee stack-argument extent turned {prop}")))
    }
}

/// Instructions decoded per callee before the walk gives up.
const BUDGET: usize = 2048;

/// The stack-argument bytes the direct call target `entry` reads, when the
/// language is x86-32 under a default model that leaves extrapop unknown and
/// the walk can account for every access.
pub fn callee_stack_arg_bytes(arch: &Architecture, entry: &Address) -> Option<u32> {
    if !arch.archid.starts_with("x86:LE:32:") || entry.is_invalid() {
        return None;
    }
    if arch.default_fp().map(|m| m.get_extra_pop()) != Some(EXTRAPOP_UNKNOWN) {
        return None;
    }
    let off = entry.get_offset();
    if let Some(hit) = arch.kuna_callee_stackargs.borrow().get(&off) {
        return *hit;
    }
    let found = walk(arch, entry);
    arch.kuna_callee_stackargs.borrow_mut().insert(off, found);
    found
}

#[derive(Default)]
struct Collect(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);

impl PcodeEmit for Collect {
    fn dump(&mut self, _at: &Address, opcode: OpCode, out: Option<&VarnodeData>, inputs: &[VarnodeData]) {
        self.0.push((opcode, out.cloned(), inputs.to_vec()));
    }
}

/// The stack pointer and frame pointer, as entry-relative byte offsets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Frame {
    pub sp: Option<i64>,
    pub bp: Option<i64>,
}

/// What one instruction does to the walk.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Step {
    pub frame: Frame,
    /// The end (entry-relative) of the highest argument slot touched, 4 if none.
    pub arg_end: i64,
    /// An argument slot's address was handed on.
    pub escaped: bool,
    /// A stack access could not be placed.
    pub untracked: bool,
    pub targets: Vec<uintb>,
    pub falls_through: bool,
    pub returns: bool,
    /// The direct call target whose purge moves the stack pointer after this
    /// instruction.
    pub call: Option<uintb>,
}

/// The registers the walk tracks.
pub(crate) struct Regs {
    pub sp: VarnodeData,
    pub bp: VarnodeData,
}

fn same(a: &VarnodeData, b: &VarnodeData) -> bool {
    a.offset == b.offset
        && a.size == b.size
        && match (&a.space, &b.space) {
            (Some(x), Some(y)) => x.get_index() == y.get_index(),
            _ => false,
        }
}

fn kind(v: &VarnodeData) -> Option<spacetype> {
    v.space.as_ref().map(|s| s.get_type())
}

/// A value the walk can place: an entry-relative stack address, or a stack
/// address it cannot place.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Val {
    Stack(i64),
    Lost,
}

/// Summarize one instruction's p-code from `frame`.
pub(crate) fn step(ops: &[(OpCode, Option<VarnodeData>, Vec<VarnodeData>)], regs: &Regs, frame: Frame) -> Step {
    let mut s = Step { frame, falls_through: true, arg_end: 4, ..Step::default() };
    let mut temps: HashMap<u64, Val> = HashMap::new();
    let eval = |v: &VarnodeData, s: &Step, temps: &HashMap<u64, Val>| -> Option<Val> {
        if same(v, &regs.sp) {
            return Some(s.frame.sp.map_or(Val::Lost, Val::Stack));
        }
        if same(v, &regs.bp) {
            return s.frame.bp.map(Val::Stack);
        }
        if kind(v) == Some(spacetype::IPTR_INTERNAL) {
            return temps.get(&v.offset).copied();
        }
        None
    };
    let touch = |val: Option<Val>, size: u32, s: &mut Step| match val {
        Some(Val::Stack(o)) if o + size as i64 > 4 => s.arg_end = s.arg_end.max(o + size as i64),
        Some(Val::Lost) => s.untracked = true,
        _ => {}
    };
    let escape = |val: Option<Val>, s: &mut Step| match val {
        Some(Val::Stack(o)) if o >= 4 => s.escaped = true,
        Some(Val::Lost) => s.untracked = true,
        _ => {}
    };
    for (opc, out, ins) in ops {
        let vals: Vec<Option<Val>> = ins.iter().map(|v| eval(v, &s, &temps)).collect();
        let result: Option<Val> = match opc {
            OpCode::CPUI_COPY => vals.first().copied().flatten(),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if ins.len() == 2 => {
                let c = |i: usize| (kind(&ins[i]) == Some(spacetype::IPTR_CONSTANT)).then(|| ins[i].offset as u32 as i32 as i64);
                match (vals[0], vals[1], c(0), c(1)) {
                    (Some(Val::Stack(o)), _, _, Some(k)) => {
                        Some(Val::Stack(if *opc == OpCode::CPUI_INT_ADD { o + k } else { o - k }))
                    }
                    (_, Some(Val::Stack(o)), Some(k), _) if *opc == OpCode::CPUI_INT_ADD => Some(Val::Stack(o + k)),
                    (Some(Val::Lost), _, _, _) | (_, Some(Val::Lost), _, _) => Some(Val::Lost),
                    (Some(Val::Stack(_)), _, _, _) | (_, Some(Val::Stack(_)), _, _) => Some(Val::Lost),
                    _ => None,
                }
            }
            OpCode::CPUI_LOAD => {
                let size = out.as_ref().map_or(4, |o| o.size);
                touch(vals.get(1).copied().flatten(), size, &mut s);
                None
            }
            OpCode::CPUI_STORE => {
                let size = ins.get(2).map_or(4, |v| v.size);
                touch(vals.get(1).copied().flatten(), size, &mut s);
                escape(vals.get(2).copied().flatten(), &mut s);
                None
            }
            OpCode::CPUI_RETURN => {
                s.returns = true;
                s.falls_through = false;
                None
            }
            OpCode::CPUI_BRANCH if ins.first().is_some_and(|d| kind(d) != Some(spacetype::IPTR_CONSTANT)) => {
                s.targets.push(ins[0].offset);
                s.falls_through = false;
                None
            }
            OpCode::CPUI_CBRANCH if ins.first().is_some_and(|d| kind(d) != Some(spacetype::IPTR_CONSTANT)) => {
                s.targets.push(ins[0].offset);
                None
            }
            OpCode::CPUI_BRANCHIND => {
                s.untracked = true;
                s.falls_through = false;
                None
            }
            OpCode::CPUI_CALL => {
                s.call = ins.first().filter(|d| kind(d) != Some(spacetype::IPTR_CONSTANT)).map(|d| d.offset);
                if s.call.is_none() {
                    s.frame.sp = None;
                }
                None
            }
            OpCode::CPUI_CALLIND => {
                s.frame.sp = None;
                None
            }
            _ => {
                for v in vals.iter().flatten() {
                    escape(Some(*v), &mut s);
                }
                None
            }
        };
        let Some(o) = out else { continue };
        if same(o, &regs.sp) {
            s.frame.sp = match result {
                Some(Val::Stack(v)) => Some(v),
                _ => None,
            };
        } else if same(o, &regs.bp) {
            s.frame.bp = match result {
                Some(Val::Stack(v)) => Some(v),
                _ => None,
            };
        } else if kind(o) == Some(spacetype::IPTR_INTERNAL) {
            match result {
                Some(v) => {
                    temps.insert(o.offset, v);
                }
                None => {
                    temps.remove(&o.offset);
                }
            }
        } else if let Some(v) = result {
            escape(Some(v), &mut s);
        }
    }
    s
}

fn registers(arch: &Architecture) -> Option<Regs> {
    let base = arch.manage().get_stack_space()?.get_spacebase(0).ok()?;
    let sp = VarnodeData { space: base.space.clone(), offset: base.offset, size: base.size as u32 };
    let bp = arch.translate().get_register_data("EBP").ok()?;
    Some(Regs { sp, bp })
}

/// The stack-argument bytes every path of the callee at `entry` stays within,
/// or `None` when the walk cannot tell.
fn walk(arch: &Architecture, entry: &Address) -> Option<u32> {
    let space: Rc<AddrSpace> = entry.get_space()?.clone();
    let regs = registers(arch)?;
    let translate = arch.translate();
    let mut work: Vec<(uintb, Frame)> = vec![(entry.get_offset(), Frame { sp: Some(0), bp: None })];
    let mut seen: HashMap<uintb, Frame> = HashMap::new();
    let mut returned = false;
    let mut end: i64 = 4;
    while let Some((at, frame)) = work.pop() {
        match seen.get(&at) {
            Some(f) if *f == frame => continue,
            Some(_) => return None,
            None => {}
        }
        seen.insert(at, frame);
        if seen.len() > BUDGET {
            return None;
        }
        let addr = Address::new(space.clone(), at);
        let mut collect = Collect::default();
        let len = translate.one_instruction(&mut collect, &addr).ok()?;
        let mut s = step(&collect.0, &regs, frame);
        if s.escaped || s.untracked {
            return None;
        }
        end = end.max(s.arg_end);
        if let Some(target) = s.call {
            let ep = crate::p4_calls::kuna_calleepurge::callee_extra_pop(arch, &Address::new(space.clone(), target));
            s.frame.sp = match (s.frame.sp, ep) {
                (Some(sp), Some(ep)) => Some(sp + ep as i64),
                _ => None,
            };
        }
        returned |= s.returns;
        if s.falls_through {
            work.push((at.wrapping_add(len.max(1) as uintb), s.frame));
        }
        work.extend(s.targets.iter().map(|&t| (t, s.frame)));
    }
    if !returned {
        return None;
    }
    u32::try_from(((end + 3) & !3) - 4).ok()
}

#[cfg(test)]
mod tests;
