//! (kuna `swiftcall`) The Swift convention of functions the metadata did not
//! bind, read off what their code does with the Swift registers.
//!
//! The `swiftcall` arm binds `__swiftcall` to the functions Swift metadata
//! places in a type. A closure, a partial-apply forwarder, a thunk, a free
//! function or a stripped method keeps the C model, under which `swiftself`
//! (x20 / r13) and `swifterror` (x21 / r12) are preserved registers the
//! function never takes, so a closure's context reads as an uninitialised
//! local and the error a call returns as a value nothing set. Every other
//! convention preserves those registers, so their code says which convention
//! a function follows:
//!
//! * A function that reads `swiftself` on entry, for anything but saving it,
//!   takes it ([`seed`]): no C function reads a preserved register it has not
//!   written. It decompiles under `__swiftcall` with the input named
//!   `swiftself`, and a direct call to it passes the register
//!   ([`callee_model`]).
//! * A function that reads `swiftasync` (x22 / r14) on entry, for anything
//!   but saving it, is `async` ([`seed`]): the register holds its context,
//!   which a resume partial that only loads its frame and returns through
//!   the context reads without calling the task runtime. It decompiles under
//!   `__swiftasynccall` with the input named `async_context`, and a direct
//!   call to it (a tail call: an `async` function hands the context on)
//!   passes the register ([`async_callee_model`]). So does an indirect
//!   tail call it makes ([`async_tail_call`]): a call through a witness or
//!   a closure's function pointer, and the return, which is a jump to the
//!   resume function the context holds.
//! * A call after which the caller tests `swifterror` while the register still
//!   holds the caller's own entry value returns an error in it
//!   ([`error_passing_call`]): the caller hands on the zero its own caller
//!   stored, and under the C model the test would read a value that was never
//!   set. The call takes `__swiftcall_throws`, like one after a zeroing store.
//! * A two-word result joined from the first two return registers whose
//!   halves are read apart is the pair Swift returns small aggregates in
//!   (a `String`, an `Array` and its count, a closure and its context), typed
//!   as the structure `swift.pair { _0, _1 }` instead of a 128-bit integer
//!   ([`pair_output_type`]).
//!
//! Inert unless the image binds the Swift convention (the metadata pass found
//! Swift): part of `option swiftcall`.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::{Datatype, TypeField, type_metatype};
use crate::fspec::ProtoModel;
use crate::funcdata::Funcdata;

/// The input name of the `swiftself` register of a function bound here.
pub const SELF_NAME: &str = "swiftself";

/// The structure a Swift two-word result is typed as.
pub const PAIR_NAME: &str = "swift.pair";

/// How many blocks [`entry_value_at`] walks back from a call.
const MAX_BLOCKS: usize = 64;

/// How many ops after a call [`error_passing_call`] looks for the test.
const MAX_SCAN: usize = 32;

fn key(addr: &Address) -> Option<(int4, uintb)> {
    Some((addr.get_space()?.get_index(), addr.get_offset()))
}

fn overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    a.overlap(0, b, bsize) >= 0 || b.overlap(0, a, asize) >= 0
}

fn enabled(arch: &crate::architecture::Architecture) -> bool {
    arch.loader_swift_abi.is_some()
}

/// How many instructions [`probe_reads`] decodes from one entry.
const MAX_PROBE: usize = 256;

/// Recording sink for one instruction's raw p-code.
#[derive(Default)]
struct OneEmit {
    ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
}

