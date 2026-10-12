//! (kuna) `varargsharedfloat` — a 32-bit PowerPC variadic double whose register
//! also feeds an earlier argument of the same call is an argument.
//!
//! `sumi(2, x * 2, x)` with `sumi(int, ...)` compiles at clang -O0 to
//! `lfd 2,16(31); fadd 1,2,2; li 3,2; crset 6; bl sumi`: `x` is loaded once into
//! f2, the second variadic double, and `x + x` is computed from it into f1.
//! Upstream's `checkCallDoubleUse` (funcdata_varnode.cc:1802) refuses a value
//! that also feeds another active argument, so the f2 trial is inactive and, as
//! the last one, dropped: `sumi(2,x + x)`.
//!
//! The same shape is what a scratch register looks like: clang compiles
//! `sumi(1, *a * *b + *c)` to `lfd 0; lfd 1; lfd 2; fmadd 1,0,1,2; crset 6`, with
//! `*c` in f2 and only f1 passed, and CR bit 6 says only that some FPR is. The
//! register choice tells them apart. A scratch double gets the lowest free FPR
//! (f0 first), so a value held in f2 while f0 or f1 was free for its whole life
//! was put in f2 for the call. The trial is kept when:
//!
//! * the call is variadic and its block sets CR bit 6;
//! * the trial is a model floating-point entry past the first, and every
//!   earlier floating-point entry is an active trial;
//! * its value is written in the call's block (not an incoming value);
//! * f0 or one of those earlier registers is free from the instruction after
//!   the one writing the value to the last instruction before the call reading
//!   it ([`register_free`]);
//! * and every other use of the value reaches only another active argument slot
//!   of the same call (upstream's `onlyOpUse` walk, with that one use allowed).
//!
//! The register-choice reading holds only for code the compiler did not
//! reschedule after register allocation (-O0): at -O1 and above clang's
//! post-RA scheduler moves instructions so a scratch value can look like it
//! had f0 or f1 free, and the rule then prints arguments the source never
//! passed. Hence off by default.
use kuna_base::address::Address;
use kuna_base::types::int4;
use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_class;
use crate::fspec::ParamTrial;
use crate::funcdata::Funcdata;
use crate::p4_calls::kuna_varargretreg::{float_register_count, VarargFloats};

/// Is the inactive trial `trial`, holding `vn` at call spec `idx`, a variadic
/// double that also feeds an earlier argument of the call (see the module)?
pub fn feeds_earlier_argument(
    data: &mut Funcdata,
    idx: int4,
    vn: VarnodeId,
    trial: &mut ParamTrial,
    maxancestor: int4,
) -> bool {
    if !data.get_arch().vararg_shared_float
        || data.get_arch().vararg_floats != VarargFloats::ConditionBit
        || trial.get_size() != 8
    {
        return false;
    }
    let fc = data.get_call_specs(idx);
    if !fc.is_dotdotdot() || !fc.proto().has_model() {
        return false;
    }
    let call = fc.get_op();
    let floats: Vec<Address> = fc
        .proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() == 8)
        .map(|e| Address::new(Rc::clone(e.get_space()), e.get_base()))
        .collect();
    let Some(k) = floats.iter().position(|a| a == trial.get_address()).filter(|&k| k > 0) else {
        return false;
    };
    let active = fc.active_input();
    let earlier_active = floats[..k].iter().all(|a| {
        (0..active.get_num_trials()).map(|t| active.get_trial(t)).any(|t| {
            t.get_address() == a && t.get_size() == 8 && t.is_checked() && t.is_active()
        })
    });
    if !earlier_active || float_register_count(data, call) != Some(1) {
        return false;
    }
    let Some(f0) = scratch_register(data) else {
        return false;
    };
    let mut lower = vec![f0];
    lower.extend(floats[..k].iter().cloned());
    if !lower.iter().any(|r| register_free(data, call, vn, r)) {
        return false;
    }
    data.kuna_set_shared_float_call(Some(call));
    let kept = data.ancestor_op_use(maxancestor, vn, call, trial, 0, 0);
    data.kuna_set_shared_float_call(None);
    kept
}

/// FPR 0, the first register a compiler gives a scratch double.
fn scratch_register(data: &Funcdata) -> Option<Address> {
    let f0 = data.get_arch().manage().register_lookup()?.probe_register("f0")?;
    let space = f0.space.filter(|_| f0.size == 8)?;
    Some(Address::new(space, f0.offset))
}

/// Is the 8-byte register `reg` free while `vn`, written in `call`'s block, is
/// live as a scratch value: from the instruction after the one writing it to
/// the last instruction before `call` reading it? A register written in that
/// stretch before the last reading instruction, or read in it (the call
/// included) with a value written before that instruction, is not.
fn register_free(data: &Funcdata, call: OpId, vn: VarnodeId, reg: &Address) -> bool {
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else {
        return false;
    };
    let (Some(d), Some(c)) = (data.obank().get(def), data.obank().get(call)) else {
        return false;
    };
    if matches!(d.code(), OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL)
        || d.get_parent().is_none()
        || d.get_parent() != c.get_parent()
    {
        return false;
    }
    let def_at = d.get_addr().clone();
    let mut window: Vec<OpId> = Vec::new();
    let mut cur = d.basic_neighbours().1;
    loop {
        let Some(op) = cur else {
            return false;
        };
        window.push(op);
        if op == call {
            break;
        }
        cur = data.obank().get(op).and_then(|o| o.basic_neighbours().1);
    }
    let position = |op: OpId| window.iter().position(|&w| w == op);
    let mut last_read = None;
    for reader in data.vbank().get(vn).map(|v| v.descend_iter().collect::<Vec<_>>()).unwrap_or_default() {
        if reader == call
            || data.obank().get(reader).is_some_and(|o| o.code() == OpCode::CPUI_INDIRECT)
        {
            continue;
        }
        let Some(p) = position(reader) else {
            return false;
        };
        last_read = last_read.max(Some(p));
    }
    let Some(last_read) = last_read else {
        return false;
    };
    let read_at = data.obank().get(window[last_read]).map(|o| o.get_addr().clone());
    let read_start =
        window.iter().position(|&w| data.obank().get(w).map(|o| o.get_addr().clone()) == read_at);
    let Some(read_start) = read_start else {
        return false;
    };
    let overlaps = |v: VarnodeId| {
        data.vbank().get(v).is_some_and(|v| {
            reg.get_space().is_some_and(|s| s.get_index() == v.get_space().get_index())
                && v.get_offset() < reg.get_offset() + 8
                && reg.get_offset() < v.get_offset() + v.get_size() as u64
        })
    };
    for (p, &op) in window.iter().enumerate() {
        let Some(o) = data.obank().get(op) else {
            return false;
        };
        if o.code() == OpCode::CPUI_INDIRECT || *o.get_addr() == def_at {
            continue;
        }
        if p < read_start && o.get_out().is_some_and(overlaps) {
            return false;
        }
        for slot in 0..o.num_input() {
            let Some(input) = o.get_in(slot).filter(|&v| overlaps(v)) else {
                continue;
            };
            let written_at = data.vbank().get(input).and_then(|v| v.get_def()).and_then(position);
            if written_at.is_none_or(|w| w < read_start) {
                return false;
            }
        }
    }
    true
}
