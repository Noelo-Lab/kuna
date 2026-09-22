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

    /// Note a live stack Varnode at `(off, size)`.
    pub fn note_access(&mut self, off: i64, size: int4) {
        self.accesses.insert((off, size));
    }

    /// Note one value stored into exactly `(off, size)`.
    pub fn note_store(&mut self, off: i64, size: int4, store: SlotStore) {
        let list = self.stores.entry((off, size)).or_default();
        if !list.contains(&store) {
            list.push(store);
        }
    }

    /// The distinct values stored into exactly `(off, size)`.
    pub fn stores_at(&self, off: i64, size: int4) -> &[SlotStore] {
        self.stores.get(&(off, size)).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Whether any pass saw a Varnode overlapping `[off, off + size)` at another
    /// offset or width.
    pub fn foreign_access(&self, off: i64, size: int4) -> bool {
        let end = off + size as i64;
        self.accesses.iter().any(|&(o, s)| (o, s) != (off, size) && o < end && off < o + s as i64)
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
    let mut stores: Vec<((i64, int4), SlotStore)> = Vec::new();
    let mut accesses: Vec<(i64, int4)> = Vec::new();
    let at = |off: u64| Address::new(Rc::clone(space), off);
    let vbank = fd.vbank();
    let whole_space = vbank.iter_loc_addr_range(&at(0), &at(1)).chain(vbank.iter_loc_addr_range(&at(1), &at(0)));
    for vn in whole_space {
        let Some(v) = vbank.get(vn) else { continue };
        if v.is_free() {
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
        for (off, size) in accesses {
            ev.note_access(off, size);
        }
        for ((off, size), store) in stores {
            ev.note_store(off, size, store);
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

fn value_type(fd: &Funcdata, arch: &Architecture, store: &SlotStore) -> Option<Rc<Datatype>> {
    let produced = match store {
        SlotStore::Null | SlotStore::Opaque => return None,
        SlotStore::Input(addr, size) => fd.find_varnode_input(*size, addr)?,
        SlotStore::Op(seq) => {
            let op = fd.obank().get(fd.obank().find_op(seq)?)?;
            if op.is_dead() {
                return None;
            }
            op.get_out()?
        }
    };
    let held = if fd.vbank().get(produced)?.is_implied() {
        match fd.lone_descend(produced).and_then(|d| fd.obank().get(d)) {
            Some(c) if c.code() == OpCode::CPUI_CAST => c.get_out().unwrap_or(produced),
            _ => produced,
        }
    } else {
        produced
    };
    declared_type(fd, arch, held)
}

/// The pointer type a filler slot at `(off, size)` takes from the values stored
/// into it, or why it keeps its width-only spelling.
pub fn slot_pointer_type(
    fd: &Funcdata,
    arch: &Architecture,
    off: i64,
    size: int4,
) -> Result<Rc<Datatype>, Decline> {
    let ev = fd.slot_evidence();
    let stores = ev.stores_at(off, size);
    if stores.is_empty() {
        return Err(Decline::NoStore);
    }
    if ev.foreign_access(off, size) {
        return Err(Decline::Overlap);
    }
    let mut found: Option<(String, Rc<Datatype>)> = None;
    for store in stores {
        if *store == SlotStore::Null {
            continue;
        }
        let ty = value_type(fd, arch, store).ok_or(Decline::Untraced)?;
        let spelled = crate::printc::type_to_c_string(arch, &ty);
        match &found {
            Some((s, _)) if *s != spelled => return Err(Decline::Disagree),
            Some(_) => {}
            None => found = Some((spelled, ty)),
        }
    }
    let (_, ty) = found.ok_or(Decline::NoStore)?;
    admit(&ty, size)?;
    Ok(ty)
}

/// Whether `ty` is a pointer a slot of `size` bytes may report: the slot's
/// width, and a pointee the recovery committed to.  `void *` is admitted; a
/// pointer to unknown bytes names no type, and a pointer to code is spelled
/// `void *` by the C printer and came, in every measured case, from a constant
/// the recovery mistook for a code address.
pub fn admit(ty: &Datatype, size: int4) -> Result<(), Decline> {
    if ty.get_metatype() != type_metatype::TYPE_PTR {
        return Err(Decline::NotPointer);
    }
    if ty.get_size() != size {
        return Err(Decline::Width);
    }
    match ty.get_ptr_to().map(|p| p.get_metatype()) {
        None | Some(type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_CODE) => Err(Decline::Pointee),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests;
