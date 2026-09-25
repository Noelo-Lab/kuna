//! (kuna `callrettype`) A call returns the type its callee declares.
//!
//! `kuna decompile-all` decompiles callees before callers
//! ([`crate::p4_calls::kuna_protoorder`]).  That channel carries a callee's
//! recovered PARAMETER types to its call sites; this one carries the RETURN.
//! Without it a call to a function the same listing declares
//! `char *sub_43ee(unsigned long *a0)` still produces an untyped value at the
//! call, and every caller that keeps the result as a `char *` prints
//! `(char *)sub_43ee(a0)` -- a conversion from a type the call does not have.
//!
//! After a function's final decompile in the callee-first order, [`record`]
//! keeps its return value -- storage, size, type -- when its own recovery
//! computed one on every live path.  At a later call, [`stated_return_type`]
//! offers that type as the CALL output's local type (upstream's
//! `TypeOpCall::getOutputLocal`, which answers only for a locked output): the
//! def-side vote in `Varnode::getLocalType`'s fold, and the type the call
//! produces when `ActionSetCasts` asks whether its result needs converting.
//!
//! Nothing is locked.  The call keeps exactly the output the caller's own
//! recovery gave it -- the vote is taken only where a CALL already HAS an
//! output in the very storage and width the callee returns in -- so no call
//! gains or loses a result and no argument moves.  A `void` callee states
//! nothing, and a declared prototype (libc, DWARF, `--assert`) is left to the
//! output lock it already carries.
//!
//! The finished caller is audited ([`contradicted`]): a statement whose type
//! reached a value it cannot describe once the merge tied the variables, or
//! that a later inference pass refused, is withdrawn for that caller and the
//! caller decompiled again without it.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::p4_calls::fspec::FuncCallSpecs;

/// A callee's recovered return value: where it is returned, how wide, and the
/// type the callee's own decompile gave it.
#[derive(Debug, Clone)]
pub struct StatedReturn {
    /// The storage the return value is recovered in.
    pub addr: Address,
    /// Its width in bytes.
    pub size: int4,
    /// Its recovered type.
    pub ct: Rc<Datatype>,
}

/// The run's statements, keyed like `Architecture::kuna_protoorder_types`.
pub type StatedReturns = HashMap<(int4, uintb), Rc<StatedReturn>>;

/// The largest function (in live p-code ops) [`contradicted`] audits.  The
/// second decompile costs what the first did, and a few large functions carry
/// most of that cost (bash -O2: 12 functions over this size were 24 of the 25
/// seconds 38 audited functions cost; dpkg-divert's one function of 1,992 ops
/// was 10% of its run); a larger function keeps its first decompile.
pub const AUDIT_MAX_OPS: usize = 1000;

/// A caller and a callee whose statement that caller does not take, both keyed
/// like [`StatedReturns`].
pub type Refusal = ((int4, uintb), (int4, uintb));

fn key(entry: &Address) -> Option<(int4, uintb)> {
    Some((entry.get_space()?.get_index(), entry.get_offset()))
}

/// 1 for an integer, 2 for a pointer, 0 for anything else.
fn value_class(t: &Datatype) -> u8 {
    match t.get_metatype() {
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT => 1,
        type_metatype::TYPE_PTR => 2,
        _ => 0,
    }
}

/// Is `t` a value of the other class than `ct` at the same width: an integer
/// beside a pointer or a pointer beside an integer?
fn other_class(t: &Datatype, ct: &Datatype) -> bool {
    t.get_size() == ct.get_size() && value_class(t) != 0 && value_class(t) != value_class(ct)
}

/// Is `v` a constant that is a number and not an address: nonzero and outside
/// every data section the loader reported?  Nothing is when the loader reported
/// none.
fn is_number(data: &Funcdata, v: Option<VarnodeId>) -> bool {
    let ranges = data.kuna_callret_data();
    !ranges.is_empty()
        && v.and_then(|v| data.vbank().get(v)).filter(|n| n.is_constant()).is_some_and(|n| {
            n.get_offset() != 0 && !crate::p9_emit::kuna_globalref::in_ranges(ranges, n.get_offset())
        })
}

