//! (kuna) A register the function sets for a system register is not the high
//! word of its return.
//!
//! `int f(void) { int r = g(); __asm volatile("vmsr fpscr, %0" :: "r"(K));
//! return r; }` is, on 32-bit ARM, `bl g; mov r1,#K; vmsr fpscr,r1; pop
//! {r11,pc}`. Upstream's `onlyOpUse` takes a COPY into another register, or a
//! CALLOTHER, for an alternate path rather than a competing use, so `r1` is
//! scored as returned, and the output model joins it under `r0`: the function
//! printed `unsigned long long f(void) { return CONCAT44(0x3000000,g()); }`.
//! FreeRTOS's `ulPortRaiseBASEPRI` (`mrs r0,basepri; mov r1,#0x50; msr
//! basepri,r1; bx lr`) returned `CONCAT44(0x50,v1)` the same way, and so did a
//! function returning a value it computes in `r0`.
//!
//! [`drop_set_aside`] makes such a trial inactive when it is a register the
//! model returns only beside another one, and when at every live RETURN each
//! value it merges is written to a system register ([`sinks`]): read, alone or
//! through temporaries, by a user op that sets processor state (`msr
//! basepri`, `msr primask`, MIPS `mtc0`, AArch64 `msr fpcr`), or written to a
//! register the prototype model names nowhere whose value the function then
//! only turns into flags (`vmsr fpscr`, Thumb's `msr cpsr_c`, which unpacks
//! `cpsr` into the flags). A prefetch, cache or barrier op is no such write:
//! it takes an address, and a returned word can be one. The first register
//! then stands alone. When it was refused too, as a call's untouched result
//! is, the function returns nothing, as `bl g; pop {r11,pc}` does until a
//! caller reads the result (`kuna_voidret`).
//!
//! The pair stays where the second register may still be the high word of a
//! 64-bit value. A 64-bit system register takes both words in one write (x86's
//! `wrmsr` reads `edx:eax`, ARM's `mcrr` reads `r0` and `r1`), so a write that
//! also reads another register the RETURN reads keeps the pair ([`alone`]),
//! and so does a first register handed to the machine as well, as a 64-bit
//! value written in halves is. A value computed from a call's result in that
//! same register, the high word of the call's 64-bit result, keeps it too
//! ([`from_call_result`]).
//!
//! The bytes cannot settle the rest: a 64-bit function that writes its own high
//! word to `fpscr` is byte for byte the `int` function that used `r1` as the
//! scratch for that write. The rule is option `retsysreg`, on by default.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::{effect_type, Containment, ParamActive};
use crate::funcdata::Funcdata;

/// How many Varnodes a forward walk visits before it gives up.
const MAX_WALK: usize = 64;

/// How many defining ops a backward walk follows.
const MAX_DEPTH: u32 = 8;

/// Make inactive each active return trial that the output model returns only
/// beside another register and whose value, at every live RETURN, the function
/// set for a system register.
///
/// Runs once `ActionReturnRecovery` has scored every trial for the last time,
/// before [`crate::kuna_retcallhalf::accept`] and the output map.
pub fn drop_set_aside(data: &Funcdata, active: &mut ParamActive, return_ops: &[OpId]) {
    if !data.get_arch().ret_sys_reg {
        return;
    }
    let rets: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    if rets.is_empty() {
        return;
    }
    for i in 0..active.get_num_trials() {
        let t = active.get_trial(i);
        if !t.is_active() {
            continue;
        }
        let (addr, size, slot) = (t.get_address().clone(), t.get_size(), t.get_slot());
        let Some(values) = rets
            .iter()
            .map(|&r| data.obank().get(r).and_then(|o| o.get_in(slot)))
            .collect::<Option<Vec<VarnodeId>>>()
        else {
            continue;
        };
        if !values.iter().all(|&vn| set_aside(data, vn, &addr, size)) {
            continue;
        }
        let others: Vec<VarnodeId> = rets
            .iter()
            .filter_map(|&r| data.obank().get(r))
            .flat_map(|o| (1..o.num_input()).filter(|&k| k != slot).filter_map(|k| o.get_in(k)))
            .collect();
        let handed_over = |vn: VarnodeId| {
            crate::kuna_retcallhalf::roots(data, vn)
                .is_none_or(|roots| roots.into_iter().any(|r| sinks(data, r).is_none_or(|s| !s.is_empty())))
        };
        if values.iter().all(|&vn| alone(data, vn, &others))
            && !others.iter().any(|&o| handed_over(o))
            && !returned_alone(data, active, i)
        {
            active.get_trial_mut(i).mark_inactive();
        }
    }
}

