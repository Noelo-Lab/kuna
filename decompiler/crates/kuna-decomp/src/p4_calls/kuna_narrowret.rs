//! (kuna `narrowret`) A call reads the narrow value its callee returns.
//!
//! A function that returns a `uint8_t`, a `uint16_t`, a `bool` or (x86-64,
//! AArch64) an `int` hands it back in a whole register, and a caller that
//! relies on the callee having zero-extended it (AAPCS requires it; an x86-64
//! callee's `movzbl` or 32-bit write performs it) reads the register at full
//! width, so the CALL's output Varnode is register-wide and every local holding
//! the result is declared `unsigned int` or `unsigned long`. The callee's own
//! decompile already narrows a returned value whose upper bits the nonzero-bit
//! analysis proves clear: the RETURN pull of `SubvariableFlow` trims the
//! RETURN's input to the narrow value, and [`note_trim`] marks a RETURN every
//! trim of which, starting from the whole register, dropped only known-zero
//! bits. In the callee-first `decompile-all` run [`record`] states, for a
//! function whose every live RETURN is so marked and returns the same narrow
//! storage, that storage, with the integer type the function declares it
//! returns (a `bool` only when every RETURN computes a truth value). At a later
//! call to it whose recovered output is a wider part of the same register
//! holding that storage as its low part, [`narrow_call_output`] makes the CALL
//! write the narrow value and zero-extends it back into the old output: every
//! reader sees the bits it read before, now as the zero extension the callee
//! performed, and the value can be declared at its own width. The call then
//! produces the type the callee declares ([`stated_type`]), and a stated truth
//! value marks the CALL a calculated boolean.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_class, type_metatype, Datatype};
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::op::pcodeop_addlflags::kuna_narrowret;

/// The narrow value a callee returns zero-extended: its storage and width.
#[derive(Debug, Clone)]
pub struct NarrowReturn {
    /// The storage of the narrow value, the low part of the return register.
    pub addr: Address,
    /// Its width in bytes.
    pub size: int4,
    /// The integer type the callee's listing declares it returns, when that
    /// is an integer of this width, or a `bool` that every RETURN computes as a
    /// comparison or a constant 0 or 1, not all of them constants.
    pub ct: Option<Rc<Datatype>>,
}

impl NarrowReturn {
    /// Is the value a truth value?
    pub fn truth(&self) -> bool {
        self.ct
            .as_ref()
            .is_some_and(|t| t.get_metatype() == type_metatype::TYPE_BOOL)
    }
}

/// The run's statements, keyed like `Architecture::kuna_callret_types`.
pub type NarrowReturns = HashMap<(int4, uintb), Rc<NarrowReturn>>;

fn key(entry: &Address) -> Option<(int4, uintb)> {
    Some((entry.get_space()?.get_index(), entry.get_offset()))
}

/// The width of the function's integer return register: the widest general
/// output entry of its model.
fn return_register_width(data: &Funcdata) -> Option<int4> {
    let proto = data.get_func_proto();
    if !proto.has_model() {
        return None;
    }
    proto
        .model()
        .output()
        .get_entry()
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_GENERAL)
        .map(|e| e.get_size())
        .max()
}

/// Record on the RETURN `op`, whose input `before` is about to be trimmed to
/// the value under `mask`, whether the RETURN still hands back the zero
/// extension of a narrow value: every trim from the whole register on dropped
/// only known-zero bits.
pub(crate) fn note_trim(data: &mut Funcdata, op: OpId, before: VarnodeId, mask: u64) {
    if !data.get_arch().narrow_ret {
        return;
    }
    let Some(o) = data.obank().get(op) else { return };
    if o.code() != OpCode::CPUI_RETURN {
        return;
    }
    let marked = o.get_addlflags() & kuna_narrowret != 0;
    let zero = crate::kuna_truncarg::drops_only_zero_bits(data, before, mask);
    let whole = data.vbank().get(before).map(|v| v.get_size()) == return_register_width(data);
    let Some(o) = data.obank_mut().get_mut(op) else { return };
    if zero && (whole || marked) {
        o.set_additional_flag(kuna_narrowret);
    } else {
        o.clear_additional_flag(kuna_narrowret);
    }
}

