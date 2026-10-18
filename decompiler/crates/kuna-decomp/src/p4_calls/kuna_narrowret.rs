//! (kuna `narrowret`) A call reads the narrow value its callee returns.
//!
//! A function that returns a `uint8_t`, a `uint16_t` or a `bool` hands it back
//! in a whole register, and a caller that relies on the callee having
//! zero-extended it reads the register at full width, so the CALL's output
//! Varnode is register-wide and every local holding the result is declared
//! `unsigned int`. The callee's own decompile already narrows a returned value
//! whose upper bits the nonzero-bit analysis proves clear: the RETURN pull of
//! `SubvariableFlow` trims the RETURN's input to the narrow value, and
//! [`note_trim`] marks a RETURN every trim of which, starting from the whole
//! register, dropped only known-zero bits. In the callee-first `decompile-all`
//! run [`record`] states, for a function whose every live RETURN is so marked,
//! or hands back unchanged the result of a stated callee, that storage and the
//! unsigned integer or `bool` the function declares it returns. At a later call
//! to it whose recovered output is a wider part of the same register holding
//! that storage as its low part, and whose variable takes no other value,
//! [`narrow_call_output`] makes the CALL write the narrow value and
//! zero-extends it back into the old output: every reader sees the bits it read
//! before, now as the zero extension the callee performed.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
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
    /// The type the callee's listing declares it returns: an unsigned integer
    /// of this width, or a `bool` that every RETURN computes as a comparison or
    /// a constant 0 or 1, not all of them constants.
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

/// What a caller computed with (`used`) and read (`read`) of a call output
/// before [`narrow_call_output`] narrowed it, which `voidret` files for the
/// call in place of what it reads of the narrowed output: the rules fold the
/// caller's own extension of the register (`mov w8, w0`) into the zero
/// extension the narrowing proved, and the result then looks read whole.
#[derive(Clone, Debug)]
pub struct ReadBefore {
    pub used: Option<(Address, int4)>,
    pub read: Option<(Address, int4)>,
}

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
    let mut tails = false;
    let mut nz = 0;
    for op in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let o = data.obank().get(op)?;
        if o.is_dead() || o.get_halt_type() != 0 {
            continue;
        }
        let vn = o.get_in(1)?;
        let v = data.vbank().get(vn)?;
        if v.get_size() != size || (!v.is_constant() && *v.get_addr() != addr) {
            return None;
        }
        any = true;
        if o.get_addlflags() & kuna_narrowret == 0 {
            let inner = tail_statement(data, vn)?;
            tails = true;
            computed = true;
            truth = truth && inner.truth();
            nz |= if inner.truth() { 1 } else { calc_mask(size) };
            continue;
        }
        computed |= !v.is_constant();
        nz |= v.get_nz_mask();
        truth = truth && crate::kuna_boolbyte::def_is_truth_valued(data, vn, 0);
    }
    if !any || (!tails && !crate::kuna_returnuncomputed::every_return_computes_with(data, true)) {
        return None;
    }
    let truth = truth && computed;
    if !truth && (nz >> (8 * size - 1)) & 1 == 0 {
        return None;
    }
    if truth && return_register_width(data)? > 4 {
        return None;
    }
    let ct = out.get_type().filter(|t| t.get_size() == size).filter(|t| match t.get_metatype() {
        type_metatype::TYPE_UINT => !truth,
        type_metatype::TYPE_BOOL => truth,
        _ => false,
    })?;
    Some(NarrowReturn {
        addr,
        size,
        ct: Some(Rc::clone(ct)),
    })
}

/// The statement of the callee whose result the RETURN input `vn` hands back
/// unchanged, in the same storage: the output of a direct CALL, read directly,
/// through copies of the same storage onto itself, or as the CALL's
/// output-trial creation (`passthrough`'s tail return).  A function that
/// returns a stated callee's result returns that callee's zero extension.
fn tail_statement(data: &Funcdata, vn: VarnodeId) -> Option<NarrowReturn> {
    let v = data.vbank().get(vn)?;
    let mut def = v.get_def();
    for _ in 0..4 {
        let Some(d) = def
            .and_then(|d| data.obank().get(d))
            .filter(|d| d.code() == OpCode::CPUI_COPY)
        else {
            break;
        };
        let src = data.vbank().get(d.get_in(0)?)?;
        if src.get_addr() != v.get_addr() || src.get_size() != v.get_size() {
            return None;
        }
        def = src.get_def();
    }
    let d = data.obank().get(def?)?;
    let call = match d.code() {
        OpCode::CPUI_CALL => def?,
        OpCode::CPUI_INDIRECT if d.is_indirect_creation() => {
            let iop = data.vbank().get(d.get_in(1)?)?;
            OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()))
        }
        _ => return None,
    };
    data.obank().get(call).filter(|c| c.code() == OpCode::CPUI_CALL)?;
    let fc = data.get_call_specs(data.get_call_specs_index(call)?);
    let stated = data.kuna_narrowret(key(fc.get_entry_address())?)?;
    (stated.size == v.get_size() && stated.addr == *v.get_addr()).then(|| stated.clone())
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