/// Keep what `data`, the function entered at `entry`, returns, for the callers
/// decompiled after it.  A statement from an earlier decompile of the same
/// function is replaced or, when this one states nothing, forgotten.
pub fn record(arch: &mut Architecture, entry: &Address, data: &Funcdata) {
    if !arch.call_ret_type {
        return;
    }
    let Some(k) = key(entry) else { return };
    let stated = if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        None
    } else {
        recovered_return(data)
    };
    match stated {
        Some(s) => {
            arch.kuna_callret_types.insert(k, Rc::new(s));
        }
        None => {
            arch.kuna_callret_types.remove(&k);
        }
    }
}

/// The function's own recovered return value, or `None` when it returns
/// nothing, has no recovered prototype, returns something other than a
/// scalar or pointer, or leaves some live path returning a value it never
/// computed ([`crate::kuna_returnuncomputed::every_return_computes`]).
fn recovered_return(data: &Funcdata) -> Option<StatedReturn> {
    let proto = data.get_func_proto();
    if !proto.has_store() || !proto.has_model() || proto.is_model_unknown() {
        return None;
    }
    let out = proto.get_output();
    let ct = out.get_type()?;
    if !states(ct) || ct.get_size() != out.get_size() {
        return None;
    }
    let addr = out.get_address();
    if addr.is_invalid() || out.get_size() <= 0 {
        return None;
    }
    if !crate::kuna_returnuncomputed::every_return_computes_with(data, true) {
        return None;
    }
    Some(StatedReturn { addr, size: out.get_size(), ct: Rc::clone(ct) })
}

/// A type a return statement may carry: a pointer, an integer wider than a
/// byte or a float, never deeper than the inferred pointer cap.  A byte is not
/// stated, as a `bool` or as a character: which of the two it is is the
/// caller's own call (`boolbyte`, `charbyte`), and a callee's `bool` over a
/// byte the caller compares with `'\x01'` prints `(bool)v != 1`.
fn states(ct: &Rc<Datatype>) -> bool {
    let scalar = match ct.get_metatype() {
        type_metatype::TYPE_PTR | type_metatype::TYPE_FLOAT => true,
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT => ct.get_size() > 1,
        _ => false,
    };
    scalar
        && crate::p5_types::kuna_ptrdepth::pointer_depth(ct, 3)
            <= crate::p5_types::kuna_ptrdepth::MAX_INFERRED_PTR_DEPTH
}

/// What the function filed under `k` states it returns, for [`restore`].
pub fn statement(arch: &Architecture, k: (int4, uintb)) -> Option<Rc<StatedReturn>> {
    arch.kuna_callret_types.get(&k).cloned()
}

/// Put back `stated`, what the function filed under `k` stated before a
/// decompile the run then discarded (a `calleevote` or convergence redo that
/// moved the arity or failed), so the statement keeps describing the body the
/// run prints.
pub fn restore(arch: &mut Architecture, k: (int4, uintb), stated: Option<Rc<StatedReturn>>) {
    match stated {
        Some(s) => {
            arch.kuna_callret_types.insert(k, s);
        }
        None => {
            arch.kuna_callret_types.remove(&k);
        }
    }
}

/// Forget every statement whose type names one of `names`, through any depth
/// of pointer: the `structsynth` convergence sweep's superseded structures.
pub fn forget_statements_naming(arch: &mut Architecture, names: &[String]) {
    if names.is_empty() {
        return;
    }
    arch.kuna_callret_types.retain(|_, s| !crate::kuna_protoorder::names_type(&s.ct, names));
}

