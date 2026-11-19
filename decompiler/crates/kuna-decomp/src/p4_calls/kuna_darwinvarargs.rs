//! (kuna `varargstackargs`) A callee whose own body runs Apple's `va_start`
//! is variadic at its call sites.
//!
//! # The gap
//!
//! Apple's arm64 ABI ("Writing ARM64 code for Apple platforms") passes every
//! variadic argument on the stack, in 8-byte slots from `[sp]`, and never in
//! `x1`-`x7`. [`varargstackargs`](crate::p4_calls::kuna_varargstackargs) already
//! scores such a call's stack tail as its own `fillinMap` section, so the empty
//! registers in front of it stop ending the argument list, but only for a call
//! whose prototype says `...`. A callee nothing declares, such as XNU's `panic`
//! or a program's own logger, has an unlocked prototype, so the caller's
//!
//! ```text
//!   stp  x8, x9, [sp]          ; the two variadic arguments
//!   adrp x0, "... @%s:%d"
//!   bl   _panic
//! ```
//!
//! renders as `panic("... @%s:%d");` with both arguments dropped.
//!
//! # The evidence
//!
//! The callee says it is variadic itself. Apple's `va_start` is the address of
//! the first stack argument, which is the caller's stack pointer at the call:
//!
//! ```text
//!   sub  sp, sp, #0x20
//!   stp  x29, x30, [sp, #0x10]
//!   add  x29, sp, #0x10
//!   add  x8, x29, #0x10        ; entry sp + 0: the va_list
//!   str  x8, [sp, #0x8]
//! ```
//!
//! A function reads a named stack parameter with a load through `sp` or `x29`;
//! it does not keep the ADDRESS of its incoming argument area in a variable.
//! [`probe`] decodes the callee's straight-line entry block, tracks every value
//! that is the entry stack pointer plus a constant, and answers yes when one at
//! or above the entry stack pointer is stored to memory or handed to a call in
//! an argument register. A call to such a callee is split by `varargstackargs`
//! exactly as a declared `...` call is.
//!
//! # A declared variadic call
//!
//! A locked `...` prototype reopens active-input recovery over every slot of
//! the model, registers included, so on Apple a register the caller merely left
//! a value in is scored like a variadic argument: at an `objc_msgSend$<sel>`
//! stub, which loads the selector into `x1` itself, the caller's stale `x1`
//! became an argument and `fillinMap`'s gap filling dropped the format in `x2`
//! (`stringWithFormat:(classRef_NSString,(NSString *)arg3,...)`).
//! [`keep_stack_tail`] applies Apple's rule after the map is filled: the fixed
//! parameters stay in the registers the prototype names and no other register
//! trial is an argument.
//!
//! Only the split changes: each stack argument still has to be ACTIVE on the
//! caller's own evidence, a store to that outgoing slot, so nothing is invented
//! for a slot the caller never wrote, and the register prefix is scored as
//! before. The rule applies only to an AArch64 image the loader identified as
//! Apple (or the Apple-silicon SLEIGH variant); AAPCS64 ELF passes varargs in
//! registers and is untouched.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::architecture::Architecture;
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::kuna_formatstring::VarargAbi;

/// The most instructions [`probe`] decodes from a callee's entry.
const MAX_PROBE_INSTRUCTIONS: usize = 24;

/// Does the loaded target pass variadic arguments the Apple arm64 way: an
/// AArch64 image the loader identified as Apple, or the Apple-silicon variant?
pub fn applies(arch: &Architecture) -> bool {
    arch.archid.starts_with("AARCH64:")
        && (arch.archid.contains("AppleSilicon") || arch.format_vararg_abi == Some(VarargAbi::Nothing))
}

/// Record, for every direct call in `data`, whether its callee runs Apple's
/// `va_start`. Taken before the action pipeline for the reason
/// [`crate::p4_calls::kuna_calleedeadarg::seed_callee_entry_dead`] is: the
/// pipeline's `ArchContext` has no translator. Cached per run on the
/// `Architecture`.
pub fn seed(arch: &mut Architecture, data: &mut Funcdata) {
    if !applies(arch) {
        data.kuna_set_darwin_variadic(None);
        return;
    }
    let Some(sp) = stack_pointer(arch) else {
        data.kuna_set_darwin_variadic(None);
        return;
    };
    let mut found = std::collections::BTreeSet::new();
    for i in 0..data.num_calls() {
        let e = data.get_call_specs(i).get_entry_address().clone();
        let Some(space) = e.get_space() else { continue };
        if space.get_type() != spacetype::IPTR_PROCESSOR {
            continue;
        }
        let key = (space.get_index(), e.get_offset());
        let hit = match arch.kuna_darwin_variadic_cache.get(&key) {
            Some(&v) => v,
            None => {
                let v = probe(arch.translate(), &e, &sp);
                arch.kuna_darwin_variadic_cache.insert(key, v);
                v
            }
        };
        if hit {
            found.insert(key);
        }
    }
    data.kuna_set_darwin_variadic(Some(found));
}

/// Is the call to `entry` one whose callee was seen running Apple's `va_start`?
pub fn is_variadic_callee(data: &Funcdata, entry: &Address) -> bool {
    let Some(found) = data.kuna_darwin_variadic() else { return false };
    entry.get_space().is_some_and(|s| found.contains(&(s.get_index(), entry.get_offset())))
}

