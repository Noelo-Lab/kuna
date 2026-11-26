//! (kuna) The pop a call through a pointer takes from its declaration is an
//! equation for the stack solver, not an `INT_ADD` folded in before the solve.
//!
//! An x86-32 `call [slot]` that [`crate::p4_calls::kuna_slotargs`] declares
//! (a Win32 `__stdcall` import) states its pop, `4 + <argument bytes>`, under
//! a model that leaves extrapop unknown. `ActionExtraPopSetup` used to write a
//! stated pop into the IR as `sp_after = INT_ADD(sp_call, #pop)`. In a function
//! whose other pops `StackSolver` still guesses, that joins the stack pointer
//! after the call to the one before it before anything is solved, and term
//! collection then rewrites stack arithmetic after the call in terms of a stack
//! pointer from before it. A pointer difference after `GetModuleFileNameA`
//! became `-(sp at its last push)`, and the local-alias boundary took that
//! slot as an escaped address, so every outgoing argument above it was refused
//! for the rest of the function.
//!
//! So the declared pop takes the INDIRECT the other calls take, and the
//! prototype keeps the pop: `StackSolver::build` reads it as the call's exact
//! equation, the answer the `INT_ADD` gave, and the solve writes it into the IR
//! with all the others. Where the solve cannot place the stack pointer (a frame
//! sized at run time, a stack pointer an exception handler loads from memory),
//! the INDIRECT is written as the `INT_ADD` it would have been
//! ([`write_unplaced`]). A direct call's declared pop (a DWARF or demangled
//! declaration of the image's own function) keeps the `INT_ADD`.
//!
//! This is a change of form only where the solve does not depend on the form
//! ([`allowed`]), and it does in two cases, which keep the `INT_ADD`:
//!
//! * A guess that reads the caller's stack. The push-run guess of a call into
//!   an import or through a pointer
//!   ([`crate::p6_variables::kuna_calleepopslot`]) counts the slots pushed in
//!   front of it. With the `INT_ADD` folded in, every reference after the
//!   declared call is an offset from one before it, and the walk reads
//!   whichever reference sits at a slot's offset, often a slot of another call;
//!   with the INDIRECT it reads the call's own slots. Neither reading is the
//!   pop, and changing one for the other moves its errors. A direct call into
//!   the image's own code is guessed to pop nothing either way.
//! * An exact equation that is false. A direct call's pop stated under that
//!   model is computed from its declared parameters as if the callee popped
//!   them, which a GCC `__cdecl` or `regparm` callee does not; its own `RET`
//!   says so. One such equation leaves the system without a consistent
//!   solution, and where the error lands depends on the order of the solve,
//!   which the form changes.

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::fspec::EXTRAPOP_UNKNOWN;
use crate::funcdata::Funcdata;

/// Whether call spec `i`'s stated pop is left to the stack solver as an exact
/// equation on its INDIRECT, in a function where that is [`allowed`].
pub fn deferred(data: &Funcdata, i: int4, allowed: bool) -> bool {
    allowed && candidate(data, i)
}

/// Does call spec `i` state its pop by a declaration of a call through a
/// pointer?
fn candidate(data: &Funcdata, i: int4) -> bool {
    let fc = data.get_call_specs(i);
    stated(data, i)
        && fc.proto().is_input_locked()
        && data.obank().get(fc.get_op()).is_some_and(|op| op.code() == OpCode::CPUI_CALLIND)
}

/// Whether the function leaves a pop to guess or settles one only by its
/// callee's return constants (`settled` is
/// [`crate::p4_calls::kuna_calleepurge::every_pop_proven`]), every pop it
/// leaves is a direct call's into the image's own code, and every pop a
/// direct call's prototype states is the one its callee's returns prove. A
/// call through a pointer the function leaves to guess may resolve to an
/// import before the solve, so it counts as one.
pub fn allowed(data: &Funcdata, settled: bool) -> bool {
    !settled
        && (0..data.num_calls()).any(|i| candidate(data, i))
        && (0..data.num_calls()).all(|i| {
            let fc = data.get_call_specs(i);
            let direct = data.obank().get(fc.get_op()).is_some_and(|op| op.code() == OpCode::CPUI_CALL);
            if fc.get_extra_pop() == EXTRAPOP_UNKNOWN {
                direct && !crate::p6_variables::kuna_calleepop::is_imported(data, fc.get_entry_address())
            } else {
                !stated(data, i) || !direct || fc.callee_purge() == Some(fc.get_extra_pop())
            }
        })
}

/// Does call spec `i` state a pop its model leaves unknown?
fn stated(data: &Funcdata, i: int4) -> bool {
    let fc = data.get_call_specs(i);
    fc.proto().has_model() && fc.get_model_extra_pop() == EXTRAPOP_UNKNOWN && fc.get_extra_pop() != EXTRAPOP_UNKNOWN
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