impl kuna_sleigh::translate::PcodeEmit for OneEmit {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

fn same(a: &VarnodeData, b: &VarnodeData) -> bool {
    a.offset == b.offset
        && a.size == b.size
        && matches!((a.space.as_ref(), b.space.as_ref()), (Some(x), Some(y)) if x.get_index() == y.get_index())
}

/// Does the body at `entry` read the register `reg` before writing it, other
/// than to store it (directly or through a temporary its instruction copies it
/// to, as an AArch64 `stp` does)? Every path from the entry is walked up to its
/// first call, return, indirect branch or write of the register; a body the
/// walk cannot cover within [`MAX_PROBE`] instructions answers no.
fn probe_reads<T: kuna_sleigh::translate::Translate + ?Sized>(tr: &T, entry: &Address, reg: &VarnodeData) -> bool {
    let hits = |v: &VarnodeData| {
        matches!((v.space.as_ref(), reg.space.as_ref()), (Some(x), Some(y)) if x.get_index() == y.get_index())
            && v.offset < reg.offset + reg.size as u64
            && reg.offset < v.offset + v.size as u64
    };
    let mut todo = vec![entry.clone()];
    let mut seen: BTreeSet<(int4, uintb)> = BTreeSet::new();
    let mut read = false;
    while let Some(at) = todo.pop() {
        let Some(k) = key(&at) else { return false };
        if !seen.insert(k) {
            continue;
        }
        if seen.len() > MAX_PROBE {
            return false;
        }
        let mut emit = OneEmit::default();
        let len = match tr.one_instruction(&mut emit, &at) {
            Ok(n) if n > 0 => n,
            _ => return false,
        };
        let mut copies: Vec<VarnodeData> = Vec::new();
        let mut written = false;
        let mut next = vec![&at + len as i64];
        for (opc, out, ins) in &emit.ops {
            for (i, v) in ins.iter().enumerate() {
                let address = i == 0
                    && matches!(
                        opc,
                        OpCode::CPUI_BRANCH
                            | OpCode::CPUI_CBRANCH
                            | OpCode::CPUI_BRANCHIND
                            | OpCode::CPUI_CALL
                            | OpCode::CPUI_CALLIND
                            | OpCode::CPUI_RETURN
                            | OpCode::CPUI_CALLOTHER
                            | OpCode::CPUI_LOAD
                            | OpCode::CPUI_STORE
                    );
                if address || !(hits(v) || copies.iter().any(|c| same(c, v))) {
                    continue;
                }
                let stored = *opc == OpCode::CPUI_STORE && i == 2;
                let staged = *opc == OpCode::CPUI_COPY
                    && hits(v)
                    && out
                        .as_ref()
                        .is_some_and(|o| o.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL));
                if staged {
                    copies.push(out.clone().unwrap());
                } else if !stored && !written {
                    read = true;
                }
            }
            if let Some(o) = out {
                if hits(o) {
                    written = true;
                }
                if !(*opc == OpCode::CPUI_COPY && ins.first().is_some_and(|v| hits(v))) {
                    copies.retain(|c| !same(c, o));
                }
            }
            match opc {
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    let Some(t) = ins.first() else { return false };
                    let Some(sp) = t.space.as_ref() else { return false };
                    if sp.get_type() == spacetype::IPTR_CONSTANT {
                        continue;
                    }
                    let target = Address::new(Rc::clone(sp), t.offset);
                    if *opc == OpCode::CPUI_BRANCH {
                        next.clear();
                    }
                    next.push(target);
                }
                OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER => next.clear(),
                _ => {}
            }
        }
        if read {
            return true;
        }
        if !written {
            todo.extend(next);
        }
    }
    false
}

/// The Swift register a body is probed for.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Probe {
    SelfReg,
    AsyncReg,
}

/// Does the body at `entry` read the probed register before writing it,
/// other than to save it? Cached per entry for the run.
fn reads_entry(arch: &mut crate::architecture::Architecture, entry: &Address, probe: Probe) -> bool {
    let Some(abi) = arch.loader_swift_abi.clone() else { return false };
    let Some(k) = key(entry) else { return false };
    if entry.is_invalid() {
        return false;
    }
    let cached = match probe {
        Probe::SelfReg => arch.kuna_swift_self_reads.get(&k),
        Probe::AsyncReg => arch.kuna_swift_async_reads.get(&k),
    };
    if let Some(&r) = cached {
        return r;
    }
    let (addr, size) = match probe {
        Probe::SelfReg => &abi.self_reg,
        Probe::AsyncReg => &abi.async_reg,
    };
    let reg = VarnodeData { space: addr.get_space().cloned(), offset: addr.get_offset(), size: *size as u32 };
    let r = probe_reads(arch.translate(), entry, &reg);
    match probe {
        Probe::SelfReg => arch.kuna_swift_self_reads.insert(k, r),
        Probe::AsyncReg => arch.kuna_swift_async_reads.insert(k, r),
    };
    r
}

