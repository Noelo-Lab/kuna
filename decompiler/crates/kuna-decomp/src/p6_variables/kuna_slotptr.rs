//! (kuna `slotptr`) Type a `framelayout` filler slot from the pointer stored
//! into it (P6, JSON surface only).
//!
//! At -O0 every C local has its own spill slot.  The dataflow copy-propagates the
//! stored value out of the slot and dead-code removal takes the slot's Varnode with
//! it, so `framelayout` reports the slot from an early pass's layout -- where the
//! type system had not committed anything yet -- as the width-only `undefinedN`,
//! while the emitted C prints the value itself as, say, `char *`.  This module
//! keeps the one link the fold destroys: which value was stored into which slot.
//!
//! [`record_pass`] runs beside the `framelayout` union at the tail of every
//! `restructure_varnode` pass and notes, per exact `(offset, size)`, the durable
//! identity of every value stored there (the root op that computed it, the input
//! it came from, or a null constant) plus every stack range any live Varnode
//! touched.  [`slot_pointer_type`] reads that evidence against the FINAL function
//! and returns the stored value's declared type when every store agrees on one
//! committed pointer type of the slot's width.  It declines on any doubt: a store
//! it cannot trace to a live value, stores that disagree, a store or read at a
//! different width or offset inside the slot, a type that is not a pointer.
//!
//! Pointer types only.  A width-only `undefinedN` is the honest report for a slot
//! whose scalar type the recovery never settled; a pointer is a claim the C body
//! already makes about the same value.  Nothing here feeds the IR or the printer.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::AddrSpace;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;

/// Which pointer types a filler slot may take from its stores.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotPtrMode {
    /// Report every filler slot by width, as `framelayout` always did.
    Off,
    /// Take a pointer whose pointee the recovery committed (`char *`, `T *`,
    /// `struct_N *`, `T **`); a `void *` or pointer-to-unknown keeps the width.
    On,
    /// As `On`, and `void *` too.
    Void,
}

impl SlotPtrMode {
    /// Parse an option value.
    pub fn parse(v: &str) -> Option<SlotPtrMode> {
        match v {
            "off" => Some(SlotPtrMode::Off),
            "on" => Some(SlotPtrMode::On),
            "void" => Some(SlotPtrMode::Void),
            _ => None,
        }
    }

    /// The option value spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            SlotPtrMode::Off => "off",
            SlotPtrMode::On => "on",
            SlotPtrMode::Void => "void",
        }
    }
}

/// The durable identity of one value stored into a slot.
#[derive(Clone, Debug, PartialEq)]
pub enum SlotStore {
    /// The constant zero: agrees with any pointer.
    Null,
    /// A value that cannot be traced to anything the final function still has.
    Opaque,
    /// A function input (a parameter's home-slot spill).
    Input(Address, int4),
    /// The op that computed the value (the root of its COPY chain).
    Op(SeqNum),
}

/// Everything [`record_pass`] learned about the frame, across passes.
#[derive(Clone, Debug, Default)]
pub struct SlotEvidence {
    stores: BTreeMap<(i64, int4), Vec<SlotStore>>,
    accesses: BTreeSet<(i64, int4)>,
}

impl SlotEvidence {
    /// Forget everything (the function is being re-decompiled from scratch).
    pub fn clear(&mut self) {
        self.stores.clear();
        self.accesses.clear();
    }

    /// The stores recorded at exactly `(off, size)`.
    pub fn stores_at(&self, off: i64, size: int4) -> Vec<SlotStore> {
        self.stores.get(&(off, size)).cloned().unwrap_or_default()
    }
}

/// Why a filler slot keeps its width-only spelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decline {
    NoStore,
    Overlap,
    Untraced,
    Disagree,
    NotPointer,
    Width,
    Pointee,
}

const COPY_CHAIN_LIMIT: usize = 16;

fn signed_offset(space: &AddrSpace, off: u64) -> i64 {
    kuna_base::address::sign_extend(off as i64, space.get_addr_size() as i32 * 8 - 1)
}

fn classify_value(fd: &Funcdata, mut vn: VarnodeId) -> SlotStore {
    for _ in 0..COPY_CHAIN_LIMIT {
        let Some(v) = fd.vbank().get(vn) else { return SlotStore::Opaque };
        if v.is_constant() {
            return if v.get_offset() == 0 { SlotStore::Null } else { SlotStore::Opaque };
        }
        if v.is_input() {
            return SlotStore::Input(v.get_addr().clone(), v.get_size());
        }
        let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else {
            return SlotStore::Opaque;
        };
        if op.is_indirect_creation() {
            return match op.get_in(1).and_then(|iop| call_of_iop(fd, iop)) {
                Some(call) => SlotStore::Op(call),
                None => SlotStore::Opaque,
            };
        }
        if op.code() != OpCode::CPUI_COPY {
            return SlotStore::Op(op.get_seq_num().clone());
        }
        match op.get_in(0) {
            Some(next) => vn = next,
            None => return SlotStore::Opaque,
        }
    }
    SlotStore::Opaque
}

