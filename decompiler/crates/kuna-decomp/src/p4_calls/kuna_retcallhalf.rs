//! (kuna) Keep the first register of a returned pair when upstream's scoring
//! refused it and the function computes the second.
//!
//! `u64 f(unsigned a) { return full(a) | 0xff00000000ULL; }` is, on 32-bit ARM,
//! `bl full; orr r1,r1,#255; bx lr`: `r0` comes back from `full` as it is and
//! only `r1` is written. Return recovery scores each return register on its
//! own. `r1` passes, but `r0` at the RETURN is the call's INDIRECT creation,
//! which upstream's `ancestorOpUse` refuses outright, and the output model
//! never returns the second register of a pair without the first. The function
//! printed as `void f(int a0) { full(a0); }`.
//!
//! Two more first halves are refused the same way. `long long f(int a) {
//! return a + 3; }` is `add r0,r0,#3; asr r1,r0,#31`: `r0` is read by the
//! RETURN and by the shift that makes `r1`, and `onlyOpUse` takes the second
//! read for a competing use. `u64 f(unsigned a) { return a; }` is `mov r1,#0;
//! bx lr`: `r0` is the caller's own value passing through, which ancestor
//! realism refuses. Both printed as `void f(void)`.
//!
//! A register that is not the first of its class is never a return value of
//! its own, so a function that computes one and hands it only to the RETURN
//! returns the pair it belongs to. [`accept`] makes the first register's trial
//! active when, at every live RETURN, its value is the untouched result of a
//! call ([`is_call_result`]), the function's own argument in that register
//! ([`is_own_input`]), or a value the function produced (a constant, a call's
//! result, anything but a clobber), and at one RETURN at least it is one of
//! the first two or a value read only by the RETURN and by what turns it into
//! the second register ([`feeds_second`]).
//!
//! The second register has to have been computed on purpose
//! ([`computed_on_purpose`]), because four kinds of write reach a RETURN in a
//! function that returns one register or none:
//!
//! * the other output of an instruction that writes two registers, such as
//!   the low word of an ARM `smull` whose high word is a division by a
//!   constant;
//! * a restore from the frame, such as gcc's `pop {r1,r2,pc}` that releases
//!   stack space, or an `-O0` reload of a spilled argument;
//! * an argument set up for a later call or `svc` whose p-code does not show
//!   it reading the register;
//! * the zero `-fzero-call-used-regs` leaves in every call-used register a
//!   function does not return in.
//!
//! Beside the argument left in place, where the second register is the only
//! evidence, it must also be set by the function itself on every path
//! ([`set_by_function`]): a load does not count, since a volatile read whose
//! value is dropped leaves it there. A zero counts only there and only when the
//! function names the register nowhere else ([`only_read_as_zero`]): the
//! scrub zeroes a register the body used, or every call-used register, which
//! would scrub the first register of a function that returns nothing.
//!
//! Wherever the pair rests on anything but a call's untouched result, the
//! second register's value must also be read by nothing but the RETURN
//! ([`read_only_by_returns`]): `mov r1,#0x3000000; vmsr fpscr,r1; bx lr` sets
//! `r1` for the system register and returns nothing.
//!
//! The rest stays as upstream decides it. The model must return nothing
//! without the first register and exactly the two registers with it, so a
//! value the function returns in another class is never joined to a leftover.
//! A function that writes the first register and leaves the second untouched
//! is not changed: `bl g; orr r0,r0,#255` is byte for byte `int f(void) {
//! return (int)g() | 255; }` as well as a 64-bit return. Nor is one whose
//! second register feeds the first (`asr r1,r0,#2; add r0,r1,r0,lsr #31` is
//! an `int` division by a constant). And a big-endian pair is left alone,
//! because a pair is still joined in little-endian order there.
//!
//! # A pair the function names only in part
//!
//! gcc's i386 code pushes a call's argument, so `u64 f(unsigned a) { return
//! full(a) | 0x1234500000000ULL; }` is `push 12(%esp); call full; add
//! $12,%esp; or $0x12345,%edx; ret`, which names `%eax` nowhere. Heritage
//! registers a return trial only for a range some op reads or writes, so there
//! was no first register to accept and the function printed `void`. A byte
//! write is narrower still: `or $0xff,%dl` makes a one-byte `DL` trial that the
//! model joined with `EAX` into five bytes, and the `DH` of `or $0xff,%dh` is a
//! byte the join cannot place at all. Before heritage, [`plant`] gives every
//! RETURN a read of the pair register the function leaves unnamed or names in
//! part, when the last write before each RETURN, back to a direct call, is to
//! the second register of a join entry and the first is named nowhere.
//!
//! Upstream's scoring follows a PIECE through its low part only, so a byte
//! above the low one leaves the second register's trial looking at the call's
//! untouched low byte. [`accept_pieced`] takes such a pair when the second
//! register is a call's result with some bytes replaced by values computed on
//! purpose and the first is the call's untouched result.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;

