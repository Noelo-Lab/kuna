//! (kuna) The quotient a division leaves in the integer return register is no
//! return value when the floating return value is computed from its remainder.
//!
//! x86 `div` and `idiv` write the quotient to `eax`/`rax` and the remainder to
//! `edx`/`rdx` at once, so `return (float)(a % b);` compiles to `idiv esi;
//! cvtsi2ss xmm0,edx; ret` and leaves a quotient nothing reads in `rax`.
//! Scoring finds both `rax` and `xmm0` holding a value only the RETURN reads,
//! and the output fill-in prefers the general register: the function returned
//! `a / b` and dead-code removal dropped the float.
//!
//! [`drop_quotient`] runs before the output fill-in. It retires an active
//! trial outside the floating output entries while a floating trial is active
//! and, at every live RETURN, each value the trial reads there (followed
//! through copies, zero extensions, low truncations, the low half of a PIECE
//! and MULTIEQUALs) is one of
//! - a quotient whose remainder, the INT_REM or INT_SREM of the same operands,
//!   the floating trial's value at that RETURN is computed from;
//! - a value that floating value is computed from (`(float)rng()` converts the
//!   integer it returns in `rax`);
//! - the register's own incoming value, which the function never wrote there.
//!
//! At least one must be a quotient. A declared output, and storage the
//! function's callers read, are never touched. A quotient the floating value
//! is not computed from (`*out = (float)(a % b); return a / b;` stores it) is
//! left as the return.

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_class;
use crate::fspec::{ParamActive, ParamEntry};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// How many varnodes the slice under one floating return value may hold.
const MAX_SLICE: usize = 96;

/// How many defining ops one integer return value may be traced through.
const MAX_STEPS: usize = 32;

/// The output entries of `fd`'s model, or of the default model before the
/// function has one.
fn output_entries(fd: &Funcdata) -> &[ParamEntry] {
    let proto = fd.get_func_proto();
    let model = if proto.has_model() { Some(proto.model()) } else { fd.get_arch().defaultfp.as_ref() };
    model.and_then(|m| m.output_list()).map_or(&[], |out| out.get_entry())
}

/// Retire an active integer return trial that only holds a division's quotient
/// whose remainder the active floating return value is computed from.
pub fn drop_quotient(data: &Funcdata, active: &mut ParamActive, returns: &[OpId]) {
    if data.get_func_proto().is_output_locked() {
        return;
    }
    let candidates: Vec<i32> = (0..active.get_num_trials())
        .filter(|&i| {
            let t = active.get_trial(i);
            t.is_active() && !t.is_unref() && t.get_slot() >= 1
        })
        .collect();
    if candidates.len() < 2 {
        return;
    }
    let entries = output_entries(data);
    let class_of = |i: i32| {
        let t = active.get_trial(i);
        entries
            .iter()
            .find(|e| e.justified_contain(t.get_address(), t.get_size()) >= 0)
            .map(|e| e.get_type())
    };
    let (floats, ints): (Vec<i32>, Vec<i32>) = candidates
        .iter()
        .copied()
        .filter(|&i| class_of(i).is_some())
        .partition(|&i| class_of(i) == Some(type_class::TYPECLASS_FLOAT));
    if floats.is_empty() || ints.is_empty() {
        return;
    }
    let live: Vec<OpId> = returns
        .iter()
        .copied()
        .filter(|&id| {
            data.obank().get(id).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0)
                && !crate::kuna_bejoin::never_reached(data, id)
        })
        .collect();
    if live.is_empty() {
        return;
    }
    let float_slots: Vec<i32> = floats.iter().map(|&i| active.get_trial(i).get_slot()).collect();
    let mut slices: Vec<Option<Vec<VarnodeId>>> = vec![None; live.len()];
    for i in ints {
        let (addr, size, slot) = {
            let t = active.get_trial(i);
            (t.get_address().clone(), t.get_size(), t.get_slot())
        };
        if crate::p4_calls::kuna_voidret::forced(data, &addr, size) {
            continue;
        }
        let reads: Vec<Option<VarnodeId>> =
            live.iter().map(|&r| data.obank().get(r).and_then(|o| o.get_in(slot))).collect();
        if !reads.iter().flatten().any(|&vn| reaches_quotient(data, vn)) {
            continue;
        }
        let mut quotients = 0usize;
        let byproduct = live.iter().enumerate().all(|(k, &r)| {
            let Some(vn) = reads[k] else { return false };
            let slice = slices[k].get_or_insert_with(|| float_slice(data, r, &float_slots));
            let mut seen = Vec::new();
            let mut steps = MAX_STEPS;
            byproduct_value(data, vn, size, &addr, slice, &mut seen, &mut steps, &mut quotients)
        });
        if byproduct && quotients > 0 {
            active.get_trial_mut(i).mark_inactive();
        }
    }
}