/// The narrow storage every live RETURN of `data` hands back zero-extended,
/// when they all do ([`note_trim`]) and the function's recovered output is
/// exactly that storage.
fn narrow_return(data: &Funcdata) -> Option<NarrowReturn> {
    let proto = data.get_func_proto();
    if !proto.has_store() || !proto.has_model() || proto.is_model_unknown() || proto.is_output_locked() {
        return None;
    }
    let out = proto.get_output();
    let (addr, size) = (out.get_address(), out.get_size());
    if addr.is_invalid() || !(1..=4).contains(&size) || size >= return_register_width(data)? {
        return None;
    }
    if addr
        .get_space()
        .is_none_or(|s| s.get_type() != spacetype::IPTR_PROCESSOR)
    {
        return None;
    }
    let mut any = false;
    let mut truth = true;
    let mut computed = false;
    for op in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let o = data.obank().get(op)?;
        if o.is_dead() || o.get_halt_type() != 0 {
            continue;
        }
        let v = data.vbank().get(o.get_in(1)?)?;
        if o.get_addlflags() & kuna_narrowret == 0 || v.get_size() != size {
            return None;
        }
        if !v.is_constant() && *v.get_addr() != addr {
            return None;
        }
        any = true;
        computed |= !v.is_constant();
        truth = truth && crate::kuna_boolbyte::def_is_truth_valued(data, o.get_in(1)?, 0);
    }
    if !any || !crate::kuna_returnuncomputed::every_return_computes_with(data, true) {
        return None;
    }
    let ct = out
        .get_type()
        .filter(|t| t.get_size() == size)
        .filter(|t| match t.get_metatype() {
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT => true,
            type_metatype::TYPE_BOOL => truth && computed,
            _ => false,
        });
    Some(NarrowReturn {
        addr,
        size,
        ct: ct.cloned(),
    })
}

/// Keep the narrow value `data`, the function entered at `entry`, returns, for
/// the callers decompiled after it.  A statement from an earlier decompile of
/// the same function is replaced or, when this one states nothing, forgotten.
pub fn record(arch: &mut Architecture, entry: &Address, data: &Funcdata) {
    if !arch.narrow_ret {
        return;
    }
    let Some(k) = key(entry) else { return };
    let stated = if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        None
    } else {
        narrow_return(data)
    };
    if trace() {
        if let Some(s) = &stated {
            let truth = if s.truth() { ", a truth value" } else { "" };
            eprintln!(
                "[narrowret] 0x{:x} returns {} bytes zero-extended{truth}",
                entry.get_offset(),
                s.size
            );
        }
    }
    match stated {
        Some(s) => {
            arch.kuna_narrowret.insert(k, Rc::new(s));
        }
        None => {
            arch.kuna_narrowret.remove(&k);
        }
    }
}

/// What the function filed under `k` states it returns, for [`restore`].
pub fn statement(arch: &Architecture, k: (int4, uintb)) -> Option<Rc<NarrowReturn>> {
    arch.kuna_narrowret.get(&k).cloned()
}

/// Put back `stated`, what the function filed under `k` stated before a
/// decompile the run then discarded.
pub fn restore(arch: &mut Architecture, k: (int4, uintb), stated: Option<Rc<NarrowReturn>>) {
    match stated {
        Some(s) => {
            arch.kuna_narrowret.insert(k, s);
        }
        None => {
            arch.kuna_narrowret.remove(&k);
        }
    }
}

/// Copy onto `data` what every callee it calls stated.  A function never reads
/// its own statement.
pub fn seed(arch: &Architecture, data: &mut Funcdata) {
    if !arch.narrow_ret || arch.kuna_narrowret.is_empty() {
        return;
    }
    let own = key(data.get_address());
    let entries: Vec<Address> = (0..data.num_calls())
        .map(|i| data.get_call_specs(i).get_entry_address().clone())
        .filter(|e| !e.is_invalid())
        .collect();
    for e in entries {
        let Some(k) = key(&e) else { continue };
        if Some(k) == own {
            continue;
        }
        if let Some(s) = arch.kuna_narrowret.get(&k) {
            data.kuna_set_narrowret(k, Rc::clone(s));
        }
    }
}

