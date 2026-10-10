//! (kuna) A register gcc's `-fzero-call-used-regs` epilogue clears before the
//! return is not part of the value the function returns (option
//! `zerocallregs`).
//!
//! With `-fzero-call-used-regs=all` (openssh's hardened builds, the kernel's
//! `used-gpr`) gcc ends every function with a run that clears each call-used
//! register that is not live at the return: `xor %edx,%edx`, `xor %ecx,%ecx`,
//! `pxor %xmm0,%xmm0` .. `pxor %xmm15,%xmm15`, then `ret`. A register that
//! carries the return value is live there, so it is the one the run leaves
//! alone. Return recovery scores each output register at the RETURN, and a
//! cleared one holds a value the function wrote, so it is accepted:
//! `int put(int *p,int v)`, which returns its callee's `eax`, printed as
//! `double put(..) { ..; return 0.0; }` from the cleared `xmm0`, a `double`
//! returner as `unsigned long f(void) { return 0; }` from the cleared `rax`,
//! and a `long` as `undefined16` whose upper half is the cleared `rdx`.
//!
//! [`drop_epilogue_zeros`] runs before the output fill-in. For each live
//! RETURN it walks back from the return instruction over the instructions of
//! its block that only clear registers: each one leaves a provable zero (a
//! value XORed or subtracted with itself, or a constant zero, through copies,
//! extensions and slices) in every register it writes, apart from one-byte
//! flags it computes from them. That run is the epilogue only when one of its
//! instructions clears nothing but registers the prototype model never returns
//! a value in (`rcx`, `rsi`, `xmm2`), to a zero nothing after it reads: `xor
//! %eax,%eax; xor %edx,%edx; ret` returns a zero pair. A trial is retired
//! when, at every RETURN with such a run, its value is a zero the run itself
//! wrote; a RETURN with no run (a tail call) has no say. The x87 `fldz`/`fstp`
//! block that `=all` puts first stops the walk, so a body's own `xor
//! %eax,%eax` before it is still the returned zero. A declared output, and
//! storage the callers forced (`kuna_voidret`), are never touched.
//!
//! What is left is what the same code returns without the epilogue: a
//! computed `eax`, `xmm0` or `rdx:rax`, or nothing for a function that only
//! forwards an unresolved call's result, which `kuna_voidret` settles from
//! its callers. Under `=all-gpr` a `return 0` written right before the run
//! cannot be told from the run's own first clear and returns `void`.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::{ParamActive, ParamEntry};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_zeroidiomuse::same_value_at;

/// (kuna) Drop the registers a `-fzero-call-used-regs` epilogue clears from
/// the return value: `zerocallregs on|off`.
pub struct OptionZeroCallRegs;

impl OptionZeroCallRegs {
    /// The option name.
    pub const NAME: &'static str = "zerocallregs";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::zero_call_regs`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Zero-call-used-regs epilogue registers dropped from the return: {prop}")))
    }
}

/// How many defining ops one zero is traced through.
const MAX_DEPTH: u32 = 8;

/// How many instructions the backward walk visits before giving up.
const MAX_RUN: usize = 96;

/// One RETURN's trailing run of clearing instructions: its block, the address
/// of its first instruction and of the return instruction.
struct Run {
    ret: OpId,
    block: BlockId,
    start: Address,
    end: Address,
}

/// The output entries of `fd`'s model, or of the default model before the
/// function has one.
fn output_entries(fd: &Funcdata) -> Vec<ParamEntry> {
    let proto = fd.get_func_proto();
    let model = if proto.has_model() { Some(proto.model()) } else { fd.get_arch().defaultfp.as_ref() };
    model.and_then(|m| m.output_list()).map(|o| o.get_entry().to_vec()).unwrap_or_default()
}

