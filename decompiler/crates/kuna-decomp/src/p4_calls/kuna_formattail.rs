//! P4 tail of a call whose format string was resolved (`formatstring`).
//!
//! # The gap
//!
//! A resolved printf/scanf call gets a prototype override with exactly the
//! arguments its format consumes. Installed closed, the call is never offered a
//! trial past them, and that changes how every OTHER call's trials are scored:
//! `Funcdata::only_op_use` rejects a value another call already took as an
//! argument, so a value the format call used to claim as a phantom was never
//! free for a later call. gnulib `version_etc` shows it. The authors' `va_list`
//! sits in the first three outgoing stack slots when the
//! `"%s (%s) %s\n"` line is printed; open, that call claims the three slots and
//! the `"Written by ..."` calls further down, which push their own arguments
//! below them, are refused them. Closed, nothing claims them, and each of those
//! calls, whose formats are not resolved, grows three phantoms.
//!
//! # The rule
//!
//! The override is installed as a locked prefix followed by `...`
//! (`first_var_arg_slot` = the declared count), so the call is offered the same
//! trials past its declared arguments that it is offered without the override,
//! and they are scored the same way. Once no live call in the function has
//! trials left, the format call sheds every input past its declared ones and
//! the `...`. Until then a claimed phantom keeps doing what it did without the
//! override: vetoing the same value at a neighbouring call.

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;

/// Keep every declared argument of a resolved format call. `fillinMap`'s
/// positional rules end an argument list at a run of unused registers, and a
/// ninth `double` goes on the stack after the four integer registers a
/// `"%f ... %d"` call leaves unused, so the locked stack trial would otherwise
/// be dropped.
pub fn keep_declared_trials(fc: &mut FuncCallSpecs) {
    let Some(arity) = fc.format_arity() else { return };
    let active = fc.get_active_input();
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial_mut(i);
        let pos = trial.get_fixed_position();
        if (0..arity).contains(&pos) && !trial.is_definitely_not_used() {
            trial.mark_active();
            trial.mark_used();
        }
    }
}

/// Shed the open tail of every resolved format call once every live call's
/// argument list is final. Returns how many calls changed.
pub fn shed_format_tails(data: &mut Funcdata) -> int4 {
    let mut pending: Vec<(int4, int4)> = Vec::new();
    for idx in 0..data.num_calls() {
        let fc = data.get_call_specs(idx);
        let live = data.obank().get(fc.get_op()).is_some_and(|o| {
            !o.is_dead() && matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
        });
        if !live {
            continue;
        }
        if fc.is_input_active() {
            return 0;
        }
        if let Some(arity) = fc.format_arity() {
            pending.push((idx, arity));
        }
    }
    let mut changed = 0;
    for (idx, arity) in pending {
        let op = data.get_call_specs(idx).get_op();
        let ninput = data.obank().get(op).map_or(0, |o| o.num_input());
        for slot in (arity + 1..ninput).rev() {
            data.op_remove_input(op, slot);
        }
        let fc = data.get_call_specs_mut(idx);
        let mut storage = fc.final_input_storage().to_vec();
        storage.truncate(arity as usize);
        fc.set_final_input_storage(storage);
        fc.proto_mut().set_dotdotdot(false);
        fc.set_format_arity(None);
        changed += 1;
    }
    changed
}