/// How many move-only ops and phis the walks follow.
const MAX_DEPTH: u32 = 8;

/// How many defining ops the zero test follows.
const ZERO_DEPTH: u32 = 8;

/// What the first register of a pair holds at one RETURN, when upstream's
/// scoring refused it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum First {
    /// A call's untouched result.
    CallResult,
    /// The function's own argument in that register, untouched.
    OwnInput,
    /// A value read only by the RETURN and by what turns it into the second
    /// register there.
    FeedsSecond,
    /// A value the function produced, with no sign at this RETURN that the
    /// second register belongs to it.
    Value,
}

/// Mark active the inactive return trial that, beside a register the function
/// computes, completes the pair the output model returns: a call's untouched
/// result, the function's own argument left in place, or a value the function
/// computes and also turns into the second register. A pair whose second
/// register is a call's result with some bytes rewritten is taken first, both
/// trials at once ([`accept_pieced`]).
///
/// Runs once `ActionReturnRecovery` has scored every trial for the last time,
/// before the output map is derived. Returns the first register's storage when
/// the pair was accepted with the argument left in place at some RETURN, which
/// the late pair repair must then keep
/// ([`crate::kuna_retinputhalf::is_moved_back`]).
pub fn accept(data: &Funcdata, active: &mut ParamActive, return_ops: &[OpId]) -> Option<(Address, i32)> {
    let active_trials: Vec<i32> = (0..active.get_num_trials()).filter(|&i| active.get_trial(i).is_active()).collect();
    let rets: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    if accept_pieced(data, active, &rets) {
        return None;
    }
    let [second] = active_trials[..] else { return None };
    let value_at = |r: OpId, slot: i32| data.obank().get(r).and_then(|o| o.get_in(slot));
    let (second_addr, second_size, second_slot) = {
        let t = active.get_trial(second);
        (t.get_address().clone(), t.get_size(), t.get_slot())
    };
    let values: Vec<(OpId, VarnodeId)> =
        rets.iter().filter_map(|&r| value_at(r, second_slot).map(|vn| (r, vn))).collect();
    if rets.is_empty() || values.len() != rets.len() || derives_any(data, active) {
        return None;
    }
    let zero = values.iter().all(|&(_, vn)| is_zero(data, vn, ZERO_DEPTH));
    let kinds_of = |t: &crate::fspec::ParamTrial| -> Option<Vec<First>> {
        rets.iter()
            .map(|&r| {
                let vn = value_at(r, t.get_slot())?;
                first_kind(data, vn, t.get_address(), t.get_size(), r, t.get_slot(), second_slot)
            })
            .collect()
    };
    let candidates: Vec<(i32, bool)> = (0..active.get_num_trials())
        .filter_map(|i| {
            let t = active.get_trial(i);
            if t.is_active()
                || t.is_definitely_not_used()
                || t.get_size() != second_size
                || t.get_address().is_big_endian()
            {
                return None;
            }
            let kinds = kinds_of(t)?;
            if kinds.iter().all(|&k| k == First::Value)
                || kinds.iter().zip(&values).any(|(&k, &(_, vn))| {
                    k == First::OwnInput && !set_by_function(data, vn, &second_addr, second_size)
                })
                || !kinds.iter().all(|&k| k == First::CallResult)
                    && !values.iter().all(|&(_, vn)| read_only_by_returns(data, vn))
            {
                return None;
            }
            let own = kinds.contains(&First::OwnInput);
            let unscrubbed = || {
                kinds.iter().all(|&k| k == First::OwnInput)
                    && t.get_size() + second_size <= 8
                    && only_read_as_zero(data, &second_addr, second_size, &values)
            };
            (!zero || unscrubbed()).then_some((i, own))
        })
        .collect();
    if candidates.is_empty() || !values.iter().all(|&(r, vn)| computed_on_purpose(data, vn, r, MAX_DEPTH))
    {
        return None;
    }
    let manager = data.get_arch().manage.clone();
    for (i, own) in candidates {
        let first_addr = active.get_trial(i).get_address().clone();
        let first_size = active.get_trial(i).get_size();
        let mut probe = active.clone();
        probe.get_trial_mut(i).mark_active();
        if data.get_func_proto().derive_output_map(&mut probe, &manager).is_err() {
            continue;
        }
        let used: Vec<Address> = (0..probe.get_num_trials())
            .map(|k| probe.get_trial(k))
            .filter(|t| t.is_used())
            .map(|t| t.get_address().clone())
            .collect();
        if used == [first_addr.clone(), second_addr.clone()] {
            active.get_trial_mut(i).mark_active();
            return own.then_some((first_addr, first_size));
        }
    }
    None
}