/// Copy onto `data` what every callee it calls stated it returns.  A function
/// never reads its own statement: at a call to itself that is what an earlier
/// decompile of this same function said.
pub fn seed(arch: &Architecture, data: &mut Funcdata) {
    if !arch.call_ret_type || arch.kuna_callret_types.is_empty() {
        return;
    }
    let own = key(data.get_address());
    let entries: Vec<Address> = (0..data.num_calls())
        .map(|i| data.get_call_specs(i).get_entry_address().clone())
        .filter(|e| !e.is_invalid())
        .collect();
    for e in entries {
        let Some(k) = key(&e) else { continue };
        if Some(k) == own || own.is_some_and(|o| arch.kuna_callret_refused.contains(&(o, k))) {
            continue;
        }
        if let Some(s) = arch.kuna_callret_types.get(&k) {
            data.kuna_set_callret_type(k, Rc::clone(s));
        }
    }
    data.kuna_set_callret_data(Rc::clone(&arch.globalref_ranges));
}

/// The callees whose stated return type the finished `data` contradicts.
///
/// The vote is taken while types are inferred, on the IR of that moment, and
/// the merge ties some storage whole-function after it (the return register of
/// a function that joins its returned values there), so a result the vote saw
/// alone can share one variable with the function's own return: another call's
/// result of the other class, a number that is not an address beside a
/// pointer (`return -1`, an error code), a value loaded through the pointer
/// itself, or a sum of integers.  A statement the caller returns, directly or
/// offset, beside such a number would make the caller's own return type a
/// pointer.  Each of these is withdrawn for this caller, when the result's
/// variable (or the function's own return) carries the stated type, and the
/// caller is decompiled again without it (`kuna-console`'s decompile step,
/// [`refuse`]).  A statement the caller's fold overruled typed nothing and is
/// left alone, and a function over [`AUDIT_MAX_OPS`] is not audited.
pub fn contradicted(data: &mut Funcdata) -> Vec<(int4, uintb)> {
    let mut out: Vec<(int4, uintb)> = Vec::new();
    if !data.kuna_has_callret_types() || data.obank().iter_alive().count() > AUDIT_MAX_OPS {
        return out;
    }
    let (returned_calls, returns_number) = returned_sources(data);
    let returns_pointer = data
        .get_func_proto()
        .get_output_type()
        .is_some_and(|t| t.get_metatype() == type_metatype::TYPE_PTR);
    let mut kept: Vec<((int4, uintb), OpId, VarnodeId)> = Vec::new();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let op = fc.get_op();
        if fc.proto().is_output_locked() {
            continue;
        }
        let Some(k) = key(fc.get_entry_address()) else { continue };
        let Some(stated) = data.kuna_callret_type(k) else { continue };
        if returns_number
            && returns_pointer
            && stated.ct.get_metatype() == type_metatype::TYPE_PTR
            && returned_calls.contains(&op)
        {
            if !out.contains(&k) {
                out.push(k);
            }
            continue;
        }
        let Some(rv) = result_of(data, op) else { continue };
        let Some(result) = data.vbank().get(rv) else { continue };
        if result.get_addr() == &stated.addr && result.get_size() == stated.size {
            kept.push((k, op, rv));
        }
    }
    for (k, op, rv) in kept {
        if out.contains(&k) {
            continue;
        }
        let Some(ct) = data.kuna_callret_type(k).map(|s| Rc::clone(&s.ct)) else { continue };
        if !data.high_get_type(rv).is_some_and(|t| crate::kuna_calleevote::same_type(&t, &ct)) {
            continue;
        }
        let Some(high) = data.vbank().get(rv).and_then(|n| n.get_high()) else { continue };
        if high_contradicts(data, op, high, &ct) {
            out.push(k);
        }
    }
    out
}