/// Do two statements say the same: one storage, one width, one type?
pub fn same_statement(a: Option<&NarrowReturn>, b: Option<&NarrowReturn>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            a.addr == b.addr
                && a.size == b.size
                && match (&a.ct, &b.ct) {
                    (None, None) => true,
                    (Some(x), Some(y)) => Rc::ptr_eq(x, y) || x.compare(y, 10).is_ok_and(|c| c == 0),
                    _ => false,
                }
        }
        _ => false,
    }
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
/// as its low part, and its variable must take no other value
/// ([`sole_value`]; `callees` maps each CALL to its callee, while the call
/// specifications are out of `data`); one already that narrow is left as it
/// is.  A stated truth value marks the CALL a calculated boolean, as a callee
/// declared `bool` does.
pub fn narrow_call_output(data: &mut Funcdata, fc: &FuncCallSpecs, callees: &[(OpId, (int4, uintb))]) {
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
    let callee = |c: OpId| callees.iter().find(|(o, _)| *o == c).map(|(_, k)| *k);
    if !holds_low_part(data, out, &stated)
        || !sole_value(data, out, &stated, &callee)
        || !sole_tail(data, op, &stated, &callee)
    {
        return;
    }
    let size = data.vbank().get(out).map(|v| v.get_size()).unwrap_or(0);
    if size > stated.size {
        let Some(at) = data.obank().get(op).map(|o| o.get_addr().clone()) else {
            return;
        };
        let before = ReadBefore {
            used: crate::kuna_voidret::used_storage(data, out),
            read: data.vbank().get(out).and_then(crate::kuna_voidret::read_storage),
        };
        data.kuna_note_narrowret_read(op, before);
        data.op_unset_output(op);
        let ext = data.new_op(1, at);
        data.op_set_opcode_code(ext, OpCode::CPUI_INT_ZEXT);
        if let Some(e) = data.obank_mut().get_mut(ext) {
            e.set_additional_flag(kuna_narrowret);
        }
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

/// The `unsigned char` or `bool` the callee of the CALL `op` (spec `fc`)
/// states it returns, for a call whose one-byte output is that narrow value
/// and whose variable takes no other value ([`sole_value`]): the def-side vote
/// of `TypeOpCall::getOutputLocal` for an unlocked output, and so the type the
/// call expression has where `ActionSetCasts` converts it.  A caller that
/// compares the byte with `0x80` otherwise types it from the comparison alone,
/// while C reads the call at the declared type.  A wider narrow value is
/// `callrettype`'s to type.
pub fn stated_type(data: &Funcdata, op: OpId, fc: &FuncCallSpecs, size: int4) -> Option<Rc<Datatype>> {
    if !data.get_arch().narrow_ret || size != 1 {
        return None;
    }
    let out = data
        .obank()
        .get(op)
        .filter(|o| o.code() == OpCode::CPUI_CALL)?
        .get_out()?;
    let k = key(fc.get_entry_address())?;
    let stated = data.kuna_narrowret(k)?;
    if stated.size != size || !holds_low_part(data, out, stated) {
        return None;
    }
    sole_value(data, out, stated, &|c| callee_key(data, c)).then(|| stated.ct.clone()).flatten()
}

/// Does the CALL `op` read a callee that states it returns a truth value, in
/// exactly the storage and width of the call's output?  `boolbyte` counts such
/// a call as a truth-valued definition of the byte that holds its result.
pub(crate) fn states_truth(data: &Funcdata, op: OpId) -> bool {
    if !data.get_arch().narrow_ret {
        return false;
    }
    let Some(out) = data.obank().get(op).and_then(|o| o.get_out()).and_then(|v| data.vbank().get(v)) else {
        return false;
    };
    callee_key(data, op)
        .and_then(|k| data.kuna_narrowret(k))
        .is_some_and(|s| s.truth() && s.size == out.get_size() && s.addr == *out.get_addr())
}

/// Each call's op and the entry of its callee, for [`narrow_call_output`].
pub fn callee_keys(qlst: &[FuncCallSpecs]) -> Vec<(OpId, (int4, uintb))> {
    qlst.iter().filter_map(|fc| Some((fc.get_op(), key(fc.get_entry_address())?))).collect()
}

/// The entry of the callee of the CALL `op`, from its call specification.
fn callee_key(data: &Funcdata, op: OpId) -> Option<(int4, uintb)> {
    key(data.get_call_specs(data.get_call_specs_index(op)?).get_entry_address())
}

/// Does the variable the call output `out` starts take no value but the
/// stated callee's results and constants of its width?  The variable is the
/// web of copies, merges, indirect effects and narrowed calls' extensions
/// through `out`; a value reaching a RETURN brings in every value the function
/// returns.
fn sole_value(
    data: &Funcdata,
    out: VarnodeId,
    stated: &NarrowReturn,
    callee: &dyn Fn(OpId) -> Option<(int4, uintb)>,
) -> bool {
    let same_callee = |op: OpId| -> bool {
        callee(op)
            .and_then(|k| data.kuna_narrowret(k))
            .is_some_and(|s| same_statement(Some(s), Some(stated)))
    };
    let mut work = vec![out];
    let mut seen: Vec<VarnodeId> = Vec::new();
    let mut returned = false;
    while let Some(v) = work.pop() {
        if seen.contains(&v) {
            continue;
        }
        if seen.len() >= 64 {
            return false;
        }
        seen.push(v);
        let Some(n) = data.vbank().get(v) else { return false };
        if n.is_constant() {
            if n.get_offset() > if stated.truth() { 1 } else { calc_mask(stated.size) } {
                return false;
            }
            continue;
        }
        let Some(def) = n.get_def() else { return false };
        let Some(d) = data.obank().get(def) else { return false };
        match d.code() {
            OpCode::CPUI_CALL if same_callee(def) => {}
            OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => work.extend((0..d.num_input()).filter_map(|i| d.get_in(i))),
            OpCode::CPUI_INDIRECT if d.is_indirect_creation() => {
                let Some(iop) = d.get_in(1).and_then(|i| data.vbank().get(i)) else { return false };
                let call = OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()));
                if !same_callee(call) {
                    return false;
                }
            }
            OpCode::CPUI_INDIRECT => work.extend(d.get_in(0)),
            OpCode::CPUI_INT_ZEXT if d.get_addlflags() & kuna_narrowret != 0 => work.extend(d.get_in(0)),
            other => {
                if trace() {
                    eprintln!(
                        "[narrowret] 0x{:x} the variable of a call output also takes {other:?} at 0x{:x}",
                        data.get_address().get_offset(),
                        d.get_addr().get_offset()
                    );
                }
                return false;
            }
        }
        for op in n.descend_iter() {
            let Some(o) = data.obank().get(op) else { continue };
            match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => work.extend(o.get_out()),
                OpCode::CPUI_INT_ZEXT if o.get_addlflags() & kuna_narrowret != 0 => work.extend(o.get_out()),
                OpCode::CPUI_RETURN => returned = true,
                _ => {}
            }
        }
    }
    if !returned {
        return true;
    }
    data.obank().iter_code(OpCode::CPUI_RETURN).all(|r| {
        let Some(o) = data.obank().get(r) else { return true };
        if o.is_dead() || o.get_halt_type() != 0 || o.num_input() < 2 {
            return true;
        }
        o.get_in(1).is_some_and(|v| seen.contains(&v))
    })
}