/// Classify the first register's value `vn` at the RETURN `ret`, or `None`
/// when some value it merges is not one the function produced or was handed.
fn first_kind(
    data: &Funcdata,
    vn: VarnodeId,
    addr: &Address,
    size: i32,
    ret: OpId,
    first_slot: i32,
    second_slot: i32,
) -> Option<First> {
    if is_call_result(data, vn, MAX_DEPTH) {
        return Some(First::CallResult);
    }
    let roots = roots(data, vn)?;
    let own = |r: VarnodeId| is_own_input(data, r, addr, size);
    if roots.iter().all(|&r| own(r)) {
        return Some(First::OwnInput);
    }
    if !roots.iter().all(|&r| own(r) || produced(data, r)) {
        return None;
    }
    let feeds = |v: VarnodeId| feeds_second(data, v, ret, first_slot, second_slot);
    let computed: Vec<VarnodeId> =
        roots.iter().copied().filter(|&r| !own(r) && data.vbank().get(r).is_some_and(|v| !v.is_constant())).collect();
    let fed = feeds(below_noops(data, vn)) || roots.len() > 1 && !computed.is_empty() && computed.iter().all(|&r| feeds(r));
    Some(if fed { First::FeedsSecond } else { First::Value })
}

/// `vn` without the injected no-ops of a return's mode switch above it.
fn below_noops(data: &Funcdata, vn: VarnodeId) -> VarnodeId {
    let mut base = vn;
    while let Some(d) = data.vbank().get(base).and_then(|v| v.get_def()) {
        if !crate::kuna_passthrough::is_injected_noop(data, d) {
            break;
        }
        let Some(src) = data.obank().get(d).and_then(|o| o.get_in(0)) else { break };
        base = src;
    }
    base
}

/// The values `vn` merges: walk back through phis, value-preserving INDIRECTs
/// and the injected no-op of a return's mode switch.
fn roots(data: &Funcdata, vn: VarnodeId) -> Option<Vec<VarnodeId>> {
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut work = vec![vn];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > 32 {
            return None;
        }
        let def = data.vbank().get(cur)?.get_def();
        let Some(op) = def.and_then(|d| data.obank().get(d)) else {
            out.push(cur);
            continue;
        };
        match op.code() {
            OpCode::CPUI_MULTIEQUAL => work.extend((0..op.num_input()).filter_map(|k| op.get_in(k))),
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => work.extend(op.get_in(0)),
            OpCode::CPUI_COPY if def.is_some_and(|d| crate::kuna_passthrough::is_injected_noop(data, d)) => {
                work.extend(op.get_in(0))
            }
            _ => out.push(cur),
        }
    }
    Some(out)
}

/// Did the function set `vn`, the second register's value stored at
/// `addr`/`size`, itself on every path: a constant, a computation, or a move
/// from another register? A load does not count, since a volatile read whose
/// value is dropped leaves it there, and neither does the caller's own value
/// passing through.
fn set_by_function(data: &Funcdata, vn: VarnodeId, addr: &Address, size: i32) -> bool {
    let Some(roots) = roots(data, vn) else { return false };
    roots.into_iter().all(|root| {
        let mut cur = root;
        let mut moved = false;
        for _ in 0..MAX_DEPTH {
            let Some(v) = data.vbank().get(cur) else { return false };
            if v.is_constant() {
                return true;
            }
            let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else {
                return moved && (v.get_addr() != addr || v.get_size() != size);
            };
            match op.code() {
                OpCode::CPUI_COPY => {
                    let Some(src) = op.get_in(0) else { return false };
                    moved = true;
                    cur = src;
                }
                OpCode::CPUI_LOAD
                | OpCode::CPUI_INDIRECT
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER => return false,
                _ => return true,
            }
        }
        false
    })
}

/// Is every value `vn` merges read only on its way to a RETURN: through phis,
/// INDIRECTs, the injected no-op of a return's mode switch and temporaries,
/// or into a flag?
///
/// A register the function hands to anything else, such as the operand of a
/// `vmsr fpscr` or `msr cpsr_c`, a store or another register, was set for that
/// use, so its presence at the RETURN is no sign of a returned high word.
fn read_only_by_returns(data: &Funcdata, vn: VarnodeId) -> bool {
    read_only_on_the_way(data, vn, false)
}

