//! (kuna) The upper half of a vector register that a narrow write zero-fills is
//! not part of the value a function returns (option `zerofillreturn`).
//!
//! AAPCS64 returns a `double` in `d0`, the low doubleword of `q0`, and the
//! compiler spec's floating output entries are the whole 16-byte `q0`..`q3`.
//! SLEIGH models a scalar or 64-bit vector write (`scvtf d0,w0`, `fmul d0,..`,
//! `fadd v0.2s,..`) as the low lane plus a COPY of zero into each higher lane
//! of the register (`zext_zd`), so heritage gives the RETURN a second trial in
//! `q0`, its upper doubleword, holding a literal 0. Scoring accepts a constant
//! as returned (`ancestorOpUse` stops at the COPY), and the output fill-in
//! joins the two into a 16-byte `q0`: `undefined16 scale(int)` with
//! `v1._8_8_ = 0`. A `float` in `s0` is not affected: the dead-code trim of a
//! zero-extended return narrows it, but its masks are 64 bits wide, so it
//! cannot narrow a 16-byte value to its low 8 bytes.
//!
//! [`seed`] runs on the lifted p-code before heritage, while each instruction's
//! ops are still the ones SLEIGH emitted, and marks a COPY of zero into the
//! upper doubleword of a floating output entry wider than 8 bytes when its
//! instruction writes the low lane of that same register, at most 8 bytes of
//! it, and nothing but zero into the rest of the entry. Once heritage has run
//! this cannot be told apart from a 128-bit write whose upper half is zero
//! (`movi v0.2d,#0`, `ldr q0`): heritage splits that write into lanes, and
//! constant folding leaves a COPY of zero at the same instruction.
//!
//! Before the output fill-in, [`drop_zero_fill`] retires an active trial in the
//! upper doubleword of such an entry when its value at every RETURN is a
//! marked COPY, directly or through MULTIEQUALs. The low lane is then the
//! return value: `double scale(int)`, and a complex double or an AAPCS64
//! aggregate of two doubles whose `d1` the function computes joins `d0` and
//! `d1` instead of returning `q0` with a zero upper half. A `float` in `s0`,
//! which the trim already narrows, is left alone. A 128-bit vector whose upper
//! half a 64-bit write zeroes (`fmov d0,d0`, `fmov d0,x0`, `ldr d0`) has the
//! same p-code as a returned `double` and returns as one. A declared output is
//! never touched.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_class;
use crate::fspec::{ParamActive, ParamEntry};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_addlflags::kuna_zerofill;
use crate::p0_knowledge::options::on_or_off;

/// (kuna) Drop the zero-filled upper half of a returned vector register:
/// `zerofillreturn on|off`.
pub struct OptionZeroFillReturn;

impl OptionZeroFillReturn {
    /// The option name.
    pub const NAME: &'static str = "zerofillreturn";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::zero_fill_return`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Zero-filled return lanes dropped: {prop}")))
    }
}

/// The low bytes a narrow write keeps; above them a lane is zero fill.
const LOW_LANE: i32 = 8;

/// How many defining ops one RETURN input may be traced through.
const MAX_STEPS: usize = 32;

/// The floating output entries wider than [`LOW_LANE`] of `fd`'s model, or of
/// the default model before the function has one.
fn wide_entries(fd: &Funcdata) -> Vec<ParamEntry> {
    let proto = fd.get_func_proto();
    let model = if proto.has_model() { Some(proto.model()) } else { fd.get_arch().defaultfp.as_ref() };
    let Some(out) = model.and_then(|m| m.output_list()) else {
        return Vec::new();
    };
    out.get_entry()
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() > LOW_LANE)
        .cloned()
        .collect()
}

/// Mark the COPYs of zero a narrow write leaves above its lane of a returnable
/// vector register. Runs on the freshly lifted p-code.
pub fn seed(arch: &crate::architecture::Architecture, fd: &mut Funcdata) {
    if !arch.zero_fill_return || fd.get_func_proto().is_output_locked() {
        return;
    }
    let wide = wide_entries(fd);
    if wide.is_empty() {
        return;
    }
    let mut marks: Vec<OpId> = Vec::new();
    let mut decided: Vec<(Address, usize, Option<i32>)> = Vec::new();
    for id in fd.obank().iter_alive() {
        if !is_zero_copy(fd, id) {
            continue;
        }
        let Some(op) = fd.obank().get(id) else { continue };
        let Some(out) = op.get_out().and_then(|v| fd.vbank().get(v)) else { continue };
        let Some((e, _)) = wide
            .iter()
            .enumerate()
            .map(|(e, entry)| (e, entry.justified_contain(out.get_addr(), out.get_size())))
            .find(|&(_, off)| off >= LOW_LANE)
        else {
            continue;
        };
        let at = op.get_addr();
        let fill = match decided.iter().find(|(a, k, _)| a == at && *k == e) {
            Some(&(_, _, known)) => known,
            None => {
                let known = fill_start(fd, at, &wide[e]);
                decided.push((at.clone(), e, known));
                known
            }
        };
        if fill.is_some() {
            marks.push(id);
        }
    }
    for id in marks {
        if let Some(op) = fd.obank_mut().get_mut(id) {
            op.set_additional_flag(kuna_zerofill);
        }
    }
}

