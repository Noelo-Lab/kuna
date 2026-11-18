//! (kuna) The pop a call through a pointer takes from its declaration is an
//! equation for the stack solver, not an `INT_ADD` folded in before the solve.
//!
//! An x86-32 `call [slot]` that [`crate::p4_calls::kuna_slotargs`] declares
//! (a Win32 `__stdcall` import) states its pop, `4 + <argument bytes>`, under
//! a model that leaves extrapop unknown. `ActionExtraPopSetup` used to write a
//! stated pop into the IR as `sp_after = INT_ADD(sp_call, #pop)`. In a function
//! whose other pops `StackSolver` still guesses, that joins the stack pointer
//! after the call to the one before it before anything is solved, and the
//! simplifier then rebases the references on both sides onto each other:
//!
//! * The push-run guess of the next call
//!   ([`crate::p6_variables::kuna_calleepopslot`]) walks stack slots by their
//!   offset from a shared base, so the slots the declared call popped read as
//!   pushes for the next call: `push v; call [VariantClear]; push s; call
//!   [lstrlenA]` credits lstrlenA with VariantClear's argument.
//! * Term collection rewrites stack arithmetic after the call in terms of a
//!   stack pointer from before it. A pointer difference after
//!   `GetModuleFileNameA` became `-(sp at its last push)`, and the local-alias
//!   boundary took that slot as an escaped address, so every outgoing argument
//!   above it was refused for the rest of the function.
//!
//! The solved pops were the same either way. So a declared pop on a call
//! through a pointer, in a function that is left something to guess
//! ([`crate::p4_calls::kuna_calleepurge::every_pop_settled`]), takes the
//! INDIRECT the other calls take, and the prototype keeps the pop:
//! `StackSolver::build` reads it as the call's exact equation, the same answer
//! the INT_ADD gave, and the solve writes it into the IR with all the others.
//! Where the solve cannot place the stack pointer (a frame sized at run time,
//! a stack pointer an exception handler loads from memory), the INDIRECT is
//! written as the INT_ADD it would have been ([`write_unplaced`]). The guesses
//! that looked past the declared call see through its INDIRECT by its pop
//! ([`crate::p6_variables::kuna_calleepop::exact_pop`]).
//!
//! A direct call's declared pop (a DWARF or demangled declaration of the
//! image's own function) keeps the `INT_ADD`, and so does every pop when no
//! pop is guessed.

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::fspec::EXTRAPOP_UNKNOWN;
use crate::funcdata::Funcdata;

/// Whether call spec `i`'s stated pop is left to the stack solver as an exact
/// equation on its INDIRECT. `settled` is
/// [`crate::p4_calls::kuna_calleepurge::every_pop_settled`].
pub fn deferred(data: &Funcdata, i: int4, settled: bool) -> bool {
    let fc = data.get_call_specs(i);
    !settled
        && fc.proto().has_model()
        && fc.get_model_extra_pop() == EXTRAPOP_UNKNOWN
        && fc.get_extra_pop() != EXTRAPOP_UNKNOWN
        && fc.proto().is_input_locked()
        && data.obank().get(fc.get_op()).is_some_and(|op| op.code() == OpCode::CPUI_CALLIND)
}

/// After the stack solve: rewrite the stack-pointer INDIRECT of each call
/// whose pop is stated under a model that leaves it unknown, and that the
/// solve left in place, as `INT_ADD(sp_call, #pop)`. Returns the count.
pub fn write_unplaced(data: &mut Funcdata, spacebase: &Address) -> int4 {
    let mut written = 0;
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let pop = fc.get_extra_pop();
        if pop == EXTRAPOP_UNKNOWN || !fc.proto().has_model() || fc.get_model_extra_pop() != EXTRAPOP_UNKNOWN {
            continue;
        }
        let call = fc.get_op();
        let Some(at) = data.obank().get(call).map(|o| o.get_addr().clone()) else { continue };
        let indirect = data.obank().iter_at(&at).map(|(_, id)| id).find(|&id| {
            data.obank().get(id).is_some_and(|o| {
                o.code() == OpCode::CPUI_INDIRECT
                    && crate::p6_variables::kuna_calleepop::at_address(data, o.get_out(), spacebase)
                    && crate::p6_variables::kuna_calleepop::at_address(data, o.get_in(0), spacebase)
                    && o.get_in(1).and_then(|v| data.vbank().get(v)).is_some_and(|v| {
                        v.get_addr().get_space().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_IOP)
                            && crate::funcdata_varnode::op_iop_decode(v.get_addr().get_offset()) == call
                    })
            })
        });
        let Some(op) = indirect else { continue };
        let Some(sp) = data.obank().get(op).and_then(|o| o.get_in(0)) else { continue };
        let size = data.vbank().get(sp).map(|v| v.get_size()).unwrap_or(0);
        let constvn = data.new_constant(size, (pop as i64 as uintb) & kuna_base::address::calc_mask(size));
        data.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
        if data.op_set_all_input(op, &[sp, constvn]).is_ok() {
            data.get_call_specs_mut(i).set_effective_extra_pop(pop);
            written += 1;
        }
    }
    written
}