/// [`read_only_by_returns`], also following a PIECE that joins the value with
/// other bytes of its register when `pieces` is set.
fn read_only_on_the_way(data: &Funcdata, vn: VarnodeId, pieces: bool) -> bool {
    let Some(roots) = roots(data, vn) else { return false };
    let mut seen = std::collections::BTreeSet::new();
    let mut work = roots;
    work.push(vn);
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > 64 {
            return false;
        }
        let Some(v) = data.vbank().get(cur) else { return false };
        for reader in v.descend_iter() {
            let Some(op) = data.obank().get(reader) else { return false };
            if op.code() == OpCode::CPUI_RETURN {
                continue;
            }
            let Some(out) = op.get_out() else { return false };
            let Some(o) = data.vbank().get(out) else { return false };
            let space = o.get_addr().get_space().map(|sp| sp.get_type());
            if matches!(op.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT)
                || pieces && op.code() == OpCode::CPUI_PIECE
                || crate::kuna_passthrough::is_injected_noop(data, reader)
                || space == Some(spacetype::IPTR_INTERNAL)
            {
                work.push(out);
            } else if space != Some(spacetype::IPTR_PROCESSOR) || o.get_size() > 1 {
                return false;
            }
        }
    }
    true
}

/// Did the function produce `vn`: a constant, a call's result, or the output
/// of an operation other than a CALLOTHER or another clobber?
fn produced(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return true;
    }
    let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
    match op.code() {
        OpCode::CPUI_INDIRECT => op.is_indirect_creation() && created_by_call(data, op),
        OpCode::CPUI_CALLOTHER => false,
        _ => true,
    }
}

/// Is `vn` the function's own argument at `addr`/`size`, never written?
fn is_own_input(data: &Funcdata, vn: VarnodeId, addr: &Address, size: i32) -> bool {
    data.vbank().get(vn).is_some_and(|v| v.get_addr() == addr && v.get_size() == size)
        && crate::kuna_retinputhalf::is_input_parameter(data, vn)
}

/// Is `vn`, a value the first register holds at `ret`, read only on its way
/// to `ret` and turned there into the second register as well?
///
/// Every reader, followed forward, must be an operation that only computes or
/// merges: no branch, call, CALLOTHER, load, store, INDIRECT or other RETURN,
/// and nothing it writes may be global. The walk must reach `ret` in the
/// second register's slot, and may reach it in no slot but the two.
fn feeds_second(data: &Funcdata, vn: VarnodeId, ret: OpId, first_slot: i32, second_slot: i32) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut work = vec![vn];
    let mut second = false;
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > 64 {
            return false;
        }
        let Some(v) = data.vbank().get(cur) else { return false };
        for reader in v.descend_iter() {
            let Some(op) = data.obank().get(reader) else { return false };
            if reader == ret {
                for slot in (0..op.num_input()).filter(|&k| op.get_in(k) == Some(cur)) {
                    if slot == second_slot {
                        second = true;
                    } else if slot != first_slot {
                        return false;
                    }
                }
                continue;
            }
            match op.code() {
                OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_LOAD
                | OpCode::CPUI_STORE
                | OpCode::CPUI_NEW
                | OpCode::CPUI_INDIRECT => return false,
                _ => {}
            }
            let Some(out) = op.get_out() else { return false };
            if data.vbank().get(out).is_none_or(|o| o.is_persist()) {
                return false;
            }
            work.push(out);
        }
    }
    second
}

/// Is the second register, stored at `addr`/`size`, read nowhere in the
/// function but as the zeros `values` the RETURNs read?
///
/// `-fzero-call-used-regs` zeroes a call-used register the function body uses,
/// or every one of them; a register the body never names is zeroed only when
/// the function returns it as a high word.
fn only_read_as_zero(data: &Funcdata, addr: &Address, size: i32, values: &[(OpId, VarnodeId)]) -> bool {
    let Some(space) = addr.get_space() else { return false };
    let mut zero_chain = std::collections::BTreeSet::new();
    let mut work: Vec<VarnodeId> = values.iter().map(|&(_, v)| v).collect();
    while let Some(cur) = work.pop() {
        if !zero_chain.insert(cur) {
            continue;
        }
        let Some(op) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            continue;
        };
        if matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL) {
            work.extend((0..op.num_input()).filter_map(|k| op.get_in(k)));
        }
    }
    let start = Address::new(Rc::clone(space), addr.get_offset().saturating_sub(8));
    let end = Address::new(Rc::clone(space), addr.get_offset() + size as u64);
    data.vbank().iter_loc_addr_range(&start, &end).all(|id| {
        let Some(v) = data.vbank().get(id) else { return true };
        let overlaps = v.get_addr().overlap(0, addr, size) >= 0 || addr.overlap(0, v.get_addr(), v.get_size()) >= 0;
        !overlaps || zero_chain.contains(&id) || v.has_no_descend()
    })
}