/// The calls whose result the function hands back, directly or offset, and
/// whether it also hands back a number that is not an address (`return -1;`,
/// an error code).  A pointer statement for such a result makes the function's
/// own return type a pointer and prints every one of those numbers as one.
fn returned_sources(data: &Funcdata) -> (Vec<OpId>, bool) {
    let mut calls: Vec<OpId> = Vec::new();
    let mut numbers = false;
    let mut work: Vec<(VarnodeId, bool)> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter_map(|r| data.obank().get(r).filter(|o| !o.is_dead()).and_then(|o| o.get_in(1)))
        .map(|v| (v, true))
        .collect();
    let mut seen: Vec<VarnodeId> = Vec::new();
    while let Some((v, whole)) = work.pop() {
        if seen.contains(&v) || seen.len() > 256 {
            continue;
        }
        seen.push(v);
        if is_number(data, Some(v)) {
            numbers |= whole;
            continue;
        }
        let Some(def) = data.vbank().get(v).and_then(|n| n.get_def()) else { continue };
        let Some(o) = data.obank().get(def) else { continue };
        match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => calls.push(def),
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
                work.extend((0..o.num_input()).filter_map(|i| o.get_in(i)).map(|i| (i, whole)));
            }
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                work.extend((0..o.num_input()).filter_map(|i| o.get_in(i)).map(|i| (i, false)));
            }
            OpCode::CPUI_INDIRECT => work.extend(o.get_in(0).map(|i| (i, whole))),
            OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => work.extend(o.get_in(0).map(|i| (i, false))),
            _ => {}
        }
    }
    (calls, numbers)
}

/// Withdraw the statements `callees` made from the caller entered at `entry`.
pub fn refuse(arch: &mut Architecture, entry: &Address, callees: &[(int4, uintb)]) {
    let Some(own) = key(entry) else { return };
    for &k in callees {
        arch.kuna_callret_refused.insert((own, k));
    }
}

/// Is the address `a` a member of `high`, or a member plus an offset?
fn address_in(data: &Funcdata, a: VarnodeId, high: crate::context::HighVariableId) -> bool {
    let mut at = a;
    for _ in 0..4 {
        let Some(n) = data.vbank().get(at) else { return false };
        if n.get_high() == Some(high) {
            return true;
        }
        let Some(o) = n.get_def().and_then(|d| data.obank().get(d)) else { return false };
        if !matches!(
            o.code(),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_COPY | OpCode::CPUI_CAST
        ) {
            return false;
        }
        if o.code() == OpCode::CPUI_INT_ADD
            && o.get_in(1).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.get_high() == Some(high))
        {
            return true;
        }
        let Some(base) = o.get_in(0) else { return false };
        at = base;
    }
    false
}

/// Where the result of the call `op` lives: its output, or the output of the
/// conversion `ActionSetCasts` put after it.
fn result_of(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    let out = data.obank().get(op)?.get_out()?;
    let node = data.vbank().get(out)?;
    let mut reads = node.descend_iter();
    let via_cast = match (reads.next(), reads.next()) {
        (Some(r), None) if node.is_implied() => data
            .obank()
            .get(r)
            .filter(|o| o.code() == OpCode::CPUI_CAST)
            .and_then(|o| o.get_out()),
        _ => None,
    };
    Some(via_cast.unwrap_or(out))
}