/// Is `id` a COPY of the constant zero into a register?
fn is_zero_copy(fd: &Funcdata, id: OpId) -> bool {
    let Some(op) = fd.obank().get(id).filter(|o| o.code() == OpCode::CPUI_COPY) else { return false };
    let reg = op
        .get_out()
        .and_then(|v| fd.vbank().get(v))
        .is_some_and(|v| v.get_space().get_type() == spacetype::IPTR_PROCESSOR);
    reg && op
        .get_in(0)
        .and_then(|c| fd.vbank().get(c))
        .is_some_and(|c| c.is_constant() && c.get_offset() == 0)
}

/// Where the zero fill starts in `entry`'s register, when the instruction at
/// `at` writes its low lane, at most [`LOW_LANE`] bytes from the least
/// significant byte up, and only zero above it. The lane is the write at the
/// low end and the writes that continue it without a gap, other than COPYs of
/// zero; nothing may straddle its end or reach outside the entry.
fn fill_start(fd: &Funcdata, at: &Address, entry: &ParamEntry) -> Option<i32> {
    let mut writes: Vec<(i32, i32, bool)> = Vec::new();
    for (_, id) in fd.obank().iter_at(at) {
        let Some(op) = fd.obank().get(id) else { continue };
        let Some(out) = op.get_out().and_then(|v| fd.vbank().get(v)) else { continue };
        let (addr, size) = (out.get_addr(), out.get_size());
        if out.get_space().get_type() != spacetype::IPTR_PROCESSOR || !entry.intersects(addr, size) {
            continue;
        }
        let off = entry.justified_contain(addr, size);
        if off < 0 {
            return None;
        }
        writes.push((off, size, is_zero_copy(fd, id)));
    }
    lane_end(&writes)
}

/// The end of the low lane among one instruction's writes to an entry, given
/// as `(justified offset, size, is a COPY of zero)`, when it ends within
/// [`LOW_LANE`] bytes, only zeros follow it in the low doubleword, and the
/// upper doubleword is one 8-byte COPY of zero (on each path through a
/// conditional instruction such as `fcsel`): the shape of SLEIGH's
/// `zext_zd`/`zext_zs` fill. `movi v0.4s,#0` writes the upper doubleword as
/// two 4-byte lanes of a 128-bit value and is no fill.
fn lane_end(writes: &[(i32, i32, bool)]) -> Option<i32> {
    let mut end = writes.iter().filter(|w| w.0 == 0).map(|w| w.1).max()?;
    while let Some(lane) = writes.iter().find(|w| w.0 == end && !w.2) {
        end += lane.1;
    }
    let low = writes
        .iter()
        .filter(|w| w.0 < LOW_LANE)
        .all(|&(off, size, zero)| if off < end { off + size <= end } else { zero && off + size <= LOW_LANE });
    let mut high = writes.iter().filter(|w| w.0 >= LOW_LANE).peekable();
    let fill = high.peek().is_some() && high.all(|w| *w == (LOW_LANE, LOW_LANE, true));
    (end <= LOW_LANE && low && fill).then_some(end)
}

