//! (kuna) Pass a floating-point argument register a callee that stated nothing
//! provably reads.
//!
//! A direct call to a function with no recovered prototype takes its arguments
//! from the caller's data flow alone, and three gaps there dropped a floating
//! argument the callee reads on entry:
//!
//! * x86-64 `pxor %xmm0,%xmm0` (or `movaps %xmm0,%xmm1`) anywhere in the
//!   caller heritages the whole 16-byte `xmm0`, the `XMM0_Qa` slot sits inside
//!   it, and `calloverlap auto` guards no such slot, so `movss k,%xmm0; call
//!   set_gi_bits` registered no trial;
//! * the ARM hard-float model refuses a `d`-register trial without a callee
//!   contract, so `vmul.f64 d0,d8,d0; bl scale` passed nothing;
//! * a `d0` or `xmm0` the caller forwards from its own entry (`double a3(double
//!   x) { if (x > 1.0) return half(x); ... }`) scores as the caller's input, not
//!   an argument.
//!
//! The callee's own body settles all three. The bounded entry decode
//! [`crate::p4_calls::kuna_calleedeadarg`] takes for every direct call shows
//! whether some path reads the register's low byte before writing it, for a
//! value that reaches something other than a store of the register itself
//! ([`crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead::reads_low_byte`]).
//! A model float slot the callee reads that way is an argument whatever the
//! caller left there. Where the model also has a narrower float slot at the
//! same address (ARM `s0` inside `d0`), the callee must read the whole slot, so
//! two floats read as the halves of `d0` stay two words.
//!
//! A variadic callee's register-save prologue reads every float argument
//! register, so a callee that reads the model's last one, or that tests the
//! SysV vector-register count in `al` first ([`tests_vector_count`]), is left
//! alone. So are a callee's `protoorder` statement, a locked or variadic
//! prototype, an import, and a body the decode could not finish. A statement
//! that names a floating-point parameter in the slot still has the slot's
//! trial registered inside a wider heritaged register ([`contained_read`]), for
//! the caller's own scoring to decide, and a refused one stays inactive
//! ([`keep_stated`]): it says the callee takes the register, and no more.

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::dtype::type_class;
use crate::fspec::{FuncCallSpecs, ParamEntry};
use crate::funcdata::Funcdata;
use crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead;

/// Does the callee of `call`, which stated nothing, read the model float
/// argument storage `[addr, addr+size)` on entry ([`read_width`])?
pub fn reads(data: &Funcdata, call: &FuncCallSpecs, addr: &Address, size: int4) -> bool {
    read_width(data, call, addr, size).is_some()
}

/// How many low bytes of the float argument storage `[addr, addr+size)`, the
/// low end of a model float slot, the callee of `call` reads on entry, when it
/// stated nothing and reads it at all.
pub fn read_width(data: &Funcdata, call: &FuncCallSpecs, addr: &Address, size: int4) -> Option<int4> {
    if call.is_input_locked() || call.is_dotdotdot() || !call.proto().has_model() {
        return None;
    }
    let entries = call.proto().model().input().get_entry();
    let float = |e: &&ParamEntry| e.get_type() == type_class::TYPECLASS_FLOAT;
    if !entries.iter().filter(float).any(|e| size <= e.get_size() && e.justified_contain(addr, size) == 0) {
        return None;
    }
    let entry = call.get_entry_address();
    if entry.is_invalid() || data.kuna_protoorder_types(entry).is_some() {
        return None;
    }
    let probe = data.kuna_callee_entry_dead(entry)?;
    if !probe.reads_low_byte(addr, size, false, true) {
        return None;
    }
    let width = probe.low_read_width(addr, size)?;
    let narrower = entries
        .iter()
        .filter(float)
        .any(|e| e.get_size() < size && e.justified_contain(addr, e.get_size()) == 0);
    if (narrower && width != size) || next_slot_only_stored(probe, entries, addr, size) {
        return None;
    }
    let saves_all = entries.iter().filter(float).last().is_some_and(|e| {
        let last = Address::new(std::rc::Rc::clone(e.get_space()), e.get_base());
        probe.reads_low_byte(&last, e.get_size(), true, false)
    });
    (!saves_all && !tests_vector_count(probe, call)).then_some(width)
}