/// The call an indirect-creation's iop annotation names: the value such an
/// INDIRECT stands for is that call's output once return recovery attaches it.
fn call_of_iop(fd: &Funcdata, iop: VarnodeId) -> Option<SeqNum> {
    let off = fd.vbank().get(iop)?.get_offset();
    let op = fd.obank().get(crate::context::OpId::from(slotmap::KeyData::from_ffi(off)))?;
    matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND).then(|| op.get_seq_num().clone())
}

/// Note the stores into, and the ranges touched in, the stack frame as this
/// `restructure_varnode` pass sees it.
pub fn record_pass(fd: &Funcdata, space: &Rc<AddrSpace>) {
    let index = space.get_index();
    let mut stores: Vec<((i64, int4), SlotStore)> = Vec::new();
    let mut accesses: Vec<(i64, int4)> = Vec::new();
    for vn in fd.vbank().iter_loc() {
        let Some(v) = fd.vbank().get(vn) else { continue };
        if v.get_addr().get_space().map(|s| s.get_index()) != Some(index) || v.is_free() {
            continue;
        }
        let key = (signed_offset(space, v.get_offset()), v.get_size());
        accesses.push(key);
        let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else { continue };
        let store = match op.code() {
            OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL => continue,
            OpCode::CPUI_COPY => match op.get_in(0) {
                Some(src) => classify_value(fd, src),
                None => SlotStore::Opaque,
            },
            _ => SlotStore::Op(op.get_seq_num().clone()),
        };
        stores.push((key, store));
    }
    fd.with_slot_evidence(|ev| {
        ev.accesses.extend(accesses);
        for (key, store) in stores {
            let list = ev.stores.entry(key).or_default();
            if !list.contains(&store) {
                list.push(store);
            }
        }
    });
}

fn declared_type(fd: &Funcdata, arch: &Architecture, vn: VarnodeId) -> Option<Rc<Datatype>> {
    let v = fd.vbank().get(vn)?;
    let rep = v
        .get_high()
        .and_then(|h| crate::printc::decl_type_representative(fd, arch, h))
        .unwrap_or(vn);
    Some(Rc::clone(fd.vbank().get(rep)?.get_type()))
}

fn value_varnode(fd: &Funcdata, store: &SlotStore) -> Option<VarnodeId> {
    match store {
        SlotStore::Null | SlotStore::Opaque => None,
        SlotStore::Input(addr, size) => fd.find_varnode_input(*size, addr),
        SlotStore::Op(seq) => {
            let op = fd.obank().get(fd.obank().find_op(seq)?)?;
            if op.is_dead() {
                return None;
            }
            let out = op.get_out()?;
            let v = fd.vbank().get(out)?;
            if !v.is_implied() {
                return Some(out);
            }
            let cast = fd.lone_descend(out).and_then(|d| fd.obank().get(d));
            match cast {
                Some(c) if c.code() == OpCode::CPUI_CAST => c.get_out().or(Some(out)),
                _ => Some(out),
            }
        }
    }
}

/// The pointer type a filler slot at `(off, size)` takes from the values stored
/// into it, or why it keeps its width-only spelling.
pub fn slot_pointer_type(
    fd: &Funcdata,
    arch: &Architecture,
    mode: SlotPtrMode,
    off: i64,
    size: int4,
) -> Result<Rc<Datatype>, Decline> {
    let ev = fd.slot_evidence();
    let Some(stores) = ev.stores.get(&(off, size)) else { return Err(Decline::NoStore) };
    let end = off + size as i64;
    if ev.accesses.iter().any(|&(o, s)| (o, s) != (off, size) && o < end && off < o + s as i64) {
        return Err(Decline::Overlap);
    }
    let mut found: Option<(String, Rc<Datatype>)> = None;
    for store in stores {
        if *store == SlotStore::Null {
            continue;
        }
        let ty = value_varnode(fd, store)
            .and_then(|vn| declared_type(fd, arch, vn))
            .ok_or(Decline::Untraced)?;
        let spelled = crate::printc::type_to_c_string(arch, &ty);
        match &found {
            Some((s, _)) if *s != spelled => return Err(Decline::Disagree),
            Some(_) => {}
            None => found = Some((spelled, ty)),
        }
    }
    let (_, ty) = found.ok_or(Decline::NoStore)?;
    if ty.get_metatype() != type_metatype::TYPE_PTR {
        return Err(Decline::NotPointer);
    }
    if ty.get_size() != size {
        return Err(Decline::Width);
    }
    let pointee = ty.get_ptr_to().map(|p| p.get_metatype());
    match pointee {
        None | Some(type_metatype::TYPE_UNKNOWN) => return Err(Decline::Pointee),
        Some(type_metatype::TYPE_VOID) if mode != SlotPtrMode::Void => return Err(Decline::Pointee),
        _ => {}
    }
    Ok(ty)
}

#[cfg(test)]
mod tests;