/// Did the function set `vn`, the value at `addr`/`size` a RETURN reads, for a
/// system register: is every value it merges handed to the machine
/// ([`sinks`]) and not computed from a call's result in that register?
fn set_aside(data: &Funcdata, vn: VarnodeId, addr: &Address, size: i32) -> bool {
    let Some(roots) = crate::kuna_retcallhalf::roots(data, vn) else { return false };
    roots.into_iter().all(|root| {
        sinks(data, root).is_some_and(|s| !s.is_empty()) && !from_call_result(data, root, addr, size)
    })
}

/// Is every write that takes a value `vn` merges to the machine free of the
/// other registers a RETURN reads (`others`)?
fn alone(data: &Funcdata, vn: VarnodeId, others: &[VarnodeId]) -> bool {
    let Some(roots) = crate::kuna_retcallhalf::roots(data, vn) else { return false };
    roots
        .into_iter()
        .all(|root| sinks(data, root).is_some_and(|s| s.iter().all(|&op| !reads_any(data, op, others))))
}

/// The ops that write `root`, followed forward through phis, INDIRECTs and
/// temporaries but not through a load, to a system register: a CALLOTHER that
/// sets processor state ([`sets_state`]), or a write to a register the
/// prototype model names nowhere and that only becomes flags after
/// ([`only_flags`]). `None` when the walk gives up.
fn sinks(data: &Funcdata, root: VarnodeId) -> Option<Vec<OpId>> {
    let mut seen = std::collections::BTreeSet::new();
    let mut work = vec![root];
    let mut found = Vec::new();
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_WALK {
            return None;
        }
        for reader in data.vbank().get(cur)?.descend_iter() {
            let op = data.obank().get(reader)?;
            if sets_state(data, reader) {
                found.push(reader);
                continue;
            }
            if matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_CALLOTHER) {
                continue;
            }
            let Some(out) = op.get_out() else { continue };
            let o = data.vbank().get(out)?;
            let space = o.get_addr().get_space().map(|sp| sp.get_type());
            if matches!(op.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT)
                || crate::kuna_passthrough::is_injected_noop(data, reader)
                || space == Some(spacetype::IPTR_INTERNAL)
            {
                work.push(out);
            } else if space == Some(spacetype::IPTR_PROCESSOR)
                && o.get_size() > 1
                && unnamed(data, o.get_addr(), o.get_size())
                && only_flags(data, out)
            {
                found.push(reader);
            }
        }
    }
    Some(found)
}

/// The user ops that set processor state from their operands: interrupt
/// masks and priorities, mode and control bits, the banked stack pointers,
/// the CP15 control registers, PowerPC's `wrtee` and AArch64's `msr` to a
/// system register the language has no name for. A hint or a cache, TLB or
/// barrier op takes an address, which a returned word can be, so none of them
/// is here.
pub const STATE_USEROP_NAMES: &[&[u8]] = &[
    b"setBasePriority",
    b"enableIRQinterrupts",
    b"disableIRQinterrupts",
    b"enableFIQinterrupts",
    b"disableFIQinterrupts",
    b"enableDataAbortInterrupts",
    b"disableDataAbortInterrupts",
    b"writeCPSRControl",
    b"setThreadModePrivileged",
    b"setStackMode",
    b"setMainStackPointer",
    b"setProcessStackPointer",
    b"setMainStackPointerLimit",
    b"setProcStackPointerLimit",
    b"coproc_moveto_Control",
    b"coproc_moveto_Auxiliary_Control",
    b"coproc_moveto_Coprocessor_Access_Control",
    b"coproc_moveto_Domain_Access_Control",
    b"coproc_moveto_Translation_table_control",
    b"coproc_moveto_Context_ID",
    b"WriteExternalEnable",
    b"UnkSytemRegWrite",
];

/// The MIPS coprocessor writes, which set processor state only for
/// coprocessor 0 (`mtc0`) and the FPU's control words (`ctc1`); coprocessor 2
/// takes data.
pub const COP_USEROP_NAMES: &[&[u8]] = &[b"setCopReg", b"setCopRegH", b"setCopControlWord"];

/// Is `op` a CALLOTHER that sets processor state: a [`STATE_USEROP_NAMES`] op,
/// a [`COP_USEROP_NAMES`] write to coprocessor 0 or 1, or the volatile write
/// of a register the prototype model names nowhere (AArch64's `msr fpcr`)?
fn sets_state(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op).filter(|o| o.code() == OpCode::CPUI_CALLOTHER) else { return false };
    let input = |k: i32| o.get_in(k).and_then(|v| data.vbank().get(v));
    let constant = |k: i32| input(k).filter(|v| v.is_constant()).map(|v| v.get_offset());
    let Some(id) = constant(0) else { return false };
    let arch = data.get_arch();
    if id == crate::userop::BUILTIN_VOLATILE_WRITE as u64 {
        let (Some(dest), Some(value)) = (input(1), input(2)) else { return false };
        let register = dest.get_addr().get_space().is_some_and(|sp| sp.get_type() == spacetype::IPTR_PROCESSOR);
        return register && unnamed(data, dest.get_addr(), value.get_size());
    }
    arch.retsysreg_userops.contains(&(id as u32))
        || arch.retsysreg_cop_userops.contains(&(id as u32)) && constant(1).is_some_and(|c| c <= 1)
}