/// Does the callee only store the model float slot after `[addr, addr+size)`
/// of the same width? newlib's ARM `__ieee754_fmodf` spills `s1` with `vstr`
/// and reloads it as an integer: an argument the entry decode does not count,
/// so activating `s0` alone would end the list the callee-body rules fill
/// whole (`fmodf(x,y)` calling it with both).
fn next_slot_only_stored(probe: &CalleeEntryDead, entries: &[ParamEntry], addr: &Address, size: int4) -> bool {
    let same: Vec<&ParamEntry> = entries
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() == size)
        .collect();
    let Some(i) = same.iter().position(|e| e.justified_contain(addr, size) == 0) else { return false };
    same.get(i + 1).is_some_and(|e| {
        let at = Address::new(std::rc::Rc::clone(e.get_space()), e.get_base());
        probe.reads_low_byte(&at, size, true, false) && !probe.reads_low_byte(&at, size, false, false)
    })
}

/// The argument a call takes from the heritaged range `[addr, addr+size)` that
/// holds exactly one model input slot, a float slot at its low end where the
/// callee's `protoorder` statement names a floating-point parameter
/// ([`stated_width`]) or that the callee reads ([`read_width`]): the slot's
/// address and as many bytes as the statement gives or the callee reads, at
/// least the slot's smallest width.
/// x86-64 `XMM0_Qa` is the one slot inside `xmm0`, and `movss %xmm0,g` reads
/// four bytes of it; a range holding two slots (ARM `s0` and `s1` inside `d0`
/// on the base model) is not one argument's storage.
pub fn contained_read(data: &Funcdata, call: &FuncCallSpecs, addr: &Address, size: int4) -> Option<(Address, int4)> {
    if !call.proto().has_model() {
        return None;
    }
    let mut inside = call.proto().model().input().get_entry().iter().filter(|e| e.contained_by(addr, size));
    let slot = inside.next()?;
    if inside.next().is_some() || slot.get_type() != type_class::TYPECLASS_FLOAT || slot.get_base() != addr.get_offset() {
        return None;
    }
    let at = Address::new(std::rc::Rc::clone(slot.get_space()), slot.get_base());
    let width = stated_width(data, call, &at, slot.get_size())
        .or_else(|| read_width(data, call, &at, slot.get_size()))?
        .max(slot.get_min_size());
    Some((at, width))
}

/// How wide the floating-point parameter is that the `protoorder` statement of
/// the direct callee of `call` names at the low end `at` of a model float slot
/// of `size` bytes, when the call's prototype is neither locked nor variadic.
/// A parameter stated as an integer there is the register's bits, which says
/// less about what the caller passes and is left to the caller's scoring.
fn stated_width(data: &Funcdata, call: &FuncCallSpecs, at: &Address, size: int4) -> Option<int4> {
    if call.is_input_locked() || call.is_dotdotdot() {
        return None;
    }
    let entry = call.get_entry_address();
    if entry.is_invalid() {
        return None;
    }
    let stated = data.kuna_protoorder_types(entry)?;
    let (_, width, _) = stated.inputs.iter().find(|(a, w, ty)| {
        a == at && *w <= size && ty.get_metatype() == crate::dtype::type_metatype::TYPE_FLOAT
    })?;
    Some(*width)
}

/// Leave trial `i` of call `idx` inactive, never definitely not used, when it
/// sits at the low end of a model float slot where its callee's `protoorder`
/// statement names a floating-point parameter ([`stated_width`]). The caller's
/// data flow may refuse the value (ucrtbase `_jn` forwards its own `x` to
/// `_j0` through `xmm7`), but a definitely-unused trial would drop every later
/// argument of the call with it.
pub fn keep_stated(data: &mut Funcdata, idx: int4, i: int4) {
    let call = data.get_call_specs(idx);
    let t = call.active_input().get_trial(i);
    if !t.is_definitely_not_used() || !call.proto().has_model() {
        return;
    }
    let (addr, size) = (t.get_address().clone(), t.get_size());
    let float = |e: &&ParamEntry| e.get_type() == type_class::TYPECLASS_FLOAT;
    let Some(slot) = call
        .proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(float)
        .find(|e| size <= e.get_size() && e.justified_contain(&addr, size) == 0)
    else {
        return;
    };
    if stated_width(data, call, &addr, slot.get_size()).is_some() {
        data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_maybe_use();
    }
}

/// Does the callee read, on entry, a general-purpose register that carries the
/// return value and no argument? A SysV variadic callee tests the
/// vector-register count in `al` before its register-save prologue, and no
/// other callee has a reason to read `rax` before writing it.
fn tests_vector_count(probe: &CalleeEntryDead, call: &FuncCallSpecs) -> bool {
    let Some(outputs) = call.proto().model().output_list() else { return false };
    outputs.get_entry().iter().filter(|e| e.get_type() == type_class::TYPECLASS_GENERAL).any(|e| {
        let at = Address::new(std::rc::Rc::clone(e.get_space()), e.get_base());
        !call.proto().possible_input_param(&at, e.get_size()) && probe.reads_low_byte(&at, e.get_size(), true, false)
    })
}