fn is_async_model(m: &ProtoModel) -> bool {
    m.get_name().starts_with(crate::kuna_swiftcall::SWIFTASYNCCALL)
}

/// Whether the function itself reads `swiftself` (when the analysis did not
/// bind it) and `swiftasync` (when nothing bound it `async`) on entry; and,
/// for its direct callees, which of them read either.
pub fn seed(arch: &mut crate::architecture::Architecture, data: &mut Funcdata) -> (bool, bool) {
    if !enabled(arch) {
        return (false, false);
    }
    let mut callees = BTreeSet::new();
    let mut async_callees = BTreeSet::new();
    for i in 0..data.num_calls() {
        let e = data.get_call_specs(i).get_entry_address().clone();
        let Some(k) = key(&e) else { continue };
        if e.is_invalid() || callees.contains(&k) || async_callees.contains(&k) {
            continue;
        }
        let bound = arch.loader_proto_models.get(&k).cloned();
        if !bound.as_ref().is_some_and(|m| !m.get_name().starts_with("__swift") || is_async_model(m))
            && reads_entry(arch, &e, Probe::AsyncReg)
        {
            async_callees.insert(k);
        }
        if bound.is_none() && reads_entry(arch, &e, Probe::SelfReg) {
            callees.insert(k);
        }
    }
    data.kuna_set_swift_self_callees(callees);
    data.kuna_set_swift_async_callees(async_callees);
    let own = data.get_address().clone();
    let Some(k) = key(&own) else { return (false, false) };
    let bound = arch.loader_proto_models.get(&k).cloned();
    let reads_async = !bound.as_ref().is_some_and(|m| !m.get_name().starts_with("__swift") || is_async_model(m))
        && reads_entry(arch, &own, Probe::AsyncReg);
    (bound.is_none() && reads_entry(arch, &own, Probe::SelfReg), reads_async)
}

/// The input hint that names the `swiftself` input of a function [`seed`] found.
pub fn self_hint(arch: &crate::architecture::Architecture) -> Option<crate::kuna_inputhints::InputHint> {
    let abi = arch.loader_swift_abi.as_ref()?;
    Some(crate::kuna_inputhints::InputHint {
        addr: abi.self_reg.0.clone(),
        size: abi.self_reg.1,
        name: SELF_NAME.to_string(),
        ct: None,
    })
}

/// `__swiftcall` for the direct call `idx` when its callee reads `swiftself`.
pub fn callee_model(data: &Funcdata, idx: int4) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let k = key(data.get_call_specs(idx).get_entry_address())?;
    data.kuna_swift_self_callees().contains(&k).then(|| Rc::clone(&abi.plain))
}

/// `__swiftasynccall` for an indirect jump out of an `async` function that
/// the flow made a call: the function it awaits, or its return to the resume
/// function of its context. Either takes the context register.
pub fn async_tail_call(data: &Funcdata, call: OpId) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let own = data.get_func_proto();
    if !own.has_model() || !is_async_model(own.model()) {
        return None;
    }
    let indirect = data.obank().get(call).is_some_and(|op| op.code() == OpCode::CPUI_CALLIND);
    (indirect && is_tail_call(data, call)).then(|| abi.model(true, false))
}

/// Is `call` a jump the flow made a call, followed by the return it planted
/// at the same address?
fn is_tail_call(data: &Funcdata, call: OpId) -> bool {
    let Some(op) = data.obank().get(call) else { return false };
    let next = op.basic_neighbours().1.and_then(|n| data.obank().get(n));
    next.is_some_and(|n| n.code() == OpCode::CPUI_RETURN && n.get_addr() == op.get_addr())
}

