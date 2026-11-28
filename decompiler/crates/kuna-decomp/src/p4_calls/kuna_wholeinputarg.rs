//! (kuna) A stated parameter the caller hands on whole to a call is passed
//! whole.
//!
//! When an 8-byte incoming `x0` also has a 4-byte reader, such as the `w0` of a
//! stated `unsigned int` return, heritage splits the register and a call's
//! trial at `x0` covers only the low piece, read as `SUBPIECE(x0(i),0)`. That
//! read is heritage's own, not an instruction, yet the call took the piece as
//! its argument, either because the SUBPIECE passes the realistic-ancestor test
//! or because `fillinMap` filled it as a hole below a later argument. The
//! callee then received the caller's pointer cut to the return width: an
//! AArch64 PLT veneer stated as `unsigned sample(struct Item *p)` printed
//! `(*dat_20000)((int)p)`. Once the trials are mapped, a used trial at a
//! non-variadic call that is such a piece of a parameter the caller's locked
//! prototype states in exactly that register is widened to the register, so
//! the call passes `p`. Which trials are arguments is left as it was. A
//! variadic call keeps the width its format gave, a recovered, narrower or
//! aggregate parameter keeps the piece, and so does a callee whose own
//! recovered prototype reads less of the register.

use crate::context::VarnodeId;
use crate::dtype::{type_class, type_metatype};
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

/// Widen each used trial of the non-variadic call `fc` whose value is the low
/// piece of a stated parameter of the caller, untouched in its register, to
/// that register, where the register is exactly the general-purpose entry the
/// trial was mapped to. Runs after `fillinMap`, before the inputs are built.
pub fn widen(fc: &mut FuncCallSpecs, data: &mut Funcdata) {
    if !fc.proto().has_model() || fc.is_dotdotdot() || !data.get_func_proto().is_input_locked() {
        return;
    }
    let op = fc.get_op();
    for i in 0..fc.active_input().get_num_trials() {
        let trial = fc.active_input().get_trial(i);
        if !trial.is_used() || trial.is_unref() {
            continue;
        }
        let slot = trial.get_slot();
        let Some((piece, whole, addr, size)) = incoming_whole(fc, data, i) else { continue };
        let others_overlap = (0..fc.active_input().get_num_trials()).any(|j| {
            let other = fc.active_input().get_trial(j);
            j != i && data.vbank().get(whole).is_some_and(|w| w.intersects_range(other.get_address(), other.get_size()))
        });
        if others_overlap {
            continue;
        }
        if data.op_set_input(op, whole, slot).is_err() {
            if data.op_set_input(op, piece, slot).is_err() {
                fc.get_active_input().get_trial_mut(i).mark_no_use();
            }
            continue;
        }
        fc.get_active_input().get_trial_mut(i).set_address(addr, size);
    }
}

/// The piece trial `i` of `fc` reads and the incoming register it is the low
/// piece of, with that register's storage, when the register holds a pointer or
/// integer parameter the caller states, is exactly the general-purpose entry of
/// the trial, and the callee states no narrower parameter there.
fn incoming_whole(fc: &FuncCallSpecs, data: &Funcdata, i: int4) -> Option<(VarnodeId, VarnodeId, Address, int4)> {
    let trial = fc.active_input().get_trial(i);
    let call = data.obank().get(fc.get_op())?;
    let piece_id = call.get_in(trial.get_slot())?;
    let piece = data.vbank().get(piece_id)?;
    if piece.get_addr() != trial.get_address() || piece.get_size() != trial.get_size() {
        return None;
    }
    let def = data.obank().get(piece.get_def()?)?;
    if def.code() != OpCode::CPUI_SUBPIECE || data.vbank().get(def.get_in(1)?)?.get_offset() != 0 {
        return None;
    }
    let whole_id = def.get_in(0)?;
    let whole = data.vbank().get(whole_id)?;
    if !whole.is_input() || whole.is_written() || whole.get_size() <= piece.get_size() {
        return None;
    }
    if whole.get_addr().get_space().map(|s| s.get_type()) != Some(spacetype::IPTR_PROCESSOR) {
        return None;
    }
    let proto = data.get_func_proto();
    let stated = (0..proto.num_params())
        .filter_map(|p| proto.get_param(p))
        .any(|p| {
            &p.get_address() == whole.get_addr()
                && p.get_size() == whole.get_size()
                && p.get_type().is_some_and(|t| {
                    matches!(t.get_metatype(), type_metatype::TYPE_PTR | type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
                })
        });
    if !stated || callee_reads_narrower(fc, data, whole) {
        return None;
    }
    let entry = fc.proto().model().input().get_entry().get(trial.get_entry()?)?;
    let register_entry = entry.get_type() == type_class::TYPECLASS_GENERAL
        && entry.get_size() == whole.get_size()
        && entry.justified_contain(whole.get_addr(), whole.get_size()) == 0
        && entry.justified_contain(piece.get_addr(), piece.get_size()) == 0;
    register_entry.then(|| (piece_id, whole_id, whole.get_addr().clone(), whole.get_size()))
}

/// Does the callee's own recovered prototype (stated by `protoorder`) take a
/// parameter narrower than `whole` in `whole`'s register?
fn callee_reads_narrower(fc: &FuncCallSpecs, data: &Funcdata, whole: &crate::varnode::Varnode) -> bool {
    let entry = fc.get_entry_address();
    if entry.is_invalid() {
        return false;
    }
    data.kuna_protoorder_types(entry).is_some_and(|stated| {
        stated.inputs.iter().any(|(addr, size, _)| *size < whole.get_size() && whole.intersects_range(addr, *size))
    })
}
