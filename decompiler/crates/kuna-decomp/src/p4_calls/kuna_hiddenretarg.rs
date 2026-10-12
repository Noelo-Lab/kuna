//! (kuna) `hiddenretarg` — a value left in the hidden-return register is not a
//! call argument unless the callee could be returning through it.
//!
//! AAPCS64 passes the address of a large returned aggregate in `x8`, the
//! indirect-result register, and every AArch64 cspec (Linux, Apple, Windows)
//! lists it as the `hiddenret` input entry. `checkInputTrialUse` scores that
//! trial like any argument register: a value the caller wrote to `x8` and never
//! read again is active, and its own resource section sorts it ahead of `x0`.
//! But `x8` is also an ordinary scratch register, so whatever a caller leaves
//! there becomes the call's first argument: `mov x8,#0x38; mov x0,#3; bl foo`
//! prints `foo(0x38,3)` for a one-argument `foo`, and every glibc syscall
//! wrapper passes its syscall number to the helper it calls after the `svc`.
//!
//! A trial scoring left active is rescored no-use (an inactive one keeps its
//! answer, so its dataflow is not freed). The callee's own body, read through
//! the cached entry walk `calleedeadarg` owns, decides first:
//!
//! * a callee that never takes `x8` ([`CalleeEntryDead::never_takes`]) vetoes
//!   it: no path reads it before writing it, and none leaves for code the walk
//!   did not read (a call, a `CALLOTHER`, an indirect branch) with it still
//!   unwritten. A RETURN needs no write first, since `x8` is not a
//!   source-level argument a stub could be ignoring;
//! * a callee that reads `x8` (a struct return storing through it, a hand
//!   written helper looping on it) or reaches a system call with it unwritten
//!   (`helper: svc #0; ret`, whose kernel reads the caller's syscall number)
//!   keeps it, whatever the value.
//!
//! Where the body does not settle it, two facts about the value do. A value
//! already vetoed from another call is vetoed here too: freeing it there leaves
//! this call its only reader, and the double-use rule that kept it off this
//! call would otherwise no longer see the first one (clang -O0 computes a
//! comparison into `w8` ahead of two calls on different paths); a frame address
//! is never remembered that way, since a hoisted `add x8,sp,#16` can be the
//! real result buffer of a later call. And a value
//! whose every leaf, through copies, zero extensions, phis and non-creation
//! INDIRECTs, is a constant in the null page (below 0x1000) cannot be a result
//! buffer, since no operating system maps that page; this answers for a callee
//! the walk cannot cover, such as an import or a glibc helper that reaches an
//! `ldaxr`/`stxr` pair before it writes `x8`. It cannot tell a callee that takes
//! a small number in `x8` by its own convention through an import the walk
//! cannot read (the OCaml runtime's `caml_allocN` through a PLT stub), and drops
//! that number.
//!
//! The entry is its own resource section, so dropping it leaves no hole in
//! `x0..x7`. A `hiddenret` entry sharing storage with an ordinary input entry
//! (tricore's `a4`) or living on the stack (SPARC's `[sp+64]`) is left alone.
//! Default-**on**; `off` restores the upstream scoring.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::type_class::TYPECLASS_HIDDENRET;
use crate::fspec::{FuncCallSpecs, ProtoModel};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// (kuna) Veto a hidden-return trial no callee could use: `hiddenretarg on|off`.
pub struct OptionHiddenRetArg;

impl OptionHiddenRetArg {
    /// The option name.
    pub const NAME: &'static str = "hiddenretarg";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::hidden_ret_arg`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Hidden-return argument veto turned {prop}")))
    }
}

/// Addresses below this are never mapped for a program, so never a result buffer.
const NULL_PAGE: u64 = 0x1000;

/// How many value-preserving links [`null_page_constant`] follows.
const MAX_DEPTH: u32 = 8;

/// Does `model` take a hidden-return pointer in a register of space `reg_idx`?
pub fn has_register_hidden_return(model: &ProtoModel, reg_idx: int4) -> bool {
    model
        .input()
        .get_entry()
        .iter()
        .any(|e| e.get_type() == TYPECLASS_HIDDENRET && e.get_space().get_index() == reg_idx)
}

/// Is `[addr, addr+size)` the call model's hidden-return input REGISTER,
/// storage no other input entry shares? SPARC's stack-slot entry is not one.
pub fn hidden_return_slot(call: &FuncCallSpecs, addr: &Address, size: int4) -> bool {
    let Some(sp) = addr.get_space() else { return false };
    if sp.get_type() == spacetype::IPTR_SPACEBASE || !call.proto().has_model() {
        return false;
    }
    let entries = call.proto().model().input().get_entry();
    let (off, end) = (addr.get_offset(), addr.get_offset().wrapping_add(size.max(0) as u64));
    entries.iter().any(|e| e.get_type() == TYPECLASS_HIDDENRET && e.justified_contain(addr, size) >= 0)
        && !entries.iter().any(|e| {
            e.get_type() != TYPECLASS_HIDDENRET
                && e.get_space().get_index() == sp.get_index()
                && e.get_base() < end
                && off < e.get_base().wrapping_add(e.get_size().max(0) as u64)
        })
}

/// How many Varnodes [`value_leaves`] visits before it stops looking.
const MAX_VISITED: usize = 64;