/// The register a CALL's output `out` occupies holds the stated narrow
/// storage as its low part: the low-addressed bytes on a little-endian space,
/// the high-addressed ones on a big-endian one.
fn holds_low_part(data: &Funcdata, out: VarnodeId, stated: &NarrowReturn) -> bool {
    let Some(v) = data.vbank().get(out) else { return false };
    let (addr, size) = (v.get_addr(), v.get_size());
    if size < stated.size || v.is_type_lock() || v.is_addr_tied() || v.is_persist() {
        return false;
    }
    let Some(space) = addr.get_space() else { return false };
    if space.get_type() != spacetype::IPTR_PROCESSOR {
        return false;
    }
    let low = if space.is_big_endian() {
        addr + (size - stated.size) as i64
    } else {
        addr.clone()
    };
    low == stated.addr
}

/// Make the CALL of `fc`, whose output its own recovery just built, write the
/// narrow value its callee states it returns, zero-extended back into the
/// output.  The output must be a register Varnode holding the stated storage
/// as its low part; one already that narrow is left as it is.  A stated truth
/// value marks the CALL a calculated boolean.
pub fn narrow_call_output(data: &mut Funcdata, fc: &FuncCallSpecs) {
    if !data.get_arch().narrow_ret {
        return;
    }
    let Some(k) = key(fc.get_entry_address()) else { return };
    let Some(stated) = data.kuna_narrowret(k).cloned() else {
        return;
    };
    let op = fc.get_op();
    let Some(out) = data
        .obank()
        .get(op)
        .filter(|o| o.code() == OpCode::CPUI_CALL)
        .and_then(|o| o.get_out())
    else {
        return;
    };
    if !holds_low_part(data, out, &stated) {
        return;
    }
    let size = data.vbank().get(out).map(|v| v.get_size()).unwrap_or(0);
    if size > stated.size {
        let Some(at) = data.obank().get(op).map(|o| o.get_addr().clone()) else {
            return;
        };
        data.op_unset_output(op);
        let ext = data.new_op(1, at);
        data.op_set_opcode_code(ext, OpCode::CPUI_INT_ZEXT);
        if data.op_set_output(ext, out).is_err() {
            let _ = data.op_destroy_raw(ext);
            let _ = data.op_set_output(op, out);
            return;
        }
        let Ok(narrow) = data.new_varnode_out(stated.size, &stated.addr, op) else {
            data.op_unset_output(ext);
            let _ = data.op_destroy_raw(ext);
            let _ = data.op_set_output(op, out);
            return;
        };
        if let Some(n) = data.vbank_mut().get_mut(narrow) {
            n.set_write_mask();
        }
        if data.op_set_input(ext, narrow, 0).is_err() {
            return;
        }
        data.op_insert_after(ext, op);
        if trace() {
            eprintln!(
                "[narrowret] 0x{:x} call at 0x{:x} reads {} of {size} bytes",
                data.get_address().get_offset(),
                data.obank().get(op).map(|o| o.get_addr().get_offset()).unwrap_or(0),
                stated.size
            );
        }
    }
    if stated.truth() {
        data.op_mark_calculated_bool(op);
    }
}

/// The type the callee of the CALL `op` (spec `fc`) declares it returns, for a
/// call whose `size`-byte output is the narrow value the callee states: the
/// def-side vote of `TypeOpCall::getOutputLocal` for an unlocked output, and so
/// the type the call expression has where `ActionSetCasts` converts it.
pub fn stated_type(data: &Funcdata, op: OpId, fc: &FuncCallSpecs, size: int4) -> Option<Rc<Datatype>> {
    if !data.get_arch().narrow_ret {
        return None;
    }
    let stated = data.kuna_narrowret(key(fc.get_entry_address())?)?;
    let out = data
        .obank()
        .get(op)
        .filter(|o| o.code() == OpCode::CPUI_CALL)?
        .get_out()?;
    if stated.size != size || !holds_low_part(data, out, stated) {
        return None;
    }
    stated.ct.clone()
}

/// Is the decision trace on (`KUNA_NARROWRET_TRACE=1`)?
fn trace() -> bool {
    std::env::var("KUNA_NARROWRET_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
}