/// The values the low `want` bytes of `vn` are copied from, with how many
/// bytes of each: the input of a COPY, a zero extension or a low truncation,
/// the low half of a PIECE that holds them all, or each input of a
/// MULTIEQUAL. `None` when `vn` is built any other way.
fn low_inputs(data: &Funcdata, vn: VarnodeId, want: i32) -> Option<(OpId, Vec<(VarnodeId, i32)>)> {
    let def = data.vbank().get(vn)?.get_def()?;
    let op = data.obank().get(def)?;
    let size = |v: VarnodeId| data.vbank().get(v).map_or(0, |x| x.get_size());
    let next = match op.code() {
        OpCode::CPUI_COPY => vec![(op.get_in(0)?, want)],
        OpCode::CPUI_INT_ZEXT => {
            let x = op.get_in(0)?;
            vec![(x, want.min(size(x)))]
        }
        OpCode::CPUI_SUBPIECE if low_truncation(data, op) => vec![(op.get_in(0)?, want)],
        OpCode::CPUI_PIECE => {
            let lo = op.get_in(1)?;
            if want > size(lo) {
                return None;
            }
            vec![(lo, want)]
        }
        OpCode::CPUI_MULTIEQUAL if op.num_input() > 0 => {
            (0..op.num_input()).map(|k| op.get_in(k).map(|x| (x, want))).collect::<Option<Vec<_>>>()?
        }
        _ => return None,
    };
    Some((def, next))
}

/// Is `vn`'s def an INT_DIV or INT_SDIV?
fn is_quotient(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .is_some_and(|o| matches!(o.code(), OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_SDIV))
}

/// Do the low bytes of `vn`, followed through [`low_inputs`], come from an
/// INT_DIV or INT_SDIV on some path?
fn reaches_quotient(data: &Funcdata, vn: VarnodeId) -> bool {
    let start = data.vbank().get(vn).map_or(0, |v| v.get_size());
    let mut stack = vec![(vn, start)];
    let mut seen: Vec<OpId> = Vec::new();
    while let Some((v, want)) = stack.pop() {
        if is_quotient(data, v) {
            return true;
        }
        if seen.len() >= MAX_STEPS {
            return false;
        }
        let Some((def, next)) = low_inputs(data, v, want) else { continue };
        if !seen.contains(&def) {
            seen.push(def);
            stack.extend(next);
        }
    }
    false
}

/// Is `op` a SUBPIECE that keeps the least significant bytes?
fn low_truncation(data: &Funcdata, op: &crate::op::PcodeOp) -> bool {
    op.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == 0)
}

/// The varnodes the floating trials' values at the RETURN `ret` are computed
/// from, themselves included, nearest first, at most [`MAX_SLICE`] of them.
fn float_slice(data: &Funcdata, ret: OpId, slots: &[i32]) -> Vec<VarnodeId> {
    let mut slice: Vec<VarnodeId> = Vec::new();
    let Some(op) = data.obank().get(ret) else { return slice };
    slice.extend(slots.iter().filter_map(|&s| op.get_in(s)));
    let mut k = 0;
    while k < slice.len() && slice.len() < MAX_SLICE {
        let vn = slice[k];
        k += 1;
        let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            continue;
        };
        for j in 0..def.num_input() {
            let Some(input) = def.get_in(j) else { continue };
            if data.vbank().get(input).is_some_and(|v| v.is_constant()) || slice.contains(&input) {
                continue;
            }
            if slice.len() >= MAX_SLICE {
                break;
            }
            slice.push(input);
        }
    }
    slice
}

/// Is every value the low `want` bytes of `vn` come from ([`low_inputs`]) a
/// quotient whose remainder is in `slice`, a member of `slice`, or the
/// incoming value of the trial's own storage at `addr`? Counts the quotients
/// in `quotients`.
#[allow(clippy::too_many_arguments)]
fn byproduct_value(
    data: &Funcdata,
    vn: VarnodeId,
    want: i32,
    addr: &kuna_base::address::Address,
    slice: &[VarnodeId],
    seen: &mut Vec<OpId>,
    steps: &mut usize,
    quotients: &mut usize,
) -> bool {
    if slice.contains(&vn) {
        return true;
    }
    if *steps == 0 {
        return false;
    }
    *steps -= 1;
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_input() {
        return v.get_addr() == addr;
    }
    if is_quotient(data, vn) {
        let fed = v.get_def().and_then(|d| remainder_of(data, d)).is_some_and(|r| slice.contains(&r));
        if fed {
            *quotients += 1;
        }
        return fed;
    }
    let Some((def, next)) = low_inputs(data, vn, want) else { return false };
    if seen.contains(&def) {
        return true;
    }
    seen.push(def);
    next.into_iter().all(|(x, w)| byproduct_value(data, x, w, addr, slice, seen, steps, quotients))
}

/// The result of the INT_REM (INT_SREM for an INT_SDIV) of the same two
/// operands as the division `div`, the other half of an x86 `div`/`idiv`.
fn remainder_of(data: &Funcdata, div: OpId) -> Option<VarnodeId> {
    let op = data.obank().get(div)?;
    let want = match op.code() {
        OpCode::CPUI_INT_DIV => OpCode::CPUI_INT_REM,
        OpCode::CPUI_INT_SDIV => OpCode::CPUI_INT_SREM,
        _ => return None,
    };
    let (num, den) = (op.get_in(0)?, op.get_in(1)?);
    let same = |a: VarnodeId, b: VarnodeId| {
        a == b
            || match (data.vbank().get(a), data.vbank().get(b)) {
                (Some(x), Some(y)) => {
                    x.is_constant() && y.is_constant() && x.get_offset() == y.get_offset() && x.get_size() == y.get_size()
                }
                _ => false,
            }
    };
    data.vbank().get(num)?.descend_iter().find_map(|d| {
        let rem = data.obank().get(d)?;
        (d != div
            && rem.code() == want
            && rem.get_in(0) == Some(num)
            && rem.get_in(1).is_some_and(|x| same(x, den)))
        .then(|| rem.get_out())
        .flatten()
    })
}