/// Mark active a pair whose second register is, at every live RETURN, a
/// call's result with some of its bytes replaced ([`pieced_call_result`]) and
/// whose first register is a call's untouched result there, when the model
/// returns exactly those two registers together.
///
/// Upstream's `ancestorOpUse` follows a PIECE only through its low part, so
/// gcc's `call full; or $0xff,%dh; ret` leaves the `EDX` trial inactive: its
/// low byte is `full`'s, untouched.
fn accept_pieced(data: &Funcdata, active: &mut ParamActive, rets: &[OpId]) -> bool {
    if rets.is_empty() {
        return false;
    }
    let value_at = |r: OpId, slot: i32| data.obank().get(r).and_then(|o| o.get_in(slot));
    let trial_at = |active: &ParamActive, (addr, size): &(Address, i32)| {
        (0..active.get_num_trials()).find(|&i| {
            let t = active.get_trial(i);
            t.get_address() == addr && t.get_size() == *size && !t.is_definitely_not_used()
        })
    };
    let manager = data.get_arch().manage.clone();
    for (first, second) in join_pairs(data) {
        let (Some(f), Some(s)) = (trial_at(active, &first), trial_at(active, &second)) else { continue };
        let (fslot, sslot) = (active.get_trial(f).get_slot(), active.get_trial(s).get_slot());
        if active.get_trial(s).is_active()
            || !rets.iter().all(|&r| value_at(r, sslot).is_some_and(|vn| pieced_call_result(data, vn, r)))
            || !rets.iter().all(|&r| value_at(r, fslot).is_some_and(|vn| is_call_result(data, vn, MAX_DEPTH)))
        {
            continue;
        }
        let mut probe = active.clone();
        probe.get_trial_mut(f).mark_active();
        probe.get_trial_mut(s).mark_active();
        if data.get_func_proto().derive_output_map(&mut probe, &manager).is_err() {
            continue;
        }
        let used: Vec<Address> = (0..probe.get_num_trials())
            .map(|k| probe.get_trial(k))
            .filter(|t| t.is_used())
            .map(|t| t.get_address().clone())
            .collect();
        if used == [first.0, second.0] {
            active.get_trial_mut(f).mark_active();
            active.get_trial_mut(s).mark_active();
            return true;
        }
    }
    false
}

/// Is `vn`, read by the RETURN `ret`, a PIECE whose parts are each a call's
/// untouched result (or a byte range of it) or a value the function computed
/// on purpose and reads only on its way to a RETURN, at least one of them the
/// latter?
fn pieced_call_result(data: &Funcdata, vn: VarnodeId, ret: OpId) -> bool {
    let def = |v: VarnodeId| data.vbank().get(v).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    let Some(whole) = data.vbank().get(vn).filter(|_| def(vn).is_some_and(|o| o.code() == OpCode::CPUI_PIECE)) else {
        return false;
    };
    let mut computed = false;
    let mut work = vec![vn];
    let mut steps = 0;
    while let Some(cur) = work.pop() {
        steps += 1;
        if steps > 16 {
            return false;
        }
        let op = def(cur);
        match op.map(|o| o.code()) {
            Some(OpCode::CPUI_PIECE) => work.extend(op.into_iter().flat_map(|o| [o.get_in(0), o.get_in(1)]).flatten()),
            Some(OpCode::CPUI_SUBPIECE) if op.and_then(|o| o.get_in(0)).is_some_and(|i| is_call_result(data, i, MAX_DEPTH)) => {}
            _ if is_call_result(data, cur, MAX_DEPTH) => {}
            _ => {
                let constant = data.vbank().get(cur).is_some_and(|v| v.is_constant());
                if !constant
                    && (!computed_within(data, cur, ret, MAX_DEPTH, (whole.get_addr(), whole.get_size()))
                        || !read_only_on_the_way(data, cur, true))
                {
                    return false;
                }
                computed = true;
            }
        }
    }
    computed
}

/// Does the output model return anything from the trials as they stand?
fn derives_any(data: &Funcdata, active: &ParamActive) -> bool {
    let mut probe = active.clone();
    let manager = data.get_arch().manage.clone();
    if data.get_func_proto().derive_output_map(&mut probe, &manager).is_err() {
        return true;
    }
    (0..probe.get_num_trials()).any(|k| probe.get_trial(k).is_used())
}

/// Is `vn` the value a call left in its register, untouched since: the call's
/// INDIRECT creation, read directly, through the injected no-op of a return's
/// mode switch, or through a phi of such values?
fn is_call_result(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    if depth == 0 {
        return false;
    }
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    match op.code() {
        OpCode::CPUI_COPY if crate::kuna_passthrough::is_injected_noop(data, def) => {
            op.get_in(0).is_some_and(|i| is_call_result(data, i, depth - 1))
        }
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input())
            .all(|k| op.get_in(k).is_some_and(|i| i != vn && is_call_result(data, i, depth - 1))),
        OpCode::CPUI_INDIRECT => op.is_indirect_creation() && created_by_call(data, op),
        _ => false,
    }
}

