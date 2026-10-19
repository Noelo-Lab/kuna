//! (kuna `passthrough`) The incoming registers a function's own body reads.
//!
//! Right after the first heritage every register the body reads before writing
//! it is an input Varnode, and its readers say how it is used ([`capture`]). A
//! register is *read* when some op computes with its incoming value for a result
//! that reaches something -- followed through copies, merges and low pieces, and
//! as wide as the low bytes that result needs (`lea 0x19(%rdx),%ecx` feeding a
//! byte store reads one byte). It is not read as a call's argument, across a
//! call's side effect, as the value a store spills (a variadic's register-save
//! prologue, an alignment push), as the preserved high part of a partial write,
//! or by an op whose result does not depend on it (`xor %esi,%esi`, `sbb
//! %ecx,%ecx`, `and $0`). A register handed untouched to a parameter a declared
//! prototype names is *declared*; one handed untouched to a direct call is
//! *forwarded* to that callee.
//!
//! `protoorder` states, with each callee's types, what its body reads and the
//! part it reads only by handing it on ([`reads_through`]): to a declared
//! parameter, or to a callee that takes it by the same evidence
//! ([`callee_takes`]). That is the evidence `passthrough` and `calleereadarg`
//! read where the bounded entry decode cannot finish, and beside it where the
//! decode cannot see past a call.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// What the first heritage saw of the function's incoming registers.
#[derive(Debug, Clone, Default)]
pub struct EntryReads {
    /// Registers the body computes with.
    pub read: Vec<(Address, int4)>,
    /// Registers passed untouched as a parameter a callee's declared
    /// prototype names (an import's library signature).
    pub declared: Vec<(Address, int4)>,
    /// Registers passed untouched to a direct call, with the callee.
    pub forwarded: Vec<(Address, int4, Address)>,
}

impl EntryReads {
    /// Does the body take any byte of `[addr, addr + size)`, reading it or
    /// passing it to a declared parameter?
    pub fn takes(&self, addr: &Address, size: int4) -> bool {
        overlaps(&self.read, addr, size) || overlaps(&self.declared, addr, size)
    }

    /// How many bytes from `addr` the body takes.
    pub fn width(&self, addr: &Address) -> Option<int4> {
        self.read.iter().chain(&self.declared).find(|(a, _)| a == addr).map(|(_, w)| *w)
    }
}

/// How many Varnodes one [`low_demand`] walk visits before answering "all".
const DEMAND_BUDGET: u32 = 64;

/// Does a range of `ranges` overlap `[addr, addr + size)`?
pub fn overlaps(ranges: &[(Address, int4)], addr: &Address, size: int4) -> bool {
    let (space, off) = (addr.get_space().map(|s| s.get_index()), addr.get_offset());
    ranges.iter().any(|(a, s)| {
        a.get_space().map(|x| x.get_index()) == space && a.get_offset() < off + size as u64 && off < a.get_offset() + *s as u64
    })
}

fn is_register(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank()
        .get(vn)
        .and_then(|v| v.get_addr().get_space().map(|s| s.get_type()))
        .is_some_and(|t| t == spacetype::IPTR_PROCESSOR)
}

fn callee(data: &Funcdata, op: OpId) -> Option<Address> {
    let fc = data.get_call_specs(data.get_call_specs_index(op)?);
    let e = fc.get_entry_address();
    (!e.is_invalid()).then(|| e.clone())
}

/// Is input `slot` of the call `op` a named parameter of a prototype the
/// program declares (an import's library signature)? That callee reads it.
fn declared_param(data: &Funcdata, op: OpId, slot: int4) -> bool {
    let Some(i) = data.get_call_specs_index(op) else { return false };
    let fc = data.get_call_specs(i);
    fc.is_input_locked() && slot - 1 < fc.proto().num_params()
}