/// Is the trial in `addr`/`size` the `swiftasync` register of a call under
/// an `async` model? An `async` callee always takes its context, which the
/// caller's own, handed on untouched, would not show as an argument.
pub fn passes_context(data: &Funcdata, idx: int4, addr: &Address, size: int4) -> bool {
    let Some(abi) = data.get_arch().swift_abi.as_ref() else { return false };
    let proto = data.get_call_specs(idx).proto();
    addr == &abi.async_reg.0 && size == abi.async_reg.1 && proto.has_model() && is_async_model(proto.model())
}

/// Is input `slot` of call `idx` the value an `async` function's indirect
/// tail call jumps through? The scratch register holding the resume function
/// or the awaited function's address is not also its argument.
pub fn carries_async_target(data: &Funcdata, idx: int4, slot: int4) -> bool {
    let call = data.get_call_specs(idx);
    if data.get_arch().swift_abi.is_none() || !call.proto().has_model() || !is_async_model(call.proto().model()) {
        return false;
    }
    let Some(op) = data.obank().get(call.get_op()).filter(|op| op.code() == OpCode::CPUI_CALLIND) else {
        return false;
    };
    let root = |vn: VarnodeId| {
        let def = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
        def.filter(|d| d.code() == OpCode::CPUI_COPY).and_then(|d| d.get_in(0)).unwrap_or(vn)
    };
    match (op.get_in(0), op.get_in(slot)) {
        (Some(target), Some(arg)) => slot != 0 && root(target) == root(arg),
        _ => false,
    }
}

/// `__swiftasynccall` for the direct call `idx` when its callee is an import
/// whose name says `async`, or reads `swiftasync` (throwing when the model
/// bound to it does).
pub fn async_callee_model(data: &Funcdata, idx: int4, bound: Option<&Rc<ProtoModel>>) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let fc = data.get_call_specs(idx);
    if let Some(&throws) = abi.async_names.get(fc.get_name()) {
        return Some(abi.model(true, throws));
    }
    let k = key(fc.get_entry_address())?;
    let throws = bound.is_some_and(|m| m.get_name().ends_with("_throws"));
    data.kuna_swift_async_callees().contains(&k).then(|| abi.model(true, throws))
}

/// Does `op` read `(reg, size)` other than as the value it stores?
fn reads(data: &Funcdata, op: OpId, reg: &Address, size: int4) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    (0..o.num_input()).any(|i| {
        o.get_in(i).and_then(|v| data.vbank().get(v)).is_some_and(|v| overlaps(v.get_addr(), v.get_size(), reg, size))
    })
}

fn writes(data: &Funcdata, op: OpId, reg: &Address, size: int4) -> bool {
    data.obank()
        .get(op)
        .and_then(|o| o.get_out())
        .and_then(|v| data.vbank().get(v))
        .is_some_and(|v| overlaps(v.get_addr(), v.get_size(), reg, size))
}

/// Does `(reg, size)` still hold the function's entry value at `call`: no op
/// on any path from the entry to it writes the register? Answers no when the
/// walk is cut short.
fn entry_value_at(data: &Funcdata, call: OpId, reg: &Address, size: int4) -> bool {
    let Some(start) = data.obank().get(call).and_then(|o| o.get_parent()) else { return false };
    let mut cur = data.op_previous_op(call);
    while let Some(id) = cur {
        if writes(data, id, reg, size) {
            return false;
        }
        cur = data.op_previous_op(id);
    }
    let mut seen: BTreeSet<BlockId> = BTreeSet::new();
    let mut todo = vec![start];
    seen.insert(start);
    while let Some(bl) = todo.pop() {
        if seen.len() > MAX_BLOCKS {
            return false;
        }
        let block = data.bblocks_ref().block(bl);
        for i in 0..block.size_in() {
            let pred = block.get_in(i);
            if !seen.insert(pred) {
                continue;
            }
            if data.bb_ops(pred).into_iter().any(|id| writes(data, id, reg, size)) {
                return false;
            }
            todo.push(pred);
        }
    }
    true
}