/// Is the INDIRECT creation `op` a call's clobber of its output?
fn created_by_call(data: &Funcdata, op: &crate::op::PcodeOp) -> bool {
    let Some(iop) = op.get_in(1).and_then(|i| data.vbank().get(i)) else { return false };
    let call = OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()));
    data.obank().get(call).is_some_and(|c| matches!(c.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
}

/// Was `vn`, read by the RETURN `ret`, written by an instruction of the
/// function on purpose, on every path?
///
/// Every producing instruction must write no register but its own and not the
/// stack pointer, must not reload the value from the frame, and must not be
/// followed on the way to `ret` by a call or a CALLOTHER, which can read the
/// register as an argument its p-code does not show. A call's result merged in
/// on one path is a value too. An unwritten Varnode counts only when it is not
/// a frame slot.
fn computed_on_purpose(data: &Funcdata, vn: VarnodeId, ret: OpId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    computed_within(data, vn, ret, depth, (v.get_addr(), v.get_size()))
}

/// [`computed_on_purpose`], where a producing instruction may write anywhere
/// in the register `within`, not just the bytes `vn` holds: the write of `DH`
/// that heritage splits out of, and joins back into, `EDX`.
fn computed_within(data: &Funcdata, vn: VarnodeId, ret: OpId, depth: u32, within: (&Address, i32)) -> bool {
    if depth == 0 {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return true;
    }
    let Some(def) = v.get_def() else { return !in_frame(v.get_addr()) };
    let Some(op) = data.obank().get(def) else { return false };
    match op.code() {
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).all(|k| {
            op.get_in(k).is_some_and(|i| {
                i != vn && (is_call_result(data, i, depth - 1) || computed_on_purpose(data, i, ret, depth - 1))
            })
        }),
        OpCode::CPUI_INDIRECT if op.is_indirect_creation() => false,
        OpCode::CPUI_INDIRECT => op.get_in(0).is_some_and(|i| computed_on_purpose(data, i, ret, depth - 1)),
        _ if writes_elsewhere(data, op.get_addr(), within.0, within.1) => false,
        _ if crate::kuna_retinputhalf::call_between(data, def, ret) => false,
        OpCode::CPUI_COPY => op.get_in(0).and_then(|i| data.vbank().get(i)).is_some_and(|i| !in_frame(i.get_addr())),
        OpCode::CPUI_LOAD => op.get_in(1).is_some_and(|p| !frame_pointer(data, p, MAX_DEPTH)),
        _ => true,
    }
}

/// Does the instruction at `pc` write the stack pointer, or a register wider
/// than a flag outside `[addr, addr+size)`?
fn writes_elsewhere(data: &Funcdata, pc: &Address, addr: &Address, size: i32) -> bool {
    if crate::kuna_retinputhalf::writes_stack_pointer(data, pc) {
        return true;
    }
    data.obank().iter_at(pc).any(|(_, op)| {
        let Some(out) = data.obank().get(op).and_then(|o| o.get_out()).and_then(|o| data.vbank().get(o)) else {
            return false;
        };
        let heritage = data.obank().get(op).is_some_and(|o| matches!(o.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT));
        if heritage
            || is_tracked_entry_value(data, op)
            || out.has_no_descend()
            || crate::kuna_passthrough::is_injected_noop(data, op)
        {
            return false;
        }
        let a = out.get_addr();
        let register = a.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR);
        register && out.get_size() > 1 && a.overlap(0, addr, size) < 0 && addr.overlap(0, a, out.get_size()) < 0
    })
}

/// Is `op` the COPY of a tracked register's known value that `ActionConstbase`
/// puts at the function's entry, ahead of the first instruction's own ops?
fn is_tracked_entry_value(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    if o.code() != OpCode::CPUI_COPY || o.get_addr() != data.get_address() {
        return false;
    }
    let constant = o.get_in(0).and_then(|i| data.vbank().get(i)).is_some_and(|i| i.is_constant());
    let Some(out) = o.get_out().and_then(|v| data.vbank().get(v)) else { return false };
    let (addr, size) = (out.get_addr(), out.get_size());
    constant
        && data.get_arch().get_tracked_set(data.get_address()).iter().any(|t| {
            t.loc.space.as_ref().is_some_and(|s| addr.get_space().is_some_and(|a| a.get_index() == s.get_index()))
                && t.loc.offset == addr.get_offset()
                && t.loc.size as i32 == size
        })
}

/// Is `addr` in the function's stack frame?
fn in_frame(addr: &Address) -> bool {
    addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
}

/// Is the pointer `vn` the stack pointer, or the stack pointer plus or minus
/// a constant?
fn frame_pointer(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_spacebase() || is_stack_pointer(data, v.get_addr()) {
        return true;
    }
    let Some(op) = v.get_def().and_then(|d| data.obank().get(d)).filter(|_| depth > 0) else { return false };
    match op.code() {
        OpCode::CPUI_COPY
        | OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_PTRSUB
        | OpCode::CPUI_PTRADD
        | OpCode::CPUI_INDIRECT => op.get_in(0).is_some_and(|i| frame_pointer(data, i, depth - 1)),
        _ => false,
    }
}