/// Retire the active output trials that only hold the zero fill of a narrow
/// write in the upper doubleword of a floating register entry. An active fill
/// fails every output rule, so the fill-in falls back to the best single
/// register, and retiring it lets the rules join what else is active. So
/// nothing is retired while a trial outside the floating entries is active
/// (`x0` and a leftover `x1` would join as a pair), or unless every register
/// with an active low lane returns a whole doubleword there: `s0` and its
/// fill are left to the trim, and the homogeneous-aggregate rule would join
/// one member per register whatever its width (`s0` beside a 64-bit `d1`).
/// Nor is anything retired when a later register's low lane is not
/// [`returned_alone`]: retiring only `q0`'s fill would return the first member
/// of what may be a pair, so every fill stays and the return is `q0` as before.
pub fn drop_zero_fill(data: &Funcdata, active: &mut ParamActive, returns: &[OpId]) {
    if !data.get_arch().zero_fill_return || data.get_func_proto().is_output_locked() {
        return;
    }
    let wide = wide_entries(data);
    if wide.is_empty() {
        return;
    }
    let live: Vec<OpId> = returns
        .iter()
        .copied()
        .filter(|&id| data.obank().get(id).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    if live.is_empty() {
        return;
    }
    let mut fills: Vec<i32> = Vec::new();
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial(i);
        if !trial.is_active() {
            continue;
        }
        if !wide.iter().any(|e| e.justified_contain(trial.get_address(), trial.get_size()) >= LOW_LANE) {
            continue;
        }
        let slot = trial.get_slot();
        let filled = live.iter().all(|&id| {
            let mut steps = MAX_STEPS;
            let mut seen = Vec::new();
            data.obank()
                .get(id)
                .and_then(|o| o.get_in(slot))
                .is_some_and(|vn| zero_fill(data, vn, &mut seen, &mut steps))
        });
        if filled {
            fills.push(i);
        }
    }
    let elsewhere = (0..active.get_num_trials()).map(|i| active.get_trial(i)).any(|t| {
        t.is_active() && !wide.iter().any(|e| e.justified_contain(t.get_address(), t.get_size()) >= 0)
    });
    if fills.is_empty() || elsewhere {
        return;
    }
    let lanes: Vec<Option<(i32, i32)>> = wide
        .iter()
        .map(|e| {
            (0..active.get_num_trials())
                .map(|i| active.get_trial(i))
                .filter(|t| t.is_active() && e.justified_contain(t.get_address(), t.get_size()) == 0)
                .map(|t| (t.get_size(), t.get_slot()))
                .max()
        })
        .collect();
    if !lanes.iter().any(Option::is_some) || lanes.iter().flatten().any(|l| l.0 != LOW_LANE) {
        return;
    }
    let pairs = wide.iter().zip(&lanes).all(|(e, lane)| {
        lane.is_none_or(|(_, slot)| {
            e.is_first_in_class()
                || live.iter().all(|&id| {
                    let mut steps = MAX_STEPS;
                    data.obank()
                        .get(id)
                        .and_then(|o| o.get_in(slot))
                        .is_some_and(|vn| returned_alone(data, vn, e, &mut steps))
                })
        })
    });
    if !pairs {
        return;
    }
    for i in fills {
        active.get_trial_mut(i).mark_inactive();
    }
}

/// Is `vn` written on every path by an op whose result only goes on to the
/// RETURN, from a value the function produced? A second member of a returned
/// aggregate is; what a function only leaves in the next register is not:
/// glibc's `math_force_eval` keeps a `fmul d1,d0,d0` no op reads, on one path,
/// beside a `fabs d1,d0` the next compare reads. Nor is what a call left there
/// or the function's own incoming `d1` ([`produced_here`]): a forwarder of a
/// complex result copies back the `d1` its callee returned, which kuna sees as
/// the incoming one, and `CMPLX(a * 2, b)` cannot be told apart from it.
fn returned_alone(data: &Funcdata, vn: VarnodeId, entry: &ParamEntry, steps: &mut usize) -> bool {
    if *steps == 0 {
        return false;
    }
    *steps -= 1;
    let Some(value) = data.vbank().get(vn).filter(|v| v.num_descend() == 1) else { return false };
    let Some(op) = value.get_def().and_then(|d| data.obank().get(d)) else { return false };
    match op.code() {
        OpCode::CPUI_MULTIEQUAL => {
            op.num_input() > 0
                && (0..op.num_input()).all(|k| op.get_in(k).is_some_and(|v| returned_alone(data, v, entry, steps)))
        }
        OpCode::CPUI_INDIRECT | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => false,
        OpCode::CPUI_COPY => op.get_in(0).is_some_and(|v| produced_here(data, v, entry, steps)),
        OpCode::CPUI_LOAD => reloads_produced(data, op, entry, steps),
        _ => true,
    }
}

/// Followed through copies, phis and frame reloads, is `vn` something the
/// function computed, rather than the incoming value of the register `entry`
/// names or what a call left there? An incoming value of another register is
/// the function's own: `CMPLX(a * 2, a)` returns its `d0` argument in `d1`.
fn produced_here(data: &Funcdata, vn: VarnodeId, entry: &ParamEntry, steps: &mut usize) -> bool {
    if *steps == 0 {
        return false;
    }
    *steps -= 1;
    let Some(value) = data.vbank().get(vn) else { return false };
    if value.is_constant() {
        return true;
    }
    if value.is_input() {
        return !entry.intersects(value.get_addr(), value.get_size());
    }
    let Some(op) = value.get_def().and_then(|d| data.obank().get(d)) else { return false };
    match op.code() {
        OpCode::CPUI_COPY => op.get_in(0).is_some_and(|v| produced_here(data, v, entry, steps)),
        OpCode::CPUI_MULTIEQUAL => {
            op.num_input() > 0
                && (0..op.num_input()).all(|k| op.get_in(k).is_some_and(|v| produced_here(data, v, entry, steps)))
        }
        OpCode::CPUI_INDIRECT | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => false,
        OpCode::CPUI_LOAD => reloads_produced(data, op, entry, steps),
        _ => true,
    }
}

