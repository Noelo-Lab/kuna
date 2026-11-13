//! (kuna) Read which registers an x86-32 callee takes as input off its own code.
//!
//! `x86win.cspec`'s default model, `__stdcall`, has no register parameters, so
//! a direct call to a function with no declared prototype only ever recovers
//! stack arguments. A `__thiscall` callee's `this` (ECX), and a `__fastcall`
//! callee's ECX/EDX arguments, are dropped at the call; a caller that reaches
//! the callee through an incremental-link `jmp` stub learns nothing from the
//! stub's own decompile either.
//!
//! The callee's p-code is read from its entry along the straight line its first
//! instructions run, following direct `jmp`s, until a conditional branch, a call,
//! a return, an indirect jump or [`BUDGET`] instructions. A register the line
//! reads before writing it is an input. Some readings are not uses: storing a
//! register through the stack pointer (a `push` that saves it), a copy of a
//! register onto itself (the `mov edi,edi` hot-patch prologue, the `lea ecx,[ecx]`
//! padding nop) and a result the old value cannot change (`xor ecx,ecx`,
//! `sub ecx,ecx`, `or ecx,-1`, `and ecx,0`); a push goes through a temporary,
//! which only counts when something other than the push reads it. A callee that
//! reads ECX alone takes the spec's own `__thiscall` model at the call, and one
//! that reads ECX and EDX takes `__fastcall`. A callee that also reads EAX, EBX,
//! ESI or EDI keeps the default: it takes a convention no spec model states
//! (Borland `register`, an MSVC whole-program custom convention, `_chkstk`) or
//! captures every register (`__report_gsfailure`, `_invoke_watson`). A callee in
//! the import table is never walked. Only with option `calleereginputs` on.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::architecture::Architecture;
use crate::fspec::{ProtoModel, EXTRAPOP_UNKNOWN};

/// Instructions read from the callee's entry.
const BUDGET: usize = 64;

/// The general registers tracked, in the order of the bits of an input set.
const REGISTERS: [&str; 6] = ["EAX", "ECX", "EDX", "EBX", "ESI", "EDI"];

const ECX: u8 = 1 << 1;
const EDX: u8 = 1 << 2;
/// EAX and the callee-saved registers.
const FOREIGN: u8 = 1 | (1 << 3) | (1 << 4) | (1 << 5);

/// The spec model a callee's register inputs select.
pub(crate) fn model_name(inputs: u8) -> Option<&'static str> {
    if inputs & ECX == 0 || inputs & FOREIGN != 0 {
        return None;
    }
    Some(if inputs & EDX != 0 { "__fastcall" } else { "__thiscall" })
}

/// The model the inputs of the direct call target `entry` select, when option
/// `calleereginputs` is on and the language is x86-32 under a default model that
/// leaves extrapop unknown.
pub fn callee_register_model(arch: &Architecture, entry: &Address) -> Option<Rc<ProtoModel>> {
    if !arch.callee_reginputs {
        return None;
    }
    let name = model_name(callee_register_inputs(arch, entry)?)?;
    arch.get_model(name).cloned()
}

fn is_import(arch: &Architecture, entry: &Address) -> bool {
    let Some(scope) = arch.symboltab.get_global_scope() else { return false };
    let size = entry.get_space().map_or(1, |s| s.get_addr_size() as i32);
    let (_, flags) = arch.symboltab.query_properties(scope, entry, size, entry);
    flags & crate::varnode::varnode_flags::externref != 0
}

/// The tracked registers the callee at `entry` reads before writing, as bits
/// in [`REGISTERS`] order.
pub fn callee_register_inputs(arch: &Architecture, entry: &Address) -> Option<u8> {
    if !arch.archid.starts_with("x86:LE:32:") || entry.is_invalid() {
        return None;
    }
    if arch.default_fp().map(|m| m.get_extra_pop()) != Some(EXTRAPOP_UNKNOWN) {
        return None;
    }
    if is_import(arch, entry) {
        return None;
    }
    let off = entry.get_offset();
    if let Some(hit) = arch.kuna_callee_reginputs.borrow().get(&off) {
        return *hit;
    }
    let found = walk(arch, entry);
    arch.kuna_callee_reginputs.borrow_mut().insert(off, found);
    found
}

#[derive(Default)]
struct Collect(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);

impl PcodeEmit for Collect {
    fn dump(&mut self, _at: &Address, opcode: OpCode, out: Option<&VarnodeData>, inputs: &[VarnodeData]) {
        self.0.push((opcode, out.cloned(), inputs.to_vec()));
    }
}

/// The tracked registers and the stack pointer, as storage.
pub(crate) struct Regs {
    pub tracked: Vec<VarnodeData>,
    pub sp: VarnodeData,
}

fn same_space(a: &VarnodeData, b: &VarnodeData) -> bool {
    match (&a.space, &b.space) {
        (Some(x), Some(y)) => x.get_index() == y.get_index(),
        _ => false,
    }
}

/// The bytes of register `r` that `v` touches, bit `i` for byte `i` of `r`.
fn byte_mask(v: &VarnodeData, r: &VarnodeData) -> u8 {
    if !same_space(v, r) {
        return 0;
    }
    let lo = v.offset.max(r.offset);
    let hi = (v.offset + v.size as u64).min(r.offset + r.size as u64);
    (lo..hi).fold(0u8, |m, b| m | (1 << (b - r.offset)))
}