/// Does a member of `high`, other than the result of `op`, hold a value the
/// statement `ct` cannot describe: another call's result of the other class,
/// and beside a pointer a number that is not an address, a value loaded
/// through the pointer itself, or a sum or product of integers?
fn high_contradicts(data: &Funcdata, op: OpId, high: crate::context::HighVariableId, ct: &Datatype) -> bool {
    let Some(h) = data.high_bank().get(high) else { return false };
    let pointer = ct.get_metatype() == type_metatype::TYPE_PTR;
    let primitive_pointee = ct.get_ptr_to().is_some_and(|p| {
        !matches!(p.get_metatype(), type_metatype::TYPE_PTR | type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION)
    });
    for i in 0..h.num_instances() {
        let Some(mut def) = data.vbank().get(h.get_instance(i)).and_then(|n| n.get_def()) else { continue };
        let Some(mut o) = data.obank().get(def) else { continue };
        if o.code() == OpCode::CPUI_CAST {
            if pointer && is_number(data, o.get_in(0)) {
                return true;
            }
            let Some(inner) = o.get_in(0).and_then(|v| data.vbank().get(v)).and_then(|n| n.get_def()) else { continue };
            let Some(io) = data.obank().get(inner) else { continue };
            (def, o) = (inner, io);
        }
        if def == op {
            continue;
        }
        let contradicts = match o.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => result_type(data, def).is_some_and(|t| other_class(&t, ct)),
            OpCode::CPUI_COPY => pointer && is_number(data, o.get_in(0)),
            OpCode::CPUI_LOAD => pointer && primitive_pointee && o.get_in(1).is_some_and(|a| address_in(data, a, high)),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_MULT => {
                pointer
                    && (0..o.num_input()).all(|s| {
                        o.get_in(s).and_then(|v| data.vbank().get(v)).is_some_and(|n| {
                            n.get_high() != Some(high) && n.get_type().get_metatype() != type_metatype::TYPE_PTR
                        })
                    })
            }
            _ => false,
        };
        if contradicts {
            return true;
        }
    }
    false
}

/// The type the call `op` returns, as its callee stated it, when the call's
/// output sits in exactly the storage and width the callee returns in.
pub fn stated_return_type(data: &Funcdata, op: OpId, fc: &FuncCallSpecs) -> Option<Rc<Datatype>> {
    let o = data.obank().get(op)?;
    if !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) || fc.proto().is_output_locked() {
        return None;
    }
    let stated = data.kuna_callret_type(key(fc.get_entry_address())?)?;
    let outvn = o.get_out()?;
    let out = data.vbank().get(outvn)?;
    if out.get_size() != stated.size || out.get_addr() != &stated.addr || stated.ct.get_size() != stated.size {
        return None;
    }
    if crate::kuna_protoorder::output_refuses(data, outvn, &stated.ct)
        || declared_contradicts(data, op, outvn, &stated.ct)
        || accesses_disagree(data, outvn, &stated.ct)
    {
        return None;
    }
    Some(Rc::clone(&stated.ct))
}