/// The values `vn` is made of: every Varnode reached through copies, zero
/// extensions, phis and the non-creation INDIRECTs around other ops that is not
/// itself one of those. Past [`MAX_DEPTH`] or [`MAX_VISITED`] the Varnode
/// reached stands for itself.
pub fn value_leaves(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let mut leaves = Vec::new();
    let mut seen: Vec<VarnodeId> = Vec::new();
    let mut todo = vec![(vn, 0u32)];
    while let Some((v, depth)) = todo.pop() {
        if seen.contains(&v) {
            continue;
        }
        seen.push(v);
        let def = data.vbank().get(v).and_then(|x| x.get_def()).and_then(|d| data.obank().get(d));
        let inputs = match def {
            Some(op) if depth < MAX_DEPTH && seen.len() < MAX_VISITED => match op.code() {
                OpCode::CPUI_INDIRECT if op.is_indirect_creation() => 0,
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INDIRECT => 1,
                OpCode::CPUI_MULTIEQUAL => op.num_input(),
                _ => 0,
            },
            _ => 0,
        };
        if inputs == 0 {
            leaves.push(v);
            continue;
        }
        let op = def.expect("an op with inputs");
        todo.extend((0..inputs).filter_map(|i| op.get_in(i)).map(|x| (x, depth + 1)));
    }
    leaves
}

/// Is `vn` the stack pointer, or reached from it through copies, adds,
/// subtracts and non-creation INDIRECTs along their first input? Before
/// constant propagation `add x8,sp,#16` adds a temporary, not a constant.
fn frame_address(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(sp) = crate::p4_calls::kuna_spillargtrial::stack_pointer_storage(data) else {
        return false;
    };
    let Some(spspace) = sp.space.as_ref() else { return false };
    let mut cur = vn;
    for _ in 0..2 * MAX_DEPTH {
        let Some(v) = data.vbank().get(cur) else { return false };
        if !v.is_constant()
            && v.get_space().get_index() == spspace.get_index()
            && v.get_addr().get_offset() == sp.offset
            && v.get_size() as u32 == sp.size
        {
            return true;
        }
        let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
        match op.code() {
            OpCode::CPUI_INDIRECT if op.is_indirect_creation() => return false,
            OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_INDIRECT => match op.get_in(0) {
                Some(x) => cur = x,
                None => return false,
            },
            _ => return false,
        }
    }
    false
}

/// Can the hidden-return value with `leaves` not be a result buffer of this call?
///
/// The callee's own body decides first: one that never takes `[addr, addr+size)`
/// vetoes, and one that reads it or reaches a system call with it unwritten
/// keeps it. Otherwise a value already vetoed from another call is vetoed here
/// too, since freeing it there must not make it this call's argument, and so is
/// a value whose every leaf is a constant in the null page.
fn not_a_result_buffer(
    data: &Funcdata,
    call: &FuncCallSpecs,
    addr: &Address,
    size: int4,
    leaves: &[VarnodeId],
) -> bool {
    let entry = call.get_entry_address();
    if let Some(d) = (!entry.is_invalid()).then(|| data.kuna_callee_entry_dead(entry)).flatten() {
        if d.never_takes(addr, size) {
            return true;
        }
        if d.proves_read(addr, size) || d.traps_unwritten(addr, size) {
            return false;
        }
    }
    let null_page = |l: &VarnodeId| {
        data.vbank().get(*l).is_some_and(|v| v.is_constant() && v.get_offset() < NULL_PAGE)
    };
    leaves.iter().any(|&l| data.kuna_hiddenret_vetoed(l))
        || (!leaves.is_empty() && leaves.iter().all(null_page))
}

/// Rescore trial `trial` of call `call_idx` no-use when it is an ACTIVE trial on
/// the model's hidden-return register that cannot be a result buffer, and
/// remember the value so no later call takes it up. Answers whether it did.
///
/// A leaf that is an address in the caller's frame is not remembered: a
/// hoisted `add x8,sp,#16` can be a leftover at one call and the real result
/// buffer at a later one.
///
/// `checkInputTrialUse` asks this once its own scoring is done, so an inactive
/// or no-use trial keeps its upstream answer.
pub fn rescore_active_trial(data: &mut Funcdata, call_idx: int4, trial: int4) -> bool {
    if !data.get_arch().hidden_ret_arg {
        return false;
    }
    let leaves = {
        let call = data.get_call_specs(call_idx);
        let t = call.active_input().get_trial(trial);
        if !t.is_active() || !hidden_return_slot(call, t.get_address(), t.get_size()) {
            return false;
        }
        let Some(vn) = data.obank().get(call.get_op()).and_then(|o| o.get_in(t.get_slot())) else {
            return false;
        };
        let leaves = value_leaves(data, vn);
        if !not_a_result_buffer(data, call, t.get_address(), t.get_size(), &leaves) {
            return false;
        }
        leaves.into_iter().filter(|&l| !frame_address(data, l)).collect::<Vec<_>>()
    };
    data.kuna_note_hiddenret_vetoed(&leaves);
    data.get_call_specs_mut(call_idx).get_active_input().get_trial_mut(trial).mark_no_use();
    true
}

#[cfg(test)]
#[path = "kuna_hiddenretarg/tests.rs"]
mod tests;