fn same(a: &VarnodeData, b: &VarnodeData) -> bool {
    same_space(a, b) && a.offset == b.offset && a.size == b.size
}

/// Is `v` the constant that fixes the result of `opc` whatever the other input
/// holds (all ones for `INT_OR`, zero for `INT_AND`)?
fn absorbs(opc: OpCode, v: &VarnodeData) -> bool {
    if !v.space.as_ref().is_some_and(|sp| sp.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT) {
        return false;
    }
    let ones = if v.size >= 8 { u64::MAX } else { (1u64 << (8 * v.size)) - 1 };
    match opc {
        OpCode::CPUI_INT_OR => v.offset & ones == ones,
        _ => v.offset & ones == 0,
    }
}

/// What one instruction's p-code does to the line.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Step {
    pub read: u8,
    /// Per tracked register, the bytes written so far (bit `i` = byte `i`).
    pub written: [u8; 6],
    /// The next instruction of the line: `Some(target)` for a direct jump,
    /// `None` when the line ends here; falls through when [`Self::ends`] is false.
    pub jump: Option<uintb>,
    pub ends: bool,
}

/// Summarize one instruction's p-code against the registers already `written`.
pub(crate) fn step(
    ops: &[(OpCode, Option<VarnodeData>, Vec<VarnodeData>)],
    regs: &Regs,
    written: [u8; 6],
) -> Step {
    let mut s = Step { written, ..Step::default() };
    let is_unique = |v: &VarnodeData| {
        v.space.as_ref().is_some_and(|sp| sp.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
    };
    let read_bits = |v: &VarnodeData, done: &[u8; 6]| {
        regs.tracked.iter().enumerate().fold(0u8, |acc, (k, r)| {
            let bytes = byte_mask(v, r);
            if bytes & !done[k] != 0 { acc | (1 << k) } else { acc }
        })
    };
    // A register copied into a temporary that is then only pushed is saved, not used.
    let mut held: Vec<(VarnodeData, u8)> = Vec::new();
    for (opc, out, ins) in ops {
        if *opc == OpCode::CPUI_COPY && ins.len() == 1 && out.as_ref().is_some_and(|o| same(o, &ins[0])) {
            continue;
        }
        match opc {
            OpCode::CPUI_COPY if ins.len() == 1 && out.as_ref().is_some_and(is_unique) => {
                let bits = read_bits(&ins[0], &s.written);
                if bits != 0 {
                    held.push((out.clone().unwrap(), bits));
                }
            }
            OpCode::CPUI_STORE if ins.len() == 3 && same(&ins[1], &regs.sp) => {
                s.read |= read_bits(&ins[1], &s.written);
                if let Some(pos) = held.iter().position(|(t, _)| same(t, &ins[2])) {
                    held.remove(pos);
                }
            }
            OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB if ins.len() == 2 && same(&ins[0], &ins[1]) => {}
            OpCode::CPUI_INT_OR | OpCode::CPUI_INT_AND if ins.len() == 2 && ins.iter().any(|v| absorbs(*opc, v)) => {}
            _ => {
                for v in ins {
                    s.read |= read_bits(v, &s.written);
                    if let Some(pos) = held.iter().position(|(t, _)| same(t, v)) {
                        s.read |= held.remove(pos).1;
                    }
                }
            }
        }
        if let Some(o) = out {
            for (k, r) in regs.tracked.iter().enumerate() {
                s.written[k] |= byte_mask(o, r);
            }
        }
        match opc {
            OpCode::CPUI_BRANCH
                if ins.first().is_some_and(|d| {
                    !d.space.as_ref().is_some_and(|sp| sp.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT)
                }) =>
            {
                s.jump = Some(ins[0].offset);
                s.ends = true;
            }
            OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_RETURN => {
                s.ends = true;
            }
            _ => {}
        }
    }
    s
}

fn registers(arch: &Architecture) -> Option<Regs> {
    let translate = arch.translate();
    let tracked = REGISTERS
        .iter()
        .map(|r| translate.get_register_data(r).ok())
        .collect::<Option<Vec<_>>>()?;
    let base = arch.manage().get_stack_space()?.get_spacebase(0).ok()?;
    let sp = VarnodeData { space: base.space.clone(), offset: base.offset, size: base.size as u32 };
    Some(Regs { tracked, sp })
}

/// The registers the straight line from `entry` reads before writing.
fn walk(arch: &Architecture, entry: &Address) -> Option<u8> {
    let space = entry.get_space()?.clone();
    let regs = registers(arch)?;
    let translate = arch.translate();
    let mut at = entry.get_offset();
    let mut written = [0u8; 6];
    let mut read = 0u8;
    let mut seen = std::collections::HashSet::new();
    for _ in 0..BUDGET {
        if !seen.insert(at) {
            break;
        }
        let addr = Address::new(space.clone(), at);
        let mut collect = Collect::default();
        let len = translate.one_instruction(&mut collect, &addr).ok()?;
        let s = step(&collect.0, &regs, written);
        read |= s.read;
        written = s.written;
        match (s.ends, s.jump) {
            (true, Some(target)) => at = target,
            (true, None) => break,
            (false, _) => at = at.wrapping_add(len.max(1) as uintb),
        }
    }
    Some(read)
}

#[cfg(test)]
mod tests;