/// Take the snapshot (once per decompile, after the first heritage).
pub fn capture(data: &mut Funcdata) {
    if data.kuna_entry_reads().is_some() {
        return;
    }
    let mut out = EntryReads::default();
    let inputs: Vec<VarnodeId> = data.vbank().iter_def_flag(crate::varnode::varnode_flags::input).collect();
    for vn in inputs {
        if !is_register(data, vn) {
            continue;
        }
        let Some((addr, size)) = data.vbank().get(vn).map(|v| (v.get_addr().clone(), v.get_size())) else { continue };
        let mut width = 0;
        let mut cap = size;
        let mut passed = 0;
        let mut to: Vec<Address> = Vec::new();
        let mut family = vec![vn];
        let mut i = 0;
        while i < family.len() && family.len() < 256 && width < cap {
            let id = family[i];
            i += 1;
            let Some(cur) = data.vbank().get(id) else { continue };
            let here = cur.get_size();
            for op in cur.descend_iter() {
                let Some(o) = data.obank().get(op) else { continue };
                let mut follow = false;
                let real = match o.code() {
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                        let slot = (1..o.num_input()).find(|&k| o.get_in(k) == Some(id));
                        if o.get_in(0) == Some(id) {
                            true
                        } else {
                            if slot.is_some_and(|k| declared_param(data, op, k)) {
                                passed = passed.max(here);
                            } else if o.code() == OpCode::CPUI_CALL {
                                if let Some(c) = callee(data, op) {
                                    to.push(c);
                                }
                            }
                            false
                        }
                    }
                    OpCode::CPUI_INDIRECT | OpCode::CPUI_RETURN => false,
                    OpCode::CPUI_STORE => o.get_in(2) != Some(id),
                    OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => {
                        follow = true;
                        false
                    }
                    OpCode::CPUI_SUBPIECE if o.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.get_offset() == 0) => {
                        follow = true;
                        false
                    }
                    OpCode::CPUI_PIECE => match reassembled(data, o) {
                        Some((start, whole)) => {
                            if start == addr {
                                follow = true;
                                cap = cap.max(whole);
                            }
                            false
                        }
                        None => true,
                    },
                    _ => true,
                };
                if real {
                    let mut budget = DEMAND_BUDGET;
                    width = width.max(need(data, o, id, here, &mut Vec::new(), &mut budget));
                }
                if follow {
                    if let Some(out) = o.get_out() {
                        if !family.contains(&out) {
                            family.push(out);
                        }
                    }
                }
            }
        }
        if width > 0 {
            out.read.push((addr, width.min(cap)));
        } else if passed > 0 {
            out.declared.push((addr, passed.min(cap)));
        } else {
            for c in to {
                out.forwarded.push((addr.clone(), cap, c));
            }
        }
    }
    data.kuna_set_entry_reads(out);
}

/// The incoming register a `PIECE` puts back together from the pieces heritage
/// split it into around a narrower read (`mov %rdi,%rax` beside `add
/// %edi,%eax`), as its storage.
fn reassembled(data: &Funcdata, o: &crate::op::PcodeOp) -> Option<(Address, int4)> {
    crate::kuna_armfloatargs::incoming_storage(data, o.get_out()?)
}

/// Is `o`'s result the same whatever the value of its register operands: one
/// value combined with itself (`xor %esi,%esi`, `sbb %ecx,%ecx`'s compares), or
/// masked away (`and $0`, `or $-1`)?
fn erases(data: &Funcdata, o: &crate::op::PcodeOp) -> bool {
    let constant = |k: int4, all_ones: bool| {
        o.get_in(k).and_then(|c| data.vbank().get(c)).is_some_and(|c| {
            c.is_constant() && if all_ones { c.get_offset() == kuna_base::address::calc_mask(c.get_size()) } else { c.get_offset() == 0 }
        })
    };
    match o.code() {
        OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_SBORROW => o.num_input() == 2 && root(data, o.get_in(0)) == root(data, o.get_in(1)),
        OpCode::CPUI_INT_AND => constant(0, false) || constant(1, false),
        OpCode::CPUI_INT_OR => constant(0, true) || constant(1, true),
        _ => false,
    }
}

/// The Varnode `vn` is a low part or copy of, through `SUBPIECE`s at offset 0
/// and `COPY`s: two reads of one register that heritage gave separate pieces.
fn root(data: &Funcdata, vn: Option<VarnodeId>) -> Option<VarnodeId> {
    let mut cur = vn?;
    for _ in 0..8 {
        let Some(def) = data.vbank().get(cur).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { break };
        let low = match def.code() {
            OpCode::CPUI_COPY => true,
            OpCode::CPUI_SUBPIECE => def.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.get_offset() == 0),
            _ => false,
        };
        match def.get_in(0) {
            Some(i) if low => cur = i,
            _ => break,
        }
    }
    Some(cur)
}

/// Do the low bytes of `op`'s output depend on no more than the low bytes of
/// its input `vn`?
fn low_bits_only(data: &Funcdata, op: &crate::op::PcodeOp, vn: VarnodeId) -> bool {
    match op.code() {
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_NEGATE
        | OpCode::CPUI_INT_2COMP
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_COPY
        | OpCode::CPUI_INDIRECT
        | OpCode::CPUI_MULTIEQUAL => true,
        OpCode::CPUI_INT_LEFT => op.get_in(0) == Some(vn) && op.get_in(1) != Some(vn),
        OpCode::CPUI_PIECE => op.get_in(1) == Some(vn) && op.get_in(0) != Some(vn),
        OpCode::CPUI_SUBPIECE => op.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.get_offset() == 0),
        _ => false,
    }
}