/// Retire the active output trials that only hold a zero the epilogue run of
/// a `-fzero-call-used-regs` build wrote.
pub fn drop_epilogue_zeros(data: &Funcdata, active: &mut ParamActive, returns: &[OpId]) {
    if !data.get_arch().zero_call_regs || data.get_func_proto().is_output_locked() {
        return;
    }
    let outputs = output_entries(data);
    if outputs.is_empty() {
        return;
    }
    let runs: Vec<Run> = returns
        .iter()
        .copied()
        .filter(|&id| data.obank().get(id).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .filter_map(|id| epilogue_run(data, id, &outputs))
        .collect();
    if runs.is_empty() {
        return;
    }
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial(i);
        if !trial.is_active()
            || crate::p4_calls::kuna_voidret::forced(data, trial.get_address(), trial.get_size())
        {
            continue;
        }
        let slot = trial.get_slot();
        let cleared = runs.iter().all(|run| {
            data.obank()
                .get(run.ret)
                .and_then(|o| o.get_in(slot))
                .is_some_and(|vn| zero_in_run(data, vn, run, MAX_DEPTH))
        });
        if cleared {
            active.get_trial_mut(i).mark_inactive();
        }
    }
}

/// The run of clearing instructions that ends at the live RETURN `ret`, when
/// one of them only clears registers no output entry names, to a zero nothing
/// after it reads: `xor %ecx,%ecx`, not the `xor %r8d,%r8d` a later `mov
/// %r8d,%eax` returns, nor AArch64's `movi d0,#0`, whose lift also clears the
/// SVE bytes above `q0`.
fn epilogue_run(data: &Funcdata, ret: OpId, outputs: &[ParamEntry]) -> Option<Run> {
    let op = data.obank().get(ret)?;
    let block = op.get_parent()?;
    let end = op.get_addr().clone();
    let mut cur = op.basic_neighbours().0;
    while let Some(id) = cur {
        let o = data.obank().get(id)?;
        if o.get_addr() != &end {
            break;
        }
        if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCHIND) {
            return None;
        }
        cur = o.basic_neighbours().0;
    }
    let mut start: Option<Address> = None;
    let mut foreign = false;
    let mut visited = 0usize;
    while let Some(first) = cur {
        visited += 1;
        if visited > MAX_RUN {
            break;
        }
        let at = data.obank().get(first)?.get_addr().clone();
        let mut ops: Vec<OpId> = Vec::new();
        let mut walk = Some(first);
        while let Some(id) = walk {
            let o = data.obank().get(id)?;
            if o.get_addr() != &at {
                break;
            }
            ops.push(id);
            walk = o.basic_neighbours().0;
        }
        let Some(clears) = clearing_instruction(data, &ops) else { break };
        foreign |= clears.iter().all(|&(vn, ref addr, size)| {
            !outputs.iter().any(|e| e.intersects(addr, size)) && unread_after(data, vn, &at)
        });
        start = Some(at);
        cur = walk;
    }
    let start = start?;
    foreign.then_some(Run { ret, block, start, end })
}

/// The registers the instruction made of `ops` (in reverse order) clears,
/// when the last value it leaves in every register it writes is a provable
/// zero, or a one-byte flag computed by a comparison, and it clears at least
/// one register wider than a flag. A write a later write of the same
/// instruction covers is heritage's slicing of the old value, not a result.
fn clearing_instruction(data: &Funcdata, ops: &[OpId]) -> Option<Vec<(VarnodeId, Address, i32)>> {
    let mut writes: Vec<(OpId, VarnodeId)> = Vec::new();
    for &id in ops.iter().rev() {
        let o = data.obank().get(id)?;
        match o.code() {
            OpCode::CPUI_MULTIEQUAL => continue,
            OpCode::CPUI_INDIRECT => return None,
            _ => {}
        }
        let vn = o.get_out()?;
        match data.vbank().get(vn)?.get_space().get_type() {
            spacetype::IPTR_INTERNAL => {}
            spacetype::IPTR_PROCESSOR => writes.push((id, vn)),
            _ => return None,
        }
    }
    let mut clears = Vec::new();
    for (k, &(id, vn)) in writes.iter().enumerate() {
        let out = data.vbank().get(vn)?;
        let (addr, size) = (out.get_addr(), out.get_size());
        let covered = writes[k + 1..].iter().any(|&(_, later)| {
            data.vbank().get(later).is_some_and(|l| addr.contained_by(size, l.get_addr(), l.get_size()))
        });
        if covered {
            continue;
        }
        if is_zero(data, vn, MAX_DEPTH) {
            if size > 1 {
                clears.push((vn, addr.clone(), size));
            }
            continue;
        }
        if size != 1 || !is_flag_op(data.obank().get(id)?.code()) {
            return None;
        }
    }
    (!clears.is_empty()).then_some(clears)
}