/// Keep a declared variadic call's arguments where Apple's ABI puts them: its
/// fixed parameters in the registers the prototype names, the variable tail on
/// the stack only.
///
/// A locked `...` prototype reopens active-input recovery over every slot of the
/// model, so a register the caller happened to leave a value in -- `x1` at an
/// `objc_msgSend$<sel>` stub, which loads the selector itself -- is scored like
/// an argument, and the gap-filling of `fillinMap` can drop a fixed parameter
/// beside it. On an Apple target no variadic argument travels in a register, so
/// every non-fixed register trial is dropped and every fixed one kept.
pub fn keep_stack_tail(fc: &mut FuncCallSpecs, data: &Funcdata) {
    if data.kuna_darwin_variadic().is_none() || !fc.is_dotdotdot() || !fc.is_input_locked() {
        return;
    }
    let active = fc.get_active_input();
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial_mut(i);
        if trial.get_fixed_position() >= 0 {
            if !trial.is_definitely_not_used() {
                trial.mark_active();
                trial.mark_used();
            }
            continue;
        }
        let on_stack = trial.get_address().get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_SPACEBASE);
        if !on_stack && trial.is_used() {
            trial.mark_no_use();
        }
    }
}

/// The stack pointer register's storage `(space, offset, size)`.
fn stack_pointer(arch: &Architecture) -> Option<(Rc<AddrSpace>, uintb, u32)> {
    let base = arch.manage().get_stack_space()?.get_spacebase(0).ok()?;
    Some((base.space.clone()?, base.offset, base.size))
}

/// One decoded instruction's p-code, in order.
#[derive(Default)]
struct Emit {
    ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
}

impl PcodeEmit for Emit {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

/// A storage location's identity: `(space index, offset, size)`.
type Loc = (int4, uintb, u32);

fn loc(v: &VarnodeData) -> Option<Loc> {
    Some((v.space.as_ref()?.get_index(), v.offset, v.size))
}

fn constant(v: &VarnodeData) -> Option<i64> {
    let s = v.space.as_ref()?;
    if s.get_type() != spacetype::IPTR_CONSTANT || v.size == 0 || v.size > 8 {
        return None;
    }
    let bits = 64 - 8 * v.size;
    Some(((v.offset << bits) as i64) >> bits)
}

/// Does the straight-line entry block of the function at `entry` keep the
/// address of its incoming stack-argument area (Apple's `va_start`)?
///
/// Tracks the values that are the entry stack pointer plus a constant through
/// `COPY`, `INT_ADD` and `INT_SUB`. Answers `true` at the first `STORE` of such
/// a value at or above the entry stack pointer, or a `CALL`/`BRANCH` reached
/// with one in a register other than the stack pointer; `false` at any other
/// control transfer, an undecodable instruction or the instruction budget.
pub fn probe<T: Translate + ?Sized>(tr: &T, entry: &Address, sp: &(Rc<AddrSpace>, uintb, u32)) -> bool {
    let sp_loc: Loc = (sp.0.get_index(), sp.1, sp.2);
    let mut known: BTreeMap<Loc, Val> = BTreeMap::new();
    known.insert(sp_loc, Val::Sp(0));
    let Some(space) = entry.get_space().cloned() else { return false };
    let mut at = entry.clone();
    for _ in 0..MAX_PROBE_INSTRUCTIONS {
        let mut emit = Emit::default();
        let len = match tr.one_instruction(&mut emit, &at) {
            Ok(n) if n > 0 => n,
            _ => return false,
        };
        for (opc, out, ins) in &emit.ops {
            let val = |v: &VarnodeData| match constant(v) {
                Some(c) => Some(Val::Const(c)),
                None => loc(v).and_then(|l| known.get(&l).copied()),
            };
            match opc {
                OpCode::CPUI_STORE => {
                    if let Some(v) = ins.get(2) {
                        if loc(v) != Some(sp_loc) && matches!(val(v), Some(Val::Sp(o)) if o >= 0) {
                            return true;
                        }
                    }
                    continue;
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND => {
                    return known
                        .iter()
                        .any(|(l, v)| *l != sp_loc && l.0 == sp_loc.0 && matches!(v, Val::Sp(o) if *o >= 0));
                }
                OpCode::CPUI_CBRANCH | OpCode::CPUI_RETURN => return false,
                _ => {}
            }
            let Some(o) = out.as_ref().and_then(loc) else { continue };
            let next = match (opc, ins.as_slice()) {
                (OpCode::CPUI_COPY, [a]) => val(a),
                (OpCode::CPUI_INT_ADD, [a, b]) => match (val(a), val(b)) {
                    (Some(Val::Sp(x)), Some(Val::Const(c))) | (Some(Val::Const(c)), Some(Val::Sp(x))) => {
                        x.checked_add(c).map(Val::Sp)
                    }
                    (Some(Val::Const(x)), Some(Val::Const(c))) => Some(Val::Const(x.wrapping_add(c))),
                    _ => None,
                },
                (OpCode::CPUI_INT_SUB, [a, b]) => match (val(a), val(b)) {
                    (Some(Val::Sp(x)), Some(Val::Const(c))) => x.checked_sub(c).map(Val::Sp),
                    _ => None,
                },
                _ => None,
            };
            known.retain(|l, _| !overlaps(*l, o));
            if let Some(v) = next {
                known.insert(o, v);
            }
        }
        let Some(n) = at.get_offset().checked_add(len as uintb) else { return false };
        at = Address::new(Rc::clone(&space), n);
    }
    false
}

/// A value the probe follows: the entry stack pointer plus an offset, or a constant.
#[derive(Clone, Copy)]
enum Val {
    Sp(i64),
    Const(i64),
}

fn overlaps(a: Loc, b: Loc) -> bool {
    a.0 == b.0 && a.1 < b.1.saturating_add(b.2 as uintb) && b.1 < a.1.saturating_add(a.2 as uintb)
}

#[cfg(test)]
#[path = "kuna_darwinvarargs/tests.rs"]
mod tests;