/// Does something the caller declares about the result outrank the statement
/// `ct`?  Another call writing the same value whose declared or stated result
/// is of the other class (an integer beside a pointer) always does.  For an
/// integer: a reader whose meaning depends on the sign (an
/// ordered comparison, a shift right, a division, an extension, a declared
/// parameter), or another call writing the same value with a declared or
/// stated result, at the other sign -- the vote would re-sign the caller's
/// variable and print a conversion at every one of them -- and, for a value
/// the function hands back, a widening of what it returns at the other sign
/// ([`returned_at_other_sign`]).  For a pointer: a
/// declared parameter the value is passed to, or another call writing the same
/// value with a declared or stated result, typed as a pointer to something
/// else -- a declaration outranks a recovery (`getgrnam`'s `group *` over a
/// wrapper's synthesized `struct_8 *`), and two recoveries that disagree leave
/// the variable to the caller's own fold (bash `sub_7dbb0` keeps one variable
/// for `array_value`'s `struct_1 *`, `dequote_string`'s `char *` and its own
/// parameter, and a vote for one of them left a `PTRSUB` the printer cannot
/// render as C).  A `void *` or a pointer to unknown bytes, on
/// either side, says nothing about the pointee.
fn declared_contradicts(data: &Funcdata, op: OpId, outvn: VarnodeId, ct: &Datatype) -> bool {
    let want = ct.get_metatype();
    let integer = matches!(want, type_metatype::TYPE_INT | type_metatype::TYPE_UINT);
    if !integer && want != type_metatype::TYPE_PTR {
        return false;
    }
    let other = |t: &Datatype| {
        if integer {
            t.get_size() == ct.get_size()
                && matches!(t.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
                && t.get_metatype() != want
        } else {
            let known = |t: &Datatype| {
                t.get_ptr_to().is_some_and(|p| {
                    !matches!(p.get_metatype(), type_metatype::TYPE_VOID | type_metatype::TYPE_UNKNOWN)
                })
            };
            t.get_metatype() == type_metatype::TYPE_PTR
                && known(t)
                && known(ct)
                && !crate::kuna_calleevote::same_type(t, ct)
        }
    };
    let extended = integer && returned_at_other_sign(data, ct);
    for v in crate::kuna_protoorder::value_family(data, outvn) {
        let Some(node) = data.vbank().get(v) else { continue };
        if extended && is_returned(data, v) {
            return true;
        }
        if let Some(def) = node.get_def().filter(|&d| d != op) {
            if data.obank().get(def).is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
                && result_type(data, def).is_some_and(|t| other(&t) || other_class(&t, ct))
            {
                return true;
            }
        }
        for r in node.descend_iter() {
            let Some(o) = data.obank().get(r) else { continue };
            let sensitive = integer
                && matches!(
                    o.code(),
                    OpCode::CPUI_INT_LESS
                        | OpCode::CPUI_INT_LESSEQUAL
                        | OpCode::CPUI_INT_SLESS
                        | OpCode::CPUI_INT_SLESSEQUAL
                        | OpCode::CPUI_INT_RIGHT
                        | OpCode::CPUI_INT_SRIGHT
                        | OpCode::CPUI_INT_DIV
                        | OpCode::CPUI_INT_SDIV
                        | OpCode::CPUI_INT_REM
                        | OpCode::CPUI_INT_SREM
                        | OpCode::CPUI_INT_ZEXT
                        | OpCode::CPUI_INT_SEXT
                );
            for s in 0..o.num_input() {
                if o.get_in(s) != Some(v) {
                    continue;
                }
                let t = if sensitive {
                    Some(crate::coreaction_infertypes::input_type_local(data, r, s))
                } else if s > 0 && matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                    declared_param_type(data, r, s)
                } else {
                    None
                };
                if t.is_some_and(|t| other(&t)) {
                    return true;
                }
            }
        }
    }
    false
}

/// Was what the function returns widened at the other sign than the statement
/// `ct`, from `ct`'s width, before the return trimming narrowed the returned
/// value back ([`note_returned_extension`])?  On x86-64 every 32-bit write
/// zero-extends into the whole register, so `call neg32; mov %eax,%ebx; call
/// other; mov %ebx,%eax; ret` hands its callers `neg32`'s result zero-extended,
/// as a reload or `mov %eax,%eax` does, and `movzwl %ax,%eax` does for a
/// 16-bit result.  An `int` statement for the call would become the
/// function's own return type once the extension is gone: `int f(...)`, whose
/// callers sign-extend what the binary's callers read zero-extended.
fn returned_at_other_sign(data: &Funcdata, ct: &Datatype) -> bool {
    let signed = ct.get_metatype() == type_metatype::TYPE_INT;
    data.kuna_callret_returned().iter().any(|&(s, from)| from == ct.get_size() && s != signed)
}

/// Is `v` handed back by a RETURN?
fn is_returned(data: &Funcdata, v: VarnodeId) -> bool {
    data.vbank().get(v).is_some_and(|n| {
        n.descend_iter().any(|r| {
            data.obank().get(r).is_some_and(|o| {
                o.code() == OpCode::CPUI_RETURN && (1..o.num_input()).any(|i| o.get_in(i) == Some(v))
            })
        })
    })
}

/// (Called by `RuleSubvarZext` and `RuleSubvarSext`.)  An extension from
/// `from` bytes, at the sign when `signed`, was just narrowed back and, with
/// it, the function's returned value: the value the function returns reaches
/// its callers converted at that sign.
pub fn note_returned_extension(data: &mut Funcdata, signed: bool, from: int4) {
    if data.kuna_has_integer_callret_types() {
        data.kuna_note_callret_returned((signed, from));
    }
}