/// Is `addr` the stack pointer register?
fn is_stack_pointer(data: &Funcdata, addr: &Address) -> bool {
    let Some(stack) = data.get_arch().manage().get_stack_space().cloned() else { return false };
    let Ok(sp) = stack.get_spacebase(0) else { return false };
    sp.space.as_ref().is_some_and(|s| addr.get_space().is_some_and(|a| a.get_index() == s.get_index()))
        && addr.get_offset() == sp.offset
}

/// Is `vn` zero whatever the function's inputs: a zero constant, `x ^ x` or
/// `x - x`, or a copy, extension, piece or concatenation of zeros?
fn is_zero(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return v.get_offset() == 0;
    }
    let Some(o) = v.get_def().filter(|_| depth > 0).and_then(|d| data.obank().get(d)) else { return false };
    let zero = |k: i32| o.get_in(k).is_some_and(|i| is_zero(data, i, depth - 1));
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => zero(0),
        OpCode::CPUI_SUBPIECE => zero(0) || piece_of_zero(data, o, depth - 1),
        OpCode::CPUI_PIECE | OpCode::CPUI_INT_OR => zero(0) && zero(1),
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB => {
            same_value(data, o.get_in(0), o.get_in(1)) || (zero(0) && zero(1))
        }
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_MULT => zero(0) || zero(1),
        _ => false,
    }
}

/// Is the SUBPIECE `o` exactly one half of a PIECE, and that half zero?
fn piece_of_zero(data: &Funcdata, o: &crate::op::PcodeOp, depth: u32) -> bool {
    let size = |v: Option<VarnodeId>| v.and_then(|v| data.vbank().get(v)).map(|v| v.get_size());
    let Some(whole) = o.get_in(0).and_then(|w| data.vbank().get(w)).and_then(|w| w.get_def()) else { return false };
    let Some(p) = data.obank().get(whole).filter(|p| p.code() == OpCode::CPUI_PIECE) else { return false };
    let off = o.get_in(1).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset() as i32);
    let out = size(o.get_out());
    let (hi, lo) = (p.get_in(0), p.get_in(1));
    match off {
        Some(0) if out == size(lo) => lo.is_some_and(|l| is_zero(data, l, depth)),
        Some(k) if Some(k) == size(lo) && out == size(hi) => hi.is_some_and(|h| is_zero(data, h, depth)),
        _ => false,
    }
}

/// Are `a` and `b` the same value: one Varnode, or the same piece of one?
fn same_value(data: &Funcdata, a: Option<VarnodeId>, b: Option<VarnodeId>) -> bool {
    let (Some(a), Some(b)) = (a, b) else { return false };
    if a == b {
        return true;
    }
    let def = |v: VarnodeId| data.vbank().get(v).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    let (Some(x), Some(y)) = (def(a), def(b)) else { return false };
    let offset = |o: &crate::op::PcodeOp| o.get_in(1).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset());
    x.code() == OpCode::CPUI_SUBPIECE
        && y.code() == OpCode::CPUI_SUBPIECE
        && x.get_in(0) == y.get_in(0)
        && offset(x) == offset(y)
}

/// How many single-predecessor blocks [`plant`] walks back from a RETURN, as
/// far as the `passthrough` tail-call walk goes.
const TAIL_BLOCKS: usize = 4;

/// Give every live RETURN a read of each register of a returned pair that the
/// function names only in part or not at all, so heritage gives the function
/// a return trial for the whole register: the first register when nothing
/// names it, the second when nothing names all of it.
///
/// Runs at the end of `ActionFuncLink`, before the first heritage. Every live
/// RETURN must be reached from a direct call along single-predecessor blocks,
/// with an instruction in between that writes a byte of the same join entry's
/// second register and not its first ([`tail_pair`]), and the first register
/// may be read only by a RETURN another rule planted.
pub fn plant(data: &mut Funcdata) {
    if data.num_calls() == 0
        || data.get_active_output().is_none()
        || !data.get_func_proto().has_model()
        || data.get_func_proto().is_output_locked()
    {
        return;
    }
    let rets: Vec<OpId> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    let pairs = join_pairs(data);
    if rets.is_empty() || pairs.is_empty() {
        return;
    }
    let mut pair: Option<usize> = None;
    for &r in &rets {
        let Some(found) = tail_pair(data, &pairs, r) else { return };
        if pair.is_some_and(|p| p != found) {
            return;
        }
        pair = Some(found);
    }
    let Some(((first_addr, first_size), (second_addr, second_size))) = pair.map(|p| pairs[p].clone()) else { return };
    if first_size != second_size || !only_returns_read(data, &first_addr, first_size) {
        return;
    }
    let plant_first = !crate::p4_calls::kuna_passthrough::touched(data, &first_addr, first_size);
    let plant_second = !named_whole(data, &second_addr, second_size);
    if plant_first {
        crate::p4_calls::kuna_voidret::plant_piece(data, first_addr, first_size);
    }
    if plant_second {
        crate::p4_calls::kuna_voidret::plant_piece(data, second_addr, second_size);
    }
}