/// Does the prototype model name `addr`/`size` nowhere: no parameter or return
/// storage overlaps it, and no unaffected or killed-by-call range holds it?
fn unnamed(data: &Funcdata, addr: &Address, size: i32) -> bool {
    let proto = data.get_func_proto();
    let model = proto.model();
    proto.has_effect(addr, size) == effect_type::UNKNOWN_EFFECT
        && model.characterize_as_input_param(addr, size) == Containment::NoContainment
        && model.characterize_as_output(addr, size) == Containment::NoContainment
}

/// Is `vn`, followed forward through phis, INDIRECTs, temporaries and other
/// registers the prototype model names nowhere, read only by CALLOTHERs that
/// set processor state and by what writes a flag?
fn only_flags(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut work = vec![vn];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_WALK {
            return false;
        }
        let Some(v) = data.vbank().get(cur) else { return false };
        for reader in v.descend_iter() {
            let Some(op) = data.obank().get(reader) else { return false };
            if sets_state(data, reader) {
                continue;
            }
            let Some(out) = op.get_out() else { return false };
            let Some(o) = data.vbank().get(out) else { return false };
            let space = o.get_addr().get_space().map(|sp| sp.get_type());
            if space == Some(spacetype::IPTR_PROCESSOR) && o.get_size() <= 1 {
                continue;
            }
            let follow = space == Some(spacetype::IPTR_INTERNAL)
                || space == Some(spacetype::IPTR_PROCESSOR) && unnamed(data, o.get_addr(), o.get_size());
            if !follow || matches!(op.code(), OpCode::CPUI_CALLOTHER | OpCode::CPUI_LOAD) {
                return false;
            }
            work.push(out);
        }
    }
    true
}

/// Does `op` read any of `others`, directly or through the temporaries that
/// feed it?
fn reads_any(data: &Funcdata, op: OpId, others: &[VarnodeId]) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut work = vec![op];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_WALK {
            return true;
        }
        let Some(o) = data.obank().get(cur) else { return true };
        for vn in (0..o.num_input()).filter_map(|k| o.get_in(k)) {
            if others.contains(&vn) {
                return true;
            }
            let Some(v) = data.vbank().get(vn) else { continue };
            let temporary = v.get_addr().get_space().is_some_and(|sp| sp.get_type() == spacetype::IPTR_INTERNAL);
            if let Some(def) = v.get_def().filter(|_| temporary) {
                work.push(def);
            }
        }
    }
    false
}

/// Is `vn` computed from a call's result in `addr`/`size`: the call's clobber
/// of that register, or a result wider than it?
fn from_call_result(data: &Funcdata, vn: VarnodeId, addr: &Address, size: i32) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut work = vec![(vn, MAX_DEPTH)];
    while let Some((cur, depth)) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_WALK {
            return true;
        }
        let Some(v) = data.vbank().get(cur) else { continue };
        let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { continue };
        match op.code() {
            OpCode::CPUI_INDIRECT if op.is_indirect_creation() => {
                let overlaps = v.get_addr().overlap(0, addr, size) >= 0 || addr.overlap(0, v.get_addr(), v.get_size()) >= 0;
                if overlaps && crate::kuna_retcallhalf::created_by_call(data, op) {
                    return true;
                }
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if v.get_size() > size => return true,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {}
            _ if depth > 0 => work.extend((0..op.num_input()).filter_map(|k| op.get_in(k)).map(|i| (i, depth - 1))),
            _ => {}
        }
    }
    false
}

/// Does the output model return trial `i` with no other trial active, so that
/// it is the first register of its class rather than the second of a pair?
fn returned_alone(data: &Funcdata, active: &ParamActive, i: i32) -> bool {
    let mut probe = active.clone();
    for k in (0..probe.get_num_trials()).filter(|&k| k != i) {
        probe.get_trial_mut(k).mark_inactive();
    }
    let manager = data.get_arch().manage.clone();
    if data.get_func_proto().derive_output_map(&mut probe, &manager).is_err() {
        return true;
    }
    (0..probe.get_num_trials()).any(|k| probe.get_trial(k).is_used())
}