/// Does the load `op` read back a frame slot every store to which writes
/// something [`produced_here`]? Return recovery runs before the stack is
/// renamed, so `-O0`'s spill and reload of a value (`str d0,[sp,#8]` ..
/// `ldr d1,[sp,#8]`) is matched by hand: same incoming base register, same
/// constant offset, same width. A store that overlaps the slot any other way,
/// or none at all, refuses; a store through an address not computed from that
/// register is taken not to reach the frame.
fn reloads_produced(data: &Funcdata, op: &crate::op::PcodeOp, entry: &ParamEntry, steps: &mut usize) -> bool {
    let Some(size) = op.get_out().and_then(|o| data.vbank().get(o)).map(|o| i64::from(o.get_size())) else {
        return false;
    };
    let Some((base, off)) = op.get_in(1).and_then(|a| frame_slot(data, a)) else { return false };
    let space_of = |o: &crate::op::PcodeOp| o.get_in(0).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset());
    let space = space_of(op);
    let mut stored = false;
    for id in data.obank().iter_code(OpCode::CPUI_STORE) {
        let Some(store) = data.obank().get(id).filter(|s| !s.is_dead()) else { continue };
        let Some((b, o)) = store.get_in(1).and_then(|a| frame_slot(data, a)) else { continue };
        let Some(value) = store.get_in(2) else { return false };
        let width = data.vbank().get(value).map_or(0, |v| i64::from(v.get_size()));
        if b != base || o.wrapping_add(width) <= off || off.wrapping_add(size) <= o {
            continue;
        }
        if o != off || width != size || space_of(store) != space || !produced_here(data, value, entry, steps) {
            return false;
        }
        stored = true;
    }
    stored
}

/// The incoming register `vn` is computed from and the constant offset it
/// adds, through copies and constant additions and subtractions.
fn frame_slot(data: &Funcdata, mut vn: VarnodeId) -> Option<(Address, i64)> {
    let mut off: i64 = 0;
    for _ in 0..MAX_STEPS {
        let v = data.vbank().get(vn)?;
        if v.is_input() {
            return Some((v.get_addr().clone(), off));
        }
        let op = data.obank().get(v.get_def()?)?;
        match op.code() {
            OpCode::CPUI_COPY => vn = op.get_in(0)?,
            OpCode::CPUI_INT_ADD => {
                off = off.wrapping_add(constant(data, op.get_in(1)?)?);
                vn = op.get_in(0)?;
            }
            OpCode::CPUI_INT_SUB => {
                off = off.wrapping_sub(constant(data, op.get_in(1)?)?);
                vn = op.get_in(0)?;
            }
            _ => return None,
        }
    }
    None
}

/// The signed value of `vn`, a constant or a copy of one.
fn constant(data: &Funcdata, mut vn: VarnodeId) -> Option<i64> {
    for _ in 0..4 {
        let v = data.vbank().get(vn)?;
        if v.is_constant() {
            return Some(kuna_base::address::sign_extend(v.get_offset() as i64, v.get_size() * 8 - 1));
        }
        let op = data.obank().get(v.get_def()?)?;
        if op.code() != OpCode::CPUI_COPY {
            return None;
        }
        vn = op.get_in(0)?;
    }
    None
}

/// Is `vn` a marked zero fill on every path into it?
fn zero_fill(data: &Funcdata, vn: VarnodeId, seen: &mut Vec<OpId>, steps: &mut usize) -> bool {
    if *steps == 0 {
        return false;
    }
    *steps -= 1;
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else { return false };
    if seen.contains(&def) {
        return true;
    }
    seen.push(def);
    let Some(op) = data.obank().get(def) else { return false };
    match op.code() {
        OpCode::CPUI_COPY => {
            op.get_addlflags() & kuna_zerofill != 0
                && op
                    .get_in(0)
                    .and_then(|c| data.vbank().get(c))
                    .is_some_and(|c| c.is_constant() && c.get_offset() == 0)
        }
        OpCode::CPUI_MULTIEQUAL if op.num_input() > 0 => {
            let inputs: Vec<Option<VarnodeId>> = (0..op.num_input()).map(|k| op.get_in(k)).collect();
            inputs
                .into_iter()
                .all(|v| v.is_some_and(|v| zero_fill(data, v, seen, steps)))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests;