/// Does the caller use a stated pointer as something other than a pointer to
/// the stated pointee?  A value it loads through the pointer must not flow
/// back into the pointer's own variable (`p = p[2]` makes one variable both
/// the pointer and what it points to).  A primitive pointee of N bytes must be
/// read and written N bytes at a time, at offsets and steps that are multiples
/// of N (unknown bytes too: `undefined8 *` prints as `unsigned long *`); a
/// `void *` must not be offset at all.  Anything else prints as a cast at every
/// such use where the caller's own type prints none.  A composite pointee is
/// left to `protoorder`'s own layout check.
fn accesses_disagree(data: &Funcdata, outvn: VarnodeId, ct: &Datatype) -> bool {
    if ct.get_metatype() != type_metatype::TYPE_PTR {
        return false;
    }
    let Some(pointee) = ct.get_ptr_to() else { return false };
    let size = match pointee.get_metatype() {
        type_metatype::TYPE_VOID => 0,
        type_metatype::TYPE_INT
        | type_metatype::TYPE_UINT
        | type_metatype::TYPE_BOOL
        | type_metatype::TYPE_FLOAT
        | type_metatype::TYPE_UNKNOWN
        | type_metatype::TYPE_PTR => pointee.get_size() as i64,
        _ => return false,
    };
    let family = with_returned_values(data, crate::kuna_protoorder::value_family(data, outvn));
    let Some((accesses, places)) = crate::kuna_protoorder::accesses_through(data, &family) else { return true };
    if accesses.iter().any(|a| family.contains(&a.value)) {
        return true;
    }
    if size == 0 {
        return places.iter().any(|&(c, stride)| c != 0 || stride != 0);
    }
    places.iter().any(|&(c, stride)| c % size != 0 || stride % size != 0)
        || accesses.iter().any(|a| a.size as i64 != size)
}

/// `family`, and when the function returns a member of it, every other value
/// the function returns: an undeclared return type follows the value, so a
/// type the call's result takes becomes the function's own return type and
/// reaches everything else it returns.
fn with_returned_values(data: &Funcdata, mut family: Vec<VarnodeId>) -> Vec<VarnodeId> {
    let returned = family.iter().any(|&v| {
        data.vbank().get(v).is_some_and(|n| {
            n.descend_iter().any(|r| {
                data.obank().get(r).is_some_and(|o| o.code() == OpCode::CPUI_RETURN && o.get_in(1) == Some(v))
            })
        })
    });
    if !returned || data.get_func_proto().is_output_locked() {
        return family;
    }
    let rets: Vec<VarnodeId> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter_map(|r| data.obank().get(r).filter(|o| !o.is_dead()).and_then(|o| o.get_in(1)))
        .collect();
    for r in rets {
        for v in crate::kuna_protoorder::value_family(data, r) {
            if !family.contains(&v) {
                family.push(v);
            }
        }
    }
    family
}

/// The type the call `call` is declared (output-locked) or stated to return.
fn result_type(data: &Funcdata, call: OpId) -> Option<Rc<Datatype>> {
    let fc = data.get_call_specs(data.get_call_specs_index(call)?);
    if fc.proto().is_output_locked() {
        return fc.proto().get_output_type().cloned();
    }
    Some(Rc::clone(&data.kuna_callret_type(key(fc.get_entry_address())?)?.ct))
}

/// The declared type of the call `call`'s parameter `slot`, when it is locked.
fn declared_param_type(data: &Funcdata, call: OpId, slot: int4) -> Option<Rc<Datatype>> {
    let fc = data.get_call_specs(data.get_call_specs_index(call)?);
    let param = fc.proto().get_param(slot - 1)?;
    if !param.is_type_locked() {
        return None;
    }
    param.get_type().cloned()
}
