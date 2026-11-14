//! (kuna `calleeargs`) Bound an x86-32 call's stack arguments by what its
//! callee's own code touches.
//!
//! An argument pushed for an outer call before an inner call sits on the stack
//! across the inner call, and nothing at the inner call site tells the two
//! apart: a `__cdecl` callee also returns with `ret`, and MSVC may defer its
//! cleanup past the next call. The callee does tell: it never touches the stack
//! above its last argument.
//!
//! The extent comes from the same walk that proves a callee's pop
//! ([`crate::p4_calls::kuna_calleepurge`]): the highest byte at or above the
//! first argument slot that a load or store touches, through the stack pointer
//! or any register holding a stack address the walk placed exactly, rounded up
//! to a whole slot. A nested direct call whose own extent is known touches its
//! arguments in the callee's frame. It is stated only when the walk saw every
//! path and placed every access: an indirect jump, a budget overrun, an access
//! through an unplaced stack address (the frame pointer the caller left, one a
//! helper such as `__SEH_prolog4` set, a stack pointer moved by a call the walk
//! cannot read, an argument address indexed at run time), or an address at or
//! above the return address that is stored, returned or live in a register at
//! a call (`va_start`, `lea ecx,[esp+8]` for a by-value `this`) states nothing.
//! A pointer into the callee's locals, however it is indexed, is taken to stay
//! among them while the walk sees it; one the walk loses (loaded back from a
//! stack slot, left by a call) may not be dereferenced, and a stack address may
//! not leave the general registers except for a slot the walk follows; a
//! local's address handed to a call is taken to stay a local's. A callee the
//! walk cannot read is taken
//! to read only the arguments pushed for it, unless it is one that moves its
//! caller's stack.
//!
//! The bound a call site uses is the extent widened to what the callee's
//! returns pop (`ret 8` takes two slots whether or not both are read) and to
//! what the caller takes off the stack right after the call (`pop ecx`,
//! `add esp,8`: a `__cdecl` cleanup covers the arguments pushed for this call).
//! At the call site (`check_input_trial_use`), a stack trial starting at or
//! past the bound (or past what the callee pops, if that is more) is no
//! argument, and one within the touched extent skips the flow-insensitive
//! local-alias test. Neither applies in a function that calls a stack-moving
//! helper, whose stack offsets the solver can only guess. A trial past the bound
//! also stays with the upstream test where a direct call whose callee states no
//! extent would take its value from past that callee's window ([`contested`]),
//! so a freed local does not become that call's stray argument. The first is a judgement:
//! an unread trailing parameter of a `__cdecl` callee whose cleanup the
//! compiler deferred past a later call is dropped with an outer call's pushes,
//! so it ships as an option.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::FxHashMap as HashMap;

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

/// The stack-argument bytes the callee `entry` of the direct call at `call_at`
/// can take, when the option is on and the walk can account for every access.
pub fn callee_stack_arg_bytes(
    arch: &Architecture,
    entry: &Address,
    call_at: &Address,
) -> Option<crate::p4_calls::kuna_calleepurge::StackArgs> {
    if !arch.kuna_calleeargs {
        return None;
    }
    crate::p4_calls::kuna_calleepurge::callee_arg_bytes(arch, entry, call_at)
}

/// For a direct callee `entry` that states no extent, the stack bytes its
/// returns pop or the caller removes after the call at `call_at`.
pub fn callee_stack_window(arch: &Architecture, entry: &Address, call_at: &Address) -> Option<u32> {
    if !arch.kuna_calleeargs {
        return None;
    }
    crate::p4_calls::kuna_calleepurge::callee_window(arch, entry, call_at)
}

/// Does the direct callee `entry` move its caller's stack pointer? The solver
/// cannot follow such a call, so the stack offsets of every call in its caller
/// are guesses, and no extent is applied there.
pub fn callee_moves_stack(arch: &Architecture, entry: &Address) -> bool {
    arch.kuna_calleeargs && crate::p4_calls::kuna_calleepurge::moves_stack(arch, entry)
}

/// Would a call in `data` whose callee states no extent take `vn`, the stored
/// value of a stack trial past the bound of the call `idx`, on some path, from
/// past what its own callee pops or its caller removes? Such a call keeps the upstream trial
/// test, which takes a pushed value no other call holds, so a value the bound
/// frees would move to it as a stray argument (a pushed cookie, a pushed local);
/// the trial then stays with `idx` and the upstream test. A call that could
/// take the value as an argument (an indirect call, one whose window covers the
/// slot) does not hold it back: there the freed push is its argument.
pub(crate) fn contested(data: &Funcdata, idx: int4, vn: VarnodeId, held: &mut Contenders) -> bool {
    let Some(off) = data.vbank().get(vn).map(|v| v.get_offset()) else { return false };
    let slots = held.slots.get_or_insert_with(|| unbounded_stack_inputs(data));
    let Some(slot) = slots.get_mut(&off) else { return false };
    let Slot { inputs, reach } = slot;
    let reach = reach.get_or_insert_with(|| reached_from(data, inputs));
    let root = stored(data, vn);
    reach.get(&root).is_some_and(|ins| ins.iter().any(|&i| inputs[i].holds(data, idx)))
}