/// When the CALL `op` is the last call before a live RETURN, is every live
/// RETURN's last call one to a callee with the same statement?  The function's
/// own output may not be recovered yet, its RETURNs not yet reading a value,
/// and a wrapper that returns the narrowed result on one path and another value
/// on the next (`jmp h3` beside `call h4; movzbl %al,%eax; ret`) would trim
/// its own return to the byte and declare it at the wrong sign.
fn sole_tail(data: &Funcdata, op: OpId, stated: &NarrowReturn, callee: &dyn Fn(OpId) -> Option<(int4, uintb)>) -> bool {
    let producers: Vec<Option<OpId>> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .map(|r| crate::p4_calls::kuna_passthrough::producing_call(data, r))
        .collect();
    if !producers.contains(&Some(op)) {
        return true;
    }
    producers.iter().all(|p| {
        p.is_some_and(|c| {
            c == op
                || callee(c)
                    .and_then(|k| data.kuna_narrowret(k))
                    .is_some_and(|s| same_statement(Some(s), Some(stated)))
        })
    })
}

/// The register-wide value a caller reads the result of a CALL through, when
/// [`narrow_call_output`] narrowed its output `out`: the zero extension it
/// wrote back into the register, read directly or through the conversion
/// `ActionSetCasts` put between them, while that extension is still there.
pub fn widened_read(data: &Funcdata, out: VarnodeId) -> VarnodeId {
    let mut cur = out;
    for _ in 0..3 {
        let Some(v) = data.vbank().get(cur) else { break };
        let mut next = None;
        for op in v.descend_iter() {
            let Some(o) = data.obank().get(op) else { continue };
            match o.code() {
                OpCode::CPUI_INT_ZEXT if o.get_addlflags() & kuna_narrowret != 0 => {
                    if let Some(wide) = o.get_out() {
                        return wide;
                    }
                }
                OpCode::CPUI_CAST | OpCode::CPUI_COPY => next = next.or(o.get_out()),
                _ => {}
            }
        }
        match next {
            Some(n) => cur = n,
            None => break,
        }
    }
    out
}

/// Is the decision trace on (`KUNA_NARROWRET_TRACE=1`)?
fn trace() -> bool {
    std::env::var("KUNA_NARROWRET_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
}