/// `__swiftcall_throws` for a call after which the caller reads `swifterror`
/// before writing it, while the register still holds the caller's entry value.
pub fn error_passing_call(data: &Funcdata, call: OpId) -> Option<Rc<ProtoModel>> {
    let abi = data.get_arch().swift_abi.as_ref()?;
    let (reg, size) = (&abi.error_reg.0, abi.error_reg.1);
    let mut cur = data.op_next_op(call);
    let mut tested = false;
    for _ in 0..MAX_SCAN {
        let Some(id) = cur else { break };
        let op = data.obank().get(id)?;
        if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN)
        {
            break;
        }
        if reads(data, id, reg, size) {
            tested = op.code() != OpCode::CPUI_STORE;
            break;
        }
        if writes(data, id, reg, size) {
            break;
        }
        if data.obank().get(id).and_then(|o| o.get_parent()) != data.obank().get(call).and_then(|o| o.get_parent()) {
            break;
        }
        cur = data.op_next_op(id);
    }
    (tested && entry_value_at(data, call, reg, size)).then(|| Rc::clone(&abi.throws))
}

/// Is `vn` a two-word call result in a join of whole registers whose every
/// reader takes one word of it, and both words are read?
fn read_as_pair(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.get_size() != 16 || v.get_addr().get_space().map(|s| s.get_type()) != Some(spacetype::IPTR_JOIN) {
        return false;
    }
    let Some(def) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
    if !matches!(def.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
        return false;
    }
    let mut halves = [false; 2];
    for d in v.descend_iter() {
        let Some(o) = data.obank().get(d) else { return false };
        if o.code() != OpCode::CPUI_SUBPIECE {
            return false;
        }
        let off = o.get_in(1).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset());
        let out = o.get_out().and_then(|c| data.vbank().get(c)).map(|c| c.get_size());
        match (off, out) {
            (Some(0), Some(8)) => halves[0] = true,
            (Some(8), Some(8)) => halves[1] = true,
            _ => return false,
        }
    }
    halves == [true, true]
}

/// Can the call `op` be to a Swift function: not to one named as a C or
/// Objective-C function (`objc_msgSend`, `-[C m]`, a plain C identifier other
/// than a placeholder `sub_` name)?
fn may_be_swift(data: &Funcdata, op: OpId) -> bool {
    let Some(i) = data.get_call_specs_index(op) else { return true };
    let name = data.get_call_specs(i).get_name();
    if name.starts_with("objc_msgSend") || name.starts_with("-[") || name.starts_with("+[") {
        return false;
    }
    let c_name = !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    !c_name || name.starts_with("sub_") || name.starts_with("FUN_")
}

/// `swift.pair` for the result of the call `op` when [`read_as_pair`]
/// accepts it, in place of the 128-bit integer the join would print as.
pub fn pair_output_type(data: &Funcdata, op: OpId) -> Option<Rc<Datatype>> {
    let arch = data.get_arch();
    if arch.swift_abi.is_none() {
        return None;
    }
    let out = data.obank().get(op)?.get_out()?;
    if !read_as_pair(data, out) || !may_be_swift(data, op) {
        return None;
    }
    let types = arch.types()?;
    if let Ok(Some(t)) = types.find_by_name(PAIR_NAME) {
        return (t.get_metatype() == type_metatype::TYPE_STRUCT && t.get_size() == 16).then_some(t);
    }
    let word = types.get_base(8, type_metatype::TYPE_UNKNOWN).ok()?;
    let fields = vec![TypeField::new(0, 0, "_0", Rc::clone(&word)), TypeField::new(1, 8, "_1", word)];
    let shell = types.get_type_struct(PAIR_NAME).ok()?;
    types.set_fields_struct_raw(&shell, fields, Vec::new(), 16, 8, 0).ok()
}