/// How many low bytes of `vn` (of `size` bytes) the op `o` reading it
/// depends on: all of them at a sink, none when the op's output reaches
/// nothing, and for an op whose low output bytes need only the low input bytes,
/// as many as its output's uses need.
fn need(data: &Funcdata, o: &crate::op::PcodeOp, vn: VarnodeId, size: int4, seen: &mut Vec<VarnodeId>, budget: &mut u32) -> int4 {
    let sink = matches!(
        o.code(),
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_STORE | OpCode::CPUI_RETURN
    );
    if erases(data, o) || (o.code() == OpCode::CPUI_PIECE && o.get_in(0) == Some(vn)) {
        return 0;
    }
    let Some(out) = o.get_out().filter(|_| !sink) else { return size };
    let d = low_demand(data, out, seen, budget);
    if low_bits_only(data, o, vn) {
        d.min(size)
    } else if d > 0 {
        size
    } else {
        0
    }
}

/// How many low bytes of `vn` some use depends on: 0 for none, its size once
/// the walk meets a use that may take more, or runs out of `budget`.
fn low_demand(data: &Funcdata, vn: VarnodeId, seen: &mut Vec<VarnodeId>, budget: &mut u32) -> int4 {
    let Some(v) = data.vbank().get(vn) else { return 0 };
    let size = v.get_size();
    if seen.contains(&vn) {
        return 0;
    }
    if *budget == 0 {
        return size;
    }
    *budget -= 1;
    seen.push(vn);
    let mut width = 0;
    for op in v.descend_iter() {
        let Some(o) = data.obank().get(op) else { continue };
        width = width.max(need(data, o, vn, size, seen, budget));
        if width >= size {
            return size;
        }
    }
    width
}

/// What the callee at `entry` was seen reading: the reads its `protoorder`
/// statement records, or the function's own snapshot when it calls itself.
pub fn callee_reads(data: &Funcdata, entry: &Address) -> Option<Vec<(Address, int4)>> {
    if entry == data.get_address() {
        return data.kuna_entry_reads().map(|r| r.read.iter().chain(&r.declared).cloned().collect());
    }
    data.kuna_protoorder_types(entry).map(|s| s.reads.clone())
}

/// The registers the callee at `entry` reads only by handing them on
/// ([`reads_through`]); for the function itself, the ones it passes to a
/// declared parameter.
pub fn callee_passes(data: &Funcdata, entry: &Address) -> Vec<(Address, int4)> {
    if entry == data.get_address() {
        return data.kuna_entry_reads().map(|r| r.declared.clone()).unwrap_or_default();
    }
    data.kuna_protoorder_types(entry).map(|s| s.passes.clone()).unwrap_or_default()
}

/// Does the callee at `entry` read `[addr, addr+size)`, by the best evidence
/// `data` holds: its complete entry decode, which follows constant branches
/// (`strict`: the low byte, a store or alignment push left out), or what its
/// `protoorder` statement hands on; else what its first heritage saw.
pub fn callee_takes(data: &Funcdata, entry: &Address, addr: &Address, size: int4, strict: bool) -> bool {
    let Some(stated) = data.kuna_protoorder_types(entry) else { return false };
    let facts = data
        .kuna_callee_entry_through(entry)
        .or_else(|| data.kuna_callee_entry_dead(entry))
        .filter(|f| f.is_complete());
    match facts {
        Some(f) => {
            (if strict { f.reads_low_byte(addr, size, false, true) } else { f.proves_input(addr, size) })
                || overlaps(&stated.passes, addr, size)
        }
        None => overlaps(&stated.reads, addr, size),
    }
}

/// The function's own incoming registers its finished argument lists hand to a
/// direct callee, with the callee: the forwards no op touched at the first
/// heritage, which `passthrough` added since.
pub fn final_forwards(data: &Funcdata) -> Vec<(Address, int4, Address)> {
    let mut out = Vec::new();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let Some(o) = data.obank().get(fc.get_op()) else { continue };
        if o.is_dead() || o.code() != OpCode::CPUI_CALL || fc.get_entry_address().is_invalid() {
            continue;
        }
        for k in 1..o.num_input() {
            let Some(vn) = o.get_in(k) else { continue };
            let Some((addr, size)) = crate::kuna_armfloatargs::incoming_storage(data, vn) else { continue };
            if addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR) {
                out.push((addr, size, fc.get_entry_address().clone()));
            }
        }
    }
    out
}

/// The registers the function reads, and the subset it reads only by handing
/// them on: to a declared parameter, or to a direct callee that `takes` them
/// ([`callee_takes`]).
pub fn reads_through(
    reads: &EntryReads,
    finals: &[(Address, int4, Address)],
    takes: impl Fn(&Address, &Address, int4) -> bool,
) -> (Vec<(Address, int4)>, Vec<(Address, int4)>) {
    let mut passes = reads.declared.clone();
    for (addr, size, c) in reads.forwarded.iter().chain(finals) {
        if takes(c, addr, *size) && !overlaps(&passes, addr, *size) {
            passes.push((addr.clone(), *size));
        }
    }
    let mut all = reads.read.clone();
    all.extend(passes.iter().filter(|(a, s)| !overlaps(&reads.read, a, *s)).cloned());
    (all, passes)
}

#[cfg(test)]
#[path = "kuna_entryreads/tests.rs"]
mod tests;