/// Is every reader of `vn` an op of the instruction at `at` itself, as the
/// flags of a clearing XOR are?
fn unread_after(data: &Funcdata, vn: VarnodeId, at: &Address) -> bool {
    data.vbank().get(vn).is_some_and(|v| {
        v.descend_iter().all(|r| data.obank().get(r).is_some_and(|o| o.get_addr() == at))
    })
}

/// Does `code` compute a boolean, as the flags of a clearing XOR are?
fn is_flag_op(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
    )
}

/// Is `vn` zero whatever the function's inputs are: a constant zero, a value
/// XORed or subtracted with itself, or a copy, extension, slice or
/// concatenation of such zeros?
fn is_zero(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    zero_def(data, vn, depth, &|_| true)
}

/// Is `vn` a zero ([`is_zero`]) every op of which lies in `run`: its block,
/// from the run's first instruction up to the return instruction?
fn zero_in_run(data: &Funcdata, vn: VarnodeId, run: &Run, depth: u32) -> bool {
    let inside = |id: OpId| {
        data.obank().get(id).is_some_and(|o| {
            o.get_parent() == Some(run.block) && o.get_addr() >= &run.start && o.get_addr() <= &run.end
        })
    };
    data.vbank().get(vn).is_some_and(|v| v.is_written()) && zero_def(data, vn, depth, &inside)
}

/// [`is_zero`], requiring `keep` of every defining op on the way.
fn zero_def(data: &Funcdata, vn: VarnodeId, depth: u32, keep: &dyn Fn(OpId) -> bool) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return v.get_offset() == 0;
    }
    if depth == 0 {
        return false;
    }
    let Some(def) = v.get_def() else { return false };
    if !keep(def) {
        return false;
    }
    let Some(op) = data.obank().get(def) else { return false };
    let input = |k: i32| op.get_in(k).is_some_and(|i| zero_def(data, i, depth - 1, keep));
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => input(0),
        OpCode::CPUI_SUBPIECE => input(0) || slice_of_zero(data, op, v.get_size(), depth - 1, keep),
        OpCode::CPUI_PIECE => input(0) && input(1),
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB => match (op.get_in(0), op.get_in(1)) {
            (Some(a), Some(b)) => same_value_at(data, a, b) || (input(0) && input(1)),
            _ => false,
        },
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_MULT => input(0) || input(1),
        _ => false,
    }
}

/// Is the SUBPIECE `op`, `size` bytes wide, a slice that lies wholly in a
/// zero half of the PIECE it reads: the low word heritage reads back out of
/// the whole register a 32-bit `xor` rebuilt around it?
fn slice_of_zero(
    data: &Funcdata,
    op: &crate::op::PcodeOp,
    size: i32,
    depth: u32,
    keep: &dyn Fn(OpId) -> bool,
) -> bool {
    let Some(off) = op.get_in(1).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset() as i64) else {
        return false;
    };
    let Some(whole) = op.get_in(0).and_then(|w| data.vbank().get(w)) else { return false };
    let Some(def) = whole.get_def().filter(|&d| keep(d)) else { return false };
    let Some(piece) = data.obank().get(def).filter(|p| p.code() == OpCode::CPUI_PIECE) else { return false };
    let (Some(hi), Some(lo)) = (piece.get_in(0), piece.get_in(1)) else { return false };
    let Some(lo_size) = data.vbank().get(lo).map(|l| i64::from(l.get_size())) else { return false };
    if off + i64::from(size) <= lo_size {
        zero_def(data, lo, depth, keep)
    } else if off >= lo_size {
        zero_def(data, hi, depth, keep)
    } else {
        false
    }
}

#[cfg(test)]
#[path = "kuna_zerocallregs/tests.rs"]
mod tests;