/// The register pairs the output model joins into one value, as `(first,
/// second)`: the two pieces of a general join entry, low piece first. A
/// big-endian pair is left out, as everywhere in this module, and so is a pair
/// the model forms by rule from two register entries (x86-64 `RAX`, `RDX`).
fn join_pairs(data: &Funcdata) -> Vec<((Address, i32), (Address, i32))> {
    let Some(list) = data.get_func_proto().model().output_list() else { return Vec::new() };
    let mut pairs = Vec::new();
    for e in list.get_entry() {
        if e.get_type() != crate::dtype::type_class::TYPECLASS_GENERAL {
            continue;
        }
        let Some(jr) = e.get_join_record().filter(|jr| jr.num_pieces() == 2) else { continue };
        let piece = |i| {
            let p = jr.get_piece(i);
            (p.get_addr(), p.size as i32)
        };
        let pair = (piece(1), piece(0));
        if !pair.0 .0.is_big_endian() && !pairs.contains(&pair) {
            pairs.push(pair);
        }
    }
    pairs
}

/// Which of `pairs` the last instruction before `ret` that writes a byte of
/// one of them writes the second register of and not the first, on the path
/// back to the direct CALL before it. `None` when the walk meets a CALLIND, a
/// CALLOTHER or a merge first, or when that instruction also moves the stack
/// pointer (gcc's `pop %edx` releasing an argument slot).
fn tail_pair(data: &Funcdata, pairs: &[((Address, i32), (Address, i32))], ret: OpId) -> Option<usize> {
    let mut found = None;
    let mut bl = data.obank().get(ret)?.get_parent()?;
    let mut cur = data.op_previous_op(ret);
    for _ in 0..TAIL_BLOCKS {
        while let Some(op) = cur {
            let o = data.obank().get(op)?;
            match o.code() {
                OpCode::CPUI_CALL => return found,
                OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => return None,
                _ => {}
            }
            let out = o.get_out().and_then(|v| data.vbank().get(v));
            if let Some(out) = out.filter(|_| found.is_none()) {
                let hits = |(a, s): &(Address, i32)| {
                    a.overlap(0, out.get_addr(), out.get_size()) >= 0 || out.get_addr().overlap(0, a, *s) >= 0
                };
                if let Some(k) = pairs.iter().position(|(first, second)| hits(second) && !hits(first)) {
                    if crate::kuna_retinputhalf::writes_stack_pointer(data, o.get_addr()) {
                        return None;
                    }
                    found = Some(k);
                }
            }
            cur = data.op_previous_op(op);
        }
        let b = data.bblocks_ref().block(bl);
        if b.size_in() != 1 {
            return None;
        }
        bl = b.get_in(0);
        cur = data.bb_op_tail(bl);
    }
    None
}

/// Is every Varnode at `addr`/`size` an unwritten read by a RETURN alone: none
/// at all, or the read another rule planted there?
fn only_returns_read(data: &Funcdata, addr: &Address, size: i32) -> bool {
    overlapping(data, addr, size).into_iter().all(|id| {
        data.vbank().get(id).is_some_and(|v| {
            v.get_def().is_none()
                && v.descend_iter().all(|r| data.obank().get(r).is_some_and(|o| o.code() == OpCode::CPUI_RETURN))
        })
    })
}

/// Does some Varnode name all of `addr`/`size`?
fn named_whole(data: &Funcdata, addr: &Address, size: i32) -> bool {
    let (off, end) = (addr.get_offset(), addr.get_offset().wrapping_add(size as u64));
    overlapping(data, addr, size).into_iter().any(|id| {
        data.vbank()
            .get(id)
            .is_some_and(|v| v.get_offset() <= off && v.get_offset().wrapping_add(v.get_size() as u64) >= end)
    })
}

/// The Varnodes sharing a byte with `addr`/`size`.
fn overlapping(data: &Funcdata, addr: &Address, size: i32) -> Vec<VarnodeId> {
    let Some(space) = addr.get_space() else { return Vec::new() };
    let off = addr.get_offset();
    let end = off.wrapping_add(size as u64);
    let lo = Address::new(Rc::clone(space), off.saturating_sub(64));
    let hi = addr + size as i64;
    data.vbank()
        .iter_loc_addr_range(&lo, &hi)
        .filter(|&id| {
            data.vbank().get(id).is_some_and(|v| v.get_offset() < end && off < v.get_offset().wrapping_add(v.get_size() as u64))
        })
        .collect()
}