/// What [`contested`] reads, gathered once per pass of the active-parameter
/// action rather than once per bounded call: the stack inputs of the calls
/// whose callee states no extent but a window, by slot, and, per slot asked
/// about, which of them carries each value. Within one pass a call only stops
/// scoring its trials or has an input freed, so each input is checked again
/// where it is used.
#[derive(Default)]
pub(crate) struct Contenders {
    slots: Option<HashMap<uintb, Slot>>,
}

/// The stack inputs at one slot, and which of them carries each value.
#[derive(Default)]
struct Slot {
    inputs: Vec<Held>,
    reach: Option<HashMap<VarnodeId, Vec<usize>>>,
}

/// A stack input past the window of the call `call`, at its trial `trial`.
struct Held {
    call: int4,
    trial: int4,
    slot: int4,
    window: u32,
    vn: VarnodeId,
}

impl Held {
    /// Is this still a stack input past the window of a call other than `idx`
    /// that scores its trials?
    fn holds(&self, data: &Funcdata, idx: int4) -> bool {
        if self.call == idx || self.call >= data.num_calls() {
            return false;
        }
        let fc = data.get_call_specs(self.call);
        if !fc.is_input_active() || fc.callee_stack_arg_bytes().is_some() || fc.callee_stack_window() != Some(self.window) {
            return false;
        }
        if self.trial >= fc.active_input().get_num_trials() {
            return false;
        }
        let trial = fc.active_input().get_trial(self.trial);
        trial.get_slot() == self.slot
            && trial.get_address().get_offset() as i64 >= 4 + self.window as i64
            && data.obank().get(fc.get_op()).and_then(|op| op.get_in(self.slot)) == Some(self.vn)
    }
}

/// For each value one of `inputs` carries on some path, through the INDIRECTs
/// calls put on it and the MULTIEQUALs where paths meet, the inputs that carry it.
fn reached_from(data: &Funcdata, inputs: &[Held]) -> HashMap<VarnodeId, Vec<usize>> {
    let mut reach: HashMap<VarnodeId, Vec<usize>> = HashMap::default();
    for (i, input) in inputs.iter().enumerate() {
        for v in reached(data, input.vn) {
            let ins = reach.entry(v).or_default();
            if ins.last() != Some(&i) {
                ins.push(i);
            }
        }
    }
    reach
}

/// The values the stack value `vn` carries: every one met walking back from it
/// through INDIRECT and MULTIEQUAL inputs, following at most 64.
fn reached(data: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let mut work = vec![vn];
    let mut seen: Vec<VarnodeId> = Vec::new();
    let mut met = Vec::new();
    while let Some(v) = work.pop() {
        met.push(v);
        if seen.contains(&v) || seen.len() >= 64 {
            continue;
        }
        seen.push(v);
        let Some(op) = data.vbank().get(v).and_then(|x| x.get_def()).and_then(|d| data.obank().get(d)) else { continue };
        match op.code() {
            OpCode::CPUI_INDIRECT => work.extend(op.get_in(0)),
            OpCode::CPUI_MULTIEQUAL => work.extend((0..op.num_input()).filter_map(|i| op.get_in(i))),
            _ => {}
        }
    }
    met
}

/// The stack inputs past the window of every call that still scores its
/// trials and whose callee states no extent but a window, by slot.
fn unbounded_stack_inputs(data: &Funcdata) -> HashMap<uintb, Slot> {
    let mut slots: HashMap<uintb, Slot> = HashMap::default();
    for j in 0..data.num_calls() {
        let fc = data.get_call_specs(j);
        if !fc.is_input_active() || fc.callee_stack_arg_bytes().is_some() {
            continue;
        }
        let Some(window) = fc.callee_stack_window() else { continue };
        let Some(op) = data.obank().get(fc.get_op()) else { continue };
        for t in 0..fc.active_input().get_num_trials() {
            let trial = fc.active_input().get_trial(t);
            if (trial.get_address().get_offset() as i64) < 4 + window as i64 {
                continue;
            }
            let Some(vn) = op.get_in(trial.get_slot()) else { continue };
            let Some(v) = data.vbank().get(vn) else { continue };
            if v.get_space().get_type() == spacetype::IPTR_SPACEBASE {
                let held = Held { call: j, trial: t, slot: trial.get_slot(), window, vn };
                slots.entry(v.get_offset()).or_default().inputs.push(held);
            }
        }
    }
    slots
}

/// The stored value `vn` carries, past the INDIRECTs calls put on it.
fn stored(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    for _ in 0..64 {
        let Some(op) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { break };
        if op.code() != OpCode::CPUI_INDIRECT {
            break;
        }
        let Some(input) = op.get_in(0) else { break };
        vn = input;
    }
    vn
}

#[cfg(test)]
mod tests;
