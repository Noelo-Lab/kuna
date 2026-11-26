//! (kuna) A register the epilogue pops to release the frame is not the return
//! value beside a value the function returns in another storage class.
//!
//! `float f(void *p, unsigned i) { double d = *(double *)((char *)p + 0x108);
//! return ((float *)&d)[i]; }` is, from clang for Win64 or for x86-64 without a
//! red zone, `push rax; vmovsd xmm0,[rcx+0x108]; vmovsd [rsp],xmm0; mov eax,edx;
//! vmovss xmm0,[rsp+rax*4]; pop rax; ret`. The `push rax`/`pop rax` pair is the
//! 8-byte frame and the spill overwrote the pushed slot, so at the RETURN `rax`
//! holds the double's bits. Both `rax` and `xmm0` pass upstream's realism and
//! only-used-here tests, and the output model's fallback prefers the general
//! register, so the function printed `unsigned long long f(void *a0) { return
//! ((unsigned long long *)a0)[0x21]; }`: the float, the index parameter and the
//! indexed read were gone.
//!
//! Compilers pop into a register only to release stack, and pick one that is
//! dead there. [`note_pops`] marks, after the first scoring pass, a return
//! trial whose value at every live RETURN is what a single-register pop wrote
//! (a load at the stack pointer by an instruction that also advances the stack
//! pointer by the loaded width) when a trial of another storage class is already
//! active. That pass still sees the frame's stores, so a float stored into the
//! slot the pop reads (an 8-byte struct built in the frame and returned in
//! `rax`) is not yet active and marks nothing. [`yield_to_other_class`] makes a
//! marked trial inactive once scoring ends, when a rival of another class is
//! still active and the output model then returns it. A pop with no rival
//! (`pushfq; pop rax; ret`) still returns its value.

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_class;
use crate::fspec::{ParamActive, ParamTrial};
use crate::funcdata::Funcdata;

/// Mark each trial whose value at every live RETURN is a pop's write, when a
/// trial of another storage class is active.
///
/// Called by `ActionReturnRecovery` right after its first pass is scored,
/// while each RETURN still reads the register the instruction wrote.
pub fn note_pops(data: &Funcdata, active: &mut ParamActive, return_ops: &[OpId]) {
    if active.get_num_passes() != 0 {
        return;
    }
    let rets: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| {
            data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0)
                && !crate::kuna_bejoin::never_reached(data, r)
        })
        .collect();
    if rets.is_empty() {
        return;
    }
    for i in 0..active.get_num_trials() {
        let slot = active.get_trial(i).get_slot();
        let popped = rets.iter().all(|&r| {
            data.obank().get(r).and_then(|o| o.get_in(slot)).is_some_and(|vn| popped(data, vn))
        });
        if popped && rival(data, active, i) {
            active.get_trial_mut(i).set_epilogue_pop();
        }
    }
}

/// Make inactive each active trial [`note_pops`] marked when a rival of
/// another storage class is still active and the output model, without the
/// marked trial, returns something.
///
/// Runs once `ActionReturnRecovery` has scored every trial for the last time,
/// before the output map.
pub fn yield_to_other_class(data: &Funcdata, active: &mut ParamActive) {
    for i in 0..active.get_num_trials() {
        let t = active.get_trial(i);
        if !t.is_active() || !t.has_epilogue_pop() || !rival(data, active, i) {
            continue;
        }
        let mut without = active.clone();
        without.get_trial_mut(i).mark_inactive();
        let manager = data.get_arch().manage.clone();
        if data.get_func_proto().derive_output_map(&mut without, &manager).is_err() {
            continue;
        }
        if (0..without.get_num_trials()).any(|j| without.get_trial(j).is_used()) {
            active.get_trial_mut(i).mark_inactive();
        }
    }
}

/// Is another trial than `i` active in an output entry of a different storage
/// class than trial `i`'s?
fn rival(data: &Funcdata, active: &ParamActive, i: i32) -> bool {
    let Some(own) = class(data, active.get_trial(i)) else { return false };
    (0..active.get_num_trials()).any(|j| {
        let r = active.get_trial(j);
        j != i && r.is_active() && class(data, r).is_some_and(|c| c != own)
    })
}

/// The storage class of the output entry trial `t` lands in.
fn class(data: &Funcdata, t: &ParamTrial) -> Option<type_class> {
    let proto = data.get_func_proto();
    let out = proto.has_model().then(|| proto.model().output_list()).flatten()?;
    out.get_entry().iter().find(|e| e.intersects(t.get_address(), t.get_size())).map(|e| e.get_type())
}

/// Is `vn` the register a single-register pop wrote: a LOAD at the stack
/// pointer, copied out of the instruction's temporary, in an instruction that
/// also adds the loaded width to that stack pointer?
fn popped(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    let size = v.get_size();
    let Some(def) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
    let at = def.get_addr().clone();
    let load = match def.code() {
        OpCode::CPUI_LOAD => def,
        OpCode::CPUI_COPY => {
            let Some(src) = def.get_in(0).and_then(|s| data.vbank().get(s)) else { return false };
            if src.get_addr().get_space().map(|s| s.get_type()) != Some(spacetype::IPTR_INTERNAL) {
                return false;
            }
            match src.get_def().and_then(|d| data.obank().get(d)) {
                Some(o) if o.code() == OpCode::CPUI_LOAD && *o.get_addr() == at => o,
                _ => return false,
            }
        }
        _ => return false,
    };
    let Some(ptr) = load.get_in(1) else { return false };
    if load.get_out().and_then(|o| data.vbank().get(o)).map(|o| o.get_size()) != Some(size)
        || !is_stack_pointer(data, ptr)
    {
        return false;
    }
    data.obank().iter_at(&at).any(|(_, id)| {
        let Some(o) = data.obank().get(id) else { return false };
        o.code() == OpCode::CPUI_INT_ADD
            && o.get_in(0) == Some(ptr)
            && o.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| {
                c.is_constant() && c.get_offset() == size as u64
            })
            && o.get_out().is_some_and(|out| is_stack_pointer(data, out))
    })
}

/// Is `vn` the stack space's base register, whole?
fn is_stack_pointer(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(stack) = data.get_arch().manage().get_stack_space().cloned() else { return false };
    let Ok(sp) = stack.get_spacebase(0) else { return false };
    let Some(v) = data.vbank().get(vn) else { return false };
    let addr = v.get_addr();
    sp.space.as_ref().zip(addr.get_space()).is_some_and(|(a, b)| a.get_index() == b.get_index())
        && addr.get_offset() == sp.offset
        && v.get_size() as u32 == sp.size
}
